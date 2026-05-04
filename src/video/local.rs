use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::process::Stdio;
use tokio::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalVideo {
    pub path: String,
    pub title: String,
    pub duration: f64,
    pub duration_str: String,
    pub width: u32,
    pub height: u32,
    pub codec: String,
    pub size: u64,
}

#[derive(Debug, Clone)]
pub struct LocalLibrary {
    pub videos: Vec<LocalVideo>,
    pub base_path: PathBuf,
}

impl LocalLibrary {
    pub fn new(base_path: PathBuf) -> Self {
        Self {
            videos: Vec::new(),
            base_path,
        }
    }

    /// Scan directory for video files using ffprobe
    pub async fn scan(&mut self) -> Result<(), String> {
        self.videos.clear();
        let mut dirs_to_scan = vec![self.base_path.clone()];
        let video_extensions = ["mp4", "mkv", "avi", "mov", "webm", "flv", "wmv"];

        while let Some(dir) = dirs_to_scan.pop() {
            let mut entries = tokio::fs::read_dir(&dir)
                .await
                .map_err(|e| format!("Failed to read directory: {}", e))?;

            while let Some(entry) = entries.next_entry().await.transpose() {
                match entry {
                    Ok(entry) => {
                        let path = entry.path();
                        if path.is_dir() {
                            dirs_to_scan.push(path);
                        } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                            if video_extensions.contains(&ext.to_lowercase().as_str()) {
                                if let Ok(metadata) = self.probe_video(&path).await {
                                    self.videos.push(metadata);
                                }
                            }
                        }
                    }
                    Err(e) => eprintln!("Error reading entry: {}", e),
                }
            }
        }
        Ok(())
    }

    async fn probe_video(&self, path: &Path) -> Result<LocalVideo, String> {
        let output = Command::new("ffprobe")
            .arg("-v")
            .arg("quiet")
            .arg("-print_format")
            .arg("json")
            .arg("-show_format")
            .arg("-show_streams")
            .arg(path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .map_err(|e| format!("Failed to run ffprobe: {}", e))?;

        if !output.status.success() {
            return Err("ffprobe failed".to_string());
        }

        let json: serde_json::Value = serde_json::from_slice(&output.stdout)
            .map_err(|e| format!("Failed to parse ffprobe output: {}", e))?;

        let format = json.get("format").ok_or("No format info")?;
        let duration = format.get("duration")
            .and_then(|v| v.as_str())
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(0.0);

        let title = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Unknown")
            .to_string();

        let size = format.get("size")
            .and_then(|v| v.as_str())
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(0);

        // Find video stream
        let mut width = 0;
        let mut height = 0;
        let mut codec = "unknown".to_string();

        if let Some(streams) = json.get("streams").and_then(|s| s.as_array()) {
            for stream in streams {
                if stream.get("codec_type")
                    .and_then(|t| t.as_str())
                    .unwrap_or("") == "video"
                {
                    width = stream.get("width")
                        .and_then(|w| w.as_u64())
                        .unwrap_or(0) as u32;
                    height = stream.get("height")
                        .and_then(|h| h.as_u64())
                        .unwrap_or(0) as u32;
                    codec = stream.get("codec_name")
                        .and_then(|c| c.as_str())
                        .unwrap_or("unknown")
                        .to_string();
                    break;
                }
            }
        }

        Ok(LocalVideo {
            path: path.to_string_lossy().to_string(),
            title,
            duration,
            duration_str: format_duration(duration),
            width,
            height,
            codec,
            size,
        })
    }

    /// Check if ffprobe is available
    pub fn is_ffprobe_available() -> bool {
        std::process::Command::new("which")
            .arg("ffprobe")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }
}

fn format_duration(seconds: f64) -> String {
    let hours = (seconds / 3600.0) as u64;
    let minutes = ((seconds % 3600.0) / 60.0) as u64;
    let secs = (seconds % 60.0) as u64;

    if hours > 0 {
        format!("{}:{:02}:{:02}", hours, minutes, secs)
    } else {
        format!("{}:{:02}", minutes, secs)
    }
}
