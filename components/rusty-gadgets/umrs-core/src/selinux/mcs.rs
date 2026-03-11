// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Jamie Adams (a.k.a, Imodium Operator)
//
/// Multi-Category Security (MCS) Rust Module
///
/// This module provides functions to bridge betwen the MCS Translation service
/// and the Controlled Unclassified Information (CUI) catalog contained in the
/// umrs_core::cui.
///
/// If a file (or directory is assigned), s0:c0 and we have this defined in the setrans.conf
/// file, the MCS daemon will show "CUI" in the directory listings or anytime you query
/// the file's security context.
///
/// This function allows you to lookup more information about the textual label from
/// the catalog and vice versa.
///
/// ===========================================================================
/// Red Hat Multi-Category Security (MCS) is an SELinux enhancement that enables advanced data
/// confidentiality by assigning security labels (categories) to files and processes. It works
/// alongside traditional permissions (DAC) and Type Enforcement (TE), ensuring that a process can
/// only access files if it holds all necessary, matching categories.
///
/// Key details about Red Hat MCS:
///
/// Access Control Principle: MCS requires that for a subject (process) to access an object (file),
/// the subject must be authorized for all categories assigned to the object.
///
/// Data Labeling: Users can assign categories (e.g., s0:c0,c1 or custom labels like
/// CompanyConfidential) to their files.
///
/// Purpose: It enforces the "need-to-know" principle, preventing unauthorized processes—even those
/// run by the same user—from accessing restricted files, such as preventing a web server from
/// reading personal user files.
///
/// Technical Basis: MCS is an adaptation of Multi-Level Security (MLS) and shares the same kernel
/// infrastructure but is designed to be more flexible and user-friendly.
///
/// **Translation Service**: The mcstrans service is often used to map complex category values (c0-c1023)
/// to human-readable labels.
///
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
