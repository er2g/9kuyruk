use axum::{
    extract::{State, Multipart, Path},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::{AppState, overlay, subtitle, video, scene, ai_agent, ai};

// ============================================================================
// OVERLAY AUTO-DISTRIBUTE
// ============================================================================

#[derive(Serialize)]
pub struct AutoDistributeResponse {
    pub placements: Vec<overlay::OverlayPlacement>,
}

pub async fn auto_distribute(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Json<AutoDistributeResponse>, StatusCode> {
    let mut video_duration: Option<f64> = None;
    let mut image_paths = Vec::new();

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
                let path = state.storage.save_file("images", &filename, &data).await
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
                image_paths.push(path);
            }
            _ => {}
        }
    }

    let video_duration = video_duration.ok_or(StatusCode::BAD_REQUEST)?;
    let placements = overlay::auto_distribute_overlays(image_paths, video_duration).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(AutoDistributeResponse { placements }))
}

// ============================================================================
// SUBTITLE ALIGNMENT
// ============================================================================

#[derive(Serialize)]
pub struct AlignSubtitlesResponse {
    pub subtitles: Vec<subtitle::SubtitleEntry>,
}

pub async fn align_subtitles(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Json<AlignSubtitlesResponse>, StatusCode> {
    let mut text: Option<String> = None;
    let mut language: Option<String> = None;
    let mut audio_path: Option<String> = None;

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();

        match name.as_str() {
            "text" => text = Some(field.text().await.unwrap()),
            "language" => language = Some(field.text().await.unwrap()),
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

    let text = text.ok_or(StatusCode::BAD_REQUEST)?;
    let language = language.ok_or(StatusCode::BAD_REQUEST)?;
    let audio_path = audio_path.ok_or(StatusCode::BAD_REQUEST)?;

    let config = subtitle::AeneasConfig {
        text,
        audio_path,
        language,
    };

    let subtitles = subtitle::generate_subtitles_aeneas(config).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(AlignSubtitlesResponse { subtitles }))
}

// ============================================================================
// WHISPER TRANSCRIPTION
// ============================================================================

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
            "language" => language = Some(field.text().await.unwrap()),
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
    let subtitles = subtitle::generate_subtitles_whisper(audio_path, language).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(TranscribeResponse { subtitles }))
}

// ============================================================================
// SCENE DETECTION
// ============================================================================

#[derive(Serialize)]
pub struct SceneDetectionResponse {
    pub scenes: Vec<scene::Scene>,
    pub total_scenes: usize,
}

pub async fn detect_scenes(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Json<SceneDetectionResponse>, StatusCode> {
    let mut video_path: Option<String> = None;
    let mut threshold = 0.3;

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();

        match name.as_str() {
            "threshold" => threshold = field.text().await.unwrap().parse().unwrap_or(0.3),
            "video" => {
                let filename = field.file_name().unwrap().to_string();
                let data = field.bytes().await.unwrap().to_vec();
                let path = state.storage.save_file("videos", &filename, &data).await
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
                video_path = Some(path);
            }
            _ => {}
        }
    }

    let video_path = video_path.ok_or(StatusCode::BAD_REQUEST)?;

    let config = scene::SceneDetectionConfig {
        video_path,
        threshold,
        min_scene_length: 1.0,
    };

    let analysis = scene::detect_scenes(config).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(SceneDetectionResponse {
        scenes: analysis.scenes,
        total_scenes: analysis.total_scenes,
    }))
}

// ============================================================================
// AUTO REFRAME
// ============================================================================

#[derive(Serialize)]
pub struct AutoReframeResponse {
    pub output_path: String,
    pub processing_time: f64,
}

pub async fn auto_reframe(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Json<AutoReframeResponse>, StatusCode> {
    let mut video_path: Option<String> = None;
    let mut target_aspect = String::from("9:16");

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();

        match name.as_str() {
            "target_aspect" => target_aspect = field.text().await.unwrap(),
            "video" => {
                let filename = field.file_name().unwrap().to_string();
                let data = field.bytes().await.unwrap().to_vec();
                let path = state.storage.save_file("videos", &filename, &data).await
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
                video_path = Some(path);
            }
            _ => {}
        }
    }

    let video_path = video_path.ok_or(StatusCode::BAD_REQUEST)?;
    let output_path = format!("/tmp/reframed_{}.mp4", Uuid::new_v4());

    let target = match target_aspect.as_str() {
        "9:16" => ai::auto_reframe::AspectRatio::Ratio9_16,
        "1:1" => ai::auto_reframe::AspectRatio::Square,
        "16:9" => ai::auto_reframe::AspectRatio::Ratio16_9,
        "4:5" => ai::auto_reframe::AspectRatio::Ratio4_5,
        _ => ai::auto_reframe::AspectRatio::Ratio9_16,
    };

    let config = ai::auto_reframe::AutoReframeConfig {
        input_video: video_path,
        output_video: output_path.clone(),
        source_aspect: ai::auto_reframe::AspectRatio::Ratio16_9,
        target_aspect: target,
        motion_priority: ai::auto_reframe::MotionPriority::Action,
        track_subject: true,
    };

    let result = ai::auto_reframe::auto_reframe(config).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(AutoReframeResponse {
        output_path: result.output_path,
        processing_time: result.processing_time,
    }))
}

