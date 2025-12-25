use serde::{Deserialize, Serialize};
use std::path::Path;
use std::process::{Command, Stdio};
use tauri::{command, Window};
use std::io::{BufRead, BufReader};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OverlayInterval {
    pub image_path: String,
    pub start: f64,
    pub end: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RenderProgress {
    pub step: String,
    pub progress: f64,
    pub message: String,
}

/// Check if FFmpeg is available
#[command]
pub async fn get_ffmpeg_version() -> Result<String, String> {
    let output = Command::new("ffmpeg")
        .arg("-version")
        .output()
        .map_err(|e| format!("FFmpeg bulunamadı: {}", e))?;

    Ok(String::from_utf8_lossy(&output.stdout).lines().next().unwrap_or("Unknown").to_string())
}

/// Render video with image overlays
#[command]
pub async fn render_video_with_overlays(
    window: Window,
    video_path: String,
    intervals: Vec<OverlayInterval>,
    output_path: String,
) -> Result<String, String> {
    // Build complex filter for overlays
    let mut filter_complex = String::new();

    if intervals.is_empty() {
        return Err("En az bir overlay gerekli".to_string());
    }

    // Load and scale all overlay images
    for (i, interval) in intervals.iter().enumerate() {
        if i > 0 {
            filter_complex.push(';');
        }
        filter_complex.push_str(&format!(
            "[{}:v]scale=1080:-1:force_original_aspect_ratio=decrease,scale='min(1080,iw)':'min(1140,ih)'[ovr{}]",
            i + 1, i
        ));
    }

    filter_complex.push_str(";[0:v]");

    // Apply overlays with time constraints
    for (i, interval) in intervals.iter().enumerate() {
        filter_complex.push_str(&format!(
            "[ovr{}]overlay=(W-w)/2:0:enable='between(t,{},{})'",
            i, interval.start, interval.end
        ));
        if i < intervals.len() - 1 {
            filter_complex.push_str(&format!("[tmp{}];[tmp{}]", i, i));
        }
    }

    // Build FFmpeg command
    let mut args = vec![
        "-y".to_string(),
        "-i".to_string(), video_path.clone(),
    ];

    // Add all overlay image inputs
    for interval in &intervals {
        args.push("-i".to_string());
        args.push(interval.image_path.clone());
    }

    args.extend(vec![
        "-filter_complex".to_string(), filter_complex,
        "-c:v".to_string(), "libx264".to_string(),
        "-preset".to_string(), "medium".to_string(),
        "-crf".to_string(), "23".to_string(),
        "-c:a".to_string(), "copy".to_string(),
        "-progress".to_string(), "pipe:1".to_string(),
        output_path.clone(),
    ]);

    // Execute FFmpeg with progress tracking
    let mut child = Command::new("ffmpeg")
        .args(&args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("FFmpeg başlatılamadı: {}", e))?;

    // Track progress
    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            if let Ok(line) = line {
                // Parse FFmpeg progress output
                if line.starts_with("out_time_ms=") {
                    if let Some(time_str) = line.strip_prefix("out_time_ms=") {
                        if let Ok(time_us) = time_str.parse::<i64>() {
                            let time_s = time_us as f64 / 1_000_000.0;
                            let _ = window.emit("render_progress", RenderProgress {
                                step: "overlay".to_string(),
                                progress: time_s,
                                message: format!("Overlay işleniyor: {:.1}s", time_s),
                            });
                        }
                    }
                }
            }
        }
    }

    let status = child.wait()
        .map_err(|e| format!("FFmpeg hatası: {}", e))?;

    if !status.success() {
        return Err("Overlay işleme başarısız".to_string());
    }

    Ok(output_path)
}

/// Embed ASS subtitles into video
#[command]
pub async fn embed_subtitles(
    window: Window,
    video_path: String,
    ass_path: String,
    output_path: String,
) -> Result<String, String> {
    let ass_escaped = ass_path.replace("\\", "\\\\").replace(":", "\\:");

    let args = vec![
        "-y",
        "-i", &video_path,
        "-vf", &format!("ass={}", ass_escaped),
        "-c:a", "copy",
        "-progress", "pipe:1",
        &output_path,
    ];

    let mut child = Command::new("ffmpeg")
        .args(&args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("FFmpeg başlatılamadı: {}", e))?;

    // Track progress
    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            if let Ok(line) = line {
                if line.starts_with("out_time_ms=") {
                    if let Some(time_str) = line.strip_prefix("out_time_ms=") {
                        if let Ok(time_us) = time_str.parse::<i64>() {
                            let time_s = time_us as f64 / 1_000_000.0;
                            let _ = window.emit("render_progress", RenderProgress {
                                step: "subtitle".to_string(),
                                progress: time_s,
                                message: format!("Altyazı gömülüyor: {:.1}s", time_s),
                            });
                        }
                    }
                }
            }
        }
    }

    let status = child.wait()
        .map_err(|e| format!("FFmpeg hatası: {}", e))?;

    if !status.success() {
        return Err("Altyazı gömme başarısız".to_string());
    }

    Ok(output_path)
}
