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
use umrs_ui::text_fit::{display_width, truncate_left, truncate_right};
use umrs_ui::theme::Theme;
use umrs_ui::viewer::{ViewerApp as _, ViewerState};

use crate::viewer_app::DirViewerApp;

// ---------------------------------------------------------------------------
// Icons — sourced from `umrs_ui::icons`, the shared glyph catalog.
//
// Do not define new glyph constants in this file. Add them to
// `libs/umrs-ui/src/icons.rs` so every UMRS TUI tool uses the same
// visual language.  See that module for the full catalog.
// ---------------------------------------------------------------------------

use umrs_ui::icons::{
    ARROW_DOWN, ARROW_LEFT, ARROW_RIGHT, ARROW_UP, CHEVRON_CLOSED, CHEVRON_CUDDLE_CLOSED,
    CHEVRON_CUDDLE_OPEN, CHEVRON_OPEN, ICON_BANNER, ICON_CURSOR, ICON_DIR, ICON_ENCRYPTED,
    ICON_FLAG, ICON_MOUNT, ICON_PARENT, ICON_PLACEHOLDER, ICON_SIBLING, ICON_SYMLINK, PROMPT_ARROW,
};


// ---------------------------------------------------------------------------
// Layout constants
// ---------------------------------------------------------------------------

/// Width of the wizard logo panel: `WIZARD_SMALL.width` (15) + 2 border columns.
const LOGO_PANEL_WIDTH: u16 = 17;

/// Height of the header row. Compact: system info + directory in fewer lines.
const HEADER_HEIGHT: u16 = 9;

/// Height of the prompt bar when active (1 content row + 2 border rows).
/// Shared by the search bar and the "Go to" bar — only one is active at a time.
const SEARCH_BAR_HEIGHT: u16 = 3;


/// Compact key legend for the umrs-ls TUI status bar.
///
/// Only the essentials stay in the status bar — everything else (search,
/// goto, refresh, column reference) lives in the `?` help popup so the bar
/// never overflows on narrow terminals.
const KEY_LEGEND: &str = "  ↑↓:nav Enter:open ?:help q:quit ";

// ---------------------------------------------------------------------------
// HelpOverlay — modal help popup
// ---------------------------------------------------------------------------

/// Which tab the `?` help popup is showing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HelpTab {
    /// Keys and navigation reference.
    #[default]
    Navigation,
    /// Column legend (IOV markers, mode, name decorations).
    Columns,
}

/// State for the `?` help popup.
///
/// `active` toggles visibility; `tab` selects which page is shown.  Tab key
/// (or Left/Right arrows) cycles between tabs while the overlay is open.
#[derive(Debug, Default)]
pub struct HelpOverlay {
    pub active: bool,
    pub tab: HelpTab,
}

impl HelpOverlay {
    /// Open the overlay on the Navigation tab.
    pub const fn open(&mut self) {
        self.active = true;
        self.tab = HelpTab::Navigation;
    }

    /// Dismiss the overlay.
    pub const fn close(&mut self) {
        self.active = false;
    }

    /// Cycle to the next tab (wraps).
    pub const fn next_tab(&mut self) {
        self.tab = match self.tab {
            HelpTab::Navigation => HelpTab::Columns,
            HelpTab::Columns => HelpTab::Navigation,
        };
    }
}

// ---------------------------------------------------------------------------
// GotoBar — "Go to path" prompt state
// ---------------------------------------------------------------------------

/// State for the "Go to..." path-entry bar.
///
/// Activated by `Shift+G`.  When `active` is `true`, keystrokes append to
/// `query`; Enter resolves the path and navigates; Esc dismisses.
///
/// The bar shares layout space with the search bar — only one may be active
/// at a time.  Mutual exclusion is enforced by the event loop.
#[derive(Debug, Default)]
pub struct GotoBar {
    /// Whether the bar is currently accepting input.
    pub active: bool,
    /// Accumulated path query.
    pub query: String,
    /// Optional error message from the last failed resolution, shown in the
    /// bar until the operator edits the query or dismisses.
    pub error: Option<String>,
}

impl GotoBar {
    /// Activate the bar with an empty query.
    pub fn open(&mut self) {
        self.active = true;
        self.query.clear();
        self.error = None;
    }

