mod api;
mod auth;
mod db;
mod video;
mod jobs;
mod storage;
mod templates;

use axum::{routing::{get, post}, Router};
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::{cors::CorsLayer, compression::CompressionLayer};

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub redis: redis::Client,
    pub storage: Arc<storage::StorageBackend>,
    pub job_queue: Arc<RwLock<jobs::JobQueue>>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")?;
    let db = PgPool::connect(&database_url).await?;
    sqlx::migrate!("./migrations").run(&db).await?;

    let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost".to_string());
    let redis = redis::Client::open(redis_url)?;

    let storage = Arc::new(storage::StorageBackend::new().await?);
    let job_queue = Arc::new(RwLock::new(jobs::JobQueue::new()));

    let state = AppState { db, redis, storage, job_queue: job_queue.clone() };

    // Start background job processor
    tokio::spawn(jobs::process_jobs(job_queue));

    let app = Router::new()
        .route("/health", get(|| async { "OK" }))

        // Auth
        .route("/api/auth/register", post(auth::register))
        .route("/api/auth/login", post(auth::login))

        // Projects
        .route("/api/projects", get(api::projects::list).post(api::projects::create))
        .route("/api/projects/:id", get(api::projects::get).delete(api::projects::delete))

        // Assets (video/image upload)
        .route("/api/assets/upload", post(api::assets::upload))
        .route("/api/assets", get(api::assets::list))

        // Timeline/Composition
        .route("/api/compositions", post(api::compositions::create))
        .route("/api/compositions/:id", get(api::compositions::get).put(api::compositions::update))

        // Rendering
        .route("/api/render", post(api::render::start))
        .route("/api/render/:job_id/status", get(api::render::status))

        // Templates
        .route("/api/templates", get(api::templates::list).post(api::templates::create))
        .route("/api/templates/:id/apply", post(api::templates::apply))

        // WebSocket for real-time updates
        .route("/ws", get(api::websocket::handler))

        .layer(CorsLayer::permissive())
        .layer(CompressionLayer::new())
        .with_state(state);

    let addr = std::env::var("BIND_ADDRESS").unwrap_or_else(|_| "0.0.0.0:3000".to_string());
    tracing::info!("🎬 Video Studio API starting on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
