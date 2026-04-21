// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams
//
// NOTE: umask configuration is the caller's responsibility.
// This binary does not set umask; deploy with an appropriate service unit
// or shell profile that enforces umask 0o027 before launching.
//
//! # umrs-label — Security Label Registry Browser
//!
//! Displays and browses UMRS CUI (Controlled Unclassified Information) and
//! Canadian Protected label catalogs loaded from JSON files.
//!
//! ## Output Modes
//!
//! - **TUI** (default when stdout is a TTY): interactive tree browser with
//!   detail panel, search, and security posture header.
//! - **CLI** (`--cli` or non-TTY stdout): human-readable grouped text listing.
//! - **JSON** (`--json`): reserved for future machine-readable output.
//!
//! ## Catalog File Paths
//!
//! Under the FHS 2.3 §4.11 layout, JSON reference databases live at
//! `/opt/umrs/share/umrs/` (the package's `/usr/share/<pkg>` analogue).
//! Defaults resolve in this order:
//!
//! 1. Explicit `--us-catalog` / `--ca-catalog` flags
//! 2. `UMRS_CONFIG_DIR` environment variable — `<dir>/US-CUI-LABELS.json`
//! 3. `/opt/umrs/share/umrs/US-CUI-LABELS.json` (install default)
//! 4. `config/US-CUI-LABELS.json` in CWD (development convenience)
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AC-16**: Security Attributes — accurate display of all
//!   CUI marking fields including designation, handling, and warning statements.
//! - **NIST SP 800-53 AU-3**: Audit Record Content — the TUI header carries
//!   hostname, OS, SELinux mode, and FIPS state on every rendered frame.
//! - **NIST SP 800-53 AC-3**: The browser is unconditionally read-only; no
//!   catalog mutation is possible through the interface.
//! - **NIST SP 800-53 SI-10**: Input Validation — clap validates all CLI
//!   arguments at entry, rejecting unknown flags and enforcing type constraints
//!   before any catalog I/O or TUI initialization occurs.
//!
#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![deny(clippy::unwrap_used)]
#![allow(clippy::option_if_let_else)]
#![allow(clippy::map_unwrap_or)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::too_many_lines)]

use std::collections::BTreeMap;
use std::io::{self, IsTerminal as _};
use std::time::Duration;

use clap::Parser;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use umrs_core::i18n;

use umrs_labels::cui::catalog;
use umrs_labels::tui::app::{DetailContent, LabelRegistryApp, Panel};
use umrs_labels::tui::render::render_label_registry;
use umrs_ui::indicators::{build_header_context, detect_os_name};
use umrs_ui::keymap::{Action, KeyMap};
use umrs_ui::theme::Theme;
use umrs_ui::viewer::ViewerState;

// ---------------------------------------------------------------------------
// CLI argument definition
// ---------------------------------------------------------------------------

/// Browse and display the UMRS security label registry.
///
/// Loads CUI (Controlled Unclassified Information) and Canadian Protected
/// label catalogs from JSON files and renders them as an interactive TUI
/// browser or a plain text listing.
///
/// ## Compliance
///
/// - **NIST SP 800-53 SI-10**: Input Validation — clap validates all arguments
///   at process entry, rejecting unknown flags before any catalog I/O occurs.
/// - **NIST SP 800-53 AC-3**: The browser is unconditionally read-only.
#[derive(Parser)]
#[command(
    name = "umrs-label",
    version,
    about = "Browse and display the UMRS security label registry (CUI and Canadian Protected catalogs)",
    long_about = "Browse and display the UMRS security label registry.\n\n\
        Loads CUI (Controlled Unclassified Information) and Canadian Protected \
        label catalogs from JSON files and renders them as an interactive TUI \
        browser or a plain text listing.\n\n\
        When stdout is a terminal the interactive TUI launches automatically. \
        Use --cli to force plain-text output or pipe to another program."
)]
struct Args {
    /// Path to the US CUI label catalog JSON file.
    ///
    /// Default: `/opt/umrs/share/umrs/US-CUI-LABELS.json` (FHS 2.3 §4.11).
    /// Override priority: `--us-catalog` flag → `UMRS_CONFIG_DIR` env var → default.
    #[arg(long, default_value = "/opt/umrs/share/umrs/US-CUI-LABELS.json")]
    us_catalog: String,

