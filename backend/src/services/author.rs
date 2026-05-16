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
        let papers = repo.get_author_papers(author_id).await?;
        Ok(AuthorWithPapers {
            author: Author {
                id: author_id.to_string(),
                name: String::new(),
                orcid: None,
            },
            papers,
        })
    }

    pub async fn get_graph_data(repo: &Neo4jRepo, workspace_id: &str) -> Result<GraphDataResponse, AppError> {
        let (nodes, links) = repo.get_graph_data(workspace_id).await?;
        Ok(GraphDataResponse { nodes, links })
    }
}
