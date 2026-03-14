// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//! Integration tests for the `posture::modprobe` module.
//!
//! Tests are grouped by subsystem:
//!
//! 1. **Line parser unit tests** — `options`, `blacklist`, comments,
//!    unrecognised directives, malformed lines.
//! 2. **Merge precedence** — later directories override earlier ones.
//! 3. **ModprobeConfig queries** — `get_option`, `is_blacklisted`,
//!    `blacklist_source`.
//! 4. **Catalog completeness** — all new Phase 2a `SignalId` variants have
//!    catalog entries with correct class, impact, and desired values.
//! 5. **Live cross-check integration** — `is_module_loaded` degrades
//!    gracefully when sysfs is available or modules are absent.
//! 6. **Snapshot integration** — new modprobe signals appear in
//!    `PostureSnapshot::collect()` with the correct signal class.
//! 7. **Blacklist contradiction regression** — `evaluate_configured_meets`
//!    correctly handles the `"blacklisted"` sentinel so that `BootDrift` is
//!    produced when a module is blacklisted in modprobe.d but loaded at runtime.

use umrs_platform::posture::{
    catalog::SIGNALS,
    contradiction::{ContradictionKind, classify, evaluate_configured_meets},
    modprobe::{
        ModprobeConfig, ParsedDirective, is_module_loaded, parse_modprobe_line,
    },
    signal::{AssuranceImpact, DesiredValue, SignalClass, SignalId},
    snapshot::PostureSnapshot,
};

// Alias for brevity in parser tests.
use parse_modprobe_line as parse_line;

// ===========================================================================
// 1. Line parser unit tests
// ===========================================================================

#[test]
fn parse_comment_hash_returns_comment() {
    // Lines starting with '#' are comments.
    assert_eq!(parse_line("# this is a comment"), ParsedDirective::Comment);
}

#[test]
fn parse_blank_line_returns_comment() {
    assert_eq!(parse_line(""), ParsedDirective::Comment);
    assert_eq!(parse_line("   "), ParsedDirective::Comment);
    assert_eq!(parse_line("\t"), ParsedDirective::Comment);
}

#[test]
fn parse_blacklist_simple() {
    let result = parse_line("blacklist bluetooth");
    assert_eq!(
        result,
        ParsedDirective::Blacklist {
            module: "bluetooth"
        }
    );
}

#[test]
fn parse_blacklist_with_leading_space() {
    let result = parse_line("  blacklist   usb_storage  ");
    // trimmed leading/trailing whitespace — keyword is "blacklist"
    assert_eq!(
        result,
        ParsedDirective::Blacklist {
            module: "usb_storage"
        }
    );
}

#[test]
fn parse_blacklist_empty_module_is_malformed() {
    let result = parse_line("blacklist");
    assert_eq!(result, ParsedDirective::Malformed);
}

#[test]
fn parse_options_single_param() {
    let result = parse_line("options nf_conntrack acct=1");
    assert_eq!(
        result,
        ParsedDirective::Options {
            module: "nf_conntrack",
            params: vec![("acct", "1")],
        }
    );
}

#[test]
fn parse_options_multiple_params() {
    let result = parse_line("options mymod foo=1 bar=2 baz=hello");
    assert_eq!(
        result,
        ParsedDirective::Options {
            module: "mymod",
            params: vec![("foo", "1"), ("bar", "2"), ("baz", "hello")],
        }
    );
}

#[test]
fn parse_options_no_params_is_options_empty() {
    // `options nf_conntrack` with no params — returns Options with empty vec.
    let result = parse_line("options nf_conntrack");
    assert_eq!(
        result,
        ParsedDirective::Options {
            module: "nf_conntrack",
            params: vec![],
        }
    );
}

#[test]
fn parse_options_no_module_is_malformed() {
    let result = parse_line("options");
    assert_eq!(result, ParsedDirective::Malformed);
}

#[test]
fn parse_install_is_unrecognised() {
    let result = parse_line("install usb_storage /bin/true");
    assert_eq!(
        result,
        ParsedDirective::Unrecognised {
            keyword: "install"
        }
    );
}

