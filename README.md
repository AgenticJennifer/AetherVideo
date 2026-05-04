# ▰▱▰▱▰▱ **AetherVideo** ▰▱▰▱▰▱

```
 ╔═══════════════════════════════════════╗
 ║  ░▒▓   ▓▒░  ╔═══════╗  ░▒▓   ▓▒░  ║
 ║   ▒▓   ▓▒    ║ PLAY  ║   ▒▓   ▓▒   ║
 ║    ▓  ▓     ╚═══════╝    ▓  ▓    ║
 ║   ▒    ▒                     ▒    ▒   ║
 ║  ░        ░  VIDEO STREAMING   ░        ░  ║
 ╚═══════════════════════════════════════╝
        [Terminal • Rust • Delight]
```

**A terminal-based internet radio + video player with real-time audio visualization, built in Rust.**

> *"Turn your terminal into a multimedia powerhouse — radio streams, IPTV playlists, Jellyfin libraries, YouTube, and local files. All from the comfort of your terminal."*

---

## ✨ Features That Delight

| Feature | Description |
|---------|-------------|
| 📻 **Radio Streaming** | Browse 30,000+ stations via RadioBrowser API with smart genre filtering |
| 📺 **Video Playback** | IPTV playlists (M3U), Jellyfin, YouTube, and local media files |
| 🎨 **Live Visualizer** | 16-band spectrum analyzer with CAVA-inspired gravity fall-off |
| 🌈 **8 Built-in Themes** | CRT, Gruvbox, Nord, Dracula, Monokai, Catppuccin, Hacker, Solarized |
| ⌨️ **Custom Keybindings** | Remap every shortcut from the in-app settings overlay |
| 💾 **Smart Persistence** | Favorites, history, and settings auto-saved to JSON |
| 📊 **Built-in Profiler** | Per-frame timing breakdown for performance tuning |
| ⚡ **Low-Power Mode** | Auto-switch to 5 FPS when visualizer is disabled |
| 🎭 **CRT Animations** | Boot-up and power-off animations for that retro feel |

---

## 🚀 Quick Start

### Prerequisites
```bash
# Install system dependencies (Linux)
sudo apt install mpv pulseaudio-utils pipewire-pulse  # Debian/Ubuntu
sudo pacman -S mpv libpulse pipewire-pulse          # Arch
brew install mpv                                    # macOS
```

### From Source (Rust 1.85+)
```bash
git clone https://github.com/JensProcessingUnit/AetherVideo.git
cd AetherVideo
cargo build --release
./target/release/AetherStream
```

### Run Options
```bash
AetherStream                  # With CRT boot animation
AetherStream --skip-menu     # Skip launch menu
AetherStream --boot-speed=fast  # Faster boot animation
```

---

## ⌨️ Keybindings (Defaults)

Press `?` in-app to see your current bindings (customizable via `S`).

| Key | Action |
|-----|--------|
| `↑` / `↓` or `j` / `k` | Navigate lists |
| `Enter` | Play selected station/video |
| `V` | Toggle video mode |
| `L` | Load video playlist (when in video mode) |
| `s` | Stop playback |
| `+` / `-` | Volume up / down |
| `/` | Search stations |
| `f` | Toggle favorite |
| `i` | Station/video details |
| `n` | Load more stations |
| `Tab` | Cycle panel (Stations / Favorites / History) |
| `[` / `]` | Cycle genre category |
| `g` | Genre picker overlay |
| `t` | Theme picker overlay |
| `v` | Toggle visualizer on/off |
| `?` | Help overlay |
| `S` | Customize keybindings |
| `` ` `` | Performance profiler |
| `q` | Quit |

---

## 🎨 Themes

Press `t` to cycle through 8 built-in themes:

- **CRT** — Neon green phosphor terminal (default)
- **Gruvbox** — Warm retro palette
- **Nord** — Cool arctic blues  
- **Dracula** — Dark purple
- **Monokai** — Classic editor colors
- **Catppuccin** — Pastel dark
- **Hacker** — Green-on-black matrix style
- **Solarized** — Precision colors for readability

---

## 📺 Video Mode

Press `V` to enter video mode, then `L` to load a playlist:

- **IPTV**: Paste an M3U playlist URL
- **Jellyfin**: Configure server details in settings
- **YouTube**: Paste video/livestream URL
- **Local**: Browse and play local media files

---

## 🏗️ Architecture

```
src/
├── main.rs                   Entry point, event loop, frame timing
├── app.rs                    App state, business logic, video support
├── video/                    Video playback modules
│   ├── mod.rs               Video module exports
│   ├── player.rs            Video player abstraction
│   ├── iptv.rs             IPTV/M3U playlist parser
│   ├── jellyfin.rs          Jellyfin API integration
│   ├── youtube.rs           YouTube video streaming
│   └── local.rs            Local file playback
├── audio/
│   ├── player.rs           mpv playback, IPC, stream info
│   ├── pipe.rs             FIFO, PCM reader, FFT analysis
│   └── visualizer.rs       Bar animation (real + simulated)
├── storage/
│   ├── config.rs           User preferences + keybindings
│   ├── favorites.rs        JSON persistence for favorites
│   └── history.rs          JSON persistence for play history
└── ui/
    ├── mod.rs              Layout orchestration
    ├── launcher.rs         CRT boot animation, start menu
    ├── video_browser.rs    Video playlist browser (NEW!)
    ├── themes.rs           Color theme definitions
    └── ...                 (15+ UI modules)
```

---

## 💡 Delight Principles

This project follows **terminal delight** design:

- ✨ **Fast & Responsive** — Animations < 1 second, never blocks
- 🎲 **Surprise & Discovery** — Easter eggs hidden in menus
- 🎯 **Context-Aware** — CBT tools for stress, party mode for fun
- 📈 **Compound Joy** — Rotating taglines, random fortunes
- ♿ **Accessible** — Text-based fallbacks, no flashing animations

---

## 🐛 Troubleshooting

**No audio visualization?**
- Install `pipewire-pulse` or `pulseaudio`
- Ensure `parec` is available (pulseaudio-utils package)

**Video mode not working?**
- Check that `mpv` is installed and in PATH
- For YouTube: ensure `yt-dlp` is installed

**Performance issues?**
- Press `` ` `` to open profiler
- Use `<` / `>` to adjust tick rate
- Disable visualizer with `v` for low-power mode

---

## 📜 License

MIT License — see [LICENSE](LICENSE) for details

---

<div align="center">

**Built with ❤️ and ✨ in Rust**

[Report Bug](https://github.com/JensProcessingUnit/AetherVideo/issues) • 
[Request Feature](https://github.com/JensProcessingUnit/AetherVideo/issues) • 
[Fork on GitHub](https://github.com/JensProcessingUnit/AetherVideo)

</div>
