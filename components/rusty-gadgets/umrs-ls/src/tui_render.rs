// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)

//! # tui_render — Custom TUI Renderer for umrs-ls
//!
//! Provides [`render_dir_browser`], the full-screen TUI renderer for the
//! `umrs-ls` directory browser.  This module replaces the generic
//! `render_viewer` function with a layout tailored to `umrs-ls`:
//!
//! - **Security posture header** — hostname, OS, SELinux status, FIPS mode,
//!   and the WIZARD_SMALL logo.  Derived from a live [`HeaderContext`] snapshot.
//! - **Path bar** — the current directory path, styled as a prominent banner.
//! - **Column headers** — IOV / MODE / OWNER:GROUP / MODIFIED / NAME, matching
//!   the CLI columnar output.
//! - **Full-width listing** — group headers (expandable/collapsible SELinux
//!   type + marking rows) and file entry rows with CLI-equivalent columns.
//!   No detail panel — the operator sees CLI-style output directly.
//! - **Search bar** — appears when `state.search_active` is true.
//! - **Status bar** — scan timing, file/dir counts, and key legend.
//!
//! ## Key Exported Types
//!
//! - [`render_dir_browser`] — master render entry point.
//!
//! ## Performance
//!
//! Called every rendered frame (~10 fps in the event loop).  All string
//! allocations happen inside `terminal.draw()` which is called only when
//! the display actually needs updating.  Pre-formatting is applied to the
//! header (built once at startup) but not to the listing rows — those are
//! generated per-frame from `ViewerState::tree.display_list`.  The listing
//! is bounded by the number of visible rows (terminal height), so the hot
//! path only processes the visible window.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AU-3**: Every frame renders the full complement of
//!   audit-relevant fields — hostname, security posture, assessment time,
//!   SELinux type/marking (group headers), mode bits, ownership, and mtime.
//! - **NIST SP 800-53 AC-3**: The renderer is unconditionally read-only;
//!   no directory mutation is possible through this interface.
//! - **NIST SP 800-53 AC-4**: SELinux type and MCS marking are prominent
//!   in group header rows, surfacing information-flow boundary context.
//! - **NIST SP 800-53 CA-7**: The `assessed_at` timestamp from `HeaderContext`
//!   appears in the header, dating the collection event.
//! - **NSA RTB RAIN**: Non-bypassable read-only contract — all rendering reads
//!   from `ViewerState` and `DirViewerApp` references, never writes.

use chrono::{DateTime, Local, TimeZone as _};
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Clear, List, ListItem, ListState, Paragraph};

use umrs_core::robots::WIZARD_SMALL;
use umrs_ui::app::{HeaderContext, IndicatorValue, StatusLevel};
use umrs_ui::theme::Theme;
use umrs_ui::viewer::{ViewerApp as _, ViewerState};

use crate::viewer_app::DirViewerApp;

// ---------------------------------------------------------------------------
// Icons — single source of truth for all unicode glyphs used in the TUI.
//
// Change one constant to change every occurrence.  When `umrs-labels`
// palette support lands in Phase 5, these may become runtime-configurable.
// ---------------------------------------------------------------------------

/// Mount point icon (⛁ — Funeral Urn / database cylinder).
const ICON_MOUNT: &str = "\u{26C1}";

/// Encrypted directory icon (🔒).
const ICON_ENCRYPTED: &str = "\u{1F512}";

/// Plain directory icon (📁).
const ICON_DIR: &str = "\u{1F4C1}";

/// Symlink icon (🔗).
const ICON_SYMLINK: &str = "\u{1F517}";

/// Parent directory navigation icon (↰).
const ICON_PARENT: &str = "\u{21B0}";

/// Cuddled sibling connector (└ — box-drawing lower-left corner).
const ICON_SIBLING: &str = "\u{2514}";

/// Group header banner transition triangle (🭬).
const ICON_BANNER: &str = "\u{1FB6C}";

/// Search bar block cursor (█).
const ICON_CURSOR: &str = "\u{2588}";

