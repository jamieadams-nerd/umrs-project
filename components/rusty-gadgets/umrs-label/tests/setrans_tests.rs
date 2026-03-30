// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams
//
// Integration tests for setrans.conf data integrity against the JSON catalogs.
//
// These tests verify that the MCS category-to-label translations in
// data/MLS-setrans.conf and data/TARGETED-setrans.conf are consistent,
// well-formed, and in sync with the US CUI and Canadian Protected JSON catalogs.
//
// Errors in setrans files cause wrong SELinux access decisions — this is the
// most critical data integrity check in the project.
//
// ## Compliance
//
// - NIST SP 800-53 AC-4: Information Flow Enforcement — MCS category
//   assignments are the mechanism that controls information flow between
//   security labels.
// - NIST SP 800-53 AC-16: Security Attributes — setrans translations are
//   the authoritative display mapping for MCS attributes.
// - NIST SP 800-53 AU-3: Audit Record Content — label names derived from
//   setrans appear in security audit output.

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use umrs_labels::cui::catalog;

// ---------------------------------------------------------------------------
// Path helpers
// ---------------------------------------------------------------------------

fn mls_setrans_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config/MLS-setrans.conf")
}

fn targeted_setrans_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("config/TARGETED-setrans.conf")
}

fn us_catalog_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("config/us/US-CUI-LABELS.json")
}

fn ca_catalog_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("config/ca/CANADIAN-PROTECTED.json")
}

// ---------------------------------------------------------------------------
// Setrans parser
//
// Parses a setrans.conf file into typed entries. Comment lines and blank
// lines are skipped. Each data entry is classified as System or Data:
//
//   System — entries without a colon in the MCS field (e.g., `s0=`,
//             `s1=Generic Unclass L1`) or entries with a range (`c0.c255`)
//   Data   — entries with `sN:cX` or `sN:cX,cY` format
// ---------------------------------------------------------------------------

/// A parsed line from a setrans.conf file.
#[derive(Debug, Clone)]
struct SetransEntry {
    /// The MCS string on the left-hand side of `=` (e.g., `s1:c90,c91`).
    mcs: String,
    /// The label string on the right-hand side of `=` (e.g., `CUI//LEI/AIV`).
    label: String,
    /// The inline comment text following `#`, stripped of leading whitespace.
    comment: String,
    /// 1-based source line number.
    lineno: usize,
}

impl SetransEntry {
    /// Returns `true` if this entry is a data entry (has a colon, no range dot).
    ///
    /// System entries (`s0=`, `s1=Generic Unclass L1`, `s0-s0:c0.c255=...`)
    /// are not data entries.
    fn is_data(&self) -> bool {
        self.mcs.contains(':') && !self.mcs.contains('.')
    }

    /// Returns the sensitivity level prefix (e.g., `"s1"` from `"s1:c90"`).
    ///
    /// Returns `None` for system entries that have no colon.
    fn sensitivity(&self) -> Option<&str> {
        let colon_pos = self.mcs.find(':')?;
        Some(&self.mcs[..colon_pos])
    }

    /// Returns the category part after the colon (e.g., `"c90"` or `"c90,c91"`).
    ///
    /// Returns `None` for system entries.
    fn category_part(&self) -> Option<&str> {
        let colon_pos = self.mcs.find(':')?;
        Some(&self.mcs[colon_pos + 1..])
    }

    /// Returns `true` if this is a compound entry (two category numbers).
    ///
    /// A compound entry has the form `sN:cX,cY` and represents a subcategory.
    fn is_compound(&self) -> bool {
        self.category_part().is_some_and(|c| c.contains(','))
    }

    /// Returns the group base MCS string for a compound entry.
    ///
    /// For `s1:c90,c91` returns `"s1:c90"`. Returns `None` for simple entries.
    fn group_base_mcs(&self) -> Option<String> {
        if !self.is_compound() {
            return None;
        }
        let sens = self.sensitivity()?;
        let cats = self.category_part()?;
        let group_cat = cats.split(',').next()?;
        Some(format!("{sens}:{group_cat}"))
    }
}

/// Parse all non-blank, non-comment lines from a setrans.conf file.
///
/// Returns `Err(String)` if the file cannot be read. Malformed lines (those
/// that do not contain `=`) are returned as errors rather than silently skipped,
/// so callers can distinguish between comment lines and malformed data.
fn parse_setrans(path: &PathBuf) -> Result<Vec<SetransEntry>, String> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("Failed to read {}: {e}", path.display()))?;

    let mut entries = Vec::new();

    for (idx, line) in content.lines().enumerate() {
        let lineno = idx + 1;
        let trimmed = line.trim();

        // Skip blank lines and comment-only lines
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        // Extract inline comment
        let (main, comment) = if let Some(hash_pos) = trimmed.find('#') {
            let main = trimmed[..hash_pos].trim();
            let comment = trimmed[hash_pos + 1..].trim();
            (main, comment)
        } else {
            (trimmed, "")
        };

        // Every non-blank, non-comment line must contain `=`
        let eq_pos = main.find('=').ok_or_else(|| {
            format!(
                "{}:{lineno}: malformed line (no `=`): {main:?}",
                path.display()
            )
        })?;

        let mcs = main[..eq_pos].trim().to_string();
        let label = main[eq_pos + 1..].trim().to_string();

        entries.push(SetransEntry {
            mcs,
            label,
            comment: comment.to_string(),
            lineno,
        });
    }

    Ok(entries)
}

