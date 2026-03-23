// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//! Integration tests for `posture::display`.
//!
//! Verifies that `annotate_live_value`, `annotate_integer`, and
//! `annotate_signed_integer` produce the expected operator-facing strings for
//! known indicator/value combinations.
//!
//! In the test environment no gettext catalog is initialized, so `i18n::tr()`
//! falls back to returning the original msgid unchanged. The expected strings
//! in these tests therefore match the English msgid literals in display.rs.

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
        "0 (Pointers Visible)"
    );
    assert_eq!(
        annotate_integer(IndicatorId::KptrRestrict, 1),
        "1 (Hidden from Unprivileged)"
    );
    assert_eq!(
        annotate_integer(IndicatorId::KptrRestrict, 2),
        "2 (Hidden from All Users)"
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
        "0 (ASLR Disabled)"
    );
    assert_eq!(
        annotate_integer(IndicatorId::RandomizeVaSpace, 1),
        "1 (Partial Randomization)"
    );
    assert_eq!(
        annotate_integer(IndicatorId::RandomizeVaSpace, 2),
        "2 (Full ASLR)"
    );
}

#[test]
fn annotate_integer_unprivileged_bpf() {
    assert_eq!(
        annotate_integer(IndicatorId::UnprivBpfDisabled, 0),
        "0 (Unprivileged BPF Allowed)"
    );
    assert_eq!(
        annotate_integer(IndicatorId::UnprivBpfDisabled, 1),
        "1 (Restricted to CAP_BPF)"
    );
}

#[test]
fn annotate_integer_yama_ptrace_scope_all_levels() {
    assert_eq!(
        annotate_integer(IndicatorId::YamaPtraceScope, 0),
        "0 (Unrestricted)"
    );
    assert_eq!(
        annotate_integer(IndicatorId::YamaPtraceScope, 1),
        "1 (Children Only)"
    );
    assert_eq!(
        annotate_integer(IndicatorId::YamaPtraceScope, 2),
        "2 (Admin Only)"
    );
    assert_eq!(
        annotate_integer(IndicatorId::YamaPtraceScope, 3),
        "3 (No Attach)"
    );
}

#[test]
fn annotate_integer_dmesg_restrict() {
    assert_eq!(
        annotate_integer(IndicatorId::DmesgRestrict, 0),
        "0 (World-Readable)"
    );
    assert_eq!(
        annotate_integer(IndicatorId::DmesgRestrict, 1),
        "1 (Restricted)"
    );
}

#[test]
fn annotate_integer_modules_disabled() {
    assert_eq!(
        annotate_integer(IndicatorId::ModulesDisabled, 0),
        "0 (Loading Allowed)"
    );
    assert_eq!(
        annotate_integer(IndicatorId::ModulesDisabled, 1),
        "1 (Loading Locked)"
    );
}

#[test]
fn annotate_integer_unpriv_userns_clone() {
    assert_eq!(
        annotate_integer(IndicatorId::UnprivUsernsClone, 0),
        "0 (Restricted)"
    );
    assert_eq!(
        annotate_integer(IndicatorId::UnprivUsernsClone, 1),
        "1 (Allowed)"
    );
}

#[test]
fn annotate_integer_sysrq() {
    assert_eq!(
        annotate_integer(IndicatorId::Sysrq, 0),
        "0 (Fully Disabled)"
    );
    assert_eq!(
        annotate_integer(IndicatorId::Sysrq, 1),
        "1 (All Functions Enabled)"
    );
    // Bitmask values not in the table return bare decimal.
    assert_eq!(annotate_integer(IndicatorId::Sysrq, 176), "176");
}

#[test]
fn annotate_integer_suid_dumpable() {
    assert_eq!(
        annotate_integer(IndicatorId::SuidDumpable, 0),
        "0 (No Core Dumps)"
    );
    assert_eq!(
        annotate_integer(IndicatorId::SuidDumpable, 1),
        "1 (Core Dumps Enabled)"
    );
    assert_eq!(
        annotate_integer(IndicatorId::SuidDumpable, 2),
        "2 (Readable by Root Only)"
    );
}

#[test]
fn annotate_integer_protected_symlinks_and_hardlinks() {
    assert_eq!(
        annotate_integer(IndicatorId::ProtectedSymlinks, 0),
        "0 (Not Protected)"
    );
    assert_eq!(
        annotate_integer(IndicatorId::ProtectedSymlinks, 1),
        "1 (Protected)"
    );
    assert_eq!(
        annotate_integer(IndicatorId::ProtectedHardlinks, 0),
        "0 (Not Protected)"
    );
    assert_eq!(
        annotate_integer(IndicatorId::ProtectedHardlinks, 1),
        "1 (Protected)"
    );
}

#[test]
fn annotate_integer_protected_fifos_and_regular() {
    assert_eq!(
        annotate_integer(IndicatorId::ProtectedFifos, 0),
        "0 (Not Protected)"
    );
    assert_eq!(
        annotate_integer(IndicatorId::ProtectedFifos, 1),
        "1 (Partial Protection)"
    );
    assert_eq!(
        annotate_integer(IndicatorId::ProtectedFifos, 2),
        "2 (Fully Protected)"
    );
    assert_eq!(
        annotate_integer(IndicatorId::ProtectedRegular, 2),
        "2 (Fully Protected)"
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
        "0 (Accounting Off)"
    );
    assert_eq!(
        annotate_integer(IndicatorId::NfConntrackAcct, 1),
        "1 (Accounting On)"
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
        "-1 (Fully Open)"
    );
    assert_eq!(
        annotate_signed_integer(IndicatorId::PerfEventParanoid, i64::MIN),
        format!("{} (Fully Open)", i64::MIN)
    );
    // Zero — kernel profiling allowed
    assert_eq!(
        annotate_signed_integer(IndicatorId::PerfEventParanoid, 0),
        "0 (Kernel Profiling Allowed)"
    );
    // One — user profiling allowed
    assert_eq!(
        annotate_signed_integer(IndicatorId::PerfEventParanoid, 1),
        "1 (User Profiling Allowed)"
    );
    // Two and above — restricted
    assert_eq!(
        annotate_signed_integer(IndicatorId::PerfEventParanoid, 2),
        "2 (Restricted)"
    );
    assert_eq!(
        annotate_signed_integer(IndicatorId::PerfEventParanoid, 99),
        "99 (Restricted)"
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
        "Enabled"
    );
}

#[test]
fn annotate_live_value_bool_false_is_disabled() {
    assert_eq!(
        annotate_live_value(IndicatorId::FipsEnabled, &LiveValue::Bool(false)),
        "Disabled"
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
        "2 (Hidden from All Users)"
    );
}

#[test]
fn annotate_live_value_signed_integer_routes_to_annotate_signed_integer() {
    let lv = LiveValue::SignedInteger(-1);
    assert_eq!(
        annotate_live_value(IndicatorId::PerfEventParanoid, &lv),
        "-1 (Fully Open)"
    );
}
