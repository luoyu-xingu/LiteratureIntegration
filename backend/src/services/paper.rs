use crate::errors::AppError;
use crate::models::dto::{PaperDetailResponse, ImportPaperRequest, UpdatePaperRequest};
use crate::models::paper::Paper;
use crate::repositories::external_api;
use crate::repositories::neo4j_repo::Neo4jRepo;

pub struct PaperService;

impl PaperService {
    pub async fn import(repo: &Neo4jRepo, workspace_id: &str, req: ImportPaperRequest) -> Result<PaperDetailResponse, AppError> {
        let meta = external_api::ExternalApiClient::shared().fetch_by_identifier(&req.identifier).await?;

        let _workspace = repo.get_workspace(workspace_id).await?
            .ok_or_else(|| AppError::WorkspaceNotFound(workspace_id.to_string()))?;

        let paper_id = uuid::Uuid::new_v4().to_string();
        let created_at = chrono::Utc::now().to_rfc3339();

        let paper = repo.create_paper_if_not_exists(
            &paper_id,
            &meta.title,
            meta.doi.as_deref(),
            meta.arxiv_id.as_deref(),
            meta.abstract_text.as_deref(),
            meta.year,
            meta.journal.as_deref(),
            &created_at,
        ).await?;

        let added_at = chrono::Utc::now().to_rfc3339();
        repo.add_paper_to_workspace(workspace_id, &paper.id, &added_at).await?;

        let authors_data: Vec<(String, String, Option<String>)> = meta.authors.iter()
            .map(|am| (uuid::Uuid::new_v4().to_string(), am.name.clone(), am.orcid.clone()))
            .collect();

        let authors = repo.create_authors_batch(&authors_data).await?;

        let first_author = authors.iter()
            .enumerate()
            .find(|(i, _)| meta.authors[*i].is_first)
            .map(|(_, a)| a.clone());

        let corresponding_author = authors.iter()
            .enumerate()
            .find(|(i, _)| meta.authors[*i].is_corresponding)
            .map(|(_, a)| a.clone());

        let first_author_id = first_author.as_ref().map(|a| a.id.as_str());
        let corresponding_author_id = corresponding_author.as_ref().map(|a| a.id.as_str());

        repo.link_authors_to_paper_batch(first_author_id, corresponding_author_id, &paper.id).await?;

        if let (Some(ref fa), Some(ref ca)) = (&first_author, &corresponding_author) {
            if fa.id != ca.id {
                repo.link_co_authors(&fa.id, &ca.id, workspace_id).await?;
            }
        }

        let keywords_data: Vec<(String, String)> = meta.keywords.iter()
            .map(|kn| (uuid::Uuid::new_v4().to_string(), kn.clone()))
            .collect();

        repo.add_keywords_batch(&keywords_data, &paper.id).await?;

        let keywords = repo.get_paper_keywords(&paper.id).await?;

        Ok(PaperDetailResponse {
            paper,
            first_author,
            corresponding_author,
            keywords,
        })
    }

    pub async fn list_in_workspace(repo: &Neo4jRepo, workspace_id: &str) -> Result<Vec<Paper>, AppError> {
        repo.list_papers_in_workspace(workspace_id).await
    }

    pub async fn get_detail(repo: &Neo4jRepo, id: &str) -> Result<PaperDetailResponse, AppError> {
        let (paper, first_author, corresponding_author, keywords) = repo.get_paper_detail(id).await?
            .ok_or_else(|| AppError::PaperNotFound(id.to_string()))?;
        Ok(PaperDetailResponse {
            paper,
            first_author,
            corresponding_author,
            keywords,
        })
    }

    pub async fn update(repo: &Neo4jRepo, id: &str, req: UpdatePaperRequest) -> Result<Paper, AppError> {
        if let Some(notes) = req.user_notes {
            repo.update_paper_notes(id, &notes).await?
                .ok_or_else(|| AppError::PaperNotFound(id.to_string()))?;
        }
        repo.get_paper(id).await?
            .ok_or_else(|| AppError::PaperNotFound(id.to_string()))
    }

    pub async fn remove_from_workspace(repo: &Neo4jRepo, workspace_id: &str, paper_id: &str) -> Result<(), AppError> {
        let removed = repo.remove_paper_from_workspace(workspace_id, paper_id).await?;
        if !removed {
            return Err(AppError::PaperNotFound(paper_id.to_string()));
        }
        Ok(())
    }
}
