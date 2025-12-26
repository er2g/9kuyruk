use serde::{Deserialize, Serialize};
use anyhow::Result;

const TARGET_W: u32 = 1080;
const MAX_H: u32 = 1140;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverlayPlacement {
    pub image_path: String,
    pub start_time: f64,
    pub end_time: f64,
    pub position: Position,
    pub size: Size,
    pub original_size: Size,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub anchor: PositionAnchor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PositionAnchor {
    TopLeft,
    TopCenter,
    TopRight,
    CenterLeft,
    Center,
    CenterRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

/// Smart resize for overlay images (from Python script logic)
pub async fn smart_resize_overlay(image_path: &str) -> Result<Size> {
    // Get original dimensions using ffprobe
    let output = std::process::Command::new("ffprobe")
        .args([
            "-v", "error",
            "-select_streams", "v:0",
            "-show_entries", "stream=width,height",
            "-of", "csv=p=0",
            image_path,
        ])
        .output()?;

    let dimensions = String::from_utf8_lossy(&output.stdout);
    let parts: Vec<&str> = dimensions.trim().split(',').collect();

    if parts.len() != 2 {
        anyhow::bail!("Failed to get image dimensions");
    }

    let orig_width: u32 = parts[0].parse()?;
    let orig_height: u32 = parts[1].parse()?;

    // First resize to target width
    let mut new_width = TARGET_W;
    let mut new_height = (orig_height as f64 * (TARGET_W as f64 / orig_width as f64)) as u32;

    // If height exceeds max, resize by height instead
    if new_height > MAX_H {
        new_height = MAX_H;
        new_width = (orig_width as f64 * (MAX_H as f64 / orig_height as f64)) as u32;
    }

    Ok(Size {
        width: new_width,
        height: new_height,
    })
}

/// Calculate position based on anchor and video dimensions
pub fn calculate_position(
    anchor: &PositionAnchor,
    overlay_size: &Size,
    video_width: u32,
    video_height: u32,
    offset_x: i32,
    offset_y: i32,
) -> (i32, i32) {
    let (base_x, base_y) = match anchor {
        PositionAnchor::TopLeft => (0, 0),
        PositionAnchor::TopCenter => ((video_width - overlay_size.width) / 2, 0),
        PositionAnchor::TopRight => (video_width - overlay_size.width, 0),
        PositionAnchor::CenterLeft => (0, (video_height - overlay_size.height) / 2),
        PositionAnchor::Center => (
            (video_width - overlay_size.width) / 2,
            (video_height - overlay_size.height) / 2,
        ),
        PositionAnchor::CenterRight => (
            video_width - overlay_size.width,
            (video_height - overlay_size.height) / 2,
        ),
        PositionAnchor::BottomLeft => (0, video_height - overlay_size.height),
        PositionAnchor::BottomCenter => (
            (video_width - overlay_size.width) / 2,
            video_height - overlay_size.height,
        ),
        PositionAnchor::BottomRight => (
            video_width - overlay_size.width,
            video_height - overlay_size.height,
        ),
    };

    (base_x as i32 + offset_x, base_y as i32 + offset_y)
}

/// Find numbered images in folder (1.png, 2.jpg, etc.)
pub async fn find_numbered_images(folder_path: &str) -> Result<Vec<String>> {
    let valid_exts = ["png", "jpg", "jpeg", "webp"];
    let mut images = Vec::new();
    let mut index = 1;

    loop {
        let mut found = false;
        for ext in &valid_exts {
            let file_path = format!("{}/{}.{}", folder_path, index, ext);
            if tokio::fs::metadata(&file_path).await.is_ok() {
                images.push(file_path);
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

/// Auto-distribute images across video duration
pub async fn auto_distribute_overlays(
    images: Vec<String>,
    video_duration: f64,
) -> Result<Vec<OverlayPlacement>> {
    let count = images.len();
    if count == 0 {
        return Ok(Vec::new());
    }

    let step = video_duration / count as f64;
    let mut placements = Vec::new();

    for (i, image_path) in images.iter().enumerate() {
        let start_time = i as f64 * step;
        let end_time = if i < count - 1 {
            (i + 1) as f64 * step
        } else {
            video_duration
        };

        let resized = smart_resize_overlay(image_path).await?;

        placements.push(OverlayPlacement {
            image_path: image_path.clone(),
            start_time,
            end_time,
            position: Position {
                x: 0,
                y: 0,
                anchor: PositionAnchor::TopCenter, // Default from Python script
            },
            size: resized.clone(),
            original_size: resized,
        });
    }

    Ok(placements)
}

/// Generate FFmpeg filter for overlays
pub fn generate_overlay_filter(placements: &[OverlayPlacement], video_width: u32, video_height: u32) -> String {
    if placements.is_empty() {
        return String::new();
    }

    let mut filter = String::new();

    // Load and scale all overlays
    for (i, placement) in placements.iter().enumerate() {
        if i > 0 {
            filter.push(';');
        }
        filter.push_str(&format!(
            "movie={}:loop=0,setpts=N/(FRAME_RATE*TB),scale={}:{}[ovr{}]",
            placement.image_path.replace(':', "\\:"),
            placement.size.width,
            placement.size.height,
            i
        ));
    }

    // Apply overlays with timing
    filter.push_str(";[0:v]");
    for (i, placement) in placements.iter().enumerate() {
        let (x, y) = calculate_position(
            &placement.position.anchor,
            &placement.size,
            video_width,
            video_height,
            placement.position.x,
            placement.position.y,
        );

        filter.push_str(&format!(
            "[ovr{}]overlay={}:{}:enable='between(t,{},{})'",
            i, x, y, placement.start_time, placement.end_time
        ));

        if i < placements.len() - 1 {
            filter.push_str(&format!("[tmp{}];[tmp{}]", i, i));
        }
    }

    filter
}
