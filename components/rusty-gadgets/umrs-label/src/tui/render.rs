// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams

//! # render — Custom TUI Renderer for the Security Label Registry
//!
//! Provides [`render_label_registry`], the full-screen TUI renderer for the
//! `umrs-label` Security Label Registry browser.
//!
//! ## Layout (top to bottom)
//!
//! 1. **Header row** — security posture panel (hostname, OS, SELinux, FIPS)
//!    plus WIZARD_SMALL logo, `HEADER_HEIGHT` rows tall.
//! 2. **Body** — horizontal split: tree panel (left, `TREE_PERCENT`%) and
//!    detail panel (right, remainder).  Active panel border is bright;
//!    inactive panel border is dimmed.
//! 3. **Search bar** — `SEARCH_BAR_HEIGHT` rows when `state.search_active`.
//! 4. **Status bar** — marking count, key legend, and tool identity.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AU-3**: Every frame carries hostname, OS, SELinux
//!   mode, FIPS state, and marking count.  No frame renders without full
//!   audit identification.
//! - **NIST SP 800-53 AC-3**: All rendering reads from shared references;
//!   no write path exists through the renderer.
//! - **NIST SP 800-53 AC-16**: The detail panel renders every field in the
//!   selected marking without omission, satisfying label display fidelity.
//! - **NSA RTB RAIN**: Non-bypassable read-only contract — the renderer
//!   receives only shared references and produces no side effects.

use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph};

use umrs_core::robots::WIZARD_SMALL;
use umrs_ui::app::{HeaderContext, IndicatorValue};
use umrs_ui::marking_detail::MarkingDetailData;
use umrs_ui::palette::{palette_bg, palette_fg};
use umrs_ui::text_fit::{display_width, truncate_right, wrap_indented, wrap_text_lines};
use umrs_ui::theme::Theme;
use umrs_ui::viewer::ViewerState;
use umrs_ui::viewer::tree::DisplayEntry;

use crate::tui::app::{DetailContent, LabelRegistryApp, Panel};

// ---------------------------------------------------------------------------
// Icons — subset needed for the tree display.
// Add to umrs_ui::icons if these become more widely used.
// ---------------------------------------------------------------------------

use umrs_ui::icons::{CHEVRON_CLOSED, CHEVRON_OPEN, ICON_MARKING};

// ---------------------------------------------------------------------------
// Layout constants
// ---------------------------------------------------------------------------

/// Width of the wizard logo panel: `WIZARD_SMALL.width` (15) + 2 border columns.
const LOGO_PANEL_WIDTH: u16 = 17;

/// Height of the header row (posture info box + wizard logo).
const HEADER_HEIGHT: u16 = 9;

/// Height of the search bar when active (content + 2 border rows).
const SEARCH_BAR_HEIGHT: u16 = 3;

/// Tree panel width as a percentage of the body area.
const TREE_PERCENT: u16 = 48;

/// Compact key legend for the status bar.
const KEY_LEGEND: &str = "  ↑↓: nav | Enter: show | Tab: panel | /: search | ?: help | q: quit";

// ---------------------------------------------------------------------------
// Master render entry point
// ---------------------------------------------------------------------------