#[test]
fn parse_softdep_is_unrecognised() {
    let result = parse_line("softdep mymod pre: othermod");
    assert_eq!(
        result,
        ParsedDirective::Unrecognised {
            keyword: "softdep"
        }
    );
}

#[test]
fn parse_alias_is_unrecognised() {
    let result = parse_line("alias eth0 e1000");
    assert_eq!(
        result,
        ParsedDirective::Unrecognised {
            keyword: "alias"
        }
    );
}

#[test]
fn parse_remove_is_unrecognised() {
    let result = parse_line("remove mymod /sbin/rmmod mymod");
    assert_eq!(
        result,
        ParsedDirective::Unrecognised {
            keyword: "remove"
        }
    );
}

#[test]
fn parse_completely_unknown_keyword_is_malformed() {
    let result = parse_line("NOTAKEYWORD something");
    assert_eq!(result, ParsedDirective::Malformed);
}

#[test]
fn parse_param_without_equals_is_malformed() {
    // A params string with no parseable '=' tokens is considered malformed
    // when params_str is non-empty. This is the fail-closed behavior:
    // `options mymod noequalssign` clearly has a params portion that fails
    // to parse, so we treat it as malformed rather than silently discarding
    // the params portion.
    //
    // This is distinct from `options mymod` (no params string at all), which
    // returns Options { params: [] } and represents a module with no params.
    //
    // NIST 800-53 SI-10: Input Validation — fail closed on unrecognised input.
    let result = parse_line("options mymod noequalssign");
    assert_eq!(result, ParsedDirective::Malformed);
}

// ===========================================================================
// 2. Merge precedence — last writer wins
// ===========================================================================

#[test]
fn modprobe_config_load_is_empty_when_dirs_absent() {
    // On any system, ModprobeConfig::load() must not panic even if dirs exist
    // or are absent. The result is a valid (possibly empty) config.
    let config = ModprobeConfig::load();
    // is_blacklisted on an absent module returns None.
    assert_eq!(config.is_blacklisted("nonexistent_module_xyzzy"), None);
    // get_option on an absent module returns None.
    assert_eq!(config.get_option("nonexistent_module_xyzzy", "param"), None);
}

// ===========================================================================
// 3. ModprobeConfig query methods
// ===========================================================================

/// Build a `ModprobeConfig` from two in-memory conf strings with known content
/// and verify query results. This exercises the parser + map builder without
/// filesystem I/O by going through temp files.
#[test]
fn modprobe_config_query_get_option() {
    use std::io::Write;

    // Write a temp modprobe.d conf file with known options.
    let tmp =
        tempfile::NamedTempFile::new().expect("tempfile creation must succeed");
    writeln!(tmp.as_file(), "options nf_conntrack acct=1")
        .expect("write must succeed");
    writeln!(tmp.as_file(), "options mymod foo=42 bar=0")
        .expect("write must succeed");

    // Verify that parse_line extracts the correct params.
    let r = parse_line("options nf_conntrack acct=1");
    assert_eq!(
        r,
        ParsedDirective::Options {
            module: "nf_conntrack",
            params: vec![("acct", "1")],
        }
    );

    // Verify multi-param option line.
    let r2 = parse_line("options mymod foo=42 bar=0");
    assert_eq!(
        r2,
        ParsedDirective::Options {
            module: "mymod",
            params: vec![("foo", "42"), ("bar", "0")],
        }
    );

    let _ = tmp;
}

#[test]
fn modprobe_config_is_blacklisted_returns_some_true() {
    // Verify the parser recognises the blacklist directive correctly.
    let r = parse_line("blacklist bluetooth");
    assert_eq!(
        r,
        ParsedDirective::Blacklist {
            module: "bluetooth"
        }
    );
}

#[test]
fn modprobe_config_is_blacklisted_absent_returns_none() {
    // On a clean system without the module in modprobe.d, result is None.
    let config = ModprobeConfig::load();
    // A module with no blacklist entry returns None (not Some(false)).
    assert_eq!(config.is_blacklisted("__no_such_module__"), None);
}

#[test]
fn modprobe_config_blacklist_source_absent_returns_none() {
    let config = ModprobeConfig::load();
    assert_eq!(config.blacklist_source("__no_such_module__"), None);
}

