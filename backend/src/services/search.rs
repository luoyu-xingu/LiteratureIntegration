use crate::errors::AppError;
use crate::models::dto::AuthorWithPapers;
use crate::models::paper::Paper;
use crate::repositories::neo4j_repo::Neo4jRepo;

pub struct SearchService;

impl SearchService {
    pub async fn search_by_keyword(repo: &Neo4jRepo, workspace_id: &str, query: &str) -> Result<Vec<Paper>, AppError> {
        repo.search_by_keyword(workspace_id, query).await
    }

    pub async fn search_by_author(repo: &Neo4jRepo, workspace_id: &str, author_name: &str) -> Result<Vec<AuthorWithPapers>, AppError> {
        repo.search_by_author(workspace_id, author_name).await
    }
}
