use log::{debug, info};

use tauri::AppHandle;
use tauri_plugin_shell::process::CommandEvent;
use tauri_plugin_shell::ShellExt;

use std::sync::Arc;
use tokio::io::{AsyncWriteExt, WriteHalf};
use tokio::net::TcpStream;
use tokio::sync::mpsc::unbounded_channel;
use tokio::sync::Mutex;

#[derive(Debug)]
pub(crate) struct Client {
    writer: Arc<Mutex<WriteHalf<TcpStream>>>,
}

impl Client {
    pub(crate) async fn write(&self, s: String) {
        self.writer
            .lock()
            .await
            .write_all((s + "\n").as_bytes()) // "\n" is the messages divider
            .await
            .expect("can't write data to TCP socket");
    }
}

pub(crate) async fn spawn(app_handle: &AppHandle) -> u32 {
    info!("Spawning sidecar process");
    let command = app_handle
        .shell()
        .sidecar("sidecar")
        .expect("couldn't get sidecar executable");

    let (port_tx, mut port_rx) = unbounded_channel::<u32>();
    let (mut rx, mut _child) = command.spawn().expect("failed to spawn sidecar");

    tauri::async_runtime::spawn(async move {
        let mut port_parsed = false;
        while let Some(event) = rx.recv().await {
            match event {
                // Receive only port number from STDOUT
                CommandEvent::Stdout(bytes) => {
                    if !port_tx.is_closed() && !port_parsed {
                        if let Ok(port) = String::from_utf8_lossy(&bytes).trim().parse() {
                            port_tx.send(port).expect("unable to send port");
                            port_parsed = true;
                            continue;
                        }
                    }
                    debug!("{}", String::from_utf8_lossy(&bytes).trim());
                }
                _ => {}
            }
        }
    });

    let port = port_rx
        .recv()
        .await
        .expect("couldn't receive daemon port number");
    port_rx.close();

    port
}

pub(crate) async fn connect(port: u32) -> Client {
    info!("Connecting to a sidecar at port {}", port);
    let stream = TcpStream::connect(format!("127.0.0.1:{}", port))
        .await
        .expect("couldn't connect to a sidecar socket");

    let (_read_stream, write_stream) = tokio::io::split(stream);

    Client {
        writer: Arc::new(Mutex::new(write_stream)),
    }
}
