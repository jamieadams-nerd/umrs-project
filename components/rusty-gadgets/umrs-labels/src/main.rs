// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Jamie Adams
//
// NOTE: umask configuration is the caller's responsibility.
// This binary does not set umask; deploy with an appropriate service unit
// or shell profile that enforces umask 0o027 before launching.
//
use umrs_labels::cui::catalog;

fn main() {
    let path = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: umrs-labels <catalog.json>");
        std::process::exit(1);
    });

    let cat = catalog::load_catalog(&path).unwrap_or_else(|e| {
        eprintln!("[FAIL] {e}");
        std::process::exit(2);
    });
    umrs_core::verbose!("Loaded CUI catalog");

    println!("\nMarkings: Categories and subcategories");
    if cat.marking("CUI//LEI").is_some() {
        for (key, child) in cat.marking_children("CUI//LEI") {
            println!("{key} -> {}", child.name);
        }
    }
    println!("End.\n");
}
