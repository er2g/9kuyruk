// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod video;
mod srt;
mod ffmpeg;

use tauri::Manager;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

// State to track ongoing render jobs
#[derive(Default)]
struct RenderState {
    jobs: Arc<Mutex<HashMap<String, String>>>,
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .manage(RenderState::default())
        .invoke_handler(tauri::generate_handler![
            video::get_video_info,
            video::get_video_thumbnail,
            video::count_images_in_folder,
            video::generate_preview_frame,
            srt::parse_srt_file,
            srt::srt_to_ass,
            ffmpeg::render_video_with_overlays,
            ffmpeg::embed_subtitles,
            ffmpeg::get_ffmpeg_version,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
