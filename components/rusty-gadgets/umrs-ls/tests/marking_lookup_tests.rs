// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
// Crate: umrs-ls
// Module: tests/marking_lookup_tests

//! Regression tests for the two-strategy marking lookup path.
//!
//! These tests cover the Enter-on-group-header dispatch for both US CUI and
//! Canadian Protected group headers.  The bug fixed here: Canadian group
//! headers carried raw MCS level strings (e.g., `"s1:c300"`) because
//! `setrans.conf` has no entries for c300–c302.  The direct key lookup
//! against `"PROTECTED-A"` / `"PROTECTED-B"` / `"PROTECTED-C"` always
//! failed, so pressing Enter on a Canadian group header produced no popup.
//!
//! The fix adds `Catalog::marking_by_mcs_level()` as a fallback for catalog
//! entries that lack `setrans.conf` translations.  These tests verify the
//! complete lookup path mirrors what `lookup_marking_detail()` in `main.rs`
//! performs, ensuring both US CUI and Canadian headers produce popup data.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AC-16**: Security Attributes — group header popups
//!   display the full regulatory label definition for the marking shown.

use std::path::PathBuf;

use umrs_labels::cui::catalog;

// ---------------------------------------------------------------------------
// Path helpers
// ---------------------------------------------------------------------------

fn us_catalog_path() -> PathBuf {
    // Tests run with the crate root as the working directory; resolve from
    // the umrs-label crate directory via CARGO_MANIFEST_DIR of umrs-ls.
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../umrs-label/config/us/US-CUI-LABELS.json")
}

fn ca_catalog_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../umrs-label/config/ca/CANADIAN-PROTECTED.json")
}

// ---------------------------------------------------------------------------
// Simulate the two-strategy lookup in lookup_marking_detail()
// ---------------------------------------------------------------------------

/// Replicate the logic of `lookup_marking_detail` for test assertions.
///
/// Returns `Some(catalog_key)` when a catalog entry is found, `None` otherwise.
fn simulate_lookup(
    marking: &str,
    us: Option<&catalog::Catalog>,
    ca: Option<&catalog::Catalog>,
) -> Option<String> {
    if let Some(cat) = us {
        if cat.marking(marking).is_some() {
            return Some(marking.to_owned());
        }
        if let Some((key, _)) = cat.marking_by_mcs_level(marking) {
            return Some(key.clone());
        }
    }
    if let Some(cat) = ca {
        if cat.marking(marking).is_some() {
            return Some(marking.to_owned());
        }
        if let Some((key, _)) = cat.marking_by_mcs_level(marking) {
            return Some(key.clone());
        }
    }
    None
}

// ---------------------------------------------------------------------------
// US CUI group headers — direct key lookup
// ---------------------------------------------------------------------------

/// # TEST-ID: MKL-001
/// # REQUIREMENT: US CUI group header marking (translated by setrans) resolves via direct key
/// # COMPLIANCE: NIST SP 800-53 AC-16
#[test]
fn us_cui_direct_key_lookup_resolves() {
    let us = catalog::load_catalog(us_catalog_path()).ok();
    let ca = catalog::load_catalog(ca_catalog_path()).ok();

    // "CUI//LEI" is a typical translated marking for a US CUI group header.
    let result = simulate_lookup("CUI//LEI", us.as_ref(), ca.as_ref());
    assert_eq!(
        result.as_deref(),
        Some("CUI//LEI"),
        "US CUI direct key lookup should resolve CUI//LEI"
    );
}

/// # TEST-ID: MKL-002
/// # REQUIREMENT: US CUI SystemLow group header does not produce a catalog match
/// # COMPLIANCE: NIST SP 800-53 AC-16
#[test]
fn system_level_marking_returns_none_from_catalog() {
    let us = catalog::load_catalog(us_catalog_path()).ok();
    let ca = catalog::load_catalog(ca_catalog_path()).ok();

    // SystemLow is handled by the system-info branch, not the catalog lookup.
    let result = simulate_lookup("SystemLow", us.as_ref(), ca.as_ref());
    assert!(
        result.is_none(),
        "SystemLow should not match any catalog entry"
    );
}