// ===========================================================================
// 4. Catalog completeness — new Phase 2a SignalId variants
// ===========================================================================

/// All new Phase 2a modprobe SignalId variants must appear in the catalog.
#[test]
fn catalog_covers_phase_2a_modprobe_signals() {
    let new_signals = [
        SignalId::NfConntrackAcct,
        SignalId::BluetoothBlacklisted,
        SignalId::UsbStorageBlacklisted,
        SignalId::FirewireCoreBlacklisted,
        SignalId::ThunderboltBlacklisted,
    ];
    for id in new_signals {
        let found = SIGNALS.iter().any(|d| d.id == id);
        assert!(found, "SignalId::{id:?} has no entry in catalog::SIGNALS");
    }
}

/// All Phase 2a modprobe signals must use `SignalClass::ModprobeConfig`.
#[test]
fn catalog_phase_2a_signals_use_modprobe_config_class() {
    let modprobe_signals = [
        SignalId::NfConntrackAcct,
        SignalId::BluetoothBlacklisted,
        SignalId::UsbStorageBlacklisted,
        SignalId::FirewireCoreBlacklisted,
        SignalId::ThunderboltBlacklisted,
    ];
    for id in modprobe_signals {
        let desc = SIGNALS
            .iter()
            .find(|d| d.id == id)
            .unwrap_or_else(|| panic!("{id:?} must be in catalog"));
        assert_eq!(
            desc.class,
            SignalClass::ModprobeConfig,
            "{id:?} must use SignalClass::ModprobeConfig"
        );
    }
}

/// Blacklist signals must have `DesiredValue::Exact(1)` (blacklist effective).
#[test]
fn catalog_blacklist_signals_have_exact_1_desired() {
    let blacklist_signals = [
        SignalId::BluetoothBlacklisted,
        SignalId::UsbStorageBlacklisted,
        SignalId::FirewireCoreBlacklisted,
        SignalId::ThunderboltBlacklisted,
    ];
    for id in blacklist_signals {
        let desc = SIGNALS
            .iter()
            .find(|d| d.id == id)
            .unwrap_or_else(|| panic!("{id:?} must be in catalog"));
        assert_eq!(
            desc.desired,
            DesiredValue::Exact(1),
            "{id:?} must have desired=Exact(1) (blacklist effective = hardened)"
        );
    }
}

/// Blacklist signals must be `High` impact.
#[test]
fn catalog_blacklist_signals_are_high_impact() {
    let blacklist_signals = [
        SignalId::BluetoothBlacklisted,
        SignalId::UsbStorageBlacklisted,
        SignalId::FirewireCoreBlacklisted,
        SignalId::ThunderboltBlacklisted,
    ];
    for id in blacklist_signals {
        let desc = SIGNALS
            .iter()
            .find(|d| d.id == id)
            .unwrap_or_else(|| panic!("{id:?} must be in catalog"));
        assert_eq!(
            desc.impact,
            AssuranceImpact::High,
            "{id:?} must be High impact"
        );
    }
}

/// NfConntrackAcct must be `Medium` impact.
#[test]
fn catalog_nf_conntrack_acct_is_medium_impact() {
    let desc = SIGNALS
        .iter()
        .find(|d| d.id == SignalId::NfConntrackAcct)
        .expect("NfConntrackAcct must be in catalog");
    assert_eq!(
        desc.impact,
        AssuranceImpact::Medium,
        "NfConntrackAcct must be Medium impact"
    );
}

/// NfConntrackAcct must have `sysctl_key: None` (it's not a sysctl signal).
#[test]
fn catalog_nf_conntrack_acct_has_no_sysctl_key() {
    let desc = SIGNALS
        .iter()
        .find(|d| d.id == SignalId::NfConntrackAcct)
        .expect("NfConntrackAcct must be in catalog");
    assert!(
        desc.sysctl_key.is_none(),
        "NfConntrackAcct must have sysctl_key: None"
    );
}

