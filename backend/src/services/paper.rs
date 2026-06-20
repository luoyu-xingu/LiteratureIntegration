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

        // Process authors concurrently for better performance
        let author_futures: Vec<_> = meta.authors.iter().map(|author_meta| {
            let author_id = uuid::Uuid::new_v4().to_string();
            let paper_id = paper.id.clone();
            let is_first = author_meta.is_first;
            let is_corresponding = author_meta.is_corresponding;
            async move {
                let author = repo.create_author_if_not_exists(
                    &author_id,
                    &author_meta.name,
                    author_meta.orcid.as_deref(),
                ).await?;

                if is_first {
                    repo.link_first_author(&author.id, &paper_id).await?;
                }
                if is_corresponding {
                    repo.link_corresponding_author(&author.id, &paper_id).await?;
                }
                Ok::<_, AppError>((author, is_first, is_corresponding))
            }
        }).collect();

        let author_results = futures::future::try_join_all(author_futures).await?;

        let mut first_author = None;
        let mut corresponding_author = None;

        for (author, is_first, is_corresponding) in author_results {
            if is_first {
                first_author = Some(author.clone());
            }
            if is_corresponding {
                corresponding_author = Some(author.clone());
            }
        }

        if let (Some(ref fa), Some(ref ca)) = (&first_author, &corresponding_author) {
            if fa.id != ca.id {
                repo.link_co_authors(&fa.id, &ca.id, workspace_id).await?;
            }
        }

        // Add keywords concurrently
        let keyword_futures: Vec<_> = meta.keywords.iter().map(|keyword_name| {
            let keyword_id = uuid::Uuid::new_v4().to_string();
            let paper_id = paper.id.clone();
            async move {
                repo.add_keyword(&keyword_id, keyword_name, &paper_id).await
            }
        }).collect();

        futures::future::try_join_all(keyword_futures).await?;

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
        let (paper, first_author, corresponding_author, keywords) = repo.get_paper_detail(id).await?;
        let paper = paper.ok_or_else(|| AppError::PaperNotFound(id.to_string()))?;
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
