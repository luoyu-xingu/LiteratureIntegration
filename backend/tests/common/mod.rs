use axum::Router;
use neo4rs::Graph;
use tower::util::ServiceExt;

pub fn app(graph: Graph) -> Router {
    use axum::routing::{get, post, put, delete};
    use tower_http::cors::CorsLayer;

    let cors = CorsLayer::permissive();

    Router::new()
        .route("/api/health", get(|| async { "ok" }))
        .route("/api/workspaces", get(literature_integration::routes::workspace::list_workspaces).post(literature_integration::routes::workspace::create_workspace))
        .route("/api/workspace/:id", get(literature_integration::routes::workspace::get_workspace).put(literature_integration::routes::workspace::update_workspace).delete(literature_integration::routes::workspace::delete_workspace))
        .route("/api/papers", get(literature_integration::routes::paper::list_papers).post(literature_integration::routes::paper::import_paper))
        .route("/api/paper/:id", get(literature_integration::routes::paper::get_paper).put(literature_integration::routes::paper::update_paper))
        .route("/api/paper-rm", delete(literature_integration::routes::paper::delete_paper))
        .route("/api/authors", get(literature_integration::routes::author::list_authors))
        .route("/api/graph", get(literature_integration::routes::author::get_graph))
        .route("/api/author-papers/:id", get(literature_integration::routes::author::get_author_papers))
        .route("/api/search", get(literature_integration::routes::search::search))
        .route("/api/export", post(literature_integration::routes::export::export_workspace))
        .with_state(graph)
        .layer(cors)
}

pub async fn spawn_neo4j() -> Graph {
    dotenvy::dotenv().ok();
    let uri = std::env::var("NEO4J_URI").unwrap_or_else(|_| "bolt://localhost:7687".into());
    let user = std::env::var("NEO4J_USER").unwrap_or_else(|_| "neo4j".into());
    let password = std::env::var("NEO4J_PASSWORD").unwrap_or_else(|_| "password".into());

    let config = neo4rs::ConfigBuilder::default()
        .uri(&uri)
        .user(&user)
        .password(&password)
        .max_connections(4)
        .fetch_size(1000)
        .build()
        .unwrap();

    Graph::connect(config).await.unwrap()
}
