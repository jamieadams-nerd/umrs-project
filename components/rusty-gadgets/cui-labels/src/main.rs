// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Jamie Adams
//
use libc::umask;

use umrs_core::console::*;
use umrs_core::cui::catalog;

fn main() {
    unsafe {
        umask(0o027);
    }

    let path = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: cargo run -- <catalog.json>");
        std::process::exit(1);
    });

    let cat = catalog::load_catalog(&path).unwrap_or_else(|e| {
        eprintln!("[FAIL] {}", e);
        std::process::exit(2);
    });
    verbose!("Loaded CUI catalog");

    println!("\nMarkings: Categories and subcategories");
    if cat.marking("CUI//LEI").is_some() {
        for (key, child) in cat.marking_children("CUI//LEI") {
            println!("{} -> {}", key, child.name);
        }
    }
    println!("End.\n");
}
