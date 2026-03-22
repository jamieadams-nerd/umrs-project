// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//! Integration tests for `posture::display`.
//!
//! Verifies that `annotate_live_value`, `annotate_integer`, and
//! `annotate_signed_integer` produce the expected operator-facing strings for
//! known indicator/value combinations.

use umrs_platform::posture::{
    annotate_integer, annotate_live_value, annotate_signed_integer,
    indicator::{IndicatorId, LiveValue},
};

// ===========================================================================
// annotate_integer
// ===========================================================================

#[test]
fn annotate_integer_kptr_restrict_levels() {
    assert_eq!(
        annotate_integer(IndicatorId::KptrRestrict, 0),
        "0 (pointers visible)"
    );
    assert_eq!(
        annotate_integer(IndicatorId::KptrRestrict, 1),
        "1 (hidden from unprivileged)"
    );
    assert_eq!(
        annotate_integer(IndicatorId::KptrRestrict, 2),
        "2 (hidden from all users)"
    );
}

#[test]
fn annotate_integer_kptr_restrict_unknown_value() {
    // Out-of-range values should be returned as bare decimal strings.
    assert_eq!(annotate_integer(IndicatorId::KptrRestrict, 99), "99");
}

#[test]
fn annotate_integer_randomize_va_space_levels() {
    assert_eq!(
        annotate_integer(IndicatorId::RandomizeVaSpace, 0),
        "0 (ASLR disabled)"
    );
    assert_eq!(
        annotate_integer(IndicatorId::RandomizeVaSpace, 1),
        "1 (partial randomization)"
    );
    assert_eq!(
        annotate_integer(IndicatorId::RandomizeVaSpace, 2),
        "2 (full ASLR)"
    );
}

#[test]
fn annotate_integer_unprivileged_bpf() {
    assert_eq!(
        annotate_integer(IndicatorId::UnprivBpfDisabled, 0),
        "0 (unprivileged BPF allowed)"
    );
    assert_eq!(
        annotate_integer(IndicatorId::UnprivBpfDisabled, 1),
        "1 (restricted to CAP_BPF)"
    );
}

#[test]
fn annotate_integer_yama_ptrace_scope_all_levels() {
    assert_eq!(
        annotate_integer(IndicatorId::YamaPtraceScope, 0),
        "0 (unrestricted)"
    );
    assert_eq!(
        annotate_integer(IndicatorId::YamaPtraceScope, 1),
        "1 (children only)"
    );
    assert_eq!(
        annotate_integer(IndicatorId::YamaPtraceScope, 2),
        "2 (admin only)"
    );
    assert_eq!(
        annotate_integer(IndicatorId::YamaPtraceScope, 3),
        "3 (no attach)"
    );
}

#[test]
fn annotate_integer_dmesg_restrict() {
    assert_eq!(
        annotate_integer(IndicatorId::DmesgRestrict, 0),
        "0 (world-readable)"
    );
    assert_eq!(
        annotate_integer(IndicatorId::DmesgRestrict, 1),
        "1 (restricted)"
    );
}

#[test]
fn annotate_integer_modules_disabled() {
    assert_eq!(
        annotate_integer(IndicatorId::ModulesDisabled, 0),
        "0 (loading allowed)"
    );
    assert_eq!(
        annotate_integer(IndicatorId::ModulesDisabled, 1),
        "1 (loading locked)"
    );
}

#[test]
fn annotate_integer_unpriv_userns_clone() {
    assert_eq!(
        annotate_integer(IndicatorId::UnprivUsernsClone, 0),
        "0 (restricted)"
    );
    assert_eq!(
        annotate_integer(IndicatorId::UnprivUsernsClone, 1),
        "1 (allowed)"
    );
}

#[test]
fn annotate_integer_sysrq() {
    assert_eq!(
        annotate_integer(IndicatorId::Sysrq, 0),
        "0 (fully disabled)"
    );
    assert_eq!(
        annotate_integer(IndicatorId::Sysrq, 1),
        "1 (all functions enabled)"
    );
    // Bitmask values not in the table return bare decimal.
    assert_eq!(annotate_integer(IndicatorId::Sysrq, 176), "176");
}

#[test]
fn annotate_integer_suid_dumpable() {
    assert_eq!(
        annotate_integer(IndicatorId::SuidDumpable, 0),
        "0 (no core dumps)"
    );
    assert_eq!(
        annotate_integer(IndicatorId::SuidDumpable, 1),
        "1 (core dumps enabled)"
    );
    assert_eq!(
        annotate_integer(IndicatorId::SuidDumpable, 2),
        "2 (readable by root only)"
    );
}

