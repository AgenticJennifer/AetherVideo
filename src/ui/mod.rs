pub mod header;
pub mod helpers;
pub mod genre_picker;
pub mod launcher;
pub mod media_browser;
pub mod now_playing;
pub mod overlays;
pub mod perf_overlay;
pub mod settings;
pub mod shutdown;
pub mod song_log;
pub mod station_list;
pub mod stream_info;
pub mod themes;
pub mod theme_picker;
pub mod visualizer;
pub mod video_browser;

use crate::app::{App, Overlay};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Modifier},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn draw(f: &mut Frame, app: &App) {
    let size = f.size();

    // Fill background with theme color
    f.render_widget(
        Block::default().style(Style::default().bg(app.theme.bg_dark)),
        size,
    );

    // ── Main Layout: header / body ────────────────────────────────
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header bar
            Constraint::Min(10),   // Body
        ])
        .split(size);

    header::draw(f, app, main_chunks[0]);
    draw_body(f, app, main_chunks[1]);

    // ── Overlays ───────────────────────────────────────────────────
    match &app.overlay {
        Overlay::Help => overlays::draw_help(f, app, size),
        Overlay::StationDetail => overlays::draw_detail(f, app, size),
        Overlay::Settings => settings::draw(f, app, size),
        Overlay::GenrePicker => genre_picker::draw(f, app, size),
        Overlay::ThemePicker => theme_picker::draw(f, app, size),
        Overlay::None => {}
    }

    // Perf overlay renders on top of everything (independent of Overlay enum)
    if app.show_perf {
        perf_overlay::draw(f, app, size);
    }
}

fn draw_body(f: &mut Frame, app: &App, area: Rect) {
    // Check if video mode is active and video_browser exists
    if app.video_mode && app.video_browser.is_some() {
        draw_video_body(f, app, area);
        return;
    }

    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(area);

    station_list::draw(f, app, body_chunks[0]);

    // Right side: now playing / (song log + vis column) / media browser
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(11),  // Now playing info + clock
            Constraint::Min(6),     // Song log + (visualizer / stream info)
            Constraint::Length(8),  // Media browser stub
        ])
        .split(body_chunks[1]);

    now_playing::draw(f, app, right_chunks[0]);

    // Middle row: song log on left, visualizer + stream info stacked on right
    if app.visualizer_enabled {
        let middle_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(60), // Song log
                Constraint::Percentage(40), // Visualizer + stream info column
            ])
            .split(right_chunks[1]);

        song_log::draw(f, app, middle_chunks[0]);

        // Right column: visualizer on top, stream info below
        let vis_column = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(50), // Visualizer
                Constraint::Percentage(50), // Stream info
            ])
            .split(middle_chunks[1]);

        visualizer::draw(f, app, vis_column[0]);
        stream_info::draw(f, app, vis_column[1]);
    } else {
        // Visualizer disabled — song log gets full width, stream info below
        let middle_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(70), // Song log (wider)
                Constraint::Percentage(30), // Stream info only
            ])
            .split(right_chunks[1]);

        song_log::draw(f, app, middle_chunks[0]);
        stream_info::draw(f, app, middle_chunks[1]);
    }

    // Bottom: media browser
    media_browser::draw(f, app, right_chunks[2]);
}

fn draw_video_body(f: &mut Frame, app: &App, area: Rect) {
    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(area);

    // Left: video channel browser
    video_browser::draw(f, app, body_chunks[0]);

    // Right: now playing video info
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(11),  // Now playing video info
            Constraint::Min(10),     // Video playlist/channels
            Constraint::Length(8),   // Controls
        ])
        .split(body_chunks[1]);

    draw_video_now_playing(f, app, right_chunks[0]);
    draw_video_info(f, app, right_chunks[1]);
    draw_video_controls(f, app, right_chunks[2]);
}

fn draw_video_now_playing(f: &mut Frame, app: &App, area: Rect) {
    let theme = &app.theme;
    let block = Block::default()
        .title(" 🎬 Now Playing Video ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.accent))
        .style(Style::default().bg(theme.bg_dark));

    let inner = block.inner(area);
    f.render_widget(block, area);

    if let Some(ref video) = app.now_playing_video {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
            ])
            .split(inner);

        f.render_widget(
            Paragraph::new(format!("Title: {}", video.title))
                .style(Style::default().fg(theme.text_muted)),
            chunks[0],
        );

        if let Some(ref desc) = video.description {
            f.render_widget(
                Paragraph::new(format!("Source: {}", desc))
                    .style(Style::default().fg(theme.text_muted)),
                chunks[1],
            );
        }

        if let Some(ref thumb) = video.thumbnail {
            f.render_widget(
                Paragraph::new(format!("Thumb: {}", thumb))
                    .style(Style::default().fg(theme.text_muted)),
                chunks[2],
            );
        }

        let playing = if let Some(ref player) = app.video_player {
            if player.is_playing() { "▶ Playing" } else { "⏸ Stopped" }
        } else { "⏹ Stopped" };
        f.render_widget(
            Paragraph::new(playing)
                .style(Style::default().fg(theme.accent)),
            chunks[3],
        );
    } else {
        f.render_widget(
            Paragraph::new("No video playing")
                .style(Style::default().fg(theme.text_muted)),
            inner,
        );
    }
}

fn draw_video_info(f: &mut Frame, app: &App, area: Rect) {
    let theme = &app.theme;
    let block = Block::default()
        .title(" 📺 Video Info ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.accent))
        .style(Style::default().bg(theme.bg_dark));

    f.render_widget(block, area);
}

fn draw_video_controls(f: &mut Frame, app: &App, area: Rect) {
    let theme = &app.theme;
    let block = Block::default()
        .title(" Controls ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.accent))
        .style(Style::default().bg(theme.bg_dark));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(inner);

    f.render_widget(
        Paragraph::new("V: Switch to Audio | Esc: Back to Audio")
            .style(Style::default().fg(theme.text_muted).add_modifier(Modifier::ITALIC)),
        chunks[0],
    );

    let kitty_status = if crate::video::player::VideoPlayer::is_kitty_available() {
        "✓ Kitty mode available"
    } else {
        "⚠ Install Kitty for embedded video"
    };
    f.render_widget(
        Paragraph::new(kitty_status)
            .style(Style::default().fg(theme.text_warn)),
        chunks[1],
    );
}