/// Prohibited / denied (🚫 — no entry sign). Reserved for future use in
/// access-denied entries or restricted group headers.
#[expect(dead_code, reason = "reserved icon — will be used when restricted group rendering is enhanced")]
const ICON_DENIED: &str = "\u{1F6AB}";

/// Expanded group chevron.
const CHEVRON_OPEN: &str = "▼ ";

/// Collapsed group chevron.
const CHEVRON_CLOSED: &str = "▶ ";

// ---------------------------------------------------------------------------
// Layout constants
// ---------------------------------------------------------------------------

/// Width of the wizard logo panel: `WIZARD_SMALL.width` (15) + 2 border columns.
const LOGO_PANEL_WIDTH: u16 = 17;

/// Height of the header row. Compact: system info + directory in fewer lines.
const HEADER_HEIGHT: u16 = 9;

/// Height of the search bar when active (1 content row + 2 border rows).
const SEARCH_BAR_HEIGHT: u16 = 3;


/// Key legend for the umrs-ls TUI status bar.
const KEY_LEGEND: &str =
    "  ↑↓:nav ←→:expand/collapse Enter:open /:search r:refresh q:quit";

// ---------------------------------------------------------------------------
// Master render entry point
// ---------------------------------------------------------------------------

/// Render the full `umrs-ls` directory browser TUI into `area`.
///
/// Layout (top to bottom):
/// 1. Header (security posture + wizard logo) — `HEADER_HEIGHT` rows.
/// 2. Path bar — 1 row.
/// 3. Column header + separator — 2 rows.
/// 4. File listing — fills remaining space.
/// 5. Search bar — `SEARCH_BAR_HEIGHT` rows when `state.search_active`.
/// 6. Status bar — 1 row.
///
/// Call this inside `terminal.draw(|f| render_dir_browser(f, f.area(), ...))`.
///
/// NIST SP 800-53 AU-3 — all identification, labeling, and audit-relevant
/// fields appear in every rendered frame.
/// NIST SP 800-53 AC-3 — rendering is unconditionally read-only.
pub fn render_dir_browser(
    frame: &mut Frame,
    area: Rect,
    app: &DirViewerApp,
    state: &ViewerState,
    ctx: &HeaderContext,
    theme: &Theme,
) {
    let search_height = if state.search_active { SEARCH_BAR_HEIGHT } else { 0 };

    // ── Top-level vertical split: header row + display panel ────────────
    // The status bar is inside the display panel, separated by T-connectors.
    let top = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(HEADER_HEIGHT), // header row (boxes 1 + 2)
            Constraint::Min(0),                // display panel (listing + status)
        ])
        .split(area);

    let [header_row_area, display_panel_area] = *top else {
        return;
    };

    // ── Header row: Box 1 (status, left) + Box 2 (wizard, right) ────────
    let header_cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(0), Constraint::Length(LOGO_PANEL_WIDTH)])
        .split(header_row_area);

    let [header_text_area, logo_area] = *header_cols else {
        return;
    };

    render_posture_header(frame, header_text_area, ctx, app, theme);
    render_wizard_logo(frame, logo_area, theme);

    // ── Box 3: Display panel (bordered, contains col headers + listing) ──
    let display_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(theme.border);
    let display_inner = display_block.inner(display_panel_area);
    frame.render_widget(display_block, display_panel_area);

    // Inside the display panel box: col header, ├───┤, listing, search, ├───┤, status
    let display_split = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // column header row
            Constraint::Length(1), // ├───┤ divider
            Constraint::Min(0),    // listing
            Constraint::Length(search_height),
            Constraint::Length(1), // ├───┤ divider (status separator)
            Constraint::Length(1), // status bar row
        ])
        .split(display_inner);

    let [col_header_area, div1_area, listing_area, search_area, div2_area, status_area] =
        *display_split
    else {
        return;
    };

    // ── Column headers ───────────────────────────────────────────────────
    render_col_headers(frame, col_header_area, theme);

    // ── Divider: ├───┤ under column headers ─────────────────────────────
    render_divider(frame, div1_area, display_panel_area, theme);

    // ── Listing ──────────────────────────────────────────────────────────
    render_listing(frame, listing_area, state, theme);

    // ── Search bar ───────────────────────────────────────────────────────
    if state.search_active {
        render_search_bar(frame, search_area, &state.search_query, theme);
    }

    // ── Divider: ├───┤ above status bar ─────────────────────────────────
    render_divider(frame, div2_area, display_panel_area, theme);

    // ── Status bar ──────────────────────────────────────────────────────
    render_status_bar(frame, status_area, app, theme);
}