/// Return only the data entries from a parsed setrans file.
fn data_entries(entries: &[SetransEntry]) -> Vec<&SetransEntry> {
    entries.iter().filter(|e| e.is_data()).collect()
}

// ---------------------------------------------------------------------------
// 1. Parsing validity
// ---------------------------------------------------------------------------

#[test]
fn mls_setrans_parses_without_error() {
    let result = parse_setrans(&mls_setrans_path());
    assert!(
        result.is_ok(),
        "MLS-setrans.conf failed to parse: {:?}",
        result.err()
    );
}

#[test]
fn targeted_setrans_parses_without_error() {
    let result = parse_setrans(&targeted_setrans_path());
    assert!(
        result.is_ok(),
        "TARGETED-setrans.conf failed to parse: {:?}",
        result.err()
    );
}

#[test]
fn mls_no_line_lacks_equals_sign() {
    // Every non-blank non-comment line must be an assignment. If parse_setrans
    // returns Ok, the invariant holds because parse_setrans fails on missing `=`.
    let entries = parse_setrans(&mls_setrans_path()).expect("MLS parse");
    assert!(
        !entries.is_empty(),
        "MLS-setrans.conf should contain at least one entry"
    );
}

#[test]
fn targeted_no_line_lacks_equals_sign() {
    let entries =
        parse_setrans(&targeted_setrans_path()).expect("TARGETED parse");
    assert!(
        !entries.is_empty(),
        "TARGETED-setrans.conf should contain at least one entry"
    );
}

#[test]
fn mls_no_trailing_whitespace_in_labels() {
    let entries = parse_setrans(&mls_setrans_path()).expect("MLS parse");
    for e in data_entries(&entries) {
        assert_eq!(
            e.label,
            e.label.trim(),
            "MLS line {}: label {:?} has trailing whitespace",
            e.lineno,
            e.label
        );
    }
}

#[test]
fn targeted_no_trailing_whitespace_in_labels() {
    let entries =
        parse_setrans(&targeted_setrans_path()).expect("TARGETED parse");
    for e in data_entries(&entries) {
        assert_eq!(
            e.label,
            e.label.trim(),
            "TARGETED line {}: label {:?} has trailing whitespace",
            e.lineno,
            e.label
        );
    }
}

#[test]
fn mls_no_trailing_whitespace_in_mcs_strings() {
    let entries = parse_setrans(&mls_setrans_path()).expect("MLS parse");
    for e in &entries {
        assert_eq!(
            e.mcs,
            e.mcs.trim(),
            "MLS line {}: MCS string {:?} has leading/trailing whitespace",
            e.lineno,
            e.mcs
        );
    }
}

#[test]
fn targeted_no_trailing_whitespace_in_mcs_strings() {
    let entries =
        parse_setrans(&targeted_setrans_path()).expect("TARGETED parse");
    for e in &entries {
        assert_eq!(
            e.mcs,
            e.mcs.trim(),
            "TARGETED line {}: MCS string {:?} has leading/trailing whitespace",
            e.lineno,
            e.mcs
        );
    }
}

// ---------------------------------------------------------------------------
// 2. No duplicates (CRITICAL — ambiguous entries cause wrong access decisions)
// ---------------------------------------------------------------------------

#[test]
fn mls_no_duplicate_mcs_strings() {
    let entries = parse_setrans(&mls_setrans_path()).expect("MLS parse");
    let data = data_entries(&entries);
    let mut seen: HashMap<&str, usize> = HashMap::new();
    for e in &data {
        if let Some(first_lineno) = seen.insert(e.mcs.as_str(), e.lineno) {
            panic!(
                "Duplicate MCS string {:?} in MLS-setrans.conf: first seen on line {first_lineno}, \
                 repeated on line {}",
                e.mcs, e.lineno
            );
        }
    }
}

#[test]
fn targeted_no_duplicate_mcs_strings() {
    let entries =
        parse_setrans(&targeted_setrans_path()).expect("TARGETED parse");
    let data = data_entries(&entries);
    let mut seen: HashMap<&str, usize> = HashMap::new();
    for e in &data {
        if let Some(first_lineno) = seen.insert(e.mcs.as_str(), e.lineno) {
            panic!(
                "Duplicate MCS string {:?} in TARGETED-setrans.conf: first seen on line {first_lineno}, \
                 repeated on line {}",
                e.mcs, e.lineno
            );
        }
    }
}

#[test]
fn mls_no_duplicate_label_strings() {
    let entries = parse_setrans(&mls_setrans_path()).expect("MLS parse");
    let data = data_entries(&entries);
    let mut seen: HashMap<&str, usize> = HashMap::new();
    for e in &data {
        if let Some(first_lineno) = seen.insert(e.label.as_str(), e.lineno) {
            panic!(
                "Duplicate label {:?} in MLS-setrans.conf: first seen on line {first_lineno}, \
                 repeated on line {} (MCS: {:?})",
                e.label, e.lineno, e.mcs
            );
        }
    }
}

