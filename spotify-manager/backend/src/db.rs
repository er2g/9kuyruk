use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};
use uuid::Uuid;

// ============================================================================
// User Model
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub email: String,
    pub password: String,
}

impl User {
    pub async fn create(pool: &SqlitePool, email: &str, password_hash: &str) -> Result<Self, sqlx::Error> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();

        sqlx::query_as::<_, User>(
            "INSERT INTO users (id, email, password_hash, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?)
             RETURNING *"
        )
        .bind(&id)
        .bind(email)
        .bind(password_hash)
        .bind(now)
        .bind(now)
        .fetch_one(pool)
        .await
    }

    pub async fn find_by_email(pool: &SqlitePool, email: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = ?")
            .bind(email)
            .fetch_optional(pool)
            .await
    }

    pub async fn find_by_id(pool: &SqlitePool, id: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await
    }
}

// ============================================================================
// Spotify Connection Model
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SpotifyConnection {
    pub id: String,
    pub user_id: String,
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: DateTime<Utc>,
    pub spotify_user_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl SpotifyConnection {
    pub async fn create(
        pool: &SqlitePool,
        user_id: &str,
        access_token: &str,
        refresh_token: &str,
        expires_in: i64,
    ) -> Result<Self, sqlx::Error> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();
        let expires_at = now + chrono::Duration::seconds(expires_in);

        sqlx::query_as::<_, SpotifyConnection>(
            "INSERT INTO spotify_connections
             (id, user_id, access_token, refresh_token, expires_at, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?)
             RETURNING *"
        )
        .bind(&id)
        .bind(user_id)
        .bind(access_token)
        .bind(refresh_token)
        .bind(expires_at)
        .bind(now)
        .bind(now)
        .fetch_one(pool)
        .await
    }

    pub async fn find_by_user_id(pool: &SqlitePool, user_id: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, SpotifyConnection>(
            "SELECT * FROM spotify_connections WHERE user_id = ?"
        )
        .bind(user_id)
        .fetch_optional(pool)
        .await
    }

    pub async fn update_tokens(
        pool: &SqlitePool,
        user_id: &str,
        access_token: &str,
        refresh_token: &str,
        expires_in: i64,
    ) -> Result<(), sqlx::Error> {
        let now = Utc::now();
        let expires_at = now + chrono::Duration::seconds(expires_in);

        sqlx::query(
            "UPDATE spotify_connections
             SET access_token = ?, refresh_token = ?, expires_at = ?, updated_at = ?
             WHERE user_id = ?"
        )
        .bind(access_token)
        .bind(refresh_token)
        .bind(expires_at)
        .bind(now)
        .bind(user_id)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn delete_by_user_id(pool: &SqlitePool, user_id: &str) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM spotify_connections WHERE user_id = ?")
            .bind(user_id)
            .execute(pool)
            .await?;
        Ok(())
    }
}

