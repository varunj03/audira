// icons.rs — inline SVG icon data (Phosphor Icons, MIT license)
// https://phosphoricons.com

/// Returns SVG bytes for the given icon name
pub fn svg_bytes(name: &str) -> &'static [u8] {
    match name {
        "play"       => include_bytes!("../icons/play.svg"),
        "pause"      => include_bytes!("../icons/pause.svg"),
        "skip-back"  => include_bytes!("../icons/skip-back.svg"),
        "skip-fwd"   => include_bytes!("../icons/skip-fwd.svg"),
        "repeat"     => include_bytes!("../icons/repeat.svg"),
        "repeat-one" => include_bytes!("../icons/repeat-one.svg"),
        "shuffle"    => include_bytes!("../icons/shuffle.svg"),
        "list"       => include_bytes!("../icons/list.svg"),
        "hamburger"  => include_bytes!("../icons/hamburger.svg"),
        "plus"       => include_bytes!("../icons/plus.svg"),
        "close"      => include_bytes!("../icons/close.svg"),
        "folder"     => include_bytes!("../icons/folder.svg"),
        "music-note" => include_bytes!("../icons/music-note.svg"),
        "volume-low" => include_bytes!("../icons/volume-low.svg"),
        "volume-high"=> include_bytes!("../icons/volume-high.svg"),
        "dots"       => include_bytes!("../icons/dots.svg"),
        _            => include_bytes!("../icons/music-note.svg"),
    }
}
