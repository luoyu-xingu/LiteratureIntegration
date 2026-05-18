use axum::{extract::{Query, State}, Json, http::{StatusCode, header}};
use neo4rs::Graph;
use serde::Deserialize;
use crate::errors::AppError;
use crate::models::dto::ExportRequest;
use crate::repositories::neo4j_repo::Neo4jRepo;
use crate::services::export::ExportService;

#[derive(Deserialize)]
pub struct ExportQuery {
    pub workspace_id: String,
}

pub async fn export_workspace(
    State(graph): State<Graph>,
    Query(params): Query<ExportQuery>,
    Json(req): Json<ExportRequest>,
) -> Result<(StatusCode, [(header::HeaderName, &'static str); 1], String), AppError> {
    let repo = Neo4jRepo::new(graph);
    let markdown = ExportService::export_markdown(&repo, &params.workspace_id, req).await?;
    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/markdown; charset=utf-8")],
        markdown,
    ))
}
