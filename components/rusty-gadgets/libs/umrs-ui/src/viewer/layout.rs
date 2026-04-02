// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams

//! # Layout — Master Render Function for ViewerApp
//!
//! Composes all viewer panels into the full viewer layout:
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────┐
//! │ Header (tool name, data source, record count)           │
//! ├─────────────────────────────────────────────────────────┤
//! │ Tab bar                                                  │
//! ├────────────────────────────────┬────────────────────────┤
//! │ Tree panel                     │ Detail panel           │
//! │ (hierarchical node list)       │ (selected node fields) │
//! │                                │                        │
//! ├─────────────────────────────────────────────────────────┤
//! │ Search bar (collapsed when inactive)                    │
//! ├─────────────────────────────────────────────────────────┤
//! │ Status bar                                              │
//! └─────────────────────────────────────────────────────────┘
//! ```
//!
//! The tree/detail split is horizontal: the tree panel takes
//! `TREE_PANEL_RATIO` percent of the body width and the detail panel
//! takes the remainder. The search bar collapses to 0 rows when
//! `ViewerState::search_active` is `false`.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AU-3**: Every frame carries tool identification
//!   (header), navigation context (tabs, breadcrumb), and the full detail
//!   of the selected record (detail panel).
//! - **NSA RTB**: Tree width ratio is a compile-time constant; no runtime
//!   parameter can change the layout proportions.

use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph};

use crate::theme::Theme;

use super::ViewerApp;
use super::ViewerState;
use super::detail::render_detail;
use super::tree::DisplayEntry;

// ---------------------------------------------------------------------------
// Layout constants
// ---------------------------------------------------------------------------

/// Fixed header height (rows): title + source + count + breadcrumb + borders.
const VIEWER_HEADER_HEIGHT: u16 = 6;

/// Tree panel width as a percentage of the available body width.
const TREE_PANEL_PERCENT: u16 = 40;

/// Detail panel width as the remainder after the tree panel.
const DETAIL_PANEL_PERCENT: u16 = 60;

/// Height of the search bar when active (1 content row + 0 border rows —
/// the bar is rendered as a plain `Paragraph` with a visible border block).
const SEARCH_BAR_HEIGHT: u16 = 3;

// ---------------------------------------------------------------------------
// Master render entry point
// ---------------------------------------------------------------------------

/// Render the complete viewer layout into `area`.
///
/// Call this inside `terminal.draw(|f| render_viewer(f, f.area(), ...))`.
///
/// The tree column shows the flat display list from `state.tree`. The
/// detail column shows the metadata of the currently selected node. The
/// search bar appears at the bottom of the body area when
/// `state.search_active` is `true`.
///
/// Callers must call `state.tree.rebuild_display()` after any topology or
/// expansion change before passing state to this function — the renderer
/// reads `display_list` directly and does not trigger rebuilds.
///
/// NIST SP 800-53 AU-3 — all identification fields (tool name, source,
/// record count, selected node detail) are present in every rendered frame.
pub fn render_viewer(
    frame: &mut Frame,
    area: Rect,
    app: &dyn ViewerApp,
    state: &ViewerState,
    theme: &Theme,
) {
    let search_height = if state.search_active {
        SEARCH_BAR_HEIGHT
    } else {
        0
    };

    // ── Outer vertical split ─────────────────────────────────────────────
    // [header, tab_bar, body, search_bar, status_bar]
    let outer = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(VIEWER_HEADER_HEIGHT),
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(search_height),
            Constraint::Length(1),
        ])
        .split(area);

    let [header_area, tab_area, body_area, search_area, status_area] = *outer else {
        return;
    };

    // ── Header ───────────────────────────────────────────────────────────
    render_viewer_header(frame, header_area, app, state, theme);

    // ── Tab bar ──────────────────────────────────────────────────────────
    render_viewer_tabs(frame, tab_area, app, state.active_tab, theme);

    // ── Body: tree | detail ──────────────────────────────────────────────
    let body_cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(TREE_PANEL_PERCENT),
            Constraint::Percentage(DETAIL_PANEL_PERCENT),
        ])
        .split(body_area);

    let [tree_area, detail_area] = *body_cols else {
        return;
    };

    render_tree_panel(frame, tree_area, state, theme);

    let selected_node = state
        .tree
        .display_list
        .get(state.selected_index)
        .and_then(|entry| state.tree.node_ref(&entry.path));

    render_detail(frame, detail_area, selected_node, theme);

    // ── Search bar ───────────────────────────────────────────────────────
    if state.search_active {
        render_search_bar(frame, search_area, &state.search_query, theme);
    }

    // ── Status bar ───────────────────────────────────────────────────────
    render_viewer_status(frame, status_area, app, theme);
}

// ---------------------------------------------------------------------------
// Header
// ---------------------------------------------------------------------------

