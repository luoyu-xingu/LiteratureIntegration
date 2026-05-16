use axum::{extract::{Path, State, Query}, Json};
use neo4rs::Graph;
use serde::Deserialize;
use crate::errors::AppError;
use crate::models::dto::{AuthorWithPapers, GraphDataResponse};
use crate::models::author::Author;
use crate::repositories::neo4j_repo::Neo4jRepo;
use crate::services::author::AuthorService;

#[derive(Deserialize)]
pub struct WorkspaceQuery {
    pub workspace_id: String,
}

pub async fn list_authors(
    State(graph): State<Graph>,
    Query(params): Query<WorkspaceQuery>,
) -> Result<Json<Vec<Author>>, AppError> {
    let repo = Neo4jRepo::new(graph);
    let authors = AuthorService::list_in_workspace(&repo, &params.workspace_id).await?;
    Ok(Json(authors))
}

pub async fn get_graph(
    State(graph): State<Graph>,
    Query(params): Query<WorkspaceQuery>,
) -> Result<Json<GraphDataResponse>, AppError> {
    let repo = Neo4jRepo::new(graph);
    let data = AuthorService::get_graph_data(&repo, &params.workspace_id).await?;
    Ok(Json(data))
}

pub async fn get_author_papers(
    State(graph): State<Graph>,
    Path(id): Path<String>,
) -> Result<Json<AuthorWithPapers>, AppError> {
    let repo = Neo4jRepo::new(graph);
    let result = AuthorService::get_author_papers(&repo, &id).await?;
    Ok(Json(result))
}
