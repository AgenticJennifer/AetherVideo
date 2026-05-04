/// Video channel browser UI for IPTV and other video sources
use crate::video::iptv::Channel;
use crate::video::VideoItem;
use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

/// State for the video browser panel
pub struct VideoBrowserState {
    pub channels: Vec<Channel>,
    pub selected_index: usize,
    pub filter: String,
    pub groups: Vec<String>,
    pub selected_group: Option<String>,
    pub loading: bool,
    pub status_message: Option<String>,
}

impl Default for VideoBrowserState {
    fn default() -> Self {
        Self {
            channels: Vec::new(),
            selected_index: 0,
            filter: String::new(),
            groups: Vec::new(),
            selected_group: None,
            loading: false,
            status_message: None,
        }
    }
}

/// Draw the video browser panel
pub fn draw(f: &mut Frame, app: &App, area: Rect) {
    let state = match &app.video_browser {
        Some(v) => v,
        None => return,
    };

    let theme = &app.theme;
    let border_style = Style::default().fg(theme.accent);

    // Split into title, list, and status
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Title + filter
            Constraint::Min(5),     // Channel list
            Constraint::Length(2),  // Status
        ])
        .split(area);

    // Title block with filter
    let title_text = if let Some(ref group) = state.selected_group {
        format!(" 📺 Video Channels [{}] ", group)
    } else {
        " 📺 Video Channels ".to_string()
    };

    let filter_text = if state.filter.is_empty() {
        "Type to filter...".to_string()
    } else {
        format!("Filter: {}", state.filter)
    };

    let title_block = Block::default()
        .title(title_text)
        .borders(Borders::ALL)
        .border_style(border_style)
        .style(Style::default().bg(theme.bg_dark));

    let inner = title_block.inner(chunks[0]);
    f.render_widget(title_block, chunks[0]);

    let title_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(inner);

    f.render_widget(
        Paragraph::new(filter_text)
            .style(Style::default().fg(theme.text_muted).add_modifier(Modifier::ITALIC)),
        title_layout[1],
    );

    // Channel list
    let filtered_channels: Vec<&Channel> = if state.filter.is_empty() {
        state.channels.iter().collect()
    } else {
        let filter_lower = state.filter.to_lowercase();
        state.channels
            .iter()
            .filter(|ch| {
                ch.name.to_lowercase().contains(&filter_lower)
                    || ch.group.as_ref().map_or(false, |g| g.to_lowercase().contains(&filter_lower))
            })
            .collect()
    };

    let items: Vec<ListItem> = filtered_channels
        .iter()
        .enumerate()
        .map(|(i, ch)| {
            let style = if i == state.selected_index {
                Style::default().bg(theme.accent).fg(theme.bg_dark).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(theme.text_muted)
            };

            let group_str = ch.group.as_deref().unwrap_or("");
            let logo_indicator = if ch.logo.is_some() { "🖼 " } else { "" };
            let line = format!("{}[{}] {}{}", 
                if i == state.selected_index { "▶ " } else { "  " },
                group_str,
                logo_indicator,
                ch.name
            );
            ListItem::new(line).style(style)
        })
        .collect();

    let list_block = Block::default()
        .borders(Borders::ALL)
        .border_style(border_style)
        .style(Style::default().bg(theme.bg_dark));

    let list = List::new(items)
        .block(list_block)
        .highlight_style(Style::default().bg(theme.accent))
        .highlight_symbol("▶ ");

    f.render_widget(list, chunks[1]);

    // Status line
    let status_text = if state.loading {
        "Loading channels...".to_string()
    } else {
        state.status_message.clone().unwrap_or_else(|| {
            format!("{} channels loaded", filtered_channels.len())
        })
    };

    f.render_widget(
        Paragraph::new(status_text)
            .style(Style::default().fg(theme.positive).add_modifier(Modifier::ITALIC)),
        chunks[2],
    );
}

/// Handle input for video browser
pub fn handle_input(app: &mut App, key: crossterm::event::KeyEvent) -> bool {
    let state = match app.video_browser.as_mut() {
        Some(v) => v,
        None => return false,
    };

    match key.code {
        crossterm::event::KeyCode::Up => {
            if state.selected_index > 0 {
                state.selected_index -= 1;
            }
            true
        }
        crossterm::event::KeyCode::Down => {
            let max = if state.filter.is_empty() {
                state.channels.len()
            } else {
                state.channels.iter().filter(|ch| {
                    let filter_lower = state.filter.to_lowercase();
                    ch.name.to_lowercase().contains(&filter_lower)
                }).count()
            };
            if state.selected_index + 1 < max {
                state.selected_index += 1;
            }
            true
        }
        crossterm::event::KeyCode::Enter => {
            // Play selected channel
            let filtered: Vec<&Channel> = if state.filter.is_empty() {
                state.channels.iter().collect()
            } else {
                let filter_lower = state.filter.to_lowercase();
                state.channels.iter().filter(|ch| {
                    ch.name.to_lowercase().contains(&filter_lower)
                }).collect()
            };

            if let Some(channel) = filtered.get(state.selected_index) {
                // Play the channel URL
                if let Some(ref mut player) = app.video_player {
                    player.play_url(&channel.url, app.volume);
                    app.now_playing_video = Some(VideoItem {
                        id: channel.tvg_id.clone().unwrap_or_else(|| channel.name.clone()),
                        title: channel.name.clone(),
                        url: channel.url.clone(),
                        description: channel.group.clone(),
                        thumbnail: channel.logo.clone(),
                        duration: None,
                        source: crate::video::VideoSource::IPTV,
                    });
                }
            }
            true
        }
        crossterm::event::KeyCode::Backspace => {
            state.filter.pop();
            state.selected_index = 0;
            true
        }
        crossterm::event::KeyCode::Esc => {
            // Toggle back to audio mode
            app.video_mode = false;
            true
        }
        crossterm::event::KeyCode::Char('L') | crossterm::event::KeyCode::Char('l') => {
            // Start loading IPTV playlist from default URL
            state.loading = true;
            state.status_message = Some("Loading IPTV playlist...".to_string());
            app.start_video_playlist_fetch(
                "https://iptv-org.github.io/iptv/index.m3u".to_string()
            );
            true
        }
        crossterm::event::KeyCode::Char(c) => {
            state.filter.push(c);
            state.selected_index = 0;
            true
        }
        _ => false,
    }
}