/// Render the complete Security Label Registry TUI into `area`.
///
/// Call this inside `terminal.draw(|f| render_label_registry(f, f.area(), ...))`.
///
/// `detail_content` is the prepared detail panel content for the currently
/// selected tree node.  `detail_scroll` is the scroll offset for the detail
/// panel.
///
/// NIST SP 800-53 AU-3 — all identification, security posture, and catalog
/// fields are present in every rendered frame.
/// NIST SP 800-53 AC-3 — rendering is unconditionally read-only.
#[expect(
    clippy::too_many_arguments,
    reason = "render entry points aggregate all display state; splitting would scatter the layout pass"
)]
pub fn render_label_registry(
    frame: &mut Frame,
    area: Rect,
    app: &LabelRegistryApp,
    state: &ViewerState,
    ctx: &HeaderContext,
    theme: &Theme,
    detail_content: &DetailContent,
    detail_scroll: u16,
    active_panel: Panel,
) {
    let search_height = if state.search_active {
        SEARCH_BAR_HEIGHT
    } else {
        0
    };

    // ── Outer vertical split ─────────────────────────────────────────────────
    let outer = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(HEADER_HEIGHT), // header row
            Constraint::Min(0),                // body (tree + detail)
            Constraint::Length(search_height), // search bar
            Constraint::Length(1),             // status bar
        ])
        .split(area);

    let [header_area, body_area, search_area, status_area] = *outer else {
        return;
    };

    // ── Header row: posture (left) + wizard (right) ──────────────────────────
    let header_cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(0), Constraint::Length(LOGO_PANEL_WIDTH)])
        .split(header_area);

    let [header_text_area, logo_area] = *header_cols else {
        return;
    };

    render_posture_header(frame, header_text_area, ctx, theme);
    render_wizard_logo(frame, logo_area, theme);

    // ── Body: tree | detail ──────────────────────────────────────────────────
    let body_cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(TREE_PERCENT),
            Constraint::Percentage(100 - TREE_PERCENT),
        ])
        .split(body_area);

    let [tree_area, detail_area] = *body_cols else {
        return;
    };

    render_tree_panel(frame, tree_area, state, theme, active_panel);
    render_detail_panel(
        frame,
        detail_area,
        detail_content,
        detail_scroll,
        theme,
        active_panel,
    );

    // ── Search bar ───────────────────────────────────────────────────────────
    if state.search_active {
        render_search_bar(frame, search_area, &state.search_query, theme);
    }

    // ── Status bar ───────────────────────────────────────────────────────────
    render_status_bar(frame, status_area, app, theme);
}

// ---------------------------------------------------------------------------
// Security posture header
// ---------------------------------------------------------------------------

/// Render the security posture header (left panel of the header row).
///
/// Layout (inside the bordered block):
/// - Row pair 1-4: system posture (left 55%) + session info (right 45%).
/// - Remaining rows: absorbed by the minimum constraint.
///
/// NIST SP 800-53 AU-3 — hostname, OS, SELinux mode, and FIPS state are
/// visible in every rendered frame.
fn render_posture_header(frame: &mut Frame, area: Rect, ctx: &HeaderContext, theme: &Theme) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(theme.border);
    let inner = block.inner(area);
    frame.render_widget(block, area);

    // Vertical split: 4 posture rows, gap, 1 title row.
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),
            Constraint::Min(0),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(inner);
    let [top_area, _gap, title_area, _bottom_pad] = *rows else {
        return;
    };

    // Horizontal split inside the posture block: system left + session right.
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
        .split(top_area);
    let [left_area, right_area] = *cols else {
        return;
    };

    render_system_posture_lines(frame, left_area, ctx, theme);
    render_session_lines(frame, right_area, theme);

    // Title: centered pill badge via shared umrs-ui widget.
    umrs_ui::pill::render_pill(
        frame,
        title_area,
        "UMRS Security Label Registry",
        Color::Black,
        Color::White,
        5,
    );
}

/// Host, OS, SELinux, FIPS rows.
fn render_system_posture_lines(frame: &mut Frame, area: Rect, ctx: &HeaderContext, theme: &Theme) {
    let selinux_value = indicator_text(&ctx.indicators.selinux_status);
    let selinux_style = theme.indicator_style(&ctx.indicators.selinux_status);
    let fips_value = indicator_text(&ctx.indicators.fips_mode);
    let fips_style = theme.indicator_style(&ctx.indicators.fips_mode);
    let os_value = format!("{} ({})", ctx.os_name, ctx.architecture);

    let rows: [(&str, String, Style); 4] = [
        ("Host", ctx.hostname.clone(), theme.data_value),
        ("OS", os_value, theme.data_value),
        ("SELinux", selinux_value, selinux_style),
        ("FIPS", fips_value, fips_style),
    ];
    render_kv_rows(frame, area, &rows, theme);
}

