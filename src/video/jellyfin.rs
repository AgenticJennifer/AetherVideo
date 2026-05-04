use serde::{Deserialize, Serialize};
use reqwest::Client;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JellyfinServer {
    pub name: String,
    pub url: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JellyfinUser {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JellyfinItem {
    pub id: String,
    pub name: String,
    pub item_type: String,
    pub path: Option<String>,
    pub media_sources: Option<Vec<MediaSource>>,
    pub overview: Option<String>,
    pub run_time_ticks: Option<u64>,
    pub thumbnail_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaSource {
    pub id: String,
    pub path: String,
    pub container: Option<String>,
    pub bitrate: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct JellyfinClient {
    pub server_url: String,
    pub api_key: String,
    pub user_id: String,
    pub client: Client,
}

impl JellyfinClient {
    pub fn new(server_url: String, api_key: String, user_id: String) -> Self {
        Self {
            server_url: server_url.trim_end_matches('/').to_string(),
            api_key,
            user_id,
            client: Client::new(),
        }
    }

    /// Discover Jellyfin servers on local network using SSDP
    pub async fn discover_servers() -> Result<Vec<JellyfinServer>, String> {
        // Simple discovery - in production you'd use SSDP/mDNS
        // For now, return empty - user needs to configure manually
        Ok(Vec::new())
    }

    /// Test connection to a Jellyfin server
    pub async fn test_connection(server_url: &str) -> Result<JellyfinServer, String> {
        let client = Client::new();
        let url = format!("{}/System/Info/Public", server_url.trim_end_matches('/'));

        let response = client.get(&url)
            .send()
            .await
            .map_err(|e| format!("Failed to connect: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("Server returned: {}", response.status()));
        }

        let info: serde_json::Value = response.json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        Ok(JellyfinServer {
            name: info.get("ServerName")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown")
                .to_string(),
            url: server_url.to_string(),
            version: info.get("Version")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown")
                .to_string(),
        })
    }

    /// Get video libraries (collections)
    pub async fn get_video_libraries(&self) -> Result<Vec<JellyfinItem>, String> {
        let url = format!("{}/Users/{}/Views", self.server_url, self.user_id);

        let response = self.client.get(&url)
            .header("X-Emby-Token", &self.api_key)
            .send()
            .await
            .map_err(|e| format!("Failed to fetch libraries: {}", e))?;

        let json: serde_json::Value = response.json()
            .await
            .map_err(|e| format!("Failed to parse: {}", e))?;

        let mut libraries = Vec::new();
        if let Some(items) = json.get("Items").and_then(|i| i.as_array()) {
            for item in items {
                if let Some(item_type) = item.get("CollectionType").and_then(|t| t.as_str()) {
                    if item_type == "movies" || item_type == "tvshows" {
                        libraries.push(JellyfinItem {
                            id: item.get("Id")
                                .and_then(|i| i.as_str())
                                .unwrap_or("")
                                .to_string(),
                            name: item.get("Name")
                                .and_then(|n| n.as_str())
                                .unwrap_or("Unknown")
                                .to_string(),
                            item_type: item_type.to_string(),
                            path: None,
                            media_sources: None,
                            overview: None,
                            run_time_ticks: None,
                            thumbnail_url: None,
                        });
                    }
                }
            }
        }

        Ok(libraries)
    }

    /// Get videos from a library
    pub async fn get_videos(&self, library_id: &str) -> Result<Vec<JellyfinItem>, String> {
        let url = format!(
            "{}/Users/{}/Items?ParentId={}&IncludeItemTypes=Movie,Series,Episode&Recursive=true",
            self.server_url, self.user_id, library_id
        );

        let response = self.client.get(&url)
            .header("X-Emby-Token", &self.api_key)
            .send()
            .await
            .map_err(|e| format!("Failed to fetch videos: {}", e))?;

        let json: serde_json::Value = response.json()
            .await
            .map_err(|e| format!("Failed to parse: {}", e))?;

        let mut videos = Vec::new();
        if let Some(items) = json.get("Items").and_then(|i| i.as_array()) {
            for item in items {
                let id = item.get("Id")
                    .and_then(|i| i.as_str())
                    .unwrap_or("")
                    .to_string();

                videos.push(JellyfinItem {
                    id: id.clone(),
                    name: item.get("Name")
                        .and_then(|n| n.as_str())
                        .unwrap_or("Unknown")
                        .to_string(),
                    item_type: item.get("Type")
                        .and_then(|t| t.as_str())
                        .unwrap_or("Unknown")
                        .to_string(),
                    path: None,
                    media_sources: None,
                    overview: item.get("Overview").and_then(|o| o.as_str()).map(|s| s.to_string()),
                    run_time_ticks: item.get("RunTimeTicks").and_then(|t| t.as_u64()),
                    thumbnail_url: Some(format!(
                        "{}/Items/{}/Images/Primary?maxHeight=200",
                        self.server_url, id
                    )),
                });
            }
        }

        Ok(videos)
    }

    /// Get stream URL for a video
    pub fn get_stream_url(&self, video_id: &str) -> String {
        format!(
            "{}/Videos/{}/stream?api_key={}",
            self.server_url, video_id, self.api_key
        )
    }
}
