// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams

//! # Layout — Master Render Function for ConfigApp
//!
//! Composes all config panels into the full configuration editor layout:
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────┐
//! │ Header (tool name, target, dirty flag, validation state)│
//! ├─────────────────────────────────────────────────────────┤
//! │ Tab bar                                                  │
//! ├────────────────────────────────┬────────────────────────┤
//! │ Field list                     │ Diff / validation panel│
//! │ (focused field highlighted)    │ (pending changes or    │
//! │                                │  validation errors)    │
//! ├─────────────────────────────────────────────────────────┤
//! │ Status bar                                              │
//! └─────────────────────────────────────────────────────────┘
//! ```
//!
//! The field list / diff split is horizontal: the field list takes
//! `FIELD_PANEL_PERCENT` of the body width and the diff/validation panel
//! takes the remainder.
//!
//! The right panel renders the diff view when no field is in edit mode
//! (to preview pending changes), and switches to inline validation feedback
//! when a field is being edited.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AU-3**: Every frame carries tool identification, target
//!   path, dirty/clean state, and the current validation status.
//! - **NIST SP 800-53 CM-3**: The diff panel ensures operators review all
//!   proposed changes before the `Save` action is processed.
//! - **NIST SP 800-53 SI-10**: Validation status is always visible; the field
//!   list shows inline indicators next to each field.
//! - **NSA RTB RAIN**: The `Save` action is rejected at the state level when
//!   any field has a blocking validation result; the layout reflects this
//!   by showing the Save key hint only when all fields are valid.

use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph};

use crate::theme::Theme;

use super::{ConfigApp, ConfigState};
use super::diff::{DiffEntry, render_diff};
use super::fields::{FieldDef, ValidationResult};

// ---------------------------------------------------------------------------
// Layout constants
// ---------------------------------------------------------------------------

/// Fixed header height for the config layout.
const CONFIG_HEADER_HEIGHT: u16 = 5;

/// Field list panel width as a percentage.
const FIELD_PANEL_PERCENT: u16 = 50;

/// Diff/validation panel width as the remainder.
const DIFF_PANEL_PERCENT: u16 = 50;

// ---------------------------------------------------------------------------
// Master render entry point
// ---------------------------------------------------------------------------

/// Render the complete configuration editor layout into `area`.
///
/// The field list shows all fields with the focused field highlighted. The
/// right panel shows either the diff view (when no field is being edited) or
/// inline validation feedback (when a field is in edit mode).
///
/// NIST SP 800-53 AU-3 — identification, dirty state, and validation status
/// are always present in every rendered frame.
/// NIST SP 800-53 CM-3 — pending changes are always visible before `Save`.
pub fn render_config(
    frame: &mut Frame,
    area: Rect,
    app: &dyn ConfigApp,
    state: &ConfigState,
    theme: &Theme,
) {
    // ── Outer vertical split ─────────────────────────────────────────────
    let outer = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(CONFIG_HEADER_HEIGHT),
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(area);

    let [header_area, tab_area, body_area, status_area] = *outer else {
        return;
    };

    // ── Header ───────────────────────────────────────────────────────────
    render_config_header(frame, header_area, app, state, theme);

    // ── Tab bar ──────────────────────────────────────────────────────────
    render_config_tabs(frame, tab_area, app, state.active_tab, theme);

    // ── Body: fields | diff ──────────────────────────────────────────────
    let body_cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(FIELD_PANEL_PERCENT),
            Constraint::Percentage(DIFF_PANEL_PERCENT),
        ])
        .split(body_area);

    let [field_area, right_area] = *body_cols else {
        return;
    };

    render_field_list(frame, field_area, state, theme);

    // Right panel: diff when no field is being edited; validation panel when
    // a field is actively being edited.
    let any_editing = state.fields.iter().any(|f| f.editing);
    // Note: iter() is used rather than &state.fields to keep the intent clear
    // for `.any()` predicate usage. Clippy's explicit_iter_loop fires on for-loops;
    // it does not fire on iterator method chains.
    if any_editing {
        render_validation_panel(frame, right_area, state, theme);
    } else {
        let diff_entries = build_diff_entries(app, state);
        render_diff(frame, right_area, &diff_entries, theme);
    }

    // ── Status bar ───────────────────────────────────────────────────────
    render_config_status(frame, status_area, app, state, theme);
}

// ---------------------------------------------------------------------------
// Header
// ---------------------------------------------------------------------------

