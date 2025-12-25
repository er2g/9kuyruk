use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::process::Command;
use tauri::command;

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoInfo {
    pub duration: f64,
    pub width: u32,
    pub height: u32,
    pub fps: f64,
    pub codec: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageInfo {
    pub index: usize,
    pub path: String,
    pub exists: bool,
}

/// Get video metadata using ffprobe
#[command]
pub async fn get_video_info(video_path: String) -> Result<VideoInfo, String> {
    let output = Command::new("ffprobe")
        .args([
            "-v", "error",
            "-select_streams", "v:0",
            "-show_entries", "stream=width,height,r_frame_rate,codec_name,duration",
            "-show_entries", "format=duration",
            "-of", "json",
            &video_path,
        ])
        .output()
        .map_err(|e| format!("FFprobe çalıştırılamadı: {}", e))?;

    if !output.status.success() {
        return Err(format!("FFprobe hatası: {}", String::from_utf8_lossy(&output.stderr)));
    }

    let json_str = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value = serde_json::from_str(&json_str)
        .map_err(|e| format!("JSON parse hatası: {}", e))?;

    let stream = json["streams"][0].as_object()
        .ok_or("Stream bilgisi bulunamadı")?;

    let width = stream["width"].as_u64().unwrap_or(0) as u32;
    let height = stream["height"].as_u64().unwrap_or(0) as u32;
    let codec = stream["codec_name"].as_str().unwrap_or("unknown").to_string();

    // FPS calculation
    let fps_str = stream["r_frame_rate"].as_str().unwrap_or("30/1");
    let fps = parse_fps(fps_str);

    // Duration - try stream first, then format
    let duration = stream.get("duration")
        .and_then(|v| v.as_str())
        .and_then(|s| s.parse::<f64>().ok())
        .or_else(|| {
            json["format"]["duration"].as_str()
                .and_then(|s| s.parse::<f64>().ok())
        })
        .unwrap_or(0.0);

    Ok(VideoInfo {
        duration,
        width,
        height,
        fps,
        codec,
    })
}

/// Generate a thumbnail from the video at a specific timestamp
#[command]
pub async fn get_video_thumbnail(video_path: String, timestamp: f64, output_path: String) -> Result<String, String> {
    let output = Command::new("ffmpeg")
        .args([
            "-y",
            "-ss", &timestamp.to_string(),
            "-i", &video_path,
            "-vframes", "1",
            "-vf", "scale=640:-1",
            &output_path,
        ])
        .output()
        .map_err(|e| format!("FFmpeg çalıştırılamadı: {}", e))?;

    if !output.status.success() {
        return Err(format!("Thumbnail oluşturulamadı: {}", String::from_utf8_lossy(&output.stderr)));
    }

    Ok(output_path)
}

/// Count numbered images in a folder (1.png, 2.jpg, etc.)
#[command]
pub async fn count_images_in_folder(folder_path: String) -> Result<Vec<ImageInfo>, String> {
    let valid_exts = vec!["png", "jpg", "jpeg", "webp"];
    let mut images = Vec::new();
    let folder = Path::new(&folder_path);

    if !folder.exists() || !folder.is_dir() {
        return Err("Klasör bulunamadı".to_string());
    }

    let mut index = 1;
    loop {
        let mut found = false;
        for ext in &valid_exts {
            let file_name = format!("{}.{}", index, ext);
            let file_path = folder.join(&file_name);

            if file_path.exists() {
                images.push(ImageInfo {
                    index,
                    path: file_path.to_string_lossy().to_string(),
                    exists: true,
                });
                found = true;
                break;
            }
        }

        if !found {
            break;
        }
        index += 1;
    }

    Ok(images)
}

/// Generate a preview frame with overlay and subtitle
#[command]
pub async fn generate_preview_frame(
    video_path: String,
    overlay_path: Option<String>,
    timestamp: f64,
    output_path: String,
    subtitle_text: Option<String>,
) -> Result<String, String> {
    let mut filter_parts = Vec::new();

    // Add overlay if provided
    if let Some(overlay) = overlay_path {
        // Create a complex filter: scale overlay and position it at top-center
        filter_parts.push(format!(
            "movie={}:loop=0,setpts=N/(FRAME_RATE*TB),scale=1080:-1:force_original_aspect_ratio=decrease,scale='min(1080,iw)':' min(1140,ih)'[ovr];[0:v][ovr]overlay=(W-w)/2:0",
            overlay.replace(":", "\\:")
        ));
    }

    // Add subtitle text if provided
    if let Some(text) = subtitle_text {
        let escaped_text = text.replace("'", "\\'").replace(":", "\\:");
        let drawtext = format!(
            "drawtext=text='{}':fontfile=/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf:fontsize=60:fontcolor=white:borderw=4:bordercolor=black:x=(w-text_w)/2:y=h-150",
            escaped_text
        );

        if filter_parts.is_empty() {
            filter_parts.push(drawtext);
        } else {
            filter_parts.push(format!(",{}", drawtext));
        }
    }

    let mut args = vec![
        "-y".to_string(),
        "-ss".to_string(), timestamp.to_string(),
        "-i".to_string(), video_path,
        "-vframes".to_string(), "1".to_string(),
    ];

    if !filter_parts.is_empty() {
        args.push("-vf".to_string());
        args.push(filter_parts.join(""));
    }

    args.push("-vf".to_string());
    args.push("scale=800:-1".to_string());
    args.push(output_path.clone());

    let output = Command::new("ffmpeg")
        .args(&args)
        .output()
        .map_err(|e| format!("FFmpeg çalıştırılamadı: {}", e))?;

    if !output.status.success() {
        return Err(format!("Preview oluşturulamadı: {}", String::from_utf8_lossy(&output.stderr)));
    }

    Ok(output_path)
}

// Helper function to parse FPS from "num/den" format
fn parse_fps(fps_str: &str) -> f64 {
    let parts: Vec<&str> = fps_str.split('/').collect();
    if parts.len() == 2 {
        if let (Ok(num), Ok(den)) = (parts[0].parse::<f64>(), parts[1].parse::<f64>()) {
            if den != 0.0 {
                return num / den;
            }
        }
    }
    30.0 // default
}
