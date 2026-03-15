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
// Row builders
// ---------------------------------------------------------------------------

/// Half the standard key column width, used for each side of a `TwoColumn` row.
const HALF_KEY_COL_WIDTH: usize = KEY_COL_WIDTH / 2;

// ---------------------------------------------------------------------------
// Evidence table column widths
// ---------------------------------------------------------------------------

/// Fixed width of the `Evidence Type` column in a `TableRow` / `TableHeader`.
const TABLE_COL1_WIDTH: usize = 20;

/// Fixed width of the `Source` column in a `TableRow` / `TableHeader`.
const TABLE_COL2_WIDTH: usize = 24;

/// Build a single [`Line`] from a [`DataRow`].
///
/// Each variant produces a line that fits within the panel's text area:
/// - `KeyValue` — key padded to `KEY_COL_WIDTH`, then value.
/// - `TwoColumn` — left half (key+value) padded to fill half the panel,
///   right half (key+value) following. Each key uses `HALF_KEY_COL_WIDTH`.
/// - `GroupTitle` — title string rendered flush-left using `theme.group_title`
///   (bold white). Takes one display row. No border or ASCII decoration.
/// - `Separator` — blank line.
fn build_row_line<'a>(row: &'a DataRow, theme: &'a Theme) -> Line<'a> {
    match row {
        DataRow::KeyValue {
            key,
            value,
            style_hint,
        } => {
            let key_padded = pad_key(key, KEY_COL_WIDTH);
            let value_color = style_hint_color(*style_hint);
            Line::from(vec![
                Span::styled(key_padded, theme.data_key),
                Span::styled(value.clone(), theme.data_value.fg(value_color)),
            ])
        }

        DataRow::TwoColumn {
            left_key,
            left_value,
            left_hint,
            right_key,
            right_value,
            right_hint,
        } => {
            // Left half: key (half width) + value padded to fill the left side.
            // Right half: key (half width) + value.
            //
            // The total width of each half is KEY_COL_WIDTH (left key + left
            // value) and KEY_COL_WIDTH (right key + right value). We pad the
            // left value so the right column starts consistently.
            let left_key_str = pad_key(left_key, HALF_KEY_COL_WIDTH);
            let left_val_color = style_hint_color(*left_hint);
            let right_key_str = pad_key(right_key, HALF_KEY_COL_WIDTH);
            let right_val_color = style_hint_color(*right_hint);

            // Pad the left value to fill the remaining left-column budget so
            // the right column is aligned. Total left-column chars = KEY_COL_WIDTH.
            let left_budget = KEY_COL_WIDTH.saturating_sub(HALF_KEY_COL_WIDTH);
            let left_val_padded = pad_value(left_value, left_budget);

            Line::from(vec![
                Span::styled(left_key_str, theme.data_key),
                Span::styled(
                    left_val_padded,
                    theme.data_value.fg(left_val_color),
                ),
                Span::styled(right_key_str, theme.data_key),
                Span::styled(
                    right_value.clone(),
                    theme.data_value.fg(right_val_color),
                ),
            ])
        }

        DataRow::GroupTitle(title) => {
            // Flush-left, single styled span. No padding — the group title
            // fills the full line width naturally. Indentation of rows under
            // this title is the caller's responsibility (see DataRow docs).
            Line::from(Span::styled(title.clone(), theme.group_title))
        }

        DataRow::Separator => Line::from(""),

        DataRow::TableRow {
            col1,
            col2,
            col3,
            style_hint,
        } => {
            // col1: evidence type — clipped to TABLE_COL1_WIDTH, then padded.
            let col1_str = clip_pad(col1, TABLE_COL1_WIDTH);
            // col2: source path — clipped to TABLE_COL2_WIDTH, then padded.
            let col2_str = clip_pad(col2, TABLE_COL2_WIDTH);
            // col3: verification outcome — rendered with the style hint color;
            // remainder of the line so no fixed width needed.
            let col3_color = style_hint_color(*style_hint);
            Line::from(vec![
                Span::styled(col1_str, theme.data_key),
                Span::styled(col2_str, theme.data_value),
                Span::styled(col3.clone(), theme.data_value.fg(col3_color)),
            ])
        }

        DataRow::TableHeader {
            col1,
            col2,
            col3,
        } => {
            // All three columns rendered with the bold key style to signal
            // that this is a header, not a data row.
            let col1_str = clip_pad(col1, TABLE_COL1_WIDTH);
            let col2_str = clip_pad(col2, TABLE_COL2_WIDTH);
            Line::from(vec![
                Span::styled(col1_str, theme.data_key),
                Span::styled(col2_str, theme.data_key),
                Span::styled(col3.clone(), theme.data_key),
            ])
        }
    }
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

/// Pad (or truncate) a value string to exactly `width` characters so that
/// the right column of a `TwoColumn` row starts at a consistent position.
fn pad_value(value: &str, width: usize) -> String {
    let char_count = value.chars().count();
    if char_count >= width {
        value.chars().take(width).collect()
    } else {
        let pad = width.saturating_sub(char_count);
        format!("{value}{}", " ".repeat(pad))
    }
}

/// Clip a string to `width` characters and right-pad to exactly `width`.
///
/// Used for `TableRow` and `TableHeader` columns where overflow would push
/// subsequent columns out of alignment. The string is clipped to `width`
/// characters (not bytes) and then space-padded to exactly `width` chars.
///
/// A string already shorter than `width` is padded; a string that equals
/// or exceeds `width` is truncated to exactly `width` characters. This
/// guarantees a fixed-width field for column alignment without overflow.
fn clip_pad(value: &str, width: usize) -> String {
    let char_count = value.chars().count();
    if char_count >= width {
        value.chars().take(width).collect()
    } else {
        let pad = width.saturating_sub(char_count);
        format!("{value}{}", " ".repeat(pad))
    }
}
