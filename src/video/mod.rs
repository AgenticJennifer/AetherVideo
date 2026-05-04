/// Video source abstraction layer for AetherStream
pub mod iptv;
pub mod player;
pub mod youtube;
pub mod local;
pub mod jellyfin;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoItem {
    pub id: String,
    pub title: String,
    pub url: String,
    pub description: Option<String>,
    pub thumbnail: Option<String>,
    pub duration: Option<u64>, // in seconds
    pub source: VideoSource,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VideoSource {
    IPTV,
    YouTube,
    Local,
    Jellyfin,
}

/// M3U playlist entry
#[derive(Debug, Clone)]
pub struct M3UEntry {
    pub name: String,
    pub url: String,
    pub group: Option<String>,
    pub logo: Option<String>,
    pub description: Option<String>,
}
