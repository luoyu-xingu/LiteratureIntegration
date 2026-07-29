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

        // More accurate size estimation based on actual content
        let mut estimated_size = 200 + workspace.name.len();
        for (paper, _fa, _ca, kws) in &papers_detail {
            estimated_size += paper.title.len() + 100;
            estimated_size += paper.abstract_text.as_ref().map(|s| s.len() + 20).unwrap_or(0);
            estimated_size += paper.user_notes.as_ref().filter(|s| !s.is_empty()).map(|s| s.len() + 20).unwrap_or(0);
            estimated_size += kws.iter().map(|k| k.name.len() + 2).sum::<usize>();
        }
        let mut md = String::with_capacity(estimated_size);

        // Build header
        md.push_str("# 工作区: ");
        md.push_str(&workspace.name);
        md.push_str("\n\n> 导出时间: ");
        let now = chrono::Utc::now();
        let dt_str = now.format("%Y-%m-%d %H:%M").to_string();
        md.push_str(&dt_str);
        md.push_str("\n> 论文数量: ");
        md.push_str(&usize_to_str(papers_detail.len()));
        md.push_str("\n\n---\n\n");

        for (paper, first_author, corr_author, keywords) in &papers_detail {
            // Build paper section
            md.push_str("### ");
            md.push_str(&paper.title);
            md.push_str("\n- **年份**: ");
            if let Some(y) = paper.year {
                md.push_str(&i32_to_str(y));
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

            // Keywords - avoid per-iteration bounds check
            let kw_count = keywords.len();
            for (i, kw) in keywords.iter().enumerate() {
                md.push_str(&kw.name);
                if i + 1 < kw_count {
                    md.push_str(", ");
                }
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

        Ok(md)
    }
}

// Fast integer to string conversion using stack buffer
#[inline]
fn usize_to_str(n: usize) -> String {
    let mut buf = [0u8; 20];
    let len = write_usize_to_buf(n, &mut buf);
    // SAFETY: We just wrote valid UTF-8 ASCII digits
    unsafe {
        std::str::from_utf8_unchecked(&buf[20 - len..]).to_string()
    }
}

#[inline]
fn i32_to_str(n: i32) -> String {
    let mut buf = [0u8; 20];
    if n < 0 {
        let len = write_abs_i32_to_buf(n, &mut buf);
        buf[20 - len - 1] = b'-';
        unsafe {
            std::str::from_utf8_unchecked(&buf[20 - len - 1..]).to_string()
        }
    } else {
        let len = write_pos_i32_to_buf(n as u32, &mut buf);
        unsafe {
            std::str::from_utf8_unchecked(&buf[20 - len..]).to_string()
        }
    }
}

#[inline]
fn write_usize_to_buf(mut n: usize, buf: &mut [u8; 20]) -> usize {
    let mut idx = 20;
    if n == 0 {
        idx -= 1;
        buf[idx] = b'0';
        return 1;
    }
    while n > 0 {
        idx -= 1;
        buf[idx] = b'0' + (n % 10) as u8;
        n /= 10;
    }
    20 - idx
}

#[inline]
fn write_pos_i32_to_buf(mut n: u32, buf: &mut [u8; 20]) -> usize {
    let mut idx = 20;
    if n == 0 {
        idx -= 1;
        buf[idx] = b'0';
        return 1;
    }
    while n > 0 {
        idx -= 1;
        buf[idx] = b'0' + (n % 10) as u8;
        n /= 10;
    }
    20 - idx
}

#[inline]
fn write_abs_i32_to_buf(n: i32, buf: &mut [u8; 20]) -> usize {
    let abs = if n == i32::MIN {
        2147483648u32
    } else {
        n.unsigned_abs()
    };
    write_pos_i32_to_buf(abs, buf)
}