#[test]
fn targeted_no_duplicate_label_strings() {
    let entries =
        parse_setrans(&targeted_setrans_path()).expect("TARGETED parse");
    let data = data_entries(&entries);
    let mut seen: HashMap<&str, usize> = HashMap::new();
    for e in &data {
        if let Some(first_lineno) = seen.insert(e.label.as_str(), e.lineno) {
            panic!(
                "Duplicate label {:?} in TARGETED-setrans.conf: first seen on line {first_lineno}, \
                 repeated on line {} (MCS: {:?})",
                e.label, e.lineno, e.mcs
            );
        }
    }
}

#[test]
fn mls_and_targeted_have_same_label_set() {
    let mls_entries = parse_setrans(&mls_setrans_path()).expect("MLS parse");
    let targeted_entries =
        parse_setrans(&targeted_setrans_path()).expect("TARGETED parse");

    let mls_labels: HashSet<&str> =
        data_entries(&mls_entries).iter().map(|e| e.label.as_str()).collect();
    let targeted_labels: HashSet<&str> = data_entries(&targeted_entries)
        .iter()
        .map(|e| e.label.as_str())
        .collect();

    let only_in_mls: Vec<&&str> =
        mls_labels.difference(&targeted_labels).collect();
    let only_in_targeted: Vec<&&str> =
        targeted_labels.difference(&mls_labels).collect();

    assert!(
        only_in_mls.is_empty(),
        "Labels in MLS but not in TARGETED: {only_in_mls:?}"
    );
    assert!(
        only_in_targeted.is_empty(),
        "Labels in TARGETED but not in MLS: {only_in_targeted:?}"
    );
}

// ---------------------------------------------------------------------------
// 3. JSON catalog sync (CRITICAL — access decisions rely on this mapping)
// ---------------------------------------------------------------------------

#[test]
fn mls_every_json_marking_has_setrans_entry() {
    let us_cat =
        catalog::load_catalog(us_catalog_path()).expect("US catalog load");
    let entries = parse_setrans(&mls_setrans_path()).expect("MLS parse");
    let setrans_labels: HashSet<&str> =
        data_entries(&entries).iter().map(|e| e.label.as_str()).collect();

    for (key, _marking) in us_cat.iter_markings() {
        assert!(
            setrans_labels.contains(key.as_str()),
            "US marking {key:?} from JSON catalog has no entry in MLS-setrans.conf"
        );
    }
}

#[test]
fn targeted_every_json_marking_has_setrans_entry() {
    let us_cat =
        catalog::load_catalog(us_catalog_path()).expect("US catalog load");
    let entries =
        parse_setrans(&targeted_setrans_path()).expect("TARGETED parse");
    let setrans_labels: HashSet<&str> =
        data_entries(&entries).iter().map(|e| e.label.as_str()).collect();

    for (key, _marking) in us_cat.iter_markings() {
        assert!(
            setrans_labels.contains(key.as_str()),
            "US marking {key:?} from JSON catalog has no entry in TARGETED-setrans.conf"
        );
    }
}

#[test]
fn mls_every_us_setrans_label_exists_in_json() {
    let us_cat =
        catalog::load_catalog(us_catalog_path()).expect("US catalog load");
    let entries = parse_setrans(&mls_setrans_path()).expect("MLS parse");

    let json_keys: HashSet<&str> =
        us_cat.iter_markings().map(|(k, _)| k.as_str()).collect();

    // Only check CUI-prefixed labels — Canadian entries are validated separately
    for e in data_entries(&entries) {
        if !e.label.starts_with("CUI") {
            continue;
        }
        assert!(
            json_keys.contains(e.label.as_str()),
            "MLS line {}: label {:?} (MCS {:?}) not found in US-CUI-LABELS.json",
            e.lineno,
            e.label,
            e.mcs
        );
    }
}

#[test]
fn targeted_every_us_setrans_label_exists_in_json() {
    let us_cat =
        catalog::load_catalog(us_catalog_path()).expect("US catalog load");
    let entries =
        parse_setrans(&targeted_setrans_path()).expect("TARGETED parse");

    let json_keys: HashSet<&str> =
        us_cat.iter_markings().map(|(k, _)| k.as_str()).collect();

    for e in data_entries(&entries) {
        if !e.label.starts_with("CUI") {
            continue;
        }
        assert!(
            json_keys.contains(e.label.as_str()),
            "TARGETED line {}: label {:?} (MCS {:?}) not found in US-CUI-LABELS.json",
            e.lineno,
            e.label,
            e.mcs
        );
    }
}

#[test]
fn mls_and_targeted_have_exactly_121_us_entries() {
    // US-CUI-LABELS.json has 121 markings. Both setrans files must cover all of them.
    let us_cat =
        catalog::load_catalog(us_catalog_path()).expect("US catalog load");
    let json_count = us_cat.iter_markings().count();
    assert_eq!(
        json_count, 121,
        "US catalog should have 121 markings, got {json_count}"
    );

    let mls_entries = parse_setrans(&mls_setrans_path()).expect("MLS parse");
    let mls_us_count = data_entries(&mls_entries)
        .iter()
        .filter(|e| e.label.starts_with("CUI"))
        .count();

    let targeted_entries =
        parse_setrans(&targeted_setrans_path()).expect("TARGETED parse");
    let targeted_us_count = data_entries(&targeted_entries)
        .iter()
        .filter(|e| e.label.starts_with("CUI"))
        .count();

    assert_eq!(
        mls_us_count, json_count,
        "MLS has {mls_us_count} CUI entries but JSON has {json_count}"
    );
    assert_eq!(
        targeted_us_count, json_count,
        "TARGETED has {targeted_us_count} CUI entries but JSON has {json_count}"
    );
}