// ============================================================================
// CHROMA KEY
// ============================================================================

#[derive(Serialize)]
pub struct ChromaKeyResponse {
    pub output_path: String,
}

pub async fn chroma_key(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Json<ChromaKeyResponse>, StatusCode> {
    let mut video_path: Option<String> = None;
    let mut key_color = String::from("00ff00");
    let mut similarity = 0.3;
    let mut blend = 0.1;

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();

        match name.as_str() {
            "key_color" => key_color = field.text().await.unwrap(),
            "similarity" => similarity = field.text().await.unwrap().parse().unwrap_or(0.3),
            "blend" => blend = field.text().await.unwrap().parse().unwrap_or(0.1),
            "video" => {
                let filename = field.file_name().unwrap().to_string();
                let data = field.bytes().await.unwrap().to_vec();
                let path = state.storage.save_file("videos", &filename, &data).await
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
                video_path = Some(path);
            }
            _ => {}
        }
    }

    let video_path = video_path.ok_or(StatusCode::BAD_REQUEST)?;
    let output_path = format!("/tmp/chromakey_{}.mp4", Uuid::new_v4());

    let clip_id = Uuid::new_v4().to_string();
    let config = ai::chroma_key::ChromaKeyConfig {
        clip_id,
        key_color,
        similarity,
        blend,
    };

    let result = ai::chroma_key::apply_chroma_key(config, &video_path, &output_path).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ChromaKeyResponse { output_path: result.output_path }))
}

// ============================================================================
// BEAT DETECTION
// ============================================================================

#[derive(Serialize)]
pub struct BeatDetectionResponse {
    pub beats: Vec<ai::beat_detection::Beat>,
    pub bpm: f64,
}

pub async fn detect_beats(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Json<BeatDetectionResponse>, StatusCode> {
    let mut audio_path: Option<String> = None;
    let mut sensitivity = 0.5;

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();

        match name.as_str() {
            "sensitivity" => sensitivity = field.text().await.unwrap().parse().unwrap_or(0.5),
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
    let audio_clip_id = Uuid::new_v4().to_string();

    let config = ai::beat_detection::BeatDetectionConfig {
        audio_clip_id,
        sensitivity,
    };

    let result = ai::beat_detection::detect_beats(config, &audio_path).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(BeatDetectionResponse {
        beats: result.beats,
        bpm: result.bpm,
    }))
}

// ============================================================================
// AI AGENT CHAT
// ============================================================================

#[derive(Deserialize)]
pub struct AIAgentRequest {
    pub message: String,
    pub project_id: Option<Uuid>,
}

#[derive(Serialize)]
pub struct AIAgentResponse {
    pub response: String,
}

pub async fn ai_agent_chat(
    State(_state): State<AppState>,
    Json(req): Json<AIAgentRequest>,
) -> Result<Json<AIAgentResponse>, StatusCode> {
    // Simplified AI agent - just return a placeholder response
    // Full implementation would require OpenAI/Gemini API integration

    let response = format!(
        "AI Agent received: '{}'. Full AI integration requires API keys (OPENAI_API_KEY or GEMINI_API_KEY).",
        req.message
    );

    Ok(Json(AIAgentResponse { response }))
}

// ============================================================================
// VIDEO PROXY GENERATION
// ============================================================================

#[derive(Serialize)]
pub struct ProxyResponse {
    pub proxy_path: String,
}

pub async fn generate_proxy(
    State(state): State<AppState>,
    Path(asset_id): Path<Uuid>,
) -> Result<Json<ProxyResponse>, StatusCode> {
    // Find asset in database - using optional since we might not have find_by_id
    // For now, just generate a placeholder response
    let proxy_path = format!("/tmp/proxy_{}.mp4", asset_id);

    // In production, would call video::generate_proxy() here
    // video::generate_proxy(&asset.file_path, &proxy_path).await

    Ok(Json(ProxyResponse { proxy_path }))
}
