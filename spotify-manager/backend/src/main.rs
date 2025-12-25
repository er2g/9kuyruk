mod api;
mod auth;
mod db;
mod device;
mod spotify;
mod websocket;

use axum::{
    routing::{get, post},
    Router,
};
use sqlx::sqlite::SqlitePool;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber;

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
    pub device_tracker: Arc<RwLock<device::DeviceTracker>>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Load environment variables
    dotenvy::dotenv().ok();

    // Initialize database
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:./spotify_manager.db".to_string());

    let db = SqlitePool::connect(&database_url).await?;
    sqlx::migrate!("./migrations").run(&db).await?;

    // Initialize device tracker
    let device_tracker = Arc::new(RwLock::new(device::DeviceTracker::new()));

    let state = AppState {
        db,
        device_tracker: device_tracker.clone(),
    };

    // Start device monitoring background task
    tokio::spawn(device::monitor_devices(device_tracker));

    // Build CORS layer
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build application routes
    let app = Router::new()
        // Health check
        .route("/health", get(|| async { "OK" }))

        // Authentication routes
        .route("/api/auth/login", post(auth::login))
        .route("/api/auth/register", post(auth::register))
        .route("/api/auth/refresh", post(auth::refresh_token))

        // Spotify OAuth routes
        .route("/api/spotify/auth", get(spotify::auth_url))
        .route("/api/spotify/callback", get(spotify::callback))
        .route("/api/spotify/status", get(spotify::get_status))
        .route("/api/spotify/disconnect", post(spotify::disconnect))

        // Spotify control routes
        .route("/api/spotify/player/play", post(spotify::player::play))
        .route("/api/spotify/player/pause", post(spotify::player::pause))
        .route("/api/spotify/player/next", post(spotify::player::next))
        .route("/api/spotify/player/previous", post(spotify::player::previous))
        .route("/api/spotify/player/devices", get(spotify::player::get_devices))
        .route("/api/spotify/player/transfer", post(spotify::player::transfer_playback))

        // Playlist routes
        .route("/api/spotify/playlists", get(spotify::playlists::list))
        .route("/api/spotify/playlists/:id", get(spotify::playlists::get))
        .route("/api/spotify/playlists/:id/tracks", get(spotify::playlists::get_tracks))

        // Device management routes
        .route("/api/devices", get(device::list_devices))
        .route("/api/devices/:id", get(device::get_device))
        .route("/api/devices/:id/lock", post(device::lock_device))
        .route("/api/devices/:id/unlock", post(device::unlock_device))
        .route("/api/devices/:id/set-primary", post(device::set_primary))

        // Settings routes
        .route("/api/settings", get(api::settings::get_settings))
        .route("/api/settings", post(api::settings::update_settings))

        // WebSocket for real-time updates
        .route("/ws", get(websocket::handler))

        .layer(cors)
        .with_state(state);

    // Get bind address from environment or use default
    let addr = std::env::var("BIND_ADDRESS")
        .unwrap_or_else(|_| "0.0.0.0:3000".to_string());

    tracing::info!("🚀 Server starting on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