#[test]
fn mls_canadian_entries_present() {
    let entries = parse_setrans(&mls_setrans_path()).expect("MLS parse");
    let labels: HashSet<&str> =
        data_entries(&entries).iter().map(|e| e.label.as_str()).collect();

    assert!(labels.contains("PROTÉGÉ A"), "MLS missing PROTÉGÉ A");
    assert!(labels.contains("PROTÉGÉ B"), "MLS missing PROTÉGÉ B");
    assert!(labels.contains("PROTÉGÉ C"), "MLS missing PROTÉGÉ C");
}

#[test]
fn targeted_canadian_entries_present() {
    let entries =
        parse_setrans(&targeted_setrans_path()).expect("TARGETED parse");
    let labels: HashSet<&str> =
        data_entries(&entries).iter().map(|e| e.label.as_str()).collect();

    assert!(labels.contains("PROTÉGÉ A"), "TARGETED missing PROTÉGÉ A");
    assert!(labels.contains("PROTÉGÉ B"), "TARGETED missing PROTÉGÉ B");
    assert!(labels.contains("PROTÉGÉ C"), "TARGETED missing PROTÉGÉ C");
}

// ---------------------------------------------------------------------------
// 4. Comment accuracy — inline comments should reflect JSON names
// ---------------------------------------------------------------------------

#[test]
fn mls_inline_comments_match_json_names() {
    let us_cat =
        catalog::load_catalog(us_catalog_path()).expect("US catalog load");
    let entries = parse_setrans(&mls_setrans_path()).expect("MLS parse");

    let mut mismatches: Vec<String> = Vec::new();

    for e in data_entries(&entries) {
        if !e.label.starts_with("CUI") || e.comment.is_empty() {
            continue;
        }
        let Some(marking) = us_cat.marking(&e.label) else {
            // Absence is already caught by other tests
            continue;
        };
        let json_name = &marking.name;
        // The comment should be a substring of the JSON name or the JSON name
        // should contain the comment. Strip leading whitespace from comment.
        let comment_trimmed = e.comment.trim_start_matches(' ');
        if !json_name.contains(comment_trimmed)
            && !comment_trimmed.contains(json_name.as_str())
        {
            mismatches.push(format!(
                "MLS line {}: label {:?} comment {:?} does not match JSON name {:?}",
                e.lineno, e.label, comment_trimmed, json_name
            ));
        }
    }

    assert!(
        mismatches.is_empty(),
        "Inline comment mismatches in MLS-setrans.conf:\n{}",
        mismatches.join("\n")
    );
}

#[test]
fn targeted_inline_comments_match_json_names() {
    let us_cat =
        catalog::load_catalog(us_catalog_path()).expect("US catalog load");
    let entries =
        parse_setrans(&targeted_setrans_path()).expect("TARGETED parse");

    let mut mismatches: Vec<String> = Vec::new();

    for e in data_entries(&entries) {
        if !e.label.starts_with("CUI") || e.comment.is_empty() {
            continue;
        }
        let Some(marking) = us_cat.marking(&e.label) else {
            continue;
        };
        let json_name = &marking.name;
        let comment_trimmed = e.comment.trim_start_matches(' ');
        if !json_name.contains(comment_trimmed)
            && !comment_trimmed.contains(json_name.as_str())
        {
            mismatches.push(format!(
                "TARGETED line {}: label {:?} comment {:?} does not match JSON name {:?}",
                e.lineno, e.label, comment_trimmed, json_name
            ));
        }
    }

    assert!(
        mismatches.is_empty(),
        "Inline comment mismatches in TARGETED-setrans.conf:\n{}",
        mismatches.join("\n")
    );
}

// ---------------------------------------------------------------------------
// 5. MCS structural integrity
// ---------------------------------------------------------------------------

#[test]
fn mls_all_us_entries_use_sensitivity_s1() {
    let entries = parse_setrans(&mls_setrans_path()).expect("MLS parse");
    for e in data_entries(&entries) {
        if !e.label.starts_with("CUI") {
            continue;
        }
        assert_eq!(
            e.sensitivity(),
            Some("s1"),
            "MLS line {}: US entry {:?} uses wrong sensitivity (MCS {:?}, expected s1:...)",
            e.lineno,
            e.label,
            e.mcs
        );
    }
}

#[test]
fn targeted_all_entries_use_sensitivity_s0() {
    let entries =
        parse_setrans(&targeted_setrans_path()).expect("TARGETED parse");
    for e in data_entries(&entries) {
        assert_eq!(
            e.sensitivity(),
            Some("s0"),
            "TARGETED line {}: entry {:?} uses wrong sensitivity (MCS {:?}, expected s0:...)",
            e.lineno,
            e.label,
            e.mcs
        );
    }
}

