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
//! Defaults to `config/us/US-CUI-LABELS.json` and
//! `config/ca/CANADIAN-PROTECTED.json` relative to the current directory.
//! Override with `--us-catalog` and `--ca-catalog`.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AC-16**: Security Attributes — accurate display of all
//!   CUI marking fields including designation, handling, and warning statements.
//! - **NIST SP 800-53 AU-3**: Audit Record Content — the TUI header carries
//!   hostname, OS, SELinux mode, and FIPS state on every rendered frame.
//! - **NIST SP 800-53 AC-3**: The browser is unconditionally read-only; no
//!   catalog mutation is possible through the interface.
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

use crossterm::event::{self, Event, KeyCode, KeyModifiers};

use umrs_labels::cui::catalog;
use umrs_labels::tui::app::{DetailContent, LabelRegistryApp, Panel};
use umrs_labels::tui::render::render_label_registry;
use umrs_ui::indicators::build_header_context;
use umrs_ui::keymap::KeyMap;
use umrs_ui::theme::Theme;
use umrs_ui::viewer::ViewerState;

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn"))
        .format_timestamp(None)
        .init();

    let args: Vec<String> = std::env::args().collect();

    let json_mode = args.contains(&"--json".to_owned());
    let cli_mode = args.contains(&"--cli".to_owned());

    // Parse catalog path overrides.
    let us_catalog_path = arg_value(&args, "--us-catalog")
        .unwrap_or_else(|| "config/us/US-CUI-LABELS.json".to_owned());
    let ca_catalog_path = arg_value(&args, "--ca-catalog")
        .unwrap_or_else(|| "config/ca/CANADIAN-PROTECTED.json".to_owned());

    // Load US catalog (required).
    let us_catalog = catalog::load_catalog(&us_catalog_path).unwrap_or_else(|e| {
        eprintln!("[FAIL] Could not load US catalog: {e}");
        std::process::exit(2);
    });

    // Load Canadian catalog (optional — log a warning but continue if absent).
    let ca_catalog = match catalog::load_catalog(&ca_catalog_path) {
        Ok(c) => Some(c),
        Err(e) => {
            log::warn!("Canadian catalog unavailable: {e}");
            None
        }
    };

    if json_mode {
        eprintln!("[INFO] --json output is not yet implemented for umrs-label");
        std::process::exit(0);
    }

    if cli_mode || !io::stdout().is_terminal() {
        run_cli(&us_catalog, ca_catalog.as_ref());
        return;
    }

    run_tui(us_catalog, ca_catalog);
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
        println!("  {} ({})", meta.catalog_name.en(), meta.country_code.as_deref().unwrap_or("??"));
        println!("  Version {}", meta.version);
    }
    println!("  {} markings loaded", cat.iter_markings().count());
    println!();

    let mut groups: BTreeMap<String, Vec<(&String, &catalog::Marking)>> = BTreeMap::new();
    for (key, marking) in cat.iter_markings() {
        let group_name =
            marking.index_group.clone().unwrap_or_else(|| "(No Group)".to_owned());
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
            let tag = if designation == "specified" { " [SP]" } else { "" };
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
    // Silence logger: env_logger writes to stderr which ratatui shares with
    // its alt-screen output — any log::warn! would corrupt the rendered frame.
    log::set_max_level(log::LevelFilter::Off);

    let app = LabelRegistryApp::new(us_catalog, ca_catalog);

    // Build the tree and load into viewer state.
    let tree = app.build_tree();
    let mut state = ViewerState::new(1); // single virtual "tab" (no tab bar shown)
    state.load_tree(tree);

    let keymap = KeyMap::default();
    let theme = Theme::default();

    // Snapshot the security posture header once at startup.
    let os_name = read_os_name();
    let ctx = build_header_context(
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        os_name,
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
#[expect(clippy::too_many_arguments, reason = "TUI event loop aggregates all mutable state; extracting would require a wrapper struct")]
fn run_event_loop(
    terminal: &mut ratatui::DefaultTerminal,
    app: &LabelRegistryApp,
    state: &mut ViewerState,
    ctx: &umrs_ui::app::HeaderContext,
    theme: &Theme,
    _keymap: &KeyMap,
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

        // Normal mode input.
        match key.code {
            KeyCode::Char('q' | 'Q') | KeyCode::Esc => break,

            KeyCode::Char('?') => {
                *help_active = true;
            }

            KeyCode::Tab | KeyCode::BackTab => {
                *active_panel = match *active_panel {
                    Panel::Tree => Panel::Detail,
                    Panel::Detail => Panel::Tree,
                };
                *detail_scroll = 0;
            }

            KeyCode::Up | KeyCode::Char('k') => {
                if *active_panel == Panel::Detail {
                    *detail_scroll = detail_scroll.saturating_sub(1);
                } else if state.selected_index > 0 {
                    state.selected_index -= 1;
                }
            }

            KeyCode::Down | KeyCode::Char('j') => {
                if *active_panel == Panel::Detail {
                    *detail_scroll = detail_scroll.saturating_add(1);
                } else {
                    let max = state.tree.display_count().saturating_sub(1);
                    if state.selected_index < max {
                        state.selected_index += 1;
                    }
                }
            }

            KeyCode::PageUp => {
                if *active_panel == Panel::Detail {
                    *detail_scroll = detail_scroll.saturating_sub(10);
                } else {
                    state.selected_index = state.selected_index.saturating_sub(10);
                }
            }

            KeyCode::PageDown => {
                if *active_panel == Panel::Detail {
                    *detail_scroll = detail_scroll.saturating_add(10);
                } else {
                    let max = state.tree.display_count().saturating_sub(1);
                    state.selected_index =
                        state.selected_index.saturating_add(10).min(max);
                }
            }

            KeyCode::Right | KeyCode::Char('l') => {
                expand_selected(state);
            }

            KeyCode::Left | KeyCode::Char('h') => {
                collapse_selected(state);
            }

            KeyCode::Char('/') => {
                state.search_active = true;
                state.search_query.clear();
            }

            KeyCode::Enter | KeyCode::Char(' ') => {
                handle_enter(app, state, detail_content, detail_scroll);
            }

            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => break,

            _ => {}
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
            let is_us = node
                .metadata
                .get("is_us")
                .map(|v| v == "1")
                .unwrap_or(true);
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
            let name = node.metadata.get("group_name").cloned().unwrap_or_else(|| {
                node.label.trim_start_matches("Group: ").to_owned()
            });
            let count_str = node.metadata.get("count").cloned().unwrap_or_default();
            let count = count_str.parse::<usize>().unwrap_or(0);
            *detail_content = DetailContent::Group { name, count };
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
fn render_help_overlay(
    frame: &mut ratatui::Frame,
    area: ratatui::layout::Rect,
    theme: &Theme,
) {
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
            ratatui::text::Span::styled("Move up", theme.data_value),
        ]),
        ratatui::text::Line::from(vec![
            ratatui::text::Span::styled("  ↓ / j        ", theme.header_field),
            ratatui::text::Span::styled("Move down", theme.data_value),
        ]),
        ratatui::text::Line::from(vec![
            ratatui::text::Span::styled("  ← / h        ", theme.header_field),
            ratatui::text::Span::styled("Collapse node", theme.data_value),
        ]),
        ratatui::text::Line::from(vec![
            ratatui::text::Span::styled("  → / l        ", theme.header_field),
            ratatui::text::Span::styled("Expand node", theme.data_value),
        ]),
        ratatui::text::Line::from(vec![
            ratatui::text::Span::styled("  Enter        ", theme.header_field),
            ratatui::text::Span::styled("Show details / toggle branch", theme.data_value),
        ]),
        ratatui::text::Line::from(vec![
            ratatui::text::Span::styled("  Tab          ", theme.header_field),
            ratatui::text::Span::styled("Switch focus (Tree ↔ Detail)", theme.data_value),
        ]),
        ratatui::text::Line::from(vec![
            ratatui::text::Span::styled("  /            ", theme.header_field),
            ratatui::text::Span::styled("Search / filter catalog", theme.data_value),
        ]),
        ratatui::text::Line::from(vec![
            ratatui::text::Span::styled("  Esc          ", theme.header_field),
            ratatui::text::Span::styled("Cancel search", theme.data_value),
        ]),
        ratatui::text::Line::from(vec![
            ratatui::text::Span::styled("  PgUp/PgDn    ", theme.header_field),
            ratatui::text::Span::styled("Scroll active panel", theme.data_value),
        ]),
        ratatui::text::Line::from(vec![
            ratatui::text::Span::styled("  ?            ", theme.header_field),
            ratatui::text::Span::styled("Toggle this help", theme.data_value),
        ]),
        ratatui::text::Line::from(vec![
            ratatui::text::Span::styled("  q / Esc      ", theme.header_field),
            ratatui::text::Span::styled("Quit", theme.data_value),
        ]),
        ratatui::text::Line::from(""),
        ratatui::text::Line::from(ratatui::text::Span::styled(
            "  Press ? or Esc to close",
            theme.data_key,
        )),
    ];

    frame.render_widget(ratatui::widgets::Paragraph::new(lines), inner);
}

// ---------------------------------------------------------------------------
// Utility helpers
// ---------------------------------------------------------------------------

/// Extract the value following `--flag=<value>` or `--flag <value>` from args.
fn arg_value(args: &[String], flag: &str) -> Option<String> {
    let prefix = format!("{flag}=");
    for (i, arg) in args.iter().enumerate() {
        if let Some(v) = arg.strip_prefix(&prefix) {
            return Some(v.to_owned());
        }
        if arg == flag {
            return args.get(i + 1).cloned();
        }
    }
    None
}

/// Read the OS name from `/etc/os-release`.
fn read_os_name() -> String {
    let Ok(content) = std::fs::read_to_string("/etc/os-release") else {
        return "unavailable".to_owned();
    };
    let mut name = None;
    let mut version_id = None;
    for line in content.lines() {
        if let Some(val) = line.strip_prefix("NAME=") {
            name = Some(val.trim_matches('"').to_owned());
        }
        if let Some(val) = line.strip_prefix("VERSION_ID=") {
            version_id = Some(val.trim_matches('"').to_owned());
        }
    }
    match (name, version_id) {
        (Some(n), Some(v)) => format!("{n} {v}"),
        (Some(n), None) => n,
        _ => "unavailable".to_owned(),
    }
}
