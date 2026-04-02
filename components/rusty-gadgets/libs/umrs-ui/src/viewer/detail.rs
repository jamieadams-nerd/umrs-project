// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams

//! # Detail — Detail Panel Rendering for ViewerApp
//!
//! Renders the right-hand detail panel that shows the metadata fields of the
//! currently selected [`super::tree::TreeNode`].
//!
//! ## Layout
//!
//! The detail panel is a bordered `Paragraph` widget. Each metadata entry
//! is rendered as a key-value row using the same dim-cyan key / white value
//! styling as the audit card data panel.
//!
//! When no node is selected, or the selected node has no metadata, a
//! placeholder message is shown so the panel never appears empty.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AU-3**: The detail panel presents node identification
//!   and associated metadata without truncation; every field that was
//!   provided by the data source is rendered.
//! - **NSA RTB**: No dynamic allocation beyond what ratatui requires for
//!   `Line` construction; the detail data is read-only and display-only.

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph, Wrap};

use crate::theme::Theme;

use super::tree::TreeNode;

// ---------------------------------------------------------------------------
// Label formatting constant
// ---------------------------------------------------------------------------

/// Width of the key label column in detail rows.
///
/// Matches the `LABEL_WIDTH` used in the audit card header to give a
/// consistent visual language across all UMRS TUI panels.
const DETAIL_KEY_WIDTH: usize = 18;

// ---------------------------------------------------------------------------
// Render
// ---------------------------------------------------------------------------

/// Render the detail panel for the given node.
///
/// If `selected_node` is `None` (nothing selected, or the display list is
/// empty), a placeholder line is shown. Otherwise, the node's `label`,
/// `detail`, and all `metadata` entries are rendered.
///
/// The panel uses `Wrap::Word` so long metadata values wrap within the
/// available width rather than overflowing off-screen.
///
/// NIST SP 800-53 AU-3 — every metadata field is rendered without
/// omission or truncation.
pub fn render_detail(
    frame: &mut Frame,
    area: Rect,
    selected_node: Option<&TreeNode>,
    theme: &Theme,
) {
    let lines = match selected_node {
        None => {
            vec![Line::from(""), Line::from(Span::styled("  No item selected.", theme.data_value))]
        }
        Some(node) => build_detail_lines(node, theme),
    };

    let block = Block::default()
        .title(" Detail ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(theme.border);

    let paragraph = Paragraph::new(lines).block(block).wrap(Wrap {
        trim: false,
    });

    frame.render_widget(paragraph, area);
}

// ---------------------------------------------------------------------------
// Line builder
// ---------------------------------------------------------------------------

/// Build the detail panel lines for a selected node.
///
/// Always includes the `Label` and `Detail` rows from the node's own fields.
/// Appends all `metadata` entries in key-sorted order. A blank line
/// separates the node identity rows from the metadata section.
fn build_detail_lines<'a>(node: &TreeNode, theme: &'a Theme) -> Vec<Line<'a>> {
    let mut lines: Vec<Line<'a>> = Vec::new();

    // Blank leading line for visual padding.
    lines.push(Line::from(""));

    // Node label row.
    lines.push(kv_line("Label", &node.label, theme));

    // Node detail row — only if non-empty.
    if !node.detail.is_empty() {
        lines.push(kv_line("Detail", &node.detail, theme));
    }

    // Blank separator before metadata section.
    if !node.metadata.is_empty() {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled("  Metadata", theme.group_title)));
        lines.push(Line::from(""));

        for (key, value) in &node.metadata {
            lines.push(kv_line(key, value, theme));
        }
    }

    lines
}

/// Build a single key-value line for the detail panel.
fn kv_line<'a>(key: &str, value: &str, theme: &'a Theme) -> Line<'a> {
    let key_str = format!("  {key:<DETAIL_KEY_WIDTH$} : ");
    Line::from(vec![
        Span::styled(key_str, theme.data_key),
        Span::styled(value.to_owned(), theme.data_value),
    ])
}
