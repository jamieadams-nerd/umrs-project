// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//! # umrs-ls — Security-Focused Directory Listing
//!
//! Displays directory entries enriched with `SELinux` security context, MCS
//! marking, POSIX ownership, and security observations. Output is grouped by
//! `(SELinux type, security marking)` — the type and marking appear in group
//! headers only and are not repeated for every row.
//!
//! By default, related files (rotations, signatures, checksums, backups) are
//! *cuddled* under their base file — one summary line replaces the individual
//! sibling rows. Pass `--flat` to disable cuddling and show every entry on
//! its own row.
//!
//! ## Output Modes
//!
//! - **TUI** (default when stdout is a TTY): interactive directory browser with
//!   tree navigation, search, and directory traversal.
//! - **CLI** (`--cli` or non-TTY stdout): human-readable columnar listing.
//!   Use `--cli` to force text mode even on a TTY (e.g., `umrs-ls --cli | less`).
//! - **JSON** (`--json`): machine-readable grouped output; always bypasses TUI.
//!
//! ## Usage
//!
//! ```text
//! umrs-ls [PATH] [--color] [--no-iov] [--no-mtime]
//!         [--with-size] [--with-inode] [--flat] [--cli] [--json]
//! ```
//!
//! Default path is the current directory. Color output is off by default.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AC-3**: Access Enforcement — every entry displays the
//!   `SELinux` label used in access decisions.
//! - **NIST SP 800-53 AC-4**: Information Flow Enforcement — MCS markings and
//!   security observations surface information flow boundary violations.
//! - **NIST SP 800-53 AU-3**: Audit Record Content — operator-visible output
//!   includes all identity, label, and observation fields required for audit.
//! - **NSA RTB RAIN**: Non-Bypassability and TOCTOU safety — all directory
//!   reads are fd-anchored via `SecureDirent`.

#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![deny(clippy::unwrap_used)]
// Lint suppressions — rationale mirrors umrs-selinux policy:
//   doc_markdown:       backtick-wrapping every field name in prose is disruptive
//   missing_errors_doc: # Errors sections on every Result fn adds noise
//   missing_panics_doc: # Panics sections for unreachable paths add no value
//   option_if_let_else: explicit if/else preferred over .map_or_else() chains
//   map_unwrap_or:      multi-step find/map/unwrap_or chains are clear as written
//   format_push_string: write! over push_str(&format!()) is micro-opt; readability wins
#![allow(clippy::option_if_let_else)]
#![allow(clippy::map_unwrap_or)]
// format_push_string: write! over push_str(&format!()) is a micro-opt; readability wins here.
// Rationale mirrors Cargo.toml [lints.clippy] entry — needed because xtask passes -D warnings
// which overrides Cargo.toml lint table entries.
#![allow(clippy::format_push_string)]

use std::borrow::Cow;
use std::fmt::Write;
use std::io::{self, IsTerminal as _};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use serde::Serialize;

use chrono::{DateTime, Local};
use umrs_core::human::sizefmt::{SizeBase, auto_format as fmt_size};
use umrs_core::i18n;
use umrs_ls::grouping::{FileGroup, SiblingKind, aggregate_size, group_entries, sibling_summary};
use umrs_ls::viewer_app::DirViewerApp;
use umrs_selinux::ObservationKind;
use umrs_selinux::mcs::colors::{
    ContextComponents, Rgb, SeColorConfig, load_secolors_cached, resolve_colors,
};
use umrs_selinux::secure_dirent::{FileType, InodeSecurityFlags};
use umrs_selinux::utils::dirlist::{
    Column, ColumnSet, DirGroup, GroupKey, ListEntry, list_directory,
};
use umrs_ui::indicators::build_header_context;
use umrs_ui::keymap::{Action, KeyMap};
use umrs_ui::theme::Theme;
use umrs_ui::viewer::{ViewerApp as _, ViewerState};
use umrs_ls::tui_render::{render_dir_browser, render_permission_denied};

// ============================================================================
// JSON output types
//
// Lightweight, serialisable mirror of the grouped structure.  Only the fields
// that are meaningful in a machine-readable context are included — full
// SecureDirent is not serialised (it contains OS-level primitives that would
// need custom implementations).
// ============================================================================