#[test]
fn mls_canadian_pa_uses_s1() {
    let entries = parse_setrans(&mls_setrans_path()).expect("MLS parse");
    let pa = data_entries(&entries)
        .into_iter()
        .find(|e| e.label == "PROTÉGÉ A")
        .expect("PROTÉGÉ A must exist in MLS");
    assert_eq!(
        pa.mcs, "s1:c200",
        "Canadian Protected A must be s1:c200 in MLS, got {:?}",
        pa.mcs
    );
}

#[test]
fn mls_canadian_pb_uses_s2() {
    let entries = parse_setrans(&mls_setrans_path()).expect("MLS parse");
    let pb = data_entries(&entries)
        .into_iter()
        .find(|e| e.label == "PROTÉGÉ B")
        .expect("PROTÉGÉ B must exist in MLS");
    assert_eq!(
        pb.mcs, "s2:c201",
        "Canadian Protected B must be s2:c201 in MLS, got {:?}",
        pb.mcs
    );
}

#[test]
fn mls_canadian_pc_uses_s3() {
    let entries = parse_setrans(&mls_setrans_path()).expect("MLS parse");
    let pc = data_entries(&entries)
        .into_iter()
        .find(|e| e.label == "PROTÉGÉ C")
        .expect("PROTÉGÉ C must exist in MLS");
    assert_eq!(
        pc.mcs, "s3:c202",
        "Canadian Protected C must be s3:c202 in MLS, got {:?}",
        pc.mcs
    );
}

#[test]
fn targeted_canadian_all_use_s0() {
    let entries =
        parse_setrans(&targeted_setrans_path()).expect("TARGETED parse");
    let data = data_entries(&entries);

    let pa = data
        .iter()
        .find(|e| e.label == "PROTÉGÉ A")
        .expect("PROTÉGÉ A must exist in TARGETED");
    let pb = data
        .iter()
        .find(|e| e.label == "PROTÉGÉ B")
        .expect("PROTÉGÉ B must exist in TARGETED");
    let pc = data
        .iter()
        .find(|e| e.label == "PROTÉGÉ C")
        .expect("PROTÉGÉ C must exist in TARGETED");

    assert_eq!(
        pa.mcs, "s0:c200",
        "PA targeted MCS should be s0:c200, got {:?}",
        pa.mcs
    );
    assert_eq!(
        pb.mcs, "s0:c201",
        "PB targeted MCS should be s0:c201, got {:?}",
        pb.mcs
    );
    assert_eq!(
        pc.mcs, "s0:c202",
        "PC targeted MCS should be s0:c202, got {:?}",
        pc.mcs
    );
}

#[test]
fn mls_subcategory_entries_are_compound_format() {
    // Subcategory entries (CUI//GROUP/SUBCATEGORY) must use the compound
    // sN:cX,cY format — a single category would be ambiguous with a group header.
    //
    // Label structure:
    //   CUI            — 0 slashes  — base umbrella (simple MCS: sN:c0)
    //   CUI//GROUP     — 2 slashes  — group header  (simple MCS: sN:cX)
    //   CUI//GROUP/SUB — 3 slashes  — subcategory   (compound MCS: sN:cX,cY)
    let entries = parse_setrans(&mls_setrans_path()).expect("MLS parse");
    for e in data_entries(&entries) {
        if !e.label.starts_with("CUI") {
            continue;
        }
        let total_slashes = e.label.chars().filter(|&c| c == '/').count();
        let is_subcategory = total_slashes >= 3;
        if is_subcategory {
            assert!(
                e.is_compound(),
                "MLS line {}: subcategory label {:?} should use compound MCS format (cX,cY), got {:?}",
                e.lineno,
                e.label,
                e.mcs
            );
        }
    }
}

#[test]
fn targeted_subcategory_entries_are_compound_format() {
    let entries =
        parse_setrans(&targeted_setrans_path()).expect("TARGETED parse");
    for e in data_entries(&entries) {
        if !e.label.starts_with("CUI") {
            continue;
        }
        let total_slashes = e.label.chars().filter(|&c| c == '/').count();
        let is_subcategory = total_slashes >= 3;
        if is_subcategory {
            assert!(
                e.is_compound(),
                "TARGETED line {}: subcategory label {:?} should use compound MCS format (cX,cY), got {:?}",
                e.lineno,
                e.label,
                e.mcs
            );
        }
    }
}

#[test]
fn mls_group_header_entries_are_simple_format() {
    // Group header entries (CUI//GROUP with exactly 2 slashes) must use the
    // simple sN:cX format — they are the group anchor, not a subcategory.
    //
    // CUI//EXPT is the only known exception to this rule: Export Control has
    // no unqualified group header because there is no "base EXPT" category —
    // all EXPT entries require at least one qualifier. The exception is
    // documented and verified by mls_expt_known_exception_documented.
    let entries = parse_setrans(&mls_setrans_path()).expect("MLS parse");
    for e in data_entries(&entries) {
        if !e.label.starts_with("CUI") {
            continue;
        }
        // CUI//EXPT is the known exception — skip it here
        if e.label == "CUI//EXPT" || e.label == "CUI//EXPT/EXPTR" {
            continue;
        }
        let total_slashes = e.label.chars().filter(|&c| c == '/').count();
        let is_group_header = total_slashes == 2;
        if is_group_header {
            assert!(
                !e.is_compound(),
                "MLS line {}: group header {:?} should use simple MCS format (cX only), got {:?}",
                e.lineno,
                e.label,
                e.mcs
            );
        }
    }
}

