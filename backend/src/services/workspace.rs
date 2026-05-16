use crate::errors::AppError;
use crate::models::workspace::Workspace;
use crate::repositories::neo4j_repo::Neo4jRepo;

pub struct WorkspaceService;

impl WorkspaceService {
    pub async fn create(repo: &Neo4jRepo, name: String, description: Option<String>) -> Result<Workspace, AppError> {
        let id = uuid::Uuid::new_v4().to_string();
        let created_at = chrono::Utc::now().to_rfc3339();
        let desc = description.unwrap_or_default();
        repo.create_workspace(&id, &name, &desc, &created_at).await
    }

    pub async fn list(repo: &Neo4jRepo) -> Result<Vec<Workspace>, AppError> {
        repo.list_workspaces().await
    }

    pub async fn get(repo: &Neo4jRepo, id: &str) -> Result<Workspace, AppError> {
        repo.get_workspace(id)
            .await?
            .ok_or_else(|| AppError::WorkspaceNotFound(id.to_string()))
    }

    pub async fn update(repo: &Neo4jRepo, id: &str, name: Option<String>, description: Option<String>) -> Result<Workspace, AppError> {
        repo.update_workspace(id, name.as_deref(), description.as_deref())
            .await?
            .ok_or_else(|| AppError::WorkspaceNotFound(id.to_string()))
    }

    pub async fn delete(repo: &Neo4jRepo, id: &str) -> Result<(), AppError> {
        let deleted = repo.delete_workspace(id).await?;
        if !deleted {
            return Err(AppError::WorkspaceNotFound(id.to_string()));
        }
        Ok(())
    }
}
