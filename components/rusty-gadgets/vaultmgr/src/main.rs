// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Jamie Adams
//
mod catalog;
mod fs;

use libc::umask;

use umrs_core::console::*;


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


    println!("\nMarkings: Categories and subcategories");
    if let Some(_) = cat.marking("CUI//LEI") {
        for (key, child) in cat.marking_children("CUI//LEI") {
            println!("{} -> {}", key, child.name);
        }
    }
    println!("End.\n");

    // Ignore results
    //let _ = fs::ensure_dir("./vaults-lei");
    
    // Match
    match fs::ensure_dir("./vaults-lei") {
        Ok(_) => {
            console_info!("Base directory ready.")
        }

        Err(_e) => {
            console_info!("DONE");
            std::process::exit(0);
        }
    }
    

}