    /// Dismiss the bar and clear any pending state.
    pub fn close(&mut self) {
        self.active = false;
        self.query.clear();
        self.error = None;
    }

    /// Append a printable character to the query, clearing any prior error.
    pub fn push_char(&mut self, ch: char) {
        self.query.push(ch);
        self.error = None;
    }

    /// Remove the last character from the query, clearing any prior error.
    pub fn pop_char(&mut self) {
        self.query.pop();
        self.error = None;
    }
}

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
    goto: &GotoBar,
) {
    // Search and Goto share the same bar area (mutually exclusive).
    let prompt_active = state.search_active || goto.active;
    let search_height = if prompt_active { SEARCH_BAR_HEIGHT } else { 0 };

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

    // ── Prompt bar (search or goto — mutually exclusive) ────────────────
    if state.search_active {
        render_search_bar(frame, search_area, &state.search_query, theme);
    } else if goto.active {
        render_goto_bar(frame, search_area, goto, theme);
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
    // Outer bordered block.  Inside, a vertical split separates the
    // "who and where the operator is" block from the "what directory
    // are we looking at" block — previously all one column, which read
    // as cluttered at narrow widths.
    //
    //   ┌──────────────────────────────────────────────────┐
    //   │  Host    : …        │  Type : unconfined_t       │  ← 4 rows
    //   │  OS      : …        │  Time : 2026-04-05 … -08:00│     posture
    //   │  SELinux : …        │                            │     + session
    //   │  FIPS    : …        │                            │
    //   │                                                  │  ← separator
    //   │  Directory : /…/long/path                        │  ← 2 rows
    //   │              drwxr-xr-x  root:root  etc_t  s0    │     dir info
    //   └──────────────────────────────────────────────────┘
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(theme.border);
    let inner = block.inner(area);
    frame.render_widget(block, area);

    // Vertical split: 4 rows (posture+session) · 1 blank · 2 rows (directory)
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),
            Constraint::Length(1),
            Constraint::Min(2),
        ])
        .split(inner);
    let [top_area, _gap_area, dir_area] = *rows else {
        return;
    };

    // Horizontal split inside the top row: system posture (left) + session (right).
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
        .split(top_area);
    let [left_area, right_area] = *cols else {
        return;
    };

    render_system_posture_lines(frame, left_area, ctx, theme);
    render_user_session_lines(frame, right_area, theme);
    render_directory_info_lines(frame, dir_area, app, theme);
}

/// Top-left of the header: system posture (Host, OS, SELinux, FIPS).
///
/// Labels are discovered from the data and padded algorithmically — the
/// longest label name drives the column width, and every row is formatted
/// with `{label:<max}` so colons align without any hand-counted spaces.
/// Rename a label, the padding recomputes; add a new row, the padding
/// recomputes; no silent misalignment.
///
/// Every value is right-truncated to fit the remaining column width.
/// Hostnames can be arbitrarily long FQDNs and OS strings carry an
/// architecture suffix — the operator has essentially no bound on these
/// fields, so the renderer must never trust the value to fit.
fn render_system_posture_lines(
    frame: &mut Frame,
    area: Rect,
    ctx: &HeaderContext,
    theme: &Theme,
) {
    // Each entry: (label, value, value_style).  The `value_style` is
    // either `theme.data_value` for plain text or an indicator style
    // for enabled/disabled/unavailable state fields.
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

    render_label_value_rows(frame, area, &rows, theme);
}