// ---------------------------------------------------------------------------
// Header panel
// ---------------------------------------------------------------------------

/// Render the security posture header (left panel of the header row).
///
/// Four rows:
/// - Row 1: `Host : {hostname}`
/// - Row 2: `OS   : {os_name} ({architecture})`
/// - Row 3: `SELinux : {selinux_status}   FIPS : {fips_mode}`
/// - Row 4: `Directory : {current_path}` (bold, prominent)
///
/// Indicator values are styled with `theme.indicator_style()`.
///
/// NIST SP 800-53 AU-3 — hostname, OS, architecture, security posture,
/// and data source path are present in every rendered frame.
/// NIST SP 800-53 SI-7 — indicator values originate from provenance-verified
/// kattr reads in `build_header_context`.
fn render_posture_header(
    frame: &mut Frame,
    area: Rect,
    ctx: &HeaderContext,
    app: &DirViewerApp,
    theme: &Theme,
) {
    let host_line = {
        let label = Span::styled(" Host       : ", theme.data_key);
        let value = Span::styled(ctx.hostname.clone(), theme.data_value);
        Line::from(vec![label, value])
    };

    let os_value = format!("{} ({})", ctx.os_name, ctx.architecture);
    let os_line = {
        let label = Span::styled(" OS         : ", theme.data_key);
        let value = Span::styled(os_value, theme.data_value);
        Line::from(vec![label, value])
    };

    // SELinux indicator — its own line.
    let selinux_line = {
        let label = Span::styled(" SELinux    : ", theme.data_key);
        let style = theme.indicator_style(&ctx.indicators.selinux_status);
        let value = Span::styled(indicator_text(&ctx.indicators.selinux_status), style);
        Line::from(vec![label, value])
    };

    // FIPS indicator — its own line under SELinux.
    let fips_line = {
        let label = Span::styled(" FIPS       : ", theme.data_key);
        let style = theme.indicator_style(&ctx.indicators.fips_mode);
        let value = Span::styled(indicator_text(&ctx.indicators.fips_mode), style);
        Line::from(vec![label, value])
    };

    // Directory path — bold and prominent so it's obvious what we're looking at.
    let mount_icon = if app.dir_meta().is_mountpoint { format!("{ICON_MOUNT} ") } else { String::new() };
    let dir_line = {
        let label = Span::styled(" Directory  : ", theme.data_key);
        let value = Span::styled(
            format!("{mount_icon}{}", app.current_path().display()),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        );
        Line::from(vec![label, value])
    };

    // Directory security metadata — same color as the "Directory" label.
    let dm = app.dir_meta();
    let dir_meta_line = {
        let text = format!(
            "              {}  {}:{}  {}  {}",
            dm.mode, dm.owner, dm.group, dm.selinux_type, dm.marking,
        );
        Line::from(Span::styled(text, theme.data_key))
    };

    let lines = vec![
        host_line,
        os_line,
        selinux_line,
        fips_line,
        Line::from(""),
        dir_line,
        dir_meta_line,
    ];

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(theme.border);

    frame.render_widget(Paragraph::new(lines).block(block), area);
}

/// Extract the display text from an `IndicatorValue`.
///
/// - `Enabled(s)` / `Disabled(s)` → the inner string.
/// - `Unavailable` → `"unavailable"`.
fn indicator_text(v: &IndicatorValue) -> String {
    match v {
        IndicatorValue::Enabled(s) | IndicatorValue::Disabled(s) => s.clone(),
        IndicatorValue::Unavailable => "unavailable".to_owned(),
    }
}

// ---------------------------------------------------------------------------
// Wizard logo panel
// ---------------------------------------------------------------------------