#[test]
fn targeted_group_header_entries_are_simple_format() {
    // Same rule as MLS — CUI//EXPT is the only known exception.
    let entries =
        parse_setrans(&targeted_setrans_path()).expect("TARGETED parse");
    for e in data_entries(&entries) {
        if !e.label.starts_with("CUI") {
            continue;
        }
        if e.label == "CUI//EXPT" || e.label == "CUI//EXPT/EXPTR" {
            continue;
        }
        let total_slashes = e.label.chars().filter(|&c| c == '/').count();
        let is_group_header = total_slashes == 2;
        if is_group_header {
            assert!(
                !e.is_compound(),
                "TARGETED line {}: group header {:?} should use simple MCS format (cX only), got {:?}",
                e.lineno,
                e.label,
                e.mcs
            );
        }
    }
}

// ---------------------------------------------------------------------------
// 6. MLS vs TARGETED consistency
// ---------------------------------------------------------------------------

#[test]
fn mls_and_targeted_have_same_category_numbers() {
    // Category numbers must be identical between MLS and TARGETED.
    // Only the sensitivity prefix differs (s1 vs s0 for US; s1/s2/s3 vs s0 for CA).
    let mls_entries = parse_setrans(&mls_setrans_path()).expect("MLS parse");
    let targeted_entries =
        parse_setrans(&targeted_setrans_path()).expect("TARGETED parse");

    let mls_cats: HashSet<String> = data_entries(&mls_entries)
        .iter()
        .filter_map(|e| e.category_part().map(str::to_string))
        .collect();

    let targeted_cats: HashSet<String> = data_entries(&targeted_entries)
        .iter()
        .filter_map(|e| e.category_part().map(str::to_string))
        .collect();

    let only_in_mls: Vec<&String> =
        mls_cats.difference(&targeted_cats).collect();
    let only_in_targeted: Vec<&String> =
        targeted_cats.difference(&mls_cats).collect();

    assert!(
        only_in_mls.is_empty(),
        "Category parts in MLS but not in TARGETED: {only_in_mls:?}"
    );
    assert!(
        only_in_targeted.is_empty(),
        "Category parts in TARGETED but not in MLS: {only_in_targeted:?}"
    );
}

#[test]
fn mls_and_targeted_entry_counts_are_equal() {
    let mls_entries = parse_setrans(&mls_setrans_path()).expect("MLS parse");
    let targeted_entries =
        parse_setrans(&targeted_setrans_path()).expect("TARGETED parse");
    let mls_count = data_entries(&mls_entries).len();
    let targeted_count = data_entries(&targeted_entries).len();
    assert_eq!(
        mls_count, targeted_count,
        "MLS has {mls_count} data entries but TARGETED has {targeted_count}"
    );
}

#[test]
fn mls_and_targeted_same_canadian_category_assignments() {
    // Canadian category numbers must be identical in both files.
    let mls_entries = parse_setrans(&mls_setrans_path()).expect("MLS parse");
    let targeted_entries =
        parse_setrans(&targeted_setrans_path()).expect("TARGETED parse");

    let canadian_labels = ["PROTÉGÉ A", "PROTÉGÉ B", "PROTÉGÉ C"];

    for label in &canadian_labels {
        let mls_entry = data_entries(&mls_entries)
            .into_iter()
            .find(|e| e.label == *label)
            .unwrap_or_else(|| panic!("MLS missing Canadian entry {label:?}"));

        let targeted_entry = data_entries(&targeted_entries)
            .into_iter()
            .find(|e| e.label == *label)
            .unwrap_or_else(|| {
                panic!("TARGETED missing Canadian entry {label:?}")
            });

        let mls_cat = mls_entry
            .category_part()
            .expect("MLS Canadian entry should have category");
        let targeted_cat = targeted_entry
            .category_part()
            .expect("TARGETED Canadian entry should have category");

        assert_eq!(
            mls_cat, targeted_cat,
            "Canadian entry {label:?}: MLS category {mls_cat:?} != TARGETED category {targeted_cat:?}"
        );
    }
}

// ---------------------------------------------------------------------------
// 7. Hierarchy integrity
// ---------------------------------------------------------------------------

