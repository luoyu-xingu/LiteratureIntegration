use crate::models::workspace::{CreateWorkspaceRequest, UpdateWorkspaceRequest, Workspace};
use crate::storage::AppState;
use crate::storage::index::get_root_dir;
use crate::storage::workspace;
use tauri::State;

#[tauri::command]
pub async fn list_workspaces(state: State<'_, AppState>) -> Result<Vec<Workspace>, String> {
    let root = get_root_dir(&state).map_err(|e| e.to_string())?;
    workspace::list_workspaces(&root).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_workspace(req: CreateWorkspaceRequest, state: State<'_, AppState>) -> Result<Workspace, String> {
    let root = get_root_dir(&state).map_err(|e| e.to_string())?;
    workspace::create_workspace(&root, &req.name, &req.description.unwrap_or_default())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_workspace(id: String, state: State<'_, AppState>) -> Result<Workspace, String> {
    let root = get_root_dir(&state).map_err(|e| e.to_string())?;
    workspace::get_workspace(&root, &id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_workspace(id: String, req: UpdateWorkspaceRequest, state: State<'_, AppState>) -> Result<Workspace, String> {
    let root = get_root_dir(&state).map_err(|e| e.to_string())?;
    workspace::update_workspace(&root, &id, req.name, req.description)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_workspace(id: String, state: State<'_, AppState>) -> Result<bool, String> {
    let root = get_root_dir(&state).map_err(|e| e.to_string())?;
    workspace::delete_workspace(&root, &id).map_err(|e| e.to_string())?;
    Ok(true)
}
