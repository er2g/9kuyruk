use axum::{
    extract::{State, Path, Multipart, Query},
    http::StatusCode,
    Json,
};
use uuid::Uuid;
use crate::{AppState, db::Asset, video};

#[derive(serde::Deserialize)]
pub struct ListAssetsQuery {
    pub project_id: Uuid,
}

pub async fn upload(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Json<Asset>, StatusCode> {
    let mut project_id: Option<Uuid> = None;
    let mut filename: Option<String> = None;
    let mut file_data: Option<Vec<u8>> = None;

    // Parse multipart form data
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();

        match name.as_str() {
            "project_id" => {
                let data = field.text().await.unwrap();
                project_id = Some(Uuid::parse_str(&data).map_err(|_| StatusCode::BAD_REQUEST)?);
            }
            "file" => {
                filename = Some(field.file_name().unwrap().to_string());
                file_data = Some(field.bytes().await.unwrap().to_vec());
            }
            _ => {}
        }
    }

    let project_id = project_id.ok_or(StatusCode::BAD_REQUEST)?;
    let filename = filename.ok_or(StatusCode::BAD_REQUEST)?;
    let file_data = file_data.ok_or(StatusCode::BAD_REQUEST)?;

    // Determine asset type from extension
    let asset_type = if filename.ends_with(".mp4") || filename.ends_with(".mov") || filename.ends_with(".avi") {
        "video"
    } else if filename.ends_with(".jpg") || filename.ends_with(".png") || filename.ends_with(".jpeg") {
        "image"
    } else if filename.ends_with(".mp3") || filename.ends_with(".wav") || filename.ends_with(".aac") {
        "audio"
    } else {
        return Err(StatusCode::BAD_REQUEST);
    };

    // Save file to storage
    let file_path = state.storage.save_file(asset_type, &filename, &file_data).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Create asset record
    let file_size = file_data.len() as i64;
    let mut asset = Asset::create(
        &state.db,
        project_id,
        asset_type,
        &filename,
        &file_path,
        file_size,
    ).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Extract metadata for video files
    if asset_type == "video" {
        if let Ok(metadata) = video::extract_metadata(&file_path).await {
            Asset::update_metadata(
                &state.db,
                asset.id,
                Some(metadata.duration),
                Some(metadata.width),
                Some(metadata.height),
                Some(metadata.fps),
            ).await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            asset.duration = Some(metadata.duration);
            asset.width = Some(metadata.width);
            asset.height = Some(metadata.height);
            asset.fps = Some(metadata.fps);

            // Generate thumbnail
            let _ = state.storage.generate_thumbnail(&file_path, &format!("thumb_{}", asset.id)).await;
        }
    }

    Ok(Json(asset))
}

pub async fn list(
    State(state): State<AppState>,
    Query(query): Query<ListAssetsQuery>,
) -> Result<Json<Vec<Asset>>, StatusCode> {
    let assets = Asset::list_by_project(&state.db, query.project_id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(assets))
}

pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    Asset::delete(&state.db, id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}
