use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use crate::AppState;

#[derive(Serialize, Deserialize)]
pub struct Composition {
    pub id: String,
}

pub async fn create(State(_state): State<AppState>) -> Json<Composition> {
    Json(Composition { id: "1".to_string() })
}

pub async fn get(State(_state): State<AppState>) -> Json<Composition> {
    Json(Composition { id: "1".to_string() })
}

pub async fn update(State(_state): State<AppState>) -> &'static str {
    "Updated"
}