/// A sibling entry in JSON output.
#[derive(Serialize)]
struct JsonSibling {
    name: String,
    kind: &'static str,
    size: u64,
}

/// A file group in JSON output: base entry plus its siblings.
#[derive(Serialize)]
struct JsonFileGroup {
    base_name: String,
    base_size: u64,
    siblings: Vec<JsonSibling>,
    aggregate_size: u64,
}

/// One SELinux-type+marking group in JSON output.
#[derive(Serialize)]
struct JsonDirGroup {
    selinux_type: String,
    marking: String,
    file_groups: Vec<JsonFileGroup>,
}

/// Root JSON document produced by `--json`.
#[derive(Serialize)]
struct JsonListing {
    path: String,
    groups: Vec<JsonDirGroup>,
    elapsed_us: u64,
}

const TERM_WIDTH: usize = 100;
const ROW_INDENT: &str = "  "; // 2-space left indent on every row
const NAME_PREFIX: &str = "   "; // 3-char icon zone before filename

const BOLD_RED: &str = "\x1b[1;31m";
const DIM_ITALIC: &str = "\x1b[2;3m";
const DIM: &str = "\x1b[2m";
const GREEN: &str = "\x1b[32m";
const RED: &str = "\x1b[31m";
const RESET: &str = "\x1b[0m";
const UNDERLINE: &str = "\x1b[4m";
const REVERSE: &str = "\x1b[7m";
const BLACK_ON_CYAN: &str = "\x1b[30;46m";

// Runtime display configuration — colour switch, mount symbols, and loaded
// secolor config.
//
// Pass `&DisplayConfig` down to every rendering function so all display
// decisions are centralised here rather than scattered across flags.
//  -  use_color - When `false` (default) all ANSI escape codes are suppressed.
//  -  mount_symbol - Unicode symbol shown in the icon zone for ordinary mount points.
//  -  plaindir_symbol - Unicode symbol for plain folder.
//  -  encrypted_symbol - Unicode symbol shown in the icon zone for encrypted mount points.
//  -  secolor - Loaded secolor.conf — `None` if absent or unreadable.
//
struct DisplayConfig {
    use_color: bool,
    mount_symbol: &'static str,
    encrypted_symbol: &'static str,
    plaindir_symbol: &'static str,
    secolor: Option<SeColorConfig>,
}

impl DisplayConfig {
    fn build(use_color: bool) -> Self {
        let secolor = if use_color {
            load_secolors_cached(Path::new("/etc/selinux/targeted/secolor.conf")).ok()
        } else {
            None
        };

        Self {
            use_color,
            mount_symbol: "\u{26C1}",
            plaindir_symbol: "\u{1F4C1}",
            encrypted_symbol: "\u{1F512}", // Lock icon
            secolor,
        }
    }
}

fn main() -> io::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn"))
        .format_timestamp(None)
        .init();

    i18n::init("umrs-ls");

    let args: Vec<String> = std::env::args().collect();

    // First non-flag argument after the binary name is the target path.
    let target =
        args.iter().skip(1).find(|a| !a.starts_with("--")).map(String::as_str).unwrap_or(".");

    let json_mode = args.contains(&"--json".to_owned());
    let cli_mode = args.contains(&"--cli".to_owned());
    let flat_mode = args.contains(&"--flat".to_owned());

    // Mode selection:
    //   --json              → JSON output, always (no TUI, no ANSI table)
    //   --cli or non-TTY   → CLI columnar text output
    //   otherwise          → TUI interactive viewer
    if json_mode {
        return run_json(&args, target);
    }

    if cli_mode || !io::stdout().is_terminal() {
        return run_cli(&args, target, flat_mode);
    }

    run_tui(target, flat_mode)
}

// ============================================================================
// JSON output path
// ============================================================================

/// Emit a JSON listing for `target` and return.
fn run_json(args: &[String], target: &str) -> io::Result<()> {
    // flat_mode has no effect on JSON output but we accept the flag for
    // forward-compatibility — callers may pass it without knowing the mode.
    let _ = args; // consumed only for flag detection in main()
    let listing = list_directory(Path::new(target))?;
    emit_json(&listing.groups, &listing.path.display().to_string(), listing.elapsed_us)
}

