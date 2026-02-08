mod core;

use core::{PluginInfo, PluginPreview, RunResult};
use serde_json::Value;
use tauri::AppHandle;

#[tauri::command]
fn list_plugins(app: AppHandle) -> Vec<PluginInfo> {
    core::list_plugins(app)
}

#[tauri::command]
async fn preview_plugin(app: AppHandle, id: String) -> Result<Option<PluginPreview>, String> {
    let handle = app.clone();
    tauri::async_runtime::spawn_blocking(move || core::preview_plugin(handle, &id))
        .await
        .map_err(|e| format!("Не удалось запустить превью задачу: {e}"))?
}

#[tauri::command]
async fn run_plugin(app: AppHandle, id: String, settings: Value) -> Result<RunResult, String> {
    let handle = app.clone();
    tauri::async_runtime::spawn_blocking(move || core::run_plugin(handle, &id, settings))
        .await
        .map_err(|e| format!("Не удалось запустить задачу: {e}"))?
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            list_plugins,
            preview_plugin,
            run_plugin
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
