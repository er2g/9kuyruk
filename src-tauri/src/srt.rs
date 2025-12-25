use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use tauri::command;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SrtEntry {
    pub index: usize,
    pub start: String,
    pub end: String,
    pub text: String,
    pub start_seconds: f64,
    pub end_seconds: f64,
}

/// Parse an SRT file and return entries
#[command]
pub async fn parse_srt_file(srt_path: String) -> Result<Vec<SrtEntry>, String> {
    let content = fs::read_to_string(&srt_path)
        .map_err(|e| format!("SRT dosyası okunamadı: {}", e))?;

    let blocks: Vec<&str> = content.trim().split("\n\n").collect();
    let mut entries = Vec::new();

    for block in blocks {
        let lines: Vec<&str> = block.trim().split('\n').collect();
        if lines.len() >= 3 {
            // Parse index
            let index = lines[0].trim().parse::<usize>().unwrap_or(0);

            // Parse time
            let time_parts: Vec<&str> = lines[1].split(" --> ").collect();
            if time_parts.len() == 2 {
                let start = time_parts[0].trim().to_string();
                let end = time_parts[1].trim().to_string();

                let start_seconds = srt_time_to_seconds(&start);
                let end_seconds = srt_time_to_seconds(&end);

                // Join remaining lines as text
                let text = lines[2..].join(" ");

                entries.push(SrtEntry {
                    index,
                    start,
                    end,
                    text,
                    start_seconds,
                    end_seconds,
                });
            }
        }
    }

    Ok(entries)
}

/// Convert SRT file to ASS format
#[command]
pub async fn srt_to_ass(srt_path: String, ass_path: String) -> Result<String, String> {
    let entries = parse_srt_file(srt_path).await?;

    let mut ass_content = String::from(
r#"[Script Info]
Title: Sag Alt Ortaya Hizali Yazı Kutusu
ScriptType: v4.00+
PlayResX: 1080
PlayResY: 1920
WrapStyle: 0
ScaledBorderAndShadow: yes
Collisions: Normal

[V4+ Styles]
Format: Name, Fontname, Fontsize, PrimaryColour, SecondaryColour, OutlineColour, BackColour, Bold, Italic, Underline, StrikeOut, ScaleX, ScaleY, Spacing, Angle, BorderStyle, Outline, Shadow, Alignment, MarginL, MarginR, MarginV, Encoding
Style: SagAltOrtali,Arial Black,80,&H00FFFFFF,&H000000FF,&H00000000,&H64000000,-1,0,0,0,100,100,0,0,1,6,0,2,540,0,150,1

[Events]
Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text
"#);

    for entry in entries {
        let start_ass = srt_time_to_ass(&entry.start);
        let end_ass = srt_time_to_ass(&entry.end);
        let text = entry.text.replace("\n", "\\N");

        ass_content.push_str(&format!(
            "Dialogue: 0,{},{},SagAltOrtali,,0,0,0,,{}\n",
            start_ass, end_ass, text
        ));
    }

    fs::write(&ass_path, ass_content)
        .map_err(|e| format!("ASS dosyası yazılamadı: {}", e))?;

    Ok(ass_path)
}

// Helper: Convert SRT time to seconds
fn srt_time_to_seconds(time_str: &str) -> f64 {
    // Format: HH:MM:SS,mmm
    let parts: Vec<&str> = time_str.split(':').collect();
    if parts.len() != 3 {
        return 0.0;
    }

    let hours = parts[0].parse::<f64>().unwrap_or(0.0);
    let minutes = parts[1].parse::<f64>().unwrap_or(0.0);
    let sec_ms: Vec<&str> = parts[2].split(',').collect();
    let seconds = sec_ms[0].parse::<f64>().unwrap_or(0.0);
    let millis = if sec_ms.len() > 1 {
        sec_ms[1].parse::<f64>().unwrap_or(0.0) / 1000.0
    } else {
        0.0
    };

    hours * 3600.0 + minutes * 60.0 + seconds + millis
}

// Helper: Convert SRT time to ASS time format
fn srt_time_to_ass(time_str: &str) -> String {
    // SRT: HH:MM:SS,mmm -> ASS: H:MM:SS.cc
    let parts: Vec<&str> = time_str.split(':').collect();
    if parts.len() != 3 {
        return "0:00:00.00".to_string();
    }

    let h = parts[0].parse::<u32>().unwrap_or(0);
    let m = parts[1].parse::<u32>().unwrap_or(0);
    let sec_ms: Vec<&str> = parts[2].split(',').collect();
    let s = sec_ms[0].parse::<u32>().unwrap_or(0);
    let ms = if sec_ms.len() > 1 {
        sec_ms[1].parse::<u32>().unwrap_or(0) / 10
    } else {
        0
    };

    format!("{}:{:02}:{:02}.{:02}", h, m, s, ms)
}
