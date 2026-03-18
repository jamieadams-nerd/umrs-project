// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Jamie Adams (a.k.a, Imodium Operator)
//
//! # MCS Bridge — CUI Catalog Lookup
//!
//! Bridges the MCS Translation service and the CUI catalog in `umrs_core::cui`.
//!
//! When a file is assigned `s0:c0` and this range is defined in `setrans.conf`,
//! the MCS daemon renders it as a human-readable label (e.g., `CUI`) in
//! directory listings and security context queries. This module allows callers
//! to look up the CUI catalog entry for a given textual marking, and vice versa.
//!
//! ## MCS Background
//!
//! MCS (Multi-Category Security) is a SELinux mechanism that compartmentalizes
//! data by assigning non-hierarchical category labels to files and processes.
//! For a subject to access an object, the subject's category set must subsume
//! the object's category set (the "need-to-know" principle).
//!
//! MCS shares the SELinux MLS kernel infrastructure but operates at a fixed
//! sensitivity level (`s0`), making it more flexible and user-friendly than
//! full MLS.
//!
//! The `mcstrans` service maps complex category values (e.g., `c0-c1023`) to
//! human-readable labels defined in `setrans.conf`.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AC-4**: Information Flow Enforcement — MCS category
//!   containment implements the non-hierarchical compartment access check.
//! - **NIST SP 800-53 AC-16**: Security Attributes — catalog lookups surface
//!   the regulatory marking for an MCS label in operator-visible output.
// ===========================================================================
//
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
//use std::env;

/// Parse setrans.conf and build marking → numeric mapping.
fn build_translation_map<P: AsRef<Path>>(
    path: P,
) -> std::io::Result<std::collections::HashMap<String, String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut map = std::collections::HashMap::new();

    for line in reader.lines() {
        let line = line?;

        // Only process translation lines
        if !line.starts_with("s0:c") {
            continue;
        }

        // Split on '='
        let parts: Vec<&str> = line.splitn(2, '=').collect();
        if parts.len() != 2 {
            continue;
        }

        let numeric = parts[0].trim();

        // Strip comment
        let rhs = parts[1];
        let marking = match rhs.find('#') {
            Some(pos) => &rhs[..pos],
            None => rhs,
        };

        let marking = marking.trim();

        map.insert(marking.to_string(), numeric.to_string());
    }

    Ok(map)
}

/// Lookup numeric label from marking string.
/// Provided a CUI marking string (e.g., "CUI//LEI"), return the s#:c#[,c#]
pub fn lookup_numeric_label(marking: &str) -> Option<String> {
    let path = "/etc/selinux/targeted/setrans.conf";

    match build_translation_map(path) {
        Ok(map) => map.get(marking).cloned(),
        Err(_) => None,
    }
}

//fn main() {
//// Expect marking string as first argument
//let args: Vec<String> = env::args().collect();
//
//if args.len() != 2 {
//eprintln!(
//"Usage: {} <MARKING>\nExample: {} CUI//GOVT/GOVTD",
//args[0], args[0]
//);
//std::process::exit(1);
//}
//
//let marking = &args[1];
//
//match lookup_numeric_label(marking) {
//Some(numeric) => {
//println!("{}", numeric);
//}
//None => {
//eprintln!("[FAIL] Marking not found: {}", marking);
//std::process::exit(2);
//}
//}
//}
