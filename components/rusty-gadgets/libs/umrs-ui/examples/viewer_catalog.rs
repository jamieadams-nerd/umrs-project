// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams

//! # viewer_catalog — ViewerApp Usage Example
//!
//! Demonstrates the `ViewerApp` / `ViewerState` pattern by building a small
//! in-memory catalog browser. The example does not connect to a real terminal
//! (to allow running in CI without a TTY) — instead it drives the state
//! machine directly and prints the resulting tree structure.
//!
//! ## What this shows
//!
//! - Implementing `ViewerApp` on a data struct
//! - Populating `ViewerState` with a `TreeModel`
//! - Using `handle_action` to expand nodes and navigate
//! - Reading breadcrumb and display list state after navigation
//!
//! ## Running
//!
//! ```bash
//! cargo run -p umrs-ui --example viewer_catalog
//! ```

use std::collections::HashMap;

use umrs_ui::Action;
use umrs_ui::app::{StatusLevel, StatusMessage, TabDef};
use umrs_ui::viewer::tree::{TreeModel, TreeNode};
use umrs_ui::viewer::{ViewerApp, ViewerHeaderContext, ViewerState};

// ---------------------------------------------------------------------------
// Data struct
// ---------------------------------------------------------------------------

/// A minimal catalog data struct implementing `ViewerApp`.
struct CatalogApp {
    title: String,
    tabs: Vec<TabDef>,
}

impl CatalogApp {
    fn new() -> Self {
        Self {
            title: "UMRS Label Catalog Viewer".to_owned(),
            tabs: vec![TabDef::new("Catalog"), TabDef::new("Help")],
        }
    }

    /// Build the example tree representing a small label catalog.
    fn build_tree() -> TreeModel {
        let mut model = TreeModel::new();

        // Category: Controlled Technical Information
        let mut cti = TreeNode::branch("Controlled Technical Information", "CTI");
        cti.metadata.insert("Code".to_owned(), "CTI".to_owned());
        cti.metadata.insert("Authority".to_owned(), "DFARS 252.204-7012".to_owned());

        let mut naval = TreeNode::branch("Naval Systems", "CTI-NS");
        naval.children.push({
            let mut n = TreeNode::leaf("Propulsion", "CTI-NS-P");
            n.metadata.insert("Description".to_owned(), "Naval propulsion data".to_owned());
            n
        });
        naval.children.push(TreeNode::leaf("Navigation", "CTI-NS-N"));
        cti.children.push(naval);
        cti.children.push(TreeNode::leaf("Aerospace", "CTI-AS"));

        // Category: Export Controlled
        let mut ec = TreeNode::branch("Export Controlled", "EC");
        ec.metadata.insert("Code".to_owned(), "EC".to_owned());
        ec.metadata.insert("Authority".to_owned(), "EAR / ITAR".to_owned());
        ec.children.push(TreeNode::leaf("EAR99 Technology", "EC-EAR"));
        ec.children.push(TreeNode::leaf("ITAR Defense Articles", "EC-ITAR"));

        model.roots.push(cti);
        model.roots.push(ec);
        model
    }
}

impl ViewerApp for CatalogApp {
    fn card_title(&self) -> &str {
        &self.title
    }

    fn tabs(&self) -> &[TabDef] {
        &self.tabs
    }

    fn status(&self) -> StatusMessage {
        StatusMessage {
            level: StatusLevel::Ok,
            text: "Catalog loaded".to_owned(),
        }
    }

    fn viewer_header(&self) -> ViewerHeaderContext {
        ViewerHeaderContext::new("umrs-labels", "US CUI Catalog v0.1.0", 2)
            .with_summary("2 top-level categories, 5 subcategories")
    }

    fn initial_tree(&self) -> Option<TreeModel> {
        Some(Self::build_tree())
    }
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

fn main() {
    let app = CatalogApp::new();
    let mut state = ViewerState::new(app.tabs().len());

    // Load the initial tree (provided by ViewerApp::initial_tree).
    if let Some(tree) = app.initial_tree() {
        state.load_tree(tree);
    }

    println!("=== UMRS Label Catalog Viewer — ViewerApp Example ===\n");

    // Initial state: both root nodes are collapsed.
    println!(
        "Initial display list ({} entries):",
        state.tree.display_count()
    );
    for entry in &state.tree.display_list {
        println!("  [depth {}] {} {}", entry.depth, entry.prefix, entry.label);
    }
    println!("Breadcrumb: {}\n", state.breadcrumb_display());

    // Expand the first root node (Controlled Technical Information).
    println!("--- Expanding root node 0 (CTI) ---");
    let _ = state.handle_action(Action::Expand);
    println!("Display list ({} entries):", state.tree.display_count());
    for entry in &state.tree.display_list {
        println!(
            "  [depth {}] {}{}  {}",
            entry.depth, entry.prefix, entry.label, entry.detail
        );
    }
    println!();

    // Navigate down to the Naval Systems child.
    println!("--- ScrollDown to index 1 (Naval Systems) ---");
    let _ = state.handle_action(Action::ScrollDown);
    println!("Breadcrumb: {}", state.breadcrumb_display());
    println!("Selected: {}", {
        state.tree.display_list.get(state.selected_index).map_or("(none)", |e| e.label.as_str())
    });
    println!();

    // Expand Naval Systems.
    println!("--- Expanding Naval Systems ---");
    let _ = state.handle_action(Action::Expand);
    println!("Display list ({} entries):", state.tree.display_count());
    for entry in &state.tree.display_list {
        println!("  [depth {}] {}{}", entry.depth, entry.prefix, entry.label);
    }
    println!();

    // Show detail for the selected node (Naval Systems).
    println!("--- Detail panel for selected node ---");
    if let Some(entry) = state.tree.display_list.get(state.selected_index) {
        if let Some(node) = state.tree.node_ref(&entry.path) {
            println!("Label:  {}", node.label);
            println!("Detail: {}", node.detail);
            for (k, v) in &node.metadata {
                println!("  {k:<20} : {v}");
            }
        }
    }
    println!();

    // Demonstrate search/filter.
    println!("--- Search: 'propulsion' ---");
    state.search_active = true;
    state.push_search_char('p');
    state.push_search_char('r');
    state.push_search_char('o');
    state.push_search_char('p');
    state.push_search_char('u');
    state.push_search_char('l');
    state.push_search_char('s');
    state.push_search_char('i');
    state.push_search_char('o');
    state.push_search_char('n');
    println!(
        "Query: '{}' — {} visible entries",
        state.search_query,
        state.tree.display_count()
    );
    for entry in &state.tree.display_list {
        println!("  {}", entry.label);
    }
    println!();

    // Navigate back to the root.
    println!("--- Navigate Back ---");
    let _ = state.handle_action(Action::Back);
    println!("Breadcrumb after Back: {}", state.breadcrumb_display());
    println!();

    // Header context.
    let ctx = app.viewer_header();
    println!("=== Header Context ===");
    println!("Tool:    {}", ctx.tool_name);
    println!("Source:  {}", ctx.data_source);
    println!("Records: {}", ctx.record_count);
    if let Some(desc) = &ctx.summary_description {
        println!("Summary: {desc}");
    }

    // Demonstrate initial_tree pattern with HashMap committed_values (not part of
    // ViewerApp, but shows the full pattern context).
    let _committed: HashMap<String, String> = HashMap::new();

    println!("\n=== Example complete ===");
}