    /// Path to the Canadian Protected label catalog JSON file.
    ///
    /// Default: `/opt/umrs/share/umrs/CANADIAN-PROTECTED.json` (FHS 2.3 §4.11).
    /// The Canadian catalog is optional; if the file is absent the tool
    /// continues with US CUI labels only.
    #[arg(long, default_value = "/opt/umrs/share/umrs/CANADIAN-PROTECTED.json")]
    ca_catalog: String,

    /// Force plain-text CLI output instead of the interactive TUI.
    ///
    /// This flag is implied when stdout is not a terminal (e.g., when piping
    /// output to another program).
    #[arg(long)]
    cli: bool,

    /// Emit machine-readable JSON output.
    ///
    /// JSON output support is reserved for a future implementation phase.
    #[arg(long)]
    json: bool,

    /// Show step-by-step progress on stderr.
    ///
    /// Narrates config paths, catalog counts, locale resolution, and any
    /// trust-gate results so operators can diagnose issues without enabling
    /// debug logging. NIST SP 800-53 SI-11.
    #[arg(long, short)]
    verbose: bool,
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

fn main() -> std::process::ExitCode {
    // Initialize gettext catalog before any i18n::tr() calls.
    // NIST SP 800-53 SI-11 — locale is resolved at startup, not on demand.
    i18n::init("umrs-label");

    if let Ok(logger) = systemd_journal_logger::JournalLog::new() {
        let _ = logger.install();
        log::set_max_level(log::LevelFilter::Info);
    }

    let args = Args::parse();

    // Enable verbose progress output to stderr. All verbose output goes to
    // stderr so it does not interfere with --json or piped stdout.
    // NIST SP 800-53 SI-11.
    let verbose = args.verbose;
    macro_rules! verbose {
        ($($arg:tt)*) => {
            if verbose {
                eprintln!("  [umrs-label] {}", format_args!($($arg)*));
            }
        };
    }

    let json_mode = args.json;
    let cli_mode = args.cli;

    // Resolve catalog paths using the documented override chain. Under the
    // FHS 2.3 §4.11 layout the reference databases live flat under
    // /opt/umrs/share/umrs/ — there is no us/ or ca/ subdirectory.
    //
    //   1. Explicit --us-catalog / --ca-catalog flags
    //   2. UMRS_CONFIG_DIR environment variable → <dir>/US-CUI-LABELS.json
    //   3. /opt/umrs/share/umrs/  (install default, FHS 2.3 §4.11)
    //   4. config/ in CWD  (development convenience, pre-install)
    //
    // NIST SP 800-53 CM-6 — configuration path resolved before catalog I/O.
    let default_us = "/opt/umrs/share/umrs/US-CUI-LABELS.json";
    let default_ca = "/opt/umrs/share/umrs/CANADIAN-PROTECTED.json";
    let cwd_us = "config/US-CUI-LABELS.json";
    let cwd_ca = "config/CANADIAN-PROTECTED.json";

    let us_catalog_path = if args.us_catalog != default_us {
        // Explicit override.
        args.us_catalog
    } else if let Ok(dir) = std::env::var("UMRS_CONFIG_DIR") {
        format!("{dir}/US-CUI-LABELS.json")
    } else if std::path::Path::new(default_us).exists() {
        default_us.to_owned()
    } else {
        cwd_us.to_owned()
    };

    let ca_catalog_path = if args.ca_catalog != default_ca {
        args.ca_catalog
    } else if let Ok(dir) = std::env::var("UMRS_CONFIG_DIR") {
        format!("{dir}/CANADIAN-PROTECTED.json")
    } else if std::path::Path::new(default_ca).exists() {
        default_ca.to_owned()
    } else {
        cwd_ca.to_owned()
    };

    verbose!("US catalog: {}", us_catalog_path);
    verbose!("CA catalog: {}", ca_catalog_path);

    // Load US catalog (required).
    let us_catalog = match catalog::load_catalog(&us_catalog_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("[FAIL] Could not load US catalog: {e}");
            return std::process::ExitCode::from(2);
        }
    };
    verbose!(
        "US catalog loaded: {} markings",
        us_catalog.iter_markings().count()
    );

