// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams

//! Integration tests for `SecurityIndicators` and `IndicatorValue`.
//!
//! These tests verify the type contract of the indicator snapshot — they do
//! not read live kernel attributes (which may not be available in CI).
//! Kernel-read behaviour is covered by integration environments with SELinux
//! and FIPS active.

use umrs_tui::app::{IndicatorValue, SecurityIndicators};

// ---------------------------------------------------------------------------
// SecurityIndicators default contract
// ---------------------------------------------------------------------------

/// Default `SecurityIndicators` must be fully unavailable — the fail-closed
/// baseline. No field should silently assume Active or Inactive.
#[test]
fn security_indicators_all_unavailable() {
    let indicators = SecurityIndicators::default();

    assert_eq!(
        indicators.selinux_status,
        IndicatorValue::Unavailable,
        "selinux_status should default to Unavailable"
    );
    assert_eq!(
        indicators.fips_mode,
        IndicatorValue::Unavailable,
        "fips_mode should default to Unavailable"
    );
    assert_eq!(
        indicators.active_lsm,
        IndicatorValue::Unavailable,
        "active_lsm should default to Unavailable"
    );
    assert_eq!(
        indicators.lockdown_mode,
        IndicatorValue::Unavailable,
        "lockdown_mode should default to Unavailable"
    );
    assert_eq!(
        indicators.secure_boot,
        IndicatorValue::Unavailable,
        "secure_boot should default to Unavailable"
    );
}

// ---------------------------------------------------------------------------
// IndicatorValue variant distinctness
// ---------------------------------------------------------------------------

/// The three `IndicatorValue` variants must be distinguishable from each other.
///
/// This is a type-contract test — the display and theme logic depends on these
/// being distinct discriminants, not aliased via equality.
#[test]
fn indicator_value_variants_are_distinct() {
    let active = IndicatorValue::Active("enforcing".to_owned());
    let inactive = IndicatorValue::Inactive("permissive".to_owned());
    let unavailable = IndicatorValue::Unavailable;

    assert_ne!(active, inactive);
    assert_ne!(active, unavailable);
    assert_ne!(inactive, unavailable);
}

// ---------------------------------------------------------------------------
// IndicatorValue::Active inner string
// ---------------------------------------------------------------------------

/// `IndicatorValue::Active` must carry the supplied display string intact.
///
/// The header render path extracts this string for badge text — it must
/// round-trip without mutation.
#[test]
fn indicator_value_active_contains_string() {
    let value = IndicatorValue::Active("enforcing".to_owned());
    if let IndicatorValue::Active(ref s) = value {
        assert_eq!(s, "enforcing");
    } else {
        panic!("expected Active variant");
    }
}

/// `IndicatorValue::Inactive` must carry the supplied display string intact.
#[test]
fn indicator_value_inactive_contains_string() {
    let value = IndicatorValue::Inactive("permissive".to_owned());
    if let IndicatorValue::Inactive(ref s) = value {
        assert_eq!(s, "permissive");
    } else {
        panic!("expected Inactive variant");
    }
}

// ---------------------------------------------------------------------------
// SecurityIndicators field assignment
// ---------------------------------------------------------------------------

/// Fields on `SecurityIndicators` can be individually set via struct literal
/// with update syntax. This verifies the public field layout is correct.
#[test]
fn security_indicators_field_assignment() {
    let indicators = SecurityIndicators {
        selinux_status: IndicatorValue::Active("enforcing".to_owned()),
        fips_mode: IndicatorValue::Active("active".to_owned()),
        lockdown_mode: IndicatorValue::Inactive("none".to_owned()),
        ..SecurityIndicators::default()
    };

    assert_eq!(
        indicators.selinux_status,
        IndicatorValue::Active("enforcing".to_owned())
    );
    assert_eq!(
        indicators.fips_mode,
        IndicatorValue::Active("active".to_owned())
    );
    assert_eq!(
        indicators.lockdown_mode,
        IndicatorValue::Inactive("none".to_owned())
    );
    // Unset fields remain unavailable
    assert_eq!(indicators.active_lsm, IndicatorValue::Unavailable);
    assert_eq!(indicators.secure_boot, IndicatorValue::Unavailable);
}
