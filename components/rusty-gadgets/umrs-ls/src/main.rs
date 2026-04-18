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
// doc_markdown: backtick-wrapping every key name and field reference in prose
// is disruptive for operational documentation.  Matches Cargo.toml policy.
#![allow(clippy::doc_markdown)]
// missing_errors_doc / missing_panics_doc: # Errors/# Panics on every Result fn adds noise.
// Matches Cargo.toml policy.
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]

use std::borrow::Cow;
use std::fmt::Write;
use std::io::{self, IsTerminal as _};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

use clap::Parser;
use owo_colors::{OwoColorize as _, Stream, Style};

use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use serde::Serialize;

use chrono::{DateTime, Local};
use umrs_core::human::sizefmt::{SizeBase, auto_format as fmt_size};
use umrs_core::i18n;
use umrs_labels::cui::catalog::{Catalog, LevelRegistry, load_catalog, load_levels};
use umrs_labels::marking_to_detail;
use umrs_ls::grouping::{FileGroup, SiblingKind, aggregate_size, group_entries, sibling_summary};
use umrs_ls::identity::resolve_owner_display;
use umrs_ls::tui_render::{
    GotoBar, HelpOverlay, render_dir_browser, render_help_overlay, render_permission_denied,
};
use umrs_ls::viewer_app::DirViewerApp;
use umrs_selinux::ObservationKind;
use umrs_selinux::mcs::colors::{
    ContextComponents, Rgb, SeColorConfig, load_default as load_secolor_default, resolve_colors,
};
use umrs_selinux::secure_dirent::{FileType, InodeSecurityFlags};
use umrs_selinux::utils::dirlist::{
    Column, ColumnSet, DirGroup, GroupKey, ListEntry, list_directory,
};
use umrs_stat::FileStatApp;
use umrs_ui::indicators::{build_header_context, detect_os_name};
use umrs_ui::keymap::{Action, KeyMap};
use umrs_ui::marking_detail::MarkingDetailData;
use umrs_ui::popup::{render_audit_card_popup, render_marking_detail_popup};
use umrs_ui::theme::Theme;
use umrs_ui::viewer::{ViewerApp as _, ViewerState};

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

// ============================================================================
// Stat popup state
// ============================================================================

/// State for the file security audit popup overlay.
///
/// Opened by pressing Enter on a non-directory, non-group-header node.
/// Closed by Esc or q.  Tab / ← / → cycle the three audit card tabs;
/// j / k / Up / Down / PageUp / PageDown scroll the active tab.
///
/// ## Fields
///
/// * app - Pre-built audit card data for the selected file.
/// * active_tab - Currently active tab (0 = Identity, 1 = Security, 2 = Observations).
/// * scroll - Per-tab scroll offsets; indexed by `active_tab`.
///
/// ## Compliance
///
/// - **NIST SP 800-53 AU-3**: The popup surfaces a complete audit card for the
///   selected file, ensuring the operator has all identity and security context
///   without leaving the directory view.
/// - **NIST SP 800-53 AC-3**: MAC label state is visible on demand per node.
///
struct StatPopupState {
    app: FileStatApp,
    active_tab: usize,
    scroll: [u16; 3],
}

const TERM_WIDTH: usize = 100;
const ROW_INDENT: &str = "  "; // 2-space left indent on every row
const NAME_PREFIX: &str = "   "; // 3-char icon zone before filename

// Color output uses owo-colors for type-safe ANSI styling.
//
// NO_COLOR compliance: `owo_colors::set_override(false)` is called at startup
// when NO_COLOR is set or --color is absent, suppressing all color output
// regardless of terminal capabilities (NIST SP 800-53 SI-11).
//
// All styled strings are produced via `OwoColorize` trait methods and only
// emitted when `DisplayConfig::use_color` is true — the gate is checked before
// any styled value is produced, so no escapes reach non-terminal sinks.

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
            // load_secolor_default queries the active policy name from the kernel
            // and constructs the correct path — no hardcoded policy name.
            load_secolor_default().ok()
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

// ============================================================================
// CUI catalog loading — label detail popup
// ============================================================================

/// Search several candidate paths for a catalog JSON file, returning the first
/// one that loads successfully.
///
/// The caller provides an ordered list of paths to try. The first success wins;
/// all errors are silently discarded. When no candidate succeeds, `None` is
/// returned and the label popup degrades gracefully (showing nothing).
///
/// Paths are tried relative to the process current working directory. In
/// development this is the workspace root (`components/rusty-gadgets/`), so
/// the `../umrs-label/config/…` path covers the normal `cargo run` case.
///
/// ## Compliance
///
/// - **NIST SP 800-53 AC-16**: Security Attributes — catalogs are the
///   authoritative source of CUI label definitions shown in the detail popup.
fn try_load_catalog(candidates: &[&str]) -> Option<Catalog> {
    candidates.iter().find_map(|p| load_catalog(p).ok())
}