    // Load Canadian catalog (optional — log a warning but continue if absent).
    let ca_catalog = match catalog::load_catalog(&ca_catalog_path) {
        Ok(c) => {
            verbose!("CA catalog loaded: {} markings", c.iter_markings().count());
            Some(c)
        }
        Err(e) => {
            log::warn!("Canadian catalog unavailable: {e}");
            verbose!("CA catalog unavailable: {e}");
            None
        }
    };

    if json_mode {
        eprintln!("[INFO] --json output is not yet implemented for umrs-label");
        return std::process::ExitCode::SUCCESS;
    }

    if cli_mode || !io::stdout().is_terminal() {
        verbose!("Mode: CLI");
        run_cli(&us_catalog, ca_catalog.as_ref());
        return std::process::ExitCode::SUCCESS;
    }

    verbose!("Mode: TUI");
    run_tui(us_catalog, ca_catalog);
    std::process::ExitCode::SUCCESS
}

// ---------------------------------------------------------------------------
// CLI output path
// ---------------------------------------------------------------------------

/// Render the catalog as a plain grouped text listing.
fn run_cli(us_catalog: &catalog::Catalog, ca_catalog: Option<&catalog::Catalog>) {
    print_catalog_listing(us_catalog);
    if let Some(ca) = ca_catalog {
        println!();
        print_catalog_listing(ca);
    }
}

fn print_catalog_listing(cat: &catalog::Catalog) {
    if let Some(meta) = &cat.metadata {
        println!();
        println!(
            "  {} ({})",
            meta.catalog_name.en(),
            meta.country_code.as_deref().unwrap_or("??")
        );
        println!("  Version {}", meta.version);
    }
    println!("  {} markings loaded", cat.iter_markings().count());
    println!();

    let mut groups: BTreeMap<String, Vec<(&String, &catalog::Marking)>> = BTreeMap::new();
    for (key, marking) in cat.iter_markings() {
        let group_name = marking.index_group.clone().unwrap_or_else(|| "(No Group)".to_owned());
        groups.entry(group_name).or_default().push((key, marking));
    }
    for entries in groups.values_mut() {
        entries.sort_by(|a, b| a.0.cmp(b.0));
    }
    for (group, entries) in &groups {
        println!("  {group}");
        println!("  {}", "-".repeat(group.len()));
        for (key, marking) in entries {
            let designation = marking.designation.as_deref().unwrap_or("");
            let tag = if designation == "specified" {
                " [SP]"
            } else {
                ""
            };
            println!("    {key}  {}{tag}", marking.name.en());
        }
        println!();
    }

    // Dissemination controls
    if cat.has_dissemination_controls() {
        println!("  Dissemination Controls");
        println!("  {}", "-".repeat(22));
        let mut dc_entries: Vec<(&String, &catalog::DisseminationControl)> =
            cat.iter_dissemination_controls().collect();
        dc_entries.sort_by(|a, b| a.0.cmp(b.0));
        for (key, dc) in dc_entries {
            println!("    {}  {}", key, dc.name.en());
        }
        println!();
    }
}

// ---------------------------------------------------------------------------
// TUI interactive path
// ---------------------------------------------------------------------------

