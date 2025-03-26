mod commands;

use tauri::{generate_handler, webview::WebviewWindowBuilder, App};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize with plugins
    let mut builder = tauri::Builder::default().plugin(tauri_plugin_fs::init());

    // Setup
    builder = builder.setup(|app| {
        create_main_window(app)?;
        if cfg!(debug_assertions) {
            app.handle().plugin(
                tauri_plugin_log::Builder::default()
                    .level(log::LevelFilter::Info)
                    .build(),
            )?;
        }
        Ok(())
    });

    // Register commands
    builder = builder.invoke_handler(generate_handler![commands::google_auth_code]);

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
