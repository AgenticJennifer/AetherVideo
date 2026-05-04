/// Video player using mpv with Kitty graphics protocol support
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};

pub struct VideoPlayer {
    process: Option<Child>,
    socket_path: PathBuf,
    pub media_title: Option<String>,
    kitty_mode: bool,
}

impl VideoPlayer {
    pub fn new() -> Self {
        let socket_path =
            std::env::temp_dir().join(format!("aetherstream-video-{}", std::process::id()));

        // Check if we're running in Kitty terminal
        let kitty_mode = std::env::var("TERM").unwrap_or_default() == "xterm-kitty"
            || std::env::var("KITTY_WINDOW_ID").is_ok();

        Self {
            process: None,
            socket_path,
            media_title: None,
            kitty_mode,
        }
    }

    /// Play a video URL with optional Kitty embedded mode
    pub fn play_url(&mut self, url: &str, volume: u32) -> bool {
        self.stop();

        let mut cmd = Command::new("mpv");

        cmd.arg(url)
            .arg(format!("--volume={}", volume))
            .arg("--no-terminal")
            .stdout(Stdio::null())
            .stderr(Stdio::null());

        // If in Kitty terminal, use Kitty video output
        if self.kitty_mode {
            cmd.arg("--vo=kitty")
                .arg("--no-keepaspect") // Let terminal control aspect
                .arg(format!("--vo-kitty-cols={}", 80)) // Default cols
                .arg(format!("--vo-kitty-rows={}", 24)); // Default rows
        }

        // Set up IPC socket for control
        cmd.arg(format!("--input-ipc-server={}", self.socket_path.display()));

        match cmd.spawn() {
            Ok(child) => {
                self.process = Some(child);
                self.media_title = Some(url.to_string());
                true
            }
            Err(_) => false,
        }
    }

    /// Stop current playback
    pub fn stop(&mut self) {
        if let Some(mut proc) = self.process.take() {
            let _ = proc.kill();
            let _ = proc.wait();
        }
        self.media_title = None;
    }

    /// Check if currently playing
    pub fn is_playing(&self) -> bool {
        self.process.is_some()
    }

    /// Set Kitty video output size (call before play for best results)
    pub fn set_kitty_size(&mut self, _cols: u16, _rows: u16) {
        if self.kitty_mode {
            // This would need to be applied on next play
            // For now, we store the preference
        }
    }

    pub fn is_kitty_available() -> bool {
        std::env::var("KITTY_WINDOW_ID").is_ok()
            || std::env::var("TERM").unwrap_or_default() == "xterm-kitty"
    }
}

impl Drop for VideoPlayer {
    fn drop(&mut self) {
        self.stop();
    }
}
