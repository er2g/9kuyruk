use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow};
use std::process::Command;

/// Scene detection using FFmpeg's scene detection filter
/// Professional-grade shot change detection

#[derive(Debug, Serialize, Deserialize)]
pub struct SceneDetectionConfig {
    pub video_path: String,
    pub threshold: f64,           // 0.0-1.0, default 0.3
    pub min_scene_length: f64,    // seconds, default 1.0
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Scene {
    pub index: usize,
    pub start_time: f64,
    pub end_time: f64,
    pub duration: f64,
    pub frame_start: i64,
    pub frame_end: i64,
    pub score: f64,               // Scene change score
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SceneAnalysis {
    pub scenes: Vec<Scene>,
    pub total_scenes: usize,
    pub average_scene_length: f64,
    pub video_duration: f64,
}

impl SceneDetectionConfig {
    pub fn new(video_path: String) -> Self {
        Self {
            video_path,
            threshold: 0.3,
            min_scene_length: 1.0,
        }
    }
}

/// Detect scenes using FFmpeg scene filter
pub async fn detect_scenes(config: SceneDetectionConfig) -> Result<SceneAnalysis> {
    // FFmpeg command for scene detection
    // Uses select=gt(scene,THRESHOLD) filter
    let output = Command::new("ffmpeg")
        .args(&[
            "-i", &config.video_path,
            "-filter:v", &format!("select='gt(scene,{})',showinfo", config.threshold),
            "-f", "null",
            "-"
        ])
        .output()?;

    let stderr = String::from_utf8_lossy(&output.stderr);

    // Parse scene timestamps from ffmpeg output
    let mut scenes = Vec::new();
    let mut scene_times = vec![0.0]; // Start with 0

    for line in stderr.lines() {
        if line.contains("pts_time:") {
            if let Some(time_str) = extract_pts_time(line) {
                if let Ok(time) = time_str.parse::<f64>() {
                    scene_times.push(time);
                }
            }
        }
    }

    // Get video duration
    let duration = get_video_duration(&config.video_path).await?;
    scene_times.push(duration);

    // Build scenes from timestamps
    for i in 0..scene_times.len() - 1 {
        let start = scene_times[i];
        let end = scene_times[i + 1];
        let scene_duration = end - start;

        // Filter by minimum scene length
        if scene_duration >= config.min_scene_length {
            scenes.push(Scene {
                index: scenes.len(),
                start_time: start,
                end_time: end,
                duration: scene_duration,
                frame_start: (start * 30.0) as i64, // Approximate, should get actual fps
                frame_end: (end * 30.0) as i64,
                score: config.threshold,
            });
        }
    }

    let total_scenes = scenes.len();
    let average_scene_length = if total_scenes > 0 {
        scenes.iter().map(|s| s.duration).sum::<f64>() / total_scenes as f64
    } else {
        0.0
    };

    Ok(SceneAnalysis {
        scenes,
        total_scenes,
        average_scene_length,
        video_duration: duration,
    })
}

/// Advanced scene analysis with frame-by-frame comparison
/// Uses histogram comparison for more accurate detection
pub async fn detect_scenes_advanced(config: SceneDetectionConfig) -> Result<SceneAnalysis> {
    // This uses FFmpeg's scdet filter with histogram analysis
    let temp_log = format!("/tmp/scene_detect_{}.log", uuid::Uuid::new_v4());

    let output = Command::new("ffmpeg")
        .args(&[
            "-i", &config.video_path,
            "-vf", &format!("scdet=t={}:s=1", config.threshold),
            "-an",
            "-f", "null",
            "-"
        ])
        .output()?;

    let stderr = String::from_utf8_lossy(&output.stderr);

    let mut scenes = Vec::new();
    let mut last_time = 0.0;
    let mut scene_index = 0;

    for line in stderr.lines() {
        if line.contains("lavfi.scd.score") {
            if let Some((time, score)) = parse_scd_line(line) {
                if score >= config.threshold && time - last_time >= config.min_scene_length {
                    if scene_index > 0 {
                        // Complete previous scene
                        if let Some(last_scene) = scenes.last_mut() {
                            last_scene.end_time = time;
                            last_scene.duration = time - last_scene.start_time;
                        }
                    }

                    // Start new scene
                    scenes.push(Scene {
                        index: scene_index,
                        start_time: time,
                        end_time: 0.0,
                        duration: 0.0,
                        frame_start: 0,
                        frame_end: 0,
                        score,
                    });

                    last_time = time;
                    scene_index += 1;
                }
            }
        }
    }

    // Complete last scene
    let duration = get_video_duration(&config.video_path).await?;
    if let Some(last_scene) = scenes.last_mut() {
        last_scene.end_time = duration;
        last_scene.duration = duration - last_scene.start_time;
    }

    let total_scenes = scenes.len();
    let average_scene_length = if total_scenes > 0 {
        scenes.iter().map(|s| s.duration).sum::<f64>() / total_scenes as f64
    } else {
        0.0
    };

    Ok(SceneAnalysis {
        scenes,
        total_scenes,
        average_scene_length,
        video_duration: duration,
    })
}

/// Analyze scene composition
/// Detects composition elements like rule of thirds, golden ratio
pub async fn analyze_scene_composition(video_path: &str, timestamp: f64) -> Result<CompositionAnalysis> {
    // Extract frame at timestamp
    let frame_path = format!("/tmp/frame_{}.jpg", timestamp);

    Command::new("ffmpeg")
        .args(&[
            "-ss", &timestamp.to_string(),
            "-i", video_path,
            "-vframes", "1",
            "-q:v", "2",
            &frame_path
        ])
        .output()?;

    // TODO: Use computer vision library (opencv-rust) for actual analysis
    // For now, return placeholder
    Ok(CompositionAnalysis {
        rule_of_thirds_score: 0.7,
        golden_ratio_score: 0.6,
        leading_lines: vec![],
        symmetry_score: 0.5,
        visual_weight: VisualWeight::Balanced,
    })
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompositionAnalysis {
    pub rule_of_thirds_score: f64,
    pub golden_ratio_score: f64,
    pub leading_lines: Vec<Line>,
    pub symmetry_score: f64,
    pub visual_weight: VisualWeight,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Line {
    pub start_x: f64,
    pub start_y: f64,
    pub end_x: f64,
    pub end_y: f64,
    pub strength: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum VisualWeight {
    LeftHeavy,
    RightHeavy,
    TopHeavy,
    BottomHeavy,
    Balanced,
}

// Helper functions

fn extract_pts_time(line: &str) -> Option<String> {
    if let Some(start) = line.find("pts_time:") {
        let rest = &line[start + 9..];
        if let Some(end) = rest.find(' ') {
            return Some(rest[..end].to_string());
        }
    }
    None
}

fn parse_scd_line(line: &str) -> Option<(f64, f64)> {
    // Parse lines like: [Parsed_scdet_0 @ ...] lavfi.scd.score: 0.456 pts_time:12.5
    let mut time = 0.0;
    let mut score = 0.0;

    if let Some(score_str) = line.split("lavfi.scd.score:").nth(1) {
        if let Some(s) = score_str.split_whitespace().next() {
            score = s.parse().ok()?;
        }
    }

    if let Some(time_str) = line.split("pts_time:").nth(1) {
        if let Some(t) = time_str.split_whitespace().next() {
            time = t.parse().ok()?;
        }
    }

    Some((time, score))
}

async fn get_video_duration(video_path: &str) -> Result<f64> {
    let output = Command::new("ffprobe")
        .args(&[
            "-v", "error",
            "-show_entries", "format=duration",
            "-of", "default=noprint_wrappers=1:nokey=1",
            video_path
        ])
        .output()?;

    let duration_str = String::from_utf8_lossy(&output.stdout);
    duration_str.trim()
        .parse::<f64>()
        .map_err(|e| anyhow!("Failed to parse duration: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_scene_detection() {
        let config = SceneDetectionConfig {
            video_path: "test_video.mp4".to_string(),
            threshold: 0.4,
            min_scene_length: 2.0,
        };

        // This would require actual test video
        // let result = detect_scenes(config).await;
        // assert!(result.is_ok());
    }
}
