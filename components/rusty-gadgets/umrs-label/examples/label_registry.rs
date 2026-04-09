// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams
//
//! # label_registry — Security Label Registry TUI Example
//!
//! Demonstrates how to load both catalogs, build the `LabelRegistryApp`,
//! construct the tree model, and access marking details.
//!
//! Run in CLI mode (no TTY) to see text output:
//! ```text
//! cargo run -p umrs-label --example label_registry -- --cli
//! ```
//!
//! Or launch the TUI (if connected to a terminal):
//! ```text
//! cargo run -p umrs-label --example label_registry
//! ```

use umrs_labels::cui::catalog::load_catalog;
use umrs_labels::tui::app::LabelRegistryApp;

fn main() {
    let us_catalog = load_catalog("config/us/US-CUI-LABELS.json").unwrap_or_else(|e| {
        eprintln!("[FAIL] US catalog: {e}");
        std::process::exit(2);
    });
    let ca_catalog = load_catalog("config/ca/CANADIAN-PROTECTED.json").ok();

    let app = LabelRegistryApp::new(us_catalog, ca_catalog);

    println!("Security Label Registry — catalog summary");
    println!("==========================================");
    println!("Total entries : {}", app.total_markings());

    let us = app.us_catalog();
    println!("US markings   : {}", us.markings.len());
    println!("US LDCs       : {}", us.dissemination_controls.len());
    if let Some(ca) = app.ca_catalog() {
        println!("CA markings   : {}", ca.markings.len());
    }

    // Build the tree model and show root node counts.
    let tree = app.build_tree();
    println!();
    println!("Tree root nodes: {}", tree.roots.len());
    for (i, root) in tree.roots.iter().enumerate() {
        println!(
            "  [{}] {} — {} children",
            i,
            root.label,
            root.children.len()
        );
    }

    // Demonstrate detail construction for a known US marking.
    println!();
    println!("Sample detail — CUI//ADJ:");
    if let Some(detail) = app.marking_detail_us("CUI//ADJ") {
        println!("  Key          : {}", detail.key);
        println!("  Name (en)    : {}", detail.name_en);
        println!("  Name (fr)    : {}", detail.name_fr);
        println!("  Abbreviation : {}", detail.abbreviation);
        println!("  Designation  : {}", detail.designation);
        println!("  Index Group  : {}", detail.index_group);
    } else {
        println!("  (CUI//ADJ not found in US catalog)");
    }

    // Demonstrate dissemination control detail.
    println!();
    println!("Sample LDC — Attorney-Client:");
    if let Some(dc) = app.dissemination_detail("Attorney-Client") {
        println!("  Name (en)    : {}", dc.name_en);
        println!("  Name (fr)    : {}", dc.name_fr);
        println!("  Abbreviation : {}", dc.abbreviation);
        println!("  Designation  : {}", dc.designation);
    } else {
        println!("  (not found)");
    }
}