/// Top-right of the header: operator session info.
///
/// Four rows:
///
/// ```text
///  Tool   : umrs-ls v0.1.0
///  User   : jadams
///  Domain : unconfined_t
///  Level  : s0-s0:c0.c1023
/// ```
///
/// - **Tool** shows the binary name and version for at-a-glance confirmation
///   that the operator is running the expected tool release.
/// - **User** is `$USER` or the NSS-resolved username for the running uid.
///   The SELinux type (domain) has moved to its own **Domain** row.
/// - **Domain** is the `_t` component of the running process context from
///   `/proc/self/attr/current`.  Surfacing it separately from the username
///   reduces visual clutter while preserving the policy-relevant information.
/// - **Level** is the raw sensitivity level of the *running process* — not
///   necessarily the operator's clearance ceiling.  Under targeted policy
///   this is usually `s0-s0:c0.c1023`; under MLS it reflects the login level.
///   A future popup will disambiguate this from the user's full clearance range.
///
/// All values are right-truncated to the remaining column budget so
/// long usernames, long MLS ranges, or exotic domain types never overflow.
///
/// NIST SP 800-53 AU-3 — subject identity is a required audit record field;
/// surfacing it in every frame keeps the operator's mental model aligned with
/// the audit log.
/// NIST SP 800-53 IA-2 — visible identification of the running user and
/// process domain supports operator accountability.
fn render_user_session_lines(frame: &mut Frame, area: Rect, theme: &Theme) {
    // ── Username ──────────────────────────────────────────────────────
    // Prefer $USER; fall back to NSS resolution via getuid, then to the
    // raw uid if NSS is unavailable.
    let username = std::env::var("USER").unwrap_or_else(|_| {
        let uid = nix::unistd::Uid::current();
        nix::unistd::User::from_uid(uid)
            .ok()
            .flatten()
            .map_or_else(|| uid.as_raw().to_string(), |u| u.name)
    });

    // ── Process SELinux context: domain type + level ──────────────────
    // Domain (type) gets its own row; level gets its own row.
    let (ctx_type, ctx_level) = match umrs_selinux::utils::get_self_context() {
        Ok(sc) => {
            let t = sc.security_type().as_str().to_owned();
            let l = sc
                .level()
                .map_or_else(|| "-".to_owned(), |l| l.raw().to_owned());
            (t, l)
        }
        Err(_) => ("<unavailable>".to_owned(), "-".to_owned()),
    };

    let tool_value = format!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

    let rows: [(&str, String, Style); 4] = [
        ("Tool", tool_value, theme.data_value),
        ("User", username, theme.data_value),
        ("Domain", ctx_type, theme.data_value),
        ("Level", ctx_level, theme.data_value),
    ];

    render_label_value_rows(frame, area, &rows, theme);
}

