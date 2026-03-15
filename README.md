<div align="center">

# 🎵 Audira

**A clean, native music player for Windows — built entirely in Rust.**

Inspired by the design of [Amberol](https://gitlab.gnome.org/World/amberol) (a GNOME/Linux player),
Audira brings the same minimal aesthetic to Windows using a fully native Rust stack.

![Rust](https://img.shields.io/badge/Rust-1.75+-orange?logo=rust&logoColor=white)
![Platform](https://img.shields.io/badge/Platform-Windows-blue?logo=windows&logoColor=white)
![License](https://img.shields.io/badge/License-MIT-green)
![Version](https://img.shields.io/badge/Version-0.1.0-purple)

</div>

---

## ✨ Features

- 🎨 **Dynamic UI theming** — app background tints to match album cover art
- 📊 **Real waveform visualizer** — extracted from audio energy, click anywhere to seek
- 🎵 **Multiple playlists** — create, rename, delete, switch; all saved automatically
- 📖 **Lyrics panel** — embedded lyrics shown inline alongside the player (no timestamps)
- 🖼️ **Album art** — embedded cover art with smooth rounded display
- 🔀 **Playback modes** — Repeat Off / Repeat All / Repeat One, Shuffle
- 🎚️ **Volume control** with icons
- ⌨️ **Keyboard shortcuts** — Space, arrow keys, Ctrl+Q
- 🪟 **Draggable frameless window**
- 💾 **Persistent playlists** saved between sessions

---

## 🛠️ Tech Stack

| Layer | Library |
|---|---|
| UI Framework | [iced 0.13](https://github.com/iced-rs/iced) — GPU-accelerated, Elm-style |
| Audio Playback | [rodio 0.20](https://github.com/RustAudio/rodio) |
| Tag/Metadata Reading | [lofty 0.22](https://github.com/Serial-ATA/lofty-rs) |
| File Dialogs | [rfd 0.15](https://github.com/PolyMeilex/rfd) |
| Image Processing | [image 0.25](https://github.com/image-rs/image) |
| Serialization | serde + serde_json |

---

## 🚀 Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (stable, 1.75+)
- Windows 10/11

### Clone & Build

```bash
git clone https://github.com/YOUR_USERNAME/audira.git
cd audira

# Release build (~5MB single exe, ~10 min first compile)
cargo build --release
```

Executable will be at `target\release\audira.exe`

---

## 🎮 Usage

1. Launch `audira.exe`
2. Click **+** to add songs, or **⋯** → Add Songs / Add Folder
3. Click any track to play
4. Click the waveform to seek
5. Enable **Match Cover Art** in ⋯ menu to tint the UI to match the album

---

## ⌨️ Keyboard Shortcuts

| Key | Action |
|---|---|
| `Space` | Play / Pause |
| `←` | Previous track |
| `→` | Next track |
| `Ctrl+Q` | Quit |

---

## 🎵 Supported Formats

`MP3` · `FLAC` · `OGG Vorbis` · `WAV` · `M4A/AAC` · `WMA` · `Opus`

---

## 📁 Project Structure

```
audira_v1/
├── src/
│   ├── main.rs        # Entry point, window setup
│   ├── ui.rs          # All UI layout, state, messages (iced)
│   ├── audio.rs       # Playback engine (rodio), reliable seeking
│   ├── metadata.rs    # Tag reading, lyrics, cover art (lofty)
│   ├── playlist.rs    # Playlist data model, persistence
│   ├── waveform.rs    # Audio energy extraction for visualizer
│   └── theme.rs       # Color extraction from cover art
├── icons/             # Phosphor SVG icons (MIT licensed)
├── Cargo.toml
└── README.md
```

---

##  Credits

- Design inspired by **[Amberol](https://gitlab.gnome.org/World/amberol)** by Emmanuele Bassi
- Icons from **[Phosphor Icons](https://phosphoricons.com/)** (MIT license)
- Built with **[iced](https://github.com/iced-rs/iced)**

---

## 📄 License

This project is licensed under the [MIT License](LICENSE).
