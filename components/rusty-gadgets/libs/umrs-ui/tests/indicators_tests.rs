// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams

//! Integration tests for `SecurityIndicators` and `IndicatorValue`.
//!
//! These tests verify the type contract of the indicator snapshot — they do
//! not read live kernel attributes (which may not be available in CI).
//! Kernel-read behaviour is covered by integration environments with SELinux
//! and FIPS active.

use umrs_ui::app::{IndicatorValue, SecurityIndicators};

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
    let enabled = IndicatorValue::Enabled("Enforcing".to_owned());
    let disabled = IndicatorValue::Disabled("Permissive".to_owned());
    let unavailable = IndicatorValue::Unavailable;

    assert_ne!(enabled, disabled);
    assert_ne!(enabled, unavailable);
    assert_ne!(disabled, unavailable);
}

// ---------------------------------------------------------------------------
// IndicatorValue::Active inner string
// ---------------------------------------------------------------------------

/// `IndicatorValue::Enabled` must carry the supplied display string intact.
///
/// The header render path extracts this string for badge text — it must
/// round-trip without mutation.
#[test]
fn indicator_value_enabled_contains_string() {
    let value = IndicatorValue::Enabled("Enforcing".to_owned());
    if let IndicatorValue::Enabled(ref s) = value {
        assert_eq!(s, "Enforcing");
    } else {
        panic!("expected Enabled variant");
    }
}

/// `IndicatorValue::Disabled` must carry the supplied display string intact.
#[test]
fn indicator_value_disabled_contains_string() {
    let value = IndicatorValue::Disabled("Permissive".to_owned());
    if let IndicatorValue::Disabled(ref s) = value {
        assert_eq!(s, "Permissive");
    } else {
        panic!("expected Disabled variant");
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
        selinux_status: IndicatorValue::Enabled("Enforcing (Targeted)".to_owned()),
        fips_mode: IndicatorValue::Enabled("Enabled".to_owned()),
        lockdown_mode: IndicatorValue::Disabled("none".to_owned()),
        ..SecurityIndicators::default()
    };

    assert_eq!(
        indicators.selinux_status,
        IndicatorValue::Enabled("Enforcing (Targeted)".to_owned())
    );
    assert_eq!(
        indicators.fips_mode,
        IndicatorValue::Enabled("Enabled".to_owned())
    );
    assert_eq!(
        indicators.lockdown_mode,
        IndicatorValue::Disabled("none".to_owned())
    );
    // Unset fields remain unavailable
    assert_eq!(indicators.active_lsm, IndicatorValue::Unavailable);
    assert_eq!(indicators.secure_boot, IndicatorValue::Unavailable);
}
