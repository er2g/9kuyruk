use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::AppState;
use super::get_access_token;

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaylistSimple {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub images: Vec<SpotifyImage>,
    pub tracks: PlaylistTracks,
    pub owner: PlaylistOwner,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpotifyImage {
    pub url: String,
    pub height: Option<i32>,
    pub width: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaylistTracks {
    pub total: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaylistOwner {
    pub id: String,
    pub display_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaylistsResponse {
    pub items: Vec<PlaylistSimple>,
    pub total: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Track {
    pub id: String,
    pub name: String,
    pub artists: Vec<Artist>,
    pub album: Album,
    pub duration_ms: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Artist {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Album {
    pub id: String,
    pub name: String,
    pub images: Vec<SpotifyImage>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaylistTrackItem {
    pub track: Track,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaylistTracksResponse {
    pub items: Vec<PlaylistTrackItem>,
    pub total: i32,
}

// List user's playlists
pub async fn list(
    State(state): State<AppState>,
) -> Result<Json<PlaylistsResponse>, (StatusCode, String)> {
    let user_id = "temp_user"; // TODO: Get from auth
    let access_token = get_access_token(&state.db, user_id)
        .await
        .map_err(|e| (StatusCode::UNAUTHORIZED, e))?;

    let client = Client::new();
    let response = client
        .get("https://api.spotify.com/v1/me/playlists?limit=50")
        .bearer_auth(&access_token)
        .send()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if !response.status().is_success() {
        return Err((
            StatusCode::from_u16(response.status().as_u16()).unwrap(),
            "Failed to fetch playlists".to_string(),
        ));
    }

    let playlists: PlaylistsResponse = response
        .json()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(playlists))
}

// Get a specific playlist
pub async fn get(
    State(state): State<AppState>,
    Path(playlist_id): Path<String>,
) -> Result<Json<PlaylistSimple>, (StatusCode, String)> {
    let user_id = "temp_user"; // TODO: Get from auth
    let access_token = get_access_token(&state.db, user_id)
        .await
        .map_err(|e| (StatusCode::UNAUTHORIZED, e))?;

    let client = Client::new();
    let response = client
        .get(&format!("https://api.spotify.com/v1/playlists/{}", playlist_id))
        .bearer_auth(&access_token)
        .send()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if !response.status().is_success() {
        return Err((
            StatusCode::from_u16(response.status().as_u16()).unwrap(),
            "Failed to fetch playlist".to_string(),
        ));
    }

    let playlist: PlaylistSimple = response
        .json()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(playlist))
}

// Get playlist tracks
pub async fn get_tracks(
    State(state): State<AppState>,
    Path(playlist_id): Path<String>,
) -> Result<Json<PlaylistTracksResponse>, (StatusCode, String)> {
    let user_id = "temp_user"; // TODO: Get from auth
    let access_token = get_access_token(&state.db, user_id)
        .await
        .map_err(|e| (StatusCode::UNAUTHORIZED, e))?;

    let client = Client::new();
    let response = client
        .get(&format!(
            "https://api.spotify.com/v1/playlists/{}/tracks?limit=50",
            playlist_id
        ))
        .bearer_auth(&access_token)
        .send()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if !response.status().is_success() {
        return Err((
            StatusCode::from_u16(response.status().as_u16()).unwrap(),
            "Failed to fetch playlist tracks".to_string(),
        ));
    }

    let tracks: PlaylistTracksResponse = response
        .json()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(tracks))
}
