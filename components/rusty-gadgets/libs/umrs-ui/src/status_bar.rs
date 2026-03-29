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

/// Key legend shown on the right side of the status bar.
///
/// Compact single-line reference for the key bindings most commonly needed
/// by a new operator. Updated here when new bindings are added.
///
/// Must be short enough to coexist with a typical status message on an
/// 80-column terminal (<=40 characters including leading separator).
const KEY_LEGEND: &str = "  Tab: tabs | ↑↓/jk/PgDn: scroll | ?: help | q: quit";

/// Render the status bar from the app's current [`StatusMessage`].
///
/// The full width is filled with the level background color so the bar
/// reads as a solid block. A compact key legend is right-aligned on the
/// same line so new operators can discover available actions at a glance.
///
/// The legend is elided if the terminal is too narrow to display both the
/// status message and the legend without overlap (under ~80 columns).
///
/// NIST SP 800-53 AU-3 — security state is always present and typed.
/// NIST SP 800-53 SA-5 — inline key legend reduces reliance on external
/// documentation during an assessment.
pub fn render_status_bar(
    frame: &mut Frame,
    area: Rect,
    app: &dyn AuditCardApp,
    theme: &Theme,
) {
    let status = app.status();
    let bg = status_bg_color(status.level);
    let icon = level_icon(status.level);

    let status_text = format!(" {icon} {} ", status.text);
    let total_width = area.width as usize;

    // Attempt to right-align the key legend. If the status message and legend
    // together exceed the available width, the legend is omitted entirely so
    // the status message is never truncated.
    let legend_chars = KEY_LEGEND.chars().count();
    let status_chars = status_text.chars().count();

    let combined = status_chars.saturating_add(legend_chars);

    let padded = if combined <= total_width {
        // There is room for both: pad the status to push the legend right.
        let pad = total_width
            .saturating_sub(status_chars)
            .saturating_sub(legend_chars);
        format!("{status_text}{}{KEY_LEGEND}", " ".repeat(pad))
    } else if status_chars < total_width {
        // No room for the legend — pad the status to fill the bar.
        let pad = total_width.saturating_sub(status_chars);
        format!("{status_text}{}", " ".repeat(pad))
    } else {
        // Status itself is too long — truncate it to the available width.
        status_text.chars().take(total_width).collect()
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