/// Load the US and Canadian CUI catalogs for the label detail popup.
///
/// Returns `(us_catalog, ca_catalog)` where each is `Option<Catalog>`.
/// Missing or unreadable files yield `None` — the popup degrades gracefully.
///
/// Path resolution uses the documented override chain. Under the FHS 3.0
/// §3.13 layout the reference databases live flat under
/// `/opt/umrs/share/umrs/` — there is no `us/` or `ca/` subdirectory.
///
///   1. `UMRS_CONFIG_DIR` environment variable
///   2. `/opt/umrs/share/umrs/`  (install default, FHS 3.0 §3.13)
///   3. CWD-relative `config/` subtree (development via `cargo run`)
///   4. `../umrs-label/config/` (workspace-relative dev fallback)
///
/// ## Compliance
///
/// - **NIST SP 800-53 AC-16**: Security Attributes — catalog data drives the
///   label detail popup; loading errors surface as absent popup content, not
///   panics.
/// - **NIST SP 800-53 CM-6**: Configuration Settings — catalog path resolves
///   under the documented install root before any other location.
/// - **FHS 3.0 §3.13**: Package-specific static reference data path.
fn load_catalogs() -> (Option<Catalog>, Option<Catalog>) {
    // Build the candidate list honoring the UMRS_CONFIG_DIR override chain.
    let config_dir_entry: Option<String> = std::env::var("UMRS_CONFIG_DIR").ok();

    let us_candidates: Vec<String> = {
        let mut v = Vec::with_capacity(4);
        if let Some(ref dir) = config_dir_entry {
            v.push(format!("{dir}/US-CUI-LABELS.json"));
        }
        v.push("/opt/umrs/share/umrs/US-CUI-LABELS.json".to_owned());
        v.push("config/US-CUI-LABELS.json".to_owned());
        v.push("../umrs-label/config/US-CUI-LABELS.json".to_owned());
        v
    };

    let ca_candidates: Vec<String> = {
        let mut v = Vec::with_capacity(4);
        if let Some(ref dir) = config_dir_entry {
            v.push(format!("{dir}/CANADIAN-PROTECTED.json"));
        }
        v.push("/opt/umrs/share/umrs/CANADIAN-PROTECTED.json".to_owned());
        v.push("config/CANADIAN-PROTECTED.json".to_owned());
        v.push("../umrs-label/config/CANADIAN-PROTECTED.json".to_owned());
        v
    };

    let us = try_load_catalog(&us_candidates.iter().map(String::as_str).collect::<Vec<_>>());
    let ca = try_load_catalog(&ca_candidates.iter().map(String::as_str).collect::<Vec<_>>());
    (us, ca)
}

/// Load the MCS sensitivity level registry from `LEVELS.json`.
///
/// Uses the same path-resolution chain as `load_catalogs` so the operator
/// override (`UMRS_CONFIG_DIR`) applies here too.  Returns `None` if no
/// readable file is found — the system-info popup degrades to hardcoded
/// fallback text rather than failing.
///
/// ## Compliance
///
/// - **NIST SP 800-53 AC-16**: Security Attributes — level definitions are
///   read from the authoritative `LEVELS.json` rather than embedded strings.
/// - **FHS 3.0 §3.13**: Static data for `/opt/umrs/` packages lives under
///   `/opt/umrs/share/umrs/`.
fn load_levels_registry() -> Option<LevelRegistry> {
    let config_dir_entry: Option<String> = std::env::var("UMRS_CONFIG_DIR").ok();
    let mut candidates: Vec<String> = Vec::with_capacity(4);
    if let Some(ref dir) = config_dir_entry {
        candidates.push(format!("{dir}/LEVELS.json"));
    }
    candidates.push("/opt/umrs/share/umrs/LEVELS.json".to_owned());
    candidates.push("config/LEVELS.json".to_owned());
    candidates.push("../umrs-label/config/LEVELS.json".to_owned());
    candidates.iter().find_map(|p| load_levels(p).ok())
}

/// Look up a marking string in the loaded catalogs and build a detail popup.
///
/// Tries the US catalog first, then the Canadian catalog. For each catalog,
/// two lookup strategies are attempted in order:
///
/// 1. **Direct key lookup** — the marking string is the JSON catalog key
///    (e.g., `"CUI//LEI"`, `"PROTECTED-A"`). This covers US CUI markings
///    translated by `setrans.conf`.
/// 2. **MCS level fallback** — the marking string is a raw MCS level
///    (e.g., `"s1:c300"`). Used when `setrans.conf` has no translation for
///    a category, which is the case for Canadian Protected designations
///    (c300–c302). `Catalog::marking_by_mcs_level` finds the entry whose
///    `level` and `category_base` match the level components.
///
/// Returns `None` when neither catalog contains a matching entry for `marking`
/// under either strategy.
///
/// ## Compliance
///
/// - **NIST SP 800-53 AC-16**: Security Attributes — the popup shows the
///   full regulatory label definition for the marking applied to selected nodes.
///   
fn lookup_marking_detail(
    marking: &str,
    us_catalog: Option<&Catalog>,
    ca_catalog: Option<&Catalog>,
) -> Option<MarkingDetailData> {
    // US catalog — direct key, then MCS level fallback, then banner text fallback.
    if let Some(cat) = us_catalog {
        if let Some(m) = cat.marking(marking) {
            let flag = cat.country_flag().unwrap_or_default();
            return Some(marking_to_detail(marking, m, &flag));
        }
        if let Some((key, m)) = cat.marking_by_mcs_level(marking) {
            let flag = cat.country_flag().unwrap_or_default();
            return Some(marking_to_detail(key, m, &flag));
        }
        if let Some((key, m)) = cat.marking_by_banner(marking) {
            let flag = cat.country_flag().unwrap_or_default();
            return Some(marking_to_detail(key, m, &flag));
        }
    }
    // Canadian catalog — direct key, then MCS level fallback, then banner text fallback.
    if let Some(cat) = ca_catalog {
        if let Some(m) = cat.marking(marking) {
            let flag = cat.country_flag().unwrap_or_default();
            return Some(marking_to_detail(marking, m, &flag));
        }
        if let Some((key, m)) = cat.marking_by_mcs_level(marking) {
            let flag = cat.country_flag().unwrap_or_default();
            return Some(marking_to_detail(key, m, &flag));
        }
        if let Some((key, m)) = cat.marking_by_banner(marking) {
            let flag = cat.country_flag().unwrap_or_default();
            return Some(marking_to_detail(key, m, &flag));
        }
    }
    None
}

/// Find the index group for a marking string by searching the catalogs.
///
/// Returns `None` for system-level markings (SystemLow, etc.) that have
/// no catalog entry — they use the default palette color.
///
/// Applies the same two-strategy lookup as [`lookup_marking_detail`]: direct
/// key lookup first, then MCS level fallback for raw level strings (e.g.,
/// `"s1:c300"`) that arise when `setrans.conf` has no translation.
fn find_index_group_for_marking(
    marking: &str,
    us_catalog: Option<&Catalog>,
    ca_catalog: Option<&Catalog>,
) -> Option<String> {
    if let Some(cat) = us_catalog {
        if let Some(m) = cat.marking(marking) {
            return m.index_group.clone();
        }
        if let Some((_, m)) = cat.marking_by_mcs_level(marking) {
            return m.index_group.clone();
        }
        if let Some((_, m)) = cat.marking_by_banner(marking) {
            return m.index_group.clone();
        }
    }
    if let Some(cat) = ca_catalog {
        if let Some(m) = cat.marking(marking) {
            return m.index_group.clone();
        }
        if let Some((_, m)) = cat.marking_by_mcs_level(marking) {
            return m.index_group.clone();
        }
        if let Some((_, m)) = cat.marking_by_banner(marking) {
            return m.index_group.clone();
        }
    }
    None
}

