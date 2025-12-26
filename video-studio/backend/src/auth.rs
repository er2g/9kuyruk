use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use crate::AppState;

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
}

pub async fn register(State(_state): State<AppState>, Json(_req): Json<RegisterRequest>) -> Json<AuthResponse> {
    Json(AuthResponse { token: "token".to_string() })
}

pub async fn login(State(_state): State<AppState>, Json(_req): Json<RegisterRequest>) -> Json<AuthResponse> {
    Json(AuthResponse { token: "token".to_string() })
}
