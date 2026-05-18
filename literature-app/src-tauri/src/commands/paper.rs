use crate::models::paper::{ImportPaperRequest, PaperDetailResponse, Paper, UpdatePaperRequest};
use crate::storage::AppState;
use crate::storage::index::get_root_dir;
use crate::storage::workspace::get_workspace_path_by_id;
use crate::storage::paper;
use crate::external;
use tauri::State;

#[tauri::command]
pub async fn list_papers(workspace_id: String, state: State<'_, AppState>) -> Result<Vec<Paper>, String> {
    let root = get_root_dir(&state).map_err(|e| e.to_string())?;
    let ws_path = get_workspace_path_by_id(&root, &workspace_id).map_err(|e| e.to_string())?;
    paper::list_papers(&root, &ws_path).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn import_paper(workspace_id: String, req: ImportPaperRequest, state: State<'_, AppState>) -> Result<PaperDetailResponse, String> {
    let root = get_root_dir(&state).map_err(|e| e.to_string())?;
    let ws_path = get_workspace_path_by_id(&root, &workspace_id).map_err(|e| e.to_string())?;

    let result = external::import_by_identifier(&req.identifier).await
        .map_err(|e| e.to_string())?;

    paper::create_paper_file(&root, &ws_path, &result.paper, result.abstract_text.as_deref(), None)
        .map_err(|e| e.to_string())?;

    Ok(PaperDetailResponse {
        paper: result.paper,
        abstract_text: result.abstract_text,
        user_notes: None,
    })
}

#[tauri::command]
pub async fn get_paper(id: String, state: State<'_, AppState>) -> Result<PaperDetailResponse, String> {
    let root = get_root_dir(&state).map_err(|e| e.to_string())?;
    paper::get_paper_detail(&root, &id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_paper(id: String, req: UpdatePaperRequest, state: State<'_, AppState>) -> Result<PaperDetailResponse, String> {
    let root = get_root_dir(&state).map_err(|e| e.to_string())?;
    if let Some(notes) = req.user_notes {
        paper::update_paper_notes(&root, &id, &notes).map_err(|e| e.to_string())
    } else {
        paper::get_paper_detail(&root, &id).map_err(|e| e.to_string())
    }
}

#[tauri::command]
pub async fn delete_paper(workspace_id: String, paper_id: String, state: State<'_, AppState>) -> Result<bool, String> {
    let root = get_root_dir(&state).map_err(|e| e.to_string())?;
    let ws_path = get_workspace_path_by_id(&root, &workspace_id).map_err(|e| e.to_string())?;
    paper::delete_paper_file(&root, &ws_path, &paper_id).map_err(|e| e.to_string())?;
    Ok(true)
}
