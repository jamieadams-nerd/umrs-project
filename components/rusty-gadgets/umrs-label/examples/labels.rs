// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Jamie Adams
//
// Example: catalog inspection across US CUI and Canadian Protected catalogs.
//
// Usage:
//   cargo run -p umrs-labels --example labels -- <catalog.json>
//   cargo run -p umrs-labels --example labels -- config/us/US-CUI-LABELS.json
//   cargo run -p umrs-labels --example labels -- config/ca/CANADIAN-PROTECTED.json

use umrs_labels::cui::catalog;

fn main() {
    let path = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: cargo run --example labels -- <catalog.json>");
        std::process::exit(1);
    });

    let cat = catalog::load_catalog(&path).unwrap_or_else(|e| {
        eprintln!("[FAIL] {}", e);
        std::process::exit(2);
    });

    // ---------------------------------------------------------------------------
    // Metadata
    // ---------------------------------------------------------------------------
    println!("=========== Catalog Metadata ===================");
    if let Some(meta) = &cat.metadata {
        if !meta.catalog_name.is_empty() {
            println!("Name:      {}", meta.catalog_name);
        }
        println!("Version:   {}", meta.version);
        if !meta.authority.is_empty() {
            println!("Authority: {}", meta.authority);
        }
        if let Some(cc) = &meta.country_code {
            println!("Country:   {cc}");
        }
        if let Some(range) = &meta.mcs_category_range {
            println!("MCS range: {range}");
        }
    } else {
        println!("(no metadata block)");
    }

    // ---------------------------------------------------------------------------
    // Markings
    // ---------------------------------------------------------------------------
    println!("\n=========== Markings (first 5) ===================");
    for (k, m) in cat.iter_markings().take(5) {
        let level = m.level.as_deref().unwrap_or("(none)");
        println!("{} -> {} ({}) [level={}]", k, m.name, m.abbrv_name, level);
        // Show handling type.
        if m.handling_as_str().is_some() {
            println!("  handling: string");
        } else if m.handling_as_object().is_some() {
            println!("  handling: structured object");
        }
        // Show optional enrichment fields.
        if let Some(palette) = &m.palette_ref
            && !palette.is_empty()
        {
            println!("  palette_ref: {palette}");
        }
        if let Some(domains) = &m.risk_domains
            && !domains.is_empty()
        {
            println!("  risk_domains: {}", domains.join(", "));
        }
    }

    // ---------------------------------------------------------------------------
    // Index group summary (groups by display grouping hint)
    // ---------------------------------------------------------------------------
    println!("\n=========== Index groups ===================");
    let mut groups: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for (_, m) in cat.iter_markings() {
        let group = m.index_group.as_deref().unwrap_or("(ungrouped)");
        *groups.entry(group.to_string()).or_insert(0) += 1;
    }
    let mut sorted: Vec<_> = groups.iter().collect();
    sorted.sort_by_key(|(k, _)| k.as_str());
    for (group, count) in sorted {
        println!("  {group}: {count}");
    }

    // ---------------------------------------------------------------------------
    // Field presence audit
    // ---------------------------------------------------------------------------
    println!("\n=========== Field presence audit ===================");
    let total = cat.iter_markings().count();
    let with_description =
        cat.iter_markings().filter(|(_, m): &(_, &_)| m.has_description()).count();
    let with_handling = cat.iter_markings().filter(|(_, m): &(_, &_)| m.has_handling()).count();
    let with_level = cat.iter_markings().filter(|(_, m): &(_, &_)| m.level.is_some()).count();
    println!("Markings total:         {total}");
    println!("  with description:     {with_description}");
    println!("  with handling:        {with_handling}");
    println!("  with level set:       {with_level}");

    // ---------------------------------------------------------------------------
    // Missing descriptions
    // ---------------------------------------------------------------------------
    let missing: Vec<_> = cat
        .iter_markings()
        .filter(|(_, m)| !m.has_description())
        .map(|(k, _)| k.as_str())
        .collect();
    if !missing.is_empty() {
        println!("\n=========== Missing descriptions ===================");
        for key in missing {
            println!("  {key}");
        }
    }
}
