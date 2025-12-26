use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use crate::AppState;

#[derive(Serialize, Deserialize)]
pub struct Asset {
    pub id: String,
    pub name: String,
}

pub async fn upload(State(_state): State<AppState>) -> &'static str {
    "Uploaded"
}

pub async fn list(State(_state): State<AppState>) -> Json<Vec<Asset>> {
    Json(vec![])
}