// ============================================================================
// CLI (non-interactive) output path
// ============================================================================

/// Render the directory listing to stdout as a formatted columnar table.
///
/// This is the full existing CLI rendering path, extracted from `main()` so
/// that the TUI path can branch at the top of `main()` without touching any
/// of the rendering logic.
///
/// NIST SP 800-53 AU-3 — all identity, label, and observation fields required
/// for audit are included in the tabular output.
fn run_cli(args: &[String], target: &str, flat_mode: bool) -> io::Result<()> {
    // SelinuxType and Marking appear in the group header — omit from rows.
    let mut cols = ColumnSet::default().without(Column::SelinuxType).without(Column::Marking);

    if args.contains(&"--no-iov".to_owned()) {
        cols = cols.without(Column::Iov);
    }
    if args.contains(&"--no-mtime".to_owned()) {
        cols = cols.without(Column::Mtime);
    }
    if args.contains(&"--with-size".to_owned()) {
        cols = cols.with(Column::Size);
    }
    if args.contains(&"--with-inode".to_owned()) {
        cols = cols.with(Column::Inode);
    }

    let use_color = args.contains(&"--color".to_owned());
    let cfg = DisplayConfig::build(use_color);

    let listing = list_directory(Path::new(target))?;

    // Pre-scan column widths across all groups and entries.
    let widths = compute_widths(&listing.groups, &cols, &cfg);

    // Header row + rule.
    print_header(&cols, &widths);
    println!("{}", "=".repeat(TERM_WIDTH));

    // Groups.
    let mut total_entries = 0usize;
    let mut total_file_groups = 0usize;

    for group in &listing.groups {
        println!(); // Separate every group
        println!("{}", group_separator(&group.key, &cfg));

        if flat_mode {
            // Traditional flat listing — one row per entry.
            for entry in &group.entries {
                print_row(entry, &group.key, &cols, &widths, &cfg);
                total_entries += 1;
                total_file_groups += 1;
            }
        } else {
            // Cuddled view — group related files under their base.
            let file_groups = group_entries(&group.entries);
            total_file_groups += file_groups.len();
            for fg in &file_groups {
                print_row(&fg.base, &group.key, &cols, &widths, &cfg);
                total_entries += 1;
                if !fg.siblings.is_empty() {
                    total_entries += fg.siblings.len();
                    print_cuddle_line(fg, &cfg);
                }
            }
        }
    }

    // Access-denied summary.
    if !listing.access_denied.is_empty() {
        println!();
        let n = listing.access_denied.len();
        // Translate the static label, then append the count in parentheses.
        let label = format!("{} ({n}) ", i18n::tr("access denied"));
        let fill = "=".repeat(TERM_WIDTH.saturating_sub(label.len()));
        println!("{label}{fill}");
        for name in &listing.access_denied {
            println!("{ROW_INDENT}{name}");
        }
    }

    // Timing footer.
    println!();
    if flat_mode {
        println!(
            "{total_entries} entries  {}  {} groups  {} µs",
            listing.path.display(),
            listing.groups.len(),
            listing.elapsed_us,
        );
    } else {
        println!(
            "{total_entries} entries  {}  {} file groups  {} SELinux groups  {} µs",
            listing.path.display(),
            total_file_groups,
            listing.groups.len(),
            listing.elapsed_us,
        );
    }

    Ok(())
}

// ============================================================================
// TUI interactive viewer path
// ============================================================================