/// Render a block of `Label : value` rows with algorithmically-computed
/// padding.
///
/// This is the single point of truth for header-row layout.  Given a
/// list of `(label, value, value_style)` triples and a render area, it:
///
/// 1. Finds the longest label name.
/// 2. Formats every row as `" {label:<max} : "` so colons align
///    vertically without any hand-counted spaces in the source.
/// 3. Computes the value budget from the area width and label width.
/// 4. Right-truncates every value through [`truncate_right`] so no
///    row overflows the column regardless of content.
///
/// Callers never hand-pad labels or hand-compute value budgets.  If a
/// label is renamed or a new row is added, the padding and budget
/// recompute automatically — no silent misalignment, no dead whitespace.
fn render_label_value_rows(
    frame: &mut Frame,
    area: Rect,
    rows: &[(&str, String, Style)],
    theme: &Theme,
) {
    // Discover the longest label; degenerate to 0 if the slice is empty.
    let max_label = rows.iter().map(|(l, ..)| display_width(l)).max().unwrap_or(0);

    // Label span layout: " {label:<max} : "
    //                    │└── leading space (1)
    //                    │    └── label, left-justified to max cells
    //                    │                    └── space · colon · space (3)
    let label_cell_width = 1 + max_label + 3;

    // Value budget = area width − label cell width − 1 cell safety margin.
    let value_budget = (area.width as usize)
        .saturating_sub(label_cell_width)
        .saturating_sub(1);

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

/// Bottom of the header: the current directory path and its metadata.
///
/// Rendered full-width (not inside the 55/45 column split) so the path
/// has the full header width to work with.  The path is left-truncated
/// with `…` when it overflows; the metadata line (mode, owner:group,
/// SELinux type, marking) is aligned under the path with a leading
/// indent matching the `Directory : ` label width.
fn render_directory_info_lines(
    frame: &mut Frame,
    area: Rect,
    app: &DirViewerApp,
    theme: &Theme,
) {
    let dir_label = " Directory : ";
    let mount_icon = if app.dir_meta().is_mountpoint {
        format!("{ICON_MOUNT} ")
    } else {
        String::new()
    };

    // Left-truncate the path to fit the full header width minus the
    // label and mount icon, with a small safety margin.
    let full_path = app.current_path().display().to_string();
    let label_width = display_width(dir_label);
    let icon_width = display_width(&mount_icon);
    let budget = (area.width as usize)
        .saturating_sub(label_width)
        .saturating_sub(icon_width)
        .saturating_sub(1);
    let shown_path = truncate_left(&full_path, budget);

    let dir_line = Line::from(vec![
        Span::styled(dir_label, theme.data_key),
        Span::styled(
            format!("{mount_icon}{shown_path}"),
            Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
        ),
    ]);

    let dm = app.dir_meta();
    // Indent matches the label width so the metadata aligns under the
    // path value.
    let indent = " ".repeat(label_width);
    let dir_meta_line = Line::from(Span::styled(
        format!(
            "{indent}{}  {}:{}  {}  {}",
            dm.mode, dm.owner, dm.group, dm.selinux_type, dm.marking,
        ),
        theme.data_key,
    ));

    frame.render_widget(Paragraph::new(vec![dir_line, dir_meta_line]), area);
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
/// The selected row is highlighted using `theme.list_selection` (a subtle
/// warm yellow by default).  Scroll is managed by ratatui's `ListState`
/// with `.select()`.
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
        .highlight_style(theme.list_selection)
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
///   character (`ICON_BANNER` — 🭬 U+1FB6C), centered marking, underlined fill.
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
            .add_modifier(Modifier::ITALIC);
        let text = format!("{chevron} {selinux_type:<19} :: {marking}");
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

    // Type block: white text on type_bg, left-justified with a leading
    // space so names read from the start of the column rather than being
    // centered (centering made short vs long type names look misaligned).
    let type_span = Span::styled(
        format!(" {selinux_type:<20} "),
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
    //let fill_width = 80usize.saturating_sub(2 + 22 + 1 + 20 + 1 + 1);
    //let fill_span = Span::styled(
        //" ".repeat(fill_width),
        //Style::default().add_modifier(Modifier::UNDERLINED),
    //);
    let fill_span = Span::styled(" ", Style::default());

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
#[expect(
    clippy::too_many_lines,
    reason = "columnar composition: each IOV/mode/owner/mtime/name/summary/chevron \
              span is expressed inline for readability; splitting into helpers would \
              fragment the column layout across multiple functions"
)]
fn build_file_entry_item<'a>(
    node: &umrs_ui::viewer::tree::TreeNode,
    _entry: &umrs_ui::viewer::tree::DisplayEntry,
    theme: &'a Theme,
) -> ListItem<'a> {
    let meta = &node.metadata;

    // Parent navigation entry — special single-span row, not columnar.
    // Rendered in bright cyan bold so the "go up one level" affordance
    // reads unmistakably as a navigation target, not as a data row.
    if meta.get("is_parent_nav").map(String::as_str) == Some("true") {
        let path_display = meta.get("path").map(String::as_str).unwrap_or("..");
        let text = format!("  {ICON_PARENT}  parent directory  ({path_display})");
        let style = Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD);
        return ListItem::new(Line::from(Span::styled(text, style)));
    }

    // Siblings use the same indent as regular entries — no offset — so all
    // columns stay aligned.  The dim styling and [kind] annotation distinguish
    // siblings visually without breaking the tabular layout.
    let is_sibling = meta.contains_key("sibling_kind");
    let base_indent = "  ";

    // IOV posture markers — read from metadata keys populated by
    // `tree_adapter::populate_entry_metadata`.  Each flag is rendered as a
    // separate styled span so the live state (red I, flag O, green V) is
    // visible without recomputing `InodeSecurityFlags` in the render path.
    //
    // NIST SP 800-53 AU-3, SI-7 — posture markers are audit-relevant.
    let iov_i = meta.contains_key("iov_i");
    // `iov_o` is a tiered string: "risk" or "warning". Absent means clear.
    let iov_o = meta.get("iov_o").map(String::as_str);
    let iov_v = meta.contains_key("iov_v");

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

    // Sibling row: `└` prefix before the name, `[kind]` suffix after.
    // Kept on the name span — styling is uniformly dim via `tab_inactive`.
    let sibling_prefix = if is_sibling {
        format!("  {ICON_SIBLING} ")
    } else {
        String::new()
    };
    let sibling_kind_suffix = if is_sibling {
        meta.get("sibling_kind")
            .map(|k| format!(" [{k}]"))
            .unwrap_or_default()
    } else {
        String::new()
    };

    // Cuddled-base row: name stays in normal (bright) style, then a dim-italic
    // `(N rotations, ...)` summary, then the expand/collapse chevron at the
    // end of the line.  Leaves render nothing extra.
    let is_cuddled_base = !is_sibling && !node.is_leaf();
    let cuddle_summary = if is_cuddled_base {
        meta.get("sibling_summary")
            .map(String::as_str)
            .filter(|s| !s.is_empty())
            .map(|s| format!("  ({s})"))
            .unwrap_or_default()
    } else {
        String::new()
    };
    let cuddle_chevron = if is_cuddled_base {
        if node.expanded {
            CHEVRON_CUDDLE_OPEN
        } else {
            CHEVRON_CUDDLE_CLOSED
        }
    } else {
        ""
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
    let mut spans: Vec<Span<'a>> = Vec::with_capacity(11);
    spans.push(Span::styled(
        base_indent.to_owned(),
        Style::default().fg(Color::DarkGray),
    ));
    append_iov_spans(&mut spans, iov_i, iov_o, iov_v);

    spans.push(Span::styled(format!("{mode:<12} "), theme.data_value));
    spans.push(Span::styled(format!("{uid_gid:<16} "), theme.data_key));
    spans.push(Span::styled(format!("{mtime:<17} "), theme.tab_inactive));
    spans.push(Span::styled(
        format!("{icon}{sibling_prefix}{}{sibling_kind_suffix}", node.label),
        name_style,
    ));
    if is_cuddled_base {
        spans.push(Span::styled(
            cuddle_summary,
            Style::default()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::ITALIC | Modifier::DIM),
        ));
        spans.push(Span::styled(
            format!(" {cuddle_chevron}"),
            Style::default().fg(Color::DarkGray),
        ));
    }
    ListItem::new(Line::from(spans))
}

