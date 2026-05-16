use crate::models::workspace::{RootIndex, WorkspaceIndexEntry};
use anyhow::{Context, Result};
use std::path::PathBuf;

pub fn get_index_path(root: &PathBuf) -> PathBuf {
    root.join("_index.yaml")
}

pub fn read_index(root: &PathBuf) -> Result<RootIndex> {
    let path = get_index_path(root);
    if !path.exists() {
        return Ok(RootIndex {
            version: 1,
            root: root.to_string_lossy().to_string(),
            workspaces: vec![],
        });
    }
    let content = std::fs::read_to_string(&path)
        .context("Failed to read _index.yaml")?;
    let index: RootIndex = serde_yaml::from_str(&content)
        .context("Failed to parse _index.yaml")?;
    Ok(index)
}

pub fn write_index(root: &PathBuf, index: &RootIndex) -> Result<()> {
    let path = get_index_path(root);
    let content = serde_yaml::to_string(index)
        .context("Failed to serialize _index.yaml")?;
    std::fs::write(&path, content)
        .context("Failed to write _index.yaml")?;
    Ok(())
}

pub fn ensure_index(root: &PathBuf) -> Result<RootIndex> {
    let index = read_index(root)?;
    if !get_index_path(root).exists() {
        let new_index = RootIndex {
            version: 1,
            root: root.to_string_lossy().to_string(),
            workspaces: vec![],
        };
        write_index(root, &new_index)?;
        return Ok(new_index);
    }
    Ok(index)
}

pub fn get_root_dir(state: &crate::storage::AppState) -> Result<PathBuf> {
    let guard = state.root_dir.lock().unwrap();
    guard.clone().context("Root directory not set. Please select a root directory first.")
}

pub fn add_workspace_to_index(root: &PathBuf, entry: WorkspaceIndexEntry) -> Result<()> {
    let mut index = read_index(root)?;
    index.workspaces.push(entry);
    write_index(root, &index)
}

pub fn remove_workspace_from_index(root: &PathBuf, workspace_id: &str) -> Result<()> {
    let mut index = read_index(root)?;
    index.workspaces.retain(|w| w.id != workspace_id);
    write_index(root, &index)
}

pub fn update_workspace_in_index(root: &PathBuf, workspace_id: &str, new_name: &str, new_path: &str) -> Result<()> {
    let mut index = read_index(root)?;
    for ws in &mut index.workspaces {
        if ws.id == workspace_id {
            ws.name = new_name.to_string();
            ws.path = new_path.to_string();
        }
    }
    write_index(root, &index)
}
