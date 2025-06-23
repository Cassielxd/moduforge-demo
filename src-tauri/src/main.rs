// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use app_lib::{initialize::init_contex, router::build_app, serve::AppBuilder};
use axum::{http::StatusCode, response::IntoResponse, Router};
use moduforge_state::init_logging;
use tauri::Manager;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_logging("info", Some("logs/demo.log")).unwrap();
    init_contex().await;
    tokio::spawn(async move {
        let app: Router = build_app();
        AppBuilder::new()
            .next("/api", app.fallback(handler_404))
            .build()
            .run()
            .await;
    });

    tauri::Builder::default()
        .setup(|app| {
            let main = app.handle().get_webview_window("main").unwrap();
            main.open_devtools();
            Ok(())
        })
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .run(tauri::generate_context!())?;
    Ok(())
}
async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}
