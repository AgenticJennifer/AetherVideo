use serde::{Deserialize, Serialize};
use std::process::Stdio;
use tokio::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YouTubeVideo {
    pub id: String,
    pub title: String,
    pub channel: String,
    pub duration: String,
    pub thumbnail: String,
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct YouTubeSearchResult {
    pub videos: Vec<YouTubeVideo>,
    pub next_page_token: Option<String>,
}

/// Search YouTube using yt-dlp (no API key needed)
pub async fn search_youtube(query: &str, _page_token: Option<&str>) -> Result<YouTubeSearchResult, String> {
    let output = Command::new("yt-dlp")
        .arg("--flat-playlist")
        .arg("--print-json")
        .arg("--skip-download")
        .arg("--playlist-end")
        .arg("20")
        .arg(format!("ytsearch20:{}", query))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(|e| format!("Failed to run yt-dlp: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("yt-dlp error: {}", stderr));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut videos = Vec::new();

    for line in stdout.lines() {
        if line.trim().is_empty() {
            continue;
        }
        if let Ok(json_val) = serde_json::from_str::<serde_json::Value>(line) {
            let id = json_val.get("id")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let title = json_val.get("title")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown")
                .to_string();
            let url = format!("https://www.youtube.com/watch?v={}", id);

            videos.push(YouTubeVideo {
                id: id.clone(),
                title,
                channel: json_val.get("channel")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Unknown")
                    .to_string(),
                duration: format_duration(json_val.get("duration").and_then(|v| v.as_f64())),
                thumbnail: format!("https://img.youtube.com/vi/{}/mqdefault.jpg", id),
                url,
            });
        }
    }

    Ok(YouTubeSearchResult {
        videos,
        next_page_token: None,
    })
}

/// Get direct stream URL for a YouTube video
pub async fn get_stream_url(video_id: &str) -> Result<String, String> {
    let output = Command::new("yt-dlp")
        .arg("-f")
        .arg("bestvideo[ext=mp4]+bestaudio[ext=m4a]/mp4")
        .arg("-g")
        .arg(format!("https://www.youtube.com/watch?v={}", video_id))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(|e| format!("Failed to run yt-dlp: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("yt-dlp error: {}", stderr));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let url = stdout.lines().next().unwrap_or("").trim().to_string();

    if url.is_empty() {
        Err("No stream URL found".to_string())
    } else {
        Ok(url)
    }
}

/// Check if yt-dlp is installed
pub fn is_ytdlp_available() -> bool {
    std::process::Command::new("which")
        .arg("yt-dlp")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn format_duration(seconds: Option<f64>) -> String {
    let seconds = match seconds {
        Some(s) => s as u64,
        None => return "Unknown".to_string(),
    };
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;

    if hours > 0 {
        format!("{}:{:02}:{:02}", hours, minutes, secs)
    } else {
        format!("{}:{:02}", minutes, secs)
    }
}
