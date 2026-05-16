use axum::{extract::{Path, State}, Json, http::{StatusCode, header}};
use neo4rs::Graph;
use crate::errors::AppError;
use crate::models::dto::ExportRequest;
use crate::repositories::neo4j_repo::Neo4jRepo;
use crate::services::export::ExportService;

pub async fn export_workspace(
    State(graph): State<Graph>,
    Path(workspace_id): Path<String>,
    Json(req): Json<ExportRequest>,
) -> Result<(StatusCode, [(header::HeaderName, &'static str); 1], String), AppError> {
    let repo = Neo4jRepo::new(graph);
    let markdown = ExportService::export_markdown(&repo, &workspace_id, req).await?;
    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/markdown; charset=utf-8")],
        markdown,
    ))
}
