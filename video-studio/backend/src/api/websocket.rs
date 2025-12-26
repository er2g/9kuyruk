use axum::extract::State;
use crate::AppState;

pub async fn handler(State(_state): State<AppState>) -> &'static str {
    "WebSocket endpoint"
}