/// Tool, User, Domain, Level rows.
///
/// - **Tool** shows the binary name and version for at-a-glance confirmation.
/// - **User** is the operator username; SELinux domain is surfaced separately.
/// - **Domain** is the `_t` type of the running process context from
///   `/proc/self/attr/current`, shown on its own row for clarity.
/// - **Level** is the raw sensitivity level of the running process.
///
/// NIST SP 800-53 AU-3 — subject identity is a required audit record field.
/// NIST SP 800-53 IA-2 — visible identification of the running user and
/// process domain supports operator accountability.
fn render_session_lines(frame: &mut Frame, area: Rect, theme: &Theme) {
    let username = std::env::var("USER").unwrap_or_else(|_| {
        let uid = nix::unistd::Uid::current();
        nix::unistd::User::from_uid(uid)
            .ok()
            .flatten()
            .map_or_else(|| uid.as_raw().to_string(), |u| u.name)
    });

    let (ctx_type, ctx_level) = umrs_selinux::utils::get_self_context().map_or_else(
        |_| ("<unavailable>".to_owned(), "-".to_owned()),
        |sc| {
            let t = sc.security_type().to_string();
            let l = sc.level().map_or_else(|| "-".to_owned(), |lvl| lvl.raw().to_owned());
            (t, l)
        },
    );

    let tool_value = format!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

    let rows: [(&str, String, Style); 4] = [
        ("Tool", tool_value, theme.data_value),
        ("User", username, theme.data_value),
        ("Domain", ctx_type, theme.data_value),
        ("Level", ctx_level, theme.data_value),
    ];
    render_kv_rows(frame, area, &rows, theme);
}

/// Reusable `Label : value` row renderer — matches umrs-ls header pattern.
fn render_kv_rows(frame: &mut Frame, area: Rect, rows: &[(&str, String, Style)], theme: &Theme) {
    let max_label = rows.iter().map(|(l, ..)| display_width(l)).max().unwrap_or(0);
    let label_cell_width = 1 + max_label + 3;
    let value_budget = (area.width as usize).saturating_sub(label_cell_width).saturating_sub(1);

    let lines: Vec<Line<'static>> = rows
        .iter()
        .map(|(label, value, value_style)| {
            let label_text = format!(" {label:<max_label$} : ");
            let fitted = truncate_right(value, value_budget);
            Line::from(vec![
                Span::styled(label_text, theme.data_key),
                Span::styled(fitted, *value_style),
            ])
        })
        .collect();

    frame.render_widget(Paragraph::new(lines), area);
}

// ---------------------------------------------------------------------------
// Wizard logo
// ---------------------------------------------------------------------------

/// Render the WIZARD_SMALL ASCII art logo panel (identical to umrs-ls).
fn render_wizard_logo(frame: &mut Frame, area: Rect, theme: &Theme) {
    let lines: Vec<Line<'_>> =
        WIZARD_SMALL.lines.iter().map(|l| Line::from(Span::styled(*l, theme.wizard))).collect();

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(theme.border);

    frame.render_widget(Paragraph::new(lines).block(block), area);
}

// ---------------------------------------------------------------------------
// Tree panel
// ---------------------------------------------------------------------------

