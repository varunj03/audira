// ui.rs — Audira, iced 0.13

use std::time::Duration;

use iced::{
    Alignment, Color, Element, Font, Length, Subscription, Task, Theme,
    font,
    keyboard::{self, Key, Modifiers},
    mouse,
    widget::{
        button, canvas, column, container, horizontal_space, mouse_area, row,
        scrollable, slider, svg, stack, text, text_input, vertical_space, Space,
        Canvas, Image,
    },
    window,
};
use rand::Rng;

use crate::audio::AudioEngine;
use crate::metadata::{format_secs, format_secs_remaining};
use crate::playlist::{load_playlists, save_playlists, Playlist};
use crate::theme::{extract_accent, fallback_palette, lighten,
                   NEUTRAL_BOTTOM, NEUTRAL_TOP, WAVEFORM_PLAYED, WAVEFORM_UNPLAYED};
use crate::waveform::{extract as waveform_extract, NUM_BARS};

// Inline SVG icons (Phosphor Icons, MIT license — phosphoricons.com)
const ICON_PLAY:    &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256"><path fill="currentColor" d="M240 128a15.74 15.74 0 0 1-7.6 13.51L88.32 229.65a16 16 0 0 1-16.2.3A15.86 15.86 0 0 1 64 216.13V39.87a15.86 15.86 0 0 1 8.12-13.82 16 16 0 0 1 16.2.3l144.08 88.14A15.74 15.74 0 0 1 240 128Z"/></svg>"#;
const ICON_PAUSE:   &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256"><path fill="currentColor" d="M216 48v160a16 16 0 0 1-16 16h-40a16 16 0 0 1-16-16V48a16 16 0 0 1 16-16h40a16 16 0 0 1 16 16ZM96 32H56a16 16 0 0 0-16 16v160a16 16 0 0 0 16 16h40a16 16 0 0 0 16-16V48a16 16 0 0 0-16-16Z"/></svg>"#;
const ICON_PREV:    &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256"><path fill="currentColor" d="M208 47.88v160.24a16 16 0 0 1-24.43 13.43L64 149.83V208a8 8 0 0 1-16 0V48a8 8 0 0 1 16 0v58.17l119.57-71.72A16 16 0 0 1 208 47.88Z"/></svg>"#;
const ICON_NEXT:    &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256"><path fill="currentColor" d="M208 48v160a8 8 0 0 1-16 0v-58.17l-119.57 71.72A16 16 0 0 1 48 208.12V47.88a16 16 0 0 1 24.43-13.43L192 106.17V48a8 8 0 0 1 16 0Z"/></svg>"#;
const ICON_REPEAT:  &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256"><path fill="currentColor" d="M200 176H64v-24a8 8 0 0 0-13.66-5.66l-32 32a8 8 0 0 0 0 11.32l32 32A8 8 0 0 0 64 216v-24h136a16 16 0 0 0 16-16V80a8 8 0 0 0-16 0ZM56 80h136V56a8 8 0 0 1 13.66-5.66l32 32a8 8 0 0 1 0 11.32l-32 32A8 8 0 0 1 192 120V96H56a16 16 0 0 1-16-16 8 8 0 0 1 16 0Z"/></svg>"#;
const ICON_REPEAT1: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256"><path fill="currentColor" d="M200 176H64v-24a8 8 0 0 0-13.66-5.66l-32 32a8 8 0 0 0 0 11.32l32 32A8 8 0 0 0 64 216v-24h136a16 16 0 0 0 16-16V80a8 8 0 0 0-16 0ZM120 168v-48a8 8 0 0 0-11.58-7.16l-16 8a8 8 0 0 0 7.16 14.32L104 133v35a8 8 0 0 0 16 0ZM56 80h136V56a8 8 0 0 1 13.66-5.66l32 32a8 8 0 0 1 0 11.32l-32 32A8 8 0 0 1 192 120V96H56a16 16 0 0 1-16-16 8 8 0 0 1 16 0Z"/></svg>"#;
const ICON_SHUFFLE: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256"><path fill="currentColor" d="m237.66 178.34-32 32a8 8 0 0 1-11.32-11.32L210.69 183H192c-25.85 0-43.82-16.18-60.42-31.07C117.07 138.78 103.35 127 88 127c-13.33 0-25.61 7.29-36 21.09L33.33 164H48a8 8 0 0 1 0 16H16a8 8 0 0 1-8-8v-32a8 8 0 0 1 16 0v11.45C35.28 138.59 51.41 111 88 111c25.85 0 43.82 16.18 60.42 31.07C163.93 155.22 177.65 167 192 167h18.69l-16.35-16.34a8 8 0 0 1 11.32-11.32l32 32a8 8 0 0 1 0 11.66ZM48 89H16a8 8 0 0 1 0-16h32a8 8 0 0 1 0 16Zm162.69-16H192c-14.35 0-28.07 11.78-43.58 24.93C131.82 112.82 113.85 129 88 129H48a8 8 0 0 1 0-16h40c15.35 0 29.07-11.78 44.58-24.93C149.18 75.18 167.15 57 192 57h18.69l-16.35-16.34a8 8 0 0 1 11.32-11.32l32 32a8 8 0 0 1-11.32 11.32Z"/></svg>"#;
const ICON_LIST:    &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256"><path fill="currentColor" d="M224 128a8 8 0 0 1-8 8H40a8 8 0 0 1 0-16h176a8 8 0 0 1 8 8ZM40 72h176a8 8 0 0 0 0-16H40a8 8 0 0 0 0 16Zm176 112H40a8 8 0 0 0 0 16h176a8 8 0 0 0 0-16Z"/></svg>"#;
const ICON_MENU:    &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256"><path fill="currentColor" d="M224 128a8 8 0 0 1-8 8H40a8 8 0 0 1 0-16h176a8 8 0 0 1 8 8ZM40 72h176a8 8 0 0 0 0-16H40a8 8 0 0 0 0 16Zm176 112H40a8 8 0 0 0 0 16h176a8 8 0 0 0 0-16Z"/></svg>"#;
// Sidebar/panel layout icon — distinct from 3-line hamburger
const ICON_SIDEBAR: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256"><rect x="24" y="40" width="208" height="176" rx="16" fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="20"/><line x1="88" y1="40" x2="88" y2="216" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="20"/></svg>"#;
const ICON_CHECK:   &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256"><path fill="currentColor" d="M229.66 77.66l-128 128a8 8 0 0 1-11.32 0l-56-56a8 8 0 0 1 11.32-11.32L96 188.69 218.34 66.34a8 8 0 0 1 11.32 11.32Z"/></svg>"#;
const ICON_PLUS:    &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256"><path fill="currentColor" d="M224 128a8 8 0 0 1-8 8h-80v80a8 8 0 0 1-16 0v-80H40a8 8 0 0 1 0-16h80V40a8 8 0 0 1 16 0v80h80a8 8 0 0 1 8 8Z"/></svg>"#;
const ICON_CLOSE:   &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256"><path fill="currentColor" d="M205.66 194.34a8 8 0 0 1-11.32 11.32L128 139.31l-66.34 66.35a8 8 0 0 1-11.32-11.32L116.69 128 50.34 61.66a8 8 0 0 1 11.32-11.32L128 116.69l66.34-66.35a8 8 0 0 1 11.32 11.32L139.31 128Z"/></svg>"#;
const ICON_DOTS:    &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256"><path fill="currentColor" d="M112 60a16 16 0 1 1 16 16 16 16 0 0 1-16-16Zm16 52a16 16 0 1 0 16 16 16 16 0 0 0-16-16Zm0 68a16 16 0 1 0 16 16 16 16 0 0 0-16-16Z"/></svg>"#;
const ICON_NOTE:    &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256"><path fill="currentColor" d="M212.92 10.1a8 8 0 0 0-6.86-.1l-128 48A8 8 0 0 0 72 65.82v109.76a36 36 0 1 0 16 29.92V111.43l112-42v82.15a36 36 0 1 0 16 29.92V16a8 8 0 0 0-3.08-5.9Z"/></svg>"#;
const ICON_VOLLOW:  &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256"><path fill="currentColor" d="M155.51 24.81a8 8 0 0 0-8.42.88L77.25 80H32a16 16 0 0 0-16 16v64a16 16 0 0 0 16 16h45.25l69.84 54.31A8 8 0 0 0 160 224V32a8 8 0 0 0-4.49-7.19ZM32 96h40v64H32Zm112 111.64-48-37.33V85.69l48-37.33Zm54-61.64a40 40 0 0 0 0-36 8 8 0 0 0-14.24 7.28 24 24 0 0 1 0 21.44 8 8 0 1 0 14.24 7.28Z"/></svg>"#;
const ICON_VOLHI:   &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256"><path fill="currentColor" d="M155.51 24.81a8 8 0 0 0-8.42.88L77.25 80H32a16 16 0 0 0-16 16v64a16 16 0 0 0 16 16h45.25l69.84 54.31A8 8 0 0 0 160 224V32a8 8 0 0 0-4.49-7.19ZM32 96h40v64H32Zm112 111.64-48-37.33V85.69l48-37.33Zm54-61.64a40 40 0 0 0 0-36 8 8 0 0 0-14.24 7.28 24 24 0 0 1 0 21.44 8 8 0 1 0 14.24 7.28Zm22.56 18.1A72 72 0 0 0 224 128a72 72 0 0 0-7.38-32.2 8 8 0 0 0-14.44 6.9A56 56 0 0 1 208 128a56 56 0 0 1-5.84 25.3 8 8 0 1 0 14.46 6.87Z"/></svg>"#;

