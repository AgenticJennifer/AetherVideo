/// IPTV M3U playlist parser and channel browser
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Channel {
    pub name: String,
    pub url: String,
    pub group: Option<String>,
    pub logo: Option<String>,
    pub tvg_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Playlist {
    pub channels: Vec<Channel>,
    pub groups: Vec<String>,
}

/// Parse an M3U playlist from a file path or URL content
pub fn parse_m3u(content: &str) -> Playlist {
    let mut channels = Vec::new();
    let mut groups = Vec::new();
    let mut current_attrs: HashMap<String, String> = HashMap::new();
    let mut current_name = String::new();

    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("#EXTINF:") {
            // Parse extended info
            let info = &trimmed[8..];
            current_attrs.clear();

            // Extract attributes like tvg-name, group-title, tvg-logo
            if let Some(tvg_id) = extract_attr(info, "tvg-id") {
                current_attrs.insert("tvg_id".to_string(), tvg_id);
            }
            if let Some(group) = extract_attr(info, "group-title") {
                let group_clone = group.clone();
                current_attrs.insert("group".to_string(), group_clone.clone());
                if !groups.contains(&group_clone) {
                    groups.push(group_clone);
                }
            }
            if let Some(logo) = extract_attr(info, "tvg-logo") {
                current_attrs.insert("logo".to_string(), logo);
            }

            // Extract the channel name (after the comma)
            if let Some(comma_pos) = info.rfind(',') {
                current_name = info[comma_pos + 1..].trim().to_string();
            }
        } else if trimmed.starts_with("#") {
            // Other M3U directives - skip
            continue;
        } else if !trimmed.is_empty() {
            // This should be the URL
            let channel = Channel {
                name: if current_name.is_empty() {
                    trimmed.to_string()
                } else {
                    current_name.clone()
                },
                url: trimmed.to_string(),
                group: current_attrs.get("group").cloned(),
                logo: current_attrs.get("logo").cloned(),
                tvg_id: current_attrs.get("tvg_id").cloned(),
            };
            channels.push(channel);
            current_name.clear();
            current_attrs.clear();
        }
    }

    groups.sort();
    groups.dedup();

    Playlist { channels, groups }
}

/// Load M3U playlist from a local file
pub fn load_playlist<P: AsRef<Path>>(path: P) -> Result<Playlist, String> {
    let content = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read playlist: {}", e))?;
    Ok(parse_m3u(&content))
}

/// Load M3U playlist from a URL (requires reqwest)
pub async fn fetch_playlist(url: &str) -> Result<Playlist, String> {
    let response = reqwest::get(url)
        .await
        .map_err(|e| format!("Failed to fetch playlist: {}", e))?;
    let content = response
        .text()
        .await
        .map_err(|e| format!("Failed to read response: {}", e))?;
    Ok(parse_m3u(&content))
}

fn extract_attr(info: &str, attr: &str) -> Option<String> {
    let prefix = format!("{}=\"", attr);
    if let Some(start) = info.find(&prefix) {
        let start = start + prefix.len();
        if let Some(end) = info[start..].find('"') {
            return Some(info[start..start + end].to_string());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_m3u() {
        let content = r#"#EXTM3U
#EXTINF:-1 tvg-id="ch1" group-title="News" tvg-logo="http://example.com/logo.png",BBC News
http://example.com/bbc.m3u8
#EXTINF:-1,CNN
http://example.com/cnn.m3u8
"#;
        let playlist = parse_m3u(content);
        assert_eq!(playlist.channels.len(), 2);
        assert_eq!(playlist.channels[0].name, "BBC News");
        assert_eq!(playlist.channels[0].group, Some("News".to_string()));
        assert_eq!(playlist.channels[1].name, "CNN");
    }
}