/// Run the interactive TUI security label registry browser.
///
/// NIST SP 800-53 AC-3 — navigation is read-only; no catalog entries are
/// created, deleted, or modified through the browser.
/// NIST SP 800-53 AU-3 — the header carries tool identity, hostname, and
/// security posture on every rendered frame.
fn run_tui(us_catalog: catalog::Catalog, ca_catalog: Option<catalog::Catalog>) {
    let app = LabelRegistryApp::new(us_catalog, ca_catalog);

    // Build the tree and load into viewer state.
    let tree = app.build_tree();
    let mut state = ViewerState::new(1); // single virtual "tab" (no tab bar shown)
    state.load_tree(tree);

    let mut keymap = KeyMap::default();

    // umrs-label uses Tab/BackTab for panel switching (Tree ↔ Detail), not tab
    // navigation. Override the default NextTab/PrevTab bindings.
    keymap.bind(
        KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE),
        Action::PanelSwitch,
    );
    keymap.bind(
        KeyEvent::new(KeyCode::BackTab, KeyModifiers::SHIFT),
        Action::PanelSwitch,
    );

    // Right/Left expand/collapse in the tree, overriding default NextTab/PrevTab.
    keymap.bind(
        KeyEvent::new(KeyCode::Right, KeyModifiers::NONE),
        Action::Expand,
    );
    keymap.bind(
        KeyEvent::new(KeyCode::Left, KeyModifiers::NONE),
        Action::Collapse,
    );

    // Vim-style expand/collapse bindings (l = right, h = left).
    keymap.bind(
        KeyEvent::new(KeyCode::Char('l'), KeyModifiers::NONE),
        Action::Expand,
    );
    keymap.bind(
        KeyEvent::new(KeyCode::Char('h'), KeyModifiers::NONE),
        Action::Collapse,
    );

    // Space behaves the same as Enter (expand branch or load leaf detail),
    // not just expand. Override the default Space = Expand binding.
    keymap.bind(
        KeyEvent::new(KeyCode::Char(' '), KeyModifiers::NONE),
        Action::DialogConfirm,
    );

    // Uppercase Q also quits.
    keymap.bind(
        KeyEvent::new(KeyCode::Char('Q'), KeyModifiers::NONE),
        Action::Quit,
    );
    keymap.bind(
        KeyEvent::new(KeyCode::Char('Q'), KeyModifiers::SHIFT),
        Action::Quit,
    );

    // NO_COLOR environment variable compliance (https://no-color.org/).
    // When set (to any non-empty value or even empty string), color output must
    // be suppressed. `var_os` is used so the value is never decoded — presence
    // alone is the signal, consistent with the NO_COLOR specification.
    // NIST SP 800-53 SI-11 / WCAG 1.4.1 — accessible output in color-restricted
    // environments (audit pipelines, screen readers, legacy terminals).
    let theme = if std::env::var_os("NO_COLOR").is_some() {
        Theme::no_color()
    } else {
        Theme::dark()
    };

    // Snapshot the security posture header once at startup.  OS name is read
    // through the umrs-platform OsDetector pipeline, which routes through
    // provenance-verified SecureReader paths rather than raw /etc/os-release I/O.
    let ctx = build_header_context(
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        detect_os_name(),
    );

    // TUI state — detail panel content + scroll + active panel.
    let mut detail_content = DetailContent::None;
    let mut detail_scroll: u16 = 0;
    let mut active_panel = Panel::Tree;
    let mut help_active = false;

    let mut terminal = ratatui::init();

    run_event_loop(
        &mut terminal,
        &app,
        &mut state,
        &ctx,
        &theme,
        &keymap,
        &mut detail_content,
        &mut detail_scroll,
        &mut active_panel,
        &mut help_active,
    );

    ratatui::restore();
}

