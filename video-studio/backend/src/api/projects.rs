use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use crate::AppState;

#[derive(Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
}

pub async fn list(State(_state): State<AppState>) -> Json<Vec<Project>> {
    Json(vec![])
}

pub async fn create(State(_state): State<AppState>) -> Json<Project> {
    Json(Project { id: "1".to_string(), name: "New Project".to_string() })
}

pub async fn get(State(_state): State<AppState>) -> Json<Project> {
    Json(Project { id: "1".to_string(), name: "Project".to_string() })
}

pub async fn delete(State(_state): State<AppState>) -> &'static str {
    "OK"
}