#[test]
fn mls_every_compound_entry_has_group_header() {
    // For every compound entry sN:cX,cY, there must be a group header sN:cX.
    //
    // Note: The EXPT group (c30) is a known exception — CUI//EXPT uses compound
    // entries (c30,c31 and c30,c32) without a standalone c30 header. This is by
    // design: EXPT has no uncontrolled base category. The test documents these
    // known exceptions and verifies no NEW exceptions appear.
    let entries = parse_setrans(&mls_setrans_path()).expect("MLS parse");
    let data = data_entries(&entries);
    let mcs_set: HashSet<&str> = data.iter().map(|e| e.mcs.as_str()).collect();

    // Known group headers that are intentionally absent: EXPT uses c30 only as
    // the base of compound entries; there is no standalone "CUI//EXPT" at s1:c30.
    // This is per the CUI Registry design — EXPT has no unqualified base label.
    let known_absent_group_headers: HashSet<&str> =
        ["s1:c30"].iter().copied().collect();

    let mut unexpected_missing: Vec<String> = Vec::new();

    for e in &data {
        if !e.is_compound() {
            continue;
        }
        let group_mcs = e
            .group_base_mcs()
            .expect("compound entry must produce a group base MCS");

        if !mcs_set.contains(group_mcs.as_str())
            && !known_absent_group_headers.contains(group_mcs.as_str())
        {
            unexpected_missing.push(format!(
                "MLS line {}: compound entry {:?} (label {:?}) missing group header {:?}",
                e.lineno, e.mcs, e.label, group_mcs
            ));
        }
    }

    assert!(
        unexpected_missing.is_empty(),
        "Unexpected missing group headers in MLS-setrans.conf:\n{}",
        unexpected_missing.join("\n")
    );
}

#[test]
fn targeted_every_compound_entry_has_group_header() {
    // Same as the MLS test — EXPT (c30) is the only known intentional exception.
    let entries =
        parse_setrans(&targeted_setrans_path()).expect("TARGETED parse");
    let data = data_entries(&entries);
    let mcs_set: HashSet<&str> = data.iter().map(|e| e.mcs.as_str()).collect();

    let known_absent_group_headers: HashSet<&str> =
        ["s0:c30"].iter().copied().collect();

    let mut unexpected_missing: Vec<String> = Vec::new();

    for e in &data {
        if !e.is_compound() {
            continue;
        }
        let group_mcs = e
            .group_base_mcs()
            .expect("compound entry must produce a group base MCS");

        if !mcs_set.contains(group_mcs.as_str())
            && !known_absent_group_headers.contains(group_mcs.as_str())
        {
            unexpected_missing.push(format!(
                "TARGETED line {}: compound entry {:?} (label {:?}) missing group header {:?}",
                e.lineno, e.mcs, e.label, group_mcs
            ));
        }
    }

    assert!(
        unexpected_missing.is_empty(),
        "Unexpected missing group headers in TARGETED-setrans.conf:\n{}",
        unexpected_missing.join("\n")
    );
}

#[test]
fn mls_compound_label_starts_with_group_label() {
    // For every compound entry sN:cX,cY with label L, the group header sN:cX
    // must have a label G such that L starts with G + "/". This enforces the
    // hierarchical label naming convention (e.g., CUI//LEI/AIV starts with CUI//LEI/).
    let entries = parse_setrans(&mls_setrans_path()).expect("MLS parse");
    let data = data_entries(&entries);
    let label_map: HashMap<&str, &str> =
        data.iter().map(|e| (e.mcs.as_str(), e.label.as_str())).collect();

    // EXPT compound entries have no group header — skip them
    let known_absent_group_headers: HashSet<&str> =
        ["s1:c30"].iter().copied().collect();

    let mut mismatches: Vec<String> = Vec::new();

    for e in &data {
        if !e.is_compound() {
            continue;
        }
        let group_mcs = e
            .group_base_mcs()
            .expect("compound entry must produce a group base MCS");

        if known_absent_group_headers.contains(group_mcs.as_str()) {
            continue;
        }

        let Some(group_label) = label_map.get(group_mcs.as_str()) else {
            // Missing header is caught by mls_every_compound_entry_has_group_header
            continue;
        };

        let expected_prefix = format!("{group_label}/");
        if !e.label.starts_with(expected_prefix.as_str()) {
            mismatches.push(format!(
                "MLS line {}: label {:?} does not start with group label prefix {:?}",
                e.lineno, e.label, expected_prefix
            ));
        }
    }

    assert!(
        mismatches.is_empty(),
        "Hierarchy prefix violations in MLS-setrans.conf:\n{}",
        mismatches.join("\n")
    );
}

#[test]
fn targeted_compound_label_starts_with_group_label() {
    let entries =
        parse_setrans(&targeted_setrans_path()).expect("TARGETED parse");
    let data = data_entries(&entries);
    let label_map: HashMap<&str, &str> =
        data.iter().map(|e| (e.mcs.as_str(), e.label.as_str())).collect();

    let known_absent_group_headers: HashSet<&str> =
        ["s0:c30"].iter().copied().collect();

    let mut mismatches: Vec<String> = Vec::new();

    for e in &data {
        if !e.is_compound() {
            continue;
        }
        let group_mcs = e
            .group_base_mcs()
            .expect("compound entry must produce a group base MCS");

        if known_absent_group_headers.contains(group_mcs.as_str()) {
            continue;
        }

        let Some(group_label) = label_map.get(group_mcs.as_str()) else {
            continue;
        };

        let expected_prefix = format!("{group_label}/");
        if !e.label.starts_with(expected_prefix.as_str()) {
            mismatches.push(format!(
                "TARGETED line {}: label {:?} does not start with group label prefix {:?}",
                e.lineno, e.label, expected_prefix
            ));
        }
    }

    assert!(
        mismatches.is_empty(),
        "Hierarchy prefix violations in TARGETED-setrans.conf:\n{}",
        mismatches.join("\n")
    );
}

