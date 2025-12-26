use axum::{
    extract::{State, ws::{WebSocket, WebSocketUpgrade, Message}},
    response::Response,
};
use futures::{sink::SinkExt, stream::StreamExt};
use crate::AppState;

pub async fn handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: AppState) {
    let (mut sender, mut receiver) = socket.split();

    // Spawn a task to send periodic updates
    let state_clone = state.clone();
    let mut send_task = tokio::spawn(async move {
        // Send periodic render job status updates
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(1));

        loop {
            interval.tick().await;

            // In production, maintain list of active jobs and only send updates for those
            let update = serde_json::json!({
                "type": "heartbeat",
                "timestamp": chrono::Utc::now().to_rfc3339(),
            });

            if sender.send(Message::Text(update.to_string())).await.is_err() {
                break;
            }
        }
    });

    // Receive messages from client
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if let Message::Text(text) = msg {
                tracing::debug!("Received WebSocket message: {}", text);
                // Handle client commands (subscribe to job, unsubscribe, etc.)
            }
        }
    });

    // Wait for either task to finish
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    }
}
