// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Jamie Adams
//
// Unclassified MLS Reference System Project
//
//  Provide ability to display a message
//   with a spinner characer to show progress
//   on the terminal. 
//
use std::io::{self, Write};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;
use std::time::Duration;

use crate::ansi::AnsiColor;

#[derive(Clone, Copy, Debug)]
pub enum SpinnerPosition {
    Prefix,
    Suffix,
}

impl Default for SpinnerPosition {
    fn default() -> Self {
        SpinnerPosition::Prefix
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SpinnerStyle {
    Line,
    Dots,
    Arrow,
}

impl Default for SpinnerStyle {
    fn default() -> Self {
        SpinnerStyle::Line
    }
}

impl SpinnerStyle {
    pub fn frames(self) -> &'static [&'static str] {
        match self {
            SpinnerStyle::Line => &["|", "/", "-", "\\"],
            SpinnerStyle::Dots => &[".", "..", "...", ".."],
            SpinnerStyle::Arrow => &["<", "<<", "<<<", "<<"],
        }
    }

    pub fn default_final_marker(self) -> &'static str {
        match self {
            SpinnerStyle::Line => "✓",
            SpinnerStyle::Dots => "done",
            SpinnerStyle::Arrow => "ok",
        }
    }
}

#[derive(Clone, Debug)]
pub struct SpinnerOptions {
    pub style: Option<SpinnerStyle>,
    pub position: Option<SpinnerPosition>,
    pub final_marker: Option<String>,
    pub spinner_color: Option<AnsiColor>,
    pub message_color: Option<AnsiColor>,
    pub frame_delay_ms: Option<u64>,
}

impl Default for SpinnerOptions {
    fn default() -> Self {
        SpinnerOptions {
            style: None,
            position: None,
            final_marker: None,
            spinner_color: None,
            message_color: None,
            frame_delay_ms: None,
        }
    }
}

pub struct Spinner {
    stop_flag: Arc<AtomicBool>,
    handle: Option<thread::JoinHandle<()>>,
}

impl Spinner {
    pub fn start(message: impl Into<String>) -> Spinner {
        Spinner::start_with_options(message, SpinnerOptions::default())
    }

    pub fn start_with_options(
        message: impl Into<String>,
        opts: SpinnerOptions,
    ) -> Spinner {
        let style = opts.style.unwrap_or_default();
        let position = opts.position.unwrap_or_default();
        let frames = style.frames();
        let final_marker = opts
            .final_marker
            .as_deref()
            .unwrap_or(style.default_final_marker())
            .to_string();

        let spinner_color = opts.spinner_color;
        let message_color = opts.message_color;
        let frame_delay_ms = opts.frame_delay_ms.unwrap_or(120);

        let message_raw = message.into();

        let stop_flag = Arc::new(AtomicBool::new(false));
        let stop_flag_clone = Arc::clone(&stop_flag);

        let handle = thread::spawn(move || {
            let mut i = 0usize;
            let mut stderr = io::stderr();

            let message_rendered = if let Some(color) = message_color {
                format!(
                    "{}{}{}",
                    color.start(),
                    message_raw,
                    AnsiColor::reset()
                )
            } else {
                message_raw.clone()
            };

            while !stop_flag_clone.load(Ordering::Relaxed) {
                let frame = frames[i % frames.len()];
                i = i.wrapping_add(1);

                let visible_width = frame.chars().count();

                let frame_rendered = if let Some(color) = spinner_color {
                    format!(
                        "{}{}{}",
                        color.start(),
                        frame,
                        AnsiColor::reset()
                    )
                } else {
                    frame.to_string()
                };

                let line = match position {
                    SpinnerPosition::Prefix => {
                        format!("{} {}", frame_rendered, message_rendered)
                    }
                    SpinnerPosition::Suffix => {
                        format!("{} {}", message_rendered, frame_rendered)
                    }
                };

                let _ = write!(stderr, "\r{}", line);
                let _ = stderr.flush();

                thread::sleep(Duration::from_millis(frame_delay_ms));

                let erase = match position {
                    SpinnerPosition::Prefix => "\r".to_string(),
                    SpinnerPosition::Suffix => {
                        let mut s = String::new();
                        for _ in 0..(visible_width + 1) {
                            s.push('\x08');
                        }
                        s
                    }
                };

                let _ = write!(stderr, "{}", erase);
                let _ = stderr.flush();
            }

            let final_marker_rendered = if let Some(color) = spinner_color {
                format!(
                    "{}{}{}",
                    color.start(),
                    final_marker,
                    AnsiColor::reset()
                )
            } else {
                final_marker
            };

            let final_line = match position {
                SpinnerPosition::Prefix => {
                    format!("{} {}", final_marker_rendered, message_rendered)
                }
                SpinnerPosition::Suffix => {
                    format!("{} {}", message_rendered, final_marker_rendered)
                }
            };

            let _ = write!(stderr, "\r{}\n", final_line);
            let _ = stderr.flush();
        });

        Spinner {
            stop_flag,
            handle: Some(handle),
        }
    }

    pub fn stop(mut self) {
        self.stop_flag.store(true, Ordering::Relaxed);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}
