use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::AppState;
use super::get_access_token;

#[derive(Debug, Serialize, Deserialize)]
pub struct SpotifyDevice {
    pub id: String,
    pub is_active: bool,
    pub is_private_session: bool,
    pub is_restricted: bool,
    pub name: String,
    #[serde(rename = "type")]
    pub device_type: String,
    pub volume_percent: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct DevicesResponse {
    pub devices: Vec<SpotifyDevice>,
}

#[derive(Debug, Deserialize)]
pub struct TransferPlaybackRequest {
    pub device_id: String,
    pub play: Option<bool>,
}

// Get available devices
pub async fn get_devices(
    State(state): State<AppState>,
) -> Result<Json<DevicesResponse>, (StatusCode, String)> {
    let user_id = "temp_user"; // TODO: Get from auth
    let access_token = get_access_token(&state.db, user_id)
        .await
        .map_err(|e| (StatusCode::UNAUTHORIZED, e))?;

    let client = Client::new();
    let response = client
        .get("https://api.spotify.com/v1/me/player/devices")
        .bearer_auth(&access_token)
        .send()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if !response.status().is_success() {
        return Err((
            StatusCode::from_u16(response.status().as_u16()).unwrap(),
            "Failed to get devices".to_string(),
        ));
    }

    let devices_response: DevicesResponse = response
        .json()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(devices_response))
}

// Transfer playback to a device
pub async fn transfer_playback(
    State(state): State<AppState>,
    Json(req): Json<TransferPlaybackRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    let user_id = "temp_user"; // TODO: Get from auth
    let access_token = get_access_token(&state.db, user_id)
        .await
        .map_err(|e| (StatusCode::UNAUTHORIZED, e))?;

    let client = Client::new();
    let body = serde_json::json!({
        "device_ids": [req.device_id],
        "play": req.play.unwrap_or(false),
    });

    let response = client
        .put("https://api.spotify.com/v1/me/player")
        .bearer_auth(&access_token)
        .json(&body)
        .send()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if !response.status().is_success() {
        return Err((
            StatusCode::from_u16(response.status().as_u16()).unwrap(),
            "Failed to transfer playback".to_string(),
        ));
    }

    Ok(StatusCode::OK)
}

// Play
pub async fn play(
    State(state): State<AppState>,
) -> Result<StatusCode, (StatusCode, String)> {
    let user_id = "temp_user"; // TODO: Get from auth
    let access_token = get_access_token(&state.db, user_id)
        .await
        .map_err(|e| (StatusCode::UNAUTHORIZED, e))?;

    let client = Client::new();
    let response = client
        .put("https://api.spotify.com/v1/me/player/play")
        .bearer_auth(&access_token)
        .send()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if !response.status().is_success() {
        return Err((
            StatusCode::from_u16(response.status().as_u16()).unwrap(),
            "Failed to play".to_string(),
        ));
    }

    Ok(StatusCode::OK)
}

// Pause
pub async fn pause(
    State(state): State<AppState>,
) -> Result<StatusCode, (StatusCode, String)> {
    let user_id = "temp_user"; // TODO: Get from auth
    let access_token = get_access_token(&state.db, user_id)
        .await
        .map_err(|e| (StatusCode::UNAUTHORIZED, e))?;

    let client = Client::new();
    let response = client
        .put("https://api.spotify.com/v1/me/player/pause")
        .bearer_auth(&access_token)
        .send()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if !response.status().is_success() {
        return Err((
            StatusCode::from_u16(response.status().as_u16()).unwrap(),
            "Failed to pause".to_string(),
        ));
    }

    Ok(StatusCode::OK)
}

// Next track
pub async fn next(
    State(state): State<AppState>,
) -> Result<StatusCode, (StatusCode, String)> {
    let user_id = "temp_user"; // TODO: Get from auth
    let access_token = get_access_token(&state.db, user_id)
        .await
        .map_err(|e| (StatusCode::UNAUTHORIZED, e))?;

    let client = Client::new();
    let response = client
        .post("https://api.spotify.com/v1/me/player/next")
        .bearer_auth(&access_token)
        .send()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if !response.status().is_success() {
        return Err((
            StatusCode::from_u16(response.status().as_u16()).unwrap(),
            "Failed to skip to next".to_string(),
        ));
    }

    Ok(StatusCode::OK)
}

// Previous track
pub async fn previous(
    State(state): State<AppState>,
) -> Result<StatusCode, (StatusCode, String)> {
    let user_id = "temp_user"; // TODO: Get from auth
    let access_token = get_access_token(&state.db, user_id)
        .await
        .map_err(|e| (StatusCode::UNAUTHORIZED, e))?;

    let client = Client::new();
    let response = client
        .post("https://api.spotify.com/v1/me/player/previous")
        .bearer_auth(&access_token)
        .send()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if !response.status().is_success() {
        return Err((
            StatusCode::from_u16(response.status().as_u16()).unwrap(),
            "Failed to skip to previous".to_string(),
        ));
    }

    Ok(StatusCode::OK)
}
