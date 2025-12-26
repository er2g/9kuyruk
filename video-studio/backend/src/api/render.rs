use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use crate::AppState;

#[derive(Serialize, Deserialize)]
pub struct RenderJob {
    pub job_id: String,
    pub status: String,
}

pub async fn start(State(_state): State<AppState>) -> Json<RenderJob> {
    Json(RenderJob { job_id: "1".to_string(), status: "started".to_string() })
}

pub async fn status(State(_state): State<AppState>) -> Json<RenderJob> {
    Json(RenderJob { job_id: "1".to_string(), status: "processing".to_string() })
}
