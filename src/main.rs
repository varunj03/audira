#![windows_subsystem = "windows"]

mod audio;
mod metadata;
mod playlist;
mod theme;
mod waveform;
mod ui;

fn main() -> iced::Result {
    iced::application("Audira", ui::Audira::update, ui::Audira::view)
        .antialiasing(true)
        .window(iced::window::Settings {
            size:         iced::Size::new(820.0, 600.0),
            min_size:     Some(iced::Size::new(620.0, 480.0)),
            decorations:  false,
            ..Default::default()
        })
        .theme(ui::Audira::theme)
        .subscription(ui::Audira::subscription)
        .run_with(ui::Audira::new)
}
