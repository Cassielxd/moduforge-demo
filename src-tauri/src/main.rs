// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use app_lib::{initialize::init_contex, router::build_app, serve::AppBuilder};
use axum::{http::StatusCode, response::IntoResponse, Router};
use moduforge_state::init_logging;
use tauri::Manager;

// 自定义事件处理函数
fn handle_tauri_error(error: tauri::Error) {
    eprintln!("Tauri错误: {:?}", error);
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化日志系统，降低tao警告级别
    init_logging("warn", Some("logs/demo.log")).unwrap();

    // 设置tao相关的日志级别为error，减少警告信息
    std::env::set_var("RUST_LOG", "tao=error,tauri=info,app=info");

    // 初始化上下文
    init_contex().await;

    // 启动API服务器
    tokio::spawn(async move {
        let app: Router = build_app();
        AppBuilder::new()
            .next("/api", app.fallback(handler_404))
            .build()
            .run()
            .await;
    });

    // 配置Tauri应用
    tauri::Builder::default()
        .setup(|app| {
            // 在开发模式下打开开发者工具
            #[cfg(debug_assertions)]
            {
                if let Some(main_window) = app.handle().get_webview_window("main") {
                    main_window.open_devtools();
                }
            }
            Ok(())
        })
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .run(tauri::generate_context!())
        .map_err(|e| {
            handle_tauri_error(e);
            anyhow::anyhow!("Tauri应用启动失败")
        })?;

    Ok(())
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}