/// Render the tree browser panel.
///
/// When `active_panel == Panel::Tree`, the border is full-brightness cyan.
/// When the detail panel is focused, the tree border is dimmed.
///
/// Each node is rendered with:
/// - Branch (expanded):   `▼ <label>`
/// - Branch (collapsed):  `▶ <label>`
/// - Leaf:                `● <label>`
///
/// The cursor line (`►`) from ratatui's `ListState` highlights the selection.
///
/// NIST SP 800-53 AU-3 — every node carries its catalog key in metadata;
/// the tree faithfully represents all loaded catalog entries.
fn render_tree_panel(
    frame: &mut Frame,
    area: Rect,
    state: &ViewerState,
    theme: &Theme,
    active_panel: Panel,
) {
    let border_style = if active_panel == Panel::Tree {
        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(border_style);
    let raw_inner = block.inner(area);
    frame.render_widget(block, area);

    // Shift content down by one row (top margin) and right by one column
    // (left padding) inside the panel.
    let inner = Rect {
        x: raw_inner.x.saturating_add(1),
        y: raw_inner.y.saturating_add(1),
        width: raw_inner.width.saturating_sub(1),
        height: raw_inner.height.saturating_sub(1),
    };

    let display = &state.tree.display_list;
    if display.is_empty() {
        let hint = Line::from(Span::styled("  (no catalogs loaded)", theme.data_key));
        frame.render_widget(Paragraph::new(vec![hint]), inner);
        return;
    }

    let panel_width = (inner.width as usize).saturating_sub(1); // 1-char right pad
    let items: Vec<ListItem<'_>> = display
        .iter()
        .filter(|e| is_visible_entry(e, &state.tree.roots))
        .map(|entry| build_tree_item(entry, &state.tree.roots, theme, panel_width))
        .collect();

    // Compute the list-state selected index, accounting for hidden nodes.
    let visible_selected = compute_visible_selected(state);

    let mut list_state = ListState::default();
    list_state.select(visible_selected);

    // No highlight_symbol — the color delta from highlight_style is sufficient,
    // and removing it eliminates the 2-char blank prefix on non-selected rows
    // that was pushing depth-0 icons away from the left edge.
    let list = List::new(items).highlight_style(theme.list_selection);

    frame.render_stateful_widget(list, inner, &mut list_state);
}

/// Returns `true` if the display entry should be shown given current filter state.
///
/// Walks the `roots` using the entry's path to find the corresponding `TreeNode`
/// and check its `visible` flag.
fn is_visible_entry(entry: &DisplayEntry, roots: &[umrs_ui::viewer::tree::TreeNode]) -> bool {
    let mut nodes = roots;
    for (i, &idx) in entry.path.iter().enumerate() {
        match nodes.get(idx) {
            None => return false,
            Some(node) => {
                if i == entry.path.len() - 1 {
                    return node.visible;
                }
                nodes = &node.children;
            }
        }
    }
    true
}

/// Walk the tree to find the node at `entry.path`.
fn node_at_path<'a>(
    roots: &'a [umrs_ui::viewer::tree::TreeNode],
    path: &[usize],
) -> Option<&'a umrs_ui::viewer::tree::TreeNode> {
    let mut nodes = roots;
    let mut node = None;
    for &idx in path {
        node = nodes.get(idx);
        if let Some(n) = node {
            nodes = &n.children;
        } else {
            return None;
        }
    }
    node
}

/// Build a `ListItem` for one display-list entry.
fn build_tree_item<'a>(
    entry: &DisplayEntry,
    roots: &'a [umrs_ui::viewer::tree::TreeNode],
    theme: &'a Theme,
    panel_width: usize,
) -> ListItem<'a> {
    let depth = entry.path.len().saturating_sub(1);
    // Each level indents by 2 spaces so each icon falls under the first character
    // of the parent row's label text (icon = 1 char + 1 trailing space = 2 chars wide).
    let indent = " ".repeat(depth * 2);

    let Some(node) = node_at_path(roots, &entry.path) else {
        return ListItem::new(Line::from(Span::raw("?")));
    };

    let (icon, icon_style) = if node.children.is_empty() {
        (ICON_MARKING, theme.data_value)
    } else if node.expanded {
        (CHEVRON_OPEN, theme.header_field)
    } else {
        (CHEVRON_CLOSED, theme.header_field)
    };

    let label_style = if node.children.is_empty() {
        theme.data_value
    } else {
        theme.header_field
    };

    let label = &node.label;

    // Build icon string. CHEVRON_OPEN/CHEVRON_CLOSED already include a trailing
    // space ("▼ ", "▶ "). ICON_MARKING has no trailing space, so append one here.
    // This avoids double-spacing chevrons while keeping uniform 2-char icon width.
    let icon_str: std::borrow::Cow<'_, str> = if node.children.is_empty() {
        std::borrow::Cow::Owned(format!("{icon} "))
    } else {
        std::borrow::Cow::Borrowed(icon)
    };

    // Compute available width for the label text after indent + icon.
    // No highlight_symbol — selection uses color only.
    let prefix_width = display_width(&indent) + display_width(icon_str.as_ref());
    let label_budget = panel_width.saturating_sub(prefix_width);

    // Truncate with ellipsis if the label overflows. The operator can
    // widen the terminal or press Enter to see the full marking in the
    // detail panel.
    let fitted = truncate_right(label, label_budget);

    Line::from(vec![
        Span::raw(indent),
        Span::styled(icon_str.into_owned(), icon_style),
        Span::styled(fitted, label_style),
    ])
    .into()
}

