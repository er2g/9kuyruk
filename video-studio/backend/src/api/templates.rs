use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use crate::AppState;

#[derive(Serialize, Deserialize)]
pub struct Template {
    pub id: String,
    pub name: String,
}

pub async fn list(State(_state): State<AppState>) -> Json<Vec<Template>> {
    Json(vec![])
}

pub async fn create(State(_state): State<AppState>) -> Json<Template> {
    Json(Template { id: "1".to_string(), name: "Template".to_string() })
}

pub async fn apply(State(_state): State<AppState>) -> &'static str {
    "Applied"
}