/// Render the `WIZARD_SMALL` ASCII art inside a rounded bordered panel.
///
/// Mirrors the `render_logo` function from `umrs-ui/src/layout.rs`.
/// Lines are styled with `theme.wizard` (green).
fn render_wizard_logo(frame: &mut Frame, area: Rect, theme: &Theme) {
    let lines: Vec<Line<'_>> = WIZARD_SMALL
        .lines
        .iter()
        .map(|l| Line::from(Span::styled(*l, theme.wizard)))
        .collect();

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(theme.border);

    frame.render_widget(Paragraph::new(lines).block(block), area);
}

// ---------------------------------------------------------------------------
// Column headers
// ---------------------------------------------------------------------------

/// Column header text — aligned to match file entry rows.
///
/// The 4-char prefix accounts for: 2-char highlight symbol zone (`► ` or `  `)
/// that ratatui prepends to every `List` item.
const COL_HEADER_TEXT: &str =
    "    IOV  MODE        OWNER:GROUP      MODIFIED             NAME";

/// Render the `├───┤` horizontal divider using the T-connector characters.
///
/// Uses `├` (left T) + `─` fill + `┤` (right T) to connect with the
/// parent panel's border.  `parent_area` is the bordered panel so the
/// divider can be rendered at the correct x position to overlap the border
/// columns.
fn render_divider(frame: &mut Frame, area: Rect, parent_area: Rect, theme: &Theme) {
    // The divider spans the parent width (including border columns).
    let total = parent_area.width as usize;
    if total < 2 {
        return;
    }
    let fill_width = total.saturating_sub(2);
    let line_str = format!("├{}┤", "─".repeat(fill_width));
    let line = Line::from(Span::styled(line_str, theme.border));
    // Render at parent_area.x so it overlaps the border columns.
    let div_rect = Rect {
        x: parent_area.x,
        y: area.y,
        width: parent_area.width,
        height: 1,
    };
    frame.render_widget(Paragraph::new(vec![line]), div_rect);
}

/// Render the column header row (single line, no border).
///
/// Bold cyan text — a visual anchor between the dividers.
///
/// NIST SP 800-53 AU-3 — labeled columns improve audit record readability.
fn render_col_headers(frame: &mut Frame, area: Rect, _theme: &Theme) {
    let header_style = Style::default()
        .fg(Color::Cyan)
        .add_modifier(Modifier::BOLD);

    let line = Line::from(Span::styled(COL_HEADER_TEXT, header_style));
    frame.render_widget(Paragraph::new(vec![line]), area);
}

// ---------------------------------------------------------------------------
// Listing panel
// ---------------------------------------------------------------------------

/// Render the full-width file listing as a scrollable `List`.
///
/// Each row is one of:
/// - A **group header** row — expandable/collapsible branch showing the
///   SELinux type and MCS marking for a group of files.
/// - A **file/directory entry** row — IOV, mode, owner:group, mtime, name.
/// - A **sibling** row — cuddled sibling entry, indented and dimmed.
///
/// The selected row is highlighted using `theme.tab_active`.  Scroll is
/// managed by ratatui's `ListState` with `.select()`.
///
/// NIST SP 800-53 AC-4 — group headers make SELinux type and MCS marking
/// prominent, surfacing information-flow boundary context for every entry.
/// NIST SP 800-53 AU-3 — every entry row carries mode, owner, mtime, and name.
fn render_listing(frame: &mut Frame, area: Rect, state: &ViewerState, theme: &Theme) {
    let display = &state.tree.display_list;

    // Build items with blank spacer lines before group headers for visual
    // separation.  Track the mapping from display_list index to items index
    // so that `selected_index` highlights the correct item.
    let mut items: Vec<ListItem<'_>> = Vec::with_capacity(display.len().saturating_add(8));
    let mut selected_item_index: Option<usize> = None;

    for (i, entry) in display.iter().enumerate() {
        let node = state.tree.node_ref(&entry.path);
        let is_group_header = node.is_some_and(|n| !n.is_leaf() && entry.depth == 0);

        // Insert a blank line before group headers (but not before . and ..).
        // The `.` and `..` entries are leaves at depth 0; group headers are
        // branches at depth 0.
        if is_group_header {
            items.push(ListItem::new(Line::from("")));
        }

        if i == state.selected_index {
            selected_item_index = Some(items.len());
        }

        items.push(build_listing_item(entry, state, theme));
    }

    let block = Block::default().borders(Borders::NONE);

    let mut list_state = ListState::default();
    if let Some(idx) = selected_item_index {
        list_state.select(Some(idx));
    }

    let list = List::new(items)
        .block(block)
        .highlight_style(theme.tab_active)
        .highlight_symbol("► ");

    frame.render_stateful_widget(list, area, &mut list_state);
}

