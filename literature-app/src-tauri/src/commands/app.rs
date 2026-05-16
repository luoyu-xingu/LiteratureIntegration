use crate::storage::AppState;
use crate::storage::index::ensure_index;
use tauri::State;
use tauri_plugin_dialog::DialogExt;

#[tauri::command]
pub async fn select_root_dir(app: tauri::AppHandle, state: State<'_, AppState>) -> Result<String, String> {
    let dir = app.dialog()
        .file()
        .blocking_pick_folder()
        .ok_or("No directory selected")?
        .to_string();

    let root = std::path::PathBuf::from(&dir);
    ensure_index(&root).map_err(|e| e.to_string())?;

    let mut guard = state.root_dir.lock().unwrap();
    *guard = Some(root);

    Ok(dir)
}

#[tauri::command]
pub async fn get_root_dir(state: State<'_, AppState>) -> Result<String, String> {
    let guard = state.root_dir.lock().unwrap();
    guard.clone()
        .map(|p| p.to_string_lossy().to_string())
        .ok_or("Root directory not set".to_string())
}

#[tauri::command]
pub async fn set_root_dir(path: String, state: State<'_, AppState>) -> Result<String, String> {
    let root = std::path::PathBuf::from(&path);
    if !root.exists() {
        std::fs::create_dir_all(&root).map_err(|e| e.to_string())?;
    }
    ensure_index(&root).map_err(|e| e.to_string())?;

    let mut guard = state.root_dir.lock().unwrap();
    *guard = Some(root);

    Ok(path)
}
