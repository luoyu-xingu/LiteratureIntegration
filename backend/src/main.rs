mod config;
mod errors;
mod models;
mod repositories;
mod services;
mod routes;

use axum::{routing::{get, post, put, delete}, Router};
use neo4rs::Graph;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let cfg = config::Config::from_env();
    let graph = config::create_neo4j_pool(&cfg)
        .await
        .expect("Failed to connect to Neo4j");

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
