/// Color space conversion utilities

use super::RGB;

/// Convert Rec.709 to DCI-P3
pub fn rec709_to_dcip3(rgb: RGB) -> RGB {
    // Using simplified Bradford chromatic adaptation
    // In production, use proper 3x3 matrix multiplication

    RGB::new(
        rgb.r * 0.8224 + rgb.g * 0.1776,
        rgb.r * 0.0331 + rgb.g * 0.9669,
        rgb.r * 0.0171 + rgb.g * 0.0720 + rgb.b * 0.9109,
    )
}

/// Convert Rec.709 to Rec.2020
pub fn rec709_to_rec2020(rgb: RGB) -> RGB {
    RGB::new(
        rgb.r * 0.6274 + rgb.g * 0.3293 + rgb.b * 0.0433,
        rgb.r * 0.0691 + rgb.g * 0.9195 + rgb.b * 0.0114,
        rgb.r * 0.0164 + rgb.g * 0.0880 + rgb.b * 0.8956,
    )
}