/// Append the three IOV posture spans (`I`, `O`, `V`) plus a trailing space
/// to `spans`.  Lit flags use color + optional glyph; clear flags render as
/// dim `-`.  The `O` marker uses `ICON_FLAG` (⚑) when a Risk-kind security
/// observation is present, making posture issues visually prominent at
/// scan-speed without disturbing the I/V column alignment.
///
/// NIST SP 800-53 AU-3, SI-7 — posture markers are audit-relevant.
fn append_iov_spans(
    spans: &mut Vec<Span<'_>>,
    iov_i: bool,
    iov_o: Option<&str>,
    iov_v: bool,
) {
    // Placeholder for a clear slot: shared `ICON_PLACEHOLDER` (· U+00B7
    // MIDDLE DOT).  Rendered dark-gray + DIM so it stays present for column
    // alignment without competing with lit markers.
    let placeholder = Span::styled(
        ICON_PLACEHOLDER,
        Style::default()
            .fg(Color::DarkGray)
            .add_modifier(Modifier::DIM),
    );

    spans.push(if iov_i {
        Span::styled("I", Style::default().fg(Color::Red))
    } else {
        placeholder.clone()
    });
    spans.push(match iov_o {
        Some("risk") => Span::styled(
            ICON_FLAG.to_owned(),
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        ),
        Some("warning") => Span::styled(
            ICON_FLAG.to_owned(),
            Style::default().fg(Color::Yellow),
        ),
        _ => placeholder.clone(),
    });
    spans.push(if iov_v {
        Span::styled("V", Style::default().fg(Color::Green))
    } else {
        placeholder
    });
    spans.push(Span::raw(" "));
}

// ---------------------------------------------------------------------------
// Search bar
// ---------------------------------------------------------------------------

