use crate::errors::AppError;
use crate::models::dto::{PaperDetailResponse, ImportPaperRequest, ImportAuthorInput, UpdatePaperRequest};
use crate::models::paper::Paper;
use crate::repositories::external_api::ExternalApiClient;
use crate::repositories::neo4j_repo::Neo4jRepo;

pub struct PaperService;

impl PaperService {
    pub async fn import(repo: &Neo4jRepo, workspace_id: &str, req: ImportPaperRequest) -> Result<PaperDetailResponse, AppError> {
        let client = ExternalApiClient::shared();
        let meta = client.fetch_by_identifier(&req.identifier).await?;

        let _workspace = repo.get_workspace(workspace_id).await?
            .ok_or_else(|| AppError::WorkspaceNotFound(workspace_id.to_string()))?;

        let paper_id = uuid::Uuid::new_v4().to_string();
        let created_at = chrono::Utc::now().to_rfc3339();
        let added_at = chrono::Utc::now().to_rfc3339();

        let authors: Vec<ImportAuthorInput> = meta.authors.iter().map(|a| ImportAuthorInput {
            id: uuid::Uuid::new_v4().to_string(),
            name: a.name.clone(),
            orcid: a.orcid.clone(),
            is_first: a.is_first,
            is_corresponding: a.is_corresponding,
        }).collect();

        let keywords: Vec<String> = meta.keywords.clone();

        let result = repo.batch_import_paper(
            workspace_id,
            &paper_id,
            &meta.title,
            meta.doi.as_deref(),
            meta.arxiv_id.as_deref(),
            meta.abstract_text.as_deref(),
            meta.year,
            meta.journal.as_deref(),
            &created_at,
            &added_at,
            &authors,
            &keywords,
        ).await?;

        let first_author = result.first_author.clone();
        let corresponding_author = result.corresponding_author.clone();

        if let (Some(ref fa), Some(ref ca)) = (&first_author, &corresponding_author) {
            if fa.id != ca.id {
                repo.link_co_authors(&fa.id, &ca.id, workspace_id).await?;
            }
        }

        Ok(PaperDetailResponse {
            paper: result.paper,
            first_author,
            corresponding_author,
            keywords: result.keywords,
        })
    }

    pub async fn list_in_workspace(repo: &Neo4jRepo, workspace_id: &str) -> Result<Vec<Paper>, AppError> {
        repo.list_papers_in_workspace(workspace_id).await
    }

    pub async fn get_detail(repo: &Neo4jRepo, id: &str) -> Result<PaperDetailResponse, AppError> {
        repo.get_paper_full(id).await?
            .ok_or_else(|| AppError::PaperNotFound(id.to_string()))
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