#[test]
fn mls_expt_known_exception_documented() {
    // The EXPT group (Export Control) uses compound entries with no standalone
    // group header at s1:c30. This test confirms the known exception is still
    // exactly as expected — two entries at s1:c30,c31 and s1:c30,c32.
    let entries = parse_setrans(&mls_setrans_path()).expect("MLS parse");
    let data = data_entries(&entries);

    let expt_entries: Vec<&SetransEntry> =
        data.iter().filter(|e| e.mcs.contains("c30")).copied().collect();

    assert_eq!(
        expt_entries.len(),
        2,
        "EXPT group should have exactly 2 entries (c30,c31 and c30,c32), found: {:?}",
        expt_entries.iter().map(|e| &e.mcs).collect::<Vec<_>>()
    );

    let mcs_set: HashSet<&str> =
        expt_entries.iter().map(|e| e.mcs.as_str()).collect();
    assert!(
        mcs_set.contains("s1:c30,c31"),
        "Expected s1:c30,c31 for CUI//EXPT"
    );
    assert!(
        mcs_set.contains("s1:c30,c32"),
        "Expected s1:c30,c32 for CUI//EXPT/EXPTR"
    );
}

#[test]
fn targeted_expt_known_exception_documented() {
    let entries =
        parse_setrans(&targeted_setrans_path()).expect("TARGETED parse");
    let data = data_entries(&entries);

    let expt_entries: Vec<&SetransEntry> =
        data.iter().filter(|e| e.mcs.contains("c30")).copied().collect();

    assert_eq!(
        expt_entries.len(),
        2,
        "EXPT group should have exactly 2 entries in TARGETED (c30,c31 and c30,c32), found: {:?}",
        expt_entries.iter().map(|e| &e.mcs).collect::<Vec<_>>()
    );

    let mcs_set: HashSet<&str> =
        expt_entries.iter().map(|e| e.mcs.as_str()).collect();
    assert!(
        mcs_set.contains("s0:c30,c31"),
        "Expected s0:c30,c31 for CUI//EXPT in TARGETED"
    );
    assert!(
        mcs_set.contains("s0:c30,c32"),
        "Expected s0:c30,c32 for CUI//EXPT/EXPTR in TARGETED"
    );
}

// ---------------------------------------------------------------------------
// 8. CA catalog cross-reference (markings key in JSON)
//
// The Canadian catalog uses "markings" as its JSON key (unified schema).
// These tests validate the setrans data against the CA catalog markings.
// ---------------------------------------------------------------------------

#[test]
fn ca_catalog_markings_contain_three_entries() {
    // Verify the CA JSON data has the three Protected tiers available for
    // cross-referencing.
    let ca_cat =
        catalog::load_catalog(ca_catalog_path()).expect("CA catalog load");
    let count = ca_cat.markings.len();
    assert_eq!(
        count, 3,
        "CA catalog should have 3 markings (PA, PB, PC), got {count}"
    );
}

#[test]
fn ca_catalog_marking_keys_match_expected_names() {
    let ca_cat =
        catalog::load_catalog(ca_catalog_path()).expect("CA catalog load");
    let keys: HashSet<&str> =
        ca_cat.markings.keys().map(String::as_str).collect();
    assert!(
        keys.contains("PROTECTED-A"),
        "CA catalog missing PROTECTED-A"
    );
    assert!(
        keys.contains("PROTECTED-B"),
        "CA catalog missing PROTECTED-B"
    );
    assert!(
        keys.contains("PROTECTED-C"),
        "CA catalog missing PROTECTED-C"
    );
}

#[test]
fn ca_catalog_levels_match_setrans_mls() {
    // Verify that the sensitivity levels in the CA JSON match the MLS setrans assignments.
    // PA = s1, PB = s2, PC = s3.
    let ca_cat =
        catalog::load_catalog(ca_catalog_path()).expect("CA catalog load");
    let entries = parse_setrans(&mls_setrans_path()).expect("MLS parse");

    let mls_map: HashMap<&str, &str> = data_entries(&entries)
        .iter()
        .map(|e| (e.label.as_str(), e.mcs.as_str()))
        .collect();

    let expected = [
        ("PROTECTED-A", "PROTÉGÉ A", "s1"),
        ("PROTECTED-B", "PROTÉGÉ B", "s2"),
        ("PROTECTED-C", "PROTÉGÉ C", "s3"),
    ];

    for (ca_key, setrans_label, expected_sens) in &expected {
        let label = ca_cat.markings.get(*ca_key).unwrap_or_else(|| {
            panic!("{ca_key} must exist in CA catalog markings")
        });

        let json_level = label
            .level
            .as_deref()
            .unwrap_or_else(|| panic!("{ca_key} must have a level field"));

        assert_eq!(
            json_level, *expected_sens,
            "{ca_key}: JSON level {json_level:?} != expected {expected_sens:?}"
        );

        let mcs = mls_map
            .get(setrans_label)
            .unwrap_or_else(|| panic!("MLS missing {setrans_label:?}"));

        let sens =
            mcs.split(':').next().expect("MCS should have sensitivity prefix");
        assert_eq!(
            sens, *expected_sens,
            "{setrans_label}: MLS sensitivity {sens:?} != expected {expected_sens:?}"
        );
    }
}
