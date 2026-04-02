// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams

//! # Diff — Configuration Change Diff View
//!
//! Renders a before/after diff panel showing what configuration values have
//! changed since the last save. Used by [`super::layout::render_config`] when
//! the caller has dirty fields to review before committing.
//!
//! ## Design
//!
//! The diff view is a simple two-column table: field label, old value, new
//! value. Changed fields are highlighted in yellow; unchanged fields are shown
//! dimmed to provide context. The view is read-only.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 CM-3**: Configuration change control — the diff view
//!   is the operator's last opportunity to review all proposed changes before
//!   committing. Every changed field is explicitly shown.
//! - **NIST SP 800-53 AU-3**: Audit record content — the diff provides the
//!   before/after values that must appear in the change audit record.
//! - **NIST SP 800-53 SI-11**: Error information — field values in the diff
//!   must not contain classified data or key material. Callers are responsible
//!   for masking sensitive values before populating `DiffEntry`.

// ---------------------------------------------------------------------------
// DiffEntry
// ---------------------------------------------------------------------------

/// A single before/after comparison entry for one configuration field.
///
/// NIST SP 800-53 CM-3 — each entry carries the field identity, the
/// previous committed value, and the proposed new value.
/// NIST SP 800-53 SI-11 — callers must mask any value that contains
/// classified data or key material before constructing a `DiffEntry`.
#[derive(Debug, Clone)]
pub struct DiffEntry {
    /// Field label (from `FieldDef::label`).
    pub label: String,

    /// The last committed value (before this editing session).
    pub before: String,

    /// The proposed new value (current `FieldDef::value.display()`).
    pub after: String,
}

impl DiffEntry {
    /// Construct a `DiffEntry`.
    #[must_use = "DiffEntry must be stored in the diff list; constructing and discarding it has no effect"]
    pub fn new(
        label: impl Into<String>,
        before: impl Into<String>,
        after: impl Into<String>,
    ) -> Self {
        Self {
            label: label.into(),
            before: before.into(),
            after: after.into(),
        }
    }

    /// Return `true` if the before and after values differ.
    #[must_use = "changed status determines rendering style; discarding it hides diff information"]
    pub fn is_changed(&self) -> bool {
        self.before != self.after
    }
}

// ---------------------------------------------------------------------------
// render_diff
// ---------------------------------------------------------------------------

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};

use crate::theme::Theme;

/// Column widths for the diff table.
const DIFF_LABEL_WIDTH: usize = 20;
const DIFF_VALUE_WIDTH: usize = 24;

/// Render the configuration diff panel.
///
/// Shows a two-section table: unchanged fields (dimmed) followed by
/// changed fields (highlighted). If `entries` is empty, a placeholder
/// message is shown.
///
/// NIST SP 800-53 CM-3 — all proposed changes are surfaced for explicit
/// operator review before the `Save` action is processed.
/// NIST SP 800-53 AU-3 — before/after values are always paired; the operator
/// cannot see an "after" without also seeing the "before".
pub fn render_diff(frame: &mut Frame, area: Rect, entries: &[DiffEntry], theme: &Theme) {
    let lines = if entries.is_empty() {
        vec![Line::from(""), Line::from(Span::styled("  No changes to review.", theme.data_value))]
    } else {
        build_diff_lines(entries, theme)
    };

    let block = Block::default()
        .title(" Pending Changes ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(theme.border);

    let paragraph = Paragraph::new(lines).block(block);
    frame.render_widget(paragraph, area);
}

/// Build diff table lines.
///
/// Renders the column header, then unchanged entries (dimmed), then changed
/// entries (highlighted).
fn build_diff_lines<'a>(entries: &[DiffEntry], theme: &'a Theme) -> Vec<Line<'a>> {
    let mut lines: Vec<Line<'a>> = Vec::new();

    // Column header.
    lines.push(Line::from(""));
    lines.push(header_line(theme));
    lines.push(Line::from(""));

    // Unchanged entries (context, dimmed).
    let unchanged: Vec<&DiffEntry> = entries.iter().filter(|e| !e.is_changed()).collect();
    if !unchanged.is_empty() {
        for entry in &unchanged {
            lines.push(diff_row_line(entry, false, theme));
        }
        lines.push(Line::from(""));
    }

    // Changed entries (highlighted).
    let changed: Vec<&DiffEntry> = entries.iter().filter(|e| e.is_changed()).collect();
    if !changed.is_empty() {
        lines.push(Line::from(Span::styled("  Changed", theme.group_title)));
        lines.push(Line::from(""));
        for entry in &changed {
            lines.push(diff_row_line(entry, true, theme));
        }
    }

    lines
}

/// Build the column header line.
fn header_line(theme: &Theme) -> Line<'_> {
    let label_col = format!("  {:<DIFF_LABEL_WIDTH$}", "Field");
    let before_col = format!("  {:<DIFF_VALUE_WIDTH$}", "Before");
    let after_col = "  After".to_owned();
    Line::from(vec![
        Span::styled(label_col, theme.group_title),
        Span::styled(before_col, theme.group_title),
        Span::styled(after_col, theme.group_title),
    ])
}

/// Build a single diff row line.
///
/// `highlighted` = `true` for changed entries (uses `indicator_active` /
/// yellow), `false` for unchanged context (uses `tab_inactive` dim style).
fn diff_row_line<'a>(entry: &DiffEntry, highlighted: bool, theme: &'a Theme) -> Line<'a> {
    let label_str = format!("  {:<DIFF_LABEL_WIDTH$}", entry.label);
    let before_str = format!("  {:<DIFF_VALUE_WIDTH$}", entry.before);
    let after_str = format!("  {}", entry.after);

    if highlighted {
        Line::from(vec![
            Span::styled(label_str, theme.data_key),
            Span::styled(before_str, theme.indicator_inactive),
            Span::styled(after_str, theme.indicator_unavailable),
        ])
    } else {
        Line::from(vec![
            Span::styled(label_str, theme.tab_inactive),
            Span::styled(before_str, theme.tab_inactive),
            Span::styled(after_str, theme.tab_inactive),
        ])
    }
}
