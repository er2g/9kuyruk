use axum::{
    extract::{State, Path},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::AppState;

#[derive(Serialize, Deserialize, Clone)]
pub struct Template {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub timeline_template: serde_json::Value,
}

#[derive(Deserialize)]
pub struct CreateTemplateRequest {
    pub name: String,
    pub description: String,
    pub timeline_template: serde_json::Value,
}

#[derive(Deserialize)]
pub struct ApplyTemplateRequest {
    pub project_id: Uuid,
    pub variables: serde_json::Value,
}

pub async fn list(State(state): State<AppState>) -> Result<Json<Vec<Template>>, StatusCode> {
    let templates_path = state.storage.get_path("templates", "");

    let mut templates = Vec::new();

    if let Ok(mut entries) = tokio::fs::read_dir(&templates_path).await {
        while let Ok(Some(entry)) = entries.next_entry().await {
            if let Ok(data) = tokio::fs::read(entry.path()).await {
                if let Ok(template) = serde_json::from_slice::<Template>(&data) {
                    templates.push(template);
                }
            }
        }
    }

    Ok(Json(templates))
}

pub async fn create(
    State(state): State<AppState>,
    Json(req): Json<CreateTemplateRequest>,
) -> Result<Json<Template>, StatusCode> {
    let template = Template {
        id: Uuid::new_v4(),
        name: req.name,
        description: req.description,
        timeline_template: req.timeline_template,
    };

    let data = serde_json::to_vec(&template)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    state.storage.save_file("templates", &format!("{}.json", template.id), &data).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(template))
}

pub async fn apply(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<ApplyTemplateRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let template_path = state.storage.get_path("templates", &format!("{}.json", id));

    let data = tokio::fs::read(&template_path).await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let template: Template = serde_json::from_slice(&data)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Simple variable substitution in timeline_template
    let mut applied_timeline = template.timeline_template.clone();

    // In production, implement proper template variable substitution
    // For now, return the template as-is with variables merged
    if let Some(timeline_obj) = applied_timeline.as_object_mut() {
        if let Some(vars) = req.variables.as_object() {
            timeline_obj.insert("applied_variables".to_string(), req.variables);
        }
    }

    Ok(Json(applied_timeline))
}
