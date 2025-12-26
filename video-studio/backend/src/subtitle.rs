use serde::{Deserialize, Serialize};
use std::path::Path;
use std::process::Command;
use tokio::fs;
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct SubtitleEntry {
    pub index: usize,
    pub start: f64,
    pub end: f64,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AeneasConfig {
    pub text: String,
    pub audio_path: String,
    pub language: String, // "tur" for Turkish, "eng" for English
}

/// Generate timed subtitles using Aeneas
/// Requires: pip install aeneas
pub async fn generate_subtitles_aeneas(config: AeneasConfig) -> Result<Vec<SubtitleEntry>> {
    // Create temp files
    let text_file = format!("/tmp/aeneas_text_{}.txt", uuid::Uuid::new_v4());
    let output_file = format!("/tmp/aeneas_output_{}.json", uuid::Uuid::new_v4());

    // Write text to temp file
    fs::write(&text_file, &config.text).await?;

    // Run aeneas
    let output = Command::new("python3")
        .args([
            "-m",
            "aeneas.tools.execute_task",
            &config.audio_path,
            &text_file,
            &format!("task_language={}|os_task_file_format=json|is_text_type=plain", config.language),
            &output_file,
        ])
        .output()?;

    if !output.status.success() {
        anyhow::bail!("Aeneas failed: {}", String::from_utf8_lossy(&output.stderr));
    }

    // Parse output
    let json_content = fs::read_to_string(&output_file).await?;
    let aeneas_output: AeneasOutput = serde_json::from_str(&json_content)?;

    // Cleanup
    let _ = fs::remove_file(&text_file).await;
    let _ = fs::remove_file(&output_file).await;

    // Convert to SubtitleEntry
    Ok(aeneas_output.fragments.into_iter().enumerate().map(|(i, f)| SubtitleEntry {
        index: i + 1,
        start: f.begin,
        end: f.end,
        text: f.lines.join(" "),
    }).collect())
}

#[derive(Debug, Deserialize)]
struct AeneasOutput {
    fragments: Vec<AeneasFragment>,
}

#[derive(Debug, Deserialize)]
struct AeneasFragment {
    begin: f64,
    end: f64,
    lines: Vec<String>,
}

/// Generate subtitles using Whisper (auto transcribe + timing)
/// Requires: pip install openai-whisper
pub async fn generate_subtitles_whisper(audio_path: String, language: Option<String>) -> Result<Vec<SubtitleEntry>> {
    let output_dir = "/tmp";
    let lang_arg = language.unwrap_or_else(|| "tr".to_string());

    // Run Whisper
    let output = Command::new("whisper")
        .args([
            &audio_path,
            "--model", "base",
            "--language", &lang_arg,
            "--output_format", "srt",
            "--output_dir", output_dir,
        ])
        .output()?;

    if !output.status.success() {
        anyhow::bail!("Whisper failed: {}", String::from_utf8_lossy(&output.stderr));
    }

    // Find generated SRT file
    let audio_name = Path::new(&audio_path).file_stem().unwrap().to_str().unwrap();
    let srt_path = format!("{}/{}.srt", output_dir, audio_name);

    // Parse SRT
    parse_srt_file(&srt_path).await
}

/// Parse SRT file
async fn parse_srt_file(path: &str) -> Result<Vec<SubtitleEntry>> {
    let content = fs::read_to_string(path).await?;
    let mut entries = Vec::new();

    for block in content.trim().split("\n\n") {
        let lines: Vec<&str> = block.lines().collect();
        if lines.len() >= 3 {
            let index = lines[0].parse::<usize>()?;
            let times: Vec<&str> = lines[1].split(" --> ").collect();
            if times.len() == 2 {
                let start = parse_srt_time(times[0])?;
                let end = parse_srt_time(times[1])?;
                let text = lines[2..].join("\n");
                entries.push(SubtitleEntry { index, start, end, text });
            }
        }
    }

    Ok(entries)
}

/// Convert SRT time to seconds
fn parse_srt_time(time: &str) -> Result<f64> {
    // Format: HH:MM:SS,mmm
    let parts: Vec<&str> = time.split(':').collect();
    if parts.len() != 3 {
        anyhow::bail!("Invalid SRT time format");
    }

    let hours: f64 = parts[0].parse()?;
    let minutes: f64 = parts[1].parse()?;
    let sec_parts: Vec<&str> = parts[2].split(',').collect();
    let seconds: f64 = sec_parts[0].parse()?;
    let millis: f64 = if sec_parts.len() > 1 {
        sec_parts[1].parse::<f64>()? / 1000.0
    } else {
        0.0
    };

    Ok(hours * 3600.0 + minutes * 60.0 + seconds + millis)
}

/// Convert subtitle entries to SRT format
pub fn entries_to_srt(entries: &[SubtitleEntry]) -> String {
    entries.iter().map(|e| {
        format!(
            "{}\n{} --> {}\n{}\n",
            e.index,
            seconds_to_srt_time(e.start),
            seconds_to_srt_time(e.end),
            e.text
        )
    }).collect::<Vec<_>>().join("\n")
}

/// Convert subtitle entries to ASS format (for embedding)
pub fn entries_to_ass(entries: &[SubtitleEntry]) -> String {
    let mut ass = String::from(
r#"[Script Info]
Title: Auto-generated Subtitles
ScriptType: v4.00+
PlayResX: 1080
PlayResY: 1920
WrapStyle: 0

[V4+ Styles]
Format: Name, Fontname, Fontsize, PrimaryColour, SecondaryColour, OutlineColour, BackColour, Bold, Italic, Underline, StrikeOut, ScaleX, ScaleY, Spacing, Angle, BorderStyle, Outline, Shadow, Alignment, MarginL, MarginR, MarginV, Encoding
Style: Default,Arial Black,80,&H00FFFFFF,&H000000FF,&H00000000,&H64000000,-1,0,0,0,100,100,0,0,1,6,0,2,540,0,150,1

[Events]
Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text
"#);

    for entry in entries {
        ass.push_str(&format!(
            "Dialogue: 0,{},{},Default,,0,0,0,,{}\n",
            seconds_to_ass_time(entry.start),
            seconds_to_ass_time(entry.end),
            entry.text.replace('\n', "\\N")
        ));
    }

    ass
}

fn seconds_to_srt_time(seconds: f64) -> String {
    let hours = (seconds / 3600.0) as u32;
    let minutes = ((seconds % 3600.0) / 60.0) as u32;
    let secs = (seconds % 60.0) as u32;
    let millis = ((seconds % 1.0) * 1000.0) as u32;
    format!("{:02}:{:02}:{:02},{:03}", hours, minutes, secs, millis)
}

fn seconds_to_ass_time(seconds: f64) -> String {
    let hours = (seconds / 3600.0) as u32;
    let minutes = ((seconds % 3600.0) / 60.0) as u32;
    let secs = (seconds % 60.0) as u32;
    let centisecs = ((seconds % 1.0) * 100.0) as u32;
    format!("{}:{:02}:{:02}.{:02}", hours, minutes, secs, centisecs)
}
