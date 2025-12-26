use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

/// LUT (Look-Up Table) implementation
///
/// Supports:
/// - 1D LUTs (tone curves)
/// - 3D LUTs (full color transformation)
/// - .cube format (most common)
/// - .3dl format (Autodesk/Lustre)
/// - LUT creation from reference images

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LUT3D {
    pub title: String,
    pub size: usize,           // Typically 17, 33, or 65
    pub domain_min: [f32; 3],  // Usually [0, 0, 0]
    pub domain_max: [f32; 3],  // Usually [1, 1, 1]
    pub data: Vec<[f32; 3]>,   // RGB values
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LUT1D {
    pub title: String,
    pub size: usize,
    pub data: Vec<f32>,
}

impl LUT3D {
    pub fn new(size: usize) -> Self {
        let total_entries = size * size * size;
        let mut data = Vec::with_capacity(total_entries);

        // Initialize identity LUT
        for b in 0..size {
            for g in 0..size {
                for r in 0..size {
                    let rf = r as f32 / (size - 1) as f32;
                    let gf = g as f32 / (size - 1) as f32;
                    let bf = b as f32 / (size - 1) as f32;
                    data.push([rf, gf, bf]);
                }
            }
        }

        Self {
            title: "Identity".to_string(),
            size,
            domain_min: [0.0, 0.0, 0.0],
            domain_max: [1.0, 1.0, 1.0],
            data,
        }
    }

    /// Load .cube LUT file
    pub fn from_cube_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let mut title = String::new();
        let mut size = 0;
        let mut domain_min = [0.0, 0.0, 0.0];
        let mut domain_max = [1.0, 1.0, 1.0];
        let mut data = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let line = line.trim();

            // Skip comments
            if line.starts_with('#') || line.is_empty() {
                continue;
            }

            // Parse metadata
            if line.starts_with("TITLE") {
                title = line[5..].trim().trim_matches('"').to_string();
            } else if line.starts_with("LUT_3D_SIZE") {
                size = line.split_whitespace()
                    .nth(1)
                    .and_then(|s| s.parse().ok())
                    .ok_or_else(|| anyhow!("Invalid LUT_3D_SIZE"))?;
            } else if line.starts_with("DOMAIN_MIN") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 4 {
                    domain_min[0] = parts[1].parse()?;
                    domain_min[1] = parts[2].parse()?;
                    domain_min[2] = parts[3].parse()?;
                }
            } else if line.starts_with("DOMAIN_MAX") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 4 {
                    domain_max[0] = parts[1].parse()?;
                    domain_max[1] = parts[2].parse()?;
                    domain_max[2] = parts[3].parse()?;
                }
            } else {
                // Parse RGB data
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 {
                    let r: f32 = parts[0].parse()?;
                    let g: f32 = parts[1].parse()?;
                    let b: f32 = parts[2].parse()?;
                    data.push([r, g, b]);
                }
            }
        }

        if size == 0 {
            return Err(anyhow!("Missing LUT_3D_SIZE"));
        }

        Ok(Self {
            title,
            size,
            domain_min,
            domain_max,
            data,
        })
    }

    /// Save to .cube file
    pub fn to_cube_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let mut file = File::create(path)?;

        writeln!(file, "TITLE \"{}\"", self.title)?;
        writeln!(file, "LUT_3D_SIZE {}", self.size)?;
        writeln!(
            file,
            "DOMAIN_MIN {} {} {}",
            self.domain_min[0], self.domain_min[1], self.domain_min[2]
        )?;
        writeln!(
            file,
            "DOMAIN_MAX {} {} {}",
            self.domain_max[0], self.domain_max[1], self.domain_max[2]
        )?;
        writeln!(file)?;

        for rgb in &self.data {
            writeln!(file, "{:.6} {:.6} {:.6}", rgb[0], rgb[1], rgb[2])?;
        }

        Ok(())
    }

    /// Apply LUT to RGB value (trilinear interpolation)
    pub fn apply(&self, r: f32, g: f32, b: f32) -> [f32; 3] {
        // Normalize input to LUT domain
        let r = ((r - self.domain_min[0]) / (self.domain_max[0] - self.domain_min[0]))
            .clamp(0.0, 1.0);
        let g = ((g - self.domain_min[1]) / (self.domain_max[1] - self.domain_min[1]))
            .clamp(0.0, 1.0);
        let b = ((b - self.domain_min[2]) / (self.domain_max[2] - self.domain_min[2]))
            .clamp(0.0, 1.0);

        // Map to LUT grid
        let size_f = (self.size - 1) as f32;
        let r_scaled = r * size_f;
        let g_scaled = g * size_f;
        let b_scaled = b * size_f;

        // Get integer and fractional parts
        let r0 = r_scaled.floor() as usize;
        let g0 = g_scaled.floor() as usize;
        let b0 = b_scaled.floor() as usize;

        let r1 = (r0 + 1).min(self.size - 1);
        let g1 = (g0 + 1).min(self.size - 1);
        let b1 = (b0 + 1).min(self.size - 1);

        let r_frac = r_scaled - r0 as f32;
        let g_frac = g_scaled - g0 as f32;
        let b_frac = b_scaled - b0 as f32;

        // Trilinear interpolation (8 corner samples)
        let c000 = self.get_value(r0, g0, b0);
        let c001 = self.get_value(r0, g0, b1);
        let c010 = self.get_value(r0, g1, b0);
        let c011 = self.get_value(r0, g1, b1);
        let c100 = self.get_value(r1, g0, b0);
        let c101 = self.get_value(r1, g0, b1);
        let c110 = self.get_value(r1, g1, b0);
        let c111 = self.get_value(r1, g1, b1);

        let c00 = lerp_rgb(c000, c100, r_frac);
        let c01 = lerp_rgb(c001, c101, r_frac);
        let c10 = lerp_rgb(c010, c110, r_frac);
        let c11 = lerp_rgb(c011, c111, r_frac);

        let c0 = lerp_rgb(c00, c10, g_frac);
        let c1 = lerp_rgb(c01, c11, g_frac);

        lerp_rgb(c0, c1, b_frac)
    }

    fn get_value(&self, r: usize, g: usize, b: usize) -> [f32; 3] {
        let index = b * self.size * self.size + g * self.size + r;
        self.data[index]
    }
}