/// Compute the list-widget selected index in the visible-only item list,
/// accounting for nodes that are hidden by search filtering.
fn compute_visible_selected(state: &ViewerState) -> Option<usize> {
    let display = &state.tree.display_list;
    let roots = &state.tree.roots;
    let mut visible_idx = 0usize;
    for (raw_idx, entry) in display.iter().enumerate() {
        if is_visible_entry(entry, roots) {
            if raw_idx == state.selected_index {
                return Some(visible_idx);
            }
            visible_idx = visible_idx.saturating_add(1);
        }
    }
    // If not found (shouldn't happen), select nothing.
    None
}

// ---------------------------------------------------------------------------
// Detail panel
// ---------------------------------------------------------------------------

/// Render the detail panel on the right side.
///
/// Content depends on what is selected in the tree:
/// - `DetailContent::None` — placeholder ("select a marking")
/// - `DetailContent::Marking(_)` / `DetailContent::DisseminationControl(_)`
///   — full marking detail via `render_marking_detail`
/// - `DetailContent::CatalogMetadata(_)` — key-value rows from `_metadata`
/// - `DetailContent::Group { .. }` — brief group name + count summary
///
/// The panel border is bright when `active_panel == Panel::Detail`.
///
/// NIST SP 800-53 AC-16 — every field in the selected marking is rendered
/// without omission, satisfying label display fidelity requirements.
#[expect(
    clippy::too_many_lines,
    reason = "match arms for each DetailContent variant each need their own \
              border + padding setup; splitting would scatter related layout logic"
)]
fn render_detail_panel(
    frame: &mut Frame,
    area: Rect,
    content: &DetailContent,
    scroll_offset: u16,
    theme: &Theme,
    active_panel: Panel,
) {
    let border_style = if active_panel == Panel::Detail {
        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    match content {
        DetailContent::None => {
            let block = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(border_style);
            let lines = vec![
                Line::from(""),
                Line::from(Span::styled(
                    "  Select a marking to view details.",
                    theme.data_value,
                )),
                Line::from(""),
                Line::from(Span::styled(
                    "  Press Enter on a leaf node to display its full record.",
                    theme.data_key,
                )),
            ];
            let paragraph = Paragraph::new(lines).block(block);
            frame.render_widget(paragraph, area);
        }

        DetailContent::Marking(data, prov) | DetailContent::DisseminationControl(data, prov) => {
            // Temporarily apply the active/inactive border style by building a
            // modified version of the data block.  `render_marking_detail`
            // creates its own block internally — we override the border by
            // rendering a separate border block first, then rendering the inner
            // content without borders, so that the active/inactive style applies.
            render_marking_detail_with_border(
                frame,
                area,
                data,
                prov,
                scroll_offset,
                theme,
                border_style,
            );
        }

        DetailContent::CatalogMetadata(rows) => {
            let block = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(border_style);
            let raw_inner = block.inner(area);
            frame.render_widget(block, area);
            let inner = Rect {
                x: raw_inner.x.saturating_add(1),
                width: raw_inner.width.saturating_sub(1),
                ..raw_inner
            };

            let mut lines: Vec<Line<'_>> = vec![Line::from("")];
            let max_label = rows
                .iter()
                .filter(|(k, _)| !k.is_empty())
                .map(|(k, _)| display_width(k))
                .max()
                .unwrap_or(0);

            for (key, value) in rows {
                if key.is_empty() && value.is_empty() {
                    // Blank separator row.
                    lines.push(Line::from(""));
                    continue;
                }
                let key_str = format!("  {key:<max_label$} : ");
                let key_prefix_width = display_width(&key_str);
                let value_budget =
                    (inner.width as usize).saturating_sub(key_prefix_width).saturating_sub(1);
                let value_lines = wrap_text_lines(value, value_budget);
                let continuation_pad = " ".repeat(key_prefix_width);
                for (i, vline) in value_lines.iter().enumerate() {
                    if i == 0 {
                        lines.push(Line::from(vec![
                            Span::styled(key_str.clone(), theme.data_key),
                            Span::styled(vline.clone(), theme.data_value),
                        ]));
                    } else {
                        lines.push(Line::from(Span::styled(
                            format!("{continuation_pad}{vline}"),
                            theme.data_value,
                        )));
                    }
                }
            }
            lines.push(Line::from(""));
            let paragraph = Paragraph::new(lines).scroll((scroll_offset, 0));
            frame.render_widget(paragraph, inner);
        }

        DetailContent::Group {
            name,
            count,
        } => {
            let block = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(border_style);
            let raw_inner = block.inner(area);
            frame.render_widget(block, area);
            let inner = Rect {
                x: raw_inner.x.saturating_add(1),
                width: raw_inner.width.saturating_sub(1),
                ..raw_inner
            };

            let lines = vec![
                Line::from(""),
                Line::from(vec![
                    Span::styled("  Group  : ", theme.data_key),
                    Span::styled(name.as_str(), theme.header_name),
                ]),
                Line::from(vec![
                    Span::styled("  Count  : ", theme.data_key),
                    Span::styled(count.to_string(), theme.data_value),
                ]),
                Line::from(""),
                Line::from(Span::styled(
                    "  Press Enter or ▶ to expand this group.",
                    theme.data_key,
                )),
            ];
            frame.render_widget(Paragraph::new(lines), inner);
        }
    }
}

/// Render a `MarkingDetailData` with a custom border style for
/// active/inactive panel distinction.
///
/// Renders the outer border block with the requested `border_style`, then
/// builds all marking content lines (including subdued provenance rows at the
/// bottom) and renders them as a single scrollable `Paragraph` inside the
/// inner area.
fn render_marking_detail_with_border(
    frame: &mut Frame,
    area: Rect,
    data: &MarkingDetailData,
    provenance: &[(String, String)],
    scroll_offset: u16,
    theme: &Theme,
    border_style: Style,
) {
    // Render the outer border with the desired active/inactive style.
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(border_style);
    let raw_inner = block.inner(area);
    frame.render_widget(block, area);

    // 1-char left padding inside the detail panel.
    let inner = Rect {
        x: raw_inner.x.saturating_add(1),
        width: raw_inner.width.saturating_sub(1),
        ..raw_inner
    };

    render_detail_content_inner(frame, inner, data, provenance, scroll_offset, theme);
}

/// Render the marking detail content inside an already-bordered area.
///
/// Builds all marking field lines plus subdued provenance rows and renders
/// them as a single scrollable `Paragraph`. Provenance rows use `DarkGray`
/// to provide attribution context without competing visually with the marking
/// content.
///
/// ## Layout
///
/// - Palette-colored marking key header (index-group background, white/black fg)
/// - Identity and classification fields (`Name`, `Abbreviation`, `Designation`, …)
/// - Multi-line text fields (`Description`, `Handling`, `Dissemination`, …)
///   rendered as label-on-own-line with 4-space-indented text below
/// - Additional fields (MCS, phase notes, …)
/// - Subdued provenance block (catalog name, version, authority, …)
#[expect(
    clippy::too_many_lines,
    reason = "sequential section-by-section rendering of all marking fields; \
              splitting into sub-functions would add indirection without improving clarity"
)]
fn render_detail_content_inner(
    frame: &mut Frame,
    area: Rect,
    data: &MarkingDetailData,
    provenance: &[(String, String)],
    scroll_offset: u16,
    theme: &Theme,
) {
    const KEY_SEP: &str = " : ";
    let panel_width = (area.width as usize).saturating_sub(1); // 1-char right pad

    // ── Dynamic key width — only labels rendered in `key : value` style ──────
    // Note: Description, Handling, Dissemination, Required Warning, and
    // Injury Examples are rendered as label-then-indented-text and are
    // excluded from the key-width calculation.
    let skip_fields = ["MCS Category Base", "MCS Range (Reserved)", "US CUI Approximation"];
    let mut labels: Vec<&str> = Vec::new();
    if !data.name_en.is_empty() {
        labels.push("Name");
    }
    if !data.abbreviation.is_empty() {
        labels.push("Abbreviation");
    }
    if !data.designation.is_empty() {
        labels.push("Designation");
    }
    if !data.index_group.is_empty() {
        labels.push("Index Group");
    }
    for (key, value) in &data.additional {
        if !value.is_empty() && !skip_fields.contains(&key.as_str()) {
            labels.push(key.as_str());
        }
    }
    let key_width = labels.iter().map(|l| display_width(l)).max().unwrap_or(12);

    let mut lines: Vec<Line<'static>> = Vec::new();

    // ── Marking key — palette-colored chip, country flag flush-right ────────
    lines.push(Line::from(""));
    if !data.key.is_empty() {
        let bg = palette_bg(&data.index_group);
        let fg = palette_fg(&data.index_group);
        let key_style = Style::default().fg(fg).bg(bg).add_modifier(Modifier::BOLD);
        let padded_key = format!("  {} ", data.key);
        let mut spans = vec![Span::raw("  "), Span::styled(padded_key.clone(), key_style)];
        if !data.country_flag.is_empty() {
            let flag_width = display_width(&data.country_flag);
            let used = 2 + display_width(&padded_key); // leading "  " + chip
            let gap = panel_width.saturating_sub(used + flag_width);
            spans.push(Span::raw(" ".repeat(gap)));
            spans.push(Span::styled(data.country_flag.clone(), Style::default()));
        }
        lines.push(Line::from(spans));
        lines.push(Line::from(""));
    }

    // Identity fields — English only; French locale display is a future feature.
    push_kv_detail(
        &mut lines,
        "Name",
        &data.name_en,
        key_width,
        KEY_SEP,
        panel_width,
        theme,
    );
    push_kv_detail(
        &mut lines,
        "Abbreviation",
        &data.abbreviation,
        key_width,
        KEY_SEP,
        panel_width,
        theme,
    );

    // Designation with colour coding
    if !data.designation.is_empty() {
        let key_str = format!("  {:<key_width$}{KEY_SEP}", "Designation");
        let value_style = match data.designation.as_str() {
            "specified" | "SP" => Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            "basic" => Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
            "LDC" => Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            _ => theme.data_value,
        };
        lines.push(Line::from(vec![
            Span::styled(key_str, theme.data_key),
            Span::styled(data.designation.clone(), value_style),
        ]));
    }

    push_kv_detail(
        &mut lines,
        "Index Group",
        &data.index_group,
        key_width,
        KEY_SEP,
        panel_width,
        theme,
    );
    // Description — label on its own line, wrapped text below with 4-char indent.
    if !data.description_en.is_empty() {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled("  Description", theme.data_key)));
        lines.extend(wrap_indented(
            &data.description_en,
            "    ",
            panel_width,
            theme.data_value,
        ));
    }

    // Handling — label on its own line, wrapped text below with 4-char indent.
    if !data.handling.is_empty() {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled("  Handling", theme.data_key)));
        lines.extend(wrap_indented(
            &data.handling,
            "    ",
            panel_width,
            theme.data_value,
        ));
    }

    // Injury examples — label on its own line, wrapped text below with 4-char indent.
    if !data.injury_examples_en.is_empty() {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "  Injury Examples",
            theme.data_key,
        )));
        lines.extend(wrap_indented(
            &data.injury_examples_en,
            "    ",
            panel_width,
            theme.data_value,
        ));
    }

    // Required warning — label on its own line, wrapped text below with 4-char indent.
    if !data.required_warning.is_empty() {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "  Required Warning",
            theme.data_key,
        )));
        lines.extend(wrap_indented(
            &data.required_warning,
            "    ",
            area.width as usize,
            Style::default().fg(Color::Yellow),
        ));
    }

    // Dissemination — label on its own line, wrapped text below with 4-char indent.
    if !data.required_dissemination.is_empty() {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled("  Dissemination", theme.data_key)));
        lines.extend(wrap_indented(
            &data.required_dissemination,
            "    ",
            area.width as usize,
            theme.data_value,
        ));
    }

    // Additional fields — skip internal MCS fields, use label-on-own-line
    // for long-text fields, key-value for short ones.
    if !data.additional.is_empty() {
        let skip = ["MCS Category Base", "MCS Range (Reserved)"];
        let long_text = ["US CUI Approximation"];
        lines.push(Line::from(""));
        for (key, value) in &data.additional {
            if value.is_empty() || skip.contains(&key.as_str()) {
                continue;
            }
            if long_text.contains(&key.as_str()) {
                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled(format!("  {key}"), theme.data_key)));
                lines.extend(wrap_indented(value, "    ", panel_width, theme.data_value));
            } else {
                push_kv_detail(
                    &mut lines,
                    key,
                    value,
                    key_width,
                    KEY_SEP,
                    panel_width,
                    theme,
                );
            }
        }
    }

    // ── Provenance — compact catalog attribution ──────────────────────────────
    if !provenance.is_empty() {
        let prov_style = Style::default().fg(Color::DarkGray);
        // Build a compact "Catalog · Version · Authority" line.
        let parts: Vec<&str> =
            provenance.iter().filter(|(_, v)| !v.is_empty()).map(|(_, v)| v.as_str()).collect();
        if !parts.is_empty() {
            let attribution = format!("  {}", parts.join("  ·  "));
            lines.push(Line::from(""));
            lines.push(Line::from(""));
            lines.extend(wrap_indented(&attribution, "  ", panel_width, prov_style));
        }
    }

    lines.push(Line::from(""));

    let paragraph = Paragraph::new(lines).scroll((scroll_offset, 0));
    frame.render_widget(paragraph, area);
}

