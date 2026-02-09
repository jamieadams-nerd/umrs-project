// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Jamie Adams
//



mod catalog;

fn main() {
    let path = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: cargo run -- <catalog.json>");
        std::process::exit(1);
    });

    let cat = catalog::load_catalog(&path).unwrap_or_else(|e| {
        eprintln!("[FAIL] {}", e);
        std::process::exit(2);
    });

    // Example: Label lookup
    println!("=========== marking checks ===================");
    if let Some(m) = cat.marking("CUI//LEI/JUV") {
        println!(
            "Found: {} ({}) parent={}",
            m.name, m.abbrv_name, m.parent_group
        );
    } else {
        println!("Not found: CUI//LEI/JUV");
    }

    // Example: iterate (first 5)
    for (k, m) in cat.iter_markings().take(5) {
        println!("{} -> {} ({})", k, m.name, m.abbrv_name);
    }

    println!("\nMarkings: Categories and subcategories");
    if let Some(_) = cat.marking("CUI//LEI") {
        for (key, child) in cat.marking_children("CUI//LEI") {
            println!("{} -> {}", key, child.name);
        }
    }

    if cat.marking("CUI//crap").is_none() {
        eprintln!("Invalid marking");
    }

    println!("\nCheck attributes and stuff....");
    // Check a single marking
    if let Some(mark) = cat.marking("CUI//LEI") {
        if mark.has_description() {
            println!("Description is populated:");
            println!("{}", mark.description);
        } else {
            println!("No description present.");
        }

        // Iterate children and test field presence
        for (key, child) in cat.marking_children("CUI//LEI") {
            if child.has_description() {
                println!(" - {key} has description.");
            }

            if child.has_handling() {
                println!(" - {key} has handling guidance.");
            }

            if child.has_handling_group() {
                println!(" - {key} has handling group id.");
            }

            if child.has_other() {
                println!(" - {key} has auxiliary metadata.");
            }

            // Show me missing descriptions.
            //
            for (key, mark) in cat.iter_markings() {
                if !mark.has_description() {
                    println!("Missing description → {key}");
                }
            }
        }
    }

    // Lookup example
    //println!("=========== Label checks ===================");
    //if let Some(label) = cat.label("CUI") {
        //println!("Label: {} level={}", label.name, label.level);
    //}

    // Iterate example
    //for (key, label) in cat.iter_labels() {
        //println!("{} -> {} ({})", key, label.name, label.level);
    //}
}