/// Run the interactive TUI directory browser.
///
/// Enters the ratatui alternate screen, runs the event loop, then restores
/// the terminal on exit (clean or error).
///
/// # Key bindings
///
/// | Key | Action |
/// |---|---|
/// | q / Esc | Quit |
/// | Up / k | Move selection up |
/// | Down / j | Move selection down |
/// | Right / l | Expand selected node |
/// | Left / h | Collapse selected node |
/// | Enter | Navigate into directory, or toggle expand on non-directory nodes |
/// | Space | Expand/toggle |
/// | Backspace | Navigate to parent in tree hierarchy |
/// | / | Activate search/filter |
/// | r | Refresh (re-scan current directory) |
/// | `PageUp` / `PageDown` | Page navigation |
///
/// NIST SP 800-53 AC-3 — navigation is read-only; no directory entries are
/// created, deleted, or modified through the viewer interface.
/// NIST SP 800-53 AU-3 — the viewer header carries tool identity, data source
/// path, and entry counts on every rendered frame.
#[expect(clippy::too_many_lines, reason = "TUI event loop is inherently sequential; splitting would scatter the state machine")]
fn run_tui(target: &str, _flat_mode: bool) -> io::Result<()> {
    // Canonicalize the target path so the header always shows an absolute path.
    let path = std::fs::canonicalize(target)?;

    // Construct the viewer app — performs the initial list_directory scan.
    let mut app = DirViewerApp::scan(&path)?;

    // Create viewer state and load the initial tree.
    let mut state = ViewerState::new(app.tabs().len());
    if let Some(tree) = app.initial_tree() {
        state.load_tree(tree);
    }

    // Build the keymap.  Start from defaults, then re-bind Left/Right to
    // Expand/Collapse (overriding the default NextTab/PrevTab bindings —
    // umrs-ls has only one tab so tab navigation is unused).
    let mut keymap = KeyMap::default();
    keymap.bind(
        crossterm::event::KeyEvent::new(KeyCode::Right, KeyModifiers::NONE),
        Action::Expand,
    );
    keymap.bind(
        crossterm::event::KeyEvent::new(KeyCode::Left, KeyModifiers::NONE),
        Action::Collapse,
    );
    // Vim users: l = expand, h = collapse.
    keymap.bind(
        crossterm::event::KeyEvent::new(KeyCode::Char('l'), KeyModifiers::NONE),
        Action::Expand,
    );
    keymap.bind(
        crossterm::event::KeyEvent::new(KeyCode::Char('h'), KeyModifiers::NONE),
        Action::Collapse,
    );

    let theme = Theme::default();

    // Build the header context once at startup.  OS name is read from
    // /etc/os-release.  build_header_context reads all kernel security
    // indicators via provenance-verified SecureReader paths.
    let os_name = umrs_ls::viewer_app::read_os_name();
    let ctx = build_header_context(
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        os_name,
    );

    // Enter the alternate screen and raw mode.
    let mut terminal = ratatui::init();

    // Permission denied overlay — (path, error_message) shown as a modal.
    // `None` = no overlay.  `Some(...)` = overlay is open, blocks all input
    // except Enter/Esc to dismiss.
    let mut nav_error: Option<(String, String)> = None;

    // Event loop — 100 ms poll timeout keeps the TUI snappy without busy-waiting.
    loop {
        if let Err(e) = terminal.draw(|f| {
            render_dir_browser(f, f.area(), &app, &state, &ctx, &theme);
            // Permission denied overlay — rendered on top when present.
            if let Some((ref path, ref msg)) = nav_error {
                render_permission_denied(f, f.area(), path, msg, &theme);
            }
        }) {
            log::error!("terminal draw error: {e}");
            break;
        }

        match event::poll(Duration::from_millis(100)) {
            Ok(true) => match event::read() {
                Ok(Event::Key(key)) => {
                    // When the permission overlay is open, only Enter/Esc dismiss it.
                    // All other input is consumed and ignored.
                    if nav_error.is_some() {
                        match key.code {
                            KeyCode::Enter | KeyCode::Esc => {
                                nav_error = None;
                            }
                            _ => {}
                        }
                        continue;
                    }

                    // Search mode: character input goes to the query buffer.
                    if state.search_active {
                        match key.code {
                            KeyCode::Char(ch) => {
                                state.push_search_char(ch);
                                continue;
                            }
                            KeyCode::Backspace => {
                                state.pop_search_char();
                                continue;
                            }
                            KeyCode::Esc => {
                                let _ = state.handle_action(Action::DialogCancel);
                                continue;
                            }
                            KeyCode::Enter => {
                                let _ = state.handle_action(Action::DialogConfirm);
                                continue;
                            }
                            _ => {}
                        }
                    }

                    if let Some(action) = keymap.lookup(&key) {
                        match action {
                            Action::Refresh => {
                                handle_refresh(&mut app, &mut state);
                            }
                            Action::DialogConfirm => {
                                // Enter: navigate into a directory, or toggle
                                // expand/collapse on non-directory nodes.
                                handle_enter(&mut app, &mut state, &mut nav_error);
                            }
                            _ => {
                                let _ = state.handle_action(action);
                            }
                        }
                    }
                }
                // ratatui handles resize internally; all other events ignored
                Ok(_) => {}
                Err(e) => {
                    log::warn!("event read error: {e}");
                }
            },
            Ok(false) => {}
            Err(e) => {
                log::warn!("event poll error: {e}");
            }
        }

        if state.should_quit {
            break;
        }
    }

    // Restore the terminal regardless of how the loop exited.
    ratatui::restore();
    Ok(())
}

