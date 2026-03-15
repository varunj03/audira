// audio.rs — rodio-based playback engine with reliable seeking

use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::time::Duration;

use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};

pub struct AudioEngine {
    _stream:              OutputStream,
    handle:               OutputStreamHandle,
    sink:                 Option<Sink>,
    current_path:         Option<PathBuf>,
    play_start:           Option<std::time::Instant>,
    elapsed_before_pause: f64,
    pub duration:         f64,
    is_paused:            bool,
}

impl AudioEngine {
    pub fn new() -> Option<Self> {
        let (stream, handle) = OutputStream::try_default().ok()?;
        Some(Self {
            _stream: stream,
            handle,
            sink: None,
            current_path: None,
            play_start: None,
            elapsed_before_pause: 0.0,
            duration: 0.0,
            is_paused: false,
        })
    }

    pub fn play_file(&mut self, path: &Path, duration_secs: f64) -> bool {
        if let Some(s) = &self.sink { s.stop(); }
        self.sink = None;

        let Ok(file)   = File::open(path)                   else { return false; };
        let Ok(source) = Decoder::new(BufReader::new(file)) else { return false; };
        let Ok(sink)   = Sink::try_new(&self.handle)        else { return false; };

        sink.append(source);
        self.sink                 = Some(sink);
        self.current_path         = Some(path.to_path_buf());
        self.duration             = duration_secs;
        self.elapsed_before_pause = 0.0;
        self.play_start           = Some(std::time::Instant::now());
        self.is_paused            = false;
        true
    }

    pub fn pause(&mut self) {
        if let Some(sink) = &self.sink {
            if !sink.is_paused() {
                sink.pause();
                if let Some(start) = self.play_start.take() {
                    self.elapsed_before_pause += start.elapsed().as_secs_f64();
                }
                self.is_paused = true;
            }
        }
    }

    pub fn resume(&mut self) {
        if let Some(sink) = &self.sink {
            if sink.is_paused() {
                sink.play();
                self.play_start = Some(std::time::Instant::now());
                self.is_paused  = false;
            }
        }
    }

    pub fn stop(&mut self) {
        if let Some(s) = &self.sink { s.stop(); }
        self.sink                 = None;
        self.play_start           = None;
        self.elapsed_before_pause = 0.0;
        self.is_paused            = false;
    }

    /// Seek by reopening and skipping — works for all formats including MP3.
    pub fn seek(&mut self, secs: f64) {
        let path = match &self.current_path {
            Some(p) => p.clone(),
            None    => return,
        };
        let was_paused = self.is_paused;

        // Stop old sink
        if let Some(s) = &self.sink { s.stop(); }
        self.sink = None;

        // Reopen file
        let Ok(file)   = File::open(&path)                  else { return; };
        let Ok(source) = Decoder::new(BufReader::new(file)) else { return; };
        let Ok(sink)   = Sink::try_new(&self.handle)        else { return; };

        // Skip forward using skip_duration (works correctly for all formats)
        let skip_dur   = Duration::from_secs_f64(secs.max(0.0));
        let skipped    = source.skip_duration(skip_dur);
        sink.append(skipped);

        if was_paused { sink.pause(); }

        self.sink                 = Some(sink);
        self.elapsed_before_pause = secs;
        self.is_paused            = was_paused;
        if !was_paused {
            self.play_start = Some(std::time::Instant::now());
        } else {
            self.play_start = None;
        }
    }

    pub fn set_volume(&mut self, vol: f32) {
        if let Some(sink) = &self.sink { sink.set_volume(vol); }
    }

    /// Returns (position_secs, is_finished).
    pub fn tick(&mut self) -> (f64, bool) {
        let pos = if let Some(start) = self.play_start {
            (self.elapsed_before_pause + start.elapsed().as_secs_f64()).min(self.duration)
        } else {
            self.elapsed_before_pause
        };

        let finished = self.duration > 0.0
            && pos >= self.duration - 0.3
            && self.sink.as_ref().map(|s| s.empty()).unwrap_or(false)
            && !self.is_paused;

        (pos, finished)
    }
}
