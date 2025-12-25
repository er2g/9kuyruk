use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time;

use crate::{db::{Device, Settings}, AppState};

// Device Tracker - monitors device status
#[derive(Clone)]
pub struct DeviceTracker {
    devices: HashMap<String, DeviceStatus>,
}

#[derive(Clone, Debug)]
pub struct DeviceStatus {
    pub device_id: String,
    pub is_active: bool,
    pub last_check: chrono::DateTime<Utc>,
}

impl DeviceTracker {
    pub fn new() -> Self {
        Self {
            devices: HashMap::new(),
        }
    }

    pub fn update_device(&mut self, device_id: String, is_active: bool) {
        self.devices.insert(
            device_id.clone(),
            DeviceStatus {
                device_id,
                is_active,
                last_check: Utc::now(),
            },
        );
    }

    pub fn get_device(&self, device_id: &str) -> Option<&DeviceStatus> {
        self.devices.get(device_id)
    }

    pub fn is_device_lost(&self, device_id: &str, timeout_seconds: i32) -> bool {
        if let Some(status) = self.devices.get(device_id) {
            let elapsed = Utc::now() - status.last_check;
            elapsed.num_seconds() > timeout_seconds as i64
        } else {
            true
        }
    }
}

// Background task to monitor devices
pub async fn monitor_devices(tracker: Arc<RwLock<DeviceTracker>>) {
    let mut interval = time::interval(time::Duration::from_secs(10));

    loop {
        interval.tick().await;

        // TODO: Implement device monitoring logic
        // 1. Fetch active devices from Spotify API
        // 2. Update tracker
        // 3. Check if primary device is lost
        // 4. Trigger pause if needed

        tracing::debug!("Device monitoring tick");
    }
}

// Response types
#[derive(Debug, Serialize)]
pub struct DeviceResponse {
    pub id: String,
    pub spotify_device_id: String,
    pub device_name: String,
    pub device_type: String,
    pub is_locked: bool,
    pub is_primary: bool,
    pub last_seen: String,
}

impl From<Device> for DeviceResponse {
    fn from(device: Device) -> Self {
        Self {
            id: device.id,
            spotify_device_id: device.spotify_device_id,
            device_name: device.device_name,
            device_type: device.device_type,
            is_locked: device.is_locked,
            is_primary: device.is_primary,
            last_seen: device.last_seen.to_rfc3339(),
        }
    }
}

// List all devices for a user
pub async fn list_devices(
    State(state): State<AppState>,
) -> Result<Json<Vec<DeviceResponse>>, (StatusCode, String)> {
    let user_id = "temp_user"; // TODO: Get from auth

    let devices = Device::find_by_user_id(&state.db, user_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let response: Vec<DeviceResponse> = devices.into_iter().map(Into::into).collect();

    Ok(Json(response))
}

// Get a specific device
pub async fn get_device(
    State(state): State<AppState>,
    Path(device_id): Path<String>,
) -> Result<Json<DeviceResponse>, (StatusCode, String)> {
    let device = Device::find_by_id(&state.db, &device_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Device not found".to_string()))?;

    Ok(Json(device.into()))
}

// Lock a device
pub async fn lock_device(
    State(state): State<AppState>,
    Path(device_id): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    Device::update_lock_status(&state.db, &device_id, true)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::OK)
}

// Unlock a device
pub async fn unlock_device(
    State(state): State<AppState>,
    Path(device_id): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    Device::update_lock_status(&state.db, &device_id, false)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::OK)
}

// Set a device as primary
pub async fn set_primary(
    State(state): State<AppState>,
    Path(device_id): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    let user_id = "temp_user"; // TODO: Get from auth

    // Verify device belongs to user
    let device = Device::find_by_id(&state.db, &device_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Device not found".to_string()))?;

    if device.user_id != user_id {
        return Err((StatusCode::FORBIDDEN, "Access denied".to_string()));
    }

    Device::set_primary(&state.db, user_id, &device_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::OK)
}