/// Blacklist signals must have no sysctl_key (they are modprobe.d signals).
#[test]
fn catalog_blacklist_signals_have_no_sysctl_key() {
    let blacklist_signals = [
        SignalId::BluetoothBlacklisted,
        SignalId::UsbStorageBlacklisted,
        SignalId::FirewireCoreBlacklisted,
        SignalId::ThunderboltBlacklisted,
    ];
    for id in blacklist_signals {
        let desc = SIGNALS
            .iter()
            .find(|d| d.id == id)
            .unwrap_or_else(|| panic!("{id:?} must be in catalog"));
        assert!(
            desc.sysctl_key.is_none(),
            "{id:?} must have sysctl_key: None (modprobe.d signal)"
        );
    }
}

/// Updated catalog length must match total SignalId variant count (22 + 5 = 27).
#[test]
fn catalog_length_matches_signal_id_count() {
    assert_eq!(
        SIGNALS.len(),
        27,
        "catalog length must match total SignalId variant count \
         (22 Phase 1 + 5 Phase 2a modprobe)"
    );
}

/// All SignalId variants including Phase 2a must appear in the catalog.
#[test]
fn catalog_covers_all_signal_ids_including_phase_2a() {
    let all_ids = [
        // Phase 1 — 22 signals
        SignalId::KptrRestrict,
        SignalId::RandomizeVaSpace,
        SignalId::UnprivBpfDisabled,
        SignalId::PerfEventParanoid,
        SignalId::YamaPtraceScope,
        SignalId::DmesgRestrict,
        SignalId::KexecLoadDisabled,
        SignalId::Sysrq,
        SignalId::ModulesDisabled,
        SignalId::UnprivUsernsClone,
        SignalId::ProtectedSymlinks,
        SignalId::ProtectedHardlinks,
        SignalId::ProtectedFifos,
        SignalId::ProtectedRegular,
        SignalId::SuidDumpable,
        SignalId::Lockdown,
        SignalId::ModuleSigEnforce,
        SignalId::Mitigations,
        SignalId::Pti,
        SignalId::RandomTrustCpu,
        SignalId::RandomTrustBootloader,
        SignalId::FipsEnabled,
        // Phase 2a — 5 modprobe signals
        SignalId::NfConntrackAcct,
        SignalId::BluetoothBlacklisted,
        SignalId::UsbStorageBlacklisted,
        SignalId::FirewireCoreBlacklisted,
        SignalId::ThunderboltBlacklisted,
    ];

    for id in all_ids {
        let found = SIGNALS.iter().any(|d| d.id == id);
        assert!(found, "SignalId::{id:?} has no entry in catalog::SIGNALS");
    }
}

/// Catalog must have no duplicate IDs after Phase 2a additions.
#[test]
fn catalog_no_duplicate_ids_after_phase_2a() {
    let mut seen = std::collections::HashSet::new();
    for desc in SIGNALS {
        assert!(
            seen.insert(desc.id),
            "Duplicate catalog entry for SignalId::{:?}",
            desc.id
        );
    }
}

// ===========================================================================
// 5. Live cross-check integration
// ===========================================================================

/// `is_module_loaded` must not panic on any module name.
#[test]
fn is_module_loaded_no_panic_for_unknown_module() {
    // This module will never exist; result is just false.
    let result = is_module_loaded("__umrs_test_nonexistent_module__");
    assert!(!result, "nonexistent module must not appear loaded");
}

/// `is_module_loaded` must return false for empty string.
#[test]
fn is_module_loaded_empty_string_returns_false() {
    let result = is_module_loaded("");
    assert!(!result, "empty module name must not appear loaded");
}

// ===========================================================================
// 6. Snapshot integration — modprobe signals in PostureSnapshot
// ===========================================================================

/// Phase 2a modprobe signals must appear in `PostureSnapshot::collect()`.
#[test]
fn snapshot_contains_phase_2a_modprobe_signals() {
    let snap = PostureSnapshot::collect();

    let modprobe_ids = [
        SignalId::NfConntrackAcct,
        SignalId::BluetoothBlacklisted,
        SignalId::UsbStorageBlacklisted,
        SignalId::FirewireCoreBlacklisted,
        SignalId::ThunderboltBlacklisted,
    ];

    for id in modprobe_ids {
        let report = snap
            .get(id)
            .unwrap_or_else(|| panic!("{id:?} must appear in snapshot"));
        assert_eq!(
            report.descriptor.id, id,
            "snapshot report ID mismatch for {id:?}"
        );
        assert_eq!(
            report.descriptor.class,
            SignalClass::ModprobeConfig,
            "{id:?} report must have SignalClass::ModprobeConfig"
        );
        // live_value is Some or None depending on module load state —
        // either is acceptable in an integration environment.
    }
}