/// Render the viewer header panel.
///
/// Displays: tool name (bold), data source, record count, and the current
/// breadcrumb trail. The breadcrumb uses ` > ` as a separator.
///
/// NIST SP 800-53 AU-3 — identification and context are always present.
fn render_viewer_header(
    frame: &mut Frame,
    area: Rect,
    app: &dyn ViewerApp,
    state: &ViewerState,
    theme: &Theme,
) {
    let ctx = app.viewer_header();
    let breadcrumb = state.breadcrumb_display();

    let source_line = format!("  {:<10} : {}", "Source", ctx.data_source);
    let count_line = if let Some(desc) = &ctx.summary_description {
        format!("  {:<10} : {}    {}", "Records", ctx.record_count, desc)
    } else {
        format!("  {:<10} : {}", "Records", ctx.record_count)
    };
    let breadcrumb_line = format!("  {:<10} : {}", "Location", breadcrumb);

    let lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            format!("  {}", ctx.tool_name),
            theme.header_name,
        )),
        Line::from(Span::styled(source_line, theme.header_field)),
        Line::from(Span::styled(count_line, theme.header_field)),
        Line::from(Span::styled(breadcrumb_line, theme.header_field)),
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

/// Render the tab bar for a viewer app.
///
/// Uses a plain `Tabs` widget driven by `ViewerApp::tabs()`.
fn render_viewer_tabs(
    frame: &mut Frame,
    area: Rect,
    app: &dyn ViewerApp,
    active_tab: usize,
    theme: &Theme,
) {
    use ratatui::widgets::Tabs;

    let tab_titles: Vec<Line<'_>> =
        app.tabs().iter().map(|t| Line::from(format!(" {} ", t.label))).collect();

    let tabs = Tabs::new(tab_titles)
        .select(active_tab)
        .style(Style::default())
        .highlight_style(theme.tab_active)
        .divider("|");

    frame.render_widget(tabs, area);
}

// ---------------------------------------------------------------------------
// Tree panel
// ---------------------------------------------------------------------------

/// Render the hierarchical tree panel.
///
/// The currently selected entry is highlighted using `theme.tab_active`.
/// Scroll is driven by `state.scroll_offset` — the renderer shows a window
/// of entries starting at `scroll_offset`.
fn render_tree_panel(frame: &mut Frame, area: Rect, state: &ViewerState, theme: &Theme) {
    let display = &state.tree.display_list;

    let items: Vec<ListItem<'_>> =
        display.iter().map(|entry| build_tree_item(entry, theme)).collect();

    let block = Block::default()
        .title(" Tree ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(theme.border);

    let mut list_state = ListState::default();
    if !display.is_empty() {
        list_state.select(Some(state.selected_index));
    }

    let list =
        List::new(items).block(block).highlight_style(theme.tab_active).highlight_symbol("► ");

    frame.render_stateful_widget(list, area, &mut list_state);
}

/// Build a single `ListItem` for a tree display entry.
///
/// The row format is: `{prefix}{label}  {detail}` where `detail` is right-
/// dimmed to visually separate it from the primary label.
fn build_tree_item<'a>(entry: &DisplayEntry, theme: &'a Theme) -> ListItem<'a> {
    let prefix = Span::styled(entry.prefix.clone(), theme.data_key);
    let label = Span::styled(entry.label.clone(), theme.data_value);

    if entry.detail.is_empty() {
        ListItem::new(Line::from(vec![prefix, label]))
    } else {
        let sep = Span::styled("  ".to_owned(), theme.data_value);
        let detail = Span::styled(entry.detail.clone(), theme.tab_inactive);
        ListItem::new(Line::from(vec![prefix, label, sep, detail]))
    }
}

// ---------------------------------------------------------------------------
// Search bar
// ---------------------------------------------------------------------------

/// Render the search input bar at the bottom of the body area.
///
/// Displays the current search query with a blinking cursor indicator.
/// The operator types characters which are accumulated in
/// `ViewerState::search_query`.
fn render_search_bar(frame: &mut Frame, area: Rect, query: &str, theme: &Theme) {
    let prompt = format!(" / {query}█");
    let line = Line::from(Span::styled(prompt, theme.data_value));

    let block = Block::default()
        .title(" Search ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(theme.border);

    let paragraph = Paragraph::new(vec![line]).block(block);
    frame.render_widget(paragraph, area);
}

// ---------------------------------------------------------------------------
// Status bar
// ---------------------------------------------------------------------------

/// Key legend for the viewer status bar.
const VIEWER_KEY_LEGEND: &str =
    "  Tab: tabs | ↑↓: select | Enter/Space: expand | Backspace: up | /: search | q: quit";

/// Render the viewer status bar.
///
/// Displays the app's current status message left-aligned with a compact
/// key legend right-aligned when space permits.
fn render_viewer_status(frame: &mut Frame, area: Rect, app: &dyn ViewerApp, theme: &Theme) {
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

    let status_text = format!(" {icon} {} ", status.text);
    let total_width = area.width as usize;
    let legend_chars = VIEWER_KEY_LEGEND.chars().count();
    let status_chars = status_text.chars().count();
    let combined = status_chars.saturating_add(legend_chars);

    let padded = if combined <= total_width {
        let pad = total_width.saturating_sub(status_chars).saturating_sub(legend_chars);
        format!("{status_text}{}{VIEWER_KEY_LEGEND}", " ".repeat(pad))
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