fn icon_el(data: &'static [u8], size: f32, color: Color) -> Element<'static, Message> {
    svg(svg::Handle::from_memory(data))
        .width(size).height(size)
        .style(move |_, _| svg::Style { color: Some(color) })
        .into()
}

// Style helpers
fn cstyle(bg: Color) -> container::Style {
    container::Style { background: Some(iced::Background::Color(bg)), border: iced::Border::default(), shadow: iced::Shadow::default(), text_color: None }
}
fn cstyle_r(bg: Color, r: f32) -> container::Style {
    container::Style { background: Some(iced::Background::Color(bg)), border: iced::Border { radius: r.into(), ..Default::default() }, shadow: iced::Shadow::default(), text_color: None }
}
fn p2(v: u16, h: u16) -> iced::Padding {
    iced::Padding { top: v as f32, bottom: v as f32, left: h as f32, right: h as f32 }
}
fn p4(top: f32, right: f32, bottom: f32, left: f32) -> iced::Padding {
    iced::Padding { top, right, bottom, left }
}

const C_DIM: Color = Color { r: 0.55, g: 0.55, b: 0.55, a: 1.0 };
const C_ACT: Color = Color { r: 0.18, g: 0.18, b: 0.18, a: 1.0 };
const C_WHT: Color = Color { r: 1.0,  g: 1.0,  b: 1.0,  a: 1.0 };

// ─── Message ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum Message {
    PlayPause, Prev, Next,
    PlayIndex { playlist: usize, track: usize },
    SeekFrac(f64),
    VolumeChanged(f64),
    Tick,
    AddFiles, AddFolder, ClearPlaylist,
    NewPlaylist, DeletePlaylist, RenamePlaylist,
    SelectPlaylist(usize),
    TogglePlaylistPanel,
    CycleRepeat, ToggleShuffle, ToggleMatchCoverArt,
    ShowLyrics, HideLyrics,
    ShowMenu, HideMenu,
    ShowPlaylistMenu, HidePlaylistMenu,
    InputChanged(String), InputConfirm, InputCancel,
    KeyPressed(Key, Modifiers),
    CloseWindow,
    DragWindow,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RepeatMode { None, All, One }

#[derive(Debug, Clone)]
enum InputPurpose { NewPlaylist, RenamePlaylist }

// ─── State ───────────────────────────────────────────────────────────────────

#[allow(dead_code)]
pub struct Audira {
    playlists:          Vec<Playlist>,
    active_playlist:    usize,
    current_track:      Option<usize>,
    engine:             Option<AudioEngine>,
    is_playing:         bool,
    position:           f64,
    total:              f64,
    volume:             f64,
    repeat:             RepeatMode,
    shuffle:            bool,
    match_cover_art:    bool,
    waveform_bars:      [f32; NUM_BARS],
    cover_art:          Option<iced::widget::image::Handle>,
    accent_top:         Color,
    accent_bottom:      Color,
    waveform_played:    Color,
    playlist_visible:   bool,
    show_lyrics:        bool,
    show_menu:          bool,
    show_playlist_menu: bool,
    input_active:       bool,
    input_purpose:      Option<InputPurpose>,
    input_value:        String,
}

impl Audira {
    pub fn new() -> (Self, Task<Message>) {
        let app = Audira {
            playlists: load_playlists(),
            active_playlist: 0, current_track: None,
            engine: AudioEngine::new(),
            is_playing: false, position: 0.0, total: 0.0, volume: 0.8,
            repeat: RepeatMode::None, shuffle: false, match_cover_art: true,
            waveform_bars: [0.3; NUM_BARS],
            cover_art: None,
            accent_top: NEUTRAL_TOP, accent_bottom: NEUTRAL_BOTTOM, waveform_played: WAVEFORM_PLAYED,
            playlist_visible: true, show_lyrics: false, show_menu: false, show_playlist_menu: false,
            input_active: false, input_purpose: None, input_value: String::new(),
        };
        (app, Task::none())
    }

    pub fn theme(&self) -> Theme { Theme::Light }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([
            iced::time::every(Duration::from_millis(200)).map(|_| Message::Tick),
            iced::event::listen_with(|event, _status, _id| {
                if let iced::Event::Keyboard(iced::keyboard::Event::KeyPressed { key, modifiers, .. }) = event {
                    return Some(Message::KeyPressed(key, modifiers));
                }
                None
            }),
        ])
    }

    fn active_tracks_len(&self) -> usize {
        self.playlists.get(self.active_playlist).map(|p| p.tracks.len()).unwrap_or(0)
    }

    fn reset_theme(&mut self) {
        self.accent_top = NEUTRAL_TOP;
        self.accent_bottom = NEUTRAL_BOTTOM;
        self.waveform_played = WAVEFORM_PLAYED;
    }

    fn apply_theme_from_cover(&mut self, cover_bytes: &[u8], track_idx: usize) {
        if !self.match_cover_art { self.reset_theme(); return; }
        if let Some(accent) = extract_accent(cover_bytes) {
            self.accent_top = lighten(accent, 1.55);
            self.accent_bottom = accent;
            self.waveform_played = accent;
        } else {
            // No accent found — use fallback palette
            let (c1, c2) = fallback_palette(track_idx);
            self.accent_top = c1;
            self.accent_bottom = c2;
            self.waveform_played = Color::from_rgb(
                (c1.r + c2.r) / 2.0 * 0.65,
                (c1.g + c2.g) / 2.0 * 0.65,
                (c1.b + c2.b) / 2.0 * 0.65,
            );
        }
    }

    fn play_index(&mut self, playlist: usize, track: usize) {
        let Some(pl) = self.playlists.get(playlist) else { return };
        let Some(t)  = pl.tracks.get(track)         else { return };
        let path     = t.path.clone();
        let duration = t.meta.duration;
        let cover    = t.meta.cover_art.clone(); // clone before mut borrow

        self.active_playlist = playlist;
        self.current_track   = Some(track);
        self.total           = duration;
        self.position        = 0.0;
        self.waveform_bars   = waveform_extract(&path);

        if let Some(ref bytes) = cover {
            self.cover_art = Some(iced::widget::image::Handle::from_bytes(bytes.clone()));
            self.apply_theme_from_cover(bytes, track);
        } else {
            self.cover_art = None;
            if self.match_cover_art {
                self.apply_theme_from_cover(&[], track); // will use fallback
            } else {
                self.reset_theme();
            }
        }

        if let Some(eng) = &mut self.engine {
            eng.play_file(&path, duration);
            eng.set_volume(self.volume as f32);
        }
        self.is_playing = true;
    }

    fn play_next(&mut self) {
        let len = self.active_tracks_len();
        if len == 0 { return; }
        let next = match self.repeat {
            RepeatMode::One => self.current_track.unwrap_or(0),
            _ => {
                let cur = self.current_track.unwrap_or(0);
                if self.shuffle { rand::thread_rng().gen_range(0..len) }
                else {
                    let n = cur + 1;
                    if n >= len {
                        if self.repeat == RepeatMode::All { 0 } else { return; }
                    } else { n }
                }
            }
        };
        let pl = self.active_playlist;
        self.play_index(pl, next);
    }

    fn play_prev(&mut self) {
        let len = self.active_tracks_len();
        if len == 0 { return; }
        if self.position > 3.0 {
            if let Some(eng) = &mut self.engine { eng.seek(0.0); }
            self.position = 0.0;
        } else {
            let cur  = self.current_track.unwrap_or(0);
            let prev = if cur == 0 { len - 1 } else { cur - 1 };
            let pl   = self.active_playlist;
            self.play_index(pl, prev);
        }
    }

    fn remaining_text(&self) -> String {
        if let Some(idx) = self.current_track {
            if let Some(pl) = self.playlists.get(self.active_playlist) {
                let rem = pl.remaining_duration(idx, self.position);
                let m   = rem as u64 / 60;
                return if m < 60 { format!("{m} min remaining") }
                       else       { format!("{}h {}m remaining", m / 60, m % 60) };
            }
        }
        String::new()
    }

    // ─── Update ──────────────────────────────────────────────────────────────

    pub fn update(&mut self, msg: Message) -> Task<Message> {
        match msg {
            Message::PlayPause => {
                if self.current_track.is_none() && self.active_tracks_len() > 0 {
                    let pl = self.active_playlist; self.play_index(pl, 0);
                } else if let Some(eng) = &mut self.engine {
                    if self.is_playing { eng.pause(); self.is_playing = false; }
                    else               { eng.resume(); self.is_playing = true; }
                }
            }
            Message::Prev => self.play_prev(),
            Message::Next => self.play_next(),
            Message::PlayIndex { playlist, track } => self.play_index(playlist, track),

            Message::SeekFrac(frac) => {
                if self.total > 0.0 {
                    let secs = (frac * self.total).max(0.0);
                    if let Some(eng) = &mut self.engine { eng.seek(secs); }
                    self.position = secs;
                }
            }

            Message::VolumeChanged(v) => {
                self.volume = v;
                if let Some(eng) = &mut self.engine { eng.set_volume(v as f32); }
            }

            Message::Tick => {
                if self.is_playing {
                    if let Some(eng) = &mut self.engine {
                        let (pos, fin) = eng.tick();
                        self.position = pos;
                        if fin { self.play_next(); }
                    }
                }
            }

            Message::AddFiles => {
                let paths = rfd::FileDialog::new()
                    .add_filter("Audio", &["mp3","flac","ogg","wav","m4a","aac","wma","opus"])
                    .pick_files().unwrap_or_default();
                if !paths.is_empty() {
                    let was_empty = self.playlists.get(self.active_playlist).map(|p| p.tracks.is_empty()).unwrap_or(true);
                    let start = self.playlists.get(self.active_playlist).map(|p| p.tracks.len()).unwrap_or(0);
                    if let Some(pl) = self.playlists.get_mut(self.active_playlist) {
                        for p in paths { pl.add_file(p); }
                    }
                    if was_empty { let ap = self.active_playlist; self.play_index(ap, start); }
                    let _ = save_playlists(&self.playlists);
                }
            }

            Message::AddFolder => {
                if let Some(dir) = rfd::FileDialog::new().pick_folder() {
                    let was_empty = self.playlists.get(self.active_playlist).map(|p| p.tracks.is_empty()).unwrap_or(true);
                    let start = self.playlists.get(self.active_playlist).map(|p| p.tracks.len()).unwrap_or(0);
                    if let Some(pl) = self.playlists.get_mut(self.active_playlist) { pl.add_dir(&dir); }
                    if was_empty { let ap = self.active_playlist; self.play_index(ap, start); }
                    let _ = save_playlists(&self.playlists);
                }
            }

            Message::ClearPlaylist => {
                if let Some(eng) = &mut self.engine { eng.stop(); }
                self.is_playing = false; self.current_track = None;
                self.position = 0.0; self.total = 0.0;
                if let Some(pl) = self.playlists.get_mut(self.active_playlist) { pl.tracks.clear(); }
                self.cover_art = None; self.reset_theme();
                self.waveform_bars = [0.3; NUM_BARS];
                let _ = save_playlists(&self.playlists);
            }

            Message::NewPlaylist => {
                self.input_purpose = Some(InputPurpose::NewPlaylist);
                self.input_value = format!("Playlist {}", self.playlists.len() + 1);
                self.input_active = true; self.show_playlist_menu = false;
            }
            Message::RenamePlaylist => {
                self.input_purpose = Some(InputPurpose::RenamePlaylist);
                self.input_value = self.playlists.get(self.active_playlist).map(|p| p.name.clone()).unwrap_or_default();
                self.input_active = true; self.show_playlist_menu = false;
            }
            Message::DeletePlaylist => {
                if self.playlists.len() > 1 { self.playlists.remove(self.active_playlist); }
                else if let Some(pl) = self.playlists.get_mut(0) { pl.tracks.clear(); }
                self.active_playlist = self.active_playlist.saturating_sub(1);
                self.current_track = None;
                if let Some(eng) = &mut self.engine { eng.stop(); }
                self.is_playing = false; self.show_playlist_menu = false;
                let _ = save_playlists(&self.playlists);
            }
            Message::SelectPlaylist(i) => {
                self.active_playlist = i; self.current_track = None;
                self.show_playlist_menu = false;
                if let Some(eng) = &mut self.engine { eng.stop(); }
                self.is_playing = false;
            }

            Message::InputChanged(s) => self.input_value = s,
            Message::InputConfirm => {
                let val = self.input_value.trim().to_string();
                if !val.is_empty() {
                    match &self.input_purpose {
                        Some(InputPurpose::NewPlaylist) => {
                            self.playlists.push(Playlist::new(&val));
                            self.active_playlist = self.playlists.len() - 1;
                        }
                        Some(InputPurpose::RenamePlaylist) => {
                            if let Some(pl) = self.playlists.get_mut(self.active_playlist) { pl.name = val; }
                        }
                        None => {}
                    }
                    let _ = save_playlists(&self.playlists);
                }
                self.input_active = false; self.input_purpose = None;
            }
            Message::InputCancel => { self.input_active = false; self.input_purpose = None; }

            Message::TogglePlaylistPanel => {
                self.playlist_visible = !self.playlist_visible;
                self.show_menu = false; // close settings menu if open
            }
            Message::CycleRepeat => {
                self.repeat = match self.repeat {
                    RepeatMode::None => RepeatMode::All,
                    RepeatMode::All  => RepeatMode::One,
                    RepeatMode::One  => RepeatMode::None,
                };
            }
            Message::ToggleShuffle => self.shuffle = !self.shuffle,

            Message::ToggleMatchCoverArt => {
                self.match_cover_art = !self.match_cover_art;
                // Clone bytes before mutable ops
                let cover = self.current_track
                    .and_then(|i| self.playlists.get(self.active_playlist)?.tracks.get(i))
                    .and_then(|t| t.meta.cover_art.clone());
                let idx = self.current_track.unwrap_or(0);
                if let Some(bytes) = cover {
                    self.apply_theme_from_cover(&bytes, idx);
                } else {
                    if self.match_cover_art { self.apply_theme_from_cover(&[], idx); }
                    else { self.reset_theme(); }
                }
            }

            Message::ShowLyrics => self.show_lyrics = true,
            Message::HideLyrics => self.show_lyrics = false,
            Message::ShowMenu   => self.show_menu   = true,
            Message::HideMenu   => self.show_menu   = false,
            Message::ShowPlaylistMenu => self.show_playlist_menu = true,
            Message::HidePlaylistMenu => self.show_playlist_menu = false,

            Message::CloseWindow => {
                let _ = save_playlists(&self.playlists);
                return window::get_oldest().and_then(window::close);
            }
            Message::DragWindow => {
                return window::get_oldest().and_then(window::drag);
            }

            Message::KeyPressed(key, mods) => match &key {
                // Space comes as Named::Space — match both forms to be safe
                Key::Named(keyboard::key::Named::Space) => { return self.update(Message::PlayPause); }
                Key::Character(c) if c.as_str() == " " => { return self.update(Message::PlayPause); }
                Key::Named(keyboard::key::Named::ArrowLeft)  if mods.is_empty() => { return self.update(Message::Prev); }
                Key::Named(keyboard::key::Named::ArrowRight) if mods.is_empty() => { return self.update(Message::Next); }
                Key::Character(c) if c.as_str() == "q" && mods.command() => { return self.update(Message::CloseWindow); }
                _ => {}
            },
        }
        Task::none()
    }

    // ─── View ────────────────────────────────────────────────────────────────

    pub fn view(&self) -> Element<Message> {
        if self.input_active { return self.view_input_dialog(); }

        // Compute theme colors first
        let bg_color = if self.match_cover_art {
            let a = self.accent_top;
            Color::from_rgb(
                (a.r * 0.18 + 0.94 * 0.82).min(1.0),
                (a.g * 0.18 + 0.934 * 0.82).min(1.0),
                (a.b * 0.18 + 0.93 * 0.82).min(1.0),
            )
        } else { Color::from_rgb(0.94, 0.934, 0.93) };
        let left_panel_bg = if self.match_cover_art {
            let a = self.accent_top;
            Color::from_rgb(
                (a.r * 0.12 + 0.925 * 0.88).min(1.0),
                (a.g * 0.12 + 0.914 * 0.88).min(1.0),
                (a.b * 0.12 + 0.907 * 0.88).min(1.0),
            )
        } else { Color::from_rgb(0.925, 0.914, 0.907) };

        let left: Element<Message> = if self.playlist_visible {
            self.view_left_panel_colored(left_panel_bg)
        } else {
            Space::new(0, Length::Fill).into()
        };

        let bg_color = if self.match_cover_art {
            // Tint: mix accent_top lightly with neutral base
            let a = self.accent_top;
            Color::from_rgb(
                (a.r * 0.18 + 0.94 * 0.82).min(1.0),
                (a.g * 0.18 + 0.934 * 0.82).min(1.0),
                (a.b * 0.18 + 0.93 * 0.82).min(1.0),
            )
        } else {
            Color::from_rgb(0.94, 0.934, 0.93)
        };
        let left_panel_bg = if self.match_cover_art {
            let a = self.accent_top;
            Color::from_rgb(
                (a.r * 0.12 + 0.925 * 0.88).min(1.0),
                (a.g * 0.12 + 0.914 * 0.88).min(1.0),
                (a.b * 0.12 + 0.907 * 0.88).min(1.0),
            )
        } else {
            Color::from_rgb(0.925, 0.914, 0.907)
        };
        let root = container(row![left, self.view_right_panel_colored(bg_color, left_panel_bg)])
            .width(Length::Fill).height(Length::Fill)
            .style(move |_| container::Style {
                background: Some(iced::Background::Color(bg_color)),
                border: iced::Border {
                    radius: iced::border::Radius { top_left: 0.0, top_right: 0.0, bottom_left: 14.0, bottom_right: 14.0 },
                    ..Default::default()
                },
                shadow: iced::Shadow::default(), text_color: None,
            });

        if self.show_playlist_menu {
            return self.view_playlist_menu(root.into());
        }
        root.into()
    }

    // ─── Left panel ──────────────────────────────────────────────────────────

    fn view_left_panel_colored(&self, left_bg: Color) -> Element<Message> {
        let pl        = self.playlists.get(self.active_playlist);
        let pl_name   = pl.map(|p| p.name.as_str()).unwrap_or("Playlist");
        let remaining = self.remaining_text();

        let header = row![
            button(icon_el(ICON_SIDEBAR, 16.0, C_ACT))
                .on_press(Message::ShowPlaylistMenu)
                .style(ghost_btn).width(32).height(32).padding(p2(7, 7)),
            horizontal_space(),
            column![
                text(pl_name).size(13).font(Font { weight: font::Weight::Bold, ..Font::DEFAULT }),
                text(remaining).size(10)
                    .style(|_| text::Style { color: Some(Color::from_rgb(0.55,0.55,0.55)) }),
            ].align_x(Alignment::Center).spacing(1),
            horizontal_space(),
            button(icon_el(ICON_PLUS, 16.0, C_ACT))
                .on_press(Message::AddFiles)
                .style(ghost_btn).width(32).height(32).padding(p2(7, 7)),
        ]
        .align_y(Alignment::Center).padding(p2(0, 8)).height(48);

        let sep = container(Space::new(Length::Fill, 1))
            .style(|_| cstyle(Color::from_rgb(0.87,0.87,0.87))).width(Length::Fill);

        let track_list: Element<Message> = if pl.map(|p| p.tracks.is_empty()).unwrap_or(true) {
            container(column![
                container(icon_el(ICON_NOTE, 36.0, Color::from_rgb(0.76,0.76,0.76)))
                    .center_x(50).center_y(50),
                text("No music yet").size(12)
                    .style(|_| text::Style { color: Some(Color::from_rgb(0.70,0.70,0.70)) }),
                text("Tap + to add files").size(10)
                    .style(|_| text::Style { color: Some(Color::from_rgb(0.74,0.74,0.74)) }),
            ].spacing(6).align_x(Alignment::Center))
            .center_x(Length::Fill).center_y(Length::Fill)
            .into()
        } else {
            let items: Vec<Element<Message>> = pl.unwrap().tracks.iter().enumerate().map(|(i, t)| {
                let active  = self.current_track == Some(i);
                let row_bg  = if active { Color::WHITE } else { Color::TRANSPARENT };
                let title_c = if active { Color::BLACK } else { Color::from_rgb(0.15,0.15,0.15) };
                let pl_i    = self.active_playlist;
                let dur_str = t.meta.format_duration();

                let thumb: Element<Message> = if let Some(art) = &t.meta.cover_art {
                    container(Image::new(iced::widget::image::Handle::from_bytes(art.clone())).width(34).height(34))
                        .width(34).height(34).clip(true)
                        .style(|_| cstyle_r(Color::TRANSPARENT, 6.0)).into()
                } else {
                    container(icon_el(ICON_NOTE, 16.0, Color::from_rgb(0.70,0.70,0.70)))
                        .width(34).height(34).center_x(34).center_y(34)
                        .style(|_| cstyle_r(Color::from_rgb(0.86,0.86,0.86), 6.0)).into()
                };

                let badge: Element<Message> = if active {
                    icon_el(ICON_VOLHI, 12.0, Color::from_rgb(0.48,0.48,0.48))
                } else {
                    text(dur_str).size(9)
                        .style(|_| text::Style { color: Some(Color::from_rgb(0.66,0.66,0.66)) }).into()
                };

                button(row![
                    thumb,
                    column![
                        text(&t.meta.title).size(12)
                            .font(Font { weight: font::Weight::Semibold, ..Font::DEFAULT })
                            .style(move |_| text::Style { color: Some(title_c) }),
                        text(&t.meta.artist).size(10)
                            .style(|_| text::Style { color: Some(Color::from_rgb(0.50,0.50,0.50)) }),
                    ].spacing(2).width(Length::Fill),
                    badge,
                ].spacing(8).align_y(Alignment::Center))
                .on_press(Message::PlayIndex { playlist: pl_i, track: i })
                .style(move |_, _| button::Style {
                    background: Some(iced::Background::Color(row_bg)),
                    border: iced::Border { radius: 8.0.into(), ..Default::default() },
                    text_color: title_c, shadow: iced::Shadow::default(),
                })
                .padding(p2(5, 8)).width(Length::Fill).into()
            }).collect();

            scrollable(column(items).spacing(2).padding(p2(4, 4))).height(Length::Fill).into()
        };

        container(column![header, sep, track_list])
            .width(245).height(Length::Fill)
            .style(move |_| container::Style {
                background: Some(iced::Background::Color(left_bg)),
                border: iced::Border {
                    radius: iced::border::Radius { top_left: 0.0, bottom_left: 14.0, top_right: 0.0, bottom_right: 0.0 },
                    ..Default::default()
                },
                shadow: iced::Shadow::default(), text_color: None,
            }).into()
    }

    // ─── Right panel ─────────────────────────────────────────────────────────

    fn view_right_panel_colored(&self, _bg_color: Color, _left_panel_bg: Color) -> Element<Message> {
        let track  = self.current_track.and_then(|i| self.playlists.get(self.active_playlist)?.tracks.get(i));
        let title  = track.map(|t| t.meta.title.as_str()) .unwrap_or("Audira");
        let artist = track.map(|t| t.meta.artist.as_str()).unwrap_or("Add music to get started");
        let album  = track.map(|t| t.meta.album.as_str()) .unwrap_or("");
        let sr     = track.and_then(|t| t.meta.sample_rate);
        let lyrics = track.and_then(|t| t.meta.lyrics.clone());

        let titlebar_inner = row![
            horizontal_space(),
            button(icon_el(ICON_CLOSE, 13.0, C_DIM))
                .on_press(Message::CloseWindow)
                .style(ghost_btn).padding(p2(5, 8)),
        ].height(36).align_y(Alignment::Center).padding(p2(0, 6));
        let titlebar: Element<Message> = mouse_area(titlebar_inner)
            .on_press(Message::DragWindow)
            .interaction(mouse::Interaction::Grab)
            .into();

        // Album art
        let accent_top = self.accent_top;
        let art: Element<Message> = if let Some(handle) = &self.cover_art {
            container(Image::new(handle.clone()).width(200).height(200).content_fit(iced::ContentFit::Cover))
                .clip(true).width(200).height(200)
                .style(|_| container::Style {
                    background: None,
                    border: iced::Border { radius: 12.0.into(), ..Default::default() },
                    shadow: iced::Shadow { color: Color { a: 0.22, ..Color::BLACK }, offset: iced::Vector::new(0.0, 5.0), blur_radius: 18.0 },
                    text_color: None,
                }).into()
        } else {
            container(icon_el(ICON_NOTE, 64.0, Color::from_rgb(0.70,0.68,0.66)))
                .center_x(200).center_y(200)
                .style(move |_| container::Style {
                    background: Some(iced::Background::Color(accent_top)),
                    border: iced::Border { radius: 12.0.into(), ..Default::default() },
                    shadow: iced::Shadow { color: Color { a: 0.20, ..Color::BLACK }, offset: iced::Vector::new(0.0, 5.0), blur_radius: 18.0 },
                    text_color: None,
                }).into()
        };

        // Waveform
        let progress = if self.total > 0.0 { (self.position / self.total) as f32 } else { 0.0 };
        let wf: Element<Message> = Canvas::new(WaveformProg {
            bars: &self.waveform_bars, progress,
            played: self.waveform_played, unplayed: WAVEFORM_UNPLAYED,
        }).width(300).height(48).into();

        let time_row = row![
            text(format_secs(self.position)).size(10).font(Font::MONOSPACE)
                .style(|_| text::Style { color: Some(Color::from_rgb(0.58,0.58,0.58)) }),
            horizontal_space(),
            text(format_secs_remaining(self.total, self.position)).size(10).font(Font::MONOSPACE)
                .style(|_| text::Style { color: Some(Color::from_rgb(0.58,0.58,0.58)) }),
        ].width(300);

        let sr_badge: Element<Message> = if let Some(hz) = sr {
            let label = if hz >= 1000 { format!("{:.1} kHz", hz as f32 / 1000.0) } else { format!("{hz} Hz") };
            container(text(label).size(10).font(Font::MONOSPACE)
                .style(|_| text::Style { color: Some(Color::from_rgb(0.50,0.50,0.50)) }))
                .padding(p2(3, 9)).style(|_| cstyle_r(Color::from_rgb(0.895,0.883,0.876), 8.0)).into()
        } else { Space::new(0, 0).into() };

        let lyrics_btn: Element<Message> = if lyrics.is_some() {
            button(text("Lyrics").size(11)).on_press(Message::ShowLyrics)
                .style(ghost_btn).padding(p2(3, 10)).into()
        } else { Space::new(0, 0).into() };

        // Controls
        let play_ic  = if self.is_playing { ICON_PAUSE } else { ICON_PLAY };
        let (rep_ic, rep_c) = match self.repeat {
            RepeatMode::None => (ICON_REPEAT,  C_DIM),
            RepeatMode::All  => (ICON_REPEAT,  C_ACT),
            RepeatMode::One  => (ICON_REPEAT1, C_ACT),
        };
        let shuf_c = if self.shuffle { C_ACT } else { C_DIM };
        let plist_c = if self.playlist_visible { C_ACT } else { C_DIM };

        let controls = row![
            button(icon_el(rep_ic, 18.0, rep_c)) .on_press(Message::CycleRepeat)    .style(ghost_btn).width(40).height(40).padding(p2(10, 10)),
            button(icon_el(ICON_PREV, 20.0, C_ACT)).on_press(Message::Prev)          .style(round_btn).width(44).height(44).padding(p2(11, 11)),
            button(icon_el(play_ic,  24.0, C_WHT)).on_press(Message::PlayPause)      .style(play_btn) .width(54).height(54).padding(p2(14, 14)),
            button(icon_el(ICON_NEXT, 20.0, C_ACT)).on_press(Message::Next)          .style(round_btn).width(44).height(44).padding(p2(11, 11)),
            button(icon_el(ICON_SHUFFLE, 18.0, shuf_c)).on_press(Message::ToggleShuffle).style(ghost_btn).width(40).height(40).padding(p2(10, 10)),
        ].spacing(6).align_y(Alignment::Center);

        let vol_slider: Element<Message> = mouse_area(
            slider(0.0..=1.0, self.volume, Message::VolumeChanged).step(0.01).width(Length::Fill)
        )
        .interaction(mouse::Interaction::Pointer)
        .into();
        let vol = row![
            container(icon_el(ICON_VOLLOW, 16.0, Color::from_rgb(0.58,0.58,0.58))).center_x(20).center_y(20),
            vol_slider,
            container(icon_el(ICON_VOLHI,  16.0, Color::from_rgb(0.58,0.58,0.58))).center_x(20).center_y(20),
        ].spacing(8).align_y(Alignment::Center).width(300);

        let bottom = row![
            button(icon_el(ICON_LIST, 18.0, plist_c)).on_press(Message::TogglePlaylistPanel).style(ghost_btn).width(40).height(40).padding(p2(10, 10)),
            button(icon_el(ICON_DOTS, 18.0, C_DIM))  .on_press(Message::ShowMenu)            .style(ghost_btn).width(40).height(40).padding(p2(10, 10)),
        ].spacing(6).align_y(Alignment::Center);

        let mut info = column![
            container(text(title).size(17).font(Font { weight: font::Weight::Bold, ..Font::DEFAULT })).center_x(300),
            container(text(artist).size(13).style(|_| text::Style { color: Some(Color::from_rgb(0.36,0.36,0.36)) })).center_x(300),
        ].spacing(2).align_x(Alignment::Center);

        if !album.is_empty() {
            info = info.push(
                container(text(album).size(11)
                    .font(Font { style: font::Style::Italic, ..Font::DEFAULT })
                    .style(|_| text::Style { color: Some(Color::from_rgb(0.58,0.58,0.58)) }))
                    .center_x(300)
            );
        }

        let player_col = column![
            vertical_space().height(4),
            container(art).center_x(Length::Fill),
            vertical_space().height(14),
            container(wf).center_x(Length::Fill),
            vertical_space().height(4),
            container(time_row).center_x(Length::Fill),
            vertical_space().height(8),
            info,
            vertical_space().height(4),
            container(row![sr_badge, lyrics_btn].spacing(6)).center_x(Length::Fill),
            vertical_space().height(14),
            container(controls).center_x(Length::Fill),
            vertical_space().height(12),
            container(vol).center_x(Length::Fill),
            vertical_space().height(10),
            container(bottom).center_x(Length::Fill),
            vertical_space().height(8),
        ].align_x(Alignment::Center);

        // Build the content area — lyrics shown side by side if active
        let content: Element<Message> = if self.show_lyrics {
            if let Some(lyr) = lyrics {
                let lyrics_col = container(column![
                    row![
                        text("Lyrics").size(14)
                            .font(Font { weight: font::Weight::Bold, ..Font::DEFAULT })
                            .width(Length::Fill),
                        button(icon_el(ICON_CLOSE, 13.0, C_DIM))
                            .on_press(Message::HideLyrics)
                            .style(ghost_btn).padding(p2(3, 3)),
                    ].align_y(Alignment::Center).padding(p4(12.0, 12.0, 8.0, 16.0)),
                    container(Space::new(Length::Fill, 1))
                        .style(|_| cstyle(Color::from_rgb(0.84, 0.84, 0.84)))
                        .width(Length::Fill),
                    scrollable(
                        container(
                            text(lyr).size(13)
                                .line_height(iced::widget::text::LineHeight::Relative(1.75))
                                .style(|_| text::Style { color: Some(Color::from_rgb(0.22, 0.22, 0.22)) })
                        ).padding(p4(12.0, 16.0, 20.0, 16.0))
                    ).height(Length::Fill),
                ].spacing(0))
                .width(220)
                .height(Length::Fill)
                .style(|_| container::Style {
                    background: Some(iced::Background::Color(Color::from_rgb(0.955, 0.948, 0.942))),
                    border: iced::Border {
                        radius: iced::border::Radius { top_left: 0.0, top_right: 0.0, bottom_left: 0.0, bottom_right: 14.0 },
                        ..Default::default()
                    },
                    shadow: iced::Shadow {
                        color: Color { a: 0.06, ..Color::BLACK },
                        offset: iced::Vector::new(-2.0, 0.0),
                        blur_radius: 8.0,
                    },
                    text_color: None,
                });

                row![
                    scrollable(container(player_col).center_x(Length::Fill)).height(Length::Fill),
                    lyrics_col,
                ].into()
            } else {
                scrollable(container(player_col).center_x(Length::Fill)).height(Length::Fill).into()
            }
        } else {
            scrollable(container(player_col).center_x(Length::Fill)).height(Length::Fill).into()
        };

        let main: Element<Message> = column![titlebar, content].into();

        if self.show_menu { return self.view_settings_menu(main); }
        main
    }

    // ─── Playlist switcher overlay ───────────────────────────────────────────

    fn view_playlist_menu<'a>(&'a self, bg: Element<'a, Message>) -> Element<'a, Message> {
        let items: Vec<Element<Message>> = self.playlists.iter().enumerate().map(|(i, p)| {
            let active = i == self.active_playlist;
            let bg_c   = if active { Color::from_rgb(0.91,0.90,0.89) } else { Color::TRANSPARENT };
            button(
                row![
                    text(&p.name).size(13)
                        .font(if active { Font { weight: font::Weight::Semibold, ..Font::DEFAULT } } else { Font::DEFAULT })
                        .style(move |_| text::Style { color: Some(if active { Color::BLACK } else { Color::from_rgb(0.18,0.18,0.18) }) })
                        .width(Length::Fill),
                    if active { icon_el(ICON_VOLHI, 11.0, C_DIM) } else { Space::new(0,0).into() },
                ].align_y(Alignment::Center)
            )
            .on_press(Message::SelectPlaylist(i))
            .style(move |_, _| button::Style {
                background: Some(iced::Background::Color(bg_c)),
                border: iced::Border { radius: 6.0.into(), ..Default::default() },
                text_color: Color::BLACK, shadow: iced::Shadow::default(),
            })
            .padding(p2(8, 12)).width(Length::Fill).into()
        }).collect();

        let mk_sep = || -> Element<'static, Message> {
            container(Space::new(Length::Fill, 1))
                .style(|_| cstyle(Color::from_rgb(0.88,0.87,0.86)))
                .width(Length::Fill).padding(p2(2, 0)).into()
        };

        let panel = container(column![
            row![
                text("Playlists").size(14).font(Font { weight: font::Weight::Bold, ..Font::DEFAULT }).width(Length::Fill),
                button(icon_el(ICON_CLOSE, 13.0, C_DIM))
                    .on_press(Message::HidePlaylistMenu).style(ghost_btn).padding(p2(4,4)),
            ].align_y(Alignment::Center).padding(p2(10, 12)),
            mk_sep(),
            scrollable(column(items).spacing(2).padding(p2(6,6))).height(Length::Shrink),
            mk_sep(),
            button(row![icon_el(ICON_PLUS, 14.0, C_ACT), text("  New Playlist").size(12)].align_y(Alignment::Center))
                .on_press(Message::NewPlaylist).style(ghost_btn).padding(p2(8,12)).width(Length::Fill),
            button(row![icon_el(ICON_NOTE, 14.0, C_ACT), text("  Rename").size(12)].align_y(Alignment::Center))
                .on_press(Message::RenamePlaylist).style(ghost_btn).padding(p2(8,12)).width(Length::Fill),
            button(row![
                icon_el(ICON_CLOSE, 14.0, Color::from_rgb(0.72,0.30,0.30)),
                text("  Delete Playlist").size(12).style(|_| text::Style { color: Some(Color::from_rgb(0.68,0.28,0.28)) }),
            ].align_y(Alignment::Center))
            .on_press(Message::DeletePlaylist).style(ghost_btn).padding(p2(8,12)).width(Length::Fill),
        ])
        .style(|_| container::Style {
            background: Some(iced::Background::Color(Color::from_rgb(0.97,0.965,0.960))),
            border: iced::Border { radius: iced::border::Radius { top_left: 14.0, bottom_left: 14.0, top_right: 0.0, bottom_right: 0.0 }, ..Default::default() },
            shadow: iced::Shadow { color: Color { a: 0.12, ..Color::BLACK }, offset: iced::Vector::new(2.0, 0.0), blur_radius: 12.0 },
            text_color: None,
        })
        .width(220).height(Length::Fill);

        let scrim = container(Space::new(Length::Fill, Length::Fill))
            .style(|_| container::Style {
                background: Some(iced::Background::Color(Color { a: 0.15, r: 0.0, g: 0.0, b: 0.0 })),
                border: iced::Border::default(), shadow: iced::Shadow::default(), text_color: None,
            })
            .width(Length::Fill).height(Length::Fill);

        stack![bg, scrim, container(row![panel, horizontal_space()]).width(Length::Fill).height(Length::Fill)].into()
    }

    // ─── Settings menu ───────────────────────────────────────────────────────

    fn view_settings_menu<'a>(&self, bg: Element<'a, Message>) -> Element<'a, Message> {
        let mk_sep = || -> Element<'static, Message> {
            container(Space::new(Length::Fill, 1))
                .style(|_| cstyle(Color::from_rgb(0.88,0.87,0.86)))
                .width(Length::Fill).padding(p2(3,0)).into()
        };
        let mitem = |label: &'static str, msg: Message| -> Element<'static, Message> {
            button(text(label).size(13)).on_press(msg)
                .style(menu_item).padding(p2(8,16)).width(Length::Fill).into()
        };
        let match_cover_art = self.match_cover_art;
        let match_item: Element<Message> = button(
            row![
                // Checkmark box
                container(
                    if match_cover_art {
                        icon_el(ICON_CHECK, 12.0, Color::from_rgb(0.20, 0.55, 0.20))
                    } else {
                        Space::new(12, 12).into()
                    }
                )
                .width(20).height(20)
                .center_x(20).center_y(20)
                .style(move |_| container::Style {
                    background: Some(iced::Background::Color(if match_cover_art {
                        Color::from_rgb(0.88, 0.96, 0.88)
                    } else {
                        Color::from_rgb(0.93, 0.92, 0.91)
                    })),
                    border: iced::Border { radius: 4.0.into(), width: 1.0, color: Color::from_rgb(0.78, 0.78, 0.78) },
                    shadow: iced::Shadow::default(), text_color: None,
                }),
                text("Match Cover Art").size(13),
            ]
            .spacing(10)
            .align_y(Alignment::Center)
        )
        .on_press(Message::ToggleMatchCoverArt)
        .style(menu_item).padding(p2(8,16)).width(Length::Fill)
        .into();

        let menu = container(column![
            row![
                horizontal_space(),
                button(icon_el(ICON_CLOSE, 12.0, C_DIM)).on_press(Message::HideMenu).style(ghost_btn).padding(p2(4,8)),
            ].padding(p4(6.0,6.0,0.0,0.0)),
            mitem("Add Songs",       Message::AddFiles),
            mitem("Add Folder",      Message::AddFolder),
            mitem("Clear Playlist",  Message::ClearPlaylist),
            mk_sep(),
            match_item,
            mk_sep(),
            mitem("Quit", Message::CloseWindow),
        ].spacing(0))
        .style(|_| container::Style {
            background: Some(iced::Background::Color(Color::from_rgb(0.97,0.965,0.960))),
            border: iced::Border { radius: iced::border::Radius { top_right: 14.0, bottom_right: 14.0, top_left: 0.0, bottom_left: 0.0 }, ..Default::default() },
            shadow: iced::Shadow { color: Color { a: 0.10, ..Color::BLACK }, offset: iced::Vector::new(2.0,0.0), blur_radius: 10.0 },
            text_color: None,
        })
        .width(200).height(Length::Fill);

        // Backdrop — clicking outside closes menu
        let backdrop: Element<Message> = button(Space::new(Length::Fill, Length::Fill))
            .on_press(Message::HideMenu)
            .style(|_, _| button::Style {
                background: Some(iced::Background::Color(Color { a: 0.0, r: 0.0, g: 0.0, b: 0.0 })),
                border: iced::Border::default(), text_color: Color::TRANSPARENT, shadow: iced::Shadow::default(),
            })
            .width(Length::Fill).height(Length::Fill)
            .into();

        stack![bg, backdrop, container(row![horizontal_space(), menu]).width(Length::Fill).height(Length::Fill)].into()
    }

    // ─── Input dialog ────────────────────────────────────────────────────────

    fn view_input_dialog(&self) -> Element<Message> {
        let title_str = match self.input_purpose {
            Some(InputPurpose::NewPlaylist)    => "New Playlist",
            Some(InputPurpose::RenamePlaylist) => "Rename Playlist",
            None => "Input",
        };
        let dialog = container(column![
            text(title_str).size(14).font(Font { weight: font::Weight::Bold, ..Font::DEFAULT }),
            vertical_space().height(10),
            text_input("Name...", &self.input_value).on_input(Message::InputChanged).on_submit(Message::InputConfirm).size(13).padding(p2(8,10)),
            vertical_space().height(14),
            row![
                horizontal_space(),
                button(text("Cancel").size(12)).on_press(Message::InputCancel).style(ghost_btn).padding(p2(6,18)),
                button(text("OK").size(12)).on_press(Message::InputConfirm).style(play_btn).padding(p2(6,24)),
            ].spacing(8),
        ].padding(24))
        .style(|_| container::Style {
            background: Some(iced::Background::Color(Color::from_rgb(0.965,0.958,0.950))),
            border: iced::Border { radius: 12.0.into(), width: 1.0, color: Color::from_rgb(0.85,0.84,0.83) },
            shadow: iced::Shadow { color: Color { a: 0.20, ..Color::BLACK }, offset: iced::Vector::new(0.0,4.0), blur_radius: 16.0 },
            text_color: None,
        }).width(320);

        stack![
            container(Space::new(Length::Fill, Length::Fill))
                .style(|_| container::Style {
                    background: Some(iced::Background::Color(Color { a: 0.35, r: 0.0, g: 0.0, b: 0.0 })),
                    border: iced::Border::default(), shadow: iced::Shadow::default(), text_color: None,
                }).width(Length::Fill).height(Length::Fill),
            container(dialog).center_x(Length::Fill).center_y(Length::Fill),
        ].into()
    }
}

