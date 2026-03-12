// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams

//! # Data Panel — Scrollable Key-Value Data Area
//!
//! Renders a bordered, scrollable list of [`DataRow`] entries. Keys are
//! displayed in dim-cyan; values are colored per their [`StyleHint`].
//!
//! A ratatui `Scrollbar` is rendered on the right edge when the content
//! exceeds the visible area height.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AU-3**: Structured key-value rows ensure every field
//!   is labelled; there is no ambiguous free-form data blob.

use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::text::{Line, Span};
use ratatui::widgets::{
    Block, BorderType, Borders, Paragraph, Scrollbar, ScrollbarOrientation,
    ScrollbarState,
};

use crate::app::{AuditCardApp, DataRow};
use crate::theme::{Theme, style_hint_color};

// ---------------------------------------------------------------------------
// Key column width
// ---------------------------------------------------------------------------

/// Fixed width of the key column (characters), including trailing padding.
const KEY_COL_WIDTH: usize = 20;

// ---------------------------------------------------------------------------
// Render
// ---------------------------------------------------------------------------

/// Render the scrollable key-value data panel.
///
/// `scroll_offset` is the number of rows scrolled past the top. The function
/// clamps it to the valid range so the caller does not need to guard it.
///
/// NIST SP 800-53 AU-3 — every data field is labelled; no ambiguous blobs.
pub fn render_data_panel(
    frame: &mut Frame,
    area: Rect,
    app: &dyn AuditCardApp,
    active_tab: usize,
    scroll_offset: usize,
    theme: &Theme,
) {
    let rows = app.data_rows(active_tab);
    let total_rows = rows.len();

    // Inner height available for content rows (subtract 2 for borders).
    let inner_height = (area.height as usize).saturating_sub(2);

    // Clamp scroll offset so we never scroll past the last visible row.
    let max_offset = total_rows.saturating_sub(inner_height);
    let offset = scroll_offset.min(max_offset);

    // Build visible lines from the scrolled slice.
    let visible: Vec<Line<'_>> = rows
        .iter()
        .skip(offset)
        .take(inner_height)
        .map(|row| build_row_line(row, theme))
        .collect();

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(theme.border);

    let paragraph = Paragraph::new(visible).block(block);

    // Split area: leave 1-col gutter on the right for the scrollbar.
    if total_rows > inner_height {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(0), Constraint::Length(1)])
            .split(area);

        frame.render_widget(paragraph, chunks[0]);

        // Scrollbar
        let mut scrollbar_state = ScrollbarState::default()
            .content_length(total_rows)
            .viewport_content_length(inner_height)
            .position(offset);

        let scrollbar = Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight);

        frame.render_stateful_widget(
            scrollbar,
            chunks[1],
            &mut scrollbar_state,
        );
    } else {
        frame.render_widget(paragraph, area);
    }
}

// ---------------------------------------------------------------------------
// Row builder
// ---------------------------------------------------------------------------

/// Build a single [`Line`] from a [`DataRow`].
fn build_row_line<'a>(row: &'a DataRow, theme: &Theme) -> Line<'a> {
    if row.key.is_empty() && row.value.is_empty() {
        // Separator — blank line
        return Line::from("");
    }

    let key_padded = pad_key(&row.key, KEY_COL_WIDTH);
    let value_color = style_hint_color(row.style_hint);

    Line::from(vec![
        Span::styled(key_padded, theme.data_key),
        Span::styled(row.value.clone(), theme.data_value.fg(value_color)),
    ])
}

/// Right-pad a key string to `width` characters (truncates if too long).
fn pad_key(key: &str, width: usize) -> String {
    let char_count = key.chars().count();
    if char_count >= width {
        // Truncate and add separator
        let truncated: String =
            key.chars().take(width.saturating_sub(2)).collect();
        format!("{truncated}: ")
    } else {
        let pad = width.saturating_sub(char_count).saturating_sub(2);
        format!("{key}: {}", " ".repeat(pad))
    }
}
