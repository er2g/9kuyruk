use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::{db::{Settings, UpdateSettings}, AppState};

#[derive(Debug, Serialize)]
pub struct SettingsResponse {
    pub auto_pause_on_device_lost: bool,
    pub device_lost_timeout_seconds: i32,
    pub enable_device_locking: bool,
}

impl From<Settings> for SettingsResponse {
    fn from(settings: Settings) -> Self {
        Self {
            auto_pause_on_device_lost: settings.auto_pause_on_device_lost,
            device_lost_timeout_seconds: settings.device_lost_timeout_seconds,
            enable_device_locking: settings.enable_device_locking,
        }
    }
}

pub async fn get_settings(
    State(state): State<AppState>,
) -> Result<Json<SettingsResponse>, (StatusCode, String)> {
    let user_id = "temp_user"; // TODO: Get from auth

    let settings = Settings::get_or_create(&state.db, user_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(settings.into()))
}

pub async fn update_settings(
    State(state): State<AppState>,
    Json(updates): Json<UpdateSettings>,
) -> Result<Json<SettingsResponse>, (StatusCode, String)> {
    let user_id = "temp_user"; // TODO: Get from auth

    let settings = Settings::update(&state.db, user_id, updates)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(settings.into()))
}
