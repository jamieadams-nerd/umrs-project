// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Jamie Adams
//
//! UMRS Core Spinner Utilities
//!
//! Minimal terminal spinner for indicating in-progress operations.
//!
//! Guarantees:
//! - Deterministic spinner frame sequencing
//! - No side effects beyond stdout rendering
//! - Stable, dependency-free API for CLI progress indication
//!
//! Non-goals:
//! - Accurate progress measurement or task completion estimation
//! - Asynchronous task scheduling or concurrency management
//! - Terminal capability detection or feature negotiation
//

use std::io::{self, Write};
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::thread;
use std::time::Duration;

use crate::console::ansi::AnsiColor;

/// Positioning mode for spinner animation relative to its associated message.
///
/// Determines whether the spinner glyph appears before or after the
/// accompanying text label during rendering.
#[derive(Clone, Copy, Debug, Default)]
pub enum SpinnerPosition {
    #[default]
    Prefix,
    Suffix,
}

/// Visual animation style for spinner rendering.
///
/// Each style defines a fixed sequence of Unicode frames used to convey
/// activity or progress in a terminal-friendly manner.
#[derive(Clone, Copy, Debug, Default)]
pub enum SpinnerStyle {
    #[default]
    Line,
    Dots,
    Arrow,
}

impl SpinnerStyle {
    /// Return the animation frame sequence associated with this spinner style.
    ///
    /// Each frame is a Unicode string representing one animation step.
    ///
    /// # Returns
    ///
    /// A static slice of frame strings corresponding to the selected style.
    ///
    /// # Behavior
    ///
    /// - Provides deterministic frame ordering for animation playback.
    /// - Does not allocate or perform I/O.
    ///
    /// # Side Effects
    ///
    /// - None.
    ///
    /// # Panics
    ///
    /// This function does not intentionally panic.
    pub fn frames(self) -> &'static [&'static str] {
        match self {
            SpinnerStyle::Line => &["|", "/", "-", "\\"],
            SpinnerStyle::Dots => &[".", "..", "...", ".."],
            SpinnerStyle::Arrow => &["<", "<<", "<<<", "<<"],
        }
    }

    /// Return the default final marker glyph for this spinner style.
    ///
    /// The final marker is displayed when the spinner is stopped to indicate
    /// completion or termination of the associated operation.
    ///
    /// # Returns
    ///
    /// A static string representing the default completion marker.
    ///
    /// # Behavior
    ///
    /// - Provides a deterministic marker glyph per spinner style.
    /// - Does not allocate or perform I/O.
    ///
    /// # Side Effects
    ///
    /// - None.
    ///
    /// # Panics
    ///
    /// This function does not intentionally panic.
    pub fn default_final_marker(self) -> &'static str {
        match self {
            SpinnerStyle::Line => "✓",
            SpinnerStyle::Dots => "done",
            SpinnerStyle::Arrow => "ok",
        }
    }
}

/// Configuration options for customizing spinner behavior and appearance.
///
/// All fields are optional. Any field not explicitly set will fall back
/// to the corresponding default behavior defined by the spinner subsystem.
#[derive(Default)]
pub struct SpinnerOptions {
    /// Optional animation style override for the spinner.
    ///
    /// If `None`, the default spinner style will be used.
    pub style: Option<SpinnerStyle>,

    /// Optional position override for the spinner glyph relative to the message.
    ///
    /// If `None`, the default spinner position will be used.
    pub position: Option<SpinnerPosition>,

    /// Optional custom marker displayed when the spinner is stopped.
    ///
    /// If `None`, the style-specific default final marker will be used.
    pub final_marker: Option<String>,

    /// Optional ANSI color override for spinner glyph rendering.
    ///
    /// If `None`, the default spinner color will be used.
    pub spinner_color: Option<AnsiColor>,

    /// Optional ANSI color override for spinner message text rendering.
    ///
    /// If `None`, the default message color will be used.
    pub message_color: Option<AnsiColor>,

    /// Optional delay interval between animation frames, in milliseconds.
    ///
    /// If `None`, the default frame delay will be used.
    pub frame_delay_ms: Option<u64>,
}

/// Active spinner instance managing an in-progress terminal animation.
///
/// A spinner represents a transient, human-facing progress indicator that
/// renders animated frames until explicitly stopped.
pub struct Spinner {
    stop_flag: Arc<AtomicBool>,
    handle: Option<thread::JoinHandle<()>>,
}

impl Spinner {
    /// Start a spinner using default configuration options.
    ///
    /// Creates and begins rendering a spinner animation associated with the
    /// provided message using default styling, timing, and positioning.
    ///
    /// # Parameters
    ///
    /// - `message`: Human-readable message describing the in-progress operation.
    ///
    /// # Returns
    ///
    /// An active `Spinner` instance managing the running animation.
    ///
    /// # Behavior
    ///
    /// - Initializes a spinner with default configuration values.
    /// - Immediately begins rendering animation frames.
    /// - Associates the spinner lifecycle with the returned handle.
    ///
    /// # Side Effects
    ///
    /// - Writes animated output to the terminal.
    /// - Spawns background animation activity.
    ///
    /// # Panics
    ///
    /// This function does not intentionally panic.
    pub fn start(message: impl Into<String>) -> Spinner {
        Spinner::start_with_options(message, SpinnerOptions::default())
    }

    /// Start a spinner using explicit configuration options.
    ///
    /// Creates and begins rendering a spinner animation associated with the
    /// provided message and customized using the supplied options.
    ///
    /// # Parameters
    ///
    /// - `message`: Human-readable message describing the in-progress operation.
    /// - `opts`: Spinner configuration overrides for style, timing, and appearance.
    ///
    /// # Returns
    ///
    /// An active `Spinner` instance managing the running animation.
    ///
    /// # Behavior
    ///
    /// - Initializes a spinner using the provided configuration options.
    /// - Immediately begins rendering animation frames.
    /// - Associates the spinner lifecycle with the returned handle.
    ///
    /// # Side Effects
    ///
    /// - Writes animated output to the terminal.
    /// - Spawns background animation activity.
    ///
    /// # Panics
    ///
    /// This function does not intentionally panic.
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
                    format!("{}{}{}", color.start(), frame, AnsiColor::reset())
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

    /// Stop the spinner and finalize its terminal output.
    ///
    /// Terminates the running animation and renders the final completion marker
    /// along with the associated message.
    ///
    /// # Behavior
    ///
    /// - Stops the background animation loop.
    /// - Clears transient spinner frames from the terminal.
    /// - Renders the final marker and message.
    ///
    /// # Side Effects
    ///
    /// - Writes final output to the terminal.
    /// - Terminates background animation activity.
    ///
    /// # Panics
    ///
    /// This function does not intentionally panic.
    pub fn stop(mut self) {
        self.stop_flag.store(true, Ordering::Relaxed);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}