// ============================================================================
// TUI navigation helpers
// ============================================================================

/// Handle an Enter keypress in the TUI viewer.
///
/// If the selected node is a directory, navigate into it (re-scan and load the
/// new tree).  If it is a file or group header, toggle expand/collapse instead.
/// On navigation error the display stays on the current listing — no crash, no
/// silent state corruption.
///
/// NIST SP 800-53 AC-3 — navigation is strictly read-only; this function never
/// creates, modifies, or deletes directory entries.
fn handle_enter(app: &mut DirViewerApp, state: &mut ViewerState, nav_error: &mut Option<(String, String)>) {
    let Some(entry) = state.tree.display_list.get(state.selected_index) else {
        return;
    };
    let path = entry.path.clone();
    let Some(node) = state.tree.node_ref(&path) else {
        return;
    };

    // Only navigate when the node represents a directory.
    if node.metadata.get("is_dir").map(String::as_str) != Some("true") {
        // File or group header: delegate to the tree's expand/collapse toggle.
        let _ = state.handle_action(Action::Expand);
        return;
    }

    let name = node.metadata.get("name").map(String::as_str).unwrap_or("");

    let new_path: PathBuf = if name == ".." {
        app.current_path()
            .parent()
            .map(Path::to_path_buf)
            .unwrap_or_else(|| app.current_path().to_path_buf())
    } else {
        app.current_path().join(name)
    };

    match app.navigate_to(&new_path) {
        Ok(tree) => {
            state.load_tree(tree);
        }
        Err(e) => {
            // Show an error dialog so the operator sees the failure and must
            // acknowledge it.  The previous listing remains valid.
            // No log::warn! here — stderr output corrupts the TUI display.
            // The error is shown to the operator via the modal overlay.
            *nav_error = Some((
                new_path.display().to_string(),
                e.to_string(),
            ));
        }
    }
}

/// Re-scan the current directory and reload the tree.
///
/// Called when the user presses `r` (Refresh).  On scan error the display
/// stays on the previous listing.
///
/// NIST SP 800-53 AU-3 — re-scan updates the status bar timing so the
/// operator can confirm the listing is current.
fn handle_refresh(app: &mut DirViewerApp, state: &mut ViewerState) {
    let path = app.current_path().to_path_buf();
    match app.navigate_to(&path) {
        Ok(tree) => {
            state.load_tree(tree);
        }
        Err(e) => {
            log::warn!("refresh failed (path: {}): {e}", path.display());
        }
    }
}

// ============================================================================
// Cuddle line
// ============================================================================

/// Print a dim summary line for a `FileGroup` that has siblings.
///
/// Format: `  └ <summary>  <aggregate_size> total`
///
/// The line is rendered dim (ANSI 2) when color is enabled so it
/// recedes behind the base file row visually.
fn print_cuddle_line(fg: &FileGroup, cfg: &DisplayConfig) {
    let summary = sibling_summary(fg);
    let agg = aggregate_size(fg);
    let size_str = fmt_size(u128::from(agg), SizeBase::Binary);
    let line = format!("{ROW_INDENT}\u{2514} {summary}  {size_str} total");
    if cfg.use_color {
        println!("{DIM}{line}{RESET}");
    } else {
        println!("{line}");
    }
}

// ============================================================================
// JSON output
// ============================================================================