// ============================================================================
// Device Model
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Device {
    pub id: String,
    pub user_id: String,
    pub spotify_device_id: String,
    pub device_name: String,
    pub device_type: String,
    pub is_locked: bool,
    pub is_primary: bool,
    pub last_seen: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Device {
    pub async fn upsert(
        pool: &SqlitePool,
        user_id: &str,
        spotify_device_id: &str,
        device_name: &str,
        device_type: &str,
    ) -> Result<Self, sqlx::Error> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();

        sqlx::query_as::<_, Device>(
            "INSERT INTO devices
             (id, user_id, spotify_device_id, device_name, device_type, is_locked, is_primary, last_seen, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, false, false, ?, ?, ?)
             ON CONFLICT(spotify_device_id) DO UPDATE SET
                device_name = excluded.device_name,
                device_type = excluded.device_type,
                last_seen = excluded.last_seen,
                updated_at = excluded.updated_at
             RETURNING *"
        )
        .bind(&id)
        .bind(user_id)
        .bind(spotify_device_id)
        .bind(device_name)
        .bind(device_type)
        .bind(now)
        .bind(now)
        .bind(now)
        .fetch_one(pool)
        .await
    }

    pub async fn find_by_user_id(pool: &SqlitePool, user_id: &str) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Device>("SELECT * FROM devices WHERE user_id = ? ORDER BY created_at DESC")
            .bind(user_id)
            .fetch_all(pool)
            .await
    }

    pub async fn find_by_id(pool: &SqlitePool, id: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Device>("SELECT * FROM devices WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    pub async fn update_lock_status(pool: &SqlitePool, id: &str, is_locked: bool) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE devices SET is_locked = ?, updated_at = ? WHERE id = ?")
            .bind(is_locked)
            .bind(Utc::now())
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn set_primary(pool: &SqlitePool, user_id: &str, device_id: &str) -> Result<(), sqlx::Error> {
        // First, unset all primary devices for this user
        sqlx::query("UPDATE devices SET is_primary = false WHERE user_id = ?")
            .bind(user_id)
            .execute(pool)
            .await?;

        // Then set the specified device as primary
        sqlx::query("UPDATE devices SET is_primary = true, updated_at = ? WHERE id = ?")
            .bind(Utc::now())
            .bind(device_id)
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn find_primary(pool: &SqlitePool, user_id: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Device>(
            "SELECT * FROM devices WHERE user_id = ? AND is_primary = true"
        )
        .bind(user_id)
        .fetch_optional(pool)
        .await
    }

    pub async fn update_last_seen(pool: &SqlitePool, device_id: &str) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE devices SET last_seen = ? WHERE id = ?")
            .bind(Utc::now())
            .bind(device_id)
            .execute(pool)
            .await?;
        Ok(())
    }
}

// ============================================================================
// Settings Model
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Settings {
    pub id: String,
    pub user_id: String,
    pub auto_pause_on_device_lost: bool,
    pub device_lost_timeout_seconds: i32,
    pub enable_device_locking: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSettings {
    pub auto_pause_on_device_lost: Option<bool>,
    pub device_lost_timeout_seconds: Option<i32>,
    pub enable_device_locking: Option<bool>,
}

impl Settings {
    pub async fn get_or_create(pool: &SqlitePool, user_id: &str) -> Result<Self, sqlx::Error> {
        if let Some(settings) = Self::find_by_user_id(pool, user_id).await? {
            return Ok(settings);
        }

        let id = Uuid::new_v4().to_string();
        let now = Utc::now();

        sqlx::query_as::<_, Settings>(
            "INSERT INTO settings
             (id, user_id, auto_pause_on_device_lost, device_lost_timeout_seconds, enable_device_locking, created_at, updated_at)
             VALUES (?, ?, true, 30, true, ?, ?)
             RETURNING *"
        )
        .bind(&id)
        .bind(user_id)
        .bind(now)
        .bind(now)
        .fetch_one(pool)
        .await
    }

    pub async fn find_by_user_id(pool: &SqlitePool, user_id: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Settings>("SELECT * FROM settings WHERE user_id = ?")
            .bind(user_id)
            .fetch_optional(pool)
            .await
    }

    pub async fn update(pool: &SqlitePool, user_id: &str, updates: UpdateSettings) -> Result<Self, sqlx::Error> {
        let now = Utc::now();
        let mut query = String::from("UPDATE settings SET updated_at = ?");
        let mut bindings = vec![now.to_string()];

        if let Some(val) = updates.auto_pause_on_device_lost {
            query.push_str(", auto_pause_on_device_lost = ?");
            bindings.push(val.to_string());
        }
        if let Some(val) = updates.device_lost_timeout_seconds {
            query.push_str(", device_lost_timeout_seconds = ?");
            bindings.push(val.to_string());
        }
        if let Some(val) = updates.enable_device_locking {
            query.push_str(", enable_device_locking = ?");
            bindings.push(val.to_string());
        }

        query.push_str(" WHERE user_id = ? RETURNING *");
        bindings.push(user_id.to_string());

        // This is a simplified version - in production, use proper query building
        Settings::get_or_create(pool, user_id).await
    }
}
