use sqlx::{PgPool, FromRow};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use anyhow::Result;

// ============================================================================
// User Model
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}

impl User {
    pub async fn create(pool: &PgPool, email: &str, password_hash: &str) -> Result<Self> {
        let user = sqlx::query_as::<_, User>(
            "INSERT INTO users (email, password_hash) VALUES ($1, $2) RETURNING *"
        )
        .bind(email)
        .bind(password_hash)
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<Option<Self>> {
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE email = $1"
        )
        .bind(email)
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Self>> {
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }
}

// ============================================================================
// Project Model
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Project {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Project {
    pub async fn create(pool: &PgPool, user_id: Uuid, name: &str, description: Option<&str>) -> Result<Self> {
        let project = sqlx::query_as::<_, Project>(
            "INSERT INTO projects (user_id, name, description) VALUES ($1, $2, $3) RETURNING *"
        )
        .bind(user_id)
        .bind(name)
        .bind(description)
        .fetch_one(pool)
        .await?;

        Ok(project)
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Self>> {
        let project = sqlx::query_as::<_, Project>(
            "SELECT * FROM projects WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        Ok(project)
    }

    pub async fn list_by_user(pool: &PgPool, user_id: Uuid) -> Result<Vec<Self>> {
        let projects = sqlx::query_as::<_, Project>(
            "SELECT * FROM projects WHERE user_id = $1 ORDER BY updated_at DESC"
        )
        .bind(user_id)
        .fetch_all(pool)
        .await?;

        Ok(projects)
    }

    pub async fn update(&self, pool: &PgPool) -> Result<()> {
        sqlx::query(
            "UPDATE projects SET name = $1, description = $2, updated_at = NOW() WHERE id = $3"
        )
        .bind(&self.name)
        .bind(&self.description)
        .bind(self.id)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM projects WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;

        Ok(())
    }
}

// ============================================================================
// Asset Model
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Asset {
    pub id: Uuid,
    pub project_id: Uuid,
    pub asset_type: String,
    pub filename: String,
    pub file_path: String,
    pub file_size: i64,
    pub duration: Option<f64>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub fps: Option<f64>,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

impl Asset {
    pub async fn create(
        pool: &PgPool,
        project_id: Uuid,
        asset_type: &str,
        filename: &str,
        file_path: &str,
        file_size: i64,
    ) -> Result<Self> {
        let asset = sqlx::query_as::<_, Asset>(
            r#"INSERT INTO assets (project_id, asset_type, filename, file_path, file_size)
               VALUES ($1, $2, $3, $4, $5) RETURNING *"#
        )
        .bind(project_id)
        .bind(asset_type)
        .bind(filename)
        .bind(file_path)
        .bind(file_size)
        .fetch_one(pool)
        .await?;

        Ok(asset)
    }

    pub async fn update_metadata(
        pool: &PgPool,
        id: Uuid,
        duration: Option<f64>,
        width: Option<i32>,
        height: Option<i32>,
        fps: Option<f64>,
    ) -> Result<()> {
        sqlx::query(
            r#"UPDATE assets SET duration = $1, width = $2, height = $3, fps = $4 WHERE id = $5"#
        )
        .bind(duration)
        .bind(width)
        .bind(height)
        .bind(fps)
        .bind(id)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Self>> {
        let asset = sqlx::query_as::<_, Asset>("SELECT * FROM assets WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await?;

        Ok(asset)
    }

    pub async fn list_by_project(pool: &PgPool, project_id: Uuid) -> Result<Vec<Self>> {
        let assets = sqlx::query_as::<_, Asset>(
            "SELECT * FROM assets WHERE project_id = $1 ORDER BY created_at DESC"
        )
        .bind(project_id)
        .fetch_all(pool)
        .await?;

        Ok(assets)
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM assets WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;

        Ok(())
    }
}

// ============================================================================
// Render Job Model
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct RenderJob {
    pub id: Uuid,
    pub project_id: Uuid,
    pub status: String,
    pub progress: f32,
    pub composition_data: serde_json::Value,
    pub output_path: Option<String>,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
}

impl RenderJob {
    pub async fn create(pool: &PgPool, project_id: Uuid, composition_data: serde_json::Value) -> Result<Self> {
        let job = sqlx::query_as::<_, RenderJob>(
            r#"INSERT INTO render_jobs (project_id, status, composition_data)
               VALUES ($1, 'queued', $2) RETURNING *"#
        )
        .bind(project_id)
        .bind(composition_data)
        .fetch_one(pool)
        .await?;

        Ok(job)
    }

    pub async fn update_status(pool: &PgPool, id: Uuid, status: &str, progress: f32) -> Result<()> {
        sqlx::query(
            r#"UPDATE render_jobs SET status = $1, progress = $2,
               started_at = CASE WHEN status = 'queued' AND $1 = 'processing' THEN NOW() ELSE started_at END,
               completed_at = CASE WHEN $1 IN ('completed', 'failed') THEN NOW() ELSE completed_at END
               WHERE id = $3"#
        )
        .bind(status)
        .bind(progress)
        .bind(id)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn set_output(pool: &PgPool, id: Uuid, output_path: &str) -> Result<()> {
        sqlx::query("UPDATE render_jobs SET output_path = $1 WHERE id = $2")
            .bind(output_path)
            .bind(id)
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn set_error(pool: &PgPool, id: Uuid, error_message: &str) -> Result<()> {
        sqlx::query("UPDATE render_jobs SET status = 'failed', error_message = $1, completed_at = NOW() WHERE id = $2")
            .bind(error_message)
            .bind(id)
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Self>> {
        let job = sqlx::query_as::<_, RenderJob>(
            "SELECT * FROM render_jobs WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        Ok(job)
    }
}
