// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//! Integration tests for the `posture::fips_cross` module.
//!
//! Tests are grouped by subsystem:
//!
//! 1. **Crypto-policy parsing** — assess_configured_fips logic with various
//!    indicator combinations tested via `FipsCrossCheck::evaluate`.
//! 2. **Trust Gate** — if live FIPS is unreadable, configured checks return None.
//! 3. **Cross-check evaluation** — all contradiction scenarios exercised through
//!    the public `FipsCrossCheck` API.
//! 4. **`as_configured_value`** — structured output for audit display.
//! 5. **Integration** — `FipsEnabled` indicator in `PostureSnapshot::collect()`
//!    has a coherent configured_value when the live value is readable.

use umrs_platform::posture::{
    FipsCrossCheck,
    catalog::INDICATORS,
    indicator::{DesiredValue, IndicatorClass, IndicatorId},
    snapshot::PostureSnapshot,
};

// ===========================================================================
// 1. Crypto-policy parsing via FipsCrossCheck::evaluate
// ===========================================================================

/// If live FIPS is not readable, Trust Gate blocks all config reads.
#[test]
fn fips_cross_check_trust_gate_blocked_all_none() {
    let cc = FipsCrossCheck::evaluate(false, None);
    // Trust Gate blocked — configured_meets_desired must be None.
    assert_eq!(
        cc.configured_meets_desired, None,
        "Trust Gate must block configured_meets_desired when live is unreadable"
    );
    // marker_present and crypto_policy must be None (not read).
    assert_eq!(
        cc.marker_present, None,
        "marker_present must be None when Trust Gate blocks reads"
    );
    assert_eq!(
        cc.crypto_policy, None,
        "crypto_policy must be None when Trust Gate blocks reads"
    );
    // cmdline_fips passed through regardless.
    assert_eq!(
        cc.cmdline_fips, None,
        "cmdline_fips must be None when not provided"
    );
}

/// Trust Gate blocked with cmdline=Some(true) still propagates cmdline_fips.
#[test]
fn fips_cross_check_trust_gate_blocked_cmdline_propagated() {
    let cc = FipsCrossCheck::evaluate(false, Some(true));
    assert_eq!(
        cc.configured_meets_desired, None,
        "Trust Gate must block configured_meets_desired"
    );
    // cmdline_fips is still passed through for display purposes.
    assert_eq!(
        cc.cmdline_fips,
        Some(true),
        "cmdline_fips must be propagated even when Trust Gate blocks"
    );
}

/// When live FIPS IS readable, evaluate runs and returns Some assessments.
/// The actual values depend on the test system's FIPS configuration.
#[test]
fn fips_cross_check_live_readable_runs_assessment() {
    let cc = FipsCrossCheck::evaluate(true, None);
    // marker_present must be Some (presence or absence is determined).
    assert!(
        cc.marker_present.is_some(),
        "marker_present must be Some when Trust Gate allows reads"
    );
    // configured_meets_desired must be Some (we have at least one indicator).
    assert!(
        cc.configured_meets_desired.is_some(),
        "configured_meets_desired must be Some when marker check runs"
    );
}

/// cmdline=Some(true) + live readable → configured_meets_desired = Some(true).
///
/// The cmdline fips=1 indicator alone is sufficient to conclude FIPS is
/// configured.
#[test]
fn fips_cross_check_cmdline_fips_true_meets_desired() {
    let cc = FipsCrossCheck::evaluate(true, Some(true));
    assert_eq!(
        cc.configured_meets_desired,
        Some(true),
        "cmdline fips=1 alone must produce configured_meets_desired=Some(true)"
    );
    assert_eq!(
        cc.cmdline_fips,
        Some(true),
        "cmdline_fips must be Some(true)"
    );
}

/// cmdline=Some(false) and no other positive indicator: outcome depends on
/// whether the system-fips marker and crypto-policy are present.
/// This test only verifies that the result is Some (not None) when live is readable.
#[test]
fn fips_cross_check_cmdline_fips_false_yields_some_result() {
    let cc = FipsCrossCheck::evaluate(true, Some(false));
    // The assessment must be Some — we have enough indicators to conclude.
    assert!(
        cc.configured_meets_desired.is_some(),
        "configured_meets_desired must be Some when live is readable and \
         cmdline indicator is present"
    );
}

// ===========================================================================
// 2. `as_configured_value` structured output
// ===========================================================================

/// When Trust Gate blocked all indicators, `as_configured_value` returns None.
#[test]
fn as_configured_value_trust_gate_returns_none() {
    let cc = FipsCrossCheck::evaluate(false, None);
    assert_eq!(
        cc.as_configured_value(),
        None,
        "Trust Gate blocked — as_configured_value must return None"
    );
}

