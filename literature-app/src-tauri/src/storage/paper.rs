use crate::models::paper::{Paper, PaperFile, PaperDetailResponse};
use anyhow::{Context, Result};
use std::path::PathBuf;

pub fn create_paper_file(root: &PathBuf, workspace_path: &str, paper: &Paper, abstract_text: Option<&str>, user_notes: Option<&str>) -> Result<()> {
    let filename = sanitize_filename(&paper.title);
    let dir = root.join(workspace_path);
    let filepath = dir.join(format!("{}.md", filename));

    let mut content = String::new();
    let frontmatter = serde_yaml::to_string(paper)
        .context("Failed to serialize paper frontmatter")?;
    content.push_str("---\n");
    content.push_str(&frontmatter);
    content.push_str("---\n\n");

    if let Some(abs) = abstract_text {
        content.push_str("## Abstract\n\n");
        content.push_str(abs);
        content.push_str("\n\n");
    }

    if let Some(notes) = user_notes {
        content.push_str("## 我的笔记\n\n");
        content.push_str(notes);
        content.push_str("\n");
    }

    std::fs::write(&filepath, content)
        .context("Failed to write paper file")?;
    Ok(())
}

pub fn list_papers(root: &PathBuf, workspace_path: &str) -> Result<Vec<Paper>> {
    let dir = root.join(workspace_path);
    if !dir.exists() {
        return Ok(vec![]);
    }

    let mut papers = Vec::new();
    for entry in std::fs::read_dir(&dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().map_or(false, |e| e == "md") {
            let filename = path.file_name().unwrap().to_string_lossy().to_string();
            if filename.starts_with('_') {
                continue;
            }
            if let Ok(pf) = parse_paper_file(&path) {
                papers.push(pf.frontmatter);
            }
        }
    }
    papers.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    Ok(papers)
}

pub fn get_paper_detail(root: &PathBuf, paper_id: &str) -> Result<PaperDetailResponse> {
    let (_path, pf) = find_paper_by_id(root, paper_id)?;
    Ok(PaperDetailResponse {
        paper: pf.frontmatter,
        abstract_text: pf.abstract_text,
        user_notes: pf.user_notes,
    })
}

pub fn update_paper_notes(root: &PathBuf, paper_id: &str, new_notes: &str) -> Result<PaperDetailResponse> {
    let (path, mut pf) = find_paper_by_id(root, paper_id)?;
    pf.user_notes = Some(new_notes.to_string());
    write_paper_file(&path, &pf)?;
    Ok(PaperDetailResponse {
        paper: pf.frontmatter,
        abstract_text: pf.abstract_text,
        user_notes: pf.user_notes,
    })
}

pub fn delete_paper_file(root: &PathBuf, _workspace_path: &str, paper_id: &str) -> Result<()> {
    let (path, _) = find_paper_by_id(root, paper_id)?;
    std::fs::remove_file(&path)
        .context("Failed to delete paper file")?;
    Ok(())
}

pub fn find_paper_by_id(root: &PathBuf, paper_id: &str) -> Result<(PathBuf, PaperFile)> {
    let index = crate::storage::index::read_index(root)?;
    for ws_entry in &index.workspaces {
        let dir = root.join(&ws_entry.path);
        if !dir.exists() {
            continue;
        }
        for entry in std::fs::read_dir(&dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().map_or(false, |e| e == "md") {
                let filename = path.file_name().unwrap().to_string_lossy().to_string();
                if filename.starts_with('_') {
                    continue;
                }
                if let Ok(pf) = parse_paper_file(&path) {
                    if pf.frontmatter.id == paper_id {
                        return Ok((path, pf));
                    }
                }
            }
        }
    }
    anyhow::bail!("Paper not found: {}", paper_id)
}

fn parse_paper_file(path: &PathBuf) -> Result<PaperFile> {
    let content = std::fs::read_to_string(path)
        .context("Failed to read paper file")?;

    let parsed = gray_matter::Matter::<gray_matter::engine::YAML>::new()
        .parse(&content);

    let paper: Paper = serde_yaml::from_str(&parsed.matter)
        .context("Failed to parse paper frontmatter")?;

    let body = parsed.content.trim();

    let (abstract_text, user_notes) = split_body(body);

    Ok(PaperFile {
        frontmatter: paper,
        abstract_text,
        user_notes,
    })
}

fn split_body(body: &str) -> (Option<String>, Option<String>) {
    let mut abstract_text = None;
    let mut user_notes = None;

    if let Some(abs_start) = body.find("## Abstract") {
        let after_abs = &body[abs_start + "## Abstract".len()..];
        let abs_end = after_abs.find("## 我的笔记").unwrap_or(after_abs.len());
        let abs_content = after_abs[..abs_end].trim();
        if !abs_content.is_empty() {
            abstract_text = Some(abs_content.to_string());
        }
    }

    if let Some(notes_start) = body.find("## 我的笔记") {
        let after_notes = &body[notes_start + "## 我的笔记".len()..];
        let notes_content = after_notes.trim();
        if !notes_content.is_empty() {
            user_notes = Some(notes_content.to_string());
        }
    }

    (abstract_text, user_notes)
}

fn write_paper_file(path: &PathBuf, pf: &PaperFile) -> Result<()> {
    let mut content = String::new();
    let frontmatter = serde_yaml::to_string(&pf.frontmatter)?;
    content.push_str("---\n");
    content.push_str(&frontmatter);
    content.push_str("---\n\n");

    if let Some(ref abs) = pf.abstract_text {
        content.push_str("## Abstract\n\n");
        content.push_str(abs);
        content.push_str("\n\n");
    }

    if let Some(ref notes) = pf.user_notes {
        content.push_str("## 我的笔记\n\n");
        content.push_str(notes);
        content.push_str("\n");
    }

    std::fs::write(path, content)?;
    Ok(())
}

pub fn sanitize_filename(title: &str) -> String {
    let sanitized: String = title
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() || c == '-' || c == '_' { c } else if c == ' ' { '-' } else { '-' })
        .collect();
    let result: String = sanitized
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-");
    if result.len() > 80 { result[..80].trim_end_matches('-').to_string() } else { result }
}
