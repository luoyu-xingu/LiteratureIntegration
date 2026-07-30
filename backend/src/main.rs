use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

use literature_integration::config;
use literature_integration::routes;
use literature_integration::repositories::neo4j_repo::Neo4jRepo;
use axum::{routing::{get, post, delete}, Router};
use neo4rs::Graph;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

async fn keepalive(graph: Graph) {
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(30)).await;
        let query = neo4rs::query("RETURN 1 AS keepalive");
        match graph.execute(query).await {
            Ok(mut result) => {
                let _ = result.next().await;
            }
            Err(e) => {
                tracing::warn!("Keepalive failed: {}, will retry next cycle", e);
            }
        }
    }
}

#[tokio::main(flavor = "multi_thread", worker_threads = 8)]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let cfg = config::Config::from_env();
    let graph = config::create_neo4j_pool(&cfg)
        .await
        .expect("Failed to connect to Neo4j");

    let repo = Neo4jRepo::new(graph.clone());
    if let Err(e) = repo.create_indexes().await {
        tracing::warn!("Failed to create indexes: {}", e);
    }

    tokio::spawn(keepalive(graph.clone()));

    let cors = CorsLayer::permissive();

    let app = Router::new()
        .route("/api/health", get(|| async { "ok" }))
        .route("/api/workspaces", get(routes::workspace::list_workspaces).post(routes::workspace::create_workspace))
        .route("/api/workspace/:id", get(routes::workspace::get_workspace).put(routes::workspace::update_workspace).delete(routes::workspace::delete_workspace))
        .route("/api/papers", get(routes::paper::list_papers).post(routes::paper::import_paper))
        .route("/api/paper/:id", get(routes::paper::get_paper).put(routes::paper::update_paper))
        .route("/api/paper-rm", delete(routes::paper::delete_paper))
        .route("/api/authors", get(routes::author::list_authors))
        .route("/api/graph", get(routes::author::get_graph))
        .route("/api/author-papers/:id", get(routes::author::get_author_papers))
        .route("/api/search", get(routes::search::search))
        .route("/api/export", post(routes::export::export_workspace))
        .fallback_service(ServeDir::new("../frontend/dist").append_index_html_on_directories(true))
        .with_state(graph)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", cfg.server_host, cfg.server_port))
        .await
        .unwrap();
    tracing::info!("Server running on {}:{}", cfg.server_host, cfg.server_port);
    axum::serve(listener, app).await.unwrap();
}
