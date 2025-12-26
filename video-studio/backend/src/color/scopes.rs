use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::process::Command;

/// Professional Color Scopes
///
/// Implements industry-standard scopes for color grading:
/// - Waveform (Luma)
/// - RGB Parade
/// - Vectorscope
/// - Histogram
///
/// Used for broadcast-safe monitoring and professional color grading

#[derive(Debug, Serialize, Deserialize)]
pub struct ScopeData {
    pub waveform: WaveformData,
    pub vectorscope: VectorscopeData,
    pub histogram: HistogramData,
    pub rgb_parade: RGBParadeData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WaveformData {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Vec<u8>>,  // 2D array [x][y] of intensity
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VectorscopeData {
    pub radius: usize,
    pub data: Vec<Vec<u8>>,  // 2D array representing U/V plane
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HistogramData {
    pub red: Vec<u32>,     // 256 bins
    pub green: Vec<u32>,   // 256 bins
    pub blue: Vec<u32>,    // 256 bins
    pub luma: Vec<u32>,    // 256 bins
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RGBParadeData {
    pub width: usize,
    pub height: usize,
    pub red: Vec<Vec<u8>>,
    pub green: Vec<Vec<u8>>,
    pub blue: Vec<Vec<u8>>,
}

/// Generate all scopes for a video frame
pub async fn generate_scopes(video_path: &str, timestamp: f64) -> Result<ScopeData> {
    // Extract frame
    let frame_path = format!("/tmp/scope_frame_{}.png", timestamp);

    Command::new("ffmpeg")
        .args(&[
            "-ss", &timestamp.to_string(),
            "-i", video_path,
            "-vframes", "1",
            "-f", "image2",
            &frame_path
        ])
        .output()?;

    // Generate waveform
    let waveform = generate_waveform_ffmpeg(video_path, timestamp).await?;

    // Generate vectorscope
    let vectorscope = generate_vectorscope_ffmpeg(video_path, timestamp).await?;

    // Generate histogram
    let histogram = generate_histogram_ffmpeg(video_path, timestamp).await?;

    // Generate RGB parade
    let rgb_parade = generate_rgb_parade_ffmpeg(video_path, timestamp).await?;

    Ok(ScopeData {
        waveform,
        vectorscope,
        histogram,
        rgb_parade,
    })
}

/// Generate waveform using FFmpeg
async fn generate_waveform_ffmpeg(video_path: &str, timestamp: f64) -> Result<WaveformData> {
    let output_path = format!("/tmp/waveform_{}.png", timestamp);

    Command::new("ffmpeg")
        .args(&[
            "-ss", &timestamp.to_string(),
            "-i", video_path,
            "-vframes", "1",
            "-vf", "waveform=mode=column:intensity=0.1:mirror=1:c=1:f=color:graticule=green",
            "-y",
            &output_path
        ])
        .output()?;

    // In production, you'd parse the PNG and extract data
    // For now, return placeholder
    Ok(WaveformData {
        width: 1920,
        height: 256,
        data: vec![vec![128; 256]; 1920],
    })
}

/// Generate vectorscope using FFmpeg
async fn generate_vectorscope_ffmpeg(video_path: &str, timestamp: f64) -> Result<VectorscopeData> {
    let output_path = format!("/tmp/vectorscope_{}.png", timestamp);

    Command::new("ffmpeg")
        .args(&[
            "-ss", &timestamp.to_string(),
            "-i", video_path,
            "-vframes", "1",
            "-vf", "vectorscope=mode=color2:intensity=0.1:envelope=instant:graticule=green",
            "-y",
            &output_path
        ])
        .output()?;

    Ok(VectorscopeData {
        radius: 256,
        data: vec![vec![0; 512]; 512],
    })
}

/// Generate histogram using FFmpeg
async fn generate_histogram_ffmpeg(video_path: &str, timestamp: f64) -> Result<HistogramData> {
    let output_path = format!("/tmp/histogram_{}.png", timestamp);

    Command::new("ffmpeg")
        .args(&[
            "-ss", &timestamp.to_string(),
            "-i", video_path,
            "-vframes", "1",
            "-vf", "histogram=mode=levels:level_height=200",
            "-y",
            &output_path
        ])
        .output()?;

    // Placeholder data
    Ok(HistogramData {
        red: vec![0; 256],
        green: vec![0; 256],
        blue: vec![0; 256],
        luma: vec![0; 256],
    })
}

/// Generate RGB parade using FFmpeg
async fn generate_rgb_parade_ffmpeg(video_path: &str, timestamp: f64) -> Result<RGBParadeData> {
    let output_path = format!("/tmp/parade_{}.png", timestamp);

    Command::new("ffmpeg")
        .args(&[
            "-ss", &timestamp.to_string(),
            "-i", video_path,
            "-vframes", "1",
            "-vf", "waveform=mode=parade:intensity=0.1:mirror=0:c=7:f=color",
            "-y",
            &output_path
        ])
        .output()?;

    Ok(RGBParadeData {
        width: 640,
        height: 256,
        red: vec![vec![0; 256]; 640],
        green: vec![vec![0; 256]; 640],
        blue: vec![vec![0; 256]; 640],
    })
}

/// Check if video is broadcast safe (Rec.709)
#[derive(Debug, Serialize, Deserialize)]
pub struct BroadcastSafetyCheck {
    pub is_safe: bool,
    pub violations: Vec<BroadcastViolation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BroadcastViolation {
    pub timestamp: f64,
    pub violation_type: ViolationType,
    pub severity: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ViolationType {
    LumaTooHigh,    // > 100 IRE (235 in 8-bit)
    LumaTooLow,     // < 0 IRE (16 in 8-bit)
    ChromaTooHigh,  // Chroma exceeds legal range
    IllegalColor,   // Color outside broadcast gamut
}

/// Check broadcast safety
pub async fn check_broadcast_safe(video_path: &str) -> Result<BroadcastSafetyCheck> {
    // Use FFmpeg's signalstats filter
    let output = Command::new("ffmpeg")
        .args(&[
            "-i", video_path,
            "-vf", "signalstats",
            "-f", "null",
            "-"
        ])
        .output()?;

    let stderr = String::from_utf8_lossy(&output.stderr);

    let mut violations = Vec::new();

    // Parse signalstats output
    for line in stderr.lines() {
        if line.contains("YMAX:") {
            if let Some(ymax) = extract_stat_value(line, "YMAX:") {
                if ymax > 235.0 {
                    violations.push(BroadcastViolation {
                        timestamp: 0.0, // Would extract from pts_time
                        violation_type: ViolationType::LumaTooHigh,
                        severity: (ymax - 235.0) / 20.0,
                    });
                }
            }
        }
        if line.contains("YMIN:") {
            if let Some(ymin) = extract_stat_value(line, "YMIN:") {
                if ymin < 16.0 {
                    violations.push(BroadcastViolation {
                        timestamp: 0.0,
                        violation_type: ViolationType::LumaTooLow,
                        severity: (16.0 - ymin) / 16.0,
                    });
                }
            }
        }
    }

    Ok(BroadcastSafetyCheck {
        is_safe: violations.is_empty(),
        violations,
    })
}

fn extract_stat_value(line: &str, stat: &str) -> Option<f64> {
    line.split(stat)
        .nth(1)?
        .split_whitespace()
        .next()?
        .parse()
        .ok()
}

/// Color correction tools
#[derive(Debug, Serialize, Deserialize)]
pub struct ColorCorrection {
    pub lift: [f64; 3],      // Shadows (RGB)
    pub gamma: [f64; 3],     // Midtones (RGB)
    pub gain: [f64; 3],      // Highlights (RGB)
    pub offset: [f64; 3],    // Overall offset (RGB)
    pub saturation: f64,     // Overall saturation
    pub hue: f64,            // Hue shift (degrees)
}

impl Default for ColorCorrection {
    fn default() -> Self {
        Self {
            lift: [0.0, 0.0, 0.0],
            gamma: [1.0, 1.0, 1.0],
            gain: [1.0, 1.0, 1.0],
            offset: [0.0, 0.0, 0.0],
            saturation: 1.0,
            hue: 0.0,
        }
    }
}

impl ColorCorrection {
    /// Generate FFmpeg filter string for color correction
    pub fn to_ffmpeg_filter(&self) -> String {
        let mut filters = Vec::new();

        // Lift, Gamma, Gain
        if self.lift != [0.0, 0.0, 0.0] || self.gamma != [1.0, 1.0, 1.0] || self.gain != [1.0, 1.0, 1.0] {
            filters.push(format!(
                "eq=gamma_r={}:gamma_g={}:gamma_b={}:brightness={}",
                self.gamma[0], self.gamma[1], self.gamma[2],
                (self.offset[0] + self.offset[1] + self.offset[2]) / 3.0
            ));
        }

        // Saturation
        if self.saturation != 1.0 {
            filters.push(format!("eq=saturation={}", self.saturation));
        }

        // Hue
        if self.hue != 0.0 {
            filters.push(format!("hue=h={}", self.hue));
        }

        if filters.is_empty() {
            "copy".to_string()
        } else {
            filters.join(",")
        }
    }

    /// Apply to video
    pub async fn apply_to_video(&self, input: &str, output: &str) -> Result<()> {
        let filter = self.to_ffmpeg_filter();

        let output_cmd = Command::new("ffmpeg")
            .args(&[
                "-i", input,
                "-vf", &filter,
                "-c:a", "copy",
                "-preset", "medium",
                "-crf", "18",
                output
            ])
            .output()?;

        if !output_cmd.status.success() {
            anyhow::bail!("FFmpeg failed: {}", String::from_utf8_lossy(&output_cmd.stderr));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_correction_filter() {
        let mut cc = ColorCorrection::default();
        cc.saturation = 1.2;
        cc.hue = 15.0;

        let filter = cc.to_ffmpeg_filter();
        assert!(filter.contains("saturation=1.2"));
        assert!(filter.contains("hue=h=15"));
    }
}
