// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//! Integration tests for `posture::catalog::lookup`.
//!
//! Verifies that `lookup()` returns correct descriptors and that all catalog
//! entries with display-layer coverage have non-empty `description` fields.

use umrs_platform::posture::{IndicatorId, catalog::INDICATORS, lookup};

// ===========================================================================
// lookup() correctness
// ===========================================================================

#[test]
fn lookup_returns_some_for_known_ids() {
    assert!(lookup(IndicatorId::KptrRestrict).is_some());
    assert!(lookup(IndicatorId::FipsEnabled).is_some());
    assert!(lookup(IndicatorId::BluetoothBlacklisted).is_some());
    assert!(lookup(IndicatorId::CorePattern).is_some());
}

#[test]
fn lookup_kptr_restrict_label_is_correct() {
    let d = lookup(IndicatorId::KptrRestrict).expect("KptrRestrict must be in catalog");
    assert_eq!(d.label, "Kernel Pointer Restriction");
}

#[test]
fn lookup_kptr_restrict_description_is_nonempty() {
    let d = lookup(IndicatorId::KptrRestrict).expect("KptrRestrict must be in catalog");
    assert!(
        !d.description.is_empty(),
        "KptrRestrict must have a description"
    );
}

#[test]
fn lookup_kptr_restrict_recommended_is_some() {
    let d = lookup(IndicatorId::KptrRestrict).expect("KptrRestrict must be in catalog");
    assert_eq!(d.recommended, Some("2 (hidden from all users)"));
}

#[test]
fn lookup_fips_enabled_description_mentions_fips() {
    let d = lookup(IndicatorId::FipsEnabled).expect("FipsEnabled must be in catalog");
    assert!(
        d.description.contains("FIPS"),
        "FipsEnabled description must mention FIPS, got: {}",
        d.description
    );
}

#[test]
fn lookup_fips_enabled_recommended_is_some() {
    let d = lookup(IndicatorId::FipsEnabled).expect("FipsEnabled must be in catalog");
    assert!(
        d.recommended.is_some(),
        "FipsEnabled must have a recommended value"
    );
}

#[test]
fn lookup_mitigations_recommended_mentions_cmdline() {
    let d = lookup(IndicatorId::Mitigations).expect("Mitigations must be in catalog");
    let rec = d.recommended.expect("Mitigations must have a recommended value");
    assert!(
        rec.contains("cmdline"),
        "Mitigations recommendation must reference cmdline, got: {rec}"
    );
}

#[test]
fn lookup_lockdown_recommended_mentions_integrity() {
    let d = lookup(IndicatorId::Lockdown).expect("Lockdown must be in catalog");
    let rec = d.recommended.expect("Lockdown must have a recommended value");
    assert!(
        rec.contains("integrity"),
        "Lockdown recommendation must mention integrity, got: {rec}"
    );
}

#[test]
fn lookup_bluetooth_recommended_mentions_blacklist() {
    let d =
        lookup(IndicatorId::BluetoothBlacklisted).expect("BluetoothBlacklisted must be in catalog");
    let rec = d.recommended.expect("BluetoothBlacklisted must have recommended value");
    assert!(
        rec.contains("blacklist"),
        "BluetoothBlacklisted recommendation must mention blacklist, got: {rec}"
    );
}

#[test]
fn lookup_spectre_v2_off_description_is_empty() {
    // CPU mitigation sub-indicators have no display-layer coverage yet.
    let d = lookup(IndicatorId::SpectreV2Off).expect("SpectreV2Off must be in catalog");
    assert_eq!(
        d.description, "",
        "SpectreV2Off description should be empty (Phase 2b pending)"
    );
    assert_eq!(
        d.recommended, None,
        "SpectreV2Off recommended should be None"
    );
}

#[test]
fn lookup_core_pattern_description_is_empty() {
    // CorePattern has no display-layer coverage yet.
    let d = lookup(IndicatorId::CorePattern).expect("CorePattern must be in catalog");
    assert_eq!(d.description, "", "CorePattern description should be empty");
    assert_eq!(
        d.recommended, None,
        "CorePattern recommended should be None"
    );
}

// ===========================================================================
// Catalog completeness — all 37 entries have description + recommended
// populated consistently with their display-layer coverage
// ===========================================================================

/// Indicators that have display-layer coverage in Phase 1/2a must have non-empty
/// descriptions. CPU mitigation sub-indicators (Phase 2b pending) may have empty
/// descriptions.
#[test]
fn phase1_and_2a_indicators_have_nonempty_descriptions() {
    // These 27 indicators had coverage in the original TUI indicator_description() function.
    let covered = [
        IndicatorId::Lockdown,
        IndicatorId::KexecLoadDisabled,
        IndicatorId::ModuleSigEnforce,
        IndicatorId::Mitigations,
        IndicatorId::Pti,
        IndicatorId::FipsEnabled,
        IndicatorId::ModulesDisabled,
        IndicatorId::RandomTrustCpu,
        IndicatorId::RandomTrustBootloader,
        IndicatorId::RandomizeVaSpace,
        IndicatorId::KptrRestrict,
        IndicatorId::UnprivBpfDisabled,
        IndicatorId::PerfEventParanoid,
        IndicatorId::YamaPtraceScope,
        IndicatorId::DmesgRestrict,
        IndicatorId::UnprivUsernsClone,
        IndicatorId::Sysrq,
        IndicatorId::SuidDumpable,
        IndicatorId::ProtectedSymlinks,
        IndicatorId::ProtectedHardlinks,
        IndicatorId::ProtectedFifos,
        IndicatorId::ProtectedRegular,
        IndicatorId::BluetoothBlacklisted,
        IndicatorId::UsbStorageBlacklisted,
        IndicatorId::FirewireCoreBlacklisted,
        IndicatorId::ThunderboltBlacklisted,
        IndicatorId::NfConntrackAcct,
    ];
    for id in covered {
        let d = lookup(id).unwrap_or_else(|| panic!("{id:?} must be in catalog"));
        assert!(
            !d.description.is_empty(),
            "{id:?} must have a non-empty description"
        );
    }
}

/// Every catalog entry has a non-None `id` (trivially true) and every entry is
/// unique — no duplicate ids.
#[test]
fn catalog_has_no_duplicate_ids() {
    let mut seen: Vec<IndicatorId> = Vec::new();
    for d in INDICATORS {
        assert!(
            !seen.contains(&d.id),
            "Duplicate catalog entry for {:?}",
            d.id
        );
        seen.push(d.id);
    }
}
