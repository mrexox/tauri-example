pub mod atomic_once_cell;
mod commands;
mod sentry;
mod sidecar;

use atomic_once_cell::AtomicOnceCell;
use tauri::plugin::TauriPlugin;
use tauri::{generate_handler, webview::WebviewWindowBuilder, App};
use tauri::{Manager, Runtime};

#[macro_use]
extern crate dotenvy_macro;

pub(crate) struct AppState {
    sidecar_client: AtomicOnceCell<sidecar::Client>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let sentry_client = sentry::setup();

    // Initialize with plugins
    let mut builder = tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_sentry::init_with_no_injection(&sentry_client));

    // Setup
    builder = builder.setup(|app| {
        create_main_window(app)?;
        if cfg!(debug_assertions) {
            app.handle().plugin(build_log_plugin())?;
        }

        let app_handle_copy = app.handle().clone();

        let client_cell = AtomicOnceCell::new();
        app_handle_copy.manage(AppState {
            sidecar_client: client_cell.clone(),
        });

        tauri::async_runtime::spawn(async move {
            let port = sidecar::spawn(&app_handle_copy).await;
            let sidecar_client = sidecar::connect(port).await;
            client_cell
                .init(sidecar_client)
                .expect("failed to init sidecar client");
        });

        Ok(())
    });

    // Register commands
    builder = builder.invoke_handler(generate_handler![
        commands::sidecar_send,
        commands::google_auth_code,
        commands::first_image_path,
    ]);

    // Run the app
    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn create_main_window(app: &mut App) -> tauri::Result<()> {
    let _ = WebviewWindowBuilder::new(app, "main", Default::default())
        .disable_drag_drop_handler()
        .resizable(true)
        .inner_size(1100.0, 800.0)
        .focused(true)
        .title("")
        .min_inner_size(600.0, 400.0)
        .build()?;

    Ok(())
}

fn build_log_plugin<R: Runtime>() -> TauriPlugin<R> {
    tauri_plugin_log::Builder::default()
        .level(log::LevelFilter::Debug)
        .build()
}