/// Emit a JSON listing to stdout and return.
///
/// Serialises the fully grouped structure.  All display-layer formatting
/// (ANSI codes, column alignment) is bypassed.
fn emit_json(groups: &[DirGroup], path: &str, elapsed_us: u64) -> io::Result<()> {
    let json_groups: Vec<JsonDirGroup> = groups
        .iter()
        .map(|g| {
            let file_groups = group_entries(&g.entries);
            let json_file_groups = file_groups
                .iter()
                .map(|fg| {
                    let siblings = fg
                        .siblings
                        .iter()
                        .map(|s| JsonSibling {
                            name: s.entry.dirent.name.as_str().to_owned(),
                            kind: sibling_kind_str(&s.kind),
                            size: s.entry.dirent.size.as_u64(),
                        })
                        .collect();
                    JsonFileGroup {
                        base_name: fg.base.dirent.name.as_str().to_owned(),
                        base_size: fg.base.dirent.size.as_u64(),
                        siblings,
                        aggregate_size: aggregate_size(fg),
                    }
                })
                .collect();
            JsonDirGroup {
                selinux_type: g.key.selinux_type.clone(),
                marking: g.key.marking.clone(),
                file_groups: json_file_groups,
            }
        })
        .collect();

    let doc = JsonListing {
        path: path.to_owned(),
        groups: json_groups,
        elapsed_us,
    };

    let json = serde_json::to_string_pretty(&doc).map_err(io::Error::other)?;
    println!("{json}");
    Ok(())
}

const fn sibling_kind_str(kind: &SiblingKind) -> &'static str {
    match kind {
        SiblingKind::Rotation => "rotation",
        SiblingKind::CompressedRotation => "compressed_rotation",
        SiblingKind::Signature => "signature",
        SiblingKind::Checksum => "checksum",
        SiblingKind::Backup => "backup",
        SiblingKind::Related => "related",
    }
}

// Column width pre-scan

fn compute_widths(
    groups: &[DirGroup],
    cols: &ColumnSet,
    cfg: &DisplayConfig,
) -> Vec<(Column, usize)> {
    cols.columns()
        .iter()
        .filter(|&&c| c != Column::Iov && c != Column::Name)
        .map(|&col| {
            let header_w = col_header(col).len();
            let data_w = groups
                .iter()
                .flat_map(|g| g.entries.iter().map(|e| cell_plain(e, col, &g.key, cfg).len()))
                .max()
                .unwrap_or(0);
            (col, header_w.max(data_w) + 2)
        })
        .collect()
}

fn col_width(widths: &[(Column, usize)], col: Column) -> usize {
    for (c, w) in widths {
        if *c == col {
            return *w;
        }
    }
    14
}

//============================================================================
// HEADER
//============================================================================
fn print_header(cols: &ColumnSet, widths: &[(Column, usize)]) {
    let mut line = ROW_INDENT.to_owned();

    for &col in cols.columns() {
        if col == Column::Iov {
            // write! formats directly into 'line' without a temporary String
            let _ = write!(line, "{:<5}", col_header(col));
        } else if col == Column::Name {
            line.push_str(NAME_PREFIX);
            line.push_str(&col_header(col));
        } else {
            let w = col_width(widths, col);
            let _ = write!(line, "{:<w$}", col_header(col));
        }
    }
    println!("{line}");
}

// This will become table header for Textual & Graphical Unser Interfaces
fn col_header(col: Column) -> Cow<'static, str> {
    match col {
        // Wrap translations in Cow::Owned
        Column::Mode => Cow::Owned(i18n::tr("MODE")),
        Column::Marking => Cow::Owned(i18n::tr("MARKING")),
        Column::UidGid => Cow::Owned(i18n::tr("OWNER:GROUP")),
        Column::Size => Cow::Owned(i18n::tr("SIZE")),
        Column::Mtime => Cow::Owned(i18n::tr("MODIFIED")),
        Column::Name => Cow::Owned(i18n::tr("NAME")),

        // Wrap static literals in Cow::Borrowed
        Column::Iov => Cow::Borrowed("IOV"),
        Column::SelinuxType => Cow::Borrowed("SELINUX TYPE"),
        Column::Inode => Cow::Borrowed("INODE"),
    }
}

