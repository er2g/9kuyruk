use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::process::Command;

#[derive(Debug, Serialize, Deserialize)]
pub struct BeatDetectionConfig {
    pub audio_clip_id: String,
    pub sensitivity: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BeatDetectionResult {
    pub beats: Vec<Beat>,
    pub bpm: f64,
    pub time_signature: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Beat {
    pub timestamp: f64,
    pub strength: f64,
    pub is_downbeat: bool,
}

/// Detect beats using FFmpeg audio analysis
/// Uses astats + volumedetect for peak detection
pub async fn detect_beats(config: BeatDetectionConfig, audio_path: &str) -> Result<BeatDetectionResult> {
    // Extract audio volume stats using FFmpeg
    let output = Command::new("ffmpeg")
        .args(&[
            "-i", audio_path,
            "-af", "astats=metadata=1:reset=1,ametadata=print:key=lavfi.astats.Overall.RMS_level:file=-",
            "-f", "null",
            "-"
        ])
        .output()?;

    let stderr = String::from_utf8_lossy(&output.stderr);

    // Parse volume peaks
    let mut peaks = Vec::new();
    let mut timestamps = Vec::new();

    for line in stderr.lines() {
        if line.contains("lavfi.astats.Overall.RMS_level") {
            if let Some(time) = extract_timestamp(line) {
                if let Some(level) = extract_rms_level(line) {
                    peaks.push(level);
                    timestamps.push(time);
                }
            }
        }
    }

    // Find beat candidates (local maxima above threshold)
    let threshold = calculate_adaptive_threshold(&peaks, config.sensitivity);
    let mut beats = Vec::new();

    for i in 1..peaks.len()-1 {
        if peaks[i] > threshold &&
           peaks[i] > peaks[i-1] &&
           peaks[i] > peaks[i+1] {
            beats.push(Beat {
                timestamp: timestamps[i],
                strength: peaks[i],
                is_downbeat: i % 4 == 0, // Simple heuristic
            });
        }
    }

    // Estimate BPM
    let bpm = estimate_bpm(&beats);

    Ok(BeatDetectionResult {
        beats,
        bpm,
        time_signature: "4/4".to_string(), // Default, could be improved
    })
}

fn extract_timestamp(line: &str) -> Option<f64> {
    if let Some(start) = line.find("pts_time:") {
        let rest = &line[start + 9..];
        if let Some(end) = rest.find(char::is_whitespace) {
            return rest[..end].parse().ok();
        }
    }
    None
}

fn extract_rms_level(line: &str) -> Option<f64> {
    if let Some(start) = line.find("lavfi.astats.Overall.RMS_level=") {
        let rest = &line[start + 32..];
        if let Some(end) = rest.find(char::is_whitespace) {
            return rest[..end].parse().ok();
        }
    }
    None
}

fn calculate_adaptive_threshold(peaks: &[f64], sensitivity: f64) -> f64 {
    if peaks.is_empty() {
        return 0.0;
    }

    let mean = peaks.iter().sum::<f64>() / peaks.len() as f64;
    let variance = peaks.iter()
        .map(|p| (p - mean).powi(2))
        .sum::<f64>() / peaks.len() as f64;
    let std_dev = variance.sqrt();

    // Higher sensitivity = lower threshold
    mean + std_dev * (1.0 - sensitivity)
}

fn estimate_bpm(beats: &[Beat]) -> f64 {
    if beats.len() < 2 {
        return 120.0; // Default
    }

    // Calculate average interval between beats
    let mut intervals = Vec::new();
    for i in 1..beats.len() {
        intervals.push(beats[i].timestamp - beats[i-1].timestamp);
    }

    let avg_interval = intervals.iter().sum::<f64>() / intervals.len() as f64;

    // Convert to BPM
    if avg_interval > 0.0 {
        60.0 / avg_interval
    } else {
        120.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adaptive_threshold() {
        let peaks = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let threshold = calculate_adaptive_threshold(&peaks, 0.5);
        assert!(threshold > 0.0 && threshold < 5.0);
    }

    #[test]
    fn test_bpm_estimation() {
        let beats = vec![
            Beat { timestamp: 0.0, strength: 1.0, is_downbeat: true },
            Beat { timestamp: 0.5, strength: 1.0, is_downbeat: false },
            Beat { timestamp: 1.0, strength: 1.0, is_downbeat: true },
        ];
        let bpm = estimate_bpm(&beats);
        assert!((bpm - 120.0).abs() < 1.0); // 0.5s interval = 120 BPM
    }
}
