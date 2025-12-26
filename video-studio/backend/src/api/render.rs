use axum::{
    extract::{State, Path},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::{AppState, db::RenderJob as DbRenderJob, jobs::Job};

#[derive(Deserialize)]
pub struct StartRenderRequest {
    pub project_id: Uuid,
    pub composition_data: serde_json::Value,
}

#[derive(Serialize)]
pub struct RenderJobResponse {
    pub job_id: Uuid,
    pub status: String,
    pub progress: f32,
    pub output_path: Option<String>,
    pub error_message: Option<String>,
}

impl From<DbRenderJob> for RenderJobResponse {
    fn from(job: DbRenderJob) -> Self {
        Self {
            job_id: job.id,
            status: job.status,
            progress: job.progress,
            output_path: job.output_path,
            error_message: job.error_message,
        }
    }
}

pub async fn start(
    State(state): State<AppState>,
    Json(req): Json<StartRenderRequest>,
) -> Result<Json<RenderJobResponse>, StatusCode> {
    // Create render job in database
    let render_job = DbRenderJob::create(&state.db, req.project_id, req.composition_data).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Enqueue job for background processing
    let job = Job {
        id: Uuid::new_v4(),
        job_type: "render".to_string(),
        payload: serde_json::json!({
            "job_id": render_job.id.to_string(),
        }),
    };

    state.job_queue.read().await.enqueue(job)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(RenderJobResponse::from(render_job)))
}

pub async fn status(
    State(state): State<AppState>,
    Path(job_id): Path<Uuid>,
) -> Result<Json<RenderJobResponse>, StatusCode> {
    let render_job = DbRenderJob::find_by_id(&state.db, job_id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(RenderJobResponse::from(render_job)))
}
