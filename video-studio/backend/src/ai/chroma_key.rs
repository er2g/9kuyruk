use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::process::Command;

/// Chroma Key (Green Screen) - Lightweight alternative to ML background removal
/// Uses FFmpeg's chromakey filter - Very fast and efficient

#[derive(Debug, Serialize, Deserialize)]
pub struct ChromaKeyConfig {
    pub clip_id: String,
    pub key_color: String,      // hex color, e.g., "00FF00" for green
    pub similarity: f64,         // 0.0-1.0, how similar colors to remove
    pub blend: f64,              // 0.0-1.0, edge blending
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChromaKeyResult {
    pub output_path: String,
    pub processing_time: f64,
}

/// Apply chroma key using FFmpeg
pub async fn apply_chroma_key(config: ChromaKeyConfig, input: &str, output: &str) -> Result<ChromaKeyResult> {
    let start = std::time::Instant::now();

    // Convert hex to RGB
    let color = hex_to_rgb(&config.key_color)?;

    // FFmpeg chromakey filter
    let filter = format!(
        "chromakey=color=0x{}:similarity={}:blend={}",
        config.key_color,
        config.similarity,
        config.blend
    );

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

    Ok(ChromaKeyResult {
        output_path: output.to_string(),
        processing_time: start.elapsed().as_secs_f64(),
    })
}

fn hex_to_rgb(hex: &str) -> Result<(u8, u8, u8)> {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 {
        anyhow::bail!("Invalid hex color");
    }

    let r = u8::from_str_radix(&hex[0..2], 16)?;
    let g = u8::from_str_radix(&hex[2..4], 16)?;
    let b = u8::from_str_radix(&hex[4..6], 16)?;

    Ok((r, g, b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_rgb() {
        assert_eq!(hex_to_rgb("00FF00").unwrap(), (0, 255, 0));
        assert_eq!(hex_to_rgb("#FF0000").unwrap(), (255, 0, 0));
    }
}
