pub mod player;
pub mod playlists;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::{db::SpotifyConnection, AppState};

// OAuth Configuration
fn get_client_id() -> String {
    std::env::var("SPOTIFY_CLIENT_ID").expect("SPOTIFY_CLIENT_ID must be set")
}

fn get_client_secret() -> String {
    std::env::var("SPOTIFY_CLIENT_SECRET").expect("SPOTIFY_CLIENT_SECRET must be set")
}

fn get_redirect_uri() -> String {
    std::env::var("SPOTIFY_REDIRECT_URI")
        .unwrap_or_else(|_| "https://yourdomain/api/spotify/callback".to_string())
}

// Request/Response types
#[derive(Debug, Serialize)]
pub struct AuthUrlResponse {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct CallbackQuery {
    pub code: String,
    pub state: Option<String>,
}

#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
    token_type: String,
    scope: String,
    expires_in: i64,
    refresh_token: String,
}

#[derive(Debug, Serialize)]
pub struct SpotifyStatusResponse {
    pub connected: bool,
    pub spotify_user_id: Option<String>,
    pub expires_at: Option<String>,
}

// Generate Spotify OAuth URL
pub async fn auth_url() -> Json<AuthUrlResponse> {
    let client_id = get_client_id();
    let redirect_uri = get_redirect_uri();
    let scopes = vec![
        "user-read-private",
        "user-read-email",
        "user-read-playback-state",
        "user-modify-playback-state",
        "user-read-currently-playing",
        "playlist-read-private",
        "playlist-read-collaborative",
        "playlist-modify-public",
        "playlist-modify-private",
    ]
    .join(" ");

    let url = format!(
        "https://accounts.spotify.com/authorize?client_id={}&response_type=code&redirect_uri={}&scope={}&show_dialog=true",
        client_id,
        urlencoding::encode(&redirect_uri),
        urlencoding::encode(&scopes)
    );

    Json(AuthUrlResponse { url })
}

// Handle OAuth callback
pub async fn callback(
    State(state): State<AppState>,
    Query(params): Query<CallbackQuery>,
) -> Result<String, (StatusCode, String)> {
    // TODO: Get user_id from session or state parameter
    // For now, this is a simplified version
    let user_id = "temp_user"; // This should come from session/state

    // Exchange code for tokens
    let client = Client::new();
    let token_response: TokenResponse = client
        .post("https://accounts.spotify.com/api/token")
        .form(&[
            ("grant_type", "authorization_code"),
            ("code", &params.code),
            ("redirect_uri", &get_redirect_uri()),
            ("client_id", &get_client_id()),
            ("client_secret", &get_client_secret()),
        ])
        .send()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .json()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Check if connection exists
    if let Ok(Some(_)) = SpotifyConnection::find_by_user_id(&state.db, user_id).await {
        // Update existing connection
        SpotifyConnection::update_tokens(
            &state.db,
            user_id,
            &token_response.access_token,
            &token_response.refresh_token,
            token_response.expires_in,
        )
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    } else {
        // Create new connection
        SpotifyConnection::create(
            &state.db,
            user_id,
            &token_response.access_token,
            &token_response.refresh_token,
            token_response.expires_in,
        )
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    }

    // Redirect to success page
    Ok("Spotify connected successfully! You can close this window.".to_string())
}

// Get Spotify connection status
pub async fn get_status(
    State(state): State<AppState>,
) -> Result<Json<SpotifyStatusResponse>, (StatusCode, String)> {
    // TODO: Get user_id from auth middleware
    let user_id = "temp_user";

    let connection = SpotifyConnection::find_by_user_id(&state.db, user_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if let Some(conn) = connection {
        Ok(Json(SpotifyStatusResponse {
            connected: true,
            spotify_user_id: conn.spotify_user_id,
            expires_at: Some(conn.expires_at.to_rfc3339()),
        }))
    } else {
        Ok(Json(SpotifyStatusResponse {
            connected: false,
            spotify_user_id: None,
            expires_at: None,
        }))
    }
}

// Disconnect Spotify
pub async fn disconnect(
    State(state): State<AppState>,
) -> Result<StatusCode, (StatusCode, String)> {
    // TODO: Get user_id from auth middleware
    let user_id = "temp_user";

    SpotifyConnection::delete_by_user_id(&state.db, user_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::OK)
}

// Helper function to get valid access token
pub async fn get_access_token(
    pool: &sqlx::SqlitePool,
    user_id: &str,
) -> Result<String, String> {
    let mut connection = SpotifyConnection::find_by_user_id(pool, user_id)
        .await
        .map_err(|e| e.to_string())?
        .ok_or("Not connected to Spotify")?;

    // Check if token is expired
    let now = chrono::Utc::now();
    if connection.expires_at <= now {
        // Refresh token
        let client = Client::new();
        let token_response: TokenResponse = client
            .post("https://accounts.spotify.com/api/token")
            .form(&[
                ("grant_type", "refresh_token"),
                ("refresh_token", &connection.refresh_token),
                ("client_id", &get_client_id()),
                ("client_secret", &get_client_secret()),
            ])
            .send()
            .await
            .map_err(|e| e.to_string())?
            .json()
            .await
            .map_err(|e| e.to_string())?;

        // Update tokens in database
        SpotifyConnection::update_tokens(
            pool,
            user_id,
            &token_response.access_token,
            &token_response.refresh_token,
            token_response.expires_in,
        )
        .await
        .map_err(|e| e.to_string())?;

        Ok(token_response.access_token)
    } else {
        Ok(connection.access_token)
    }
}
