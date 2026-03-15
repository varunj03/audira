// metadata.rs — lofty tag reading

use std::path::Path;
use lofty::prelude::*;
use lofty::probe::Probe;

#[derive(Debug, Clone)]
pub struct TrackMeta {
    pub title:       String,
    pub artist:      String,
    pub album:       String,
    pub duration:    f64,
    pub sample_rate: Option<u32>,
    pub lyrics:      Option<String>,
    pub cover_art:   Option<Vec<u8>>,
}

impl TrackMeta {
    pub fn load(path: &Path) -> Self {
        let stem = path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Unknown")
            .to_string();

        let mut meta = TrackMeta {
            title:       stem,
            artist:      "Unknown Artist".into(),
            album:       String::new(),
            duration:    0.0,
            sample_rate: None,
            lyrics:      None,
            cover_art:   None,
        };

        let tagged = match Probe::open(path).and_then(|p| p.read()) {
            Ok(t)  => t,
            Err(_) => return meta,
        };

        let props        = tagged.properties();
        meta.duration    = props.duration().as_secs_f64();
        meta.sample_rate = props.sample_rate();

        if let Some(tag) = tagged.primary_tag() {
            if let Some(t) = tag.title()  { meta.title  = t.to_string(); }
            if let Some(a) = tag.artist() { meta.artist = a.to_string(); }
            if let Some(a) = tag.album()  { meta.album  = a.to_string(); }

            // Strip LRC timestamps like [01:23.45] or [01:23]
            if let Some(raw) = tag.get_string(&lofty::tag::ItemKey::Lyrics) {
                meta.lyrics = Some(strip_lrc_timestamps(raw));
            }

            if let Some(pic) = tag.pictures().first() {
                meta.cover_art = Some(pic.data().to_vec());
            }
        }

        meta
    }

    pub fn format_duration(&self) -> String {
        format_secs(self.duration)
    }
}

/// Strip LRC-style timestamps [mm:ss.xx] or [mm:ss] from each line.
fn strip_lrc_timestamps(raw: &str) -> String {
    let mut out = String::new();
    for line in raw.lines() {
        let mut s = line.trim();
        // Strip all leading [xx:xx.xx] or [xx:xx] tags
        loop {
            if s.starts_with('[') {
                if let Some(end) = s.find(']') {
                    let tag = &s[1..end];
                    // Check if it looks like a timestamp (digits and colon/dot)
                    let is_ts = tag.chars().all(|c| c.is_ascii_digit() || c == ':' || c == '.');
                    if is_ts {
                        s = s[end + 1..].trim_start();
                        continue;
                    }
                }
            }
            break;
        }
        if !s.is_empty() {
            out.push_str(s);
            out.push('\n');
        }
    }
    out.trim_end().to_string()
}

pub fn format_secs(secs: f64) -> String {
    let s = secs as u64;
    format!("{}:{:02}", s / 60, s % 60)
}

pub fn format_secs_remaining(total: f64, pos: f64) -> String {
    let rem = (total - pos).max(0.0) as u64;
    format!("-{}:{:02}", rem / 60, rem % 60)
}
