mod config;
mod errors;
mod models;
mod repositories;
mod services;
mod routes;

use axum::{routing::{get, post, put, delete}, Router};
use neo4rs::Graph;
use tower_http::cors::CorsLayer;

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

    let app = Router::new()
        .route("/api/health", get(|| async { "ok" }))
        .nest("/api/workspaces", workspace_routes)
        .with_state(graph)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", cfg.server_host, cfg.server_port))
        .await
        .unwrap();
    tracing::info!("Server running on {}:{}", cfg.server_host, cfg.server_port);
    axum::serve(listener, app).await.unwrap();
}