// ─── Waveform canvas ─────────────────────────────────────────────────────────

struct WaveformProg<'a> {
    bars: &'a [f32; NUM_BARS], progress: f32, played: Color, unplayed: Color,
}

impl<'a> canvas::Program<Message> for WaveformProg<'a> {
    type State = ();

    fn draw(&self, _: &(), renderer: &iced::Renderer, _: &Theme, bounds: iced::Rectangle, _: iced::mouse::Cursor) -> Vec<canvas::Geometry<iced::Renderer>> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());
        let (w, h, n) = (bounds.width, bounds.height, NUM_BARS as f32);
        let bar_w = w / n;
        let gap   = (bar_w * 0.18_f32).max(0.5);
        let bw    = (bar_w - gap * 2.0).max(1.0);
        let r     = (bw / 2.0).min(3.0);
        for i in 0..NUM_BARS {
            let frac  = (i as f32 + 0.5) / n;
            let color = if frac <= self.progress { self.played } else { self.unplayed };
            let bh    = (self.bars[i] * h * 0.94).max(2.0);
            let x     = i as f32 * bar_w + gap;
            let y     = (h - bh) / 2.0;
            frame.fill(&canvas::Path::rounded_rectangle(iced::Point::new(x,y), iced::Size::new(bw,bh), r.into()), color);
        }
        vec![frame.into_geometry()]
    }

    fn update(&self, _: &mut (), event: canvas::Event, bounds: iced::Rectangle, cursor: iced::mouse::Cursor) -> (canvas::event::Status, Option<Message>) {
        if let canvas::Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left)) = event {
            if let Some(pos) = cursor.position_in(bounds) {
                let frac = (pos.x / bounds.width).clamp(0.0, 1.0) as f64;
                return (canvas::event::Status::Captured, Some(Message::SeekFrac(frac)));
            }
        }
        (canvas::event::Status::Ignored, None)
    }
}

