pub mod models;
pub mod errors;
pub mod repositories;

use axum::{routing::get, Router};
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/api/health", get(|| async { "ok" }))
        .layer(CorsLayer::permissive());

    let host = std::env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".into());
    let port: u16 = std::env::var("SERVER_PORT")
        .unwrap_or_else(|_| "3000".into())
        .parse()
        .unwrap();

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", host, port))
        .await
        .unwrap();
    tracing::info!("Server running on {}:{}", host, port);
    axum::serve(listener, app).await.unwrap();
}
