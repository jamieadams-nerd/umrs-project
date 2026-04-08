// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams

//! # Pill Badge — Rounded terminal badge for titles and labels
//!
//! Renders a pill-shaped badge using ⬤ (U+2B24 BLACK LARGE CIRCLE) as
//! rounded end caps. The left cap renders the circle glyph in the pill
//! background color on the terminal default background, creating a rounded
//! edge. The right cap renders the circle on matching background, creating
//! a clean termination.
//!
//! ## Key Exports
//!
//! - [`render_pill`] — render a pill badge into a ratatui `Frame` area
//! - [`pill_line`] — build a pill `Line` for embedding in other layouts
//!
//! ## Compliance
//!
//! This module provides internal formatting utility infrastructure with no
//! direct security surface.

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

/// Left/right end cap glyph — ⬤ BLACK LARGE CIRCLE (U+2B24).
const PILL_CAP: &str = "\u{2B24}";

/// Build a pill-shaped `Line` with the given text, colors, and padding.
///
/// - `text`: the label to display inside the pill
/// - `fg`: foreground (text) color
/// - `bg`: background (pill body) color
/// - `padding`: number of spaces on each side of the text inside the pill
///
/// The returned `Line` can be rendered directly or composed into a larger layout.
#[must_use = "pill_line builds a Line for rendering; discarding it has no effect"]
pub fn pill_line(text: &str, fg: Color, bg: Color, padding: usize) -> Line<'static> {
    let pad = " ".repeat(padding);
    let body = format!("{pad}{text}{pad}");
    Line::from(vec![
        Span::styled(PILL_CAP, Style::default().fg(bg)),
        Span::styled(body, Style::default().fg(fg).bg(bg).add_modifier(Modifier::BOLD)),
        Span::styled(PILL_CAP, Style::default().fg(bg).bg(bg)),
    ])
}

/// Render a centered pill badge into `area`.
///
/// Computes left padding to center the pill horizontally within the area.
/// If the pill is wider than the area, it renders left-aligned (no overflow).
///
/// - `text`: the label to display
/// - `fg`: foreground (text) color
/// - `bg`: background (pill body) color
/// - `padding`: spaces on each side of the text inside the pill
pub fn render_pill(
    frame: &mut Frame,
    area: Rect,
    text: &str,
    fg: Color,
    bg: Color,
    padding: usize,
) {
    let pad_str = " ".repeat(padding);
    let body = format!("{pad_str}{text}{pad_str}");
    // pill width = left cap (1) + body + right cap (1)
    // Note: ⬤ may render as 1 or 2 columns depending on terminal.
    // Use 1 as the common case.
    let pill_width = 1 + body.len() + 1;
    let left_pad = (area.width as usize).saturating_sub(pill_width) / 2;

    let line = Line::from(vec![
        Span::raw(" ".repeat(left_pad)),
        Span::styled(PILL_CAP, Style::default().fg(bg)),
        Span::styled(body, Style::default().fg(fg).bg(bg).add_modifier(Modifier::BOLD)),
        Span::styled(PILL_CAP, Style::default().fg(bg).bg(bg)),
    ]);

    frame.render_widget(Paragraph::new(vec![line]), area);
}