/// Render the search input bar.
///
/// Displays the accumulated search query with a block cursor indicator.
/// Activated when `state.search_active` is `true`.
fn render_search_bar(frame: &mut Frame, area: Rect, query: &str, theme: &Theme) {
    // Use the same `➜` prompt glyph as the Go to bar so the two prompts
    // read as visually related (both are "type here" inputs).
    let prompt = format!(" {PROMPT_ARROW} {query}{ICON_CURSOR}");
    let line = Line::from(Span::styled(prompt, theme.data_value));

    let block = Block::default()
        .title(" Search ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(theme.border);

    frame.render_widget(Paragraph::new(vec![line]).block(block), area);
}

// ---------------------------------------------------------------------------
// Go to bar
// ---------------------------------------------------------------------------

/// Render the "Go to path" input bar.
///
/// Displays the accumulated path query with a block cursor indicator. If the
/// previous resolution failed (e.g., path does not exist), a dim red error
/// suffix is appended so the operator sees the problem in context.
///
/// Activated when `GotoBar::active` is `true`.
fn render_goto_bar(frame: &mut Frame, area: Rect, goto: &GotoBar, theme: &Theme) {
    let mut spans: Vec<Span<'_>> = Vec::with_capacity(3);
    spans.push(Span::styled(
        format!(" {PROMPT_ARROW} {query}{ICON_CURSOR}", query = goto.query),
        theme.data_value,
    ));
    if let Some(err) = goto.error.as_deref() {
        spans.push(Span::styled(
            format!("   {err}"),
            Style::default().fg(Color::Red).add_modifier(Modifier::DIM),
        ));
    }

    let block = Block::default()
        .title(" Go to ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(theme.border);

    frame.render_widget(Paragraph::new(vec![Line::from(spans)]).block(block), area);
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
            "Press \u{23CE} or \u{238b}\u{20e3} to dismiss",
            hint_style,
        )).alignment(Alignment::Center),
    ];

    let paragraph = Paragraph::new(lines).block(block);
    frame.render_widget(paragraph, dialog_area);
}

// ---------------------------------------------------------------------------
// Help overlay (?)
// ---------------------------------------------------------------------------

/// Render the `?` help popup.
///
/// Centered modal, ~70% width, with a two-tab header (Navigation / Columns).
/// `Tab` or `←/→` switches tabs; `?` or `Esc` dismisses.  All background input is blocked while 
/// the overlay is open.
pub fn render_help_overlay(frame: &mut Frame, area: Rect, overlay: &HelpOverlay, theme: &Theme) {
    use ratatui::layout::Alignment;

    let dialog_width = (area.width * 7) / 10;
    let dialog_height = 22_u16.min(area.height.saturating_sub(2));
    let x = area.x + (area.width.saturating_sub(dialog_width)) / 2;
    let y = area.y + (area.height.saturating_sub(dialog_height)) / 2;
    let dialog_area = Rect { x, y, width: dialog_width, height: dialog_height };

    frame.render_widget(Clear, dialog_area);

    let border_style = Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD);
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(border_style)
        .title_alignment(Alignment::Center)
        .title(Span::styled(
            " Help ",
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        ));

    let inner = block.inner(dialog_area);
    frame.render_widget(block, dialog_area);

    // Inner split: tab header (1 row) + divider (1 row) + body (rest) + hint (1 row)
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(inner);
    let [tab_row, div_row, body_row, hint_row] = *rows else {
        return;
    };

    // Tab header: two pills, active one highlighted.
    let active = Style::default()
        .fg(Color::Black)
        .bg(Color::Cyan)
        .add_modifier(Modifier::BOLD);
    let inactive = Style::default().fg(Color::DarkGray);
    let (nav_style, col_style) = match overlay.tab {
        HelpTab::Navigation => (active, inactive),
        HelpTab::Columns => (inactive, active),
    };
    let tab_line = Line::from(vec![
        Span::raw("  "),
        Span::styled(" Navigation ", nav_style),
        Span::raw("  "),
        Span::styled(" Columns ", col_style),
    ]);
    frame.render_widget(Paragraph::new(tab_line), tab_row);

    // Divider under the tabs.
    let div_text: String = "─".repeat(div_row.width as usize);
    frame.render_widget(
        Paragraph::new(Line::from(Span::styled(div_text, theme.border))),
        div_row,
    );

    // Body lines per tab.
    let body_lines = match overlay.tab {
        HelpTab::Navigation => help_navigation_lines(),
        HelpTab::Columns => help_columns_lines(),
    };
    frame.render_widget(Paragraph::new(body_lines), body_row);

    // Hint row at the bottom.
    let hint_style = Style::default().fg(Color::DarkGray);
    let hint = Line::from(Span::styled(
        format!("  Tab / {ARROW_LEFT}{ARROW_RIGHT}: switch tab    ? or Esc: close  "),
        hint_style,
    ));
    frame.render_widget(Paragraph::new(hint), hint_row);
}