/// Push key-value line(s) with word-wrapping.
///
/// If the value fits on one line after the key, it is rendered inline.
/// If it overflows, continuation lines are indented to align under the
/// start of the value (under the character after ` : `).
fn push_kv_detail(
    lines: &mut Vec<Line<'static>>,
    key: &str,
    value: &str,
    key_width: usize,
    sep: &str,
    panel_width: usize,
    theme: &Theme,
) {
    if value.is_empty() {
        return;
    }
    let key_str = format!("  {key:<key_width$}{sep}");
    let prefix_width = display_width(&key_str);
    let value_budget = panel_width.saturating_sub(prefix_width);
    let value_lines = wrap_text_lines(value, value_budget);
    let continuation_pad = " ".repeat(prefix_width);
    for (i, vline) in value_lines.iter().enumerate() {
        if i == 0 {
            lines.push(Line::from(vec![
                Span::styled(key_str.clone(), theme.data_key),
                Span::styled(vline.clone(), theme.data_value),
            ]));
        } else {
            lines.push(Line::from(Span::styled(
                format!("{continuation_pad}{vline}"),
                theme.data_value,
            )));
        }
    }
}

// ---------------------------------------------------------------------------
// Search bar
// ---------------------------------------------------------------------------

/// Render the search/filter bar at the bottom of the body area.
fn render_search_bar(frame: &mut Frame, area: Rect, query: &str, theme: &Theme) {
    let block = Block::default()
        .title(" Search / Filter ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Yellow));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let prompt = format!("  ▶  {query}_");
    let line = Line::from(Span::styled(prompt, theme.data_value));
    frame.render_widget(Paragraph::new(vec![line]), inner);
}

// ---------------------------------------------------------------------------
// Status bar
// ---------------------------------------------------------------------------

/// Render the status bar at the very bottom of the screen.
///
/// Shows tool identity on the left, marking count in the center, and the
/// key legend on the right (elided on narrow terminals).
///
/// NIST SP 800-53 AU-3 — tool name and record count are visible in every
/// rendered frame.
fn render_status_bar(frame: &mut Frame, area: Rect, app: &LabelRegistryApp, theme: &Theme) {
    let count = app.total_markings();
    let center = format!("  Security Label Registry  |  {count} entries  ");
    let legend = KEY_LEGEND;
    let total_width = area.width as usize;

    // Elide legend if it won't fit alongside the center text.
    let text = if total_width > center.len().saturating_add(legend.len()) {
        let pad = total_width.saturating_sub(center.len()).saturating_sub(legend.len());
        format!("{center}{}{legend}", " ".repeat(pad))
    } else {
        let budget = total_width.saturating_sub(1);
        truncate_right(&center, budget)
    };

    // Info-level background — consistent with umrs-ui shared status bar palette.
    let bg = umrs_ui::theme::status_bg_color(umrs_ui::app::StatusLevel::Info);
    let style = Style::default().bg(bg).patch(theme.status_text);
    let line = Line::from(Span::styled(text, style));
    frame.render_widget(Paragraph::new(vec![line]), area);
}

// ---------------------------------------------------------------------------
// Palette color helpers
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// Private helpers
// ---------------------------------------------------------------------------

/// Extract display text from an `IndicatorValue`.
fn indicator_text(v: &IndicatorValue) -> String {
    match v {
        IndicatorValue::Enabled(s) | IndicatorValue::Disabled(s) => s.clone(),
        IndicatorValue::Unavailable => "unavailable".to_owned(),
    }
}
