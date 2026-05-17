use crate::models::paper::Paper;
use crate::models::author::AuthorWithPapers;
use crate::storage::AppState;
use crate::storage::index::get_root_dir;
use crate::storage::workspace::get_workspace_path_by_id;
use crate::storage::paper;
use tauri::State;
use serde::Serialize;

#[derive(Serialize)]
pub struct SearchResponse {
    pub mode: String,
    pub results: serde_json::Value,
}

#[tauri::command]
pub async fn search(workspace_id: String, query: String, mode: String, state: State<'_, AppState>) -> Result<SearchResponse, String> {
    let root = get_root_dir(&state).map_err(|e| e.to_string())?;
    let ws_path = get_workspace_path_by_id(&root, &workspace_id).map_err(|e| e.to_string())?;
    let papers = paper::list_papers(&root, &ws_path).map_err(|e| e.to_string())?;

    let terms: Vec<String> = query.to_lowercase()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    if terms.is_empty() {
        return Ok(SearchResponse {
            mode: mode.clone(),
            results: serde_json::json!([]),
        });
    }

    match mode.as_str() {
        "keyword" => {
            let filtered: Vec<Paper> = papers.into_iter().filter(|p| matches_keyword(p, &terms)).collect();
            Ok(SearchResponse { mode, results: serde_json::json!(filtered) })
        }
        "author" => {
            let filtered: Vec<Paper> = papers.into_iter().filter(|p| matches_author(p, &terms)).collect();
            let mut author_map: std::collections::HashMap<String, Vec<Paper>> = std::collections::HashMap::new();
            for p in &filtered {
                if let Some(ref name) = p.first_author {
                    author_map.entry(name.clone()).or_default().push(p.clone());
                }
                if let Some(ref name) = p.corresponding_author {
                    author_map.entry(name.clone()).or_default().push(p.clone());
                }
            }
            let results: Vec<AuthorWithPapers> = author_map.into_iter()
                .map(|(name, papers)| AuthorWithPapers { author_name: name, papers })
                .collect();
            Ok(SearchResponse { mode, results: serde_json::json!(results) })
        }
        "content" => {
            let mut filtered = Vec::new();
            for p in &papers {
                if let Ok(detail) = paper::get_paper_detail(&root, &p.id) {
                    if matches_content(&detail, &terms) {
                        filtered.push(p.clone());
                    }
                }
            }
            Ok(SearchResponse { mode, results: serde_json::json!(filtered) })
        }
        _ => Err("Invalid search mode. Use: keyword, author, content".to_string()),
    }
}

fn matches_keyword(paper: &Paper, terms: &[String]) -> bool {
    terms.iter().all(|term| {
        paper.title.to_lowercase().contains(term)
            || paper.keywords.iter().any(|k| k.to_lowercase().contains(term))
            || paper.journal.as_ref().map_or(false, |j| j.to_lowercase().contains(term))
    })
}

fn matches_author(paper: &Paper, terms: &[String]) -> bool {
    terms.iter().all(|term| {
        paper.first_author.as_ref().map_or(false, |a| a.to_lowercase().contains(term))
            || paper.corresponding_author.as_ref().map_or(false, |a| a.to_lowercase().contains(term))
    })
}

fn matches_content(detail: &crate::models::paper::PaperDetailResponse, terms: &[String]) -> bool {
    let full_text = format!(
        "{} {} {} {} {}",
        detail.paper.title,
        detail.abstract_text.as_deref().unwrap_or(""),
        detail.user_notes.as_deref().unwrap_or(""),
        detail.paper.first_author.as_deref().unwrap_or(""),
        detail.paper.corresponding_author.as_deref().unwrap_or(""),
    ).to_lowercase();

    terms.iter().all(|term| full_text.contains(term))
}
