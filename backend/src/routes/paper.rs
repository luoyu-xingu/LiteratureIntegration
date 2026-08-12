use axum::{extract::{Path, State, Query}, Json};
use neo4rs::Graph;
use serde::Deserialize;
use crate::errors::AppError;
use crate::models::dto::{ImportPaperRequest, UpdatePaperRequest, PaperDetailResponse, PaperRemovedResponse};
use crate::models::paper::Paper;
use crate::repositories::neo4j_repo::Neo4jRepo;
use crate::services::paper::PaperService;

#[derive(Deserialize)]
pub struct WorkspaceQuery {
    pub workspace_id: String,
}

#[derive(Deserialize)]
pub struct DeletePaperQuery {
    pub workspace_id: String,
    pub paper_id: String,
}

pub async fn import_paper(
    State(graph): State<Graph>,
    Query(params): Query<WorkspaceQuery>,
    Json(req): Json<ImportPaperRequest>,
) -> Result<Json<PaperDetailResponse>, AppError> {
    let repo = Neo4jRepo::new(graph);
    let result = PaperService::import(&repo, &params.workspace_id, req).await?;
    Ok(Json(result))
}

pub async fn list_papers(
    State(graph): State<Graph>,
    Query(params): Query<WorkspaceQuery>,
) -> Result<Json<Vec<Paper>>, AppError> {
    let repo = Neo4jRepo::new(graph);
    let papers = PaperService::list_in_workspace(&repo, &params.workspace_id).await?;
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
    Query(params): Query<DeletePaperQuery>,
) -> Result<Json<PaperRemovedResponse>, AppError> {
    let repo = Neo4jRepo::new(graph);
    PaperService::remove_from_workspace(&repo, &params.workspace_id, &params.paper_id).await?;
    Ok(Json(PaperRemovedResponse { removed: true }))
}
