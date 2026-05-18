use crate::models::author::{Author, AuthorWithPapers, GraphDataResponse, GraphNode, GraphLink};
use crate::storage::AppState;
use crate::storage::index::get_root_dir;
use crate::storage::workspace::get_workspace_path_by_id;
use crate::storage::paper;
use tauri::State;
use std::collections::HashMap;

#[tauri::command]
pub async fn get_authors(workspace_id: String, state: State<'_, AppState>) -> Result<Vec<Author>, String> {
    let root = get_root_dir(&state).map_err(|e| e.to_string())?;
    let ws_path = get_workspace_path_by_id(&root, &workspace_id).map_err(|e| e.to_string())?;
    let papers = paper::list_papers(&root, &ws_path).map_err(|e| e.to_string())?;

    let mut author_map: HashMap<String, Author> = HashMap::new();
    for p in &papers {
        if let Some(ref name) = p.first_author {
            let entry = author_map.entry(name.clone()).or_insert_with(|| Author {
                name: name.clone(),
                first_author_count: 0,
                corresponding_author_count: 0,
                paper_count: 0,
            });
            entry.first_author_count += 1;
            entry.paper_count += 1;
        }
        if let Some(ref name) = p.corresponding_author {
            let entry = author_map.entry(name.clone()).or_insert_with(|| Author {
                name: name.clone(),
                first_author_count: 0,
                corresponding_author_count: 0,
                paper_count: 0,
            });
            entry.corresponding_author_count += 1;
            if p.first_author.as_ref() != Some(name) {
                entry.paper_count += 1;
            }
        }
    }

    Ok(author_map.into_values().collect())
}

#[tauri::command]
pub async fn get_graph(workspace_id: String, state: State<'_, AppState>) -> Result<GraphDataResponse, String> {
    let root = get_root_dir(&state).map_err(|e| e.to_string())?;
    let ws_path = get_workspace_path_by_id(&root, &workspace_id).map_err(|e| e.to_string())?;
    let papers = paper::list_papers(&root, &ws_path).map_err(|e| e.to_string())?;

    let mut author_papers: HashMap<String, (i32, String)> = HashMap::new();
    let mut coauthor_pairs: HashMap<(String, String), i32> = HashMap::new();

    for p in &papers {
        let mut paper_authors: Vec<String> = vec![];
        if let Some(ref name) = p.first_author {
            paper_authors.push(name.clone());
            let entry = author_papers.entry(name.clone()).or_insert((0, "first".to_string()));
            entry.0 += 1;
        }
        if let Some(ref name) = p.corresponding_author {
            if !paper_authors.contains(name) {
                paper_authors.push(name.clone());
            }
            let entry = author_papers.entry(name.clone()).or_insert((0, "corresponding".to_string()));
            entry.0 += 1;
            if entry.1 == "first" {
                entry.1 = "both".to_string();
            }
        }
        for i in 0..paper_authors.len() {
            for j in (i + 1)..paper_authors.len() {
                let mut a = paper_authors[i].clone();
                let mut b = paper_authors[j].clone();
                if a > b { std::mem::swap(&mut a, &mut b); }
                *coauthor_pairs.entry((a, b)).or_insert(0) += 1;
            }
        }
    }

    let nodes: Vec<GraphNode> = author_papers.iter()
        .map(|(name, (count, atype))| GraphNode {
            id: name.clone(),
            name: name.clone(),
            paper_count: *count,
            author_type: atype.clone(),
        })
        .collect();

    let links: Vec<GraphLink> = coauthor_pairs.iter()
        .map(|((a, b), count)| GraphLink {
            source: a.clone(),
            target: b.clone(),
            paper_count: *count,
        })
        .collect();

    Ok(GraphDataResponse { nodes, links })
}

#[tauri::command]
pub async fn get_author_papers(author_name: String, state: State<'_, AppState>) -> Result<AuthorWithPapers, String> {
    let root = get_root_dir(&state).map_err(|e| e.to_string())?;
    let index = crate::storage::index::read_index(&root).map_err(|e| e.to_string())?;
    let mut papers = Vec::new();
    for ws in &index.workspaces {
        if let Ok(ws_papers) = paper::list_papers(&root, &ws.path) {
            for p in ws_papers {
                if p.first_author.as_deref() == Some(&author_name)
                    || p.corresponding_author.as_deref() == Some(&author_name) {
                    papers.push(p);
                }
            }
        }
    }
    Ok(AuthorWithPapers { author_name, papers })
}