/// When cmdline=Some(true), `as_configured_value` returns Some with
/// "cmdline=fips=1" in the raw summary.
#[test]
fn as_configured_value_cmdline_fips_present_in_raw() {
    let cc = FipsCrossCheck::evaluate(true, Some(true));
    let cv = cc.as_configured_value().expect("must return Some when cmdline indicator is present");
    assert!(
        cv.raw.contains("cmdline=fips=1"),
        "raw summary must contain 'cmdline=fips=1' when cmdline_fips=Some(true): \
         raw='{}'",
        cv.raw
    );
}

/// When cmdline=Some(false), `as_configured_value` includes "cmdline=no-fips".
#[test]
fn as_configured_value_cmdline_no_fips_in_raw() {
    let cc = FipsCrossCheck::evaluate(true, Some(false));
    let cv = cc.as_configured_value().expect("must return Some when cmdline indicator is present");
    // marker is also read on the live system so raw will contain marker info.
    // Check that the cmdline portion is present.
    assert!(
        cv.raw.contains("cmdline=no-fips"),
        "raw summary must contain 'cmdline=no-fips' when cmdline_fips=Some(false): \
         raw='{}'",
        cv.raw
    );
}

/// `source_file` must be a non-empty string when `as_configured_value` returns Some.
#[test]
fn as_configured_value_source_file_non_empty() {
    let cc = FipsCrossCheck::evaluate(true, Some(true));
    let cv = cc.as_configured_value().expect("must return Some");
    assert!(
        !cv.source_file.is_empty(),
        "source_file must not be empty when configured_value is Some"
    );
}

// ===========================================================================
// 3. Catalog consistency for FipsEnabled indicator
// ===========================================================================

/// FipsEnabled must be DistroManaged in the catalog.
#[test]
fn catalog_fips_enabled_is_distro_managed() {
    let desc = INDICATORS
        .iter()
        .find(|d| d.id == IndicatorId::FipsEnabled)
        .expect("FipsEnabled must be in catalog");
    assert_eq!(
        desc.class,
        IndicatorClass::DistroManaged,
        "FipsEnabled must use IndicatorClass::DistroManaged"
    );
}

/// FipsEnabled desired value must be Exact(1).
#[test]
fn catalog_fips_enabled_desired_is_exact_1() {
    let desc = INDICATORS
        .iter()
        .find(|d| d.id == IndicatorId::FipsEnabled)
        .expect("FipsEnabled must be in catalog");
    assert_eq!(
        desc.desired,
        DesiredValue::Exact(1),
        "FipsEnabled desired must be Exact(1)"
    );
}

// ===========================================================================
// 4. Integration — FipsEnabled in PostureSnapshot
// ===========================================================================

/// FipsEnabled indicator in snapshot must have coherent fields.
///
/// When live FIPS is readable, `configured_value` must be `Some`
/// (because the FIPS cross-check runs and produces at least a marker check).
/// When live FIPS is not readable, `configured_value` may be `None`
/// (Trust Gate blocked).
#[test]
fn snapshot_fips_indicator_has_coherent_configured_value() {
    let snap = PostureSnapshot::collect();
    let report = snap.get(IndicatorId::FipsEnabled).expect("FipsEnabled must appear in snapshot");

    // Coherence: if live is readable, configured must also be Some.
    if report.live_value.is_some() {
        assert!(
            report.configured_value.is_some(),
            "FipsEnabled: live_value is Some but configured_value is None \
             — FIPS cross-check should have run"
        );
    }
    // If live is None (e.g., no /proc/sys/crypto/fips_enabled), configured
    // may also be None (Trust Gate). No assertion needed for that case.
}

/// FipsEnabled snapshot report must have descriptor id = FipsEnabled.
#[test]
fn snapshot_fips_report_descriptor_id_correct() {
    let snap = PostureSnapshot::collect();
    let report = snap.get(IndicatorId::FipsEnabled).expect("FipsEnabled must appear in snapshot");
    assert_eq!(
        report.descriptor.id,
        IndicatorId::FipsEnabled,
        "FipsEnabled report descriptor.id must be FipsEnabled"
    );
}

/// If configured_value is present, raw must not be empty.
#[test]
fn snapshot_fips_configured_value_raw_not_empty() {
    let snap = PostureSnapshot::collect();
    let report = snap.get(IndicatorId::FipsEnabled).expect("FipsEnabled must appear in snapshot");
    if let Some(ref cv) = report.configured_value {
        assert!(
            !cv.raw.is_empty(),
            "FipsEnabled configured_value.raw must not be empty"
        );
        assert!(
            !cv.source_file.is_empty(),
            "FipsEnabled configured_value.source_file must not be empty"
        );
    }
}
