mod models;
mod storage;
mod commands;
mod external;

use storage::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            commands::app::select_root_dir,
            commands::app::get_root_dir,
            commands::app::set_root_dir,
            commands::workspace::list_workspaces,
            commands::workspace::create_workspace,
            commands::workspace::get_workspace,
            commands::workspace::update_workspace,
            commands::workspace::delete_workspace,
            commands::paper::list_papers,
            commands::paper::import_paper,
            commands::paper::get_paper,
            commands::paper::update_paper,
            commands::paper::delete_paper,
            commands::search::search,
            commands::author::get_authors,
            commands::author::get_graph,
            commands::author::get_author_papers,
            commands::export::export_workspace,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
