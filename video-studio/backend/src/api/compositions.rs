use axum::{
    extract::{State, Path},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::AppState;

#[derive(Serialize, Deserialize, Clone)]
pub struct Composition {
    pub id: Uuid,
    pub project_id: Uuid,
    pub name: String,
    pub timeline: serde_json::Value,
}

#[derive(Deserialize)]
pub struct CreateCompositionRequest {
    pub project_id: Uuid,
    pub name: String,
    pub timeline: serde_json::Value,
}

#[derive(Deserialize)]
pub struct UpdateCompositionRequest {
    pub name: Option<String>,
    pub timeline: Option<serde_json::Value>,
}

pub async fn create(
    State(state): State<AppState>,
    Json(req): Json<CreateCompositionRequest>,
) -> Result<Json<Composition>, StatusCode> {
    let composition = Composition {
        id: Uuid::new_v4(),
        project_id: req.project_id,
        name: req.name,
        timeline: req.timeline,
    };

    // Save composition to storage
    let data = serde_json::to_vec(&composition)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    state.storage.save_file("compositions", &format!("{}.json", composition.id), &data).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(composition))
}

pub async fn get(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Composition>, StatusCode> {
    let path = state.storage.get_path("compositions", &format!("{}.json", id));

    let data = tokio::fs::read(&path).await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let composition: Composition = serde_json::from_slice(&data)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(composition))
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateCompositionRequest>,
) -> Result<StatusCode, StatusCode> {
    let path = state.storage.get_path("compositions", &format!("{}.json", id));

    let data = tokio::fs::read(&path).await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let mut composition: Composition = serde_json::from_slice(&data)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(name) = req.name {
        composition.name = name;
    }
    if let Some(timeline) = req.timeline {
        composition.timeline = timeline;
    }

    let updated_data = serde_json::to_vec(&composition)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    tokio::fs::write(&path, updated_data).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}
