// waveform.rs — extract RMS energy bars from raw file bytes

use std::path::Path;

pub const NUM_BARS: usize = 80;

/// Read raw audio bytes and compute per-segment RMS energy.
/// Returns normalized bars in 0.10..=1.0.
pub fn extract(path: &Path) -> [f32; NUM_BARS] {
    match try_extract(path) {
        Some(bars) => bars,
        None => default_bars(path),
    }
}

fn try_extract(path: &Path) -> Option<[f32; NUM_BARS]> {
    use std::io::{Read, Seek, SeekFrom};
    use std::fs::File;

    let meta = std::fs::metadata(path).ok()?;
    let file_size = meta.len() as usize;
    if file_size < NUM_BARS * 32 { return None; }

    let skip = (file_size / 14).min(16384);
    let data_len = file_size.saturating_sub(skip);
    let chunk = (data_len / NUM_BARS).min(2048).max(64);
    let mut buf = vec![0u8; chunk];

    let mut file = File::open(path).ok()?;
    let mut bars = [0f32; NUM_BARS];

    for i in 0..NUM_BARS {
        let pos = skip + (data_len * i / NUM_BARS);
        let pos = pos.min(file_size.saturating_sub(chunk)) as u64;
        file.seek(SeekFrom::Start(pos)).ok()?;
        let read = file.read(&mut buf).unwrap_or(0);
        if read < 2 { bars[i] = 0.1; continue; }

        // Treat as 16-bit little-endian PCM
        let mut energy = 0f64;
        let pairs = read / 2;
        for j in 0..pairs {
            let lo = buf[j * 2] as i16;
            let hi = buf[j * 2 + 1] as i16;
            let sample = (lo | (hi << 8)) as f64 / 32768.0;
            energy += sample * sample;
        }
        bars[i] = (energy / pairs as f64).sqrt() as f32;
    }

    // Normalize to 0.10–1.0
    let max = bars.iter().cloned().fold(0f32, f32::max);
    let min = bars.iter().cloned().fold(f32::MAX, f32::min);
    if max - min < 0.001 { return None; }
    for b in bars.iter_mut() {
        *b = 0.10 + 0.90 * (*b - min) / (max - min);
    }

    // 3-pass smoothing
    for _ in 0..3 {
        for i in 1..NUM_BARS - 1 {
            bars[i] = (bars[i - 1] + bars[i] * 2.0 + bars[i + 1]) / 4.0;
        }
    }

    Some(bars)
}

/// Deterministic seed-based default (used when file can't be read or too small)
fn default_bars(path: &Path) -> [f32; NUM_BARS] {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    path.hash(&mut hasher);
    let seed = hasher.finish();

    let mut bars = [0f32; NUM_BARS];
    let mut rng = seed;
    for (i, b) in bars.iter_mut().enumerate() {
        rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        let noise = ((rng >> 33) as f32) / u32::MAX as f32;  // 0..1
        let pos = i as f32 / NUM_BARS as f32;
        let envelope = (pos * std::f32::consts::PI).sin() * 0.7 + 0.25;
        *b = (noise * envelope * 0.8 + 0.12).clamp(0.1, 1.0);
    }
    bars
}
