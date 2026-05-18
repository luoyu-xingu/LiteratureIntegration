use crate::storage::AppState;
use crate::storage::index::get_root_dir;
use crate::storage::workspace::get_workspace_path_by_id;
use crate::storage::paper;
use tauri::State;

#[tauri::command]
pub async fn export_workspace(workspace_id: String, group_by: Option<String>, state: State<'_, AppState>) -> Result<String, String> {
    let root = get_root_dir(&state).map_err(|e| e.to_string())?;
    let ws_path = get_workspace_path_by_id(&root, &workspace_id).map_err(|e| e.to_string())?;
    let ws = crate::storage::workspace::get_workspace(&root, &workspace_id).map_err(|e| e.to_string())?;
    let papers = paper::list_papers(&root, &ws_path).map_err(|e| e.to_string())?;

    let group = group_by.unwrap_or_else(|| "author".to_string());

    let mut md = format!("# 工作区: {}\n\n", ws.name);
    if !ws.description.is_empty() {
        md.push_str(&format!("{}\n\n", ws.description));
    }

    match group.as_str() {
        "author" => {
            let mut author_papers: std::collections::HashMap<String, Vec<_>> = std::collections::HashMap::new();
            for p in &papers {
                let author = p.first_author.as_deref()
                    .or(p.corresponding_author.as_deref())
                    .unwrap_or("Unknown")
                    .to_string();
                author_papers.entry(author).or_default().push(p);
            }
            for (author, ps) in author_papers {
                md.push_str(&format!("## {}\n\n", author));
                for p in ps {
                    md.push_str(&format_paper(p));
                }
            }
        }
        "keyword" => {
            let mut kw_papers: std::collections::HashMap<String, Vec<_>> = std::collections::HashMap::new();
            for p in &papers {
                let kws = if p.keywords.is_empty() { vec!["Uncategorized".to_string()] } else { p.keywords.clone() };
                for kw in kws {
                    kw_papers.entry(kw).or_default().push(p);
                }
            }
            for (kw, ps) in kw_papers {
                md.push_str(&format!("## {}\n\n", kw));
                for p in ps {
                    md.push_str(&format_paper(p));
                }
            }
        }
        _ => {
            for p in &papers {
                md.push_str(&format_paper(p));
            }
        }
    }

    Ok(md)
}

fn format_paper(p: &crate::models::paper::Paper) -> String {
    let mut s = format!("### {}\n\n", p.title);
    if let Some(ref y) = p.year { s.push_str(&format!("- 年份: {}\n", y)); }
    if let Some(ref j) = p.journal { s.push_str(&format!("- 期刊: {}\n", j)); }
    if let Some(ref a) = p.first_author { s.push_str(&format!("- 一作: {}\n", a)); }
    if let Some(ref a) = p.corresponding_author { s.push_str(&format!("- 通讯: {}\n", a)); }
    if let Some(ref d) = p.doi { s.push_str(&format!("- DOI: {}\n", d)); }
    if !p.keywords.is_empty() { s.push_str(&format!("- 关键词: {}\n", p.keywords.join(", "))); }
    s.push_str("\n");
    s
}
