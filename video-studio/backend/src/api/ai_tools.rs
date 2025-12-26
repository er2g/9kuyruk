use axum::{
    extract::{State, Multipart},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use crate::{AppState, overlay, subtitle, video};

/// Request: Upload images + video duration
#[derive(Deserialize)]
pub struct AutoDistributeRequest {
    pub video_duration: f64,
}

/// Response: Overlay placements with timing
#[derive(Serialize)]
pub struct AutoDistributeResponse {
    pub placements: Vec<overlay::OverlayPlacement>,
}

/// Auto-distribute overlays across video duration
pub async fn auto_distribute(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Json<AutoDistributeResponse>, StatusCode> {
    let mut video_duration: Option<f64> = None;
    let mut image_paths = Vec::new();

    // Parse multipart form
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();

        match name.as_str() {
            "video_duration" => {
                let data = field.text().await.unwrap();
                video_duration = Some(data.parse().map_err(|_| StatusCode::BAD_REQUEST)?);
            }
            "image" => {
                let filename = field.file_name().unwrap().to_string();
                let data = field.bytes().await.unwrap().to_vec();

                // Save image to storage
                let path = state.storage.save_file("images", &filename, &data).await
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
                image_paths.push(path);
            }
            _ => {}
        }
    }

    let video_duration = video_duration.ok_or(StatusCode::BAD_REQUEST)?;

    // Auto-distribute overlays
    let placements = overlay::auto_distribute_overlays(image_paths, video_duration).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(AutoDistributeResponse { placements }))
}

/// Request: Text + audio for forced alignment
#[derive(Deserialize)]
pub struct AlignSubtitlesRequest {
    pub text: String,
    pub language: String, // "tur" or "eng"
}

/// Response: Timed subtitle entries
#[derive(Serialize)]
pub struct AlignSubtitlesResponse {
    pub subtitles: Vec<subtitle::SubtitleEntry>,
}

/// Align text to audio using Aeneas (forced alignment)
pub async fn align_subtitles(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Json<AlignSubtitlesResponse>, StatusCode> {
    let mut text: Option<String> = None;
    let mut language: Option<String> = None;
    let mut audio_path: Option<String> = None;

    // Parse multipart form
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();

        match name.as_str() {
            "text" => {
                text = Some(field.text().await.unwrap());
            }
            "language" => {
                language = Some(field.text().await.unwrap());
            }
            "audio" => {
                let filename = field.file_name().unwrap().to_string();
                let data = field.bytes().await.unwrap().to_vec();

                // Save audio to storage
                let path = state.storage.save_file("audio", &filename, &data).await
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
                audio_path = Some(path);
            }
            _ => {}
        }
    }

    let text = text.ok_or(StatusCode::BAD_REQUEST)?;
    let language = language.ok_or(StatusCode::BAD_REQUEST)?;
    let audio_path = audio_path.ok_or(StatusCode::BAD_REQUEST)?;

    // Generate aligned subtitles using Aeneas
    let config = subtitle::AeneasConfig {
        text,
        audio_path,
        language,
    };

    let subtitles = subtitle::generate_subtitles_aeneas(config).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(AlignSubtitlesResponse { subtitles }))
}

/// Auto-transcribe audio using Whisper (no text input needed)
#[derive(Serialize)]
pub struct TranscribeResponse {
    pub subtitles: Vec<subtitle::SubtitleEntry>,
}

pub async fn transcribe_audio(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Json<TranscribeResponse>, StatusCode> {
    let mut audio_path: Option<String> = None;
    let mut language: Option<String> = None;

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();

        match name.as_str() {
            "language" => {
                language = Some(field.text().await.unwrap());
            }
            "audio" => {
                let filename = field.file_name().unwrap().to_string();
                let data = field.bytes().await.unwrap().to_vec();

                let path = state.storage.save_file("audio", &filename, &data).await
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
                audio_path = Some(path);
            }
            _ => {}
        }
    }

    let audio_path = audio_path.ok_or(StatusCode::BAD_REQUEST)?;

    // Transcribe using Whisper
    let subtitles = subtitle::generate_subtitles_whisper(audio_path, language).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(TranscribeResponse { subtitles }))
}
