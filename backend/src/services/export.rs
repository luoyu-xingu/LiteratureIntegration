use std::fmt::Write;
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

        write!(md, "# 工作区: {}\n\n> 导出时间: {}\n> 论文数量: {}\n\n---\n\n",
            workspace.name,
            chrono::Utc::now().format("%Y-%m-%d %H:%M"),
            papers_detail.len()
        ).unwrap();

        for (paper, first_author, corr_author, keywords) in &papers_detail {
            write!(md, "### {}\n- **年份**: {} | **期刊**: {}\n- **DOI**: {}\n- **一作**: {} | **通讯**: {}\n- **关键词**: ",
                paper.title,
                paper.year.map(|y| y.to_string()).unwrap_or_default(),
                paper.journal.as_deref().unwrap_or(""),
                paper.doi.as_deref().unwrap_or(""),
                first_author.as_ref().map(|a| a.name.as_str()).unwrap_or(""),
                corr_author.as_ref().map(|a| a.name.as_str()).unwrap_or("")
            ).unwrap();

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
