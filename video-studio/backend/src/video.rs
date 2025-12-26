use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoMetadata {
    pub width: i32,
    pub height: i32,
    pub duration: f64,
    pub fps: f64,
    pub codec: String,
    pub bitrate: i64,
}

/// Extract video metadata using FFprobe
pub async fn extract_metadata(video_path: &str) -> Result<VideoMetadata> {
    let output = Command::new("ffprobe")
        .args(&[
            "-v", "error",
            "-select_streams", "v:0",
            "-show_entries", "stream=width,height,duration,r_frame_rate,codec_name,bit_rate",
            "-of", "json",
            video_path
        ])
        .output()?;

    let json: serde_json::Value = serde_json::from_slice(&output.stdout)?;
    let stream = &json["streams"][0];

    let width = stream["width"].as_i64().unwrap_or(1920) as i32;
    let height = stream["height"].as_i64().unwrap_or(1080) as i32;
    let duration = stream["duration"]
        .as_str()
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(0.0);
    let codec = stream["codec_name"].as_str().unwrap_or("unknown").to_string();
    let bitrate = stream["bit_rate"].as_str()
        .and_then(|s| s.parse::<i64>().ok())
        .unwrap_or(0);

    let fps_str = stream["r_frame_rate"].as_str().unwrap_or("30/1");
    let fps = parse_fps(fps_str);

    Ok(VideoMetadata {
        width,
        height,
        duration,
        fps,
        codec,
        bitrate,
    })
}

fn parse_fps(fps_str: &str) -> f64 {
    if let Some(slash_pos) = fps_str.find('/') {
        let num: f64 = fps_str[..slash_pos].parse().unwrap_or(30.0);
        let den: f64 = fps_str[slash_pos + 1..].parse().unwrap_or(1.0);
        num / den
    } else {
        fps_str.parse().unwrap_or(30.0)
    }
}

/// Generate proxy (lower resolution) for editing
pub async fn generate_proxy(input: &str, output: &str) -> Result<()> {
    let output_cmd = Command::new("ffmpeg")
        .args(&[
            "-i", input,
            "-vf", "scale=1280:720",
            "-c:v", "libx264",
            "-preset", "ultrafast",
            "-crf", "28",
            "-c:a", "aac",
            "-b:a", "128k",
            output
        ])
        .output()?;

    if !output_cmd.status.success() {
        anyhow::bail!("Proxy generation failed: {}", String::from_utf8_lossy(&output_cmd.stderr));
    }

    Ok(())
}

/// Render composition to final video
#[derive(Debug, Serialize, Deserialize)]
pub struct RenderOptions {
    pub output_path: String,
    pub width: i32,
    pub height: i32,
    pub fps: f64,
    pub quality: String, // "draft", "preview", "final"
}

pub async fn render_composition(
    composition: &serde_json::Value,
    options: &RenderOptions,
) -> Result<()> {
    // Build FFmpeg filter complex from composition
    let filter_complex = build_filter_complex(composition)?;

    let (preset, crf) = match options.quality.as_str() {
        "draft" => ("ultrafast", "28"),
        "preview" => ("medium", "23"),
        "final" => ("slow", "18"),
        _ => ("medium", "23"),
    };

    let output_cmd = Command::new("ffmpeg")
        .args(&[
            "-filter_complex", &filter_complex,
            "-s", &format!("{}x{}", options.width, options.height),
            "-r", &options.fps.to_string(),
            "-c:v", "libx264",
            "-preset", preset,
            "-crf", crf,
            "-c:a", "aac",
            "-b:a", "192k",
            &options.output_path
        ])
        .output()?;

    if !output_cmd.status.success() {
        anyhow::bail!("Render failed: {}", String::from_utf8_lossy(&output_cmd.stderr));
    }

    Ok(())
}

fn build_filter_complex(composition: &serde_json::Value) -> Result<String> {
    // Simple implementation - would be expanded
    Ok("[0:v]scale=1920:1080[out]".to_string())
}
