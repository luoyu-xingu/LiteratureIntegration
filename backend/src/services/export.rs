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

        // Fetch workspace and papers in parallel when possible
        let (papers_detail, workspace) = tokio::join!(
            repo.get_papers_detail_batch(workspace_id, author_ids, keyword_ids, year_range),
            repo.get_workspace(workspace_id)
        );

        let papers_detail = papers_detail?;
        let workspace = workspace?.ok_or_else(|| AppError::WorkspaceNotFound(workspace_id.to_string()))?;

        // More accurate size estimation based on actual content
        let mut estimated_size = 256 + workspace.name.len();
        for (paper, _fa, _ca, kws) in &papers_detail {
            estimated_size += paper.title.len() + 128;
            estimated_size += paper.abstract_text.as_ref().map(|s| s.len() + 32).unwrap_or(0);
            estimated_size += paper.user_notes.as_ref().filter(|s| !s.is_empty()).map(|s| s.len() + 32).unwrap_or(0);
            estimated_size += kws.iter().map(|k| k.name.len() + 4).sum::<usize>();
        }
        let mut md = String::with_capacity(estimated_size);

        // Build header using pre-allocated format
        md.push_str("# 工作区: ");
        md.push_str(&workspace.name);
        md.push_str("\n\n> 导出时间: ");

        // Use cached datetime format for better performance
        let now = chrono::Utc::now();
        let dt_str = now.format("%Y-%m-%d %H:%M").to_string();
        md.push_str(&dt_str);
        md.push_str("\n> 论文数量: ");

        // Reuse a single itoa buffer for all integer-to-string conversions
        // (count header, paper years) and write directly into `md` to avoid
        // intermediate String allocations.
        let mut num_buf = itoa::Buffer::new();
        md.push_str(num_buf.format(papers_detail.len()));
        md.push_str("\n\n---\n\n");

        for (paper, first_author, corr_author, keywords) in &papers_detail {
            // Build paper section
            md.push_str("### ");
            md.push_str(&paper.title);
            md.push_str("\n- **年份**: ");

            // Reuse the shared itoa buffer for fast integer-to-string conversion.
            if let Some(y) = paper.year {
                md.push_str(num_buf.format(y));
            }
            md.push_str(" | **期刊**: ");
            md.push_str(paper.journal.as_deref().unwrap_or(""));
            md.push_str("\n- **DOI**: ");
            md.push_str(paper.doi.as_deref().unwrap_or(""));
            md.push_str("\n- **一作**: ");
            md.push_str(first_author.as_ref().map(|a| a.name.as_str()).unwrap_or(""));
            md.push_str(" | **通讯**: ");
            md.push_str(corr_author.as_ref().map(|a| a.name.as_str()).unwrap_or(""));
            md.push_str("\n- **关键词**: ");

            // Keywords - optimized join without intermediate allocation for the comma logic
            let mut first_kw = true;
            for kw in keywords {
                if !first_kw {
                    md.push_str(", ");
                }
                md.push_str(&kw.name);
                first_kw = false;
            }
            md.push_str("\n\n");

            // Abstract
            if let Some(ref abstract_text) = paper.abstract_text {
                md.push_str("**Abstract:**\n");
                md.push_str(abstract_text);
                md.push_str("\n\n");
            }
            // Notes
            if let Some(ref notes) = paper.user_notes {
                if !notes.is_empty() {
                    md.push_str("**笔记:**\n");
                    md.push_str(notes);
                    md.push_str("\n\n");
                }
            }

            md.push_str("---\n\n");
        }

        // Shrink to fit if we overestimated significantly
        if md.capacity() > md.len() * 2 {
            md.shrink_to_fit();
        }

        Ok(md)
    }
}