/// Key:description lines for the Navigation tab.
fn help_navigation_lines<'a>() -> Vec<Line<'a>> {
    let key_style = Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD);
    let text_style = Style::default().fg(Color::White);

    let row = |k: String, d: &'a str| -> Line<'a> {
        Line::from(vec![
            Span::raw("  "),
            Span::styled(format!("{k:<14}"), key_style),
            Span::styled(d, text_style),
        ])
    };

    vec![
        Line::from(""),
        row(format!("{ARROW_UP} {ARROW_DOWN}"), "Move selection up / down"),
        row("PgUp / PgDn".to_owned(), "Page up / down (10 rows)"),
        row(
            format!("{ARROW_LEFT} {ARROW_RIGHT}"),
            "Collapse / expand a group",
        ),
        row("Enter".to_owned(), "Open directory, toggle group expand"),
        row(String::new(), ""),
        row("/".to_owned(), "Search — filters the current listing"),
        row("G".to_owned(), "Go to path — type a path, Tab to complete"),
        row("r".to_owned(), "Refresh (also clears an active search)"),
        row(String::new(), ""),
        row("?".to_owned(), "This help"),
        row("q".to_owned(), "Quit"),
    ]
}

/// Column legend for the Columns tab.
fn help_columns_lines<'a>() -> Vec<Line<'a>> {
    let key_style = Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD);
    let text_style = Style::default().fg(Color::White);
    let dim = Style::default().fg(Color::DarkGray);

    let row = |k: String, d: &'a str| -> Line<'a> {
        Line::from(vec![
            Span::raw("  "),
            Span::styled(format!("{k:<14}"), key_style),
            Span::styled(d, text_style),
        ])
    };

    vec![
        Line::from(""),
        Line::from(Span::styled(
            "  IOV posture column — three single-character slots",
            text_style,
        )),
        Line::from(""),
        Line::from(vec![
            Span::raw("    "),
            Span::styled("I", Style::default().fg(Color::Red)),
            Span::styled("  immutable flag set (chattr +i)", text_style),
        ]),
        Line::from(vec![
            Span::raw("    "),
            Span::styled(
                ICON_FLAG,
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
            Span::styled("  security observation (Risk)", text_style),
        ]),
        Line::from(vec![
            Span::raw("    "),
            Span::styled(ICON_FLAG, Style::default().fg(Color::Yellow)),
            Span::styled("  security observation (Warning)", text_style),
        ]),
        Line::from(vec![
            Span::raw("    "),
            Span::styled("V", Style::default().fg(Color::Green)),
            Span::styled("  IMA hash present (integrity signed)", text_style),
        ]),
        Line::from(vec![
            Span::raw("    "),
            Span::styled(ICON_PLACEHOLDER, dim.add_modifier(Modifier::DIM)),
            Span::styled("  slot clear (evaluated, no finding)", text_style),
        ]),
        Line::from(""),
        row("mode".to_owned(), "Unix permissions (ls -l style)"),
        row(
            "owner:group".to_owned(),
            "Resolved via NSS (falls back to uid:gid)",
        ),
        row("mtime".to_owned(), "Last modification time"),
        Line::from(""),
        Line::from(Span::styled("  Name decorations", text_style)),
        Line::from(""),
        row(ICON_ENCRYPTED.to_owned(), "Encrypted directory (LUKS / fscrypt)"),
        row(ICON_MOUNT.to_owned(), "Mount point"),
        row(ICON_DIR.to_owned(), "Directory"),
        row(ICON_SYMLINK.to_owned(), "Symbolic link"),
        row(format!("{ICON_PARENT} .."), "Parent directory (navigation)"),
        row(
            format!("{ICON_SIBLING} name"),
            "Cuddled sibling (e.g. rotated log)",
        ),
        row(
            format!("{CHEVRON_CUDDLE_CLOSED} (N)"),
            "Collapsed cuddle group (N siblings)",
        ),
    ]
}