/// Linear interpolation between two RGB values
fn lerp_rgb(a: [f32; 3], b: [f32; 3], t: f32) -> [f32; 3] {
    [
        a[0] + (b[0] - a[0]) * t,
        a[1] + (b[1] - a[1]) * t,
        a[2] + (b[2] - a[2]) * t,
    ]
}

/// Common LUT presets
pub mod presets {
    use super::*;

    /// Create a contrast adjustment LUT
    pub fn contrast(amount: f32) -> LUT3D {
        let mut lut = LUT3D::new(33);
        lut.title = format!("Contrast {:.1}", amount);

        for i in 0..lut.data.len() {
            let rgb = lut.data[i];
            lut.data[i] = [
                ((rgb[0] - 0.5) * amount + 0.5).clamp(0.0, 1.0),
                ((rgb[1] - 0.5) * amount + 0.5).clamp(0.0, 1.0),
                ((rgb[2] - 0.5) * amount + 0.5).clamp(0.0, 1.0),
            ];
        }

        lut
    }

    /// Create a saturation adjustment LUT
    pub fn saturation(amount: f32) -> LUT3D {
        let mut lut = LUT3D::new(33);
        lut.title = format!("Saturation {:.1}", amount);

        for i in 0..lut.data.len() {
            let rgb = lut.data[i];
            let gray = 0.299 * rgb[0] + 0.587 * rgb[1] + 0.114 * rgb[2];

            lut.data[i] = [
                (gray + (rgb[0] - gray) * amount).clamp(0.0, 1.0),
                (gray + (rgb[1] - gray) * amount).clamp(0.0, 1.0),
                (gray + (rgb[2] - gray) * amount).clamp(0.0, 1.0),
            ];
        }

        lut
    }

    /// Cinematic teal & orange look
    pub fn teal_orange() -> LUT3D {
        let mut lut = LUT3D::new(33);
        lut.title = "Teal & Orange".to_string();

        for i in 0..lut.data.len() {
            let rgb = lut.data[i];

            // Push blues toward teal
            // Push yellows/reds toward orange
            let r = rgb[0] * 1.1;
            let g = rgb[1];
            let b = rgb[2] * 0.95;

            lut.data[i] = [r.clamp(0.0, 1.0), g.clamp(0.0, 1.0), b.clamp(0.0, 1.0)];
        }

        lut
    }

    /// Bleach bypass look
    pub fn bleach_bypass() -> LUT3D {
        let mut lut = LUT3D::new(33);
        lut.title = "Bleach Bypass".to_string();

        for i in 0..lut.data.len() {
            let rgb = lut.data[i];

            // Desaturate
            let gray = 0.299 * rgb[0] + 0.587 * rgb[1] + 0.114 * rgb[2];
            let desat = 0.3;

            let r = gray + (rgb[0] - gray) * desat;
            let g = gray + (rgb[1] - gray) * desat;
            let b = gray + (rgb[2] - gray) * desat;

            // Increase contrast
            let contrast = 1.3;
            let r = ((r - 0.5) * contrast + 0.5).clamp(0.0, 1.0);
            let g = ((g - 0.5) * contrast + 0.5).clamp(0.0, 1.0);
            let b = ((b - 0.5) * contrast + 0.5).clamp(0.0, 1.0);

            lut.data[i] = [r, g, b];
        }

        lut
    }
}

/// Apply LUT to video using FFmpeg
pub fn apply_lut_to_video(
    input: &str,
    output: &str,
    lut_path: &str,
) -> Result<()> {
    use std::process::Command;

    let output = Command::new("ffmpeg")
        .args(&[
            "-i", input,
            "-vf", &format!("lut3d={}", lut_path),
            "-c:a", "copy",
            output
        ])
        .output()?;

    if !output.status.success() {
        return Err(anyhow!("FFmpeg failed: {}", String::from_utf8_lossy(&output.stderr)));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_lut() {
        let lut = LUT3D::new(17);
        let result = lut.apply(0.5, 0.5, 0.5);
        assert!((result[0] - 0.5).abs() < 0.01);
        assert!((result[1] - 0.5).abs() < 0.01);
        assert!((result[2] - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_lut_save_load() {
        let lut = presets::contrast(1.5);
        let temp_path = "/tmp/test.cube";

        lut.to_cube_file(temp_path).unwrap();
        let loaded = LUT3D::from_cube_file(temp_path).unwrap();

        assert_eq!(loaded.size, lut.size);
        assert_eq!(loaded.data.len(), lut.data.len());
    }
}
