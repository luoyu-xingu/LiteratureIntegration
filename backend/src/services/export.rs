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

        let papers = repo.get_papers_for_export(workspace_id, author_ids, keyword_ids, year_range).await?;

        let workspace = repo.get_workspace(workspace_id).await?
            .ok_or_else(|| AppError::WorkspaceNotFound(workspace_id.to_string()))?;

        let paper_ids: Vec<String> = papers.iter().map(|p| p.id.clone()).collect();
        let (first_authors, corr_authors, keywords_map) =
            repo.get_paper_authors_and_keywords_batch(&paper_ids).await?;

        let mut md = format!("# 工作区: {}\n\n", workspace.name);
        md.push_str(&format!("> 导出时间: {}\n", chrono::Utc::now().format("%Y-%m-%d %H:%M")));
        md.push_str(&format!("> 论文数量: {}\n\n---\n\n", papers.len()));

        for paper in &papers {
            md.push_str(&format!("### {}\n", paper.title));
            md.push_str(&format!("- **年份**: {} | **期刊**: {}\n", paper.year.map(|y| y.to_string()).unwrap_or_default(), paper.journal.as_deref().unwrap_or("")));
            md.push_str(&format!("- **DOI**: {}\n", paper.doi.as_deref().unwrap_or("")));

            let first_author = first_authors.get(&paper.id).cloned().flatten();
            let corr_author = corr_authors.get(&paper.id).cloned().flatten();
            md.push_str(&format!("- **一作**: {} | **通讯**: {}\n",
                first_author.map(|a| a.name).unwrap_or_default(),
                corr_author.map(|a| a.name).unwrap_or_default()
            ));

            let keywords = keywords_map.get(&paper.id).cloned().unwrap_or_default();
            let kw_str: Vec<String> = keywords.iter().map(|k| k.name.clone()).collect();
            md.push_str(&format!("- **关键词**: {}\n\n", kw_str.join(", ")));

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
