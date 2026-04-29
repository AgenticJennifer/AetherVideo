# Contributing to AetherTune

Thanks for your interest in contributing! This guide covers how to set up your development environment, build the project, and submit changes.

## Getting Started

### Prerequisites

**All platforms:**

1. Install Rust via [rustup](https://rustup.rs/):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
   On Windows, download and run `rustup-init.exe` from the same site.

2. Verify your installation:
   ```bash
   rustc --version   # Should be 1.85+
   cargo --version
   ```

**Linux:**
- `mpv` — required for playback
- `parec` (from `pulseaudio-utils` or `pipewire-pulse`) — required for real-time audio visualization

```bash
# Arch
sudo pacman -S mpv pipewire-pulse

# Ubuntu/Debian
sudo apt install mpv pulseaudio-utils
```

**macOS:**
- `mpv` — required for playback

```bash
brew install mpv
```

Audio visualization runs in simulated mode on macOS (real-time capture is not yet implemented).

**Windows:**
- `mpv.exe` — must be in the same directory as the AetherTune binary, or in your `PATH`
- The release builds bundle `mpv.exe` automatically, but for development you'll need to download it from [mpv.io](https://mpv.io/) or [shinchiro's builds](https://github.com/shinchiro/mpv-winbuild-cmake/releases)
- Use [Windows Terminal](https://aka.ms/terminal) (`wt.exe`) for the best experience — the legacy `cmd.exe` console has limited ANSI and keyboard support

Audio visualization runs in simulated mode on Windows.

### Clone and Build

```bash
git clone https://github.com/nevermore23274/AetherTune.git
cd AetherTune
cargo build
```

For a release (optimized) build:
```bash
cargo build --release
```

### Run

```bash
# Debug build
cargo run

# Release build
cargo run --release

# Skip the boot animation
cargo run -- --skip-menu

# Fast boot
cargo run -- --boot-speed=fast
```

### Run Tests

```bash
cargo test
```

The test suite includes FFT accuracy tests (verifying frequency bin detection for known sine waves) and other unit tests. No external services or audio hardware are needed to run them.

## Project Structure

```
src/
├── main.rs           Event loop, key handling
├── app.rs            App state, business logic
├── audio/
│   ├── player.rs     mpv process management, IPC, stream info
│   ├── pipe.rs       FIFO, PCM reader thread, FFT analysis
│   └── visualizer.rs Bar animation (real + simulated)
├── storage/
│   ├── config.rs     Settings + keybindings (hand-rolled JSON)
│   ├── favorites.rs  Favorites persistence
│   └── history.rs    Listening history persistence
└── ui/
    ├── mod.rs         Layout orchestration
    ├── helpers.rs     Color palette, shared widgets
    ├── settings.rs    Keybinding settings overlay
    └── ...            Other UI panels and overlays
```

Key things to know about the codebase:
- **No serde.** JSON serialization/parsing is hand-rolled in `storage/` to keep the dependency tree minimal. If you add a new config field, you'll need to extend the parser manually.
- **Platform gating** uses `#[cfg(unix)]` and `#[cfg(windows)]`. macOS falls under `#[cfg(unix)]`.
- **Audio capture** (`parec`) is a runtime dependency, not a build dependency. The app detects it at startup and falls back to simulated visualization if it's not available.

## Making Changes

1. Fork the repository and create a branch for your change
2. Make your changes and verify they compile on your platform:
   ```bash
   cargo build
   cargo test
   ```
3. If you've changed key handling or UI, run the app and verify it works interactively
4. Commit with a descriptive message explaining what and why
5. Open a pull request against `main`

## Performance

AetherTune has a built-in profiler — press `` ` `` to toggle it. See [docs/PROFILING.md](PROFILING.md) for details on reading the profiler output and tuning performance.

## CI Pipeline

The release CI (`.github/workflows/`) builds for Linux, Windows, and macOS (Intel + Apple Silicon), then publishes to:
- GitHub Releases (all platforms)
- AUR (`aethertune-bin`)
- PPA (`ppa:patchgoblin/aethertune`)
- Homebrew (`nevermore23274/aethertune`)

CI is triggered by pushing a version tag (`v*`). For development, your PR will be reviewed and merged to `main` — you don't need to worry about releases or tags.

## Questions?

Open an issue if you're unsure about anything or want to discuss an approach before writing code.