// ============================================================================
// CLI argument definition
// ============================================================================

/// Security-focused directory listing with SELinux context, MCS markings,
/// and POSIX ownership.
///
/// Files are grouped by `(SELinux type, security marking)`. Related files
/// (rotations, signatures, checksums) are cuddled under their base file by
/// default; use `--flat` to show every entry on its own row.
///
/// ## Output Modes
///
/// - **TUI** (default when stdout is a TTY): interactive browser with
///   tree navigation, search, and directory traversal.
/// - **CLI** (`--cli` or non-TTY): columnar text listing.
/// - **JSON** (`--json`): machine-readable grouped output.
///
/// ## Compliance
///
/// - **NIST SP 800-53 AC-3**: Every entry displays the SELinux label used in
///   access decisions.
/// - **NIST SP 800-53 SI-10**: clap validates all arguments at entry, rejecting
///   unknown flags before any directory I/O occurs.
// CLI flag structs are naturally bool-heavy; this is not a state machine.
// Each bool corresponds to exactly one independent command-line flag.
#[expect(clippy::struct_excessive_bools, reason = "CLI flags — not a state machine")]
#[derive(Parser)]
#[command(name = "umrs-ls", version, about, long_about = None)]
struct Args {
    /// Directory to list (default: current directory).
    #[arg(value_name = "PATH", default_value = ".")]
    path: PathBuf,

    /// Force plain-text CLI output instead of the interactive TUI.
    ///
    /// Implied when stdout is not a terminal.
    #[arg(long)]
    cli: bool,

    /// Emit machine-readable JSON output.
    #[arg(long)]
    json: bool,

    /// Disable sibling cuddling — show every entry on its own row.
    #[arg(long)]
    flat: bool,

    /// Show security observation flags (IOV) in the listing.
    ///
    /// IOV flags are shown by default; use `--no-iov` to suppress them.
    #[arg(long)]
    no_iov: bool,

    /// Suppress the modification time column.
    #[arg(long)]
    no_mtime: bool,

    /// Add a file size column.
    #[arg(long)]
    with_size: bool,

    /// Add an inode number column.
    #[arg(long)]
    with_inode: bool,

    /// Enable ANSI color output.
    ///
    /// Off by default. Has no effect when `NO_COLOR` is set.
    #[arg(long)]
    color: bool,

    /// Show step-by-step progress on stderr.
    ///
    /// Narrates catalog paths, catalog counts, and trust-gate results so
    /// operators can diagnose slow starts without enabling debug logging.
    /// NIST SP 800-53 SI-11.
    #[arg(long, short)]
    verbose: bool,
}

fn main() -> io::Result<()> {
    if let Ok(logger) = systemd_journal_logger::JournalLog::new() {
        let _ = logger.install();
        log::set_max_level(log::LevelFilter::Info);
    }

    i18n::init("umrs-ls");

    let args = Args::parse();

    // Enable verbose progress output to stderr when --verbose / -v is passed.
    // All verbose output goes to stderr so it does not interfere with --json
    // or piped stdout. NIST SP 800-53 SI-11.
    let verbose = args.verbose;
    macro_rules! verbose {
        ($($arg:tt)*) => {
            if verbose {
                eprintln!("  [umrs-ls] {}", format_args!($($arg)*));
            }
        };
    }

    let target = args.path.to_str().unwrap_or(".");
    verbose!("Target: {}", target);

    let json_mode = args.json;
    let cli_mode = args.cli;
    let flat_mode = args.flat;

    // Mode selection:
    //   --json              → JSON output, always (no TUI, no ANSI table)
    //   --cli or non-TTY   → CLI columnar text output
    //   otherwise          → TUI interactive viewer
    if json_mode {
        verbose!("Mode: JSON");
        return run_json(target);
    }

    if cli_mode || !io::stdout().is_terminal() {
        verbose!("Mode: CLI");
        return run_cli(
            target,
            flat_mode,
            args.no_iov,
            args.no_mtime,
            args.with_size,
            args.with_inode,
            args.color,
        );
    }

    verbose!("Mode: TUI");
    run_tui(target, flat_mode, args.color)
}

// ============================================================================
// JSON output path
// ============================================================================

