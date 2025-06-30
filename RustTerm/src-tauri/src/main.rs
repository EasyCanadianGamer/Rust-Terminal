#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use futures_util::{SinkExt, StreamExt};
use std::net::SocketAddr;
use tauri::Manager;
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
mod commands;

#[tokio::main]
async fn main() {
    // Spawn WebSocket server task
    tokio::spawn(async {
        let addr = "127.0.0.1:8080";
        let listener = TcpListener::bind(&addr).await.expect("Failed to bind");

        println!("WebSocket server running on ws://{}", addr);

        while let Ok((stream, _)) = listener.accept().await {
            tokio::spawn(async move {
                let ws_stream = accept_async(stream)
                    .await
                    .expect("WebSocket handshake failed");

                let (mut write, mut read) = ws_stream.split();

                while let Some(msg) = read.next().await {
                    match msg {
                        Ok(tungstenite::Message::Text(input)) => {

                            // Process command
                            let result = run_command(&input);

                            // Send response
                            if let Err(e) = write.send(tungstenite::Message::Text(result)).await {
                                eprintln!("Send error: {}", e);
                                break;
                            }
                        }
                        Ok(_) => {}
                        Err(e) => {
                            eprintln!("Error: {}", e);
                            break;
                        }
                    }
                }
            });
        }
    });

    // Start Tauri normally
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri app");
}

// Mocked command execution (replace with your logic)
fn run_command(input: &str) -> String {
    commands::execute_command(input)
}
