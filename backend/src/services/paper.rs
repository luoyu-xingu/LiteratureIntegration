use crate::errors::AppError;
use crate::models::dto::{PaperDetailResponse, ImportPaperRequest, UpdatePaperRequest};
use crate::models::paper::Paper;
use crate::repositories::external_api::ExternalApiClient;
use crate::repositories::neo4j_repo::Neo4jRepo;

pub struct PaperService;

impl PaperService {
    pub async fn import(repo: &Neo4jRepo, workspace_id: &str, req: ImportPaperRequest) -> Result<PaperDetailResponse, AppError> {
        let client = ExternalApiClient::new();
        let meta = client.fetch_by_identifier(&req.identifier).await?;

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

        // 预分配容量，避免多次重新分配
        let authors_count = meta.authors.len();
        let keywords_count = meta.keywords.len();
        
        let mut authors: Vec<(String, String, Option<String>, bool, bool)> = Vec::with_capacity(authors_count);
        for a in &meta.authors {
            authors.push((
                uuid::Uuid::new_v4().to_string(),
                a.name.clone(),
                a.orcid.clone(),
                a.is_first,
                a.is_corresponding,
            ));
        }

        let (first_author, corresponding_author) = repo.create_authors_batch(&authors, &paper.id, workspace_id).await?;

        let mut keywords: Vec<(String, String)> = Vec::with_capacity(keywords_count);
        for k in &meta.keywords {
            keywords.push((uuid::Uuid::new_v4().to_string(), k.clone()));
        }

        repo.add_keywords_batch(&keywords, &paper.id).await?;

        // 使用已分配的 keywords，避免额外的迭代
        let keyword_models: Vec<crate::models::keyword::Keyword> = keywords
            .into_iter()
            .map(|(id, name)| crate::models::keyword::Keyword { id, name })
            .collect();

        Ok(PaperDetailResponse {
            paper,
            first_author,
            corresponding_author,
            keywords: keyword_models,
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
