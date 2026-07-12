use crate::errors::AppError;
use crate::models::dto::ExportRequest;
use crate::repositories::neo4j_repo::Neo4jRepo;

pub struct ExportService;

impl ExportService {
    pub async fn export_markdown(repo: &Neo4jRepo, workspace_id: &str, req: ExportRequest) -> Result<String, AppError> {
        let filter = req.filter.unwrap_or_default();
        let author_ids = filter.author_ids.as_deref();
        let keyword_ids = filter.keyword_ids.as_deref();
        let year_range = filter.year_range;

        let papers_detail = repo.get_papers_detail_batch(workspace_id, author_ids, keyword_ids, year_range).await?;

        let workspace = repo.get_workspace(workspace_id).await?
            .ok_or_else(|| AppError::WorkspaceNotFound(workspace_id.to_string()))?;

        let estimated_size = papers_detail.len() * 500 + 200;
        let mut md = String::with_capacity(estimated_size);

        md.push_str("# 工作区: ");
        md.push_str(&workspace.name);
        md.push_str("\n\n> 导出时间: ");
        md.push_str(&chrono::Utc::now().format("%Y-%m-%d %H:%M").to_string());
        md.push_str("\n> 论文数量: ");
        md.push_str(&papers_detail.len().to_string());
        md.push_str("\n\n---\n\n");

        for (paper, first_author, corr_author, keywords) in &papers_detail {
            md.push_str("### ");
            md.push_str(&paper.title);
            md.push_str("\n- **年份**: ");
            md.push_str(&paper.year.map(|y| y.to_string()).unwrap_or_default());
            md.push_str(" | **期刊**: ");
            md.push_str(paper.journal.as_deref().unwrap_or(""));
            md.push_str("\n- **DOI**: ");
            md.push_str(paper.doi.as_deref().unwrap_or(""));
            md.push_str("\n- **一作**: ");
            md.push_str(first_author.as_ref().map(|a| a.name.as_str()).unwrap_or(""));
            md.push_str(" | **通讯**: ");
            md.push_str(corr_author.as_ref().map(|a| a.name.as_str()).unwrap_or(""));
            md.push_str("\n- **关键词**: ");
            for (i, kw) in keywords.iter().enumerate() {
                if i > 0 {
                    md.push_str(", ");
                }
                md.push_str(&kw.name);
            }
            md.push_str("\n\n");

            if let Some(ref abstract_text) = paper.abstract_text {
                md.push_str("**Abstract:**\n");
                md.push_str(abstract_text);
                md.push_str("\n\n");
            }
            if let Some(ref notes) = paper.user_notes {
                if !notes.is_empty() {
                    md.push_str("**笔记:**\n");
                    md.push_str(notes);
                    md.push_str("\n\n");
                }
            }

            md.push_str("---\n\n");
        }

        Ok(md)
    }
}