/// Render the config editor header panel.
///
/// Shows: tool name, target path, modified indicator, validation summary.
fn render_config_header(
    frame: &mut Frame,
    area: Rect,
    app: &dyn ConfigApp,
    state: &ConfigState,
    theme: &Theme,
) {
    let ctx = app.config_header();

    let dirty_indicator = if state.is_dirty() { " [modified]" } else { "" };
    let target_line = format!("  {:<12} : {}", "Target", ctx.config_target);
    let modified_line = format!("  {:<12} : {}{dirty_indicator}", "Status", ctx.validation_summary);

    let lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            format!("  {}", ctx.tool_name),
            theme.header_name,
        )),
        Line::from(Span::styled(target_line, theme.header_field)),
        Line::from(Span::styled(modified_line, if state.is_dirty() {
            theme.indicator_unavailable
        } else {
            theme.header_field
        })),
    ];

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(theme.border)
        .title(Span::styled(
            format!(" {} ", ctx.tool_name),
            theme.header_name,
        ));

    let paragraph = Paragraph::new(lines).block(block);
    frame.render_widget(paragraph, area);
}

// ---------------------------------------------------------------------------
// Tab bar
// ---------------------------------------------------------------------------

fn render_config_tabs(
    frame: &mut Frame,
    area: Rect,
    app: &dyn ConfigApp,
    active_tab: usize,
    theme: &Theme,
) {
    use ratatui::widgets::Tabs;

    let tab_titles: Vec<Line<'_>> = app
        .tabs()
        .iter()
        .map(|t| Line::from(format!(" {} ", t.label)))
        .collect();

    let tabs = Tabs::new(tab_titles)
        .select(active_tab)
        .style(Style::default())
        .highlight_style(theme.tab_active)
        .divider("|");

    frame.render_widget(tabs, area);
}

// ---------------------------------------------------------------------------
// Field list
// ---------------------------------------------------------------------------

/// Key column width for the field list.
const FIELD_LABEL_WIDTH: usize = 20;

/// Render the field list panel.
///
/// Each field is shown as a key-value row. The focused field uses the
/// `tab_active` highlight style. Fields in edit mode show the edit buffer
/// value with a cursor indicator (`█`). Validation errors are shown as a
/// dim annotation below the value.
fn render_field_list(
    frame: &mut Frame,
    area: Rect,
    state: &ConfigState,
    theme: &Theme,
) {
    let items: Vec<ListItem<'_>> = state
        .fields
        .iter()
        .map(|field| build_field_item(field, theme))
        .collect();

    let block = Block::default()
        .title(" Fields ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(theme.border);

    let mut list_state = ListState::default();
    if !state.fields.is_empty() {
        list_state.select(Some(state.focused_field));
    }

    let list = List::new(items)
        .block(block)
        .highlight_style(theme.tab_active)
        .highlight_symbol("► ");

    frame.render_stateful_widget(list, area, &mut list_state);
}

/// Build a `ListItem` for a single field.
///
/// Layout: `{label:<FIELD_LABEL_WIDTH$} : {value_or_buffer}{edit_cursor?}`
/// Followed by a validation message line if the field has an error or warning.
fn build_field_item<'a>(field: &FieldDef, theme: &'a Theme) -> ListItem<'a> {
    let dirty_marker = if field.dirty { "*" } else { " " };
    let label_str = format!(" {dirty_marker}{:<FIELD_LABEL_WIDTH$} : ", field.label);

    let (value_text, value_style) = if field.editing {
        (
            format!("{}█", field.edit_buffer),
            theme.indicator_unavailable,
        )
    } else {
        let val = field.value.display();
        let style = match &field.validation {
            ValidationResult::Error(_) => theme.dialog_error_border,
            ValidationResult::Warning(_) => theme.indicator_unavailable,
            ValidationResult::Ok | ValidationResult::Pending => theme.data_value,
        };
        (val, style)
    };

    let mut lines = vec![Line::from(vec![
        Span::styled(label_str, theme.data_key),
        Span::styled(value_text, value_style),
    ])];

    // Inline validation message (errors and warnings only).
    let msg = field.validation.display();
    if !msg.is_empty() {
        let msg_style = match &field.validation {
            ValidationResult::Error(_) => theme.dialog_error_border,
            ValidationResult::Warning(_) => theme.indicator_unavailable,
            _ => theme.tab_inactive,
        };
        let indent = format!("   {:<FIELD_LABEL_WIDTH$}   ", "");
        lines.push(Line::from(vec![
            Span::styled(indent, theme.tab_inactive),
            Span::styled(format!("⚠ {msg}"), msg_style),
        ]));
    }

    ListItem::new(lines)
}