// ===========================================================================
// ROW Rendering
//
// Column Sizing:
//   Right-justify the value within (w - 2) chars and append 2 trailing
//   spaces.  Total output is still `w` chars — same as a left-justified
//   column — so the NAME icon that follows always has a visible gap.
//   Without this, the +2 padding from compute_widths ends up as leading
//   space only, leaving zero separation before the NAME icon (e.g. ⊕).
//
fn print_row(
    entry: &ListEntry,
    key: &GroupKey,
    cols: &ColumnSet,
    widths: &[(Column, usize)],
    cfg: &DisplayConfig,
) {
    let mut line = ROW_INDENT.to_owned();
    for &col in cols.columns() {
        if col == Column::Iov {
            line.push_str(&cell_iov(entry, cfg));
            line.push_str("  ");
        } else if col == Column::Name {
            line.push_str(&cell_plain(entry, col, key, cfg));
        } else if col == Column::Size {
            let w = col_width(widths, col);
            let inner = w.saturating_sub(2);
            line.push_str(&format!("{:>inner$}  ", cell_plain(entry, col, key, cfg)));
        } else {
            let w = col_width(widths, col);
            line.push_str(&format!("{:<w$}", cell_plain(entry, col, key, cfg)));
        }
    }
    println!("{line}");
}

// ==========================================================================
// Cell renderers
// ==========================================================================
// Plain cell value — used for both width pre-scan and row display.
//
// `Column::Iov` and `Column::Name` are handled separately in `print_row`;
// their arms here are used only during the width pre-scan.
fn cell_plain(entry: &ListEntry, col: Column, key: &GroupKey, cfg: &DisplayConfig) -> String {
    match col {
        Column::Mode => {
            // Standard 10-char mode string; append '+' when POSIX ACL present.
            let mut s = format!(
                "{}{}",
                file_type_char(entry.dirent.file_type),
                entry.dirent.mode.as_mode_str()
            );
            if entry.dirent.sec_flags.contains(InodeSecurityFlags::ACL_PRESENT) {
                s.push('+');
            }
            s
        }
        Column::Iov => "---".to_owned(),
        Column::SelinuxType => key.selinux_type.clone(),
        Column::Marking => key.marking.clone(),
        Column::UidGid => {
            let uid = entry.dirent.ownership.user.uid.as_u32();
            let gid = entry.dirent.ownership.group.gid.as_u32();
            let owner = resolve_username(uid);
            let group = resolve_groupname(gid);
            format!("{owner}:{group}")
        }
        Column::Size => fmt_size(u128::from(entry.dirent.size.as_u64()), SizeBase::Binary),
        Column::Mtime => format_mtime(entry.mtime),
        Column::Inode => entry.dirent.inode.as_u64().to_string(),
        Column::Name => {
            // 3-char icon zone: encrypted takes priority over plain mount.
            let icon = if entry.dirent.has_encryption() {
                format!("{}  ", cfg.encrypted_symbol)
            } else if entry.dirent.is_mountpoint {
                format!("{}  ", cfg.mount_symbol)
            } else if file_type_char(entry.dirent.file_type) == 'd' {
                // Only one trailing space because the icon is so big.
                format!("{} ", cfg.plaindir_symbol)
            } else {
                NAME_PREFIX.to_owned()
            };

            let mut name = entry.dirent.name.as_str().to_owned();
            if entry.dirent.file_type.is_directory() {
                name.push('/');
            } else if entry.dirent.mode.owner_can_execute() {
                name.push('*');
            }

            format!("{icon}{name}")
        }
    }
}