/// Main event loop for the TUI browser.
///
/// Separated from `run_tui` to allow `ratatui::restore()` to always run on
/// exit regardless of how the loop terminates.
#[expect(
    clippy::too_many_arguments,
    reason = "TUI event loop aggregates all mutable state; extracting would require a wrapper struct"
)]
fn run_event_loop(
    terminal: &mut ratatui::DefaultTerminal,
    app: &LabelRegistryApp,
    state: &mut ViewerState,
    ctx: &umrs_ui::app::HeaderContext,
    theme: &Theme,
    keymap: &KeyMap,
    detail_content: &mut DetailContent,
    detail_scroll: &mut u16,
    active_panel: &mut Panel,
    help_active: &mut bool,
) {
    loop {
        if let Err(e) = terminal.draw(|f| {
            render_label_registry(
                f,
                f.area(),
                app,
                state,
                ctx,
                theme,
                detail_content,
                *detail_scroll,
                *active_panel,
            );
            if *help_active {
                render_help_overlay(f, f.area(), theme);
            }
        }) {
            log::error!("terminal draw error: {e}");
            break;
        }

        let Ok(ready) = event::poll(Duration::from_millis(100)) else {
            break;
        };
        if !ready {
            continue;
        }
        let Ok(ev) = event::read() else {
            break;
        };

        let Event::Key(key) = ev else {
            continue;
        };

        // Help overlay: owns all input while visible.
        if *help_active {
            match key.code {
                KeyCode::Char('?') | KeyCode::Esc | KeyCode::Enter => {
                    *help_active = false;
                }
                _ => {}
            }
            continue;
        }

        // Search mode: characters go to the query buffer.
        if state.search_active {
            match key.code {
                KeyCode::Esc => {
                    state.search_active = false;
                    state.search_query.clear();
                    state.tree.clear_filter();
                    state.tree.rebuild_display();
                    state.selected_index = 0;
                }
                KeyCode::Enter => {
                    state.search_active = false;
                }
                KeyCode::Backspace => {
                    state.pop_search_char();
                }
                KeyCode::Char(ch) => {
                    state.push_search_char(ch);
                }
                _ => {}
            }
            continue;
        }

        // Normal mode — route through the shared KeyMap.
        match keymap.lookup(&key) {
            Some(Action::Quit) => break,

            Some(Action::ShowHelp) => {
                *help_active = true;
            }

            Some(Action::PanelSwitch) => {
                *active_panel = match *active_panel {
                    Panel::Tree => Panel::Detail,
                    Panel::Detail => Panel::Tree,
                };
                *detail_scroll = 0;
            }

            Some(Action::ScrollUp) => {
                if *active_panel == Panel::Detail {
                    *detail_scroll = detail_scroll.saturating_sub(1);
                } else if state.selected_index > 0 {
                    state.selected_index -= 1;
                }
            }

            Some(Action::ScrollDown) => {
                if *active_panel == Panel::Detail {
                    *detail_scroll = detail_scroll.saturating_add(1);
                } else {
                    let max = state.tree.display_count().saturating_sub(1);
                    if state.selected_index < max {
                        state.selected_index += 1;
                    }
                }
            }

            Some(Action::PageUp) => {
                if *active_panel == Panel::Detail {
                    *detail_scroll = detail_scroll.saturating_sub(10);
                } else {
                    state.selected_index = state.selected_index.saturating_sub(10);
                }
            }

            Some(Action::PageDown) => {
                if *active_panel == Panel::Detail {
                    *detail_scroll = detail_scroll.saturating_add(10);
                } else {
                    let max = state.tree.display_count().saturating_sub(1);
                    state.selected_index = state.selected_index.saturating_add(10).min(max);
                }
            }

            // DialogConfirm covers Enter and Space (both bound in keymap setup above).
            // Expands branches AND loads detail for leaf nodes.
            Some(Action::DialogConfirm) => {
                handle_enter(app, state, detail_content, detail_scroll);
            }

            // Expand covers Right, l — purely expand, no detail loading.
            Some(Action::Expand) => {
                expand_selected(state);
            }

            Some(Action::Collapse) => {
                collapse_selected(state);
            }

            Some(Action::Search) => {
                state.search_active = true;
                state.search_query.clear();
            }

            _ => {
                // Ctrl+C fallback — not representable as a standalone keymap binding
                // because the modifier pattern overlaps with the Char dispatch.
                // Handle it here as a safety net.
                if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
                    break;
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Action helpers
// ---------------------------------------------------------------------------

/// Expand the currently selected tree node (branch) or show details (leaf).
fn expand_selected(state: &mut ViewerState) {
    if let Some(entry) = state.tree.display_list.get(state.selected_index) {
        let path = entry.path.clone();
        state.tree.expand(&path);
        state.tree.rebuild_display();
    }
}

/// Collapse the currently selected tree node.
fn collapse_selected(state: &mut ViewerState) {
    if let Some(entry) = state.tree.display_list.get(state.selected_index) {
        let path = entry.path.clone();
        state.tree.collapse(&path);
        state.tree.rebuild_display();
    }
}

/// Handle the Enter key: expand/collapse branches, load details for leaves.
fn handle_enter(
    app: &LabelRegistryApp,
    state: &mut ViewerState,
    detail_content: &mut DetailContent,
    detail_scroll: &mut u16,
) {
    let Some(entry) = state.tree.display_list.get(state.selected_index) else {
        return;
    };
    let path = entry.path.clone();

    // Walk to the node.
    let mut nodes = &state.tree.roots;
    let mut node_ref = None;
    for &idx in &path {
        if let Some(n) = nodes.get(idx) {
            node_ref = Some(n);
            nodes = &n.children;
        }
    }

    let Some(node) = node_ref else {
        return;
    };

    // Determine node kind from metadata.
    let kind = node.metadata.get("kind").map(String::as_str).unwrap_or("");
    let node_key = node.metadata.get("key").cloned().unwrap_or_default();

    match kind {
        "catalog_root" => {
            // Show catalog metadata in detail panel.
            let is_us = node.metadata.get("is_us").map(|v| v == "1").unwrap_or(true);
            let rows = if is_us {
                app.us_catalog_metadata()
            } else {
                app.ca_catalog_metadata().unwrap_or_default()
            };
            *detail_content = DetailContent::CatalogMetadata(rows);
            *detail_scroll = 0;
        }

        "group" | "dc_branch" => {
            // Toggle expand/collapse on branch; also show group summary.
            let name = node
                .metadata
                .get("group_name")
                .cloned()
                .unwrap_or_else(|| node.label.trim_start_matches("Group: ").to_owned());
            let count_str = node.metadata.get("count").cloned().unwrap_or_default();
            let count = count_str.parse::<usize>().unwrap_or(0);
            *detail_content = DetailContent::Group {
                name,
                count,
            };
            *detail_scroll = 0;

            if node.expanded {
                state.tree.collapse(&path);
            } else {
                state.tree.expand(&path);
            }
            state.tree.rebuild_display();
        }

        "marking_leaf" => {
            // Show marking detail.
            // Determine which catalog the marking belongs to by checking the
            // top-level root index (path[0] == 0 → US, path[0] == 1 → CA).
            let is_us = path.first().copied() == Some(0);
            let detail = if is_us {
                app.marking_detail_us(&node_key)
            } else {
                app.marking_detail_ca(&node_key)
            };
            if let Some(d) = detail {
                let prov = app.catalog_provenance(is_us);
                *detail_content = DetailContent::Marking(d, prov);
                *detail_scroll = 0;
            }
        }

        "dc_leaf" => {
            if let Some(d) = app.dissemination_detail(&node_key) {
                let prov = app.catalog_provenance(true);
                *detail_content = DetailContent::DisseminationControl(d, prov);
                *detail_scroll = 0;
            }
        }

        _ => {}
    }
}

// ---------------------------------------------------------------------------
// Help overlay
// ---------------------------------------------------------------------------

/// Render a simple modal help overlay.
///
/// Displayed when `?` is pressed. Dismissed by `?`, `Esc`, or `Enter`.
fn render_help_overlay(frame: &mut ratatui::Frame, area: ratatui::layout::Rect, theme: &Theme) {
    use ratatui::widgets::Clear;

    let help_width = 60u16.min(area.width.saturating_sub(4));
    let help_height = 18u16.min(area.height.saturating_sub(4));
    let x = (area.width.saturating_sub(help_width)) / 2;
    let y = (area.height.saturating_sub(help_height)) / 2;

    let popup_area = ratatui::layout::Rect {
        x: area.x + x,
        y: area.y + y,
        width: help_width,
        height: help_height,
    };

    frame.render_widget(Clear, popup_area);

    let block = ratatui::widgets::Block::default()
        .title(" Help — Key Bindings ")
        .borders(ratatui::widgets::Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .border_style(theme.border);
    let inner = block.inner(popup_area);
    frame.render_widget(block, popup_area);

    let lines = vec![
        ratatui::text::Line::from(""),
        ratatui::text::Line::from(vec![
            ratatui::text::Span::styled("  ↑ / k        ", theme.header_field),
            ratatui::text::Span::styled(i18n::tr("Move up"), theme.data_value),
        ]),
        ratatui::text::Line::from(vec![
            ratatui::text::Span::styled("  ↓ / j        ", theme.header_field),
            ratatui::text::Span::styled(i18n::tr("Move down"), theme.data_value),
        ]),
        ratatui::text::Line::from(vec![
            ratatui::text::Span::styled("  ← / h        ", theme.header_field),
            ratatui::text::Span::styled(i18n::tr("Collapse node"), theme.data_value),
        ]),
        ratatui::text::Line::from(vec![
            ratatui::text::Span::styled("  → / l        ", theme.header_field),
            ratatui::text::Span::styled(i18n::tr("Expand node"), theme.data_value),
        ]),
        ratatui::text::Line::from(vec![
            ratatui::text::Span::styled("  PgUp/PgDn    ", theme.header_field),
            ratatui::text::Span::styled(i18n::tr("Scroll active panel"), theme.data_value),
        ]),
        ratatui::text::Line::from(""),
        ratatui::text::Line::from(vec![
            ratatui::text::Span::styled("  Enter        ", theme.header_field),
            ratatui::text::Span::styled(i18n::tr("Show details / toggle branch"), theme.data_value),
        ]),
        ratatui::text::Line::from(vec![
            ratatui::text::Span::styled("  Tab          ", theme.header_field),
            ratatui::text::Span::styled(i18n::tr("Switch focus (Tree ↔ Detail)"), theme.data_value),
        ]),
        ratatui::text::Line::from(vec![
            ratatui::text::Span::styled("  /            ", theme.header_field),
            ratatui::text::Span::styled(i18n::tr("Search / filter catalog"), theme.data_value),
        ]),
        ratatui::text::Line::from(vec![
            ratatui::text::Span::styled("  Esc          ", theme.header_field),
            ratatui::text::Span::styled(i18n::tr("Cancel search"), theme.data_value),
        ]),
        ratatui::text::Line::from(""),
        ratatui::text::Line::from(vec![
            ratatui::text::Span::styled("  ?            ", theme.header_field),
            ratatui::text::Span::styled(i18n::tr("Toggle this help"), theme.data_value),
        ]),
        ratatui::text::Line::from(vec![
            ratatui::text::Span::styled("  q / Esc      ", theme.header_field),
            ratatui::text::Span::styled(i18n::tr("Quit"), theme.data_value),
        ]),
        ratatui::text::Line::from(""),
        ratatui::text::Line::from(ratatui::text::Span::styled(
            "  Press ? or Esc to close",
            theme.data_key,
        )),
    ];

    frame.render_widget(ratatui::widgets::Paragraph::new(lines), inner);
}
