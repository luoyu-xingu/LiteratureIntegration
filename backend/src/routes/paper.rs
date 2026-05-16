use axum::{extract::{Path, State}, Json};
use neo4rs::Graph;
use crate::errors::AppError;
use crate::models::dto::{ImportPaperRequest, UpdatePaperRequest, PaperDetailResponse};
use crate::models::paper::Paper;
use crate::repositories::neo4j_repo::Neo4jRepo;
use crate::services::paper::PaperService;

pub async fn import_paper(
    State(graph): State<Graph>,
    Path(workspace_id): Path<String>,
    Json(req): Json<ImportPaperRequest>,
) -> Result<Json<PaperDetailResponse>, AppError> {
    let repo = Neo4jRepo::new(graph);
    let result = PaperService::import(&repo, &workspace_id, req).await?;
    Ok(Json(result))
}

pub async fn list_papers(
    State(graph): State<Graph>,
    Path(workspace_id): Path<String>,
) -> Result<Json<Vec<Paper>>, AppError> {
    let repo = Neo4jRepo::new(graph);
    let papers = PaperService::list_in_workspace(&repo, &workspace_id).await?;
    Ok(Json(papers))
}

pub async fn get_paper(
    State(graph): State<Graph>,
    Path(id): Path<String>,
) -> Result<Json<PaperDetailResponse>, AppError> {
    let repo = Neo4jRepo::new(graph);
    let detail = PaperService::get_detail(&repo, &id).await?;
    Ok(Json(detail))
}

pub async fn update_paper(
    State(graph): State<Graph>,
    Path(id): Path<String>,
    Json(req): Json<UpdatePaperRequest>,
) -> Result<Json<Paper>, AppError> {
    let repo = Neo4jRepo::new(graph);
    let paper = PaperService::update(&repo, &id, req).await?;
    Ok(Json(paper))
}

pub async fn delete_paper(
    State(graph): State<Graph>,
    Path((workspace_id, paper_id)): Path<(String, String)>,
) -> Result<Json<serde_json::Value>, AppError> {
    let repo = Neo4jRepo::new(graph);
    PaperService::remove_from_workspace(&repo, &workspace_id, &paper_id).await?;
    Ok(Json(serde_json::json!({"removed": true})))
}
