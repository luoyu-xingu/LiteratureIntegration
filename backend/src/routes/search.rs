use axum::{extract::{Query, State}, Json};
use neo4rs::Graph;
use serde::Deserialize;
use crate::errors::AppError;
use crate::repositories::neo4j_repo::Neo4jRepo;
use crate::services::search::SearchService;

#[derive(Deserialize)]
pub struct SearchParams {
    pub workspace_id: String,
    pub q: Option<String>,
    pub author: Option<String>,
}

pub async fn search(
    State(graph): State<Graph>,
    Query(params): Query<SearchParams>,
) -> Result<Json<serde_json::Value>, AppError> {
    let repo = Neo4jRepo::new(graph);
    if let Some(query) = params.q {
        let papers = SearchService::search_by_keyword(&repo, &params.workspace_id, &query).await?;
        Ok(Json(serde_json::json!({ "mode": "keyword", "query": query, "results": papers })))
    } else if let Some(author) = params.author {
        let results = SearchService::search_by_author(&repo, &params.workspace_id, &author).await?;
        Ok(Json(serde_json::json!({ "mode": "author", "query": author, "results": results })))
    } else {
        Err(crate::errors::AppError::ValidationError("Must provide q or author parameter".into()))
    }
}
