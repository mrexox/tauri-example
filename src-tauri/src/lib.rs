use tauri::{webview::WebviewWindowBuilder, App};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            create_main_window(app)?;
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
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
