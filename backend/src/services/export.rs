use crate::errors::AppError;
use crate::models::dto::ExportRequest;
use crate::repositories::neo4j_repo::Neo4jRepo;

pub struct ExportService;

impl ExportService {
    pub async fn export_markdown(repo: &Neo4jRepo, workspace_id: &str, req: ExportRequest) -> Result<String, AppError> {
        let filter = req.filter.unwrap_or_default();
        let author_ids = filter.author_ids.as_deref();
        let keyword_ids = filter.keyword_ids.as_deref();
        let _year_range = filter.year_range;

        let papers = repo.get_papers_full_for_export(workspace_id, author_ids, keyword_ids).await?;

        let workspace = repo.get_workspace(workspace_id).await?
            .ok_or_else(|| AppError::WorkspaceNotFound(workspace_id.to_string()))?;

        let mut md = format!("# 工作区: {}\n\n", workspace.name);
        md.push_str(&format!("> 导出时间: {}\n", chrono::Utc::now().format("%Y-%m-%d %H:%M")));
        md.push_str(&format!("> 论文数量: {}\n\n---\n\n", papers.len()));

        for row in &papers {
            let paper = &row.paper;
            md.push_str(&format!("### {}\n", paper.title));
            md.push_str(&format!("- **年份**: {} | **期刊**: {}\n",
                paper.year.map(|y| y.to_string()).unwrap_or_default(),
                paper.journal.as_deref().unwrap_or("")
            ));
            md.push_str(&format!("- **DOI**: {}\n", paper.doi.as_deref().unwrap_or("")));

            md.push_str(&format!("- **一作**: {} | **通讯**: {}\n",
                row.first_author_name.as_deref().unwrap_or(""),
                row.corresponding_author_name.as_deref().unwrap_or("")
            ));

            md.push_str(&format!("- **关键词**: {}\n\n", row.keywords.join(", ")));

            if let Some(ref abstract_text) = paper.abstract_text {
                md.push_str(&format!("**Abstract:**\n{}\n\n", abstract_text));
            }
            if let Some(ref notes) = paper.user_notes {
                if !notes.is_empty() {
                    md.push_str(&format!("**笔记:**\n{}\n\n", notes));
                }
            }

            md.push_str("---\n\n");
        }

        Ok(md)
    }
}
