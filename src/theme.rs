// theme.rs — color palette extraction from album art

use iced::Color;

/// Extract the most saturated color from raw image bytes (JPEG/PNG).
pub fn extract_accent(image_bytes: &[u8]) -> Option<Color> {
    let img = image::load_from_memory(image_bytes).ok()?.to_rgba8();
    let (w, h) = img.dimensions();

    // Sample a 32x32 grid
    let step_x = (w / 32).max(1);
    let step_y = (h / 32).max(1);

    let mut best_sat: f32 = 0.0;
    let mut best = Color::from_rgb(0.6, 0.55, 0.5);

    for y in (0..h).step_by(step_y as usize) {
        for x in (0..w).step_by(step_x as usize) {
            let p = img.get_pixel(x, y);
            let r = p[0] as f32 / 255.0;
            let g = p[1] as f32 / 255.0;
            let b = p[2] as f32 / 255.0;

            let max = r.max(g).max(b);
            let min = r.min(g).min(b);
            let sat = if max == 0.0 { 0.0 } else { (max - min) / max };
            let val = max;

            if sat > best_sat && val > 0.25 && val < 0.92 {
                best_sat = sat;
                // Darken slightly for use as waveform / accent
                let factor = 0.78;
                best = Color::from_rgb(r * factor, g * factor, b * factor);
            }
        }
    }

    if best_sat > 0.15 { Some(best) } else { None }
}

/// Lighten a color for gradient top stop.
pub fn lighten(c: Color, factor: f32) -> Color {
    Color::from_rgb(
        (c.r * factor).min(1.0),
        (c.g * factor).min(1.0),
        (c.b * factor).min(1.0),
    )
}

/// Fallback gradient palette by index.
pub fn fallback_palette(idx: usize) -> (Color, Color) {
    let palettes: &[(Color, Color)] = &[
        (Color::from_rgb(0.80, 0.67, 0.60), Color::from_rgb(0.67, 0.53, 0.80)),
        (Color::from_rgb(0.53, 0.73, 0.67), Color::from_rgb(0.40, 0.67, 0.53)),
        (Color::from_rgb(0.60, 0.67, 0.80), Color::from_rgb(0.47, 0.60, 0.73)),
        (Color::from_rgb(0.80, 0.73, 0.53), Color::from_rgb(0.80, 0.60, 0.47)),
        (Color::from_rgb(0.73, 0.60, 0.73), Color::from_rgb(0.60, 0.67, 0.80)),
    ];
    palettes[idx % palettes.len()]
}

pub const NEUTRAL_TOP: Color    = Color { r: 0.83, g: 0.82, b: 0.80, a: 1.0 };
pub const NEUTRAL_BOTTOM: Color = Color { r: 0.75, g: 0.74, b: 0.73, a: 1.0 };
pub const WAVEFORM_PLAYED: Color   = Color { r: 0.47, g: 0.47, b: 0.47, a: 1.0 };
pub const WAVEFORM_UNPLAYED: Color = Color { r: 0.85, g: 0.85, b: 0.85, a: 1.0 };
