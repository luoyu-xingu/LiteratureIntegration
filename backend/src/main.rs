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

    let workspace_routes = Router::new()
        .route("/", post(routes::workspace::create_workspace).get(routes::workspace::list_workspaces))
        .route("/{id}", get(routes::workspace::get_workspace).put(routes::workspace::update_workspace).delete(routes::workspace::delete_workspace));

    let paper_routes = Router::new()
        .route("/", post(routes::paper::import_paper).get(routes::paper::list_papers))
        .route("/{id}", get(routes::paper::get_paper).put(routes::paper::update_paper).delete(routes::paper::delete_paper));

    let author_routes = Router::new()
        .route("/", get(routes::author::list_authors))
        .route("/graph", get(routes::author::get_graph));

    let app = Router::new()
        .route("/api/health", get(|| async { "ok" }))
        .nest("/api/workspaces", workspace_routes)
        .nest("/api/workspaces/{workspace_id}/papers", paper_routes)
        .nest("/api/workspaces/{workspace_id}/authors", author_routes)
        .route("/api/workspaces/{workspace_id}/search", get(routes::search::search))
        .route("/api/workspaces/{workspace_id}/export", post(routes::export::export_workspace))
        .route("/api/authors/{id}/papers", get(routes::author::get_author_papers))
        .fallback_service(ServeDir::new("../frontend/dist").append_index_html_on_directories(true))
        .with_state(graph)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", cfg.server_host, cfg.server_port))
        .await
        .unwrap();
    tracing::info!("Server running on {}:{}", cfg.server_host, cfg.server_port);
    axum::serve(listener, app).await.unwrap();
}
