use axum::{extract::{Path, State}, Json};
use neo4rs::Graph;
use crate::errors::AppError;
use crate::models::dto::{CreateWorkspaceRequest, DeleteResponse, UpdateWorkspaceRequest};
use crate::models::workspace::Workspace;
use crate::repositories::neo4j_repo::Neo4jRepo;
use crate::services::workspace::WorkspaceService;

pub async fn create_workspace(
    State(graph): State<Graph>,
    Json(req): Json<CreateWorkspaceRequest>,
) -> Result<Json<Workspace>, AppError> {
    let repo = Neo4jRepo::new(graph);
    let workspace = WorkspaceService::create(&repo, req.name, req.description).await?;
    Ok(Json(workspace))
}

pub async fn list_workspaces(
    State(graph): State<Graph>,
) -> Result<Json<Vec<Workspace>>, AppError> {
    let repo = Neo4jRepo::new(graph);
    let workspaces = WorkspaceService::list(&repo).await?;
    Ok(Json(workspaces))
}

pub async fn get_workspace(
    State(graph): State<Graph>,
    Path(id): Path<String>,
) -> Result<Json<Workspace>, AppError> {
    let repo = Neo4jRepo::new(graph);
    let workspace = WorkspaceService::get(&repo, &id).await?;
    Ok(Json(workspace))
}

pub async fn update_workspace(
    State(graph): State<Graph>,
    Path(id): Path<String>,
    Json(req): Json<UpdateWorkspaceRequest>,
) -> Result<Json<Workspace>, AppError> {
    let repo = Neo4jRepo::new(graph);
    let workspace = WorkspaceService::update(&repo, &id, req.name, req.description).await?;
    Ok(Json(workspace))
}

pub async fn delete_workspace(
    State(graph): State<Graph>,
    Path(id): Path<String>,
) -> Result<Json<DeleteResponse>, AppError> {
    let repo = Neo4jRepo::new(graph);
    WorkspaceService::delete(&repo, &id).await?;
    Ok(Json(DeleteResponse { deleted: true }))
}
