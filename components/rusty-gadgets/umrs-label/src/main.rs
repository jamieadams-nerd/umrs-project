// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Jamie Adams
//
// NOTE: umask configuration is the caller's responsibility.
// This binary does not set umask; deploy with an appropriate service unit
// or shell profile that enforces umask 0o027 before launching.
//
use std::collections::BTreeMap;
use umrs_label::cui::catalog;

fn main() {
    let path = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: umrs-label <catalog.json>");
        std::process::exit(1);
    });

    let cat = catalog::load_catalog(&path).unwrap_or_else(|e| {
        eprintln!("[FAIL] {e}");
        std::process::exit(2);
    });

    let country = cat
        .country_code()
        .unwrap_or("??");

    println!();
    if let Some(meta) = &cat.metadata {
        println!("  {} ({})", meta.catalog_name, country);
        println!("  Version {}", meta.version);
    }
    println!("  {} markings loaded", cat.iter_markings().count());
    println!();

    // Group markings by index_group, sorted. Ungrouped entries go under "(No Group)".
    let mut groups: BTreeMap<String, Vec<(&String, &catalog::Marking)>> = BTreeMap::new();
    for (key, marking) in cat.iter_markings() {
        let group_name = marking
            .index_group
            .clone()
            .unwrap_or_else(|| "(No Group)".to_string());
        groups.entry(group_name).or_default().push((key, marking));
    }

    // Sort entries within each group alphabetically by key
    for entries in groups.values_mut() {
        entries.sort_by(|a, b| a.0.cmp(b.0));
    }

    for (group, entries) in &groups {
        println!("  {group}");
        println!("  {}", "-".repeat(group.len()));
        for (key, marking) in entries {
            let designation = marking
                .designation
                .as_deref()
                .unwrap_or("");
            let tag = match designation {
                "specified" => " [SP]",
                "basic" => "",
                _ => "",
            };
            println!("    {key}  {}{tag}", marking.name);
        }
        println!();
    }
}
