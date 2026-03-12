// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams

//! # Status Bar — Single-Line Status Display
//!
//! Renders a single-line status bar at the bottom of the audit card.
//! Background color is keyed to [`StatusLevel`]; a Unicode icon prefix
//! from `umrs-core` icons reinforces the severity visually.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AU-3**: Status is always visible; level and text are
//!   typed, not free-form, so callers cannot accidentally omit either.
//! - **NSA RTB**: Security state is represented as typed enum variants,
//!   never as raw color codes or magic strings.

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use umrs_core::console::symbols::icons;

use crate::app::{AuditCardApp, StatusLevel};
use crate::theme::{Theme, status_bg_color};

// ---------------------------------------------------------------------------
// Render
// ---------------------------------------------------------------------------

/// Render the status bar from the app's current [`StatusMessage`].
///
/// The full width is filled with the level background color so the bar
/// reads as a solid block.
///
/// NIST SP 800-53 AU-3 — security state is always present and typed.
pub fn render_status_bar(
    frame: &mut Frame,
    area: Rect,
    app: &dyn AuditCardApp,
    theme: &Theme,
) {
    let status = app.status();
    let bg = status_bg_color(status.level);
    let icon = level_icon(status.level);

    let text = format!(" {icon} {} ", status.text);

    // Pad to full width so the background fills the bar.
    let total_width = area.width as usize;
    let text_chars = text.chars().count();
    let padded = if text_chars < total_width {
        let pad = total_width.saturating_sub(text_chars);
        format!("{text}{}", " ".repeat(pad))
    } else {
        text
    };

    let line = Line::from(vec![Span::styled(
        padded,
        Style::default().bg(bg).patch(theme.status_text),
    )]);

    let paragraph = Paragraph::new(line);
    frame.render_widget(paragraph, area);
}

// ---------------------------------------------------------------------------
// Icon helper
// ---------------------------------------------------------------------------

/// Return the icon glyph for a status level.
///
/// Uses the centralised `umrs_core::console::symbols::icons` glyphs so
/// all UMRS tools share the same visual vocabulary.
const fn level_icon(level: StatusLevel) -> &'static str {
    match level {
        StatusLevel::Info => icons::INFO,
        StatusLevel::Ok => icons::CHECK,
        StatusLevel::Warn => icons::WARNING,
        StatusLevel::Error => icons::CROSS,
    }
}