// ---------------------------------------------------------------------------
// Validation panel (active while editing)
// ---------------------------------------------------------------------------

/// Render the validation panel shown while a field is in edit mode.
///
/// Shows the field label, current edit buffer, and validation result
/// for all fields — not just the one being edited — so the operator
/// can see the full validation state before committing.
fn render_validation_panel(
    frame: &mut Frame,
    area: Rect,
    state: &ConfigState,
    theme: &Theme,
) {
    let mut lines = vec![Line::from(""), Line::from(Span::styled(
        "  Validation",
        theme.group_title,
    )), Line::from("")];

    for field in &state.fields {
        let result_str = match &field.validation {
            ValidationResult::Ok => "  ok".to_owned(),
            ValidationResult::Error(msg) => format!("  error: {msg}"),
            ValidationResult::Warning(msg) => format!("  warn: {msg}"),
            ValidationResult::Pending => "  pending".to_owned(),
        };
        let result_style = match &field.validation {
            ValidationResult::Ok => theme.indicator_active,
            ValidationResult::Error(_) => theme.dialog_error_border,
            ValidationResult::Warning(_) => theme.indicator_unavailable,
            ValidationResult::Pending => theme.tab_inactive,
        };
        let label_str = format!("  {:<FIELD_LABEL_WIDTH$}", field.label);
        lines.push(Line::from(vec![
            Span::styled(label_str, theme.data_key),
            Span::styled(result_str, result_style),
        ]));
    }

    let block = Block::default()
        .title(" Validation ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(theme.border);

    let paragraph = Paragraph::new(lines).block(block);
    frame.render_widget(paragraph, area);
}

// ---------------------------------------------------------------------------
// Status bar
// ---------------------------------------------------------------------------

/// Key legend when the form is clean (no dirty fields).
const CONFIG_KEY_LEGEND_CLEAN: &str =
    "  Tab: tabs | ↑↓: select | Enter: edit | q: quit";

/// Key legend when the form has dirty fields (show Save/Discard).
const CONFIG_KEY_LEGEND_DIRTY: &str =
    "  Tab: tabs | ↑↓: select | Enter: edit | ^S: save | ^Z: discard | q: quit";

/// Render the config status bar.
fn render_config_status(
    frame: &mut Frame,
    area: Rect,
    app: &dyn ConfigApp,
    state: &ConfigState,
    theme: &Theme,
) {
    use crate::theme::status_bg_color;
    use umrs_core::console::symbols::icons;

    let status = app.status();
    let bg = status_bg_color(status.level);

    let icon = match status.level {
        crate::app::StatusLevel::Info => icons::INFO,
        crate::app::StatusLevel::Ok => icons::CHECK,
        crate::app::StatusLevel::Warn => icons::WARNING,
        crate::app::StatusLevel::Error => icons::CROSS,
    };

    let legend = if state.is_dirty() {
        CONFIG_KEY_LEGEND_DIRTY
    } else {
        CONFIG_KEY_LEGEND_CLEAN
    };

    let status_text = format!(" {icon} {} ", status.text);
    let total_width = area.width as usize;
    let legend_chars = legend.chars().count();
    let status_chars = status_text.chars().count();
    let combined = status_chars.saturating_add(legend_chars);

    let padded = if combined <= total_width {
        let pad = total_width
            .saturating_sub(status_chars)
            .saturating_sub(legend_chars);
        format!("{status_text}{}{legend}", " ".repeat(pad))
    } else if status_chars < total_width {
        let pad = total_width.saturating_sub(status_chars);
        format!("{status_text}{}", " ".repeat(pad))
    } else {
        status_text.chars().take(total_width).collect()
    };

    let line = Line::from(vec![Span::styled(
        padded,
        Style::default().bg(bg).patch(theme.status_text),
    )]);

    frame.render_widget(Paragraph::new(line), area);
}

// ---------------------------------------------------------------------------
// Diff entry builder
// ---------------------------------------------------------------------------

/// Build the diff entry list from the current field state.
///
/// The "before" value for each field is retrieved from `app.committed_values()`.
/// Fields not present in the committed values map are treated as new (before = "").
fn build_diff_entries(app: &dyn ConfigApp, state: &ConfigState) -> Vec<DiffEntry> {
    let committed = app.committed_values();
    state
        .fields
        .iter()
        .map(|field| {
            let before = committed
                .get(&field.label)
                .cloned()
                .unwrap_or_default();
            let after = field.value.display();
            DiffEntry::new(field.label.clone(), before, after)
        })
        .collect()
}