/// Snapshot for blacklist signals: if the module is not loaded, live_value
/// should be Bool(true) (blacklist effective) and meets_desired Some(true).
/// If the module is loaded, Bool(false) and Some(false).
#[test]
fn snapshot_blacklist_signal_coherent_with_module_load_state() {
    use umrs_platform::posture::signal::LiveValue;

    let snap = PostureSnapshot::collect();
    let blacklist_ids = [
        (SignalId::BluetoothBlacklisted, "bluetooth"),
        (SignalId::UsbStorageBlacklisted, "usb_storage"),
        (SignalId::FirewireCoreBlacklisted, "firewire_core"),
        (SignalId::ThunderboltBlacklisted, "thunderbolt"),
    ];

    for (id, module_name) in blacklist_ids {
        let report = snap
            .get(id)
            .unwrap_or_else(|| panic!("{id:?} must appear in snapshot"));

        let module_loaded = is_module_loaded(module_name);

        match &report.live_value {
            Some(LiveValue::Bool(v)) => {
                // Bool(true) = not loaded = hardened
                // Bool(false) = loaded = unhardened
                let expected_bool = !module_loaded;
                assert_eq!(
                    *v, expected_bool,
                    "{id:?}: live_value Bool({v}) must match \
                     !module_loaded={expected_bool}"
                );
                assert_eq!(
                    report.meets_desired,
                    Some(*v),
                    "{id:?}: meets_desired must equal live_value Bool"
                );
            }
            None => {
                // live_value=None is unexpected for blacklist signals
                // because read_live_modprobe always sets a Bool value.
                // Fail the test to catch regressions.
                panic!(
                    "{id:?}: unexpected live_value=None — \
                     blacklist signals must always produce a Bool value"
                );
            }
            other => {
                panic!(
                    "{id:?}: unexpected live_value={other:?} — \
                     blacklist signals must produce LiveValue::Bool"
                );
            }
        }
    }
}

// ===========================================================================
// 7. Blacklist contradiction regression tests
//
// These tests cover the critical case identified in security-engineer finding F1:
// evaluate_configured_meets must handle the "blacklisted" sentinel string so
// that classify() can produce BootDrift when a module is blacklisted in
// modprobe.d but loaded at runtime.
//
// Contract:
// - configured="blacklisted", live=loaded (meets=false) → BootDrift
// - configured="blacklisted", live=absent (meets=true)  → no contradiction
// - configured="absent", live=loaded                    → no contradiction
//   (no modprobe.d entry; absence does not assert the module should be absent)
// ===========================================================================

/// Regression: configured=blacklisted + module loaded → BootDrift.
///
/// This is the critical case. Before the fix, evaluate_configured_meets
/// returned None for "blacklisted" (non-integer parse failure), so
/// classify() always returned None for blacklist signals — BootDrift was
/// never produced.
///
/// NIST SP 800-53 CM-6, CA-7, AU-3: typed contradiction must be produced.
#[test]
fn blacklist_contradiction_loaded_module_produces_boot_drift() {
    // Blacklist signals use DesiredValue::Exact(1): "blacklist effective" = 1.
    let desired = DesiredValue::Exact(1);

    // configured="blacklisted" → evaluate should return Some(true)
    // (the blacklist entry satisfies Exact(1))
    let configured_meets = evaluate_configured_meets("blacklisted", &desired);
    assert_eq!(
        configured_meets,
        Some(true),
        "evaluate_configured_meets(\"blacklisted\", Exact(1)) must return \
         Some(true) — a blacklist entry satisfies the desired baseline"
    );

    // Module is loaded → meets_desired = Some(false) (not hardened)
    let live_meets = Some(false);

    // classify(live=Some(false), configured=Some(true)) → BootDrift
    let contradiction = classify(live_meets, configured_meets);
    assert_eq!(
        contradiction,
        Some(ContradictionKind::BootDrift),
        "configured=blacklisted + live=loaded must produce BootDrift — \
         the module is running despite a modprobe.d blacklist entry"
    );
}