#[test]
fn annotate_integer_protected_symlinks_and_hardlinks() {
    assert_eq!(
        annotate_integer(IndicatorId::ProtectedSymlinks, 0),
        "0 (not protected)"
    );
    assert_eq!(
        annotate_integer(IndicatorId::ProtectedSymlinks, 1),
        "1 (protected)"
    );
    assert_eq!(
        annotate_integer(IndicatorId::ProtectedHardlinks, 0),
        "0 (not protected)"
    );
    assert_eq!(
        annotate_integer(IndicatorId::ProtectedHardlinks, 1),
        "1 (protected)"
    );
}

#[test]
fn annotate_integer_protected_fifos_and_regular() {
    assert_eq!(
        annotate_integer(IndicatorId::ProtectedFifos, 0),
        "0 (not protected)"
    );
    assert_eq!(
        annotate_integer(IndicatorId::ProtectedFifos, 1),
        "1 (partial protection)"
    );
    assert_eq!(
        annotate_integer(IndicatorId::ProtectedFifos, 2),
        "2 (fully protected)"
    );
    assert_eq!(
        annotate_integer(IndicatorId::ProtectedRegular, 2),
        "2 (fully protected)"
    );
}

#[test]
fn annotate_integer_fips_enabled() {
    assert_eq!(
        annotate_integer(IndicatorId::FipsEnabled, 0),
        "0 (Disabled)"
    );
    assert_eq!(annotate_integer(IndicatorId::FipsEnabled, 1), "1 (Enabled)");
}

#[test]
fn annotate_integer_nf_conntrack_acct() {
    assert_eq!(
        annotate_integer(IndicatorId::NfConntrackAcct, 0),
        "0 (accounting off)"
    );
    assert_eq!(
        annotate_integer(IndicatorId::NfConntrackAcct, 1),
        "1 (accounting on)"
    );
}

#[test]
fn annotate_integer_cmdline_absent_indicator_no_annotation() {
    // CmdlineAbsent indicators have no integer annotation — value passes through bare.
    assert_eq!(annotate_integer(IndicatorId::Mitigations, 0), "0");
    assert_eq!(annotate_integer(IndicatorId::SpectreV2Off, 1), "1");
}

// ===========================================================================
// annotate_signed_integer
// ===========================================================================

#[test]
fn annotate_signed_integer_perf_event_paranoid_all_ranges() {
    // Negative values — fully open
    assert_eq!(
        annotate_signed_integer(IndicatorId::PerfEventParanoid, -1),
        "-1 (fully open)"
    );
    assert_eq!(
        annotate_signed_integer(IndicatorId::PerfEventParanoid, i64::MIN),
        format!("{} (fully open)", i64::MIN)
    );
    // Zero — kernel profiling allowed
    assert_eq!(
        annotate_signed_integer(IndicatorId::PerfEventParanoid, 0),
        "0 (kernel profiling allowed)"
    );
    // One — user profiling allowed
    assert_eq!(
        annotate_signed_integer(IndicatorId::PerfEventParanoid, 1),
        "1 (user profiling allowed)"
    );
    // Two and above — restricted
    assert_eq!(
        annotate_signed_integer(IndicatorId::PerfEventParanoid, 2),
        "2 (restricted)"
    );
    assert_eq!(
        annotate_signed_integer(IndicatorId::PerfEventParanoid, 99),
        "99 (restricted)"
    );
}

#[test]
fn annotate_signed_integer_unknown_indicator_passes_through() {
    // Indicators with no signed-integer annotation return bare string.
    assert_eq!(annotate_signed_integer(IndicatorId::KptrRestrict, 2), "2");
}

// ===========================================================================
// annotate_live_value — routing
// ===========================================================================

#[test]
fn annotate_live_value_bool_true_is_enabled() {
    assert_eq!(
        annotate_live_value(IndicatorId::FipsEnabled, &LiveValue::Bool(true)),
        "enabled"
    );
}

#[test]
fn annotate_live_value_bool_false_is_disabled() {
    assert_eq!(
        annotate_live_value(IndicatorId::FipsEnabled, &LiveValue::Bool(false)),
        "disabled"
    );
}

#[test]
fn annotate_live_value_absent_sentinel_becomes_not_present() {
    let absent = LiveValue::Text("absent".to_owned());
    assert_eq!(
        annotate_live_value(IndicatorId::Mitigations, &absent),
        "Not Present"
    );
}

#[test]
fn annotate_live_value_text_passthrough() {
    let text = LiveValue::Text("integrity".to_owned());
    assert_eq!(
        annotate_live_value(IndicatorId::Lockdown, &text),
        "integrity"
    );
}

#[test]
fn annotate_live_value_integer_routes_to_annotate_integer() {
    let lv = LiveValue::Integer(2);
    assert_eq!(
        annotate_live_value(IndicatorId::KptrRestrict, &lv),
        "2 (hidden from all users)"
    );
}

#[test]
fn annotate_live_value_signed_integer_routes_to_annotate_signed_integer() {
    let lv = LiveValue::SignedInteger(-1);
    assert_eq!(
        annotate_live_value(IndicatorId::PerfEventParanoid, &lv),
        "-1 (fully open)"
    );
}