/// Build a single `ListItem` for one entry in the display list.
///
/// Dispatches to a group-header renderer or a file-entry renderer.
///
/// Only depth-0 branch nodes are SELinux group headers. Cuddled base files
/// are also branch nodes (they have sibling children) but live at depth > 0 —
/// those render as file entries so columns stay aligned.
fn build_listing_item<'a>(
    entry: &umrs_ui::viewer::tree::DisplayEntry,
    state: &'a ViewerState,
    theme: &'a Theme,
) -> ListItem<'a> {
    let node = state.tree.node_ref(&entry.path);

    match node {
        // Depth-0 branches = SELinux group headers.
        Some(node) if !node.is_leaf() && entry.depth == 0 => {
            build_group_header_item(node, theme)
        }
        // Everything else (leaves, cuddled base branches at depth > 0).
        Some(node) => build_file_entry_item(node, entry, theme),
        None => ListItem::new(Line::from(Span::styled(
            entry.label.clone(),
            theme.data_value,
        ))),
    }
}

/// Render a group header row (SELinux branch node).
///
/// Replicates the CLI `group_separator()` styling in ratatui spans:
///
/// - Normal groups: black-on-cyan type field, reverse-video transition
///   character (`\u{1FB6C}`), centered marking, underlined fill.
/// - `<restricted>` groups: dim italic + underline throughout.
///
/// The chevron (▼/▶) precedes the styled region to show expand/collapse state.
///
/// ## Original CLI sequence (preserved here as ratatui spans)
///
/// ```text
/// BLACK_ON_CYAN  type     REVERSE 🭬 RESET REVERSE 🭬 marking   RESET UNDERLINE 🭬 fill RESET
/// ```
///
/// NIST SP 800-53 AC-4 — SELinux type and marking are visually prominent;
/// the operator can immediately identify the information-flow boundary.
fn build_group_header_item<'a>(
    node: &umrs_ui::viewer::tree::TreeNode,
    _theme: &'a Theme,
) -> ListItem<'a> {
    let chevron = if node.expanded { CHEVRON_OPEN } else { CHEVRON_CLOSED };

    // The label is "{selinux_type} :: {marking}" — split it back out.
    let (selinux_type, marking) = node
        .label
        .split_once(" :: ")
        .unwrap_or((&node.label, ""));

    let is_restricted = selinux_type == "<restricted>";

    if is_restricted {
        // Restricted: dim italic + underline, like the CLI.
        let restricted_style = Style::default()
            .fg(Color::DarkGray)
            .add_modifier(Modifier::ITALIC | Modifier::UNDERLINED);
        let text = format!("{chevron}{selinux_type} :: {marking}");
        let fill = " ".repeat(80usize.saturating_sub(text.chars().count()));
        let span = Span::styled(format!("{text}{fill}"), restricted_style);
        return ListItem::new(Line::from(vec![span]));
    }

    // Replicate the CLI's banner/streamer effect with 🭬 transition triangles.
    //
    // The 🭬 character is a right-pointing triangle that, when colored with
    // fg=left_block_color and bg=right_block_color, creates the visual effect
    // of the left block's color pointing into the right block — like two
    // colored ribbons fitting into each other.
    //
    // Span sequence:
    //   [white on type_bg]   " {type:20} "     ← type block
    //   [fg=type_bg, bg=marking_bg]  "🭬"      ← triangle: type color into marking
    //   [light on marking_bg] "{marking:^20} "  ← marking block
    //   [fg=marking_bg, bg=default]  "🭬"       ← triangle: marking color into fill
    //   [underlined]          "{fill}"           ← underline rule to the right
    //
    // Placeholder gray tones — will be replaced by umrs-labels palette in Phase 5.

    let type_bg = Color::Rgb(70, 80, 90);
    let marking_bg = Color::Rgb(55, 65, 75);

    let chevron_span = Span::styled(
        chevron.to_owned(),
        Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
    );

    // Type block: white text on type_bg.
    let type_span = Span::styled(
        format!(" {selinux_type:20} "),
        Style::default().fg(Color::White).bg(type_bg),
    );

    // Transition 🭬: type_bg color pointing into marking_bg.
    let trans1 = Span::styled(
        ICON_BANNER,
        Style::default().fg(type_bg).bg(marking_bg),
    );

    // Marking block: lighter text on marking_bg (no leading 🭬 here).
    let marking_span = Span::styled(
        format!("{marking:^20} "),
        Style::default().fg(Color::Rgb(180, 190, 200)).bg(marking_bg),
    );

    // Transition 🭬: marking_bg color pointing into the default background.
    let trans2 = Span::styled(
        ICON_BANNER,
        Style::default().fg(marking_bg),
    );

    // Underlined fill extending to the right.
    let fill_width = 80usize.saturating_sub(2 + 22 + 1 + 20 + 1 + 1);
    let fill_span = Span::styled(
        " ".repeat(fill_width),
        Style::default().add_modifier(Modifier::UNDERLINED),
    );

    ListItem::new(Line::from(vec![
        chevron_span,
        type_span,
        trans1,
        marking_span,
        trans2,
        fill_span,
    ]))
}

