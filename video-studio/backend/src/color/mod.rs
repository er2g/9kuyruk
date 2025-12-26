/// Professional Color Management System
///
/// Implements industry-standard color workflows:
/// - ACES (Academy Color Encoding System)
/// - Color space conversions (Rec.709, DCI-P3, Rec.2020)
/// - LUT (Look-Up Table) application and creation
/// - Color scopes (Waveform, Vectorscope, Histogram, RGB Parade)
/// - Color matching between clips

pub mod aces;
pub mod lut;
pub mod scopes;
pub mod match_color;
pub mod color_space;

pub use aces::*;
pub use lut::*;
pub use scopes::*;
pub use match_color::*;
pub use color_space::*;

use serde::{Deserialize, Serialize};

/// Standard color spaces used in video production
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ColorSpace {
    /// Rec.709 (HD, sRGB) - Most common for web/broadcast
    Rec709,
    /// DCI-P3 (Digital Cinema)
    DCIP3,
    /// Rec.2020 (UHD, HDR)
    Rec2020,
    /// Rec.2100 (HDR10, HLG)
    Rec2100,
    /// ACES (Academy Color Encoding System)
    ACES,
    /// ACEScg (CG working space)
    ACEScg,
    /// sRGB (web standard)
    SRGB,
}

/// Transfer function (gamma curve)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransferFunction {
    /// Standard Gamma 2.2
    Gamma22,
    /// sRGB transfer function
    SRGB,
    /// Linear (no gamma)
    Linear,
    /// Rec.709
    Rec709,
    /// PQ (Perceptual Quantizer) for HDR
    PQ,
    /// HLG (Hybrid Log-Gamma) for HDR
    HLG,
}

/// Color primaries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorPrimaries {
    pub red_x: f64,
    pub red_y: f64,
    pub green_x: f64,
    pub green_y: f64,
    pub blue_x: f64,
    pub blue_y: f64,
    pub white_x: f64,
    pub white_y: f64,
}

impl ColorSpace {
    pub fn primaries(&self) -> ColorPrimaries {
        match self {
            ColorSpace::Rec709 | ColorSpace::SRGB => ColorPrimaries {
                red_x: 0.64,
                red_y: 0.33,
                green_x: 0.30,
                green_y: 0.60,
                blue_x: 0.15,
                blue_y: 0.06,
                white_x: 0.3127,
                white_y: 0.3290,
            },
            ColorSpace::DCIP3 => ColorPrimaries {
                red_x: 0.680,
                red_y: 0.320,
                green_x: 0.265,
                green_y: 0.690,
                blue_x: 0.150,
                blue_y: 0.060,
                white_x: 0.3127,
                white_y: 0.3290,
            },
            ColorSpace::Rec2020 | ColorSpace::Rec2100 => ColorPrimaries {
                red_x: 0.708,
                red_y: 0.292,
                green_x: 0.170,
                green_y: 0.797,
                blue_x: 0.131,
                blue_y: 0.046,
                white_x: 0.3127,
                white_y: 0.3290,
            },
            _ => ColorPrimaries {
                red_x: 0.64,
                red_y: 0.33,
                green_x: 0.30,
                green_y: 0.60,
                blue_x: 0.15,
                blue_y: 0.06,
                white_x: 0.3127,
                white_y: 0.3290,
            },
        }
    }

    pub fn to_ffmpeg_string(&self) -> &str {
        match self {
            ColorSpace::Rec709 | ColorSpace::SRGB => "bt709",
            ColorSpace::DCIP3 => "smpte431",
            ColorSpace::Rec2020 | ColorSpace::Rec2100 => "bt2020",
            _ => "bt709",
        }
    }
}

/// RGB color value (0.0 - 1.0)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct RGB {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

/// YUV color value
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct YUV {
    pub y: f64,
    pub u: f64,
    pub v: f64,
}

impl RGB {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self {
            r: r.clamp(0.0, 1.0),
            g: g.clamp(0.0, 1.0),
            b: b.clamp(0.0, 1.0),
        }
    }

    /// Convert RGB to YUV (Rec.709)
    pub fn to_yuv(&self) -> YUV {
        let y = 0.2126 * self.r + 0.7152 * self.g + 0.0722 * self.b;
        let u = (self.b - y) / 1.8556;
        let v = (self.r - y) / 1.5748;

        YUV { y, u, v }
    }

    /// Apply gamma correction
    pub fn apply_gamma(&self, gamma: f64) -> RGB {
        RGB {
            r: self.r.powf(gamma),
            g: self.g.powf(gamma),
            b: self.b.powf(gamma),
        }
    }
}

impl YUV {
    pub fn new(y: f64, u: f64, v: f64) -> Self {
        Self { y, u, v }
    }

    /// Convert YUV to RGB (Rec.709)
    pub fn to_rgb(&self) -> RGB {
        let r = self.y + 1.5748 * self.v;
        let g = self.y - 0.1873 * self.u - 0.4681 * self.v;
        let b = self.y + 1.8556 * self.u;

        RGB::new(r, g, b)
    }
}