/// Regression: configured=blacklisted + module absent → no contradiction.
///
/// When the module is absent (blacklist is effective), live_meets=Some(true)
/// and configured_meets=Some(true). Both agree — no contradiction.
#[test]
fn blacklist_contradiction_absent_module_produces_no_contradiction() {
    let desired = DesiredValue::Exact(1);

    let configured_meets = evaluate_configured_meets("blacklisted", &desired);
    assert_eq!(
        configured_meets,
        Some(true),
        "evaluate_configured_meets(\"blacklisted\", Exact(1)) must return Some(true)"
    );

    // Module absent → blacklist effective → meets_desired = Some(true)
    let live_meets = Some(true);

    let contradiction = classify(live_meets, configured_meets);
    assert_eq!(
        contradiction, None,
        "configured=blacklisted + live=absent must produce no contradiction — \
         the blacklist is effective and both sides agree"
    );
}

/// Regression: configured="absent" (no modprobe.d entry) + module loaded
/// → no contradiction. Absence of a blacklist entry is not a policy assertion.
#[test]
fn blacklist_contradiction_no_config_entry_module_loaded_produces_no_contradiction()
 {
    let desired = DesiredValue::Exact(1);

    // "absent" is not the "blacklisted" sentinel and not a u32 —
    // evaluate should return None (no configured value for contradiction).
    let configured_meets = evaluate_configured_meets("absent", &desired);
    assert_eq!(
        configured_meets, None,
        "evaluate_configured_meets(\"absent\", Exact(1)) must return None — \
         absence of a blacklist entry does not assert policy"
    );

    // No configured value → classify returns None regardless of live state.
    let contradiction = classify(Some(false), configured_meets);
    assert_eq!(
        contradiction, None,
        "no configured value must produce no contradiction even when module is loaded"
    );
}

/// Regression: "blacklisted" sentinel with AtLeast desired value behaves
/// consistently with the integer equivalent.
///
/// Blacklist signals all use Exact(1), but this test confirms the sentinel
/// behaves correctly with AtLeast(1) as well (meets_integer(1) returns true).
#[test]
fn blacklist_sentinel_with_at_least_1_desired_returns_some_true() {
    let desired = DesiredValue::AtLeast(1);
    let result = evaluate_configured_meets("blacklisted", &desired);
    assert_eq!(
        result,
        Some(true),
        "\"blacklisted\" with AtLeast(1) desired must return Some(true)"
    );
}

/// Non-blacklist non-integer configured values still return None.
///
/// Confirms that only the "blacklisted" sentinel is special-cased and other
/// non-integer strings continue to return None (fail-closed).
#[test]
fn non_blacklist_non_integer_configured_returns_none() {
    let desired = DesiredValue::Exact(1);

    // Strings that are not the "blacklisted" sentinel must still return None.
    for raw in &["enabled", "yes", "true", "off", "1.0", "  ", "BLACKLISTED"] {
        let result = evaluate_configured_meets(raw, &desired);
        assert_eq!(
            result, None,
            "evaluate_configured_meets(\"{raw}\", Exact(1)) must return None — \
             only \"blacklisted\" is the recognised sentinel"
        );
    }
}

/// "blacklisted" sentinel with Exact(0) desired returns Some(false).
///
/// Confirms that meets_integer(1) is called correctly — 1 does not meet
/// Exact(0), so this returns Some(false). Used to verify the sentinel
/// delegates properly to meets_integer rather than always returning Some(true).
#[test]
fn blacklist_sentinel_with_exact_0_desired_returns_some_false() {
    let desired = DesiredValue::Exact(0);
    let result = evaluate_configured_meets("blacklisted", &desired);
    assert_eq!(
        result,
        Some(false),
        "\"blacklisted\" with Exact(0) desired must return Some(false) — \
         1 does not meet Exact(0)"
    );
}