/// Render a file or directory entry row.
///
/// Column layout (fixed widths, space-separated):
/// - IOV: 5 chars (including trailing spaces)
/// - Mode: 12 chars
/// - Owner:Group: 16 chars
/// - Mtime: 18 chars
/// - Name: remainder (no fixed width)
///
/// Sibling nodes (cuddled children) use the same column alignment as regular
/// entries but are rendered in dim style with a `[kind]` annotation after the
/// name.  No extra indent — columns must stay aligned across all rows.
///
/// NIST SP 800-53 AU-3 — mode bits, ownership, and mtime are required audit
/// record fields; all are present in every file entry row.
fn build_file_entry_item<'a>(
    node: &umrs_ui::viewer::tree::TreeNode,
    _entry: &umrs_ui::viewer::tree::DisplayEntry,
    theme: &'a Theme,
) -> ListItem<'a> {
    let meta = &node.metadata;

    // Parent navigation entry — special single-span row, not columnar.
    if meta.get("is_parent_nav").map(String::as_str) == Some("true") {
        let path_display = meta.get("path").map(String::as_str).unwrap_or("..");
        let text = format!("  {ICON_PARENT}  parent directory  ({path_display})");
        let style = Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::DIM);
        return ListItem::new(Line::from(Span::styled(text, style)));
    }

    // Siblings use the same indent as regular entries — no offset — so all
    // columns stay aligned.  The dim styling and [kind] annotation distinguish
    // siblings visually without breaking the tabular layout.
    let is_sibling = meta.contains_key("sibling_kind");
    let base_indent = "  ";

    // IOV — simplified to "---" in the TUI (full IOV rendering requires the
    // live `InodeSecurityFlags` which is not stored in metadata; enhancement
    // for a future phase when the flags are serialised into metadata).
    let iov = "--- ";

    // Mode — from metadata "mode" key (e.g., "-rwxr-xr-x").
    let mode = meta.get("mode").map(String::as_str).unwrap_or("----------");

    // Owner:Group — from metadata "owner" and "group" keys (numeric ids).
    let owner = meta.get("owner").map(String::as_str).unwrap_or("?");
    let grp = meta.get("group").map(String::as_str).unwrap_or("?");
    let uid_gid = format!("{owner}:{grp}");

    // Mtime — format from stored epoch seconds.
    let mtime = meta.get("mtime_secs").map_or_else(String::new, |s| {
        s.parse::<i64>().ok().and_then(|secs| {
            Local.timestamp_opt(secs, 0).single()
        }).map_or_else(String::new, |dt: DateTime<Local>| {
            dt.format("%Y-%m-%d %H:%M").to_string()
        })
    });

    // Name — from entry label (already includes trailing "/" for dirs).
    // Icon priority: encrypted > mountpoint > directory > symlink > plain.
    let is_dir = meta.get("is_dir").map(String::as_str) == Some("true");
    let file_type = meta.get("file_type").map(String::as_str).unwrap_or("-");
    let is_mountpoint = meta.get("is_mountpoint").map(String::as_str) == Some("true");
    let has_encryption = meta.get("has_encryption").map(String::as_str) == Some("true");
    let icon = if has_encryption {
        format!("{ICON_ENCRYPTED}  ")
    } else if is_mountpoint {
        format!("{ICON_MOUNT}  ")
    } else if is_dir {
        format!("{ICON_DIR} ")
    } else if file_type == "l" {
        format!("{ICON_SYMLINK} ")
    } else {
        "   ".to_owned()
    };

    // Name prefix and suffix for cuddled entries.
    //
    // Cuddled base files: chevron BEFORE the name to show expand/collapse,
    // followed by sibling summary (e.g., "▼ hawkey.log (3 rotations)").
    //
    // Sibling entries: └ prefix (lower-left box corner) before the name,
    // with [kind] suffix (e.g., "└ hawkey.log-20260301 [rotation]").
    let (name_prefix, name_suffix) = if is_sibling {
        let kind_str = meta
            .get("sibling_kind")
            .map(|k| format!(" [{k}]"))
            .unwrap_or_default();
        (format!("  {ICON_SIBLING} "), kind_str)
    } else if !node.is_leaf() {
        // Cuddled base: chevron before name + summary after.
        let chevron = if node.expanded { CHEVRON_OPEN } else { CHEVRON_CLOSED };
        let summary = meta.get("sibling_summary").map(String::as_str).unwrap_or("");
        let suffix = if summary.is_empty() {
            String::new()
        } else {
            format!(" ({summary})")
        };
        (chevron.to_owned(), suffix)
    } else {
        (String::new(), String::new())
    };

    // Name style: siblings use tab_inactive (dim); all other entries use
    // data_value (white).
    let name_style = if is_sibling {
        theme.tab_inactive
    } else {
        theme.data_value
    };

    // Composite multi-span line.  Each column is a separate styled span so
    // that IOV, mode, uid:gid, mtime, and name each carry distinct visual weight.
    //
    // IOV prefix is dim gray to recede behind the data columns.  Mode and name
    // use data_value (white).  UID:GID and mtime use secondary styles (key/inactive).
    let prefix_iov = Span::styled(
        format!("{base_indent}{iov}"),
        Style::default().fg(Color::DarkGray),
    );
    let mode_span = Span::styled(format!("{mode:<12} "), theme.data_value);
    let uid_gid_span = Span::styled(format!("{uid_gid:<16} "), theme.data_key);
    let mtime_span = Span::styled(format!("{mtime:<17} "), theme.tab_inactive);
    let name_span = Span::styled(
        format!("{icon}{name_prefix}{}{name_suffix}", node.label),
        name_style,
    );

    ListItem::new(Line::from(vec![
        prefix_iov,
        mode_span,
        uid_gid_span,
        mtime_span,
        name_span,
    ]))
}

