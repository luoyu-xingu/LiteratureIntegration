use crate::errors::AppError;
use crate::models::author::Author;
use crate::models::dto::{AuthorWithPapers, GraphDataResponse};
use crate::repositories::neo4j_repo::Neo4jRepo;

pub struct AuthorService;

impl AuthorService {
    pub async fn list_in_workspace(repo: &Neo4jRepo, workspace_id: &str) -> Result<Vec<Author>, AppError> {
        repo.list_authors_in_workspace(workspace_id).await
    }

    pub async fn get_author_papers(repo: &Neo4jRepo, author_id: &str) -> Result<AuthorWithPapers, AppError> {
        let (papers_opt, author_opt) = tokio::join!(
            repo.get_author_papers(author_id),
            repo.get_author_by_id(author_id),
        );
        let papers = papers_opt?;
        let author = author_opt?.unwrap_or_else(|| Author {
            id: author_id.to_string(),
            name: String::new(),
            orcid: None,
        });
        Ok(AuthorWithPapers {
            author,
            papers,
        })
    }

    pub async fn get_graph_data(repo: &Neo4jRepo, workspace_id: &str) -> Result<GraphDataResponse, AppError> {
        let (nodes, links) = repo.get_graph_data(workspace_id).await?;
        Ok(GraphDataResponse { nodes, links })
    }
}