/// Emit a JSON listing for `target` and return.
///
/// `--flat` has no effect on JSON output — the grouped structure is always
/// emitted. The flag is accepted for forward-compatibility so callers can
/// pass it without knowing the mode.
fn run_json(target: &str) -> io::Result<()> {
    let listing = list_directory(Path::new(target))?;
    emit_json(
        &listing.groups,
        &listing.path.display().to_string(),
        listing.elapsed_us,
    )
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
/// ## Compliance
///
/// - **NIST SP 800-53 AU-3**: All identity, label, and observation fields
///   required for audit are included in the tabular output.
// Each parameter is a direct CLI flag. Wrapping in a sub-struct would obscure
// the 1:1 mapping to user-visible flags without adding safety.
#[expect(
    clippy::fn_params_excessive_bools,
    reason = "each bool is a distinct CLI flag — not overloaded state"
)]
fn run_cli(
    target: &str,
    flat_mode: bool,
    no_iov: bool,
    no_mtime: bool,
    with_size: bool,
    with_inode: bool,
    color: bool,
) -> io::Result<()> {
    // SelinuxType and Marking appear in the group header — omit from rows.
    let mut cols = ColumnSet::default().without(Column::SelinuxType).without(Column::Marking);

    if no_iov {
        cols = cols.without(Column::Iov);
    }
    if no_mtime {
        cols = cols.without(Column::Mtime);
    }
    if with_size {
        cols = cols.with(Column::Size);
    }
    if with_inode {
        cols = cols.with(Column::Inode);
    }

    // NO_COLOR compliance: honor the env var regardless of --color flag.
    // `var_os` — presence is the signal, value is irrelevant (NIST SP 800-53 SI-11).
    // owo_colors::set_override(false) suppresses all owo-colors output globally
    // for this process; set_override(true) forces color even when piped.
    let use_color = color && std::env::var_os("NO_COLOR").is_none();
    owo_colors::set_override(use_color);
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
/// ## Compliance
///
/// - **NIST SP 800-53 AC-3**: Navigation is read-only; no directory entries are
///   created, deleted, or modified through the viewer interface.
/// - **NIST SP 800-53 AU-3**: The viewer header carries tool identity, data source
///   path, and entry counts on every rendered frame.
#[expect(
    clippy::too_many_lines,
    reason = "TUI event loop is inherently sequential; splitting would scatter the state machine"
)]
fn run_tui(target: &str, _flat_mode: bool, color: bool) -> io::Result<()> {
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

    // NO_COLOR compliance: mirror the same precedence as run_cli.
    // `color` is the --color flag; NO_COLOR env (any non-empty value) overrides it.
    // NIST SP 800-53 SI-11 — honor environment signals that govern output format.
    let use_color = color && std::env::var_os("NO_COLOR").is_none();
    let theme = if use_color {
        Theme::dark()
    } else {
        Theme::no_color()
    };

    // Load CUI catalogs for label detail popups.  Paths are tried in order;
    // the first successful load wins.  Missing catalogs yield `None` — the
    // popup degrades gracefully to "no data" rather than failing.
    let (us_catalog, ca_catalog) = load_catalogs();
    // Load MCS sensitivity level registry for SystemLow/SystemHigh popup text.
    let level_registry = load_levels_registry();

    // Build the header context once at startup.  OS name is read through
    // the umrs-platform OsDetector pipeline via detect_os_name(), which routes
    // through provenance-verified SecureReader paths rather than raw /etc/os-release I/O.
    let ctx = build_header_context(
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        detect_os_name(),
    );

    // Enter the alternate screen and raw mode.
    let mut terminal = ratatui::init();

    // Permission denied overlay — (path, error_message) shown as a modal.
    // `None` = no overlay.  `Some(...)` = overlay is open, blocks all input
    // except Enter/Esc to dismiss.
    let mut nav_error: Option<(String, String)> = None;

    // "Go to path" prompt state.  Activated by Shift+G.  Mutually exclusive
    // with the search bar — the event loop gates activation on the other's
    // inactive state.
    let mut goto = GotoBar::default();

    // `?` help overlay — a modal popup with tabbed content (Navigation /
    // Columns).  When active, blocks background input except for Tab / arrow
    // (to switch tabs) and `?` / Esc (to dismiss).
    let mut help = HelpOverlay::default();

    // Label detail popup state.  `Some((data, scroll))` means the popup is
    // open and displaying the given marking detail.  `None` means closed.
    // When open, all input is consumed by the popup handler before reaching
    // the directory browser event loop.
    //
    // NIST SP 800-53 AC-16 — the detail popup makes the full regulatory label
    // definition accessible to the operator without leaving the directory view.
    let mut label_popup: Option<(MarkingDetailData, u16)> = None;

    // File security audit popup state.  `Some(state)` means the three-tab
    // audit card overlay is open for a selected file node.  `None` means
    // closed.  Stat popup takes input priority over the label popup — only
    // one popup is open at a time so priority order does not cause conflicts.
    //
    // NIST SP 800-53 AU-3 — the full audit card (identity, security,
    // observations) is accessible on demand without leaving the directory view.
    let mut stat_popup: Option<StatPopupState> = None;

    // Event loop — 100 ms poll timeout keeps the TUI snappy without busy-waiting.
    loop {
        if let Err(e) = terminal.draw(|f| {
            render_dir_browser(
                f,
                f.area(),
                &app,
                &state,
                &ctx,
                &theme,
                &goto,
                us_catalog.as_ref(),
                ca_catalog.as_ref(),
            );
            // Permission denied overlay — rendered on top when present.
            if let Some((ref path, ref msg)) = nav_error {
                render_permission_denied(f, f.area(), path, msg, &theme);
            }
            // Help overlay — rendered on top of everything else.
            if help.active {
                render_help_overlay(f, f.area(), &help, &theme);
            }
            // Label detail popup — rendered on top of all other overlays.
            if let Some((ref data, scroll)) = label_popup {
                render_marking_detail_popup(f, f.area(), data, scroll, &theme);
            }
            // Stat audit popup — rendered on top of everything else when open.
            // Mutually exclusive with label_popup (Enter logic prevents both
            // from opening simultaneously).
            if let Some(ref sp) = stat_popup {
                render_audit_card_popup(
                    f,
                    f.area(),
                    &sp.app,
                    sp.active_tab,
                    sp.scroll[sp.active_tab],
                    &theme,
                );
            }
        }) {
            log::error!("terminal draw error: {e}");
            break;
        }

        match event::poll(Duration::from_millis(100)) {
            Ok(true) => match event::read() {
                Ok(Event::Key(key)) => {
                    // Stat audit popup owns all input while open.
                    // Esc / q dismiss; Tab / ← / → cycle tabs;
                    // j / k / Up / Down / PageUp / PageDown scroll active tab.
                    if let Some(ref mut sp) = stat_popup {
                        match key.code {
                            KeyCode::Esc | KeyCode::Char('q') => {
                                stat_popup = None;
                            }
                            KeyCode::Tab | KeyCode::Right => {
                                sp.active_tab = (sp.active_tab.saturating_add(1)) % 3;
                            }
                            KeyCode::BackTab | KeyCode::Left => {
                                sp.active_tab = (sp.active_tab.saturating_add(2)) % 3;
                            }
                            KeyCode::Down | KeyCode::Char('j') => {
                                sp.scroll[sp.active_tab] =
                                    sp.scroll[sp.active_tab].saturating_add(1);
                            }
                            KeyCode::Up | KeyCode::Char('k') => {
                                sp.scroll[sp.active_tab] =
                                    sp.scroll[sp.active_tab].saturating_sub(1);
                            }
                            KeyCode::PageDown => {
                                sp.scroll[sp.active_tab] =
                                    sp.scroll[sp.active_tab].saturating_add(10);
                            }
                            KeyCode::PageUp => {
                                sp.scroll[sp.active_tab] =
                                    sp.scroll[sp.active_tab].saturating_sub(10);
                            }
                            _ => {}
                        }
                        continue;
                    }

                    // Label detail popup owns all input while open.
                    // Esc / q dismiss; j / k / Up / Down / PageUp / PageDown scroll.
                    if let Some((_, ref mut scroll)) = label_popup {
                        match key.code {
                            KeyCode::Esc | KeyCode::Char('q') => {
                                label_popup = None;
                            }
                            KeyCode::Down | KeyCode::Char('j') => {
                                *scroll = scroll.saturating_add(1);
                            }
                            KeyCode::Up | KeyCode::Char('k') => {
                                *scroll = scroll.saturating_sub(1);
                            }
                            KeyCode::PageDown => {
                                *scroll = scroll.saturating_add(10);
                            }
                            KeyCode::PageUp => {
                                *scroll = scroll.saturating_sub(10);
                            }
                            _ => {}
                        }
                        continue;
                    }

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

                    // Help overlay: owns all input while visible.
                    // Tab / ← / → cycle tabs; ? or Esc dismiss.
                    if help.active {
                        match key.code {
                            KeyCode::Char('?') | KeyCode::Esc => help.close(),
                            KeyCode::Tab | KeyCode::Left | KeyCode::Right => help.next_tab(),
                            _ => {}
                        }
                        continue;
                    }

                    // `?` activates the help overlay (from normal mode only;
                    // the prompt bars get a chance to consume `?` as a
                    // literal character further down).
                    if key.code == KeyCode::Char('?') && !state.search_active && !goto.active {
                        help.open();
                        continue;
                    }

                    // Goto mode: character input goes to the goto query buffer.
                    if goto.active {
                        match key.code {
                            KeyCode::Char(ch) => {
                                goto.push_char(ch);
                                continue;
                            }
                            KeyCode::Backspace => {
                                goto.pop_char();
                                continue;
                            }
                            KeyCode::Tab => {
                                complete_goto_query(&mut goto);
                                continue;
                            }
                            KeyCode::Esc => {
                                goto.close();
                                continue;
                            }
                            KeyCode::Enter => {
                                handle_goto_submit(&mut app, &mut state, &mut goto, &mut nav_error);
                                continue;
                            }
                            _ => {}
                        }
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

                    // "Go to" activation: Shift+G (uppercase G).  Only when
                    // neither prompt bar is already active.  Crossterm reports
                    // Shift+G as `KeyCode::Char('G')` on all modern terminals,
                    // so we match the character directly.
                    if key.code == KeyCode::Char('G') {
                        goto.open();
                        continue;
                    }

                    if let Some(action) = keymap.lookup(&key) {
                        match action {
                            Action::Refresh => {
                                handle_refresh(&mut app, &mut state);
                            }
                            Action::DialogConfirm => {
                                // Enter: navigate into a directory, open the
                                // stat popup for files, or toggle expand/collapse.
                                handle_enter(
                                    &mut app,
                                    &mut state,
                                    &mut nav_error,
                                    us_catalog.as_ref(),
                                    ca_catalog.as_ref(),
                                    level_registry.as_ref(),
                                    &mut label_popup,
                                    &mut stat_popup,
                                );
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

/// Build a `StatPopupState` for the given file path.
///
/// Calls `SecureDirent::from_path` (TOCTOU-safe, fd-anchored) and
/// `tree_magic_mini::from_filepath` (display-only MIME detection).  On any
/// I/O failure the error path is used — the popup still opens but shows the
/// error rows rather than silently doing nothing.
///
/// Extracted from `handle_enter` to stay within the 100-line function limit.
///
/// ## Compliance
///
/// - **NIST SP 800-53 AU-3**: Audit card data is built in a single, traceable
///   construction step per file node selection.
fn build_stat_popup_state(file_path: &Path) -> StatPopupState {
    let path_str = file_path.to_string_lossy().into_owned();

    #[cfg(debug_assertions)]
    let t0 = std::time::Instant::now();

    let dirent_result = umrs_selinux::secure_dirent::SecureDirent::from_path(file_path);

    #[cfg(debug_assertions)]
    log::debug!(
        "stat popup SecureDirent construction completed in {} µs",
        t0.elapsed().as_micros()
    );

    let mime: &str =
        tree_magic_mini::from_filepath(file_path).unwrap_or("application/octet-stream");

    let app = match &dirent_result {
        Ok(dirent) => FileStatApp::from_dirent(dirent, mime),
        Err(e) => FileStatApp::from_error(&path_str, e),
    };

    StatPopupState {
        app,
        active_tab: 0,
        scroll: [0; 3],
    }
}

/// Handle an Enter keypress in the TUI viewer.
///
/// Priority of actions:
///
/// 1. **Group header with a known marking** — open the label detail popup
///    using the marking stored in `metadata["marking"]`.  Falls through to
///    expand/collapse when no catalog data is available for the marking.
/// 2. **File node with a non-sentinel marking** — open the label detail popup
///    for the file's applied marking.  Falls through to expand/collapse when
///    no catalog data is available.
/// 3. **Directory node** — navigate into the directory (re-scan).
/// 4. **Everything else** — toggle expand/collapse on the selected tree node.
///
/// On navigation error the display stays on the current listing — no crash, no
/// silent state corruption.
///
/// ## Compliance
///
/// - **NIST SP 800-53 AC-3**: Navigation is strictly read-only; this function
///   never creates, modifies, or deletes directory entries.
/// - **NIST SP 800-53 AC-16**: Pressing Enter on a labeled node opens the full
///   regulatory definition for the applied marking.
#[expect(
    clippy::too_many_lines,
    reason = "enter-key dispatch is a single state machine; splitting would scatter the priority logic"
)]
#[expect(
    clippy::too_many_arguments,
    reason = "enter-key handler is a single dispatch function; all arguments are distinct \
              state owned by the event loop and cannot be merged without obscuring ownership"
)]
fn handle_enter(
    app: &mut DirViewerApp,
    state: &mut ViewerState,
    nav_error: &mut Option<(String, String)>,
    us_catalog: Option<&Catalog>,
    ca_catalog: Option<&Catalog>,
    level_registry: Option<&LevelRegistry>,
    label_popup: &mut Option<(MarkingDetailData, u16)>,
    stat_popup: &mut Option<StatPopupState>,
) {
    let Some(entry) = state.tree.display_list.get(state.selected_index) else {
        return;
    };
    let path = entry.path.clone();
    let Some(node) = state.tree.node_ref(&path) else {
        return;
    };

    // ── Group header with a known marking → open label detail popup ──────────
    if node.metadata.get("kind").map(String::as_str) == Some("group_header")
        && let Some(marking_str) = node.metadata.get("marking")
        && let Some(data) = lookup_marking_detail(marking_str, us_catalog, ca_catalog)
    {
        *label_popup = Some((data, 0));
        return;
    }

    // ── Group header with a system-level marking (not in CUI catalog) ─────────
    // setrans.conf defines well-known system sensitivity labels (SystemLow,
    // SystemHigh, SystemLow-SystemHigh) that are not CUI categories and will
    // never appear in the catalog.  Show an informational popup so the operator
    // gets useful context rather than silent no-op behavior.
    if node.metadata.get("kind").map(String::as_str) == Some("group_header")
        && let Some(marking_str) = node.metadata.get("marking")
    {
        // SystemLow name and description come from LEVELS.json s0 when available.
        // SystemHigh and SystemLow-SystemHigh are setrans.conf range aliases with
        // no direct LEVELS.json equivalent; their text stays as compile-time constants.
        let system_info: Option<(String, String)> = match marking_str.as_str() {
            "SystemLow" => {
                let (name, description) = level_registry
                    .and_then(|r| r.level("s0"))
                    .map(|def| (def.name.as_str(), def.description.as_str()))
                    .unwrap_or((
                        "SystemLow (s0)",
                        "Default operating system sensitivity level. Every file and process \
                         starts here under targeted SELinux policy. Files at SystemLow carry \
                         no MCS category assignment — they are routine unclassified content \
                         (binaries, libraries, configuration). \nA file with no category (bare \
                         s0) displays as SystemLow in setrans.conf translations. New files \
                         inherit the MCS range from their parent directory. \nIn MLS policy, s0 \
                         is the lowest sensitivity tier; s1-s3 provide Bell-LaPadula dominance \
                         above it.",
                    ));
                Some((name.to_owned(), description.to_owned()))
            }
            "SystemLow-SystemHigh" => Some((
                "SystemLow-SystemHigh (s0-s0:c0.c1023)".to_owned(),
                "Full MLS range spanning all sensitivity levels and all 1024 MCS \
                 categories. Processes with this range can access objects at any \
                 sensitivity level and any category combination. Typically assigned \
                 to trusted system daemons that must operate across all security \
                 boundaries (e.g., SELinux-aware services, audit infrastructure)."
                    .to_owned(),
            )),
            "SystemHigh" => Some((
                "SystemHigh (s0:c0.c1023)".to_owned(),
                "Maximum MCS category set — all 1024 categories included at the s0 \
                 sensitivity level. Under targeted policy this is the theoretical \
                 ceiling. Files or processes at SystemHigh have access to every \
                 MCS category. In practice, only system services that must read \
                 across all CUI categories operate at this level."
                    .to_owned(),
            )),
            _ => None,
        };
        if let Some((name, description)) = system_info {
            let data = MarkingDetailData {
                key: marking_str.clone(),
                name_en: name,
                description_en: description,
                designation: "system".to_owned(),
                ..MarkingDetailData::default()
            };
            *label_popup = Some((data, 0));
            return;
        }
    }
    // No catalog data and no system-level match — fall through to expand/collapse.

    // ── File node with a non-sentinel marking → open label detail popup ──────
    // Sentinels emitted by extract_selinux_strings for non-labeled states are
    // prefixed with `<` and are never valid catalog keys; skip them.
    if node.metadata.get("kind").map(String::as_str) != Some("group_header")
        && let Some(marking_str) = node.metadata.get("marking")
        && !marking_str.starts_with('<')
        && !marking_str.is_empty()
        && let Some(data) = lookup_marking_detail(marking_str, us_catalog, ca_catalog)
    {
        *label_popup = Some((data, 0));
        return;
    }

    // ── File node (non-directory, non-group-header) → stat popup ─────────────
    if node.metadata.get("is_dir").map(String::as_str) != Some("true")
        && node.metadata.get("kind").map(String::as_str) != Some("group_header")
        && node.metadata.get("is_parent_nav").map(String::as_str) != Some("true")
    {
        let name = node.metadata.get("name").map(String::as_str).unwrap_or("");
        let file_path = app.current_path().join(name);
        let mut popup_state = build_stat_popup_state(&file_path);

        // Look up the index group for the marking so the popup pill color
        // matches the directory listing group header color.
        if let Some(marking_str) = node.metadata.get("marking") {
            let group = find_index_group_for_marking(marking_str, us_catalog, ca_catalog);
            popup_state.app.marking_index_group = group;
        }

        *stat_popup = Some(popup_state);
        return;
    }

    // ── Directory navigation ──────────────────────────────────────────────────
    // Only navigate when the node represents a directory.
    if node.metadata.get("is_dir").map(String::as_str) != Some("true") {
        // Group header with no catalog data: delegate to expand/collapse.
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
            *nav_error = Some((new_path.display().to_string(), e.to_string()));
        }
    }
}

/// Resolve a user-typed path in the "Go to" bar and navigate to it.
///
/// Resolution rules (in order):
/// 1. If the query is empty, dismiss the bar with no error.
/// 2. If the query starts with `~`, expand to `$HOME`.
/// 3. Relative paths are resolved against the current directory.
/// 4. The result is canonicalized (symlinks followed).
/// 5. If the canonicalized target is a regular file, its parent directory
///    is used — Jamie's rule: "if they accidently enter a filename, we
///    open the parent directory."
/// 6. If the target does not exist or is not accessible, the bar is kept
///    open with an inline error message; the listing does not change.
///
/// On success the bar is closed and the tree is reloaded.
///
/// ## Compliance
///
/// - **NIST SP 800-53 AC-3**: Navigation is strictly read-only; no path
///   operation mutates the filesystem.
fn handle_goto_submit(
    app: &mut DirViewerApp,
    state: &mut ViewerState,
    goto: &mut GotoBar,
    nav_error: &mut Option<(String, String)>,
) {
    let query = goto.query.trim();
    if query.is_empty() {
        goto.close();
        return;
    }

    let resolved = resolve_goto_path(query, app.current_path());

    let canonical = match std::fs::canonicalize(&resolved) {
        Ok(p) => p,
        Err(e) => {
            goto.error = Some(format!("{}: {e}", resolved.display()));
            return;
        }
    };

    // If the operator typed a filename, fall back to its parent directory.
    let target = if canonical.is_dir() {
        canonical
    } else if let Some(p) = canonical.parent() {
        p.to_path_buf()
    } else {
        goto.error = Some("no parent directory".to_owned());
        return;
    };

    match app.navigate_to(&target) {
        Ok(tree) => {
            state.load_tree(tree);
            goto.close();
        }
        Err(e) => {
            // Navigation failed (typically EACCES).  Show the permission
            // denied overlay and keep the goto bar open so the operator
            // can retype.
            *nav_error = Some((target.display().to_string(), e.to_string()));
            goto.error = Some("permission denied".to_owned());
        }
    }
}

/// Resolve a user-typed path string against the current directory.
///
/// Handles `~` expansion and relative paths.  Does not touch the
/// filesystem — callers must canonicalize separately.
fn resolve_goto_path(query: &str, cwd: &Path) -> PathBuf {
    // DIRECT-IO-EXCEPTION: HOME is read for UX-only tilde expansion in the
    // goto bar. No security decision is derived from this value. HOME is
    // user-controlled and not validated beyond basic path existence.
    if let Some(rest) = query.strip_prefix('~')
        && let Ok(home) = std::env::var("HOME")
    {
        let trimmed = rest.strip_prefix('/').unwrap_or(rest);
        if trimmed.is_empty() {
            return PathBuf::from(home);
        }
        return PathBuf::from(home).join(trimmed);
    }
    let p = Path::new(query);
    if p.is_absolute() {
        p.to_path_buf()
    } else {
        cwd.join(p)
    }
}

/// Tab-completion for the "Go to" bar.
///
/// Splits the current query into a parent directory and a filename stem,
/// lists the parent, and finds every entry whose name starts with the stem
/// (case-sensitive).  Behaviour:
///
/// - **0 matches** → sets an inline error "no match", query unchanged.
/// - **1 match** → replaces the stem with the full name; appends `/` if
///   the match is a directory.
/// - **N matches** → completes the stem to the longest common prefix of
///   all matches (no change if the stem is already that prefix).
///
/// # Direct I/O Exception
///
/// `std::fs::read_dir` is used here rather than the UMRS listing pipeline.
/// This is a justified exception because:
///
/// - The System State Read Prohibition covers system paths (`/etc/`, `/proc/`,
///   `/sys/`, `/run/`). Tab completion operates on **user-directed arbitrary
///   paths** — not system state — making the prohibition inapplicable.
/// - The operation is UX-only (no security decision, no audit surface).
///
/// DIRECT-IO-EXCEPTION: user-directed arbitrary path traversal for keystroke
/// completion. Reviewed 2026-04-08.
fn complete_goto_query(goto: &mut GotoBar) {
    // Work on an owned copy so we can borrow `goto` mutably at the end.
    let query = goto.query.clone();
    if query.is_empty() {
        return;
    }

    // Split into (parent_dir, stem).  Handle trailing slash — "foo/" means
    // "list foo, stem is empty".  Handle `~` and relative paths via the
    // existing resolver.
    let (parent_str, stem) = match query.rfind('/') {
        Some(idx) => (&query[..=idx], &query[idx + 1..]),
        None => ("", query.as_str()),
    };

    // Resolve the parent path.  An empty parent means "current directory"
    // only when the query itself has no slash; otherwise `/...` is root.
    let parent_path: PathBuf = if parent_str.is_empty() {
        PathBuf::from(".")
    } else if let Some(rest) = parent_str.strip_prefix('~') {
        // DIRECT-IO-EXCEPTION: HOME is read for UX-only tilde completion in the
        // goto bar. No security decision is derived from this value.
        if let Ok(home) = std::env::var("HOME") {
            let trimmed = rest.strip_prefix('/').unwrap_or(rest);
            PathBuf::from(home).join(trimmed)
        } else {
            PathBuf::from(parent_str)
        }
    } else {
        PathBuf::from(parent_str)
    };

    let read = match std::fs::read_dir(&parent_path) {
        Ok(r) => r,
        Err(e) => {
            goto.error = Some(format!("{}: {e}", parent_path.display()));
            return;
        }
    };

    // Collect (name, is_dir) for every entry whose name starts with `stem`.
    let mut matches: Vec<(String, bool)> = Vec::new();
    for entry in read.flatten() {
        let name = entry.file_name().to_string_lossy().into_owned();
        if name.starts_with(stem) {
            let is_dir = entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
            matches.push((name, is_dir));
        }
    }

    if matches.is_empty() {
        goto.error = Some("no match".to_owned());
        return;
    }

    if matches.len() == 1 {
        // Exact completion.  Append "/" for directories so the operator can
        // keep typing into the child.
        let (name, is_dir) = &matches[0];
        let mut completed = format!("{parent_str}{name}");
        if *is_dir {
            completed.push('/');
        }
        goto.query = completed;
        goto.error = None;
        return;
    }

    // Multiple matches: extend stem to the longest common prefix.
    let lcp = longest_common_prefix(matches.iter().map(|(n, _)| n.as_str()));
    if lcp.len() > stem.len() {
        goto.query = format!("{parent_str}{lcp}");
        goto.error = None;
    } else {
        // Already at the common prefix — hint at ambiguity.
        goto.error = Some(format!("{} matches", matches.len()));
    }
}

/// Compute the longest common prefix of an iterator of string slices.
///
/// Returns an owned `String`.  Operates on bytes (safe because all inputs
/// are valid UTF-8 and the prefix cut only happens at character boundaries
/// checked via `is_char_boundary`).
fn longest_common_prefix<'a, I: IntoIterator<Item = &'a str>>(iter: I) -> String {
    let mut it = iter.into_iter();
    let Some(first) = it.next() else {
        return String::new();
    };
    let mut prefix_len = first.len();
    for s in it {
        let bytes_a = first.as_bytes();
        let bytes_b = s.as_bytes();
        let limit = prefix_len.min(bytes_b.len());
        let mut i = 0;
        while i < limit && bytes_a[i] == bytes_b[i] {
            i += 1;
        }
        // Back off to a char boundary so multi-byte UTF-8 sequences aren't cut.
        while i > 0 && !first.is_char_boundary(i) {
            i -= 1;
        }
        prefix_len = i;
        if prefix_len == 0 {
            break;
        }
    }
    first[..prefix_len].to_owned()
}

/// Re-scan the current directory and reload the tree.
///
/// Called when the user presses `r` (Refresh).  On scan error the display
/// stays on the previous listing.
///
/// If a search filter is active, refresh also clears it — operators found
/// "how do I get back to the full listing" unclear when the only exit was
/// Esc while in search mode.  Making refresh a catch-all reset is the
/// least surprising behaviour.
///
/// ## Compliance
///
/// - **NIST SP 800-53 AU-3**: Re-scan updates the status bar timing so the
///   operator can confirm the listing is current.
fn handle_refresh(app: &mut DirViewerApp, state: &mut ViewerState) {
    // Clear any active or committed search filter before re-scanning so the
    // operator sees the full refreshed listing.  Handles both "bar still
    // open" and "query committed with Enter but filter still applied".
    if state.search_active || !state.search_query.is_empty() {
        state.search_active = false;
        state.search_query.clear();
        state.tree.clear_filter();
        state.tree.rebuild_display();
        state.selected_index = 0;
    }

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
        println!("{}", line.if_supports_color(Stream::Stdout, |t| t.dimmed()));
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
            let (owner, group) = resolve_owner_display(uid, gid);
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
            format!("{}", "I".if_supports_color(Stream::Stdout, |t| t.red()))
        } else {
            "I".to_owned()
        }
    } else if cfg.use_color {
        format!("{}", "-".if_supports_color(Stream::Stdout, |t| t.dimmed()))
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
            format!(
                "{}",
                "O".if_supports_color(Stream::Stdout, |t| t.style(Style::new().red().bold()))
            )
        } else {
            "O".to_owned()
        }
    } else if cfg.use_color {
        format!("{}", "-".if_supports_color(Stream::Stdout, |t| t.dimmed()))
    } else {
        "-".to_owned()
    };

    let v = if flags.contains(InodeSecurityFlags::IMA_PRESENT) {
        if cfg.use_color {
            format!("{}", "V".if_supports_color(Stream::Stdout, |t| t.green()))
        } else {
            "V".to_owned()
        }
    } else if cfg.use_color {
        format!("{}", "-".if_supports_color(Stream::Stdout, |t| t.dimmed()))
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
        let Rgb {
            r: tr,
            g: tg,
            b: tb,
        } = colors[2].fg;
        let Rgb {
            r: mr,
            g: mg,
            b: mb,
        } = colors[3].fg;
        let type_out =
            key.selinux_type.if_supports_color(Stream::Stdout, |t| t.truecolor(tr, tg, tb));
        let marking_out =
            key.marking.if_supports_color(Stream::Stdout, |t| t.truecolor(mr, mg, mb));
        return format!("{type_out} :: {marking_out} {fill}");
    }

    if key.selinux_type == "<restricted>" {
        let selinux_type = i18n::tr("<restricted>");
        // Dim + italic + underline on the entire header line.
        let body = format!("{selinux_type} :: {} {fill}", key.marking);
        format!(
            "{}",
            body.if_supports_color(Stream::Stdout, |t| t
                .style(Style::new().dimmed().italic().underline()))
        )
    } else {
        // BE CAREFUL HERE! This combination of reverse, colors, and unicode was challenging!
        //
        // The header is built from styled segments to preserve the exact terminal
        // rendering: black-on-cyan for the type field, a reverse-video transition
        // glyph, and a reverse-video marking field with an underlined tail fill.
        //
        // Each segment is styled individually and concatenated; owo-colors appends
        // the SGR reset after each segment automatically, so no manual RESET is needed.
        let type_field = format!(" {:20} ", key.selinux_type);
        let seg_type = format!(
            "{}",
            type_field.if_supports_color(Stream::Stdout, |t| t.black().on_cyan())
        );
        // Transition glyph: cyan-on-black → visually bridges BLACK_ON_CYAN → REVERSE
        let seg_transition = format!(
            "{}",
            "\u{1FB6C}".if_supports_color(Stream::Stdout, |t| t.cyan().on_black())
        );
        let marking_field = format!("\u{1FB6C}{:^20} ", key.marking);
        let seg_marking = format!(
            "{}",
            marking_field.if_supports_color(Stream::Stdout, |t| t.reversed())
        );
        let seg_tail = format!(
            "{}",
            format!("\u{1FB6C}{fill}").if_supports_color(Stream::Stdout, |t| t.underline())
        );
        format!("{seg_type}{seg_transition}{seg_marking}{seg_tail}")
    }
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