// ---------------------------------------------------------------------------
// Search bar
// ---------------------------------------------------------------------------

/// Render the search input bar.
///
/// Displays the accumulated search query with a block cursor indicator.
/// Activated when `state.search_active` is `true`.
fn render_search_bar(frame: &mut Frame, area: Rect, query: &str, theme: &Theme) {
    let prompt = format!(" / {query}{ICON_CURSOR}");
    let line = Line::from(Span::styled(prompt, theme.data_value));

    let block = Block::default()
        .title(" Search ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(theme.border);

    frame.render_widget(Paragraph::new(vec![line]).block(block), area);
}

// ---------------------------------------------------------------------------
// Status bar
// ---------------------------------------------------------------------------

/// Render the status bar as a single line with blue background and white text.
///
/// Format: `ℹ {elapsed}ms · {file_count} files · {dir_count} dirs  {key_legend}`
///
/// Separated from the listing above by a ├───┤ T-connector divider.
///
/// NIST SP 800-53 AU-3 — scan timing and counts are always visible.
/// NIST SP 800-53 SA-5 — inline key legend reduces reliance on external docs.
fn render_status_bar(
    frame: &mut Frame,
    area: Rect,
    app: &DirViewerApp,
    theme: &Theme,
) {
    use umrs_core::console::symbols::icons;

    let status = app.status();
    let icon = match status.level {
        StatusLevel::Info => icons::INFO,
        StatusLevel::Ok => icons::CHECK,
        StatusLevel::Warn => icons::WARNING,
        StatusLevel::Error => icons::CROSS,
    };

    let status_text = format!(" {icon} {} ", status.text);
    let bar_width = area.width as usize;
    let legend_chars = KEY_LEGEND.chars().count();
    let status_chars = status_text.chars().count();
    let combined = status_chars.saturating_add(legend_chars);

    let padded = if combined <= bar_width {
        let pad = bar_width
            .saturating_sub(status_chars)
            .saturating_sub(legend_chars);
        format!("{status_text}{}{KEY_LEGEND}", " ".repeat(pad))
    } else if status_chars < bar_width {
        let pad = bar_width.saturating_sub(status_chars);
        format!("{status_text}{}", " ".repeat(pad))
    } else {
        status_text.chars().take(bar_width).collect()
    };

    // Blue background + bold white text — matches the original status bar style.
    let _ = theme; // theme.status_text is white+bold; we add the blue bg explicitly.
    let style = Style::default()
        .fg(Color::White)
        .bg(Color::Blue)
        .add_modifier(Modifier::BOLD);

    let line = Line::from(Span::styled(padded, style));
    frame.render_widget(Paragraph::new(line), area);
}

// ---------------------------------------------------------------------------
// Permission denied overlay
// ---------------------------------------------------------------------------

/// Render a centered "Permission Denied" modal overlay.
///
/// 75% screen width, double-line border, centered text with vertical padding.
/// Title "Permission Denied" appears centered on the top border line.
/// Blocks all input until dismissed with Enter or Esc.
///
/// `path` is the directory that could not be opened.
pub fn render_permission_denied(
    frame: &mut Frame,
    area: Rect,
    path: &str,
    _error_msg: &str,
    theme: &Theme,
) {
    use ratatui::layout::Alignment;

    // 75% of screen width, centered.
    let dialog_width = (area.width * 3) / 4;
    let dialog_height = 8_u16;
    let x = area.x + (area.width.saturating_sub(dialog_width)) / 2;
    let y = area.y + (area.height.saturating_sub(dialog_height)) / 2;

    let dialog_area = Rect { x, y, width: dialog_width, height: dialog_height };

    // Clear the area behind the dialog.
    frame.render_widget(Clear, dialog_area);

    let border_style = Style::default()
        .fg(Color::Red)
        .add_modifier(Modifier::BOLD);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(border_style)
        .title_alignment(Alignment::Center)
        .title(Span::styled(
            " Permission Denied ",
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        ));

    let _ = theme;
    let path_style = Style::default()
        .fg(Color::White)
        .add_modifier(Modifier::BOLD);
    let msg_style = Style::default().fg(Color::Rgb(180, 180, 180));
    let hint_style = Style::default().fg(Color::DarkGray);

    let lines = vec![
        Line::from(""),
        Line::from(Span::styled(path.to_owned(), path_style)).alignment(Alignment::Center),
        Line::from(""),
        Line::from(Span::styled(
            "You do not have access to this restricted directory.",
            msg_style,
        )).alignment(Alignment::Center),
        Line::from(""),
        Line::from(Span::styled(
            "Press Enter or Esc to dismiss",
            hint_style,
        )).alignment(Alignment::Center),
    ];

    let paragraph = Paragraph::new(lines).block(block);
    frame.render_widget(paragraph, dialog_area);
}
