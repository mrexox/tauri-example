use log::debug;
use url::Url;

use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver, Sender};

use tauri::{AppHandle, WebviewUrl, WebviewWindowBuilder};

use dotenvy_macro::dotenv;

const CLIENT_ID: &str = dotenv!("GOOGLE_DESKTOP_OAUTH_CLIENT_ID");
const CALLBACK_URL: &str = "http://localhost/callback";

#[tauri::command]
pub(crate) async fn sidecar_send(
    state: tauri::State<'_, crate::AppState>,
    message: String,
) -> Result<(), String> {
    debug!("sending message: {}", &message);
    state.sidecar_client.get().await.write(message).await;

    Ok(())
}

#[tauri::command]
pub(crate) async fn google_auth_code(app: AppHandle) -> Result<(String, String), String> {
    let auth_url = Url::parse_with_params(
        "https://accounts.google.com/o/oauth2/v2/auth",
        &[
            ("client_id", CLIENT_ID),
            ("redirect_uri", CALLBACK_URL),
            ("response_type", "code"),
            ("scope", "openid profile email"),
        ],
    )
    .unwrap();

    let (tx, rx): (Sender<String>, Receiver<String>) = channel();
    let (err_tx, err_rx): (Sender<String>, Receiver<String>) = channel();
    let auth_window = WebviewWindowBuilder::new(&app, "auth", WebviewUrl::External(auth_url))
        .center()
        .resizable(true)
        .inner_size(600.0, 700.0)
        .focused(true)
        .title("Authenticate")
        .on_navigation(move |url| {
            if url.scheme() == "http"
                && url.host() == Some(url::Host::Domain("localhost"))
                && url.path() == "/callback"
            {
                let query: HashMap<_, _> = url.query_pairs().into_owned().collect();
                if let Some(code) = query.get("code") {
                    tx.send(code.to_string()).expect("couldn't send auth code");
                    err_tx
                        .send("".to_string())
                        .expect("couldn't send empty error");
                } else {
                    err_tx
                        .send("failed to parse the code".to_string())
                        .expect("couldn't send error message");
                }

                return false;
            }

            true
        })
        .build()
        .expect("can't open an auth window");

    let error = err_rx.recv().map_err(|err| err.to_string())?;
    if !error.is_empty() {
        let _ = auth_window.close();
        return Err(error);
    }

    let res = rx.recv().map_err(|err| err.to_string())?;

    let _ = auth_window.close();

    Ok((res, CALLBACK_URL.to_string()))
}

#[tauri::command]
pub(crate) async fn first_image_path(subdir: String) -> Result<Option<String>, String> {
    debug!("looking for an image in HOME");

    let mut dir = dirs::home_dir().ok_or("failed to get home dir")?;
    dir.push(subdir);

    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            // Check if the entry is a file and has a .jpg or .jpeg extension
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if extension == "jpg" || extension == "jpeg" {
                        return Ok(Some(path.to_string_lossy().to_string()));
                    }
                }
            }
        }
    }

    Ok(None)
}
