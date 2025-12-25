use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::Response,
};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;

use crate::AppState;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WsMessage {
    DeviceStatusUpdate {
        device_id: String,
        is_active: bool,
        is_locked: bool,
    },
    PlaybackStateChanged {
        is_playing: bool,
        device_name: Option<String>,
    },
    DeviceLost {
        device_id: String,
        device_name: String,
    },
    SystemPaused {
        reason: String,
    },
    Ping,
    Pong,
}

pub async fn handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: AppState) {
    let (mut sender, mut receiver) = socket.split();

    // TODO: Create a broadcast channel for this user
    // For now, we'll just echo messages

    let mut send_task = tokio::spawn(async move {
        // Send periodic pings
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(30));

        loop {
            interval.tick().await;

            let ping = WsMessage::Ping;
            let msg = serde_json::to_string(&ping).unwrap();

            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Text(text) => {
                    // Handle incoming messages
                    if let Ok(ws_msg) = serde_json::from_str::<WsMessage>(&text) {
                        match ws_msg {
                            WsMessage::Ping => {
                                // Respond with pong
                                tracing::debug!("Received ping");
                            }
                            _ => {
                                tracing::debug!("Received message: {:?}", ws_msg);
                            }
                        }
                    }
                }
                Message::Close(_) => {
                    tracing::info!("WebSocket connection closed");
                    break;
                }
                _ => {}
            }
        }
    });

    // Wait for either task to finish
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    }

    tracing::info!("WebSocket connection terminated");
}
