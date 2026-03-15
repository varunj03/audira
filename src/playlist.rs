// playlist.rs — playlist data and persistence

use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};

use crate::metadata::TrackMeta;

#[derive(Debug, Clone)]
pub struct Track {
    pub path: PathBuf,
    pub meta: TrackMeta,
}

impl Track {
    pub fn load(path: PathBuf) -> Self {
        let meta = TrackMeta::load(&path);
        Track { path, meta }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedPlaylist {
    pub name:   String,
    pub tracks: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Playlist {
    pub name:   String,
    pub tracks: Vec<Track>,
}

impl Playlist {
    pub fn new(name: impl Into<String>) -> Self {
        Playlist { name: name.into(), tracks: Vec::new() }
    }

    pub fn add_file(&mut self, path: PathBuf) {
        self.tracks.push(Track::load(path));
    }

    pub fn add_dir(&mut self, dir: &Path) {
        let exts = ["mp3","flac","ogg","wav","m4a","aac","wma","opus"];
        let mut paths: Vec<PathBuf> = walkdir(dir)
            .into_iter()
            .filter(|p| {
                p.extension()
                    .and_then(|e| e.to_str())
                    .map(|e| exts.contains(&e.to_lowercase().as_str()))
                    .unwrap_or(false)
            })
            .collect();
        paths.sort();
        for p in paths { self.add_file(p); }
    }

    #[allow(dead_code)]
    pub fn total_duration(&self) -> f64 {
        self.tracks.iter().map(|t| t.meta.duration).sum()
    }

    pub fn remaining_duration(&self, from_index: usize, pos: f64) -> f64 {
        let mut total: f64 = self.tracks[from_index..].iter().map(|t| t.meta.duration).sum();
        total -= pos;
        total.max(0.0)
    }

    pub fn to_saved(&self) -> SavedPlaylist {
        SavedPlaylist {
            name:   self.name.clone(),
            tracks: self.tracks.iter()
                .map(|t| t.path.to_string_lossy().to_string())
                .collect(),
        }
    }

    pub fn from_saved(saved: &SavedPlaylist) -> Self {
        let mut pl = Playlist::new(&saved.name);
        for path in &saved.tracks {
            let p = PathBuf::from(path);
            if p.exists() { pl.add_file(p); }
        }
        pl
    }
}

fn walkdir(dir: &Path) -> Vec<PathBuf> {
    let mut result = Vec::new();
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                result.extend(walkdir(&path));
            } else {
                result.push(path);
            }
        }
    }
    result
}

// ── Persistence ───────────────────────────────────────────────────────────────

pub fn save_playlists(playlists: &[Playlist]) -> std::io::Result<()> {
    let path = data_path();
    std::fs::create_dir_all(path.parent().unwrap())?;
    let saved: Vec<SavedPlaylist> = playlists.iter().map(|p| p.to_saved()).collect();
    let json = serde_json::to_string_pretty(&saved)?;
    std::fs::write(path, json)?;
    Ok(())
}

pub fn load_playlists() -> Vec<Playlist> {
    let path = data_path();
    if !path.exists() { return vec![Playlist::new("Playlist")]; }
    let json = match std::fs::read_to_string(&path) {
        Ok(s) => s,
        Err(_) => return vec![Playlist::new("Playlist")],
    };
    let saved: Vec<SavedPlaylist> = match serde_json::from_str(&json) {
        Ok(v) => v,
        Err(_) => return vec![Playlist::new("Playlist")],
    };
    let playlists: Vec<Playlist> = saved.iter().map(Playlist::from_saved).collect();
    if playlists.is_empty() { vec![Playlist::new("Playlist")] } else { playlists }
}

fn data_path() -> PathBuf {
    dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("AmberolPlayer")
        .join("playlists.json")
}
