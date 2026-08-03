use crate::errors::AppError;
use crate::models::dto::{PaperDetailResponse, ImportPaperRequest, UpdatePaperRequest};
use crate::models::paper::Paper;
use crate::repositories::external_api::ExternalApiClient;
use crate::repositories::neo4j_repo::Neo4jRepo;

pub struct PaperService;

impl PaperService {
    pub async fn import(repo: &Neo4jRepo, workspace_id: &str, req: ImportPaperRequest) -> Result<PaperDetailResponse, AppError> {
        let client = ExternalApiClient::shared();
        let meta = client.fetch_by_identifier(&req.identifier).await?;

        let paper_id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();

        // Pre-allocate with exact capacity - avoid reallocations
        let mut authors: Vec<(String, String, Option<String>, bool, bool)> = Vec::with_capacity(meta.authors.len());
        for a in &meta.authors {
            authors.push((
                uuid::Uuid::new_v4().to_string(),
                a.name.clone(),
                a.orcid.clone(),
                a.is_first,
                a.is_corresponding,
            ));
        }

        let mut keywords: Vec<(String, String)> = Vec::with_capacity(meta.keywords.len());
        for k in &meta.keywords {
            keywords.push((uuid::Uuid::new_v4().to_string(), k.clone()));
        }

        // Execute workspace check and paper creation in parallel
        let (workspace_result, paper_result) = tokio::join!(
            repo.get_workspace(workspace_id),
            repo.create_paper_if_not_exists(
                &paper_id,
                &meta.title,
                meta.doi.as_deref(),
                meta.arxiv_id.as_deref(),
                meta.abstract_text.as_deref(),
                meta.year,
                meta.journal.as_deref(),
                &now,
            )
        );

        // Validate workspace exists
        workspace_result?
            .ok_or_else(|| AppError::WorkspaceNotFound(workspace_id.to_string()))?;

        let paper = paper_result?;

        // Add paper to workspace, create authors, and add keywords in parallel where possible
        let add_paper_fut = repo.add_paper_to_workspace(workspace_id, &paper.id, &now);
        let authors_fut = repo.create_authors_batch(&authors, &paper.id, workspace_id);
        let keywords_fut = repo.add_keywords_batch(&keywords, &paper.id);

        let (add_result, authors_result, keywords_result) = tokio::join!(
            add_paper_fut,
            authors_fut,
            keywords_fut
        );

        add_result?;
        let (first_author, corresponding_author) = authors_result?;
        keywords_result?;

        // Transform keywords into models with exact capacity to avoid reallocation.
        let mut keyword_models = Vec::with_capacity(keywords.len());
        for (id, name) in keywords {
            keyword_models.push(crate::models::keyword::Keyword { id, name });
        }

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
                .ok_or_else(|| AppError::PaperNotFound(id.to_string()))
        } else {
            repo.get_paper(id).await?
                .ok_or_else(|| AppError::PaperNotFound(id.to_string()))
        }
    }

    pub async fn remove_from_workspace(repo: &Neo4jRepo, workspace_id: &str, paper_id: &str) -> Result<(), AppError> {
        let removed = repo.remove_paper_from_workspace(workspace_id, paper_id).await?;
        if !removed {
            return Err(AppError::PaperNotFound(paper_id.to_string()));
        }
        Ok(())
    }
}
