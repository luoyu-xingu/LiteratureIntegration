use crate::models::workspace::{Workspace, WorkspaceIndexEntry};
use crate::storage::index::{read_index, add_workspace_to_index, remove_workspace_from_index, update_workspace_in_index};
use anyhow::{Context, Result};
use std::path::PathBuf;

pub fn get_workspace_dir(root: &PathBuf, workspace_path: &str) -> PathBuf {
    root.join(workspace_path)
}

pub fn get_workspace_yaml(root: &PathBuf, workspace_path: &str) -> PathBuf {
    root.join(workspace_path).join("_workspace.yaml")
}

pub fn create_workspace(root: &PathBuf, name: &str, description: &str) -> Result<Workspace> {
    let id = uuid::Uuid::new_v4().to_string();
    let created_at = chrono::Utc::now().to_rfc3339();
    let path = sanitize_folder_name(name);

    let dir = get_workspace_dir(root, &path);
    std::fs::create_dir_all(&dir)
        .context("Failed to create workspace directory")?;

    let workspace = Workspace {
        id: id.clone(),
        name: name.to_string(),
        description: description.to_string(),
        created_at: created_at.clone(),
    };

    let yaml_path = get_workspace_yaml(root, &path);
    let content = serde_yaml::to_string(&workspace)
        .context("Failed to serialize workspace")?;
    std::fs::write(&yaml_path, content)
        .context("Failed to write _workspace.yaml")?;

    let entry = WorkspaceIndexEntry {
        id: id.clone(),
        name: name.to_string(),
        path: path.clone(),
    };
    add_workspace_to_index(root, entry)?;

    Ok(workspace)
}

pub fn list_workspaces(root: &PathBuf) -> Result<Vec<Workspace>> {
    let index = read_index(root)?;
    let mut workspaces = Vec::new();
    for entry in &index.workspaces {
        let yaml_path = get_workspace_yaml(root, &entry.path);
        if yaml_path.exists() {
            let content = std::fs::read_to_string(&yaml_path)?;
            let ws: Workspace = serde_yaml::from_str(&content)?;
            workspaces.push(ws);
        }
    }
    Ok(workspaces)
}

pub fn get_workspace(root: &PathBuf, workspace_id: &str) -> Result<Workspace> {
    let index = read_index(root)?;
    let entry = index.workspaces.iter()
        .find(|w| w.id == workspace_id)
        .context(format!("Workspace not found: {}", workspace_id))?;
    let yaml_path = get_workspace_yaml(root, &entry.path);
    let content = std::fs::read_to_string(&yaml_path)?;
    let ws: Workspace = serde_yaml::from_str(&content)?;
    Ok(ws)
}

pub fn update_workspace(root: &PathBuf, workspace_id: &str, name: Option<String>, description: Option<String>) -> Result<Workspace> {
    let index = read_index(root)?;
    let entry = index.workspaces.iter()
        .find(|w| w.id == workspace_id)
        .context(format!("Workspace not found: {}", workspace_id))?;

    let yaml_path = get_workspace_yaml(root, &entry.path);
    let content = std::fs::read_to_string(&yaml_path)?;
    let mut ws: Workspace = serde_yaml::from_str(&content)?;

    let old_path = entry.path.clone();
    let mut new_path = old_path.clone();

    if let Some(ref n) = name {
        ws.name = n.clone();
        new_path = sanitize_folder_name(n);
    }
    if let Some(ref d) = description {
        ws.description = d.clone();
    }

    if new_path != old_path {
        let old_dir = root.join(&old_path);
        let new_dir = root.join(&new_path);
        if old_dir != new_dir {
            std::fs::rename(&old_dir, &new_dir)
                .context("Failed to rename workspace directory")?;
        }
        let yaml_path = get_workspace_yaml(root, &new_path);
        let updated_content = serde_yaml::to_string(&ws)?;
        std::fs::write(&yaml_path, updated_content)?;
        update_workspace_in_index(root, workspace_id, &ws.name, &new_path)?;
    } else {
        let updated_content = serde_yaml::to_string(&ws)?;
        let new_yaml = get_workspace_yaml(root, &entry.path);
        std::fs::write(&new_yaml, updated_content)?;
        if name.is_some() {
            update_workspace_in_index(root, workspace_id, &ws.name, &new_path)?;
        }
    }

    Ok(ws)
}

pub fn delete_workspace(root: &PathBuf, workspace_id: &str) -> Result<()> {
    let index = read_index(root)?;
    let entry = index.workspaces.iter()
        .find(|w| w.id == workspace_id)
        .context(format!("Workspace not found: {}", workspace_id))?;

    let dir = root.join(&entry.path);
    if dir.exists() {
        std::fs::remove_dir_all(&dir)
            .context("Failed to delete workspace directory")?;
    }
    remove_workspace_from_index(root, workspace_id)
}

pub fn get_workspace_path_by_id(root: &PathBuf, workspace_id: &str) -> Result<String> {
    let index = read_index(root)?;
    let entry = index.workspaces.iter()
        .find(|w| w.id == workspace_id)
        .context(format!("Workspace not found: {}", workspace_id))?;
    Ok(entry.path.clone())
}

fn sanitize_folder_name(name: &str) -> String {
    let sanitized: String = name
        .chars()
        .map(|c| if c.is_alphanumeric() || c == '-' || c == '_' { c } else if c == ' ' { '-' } else { '-' })
        .collect();
    let trimmed = sanitized.trim_matches('-');
    if trimmed.is_empty() { "unnamed-workspace".to_string() } else { trimmed.to_string() }
}