// ─── Button styles ───────────────────────────────────────────────────────────

fn ghost_btn(_: &Theme, s: button::Status) -> button::Style {
    button::Style {
        background: if matches!(s, button::Status::Hovered | button::Status::Pressed) {
            Some(iced::Background::Color(Color::from_rgb(0.86,0.85,0.84)))
        } else { None },
        border: iced::Border { radius: 7.0.into(), ..Default::default() },
        text_color: C_ACT, shadow: iced::Shadow::default(),
    }
}

fn round_btn(_: &Theme, s: button::Status) -> button::Style {
    button::Style {
        background: Some(iced::Background::Color(match s {
            button::Status::Hovered => Color::from_rgb(0.83,0.83,0.83),
            button::Status::Pressed => Color::from_rgb(0.77,0.77,0.77),
            _                       => Color::from_rgb(0.905,0.905,0.905),
        })),
        border: iced::Border { radius: 22.0.into(), ..Default::default() },
        text_color: C_ACT, shadow: iced::Shadow::default(),
    }
}

fn play_btn(_: &Theme, s: button::Status) -> button::Style {
    button::Style {
        background: Some(iced::Background::Color(match s {
            button::Status::Hovered => Color::from_rgb(0.28,0.28,0.28),
            button::Status::Pressed => Color::from_rgb(0.18,0.18,0.18),
            _                       => Color::from_rgb(0.22,0.22,0.22),
        })),
        border: iced::Border { radius: 27.0.into(), ..Default::default() },
        text_color: C_WHT, shadow: iced::Shadow::default(),
    }
}

fn menu_item(_: &Theme, s: button::Status) -> button::Style {
    button::Style {
        background: if matches!(s, button::Status::Hovered) {
            Some(iced::Background::Color(Color::from_rgb(0.92,0.91,0.90)))
        } else { None },
        border: iced::Border { radius: 6.0.into(), ..Default::default() },
        text_color: Color::from_rgb(0.13,0.13,0.13), shadow: iced::Shadow::default(),
    }
}
