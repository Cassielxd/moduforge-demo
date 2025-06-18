// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[tokio::main]
async fn main()->anyhow::Result<()> {
  tauri::Builder::default()
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_global_shortcut::Builder::new().build())
    .run(tauri::generate_context!())?;
  Ok(())
}