// ---------------------------------------------------------------------------
// Canadian group headers — MCS level fallback
// ---------------------------------------------------------------------------

/// # TEST-ID: MKL-003
/// # REQUIREMENT: Canadian Protected A group header (raw s1:c300) resolves via mcs_level fallback
/// # COMPLIANCE: NIST SP 800-53 AC-16
#[test]
fn canadian_pa_raw_level_resolves_via_mcs_fallback() {
    let us = catalog::load_catalog(us_catalog_path()).ok();
    let ca = catalog::load_catalog(ca_catalog_path()).ok();

    // "s1:c300" is what appears in the group header when setrans has no
    // translation for c300.  The fallback should map this to PROTECTED-A.
    let result = simulate_lookup("s1:c300", us.as_ref(), ca.as_ref());
    assert_eq!(
        result.as_deref(),
        Some("PROTECTED-A"),
        "s1:c300 should resolve to PROTECTED-A via MCS level fallback"
    );
}

/// # TEST-ID: MKL-004
/// # REQUIREMENT: Canadian Protected B group header (raw s2:c301) resolves via mcs_level fallback
/// # COMPLIANCE: NIST SP 800-53 AC-16
#[test]
fn canadian_pb_raw_level_resolves_via_mcs_fallback() {
    let us = catalog::load_catalog(us_catalog_path()).ok();
    let ca = catalog::load_catalog(ca_catalog_path()).ok();

    let result = simulate_lookup("s2:c301", us.as_ref(), ca.as_ref());
    assert_eq!(
        result.as_deref(),
        Some("PROTECTED-B"),
        "s2:c301 should resolve to PROTECTED-B via MCS level fallback"
    );
}

/// # TEST-ID: MKL-005
/// # REQUIREMENT: Canadian Protected C group header (raw s3:c302) resolves via mcs_level fallback
/// # COMPLIANCE: NIST SP 800-53 AC-16
#[test]
fn canadian_pc_raw_level_resolves_via_mcs_fallback() {
    let us = catalog::load_catalog(us_catalog_path()).ok();
    let ca = catalog::load_catalog(ca_catalog_path()).ok();

    let result = simulate_lookup("s3:c302", us.as_ref(), ca.as_ref());
    assert_eq!(
        result.as_deref(),
        Some("PROTECTED-C"),
        "s3:c302 should resolve to PROTECTED-C via MCS level fallback"
    );
}

/// # TEST-ID: MKL-006
/// # REQUIREMENT: Unknown raw MCS level produces no catalog match (fail-closed)
/// # COMPLIANCE: NIST SP 800-53 AC-16
#[test]
fn unknown_raw_level_returns_none() {
    let us = catalog::load_catalog(us_catalog_path()).ok();
    let ca = catalog::load_catalog(ca_catalog_path()).ok();

    // s9:c999 is not in either catalog.
    let result = simulate_lookup("s9:c999", us.as_ref(), ca.as_ref());
    assert!(
        result.is_none(),
        "s9:c999 should not match any catalog entry — fail closed"
    );
}

/// # TEST-ID: MKL-007
/// # REQUIREMENT: US catalog does not shadow Canadian marking for Canadian MCS levels
/// # COMPLIANCE: NIST SP 800-53 AC-16
#[test]
fn us_catalog_does_not_shadow_canadian_marking() {
    let us = catalog::load_catalog(us_catalog_path()).ok();
    let ca = catalog::load_catalog(ca_catalog_path()).ok();

    // The US catalog should have no entry for s1:c300 (Canadian range c300+).
    // The result should come from the Canadian catalog, not the US one.
    if let Some(ref us_cat) = us {
        assert!(
            us_cat.marking_by_mcs_level("s1:c300").is_none(),
            "US catalog must not contain a c300 entry (Canadian range)"
        );
    }
    let result = simulate_lookup("s1:c300", us.as_ref(), ca.as_ref());
    assert_eq!(
        result.as_deref(),
        Some("PROTECTED-A"),
        "s1:c300 should still resolve from Canadian catalog"
    );
}