/// IOV security-posture marker — `I`=immutable, `O`=observations, `V`=IMA.
///
/// Visible width is always 3.  When `cfg.use_color` is `false`, ANSI
/// codes are omitted and plain ASCII characters are used instead.
///
/// `O` lights up (bold red) whenever `security_observations()` returns a
/// non-empty vec — the detail is left to the forthcoming `stat` command.
/// ACL presence is already expressed by `+` in the Mode column.
fn cell_iov(entry: &ListEntry, cfg: &DisplayConfig) -> String {
    let flags = &entry.dirent.sec_flags;

    let i = if flags.contains(InodeSecurityFlags::IMMUTABLE) {
        if cfg.use_color {
            format!("{RED}I{RESET}")
        } else {
            "I".to_owned()
        }
    } else if cfg.use_color {
        format!("{DIM}-{RESET}")
    } else {
        "-".to_owned()
    };

    // O lights up for any Risk-kind observation.
    // Using kind() keeps this logic stable as new observations are added —
    // new Risk variants automatically light up O without touching this code.
    // Warning and Good observations do not light up O.
    let posture_obs =
        entry.dirent.security_observations().into_iter().any(|o| o.kind() == ObservationKind::Risk);

    let o = if posture_obs {
        if cfg.use_color {
            format!("{BOLD_RED}O{RESET}")
        } else {
            "O".to_owned()
        }
    } else if cfg.use_color {
        format!("{DIM}-{RESET}")
    } else {
        "-".to_owned()
    };

    let v = if flags.contains(InodeSecurityFlags::IMA_PRESENT) {
        if cfg.use_color {
            format!("{GREEN}V{RESET}")
        } else {
            "V".to_owned()
        }
    } else if cfg.use_color {
        format!("{DIM}-{RESET}")
    } else {
        "-".to_owned()
    };

    format!("{i}{o}{v}")
}

// ================================================================
// GROUP HEADER - SELINUX TYPE + MARKING
// ================================================================
// Render the ` type :: marking ...` group separator line.
//
// Fill is computed from the plain text length so `=` characters always
// reach [`TERM_WIDTH`].  When `cfg.use_color` is `true` and secolor.conf
// is available, the type and marking are wrapped in ANSI 24-bit true-color.
fn group_separator(key: &GroupKey, cfg: &DisplayConfig) -> String {
    let plain = format!(
        "{0:20} \u{1FB6C}{1:20}\u{1FB6C} ",
        key.selinux_type, key.marking
    );
    let fill = " ".repeat(TERM_WIDTH.saturating_sub(plain.len()));

    if cfg.use_color
        && let Some(ref sc) = cfg.secolor
    {
        let ctx = ContextComponents {
            user: "",
            role: "",
            r#type: &key.selinux_type,
            range: &key.marking,
        };
        let colors = resolve_colors(&ctx, sc);
        let type_out = ansi_fg(colors[2].fg, &key.selinux_type);
        let marking_out = ansi_fg(colors[3].fg, &key.marking);
        return format!("{type_out} :: {marking_out} {fill}");
    }

    if key.selinux_type == "<restricted>" {
        let selinux_type = i18n::tr("<restricted>");
        format!(
            "{DIM_ITALIC}{UNDERLINE}{0} :: {1} {fill}{RESET}",
            selinux_type, key.marking
        )
    } else {
        // BE CAREFUL HERE! This combination of reverrse, colors, and unicode was challenging!
        format!(
            "{BLACK_ON_CYAN} {0:20} {REVERSE}\x1b[36;30m\u{1FB6C}{RESET}{REVERSE}\u{1FB6C}{1:^20} {RESET}{UNDERLINE}\u{1FB6C}{fill}{RESET}",
            key.selinux_type, key.marking
        )
    }
}

//
//Identity resolution
//
fn resolve_username(uid: u32) -> String {
    match nix::unistd::User::from_uid(nix::unistd::Uid::from_raw(uid)) {
        Ok(Some(u)) => u.name,
        _ => uid.to_string(),
    }
}

fn resolve_groupname(gid: u32) -> String {
    match nix::unistd::Group::from_gid(nix::unistd::Gid::from_raw(gid)) {
        Ok(Some(g)) => g.name,
        _ => gid.to_string(),
    }
}

fn ansi_fg(rgb: Rgb, text: &str) -> String {
    format!("\x1b[38;2;{};{};{}m{text}\x1b[0m", rgb.r, rgb.g, rgb.b)
}

const fn file_type_char(ft: FileType) -> char {
    match ft {
        FileType::Directory => 'd',
        FileType::Symlink => 'l',
        FileType::BlockDevice => 'b',
        FileType::CharDevice => 'c',
        FileType::Fifo => 'p',
        FileType::Socket => 's',
        FileType::RegularFile | FileType::Unknown => '-',
    }
}

fn format_mtime(mtime: Option<SystemTime>) -> String {
    if let Some(t) = mtime {
        let dt: DateTime<Local> = t.into();
        dt.format("%Y-%m-%d %H:%M").to_string()
    } else {
        "?".to_owned()
    }
}
