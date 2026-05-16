use axum::{extract::{Path, State}, Json};
use neo4rs::Graph;
use crate::errors::AppError;
use crate::models::dto::{AuthorWithPapers, GraphDataResponse};
use crate::models::author::Author;
use crate::repositories::neo4j_repo::Neo4jRepo;
use crate::services::author::AuthorService;

pub async fn list_authors(
    State(graph): State<Graph>,
    Path(workspace_id): Path<String>,
) -> Result<Json<Vec<Author>>, AppError> {
    let repo = Neo4jRepo::new(graph);
    let authors = AuthorService::list_in_workspace(&repo, &workspace_id).await?;
    Ok(Json(authors))
}

pub async fn get_graph(
    State(graph): State<Graph>,
    Path(workspace_id): Path<String>,
) -> Result<Json<GraphDataResponse>, AppError> {
    let repo = Neo4jRepo::new(graph);
    let data = AuthorService::get_graph_data(&repo, &workspace_id).await?;
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
