use std::sync::Arc;
use tokio::sync::{RwLock, mpsc};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    pub id: Uuid,
    pub job_type: String,
    pub payload: serde_json::Value,
}

pub struct JobQueue {
    tx: mpsc::UnboundedSender<Job>,
}

impl JobQueue {
    pub fn new() -> (Self, mpsc::UnboundedReceiver<Job>) {
        let (tx, rx) = mpsc::unbounded_channel();
        (Self { tx }, rx)
    }

    pub fn enqueue(&self, job: Job) -> anyhow::Result<()> {
        self.tx.send(job)?;
        Ok(())
    }
}

/// Background job processor
pub async fn process_jobs(
    mut rx: mpsc::UnboundedReceiver<Job>,
    pool: PgPool,
) {
    while let Some(job) = rx.recv().await {
        tokio::spawn(process_single_job(job, pool.clone()));
    }
}

async fn process_single_job(job: Job, pool: PgPool) {
    tracing::info!("Processing job: {:?}", job.id);

    match job.job_type.as_str() {
        "render" => {
            if let Err(e) = process_render_job(&job, &pool).await {
                tracing::error!("Render job failed: {}", e);
            }
        }
        "proxy" => {
            if let Err(e) = process_proxy_job(&job, &pool).await {
                tracing::error!("Proxy job failed: {}", e);
            }
        }
        _ => {
            tracing::warn!("Unknown job type: {}", job.job_type);
        }
    }
}

async fn process_render_job(job: &Job, pool: &PgPool) -> anyhow::Result<()> {
    use crate::db::RenderJob;
    use crate::video::{render_composition, RenderOptions};

    let job_id = job.payload["job_id"].as_str()
        .and_then(|s| Uuid::parse_str(s).ok())
        .ok_or_else(|| anyhow::anyhow!("Invalid job_id"))?;

    // Update status to processing
    RenderJob::update_status(pool, job_id, "processing", 0.0).await?;

    // Get job from database
    let render_job = RenderJob::find_by_id(pool, job_id).await?
        .ok_or_else(|| anyhow::anyhow!("Job not found"))?;

    // Render
    let output_path = format!("/tmp/render_{}.mp4", job_id);
    let options = RenderOptions {
        output_path: output_path.clone(),
        width: 1920,
        height: 1080,
        fps: 30.0,
        quality: "preview".to_string(),
    };

    match render_composition(&render_job.composition_data, &options).await {
        Ok(_) => {
            RenderJob::set_output(pool, job_id, &output_path).await?;
            RenderJob::update_status(pool, job_id, "completed", 100.0).await?;
            tracing::info!("Render job completed: {}", job_id);
        }
        Err(e) => {
            RenderJob::set_error(pool, job_id, &e.to_string()).await?;
            tracing::error!("Render job failed: {}", e);
        }
    }

    Ok(())
}

async fn process_proxy_job(_job: &Job, _pool: &PgPool) -> anyhow::Result<()> {
    // TODO: Implement proxy generation job
    Ok(())
}
