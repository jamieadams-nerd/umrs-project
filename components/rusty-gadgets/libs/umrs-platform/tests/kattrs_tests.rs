// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a, Imodium Operator)
//! =============================================================================
//! umrs-platform — kattrs Integration Tests
//! =============================================================================
//!
//! Integration tests covering:
//!   • TPI (Two-Path Independence) parsing via validate_type_redundant
//!   • EnforceState parsing (SelinuxEnforce)
//!   • MLS state parsing (SelinuxMls)
//!   • Dual-boolean parsing (GenericDualBool)
//!   • Policy version parsing (SelinuxPolicyVers)
//!   • AttributeCard Display formatting
//!   • KernelLockdown TPI parsing (LockdownMode enum)
//!   • ModuleLoadLatch single-bit parsing
//!
//! All tests are pure-logic (no kernel I/O required).
//!
//! Run with:
//!   cargo test -p umrs-platform
//! =============================================================================

use std::time::SystemTime;
use umrs_platform::kattrs::{
    AttributeCard, DualBool, EnforceState, GenericDualBool, KernelFileSource,
    KernelLockdown, LockdownMode, ModuleLoadLatch, SelinuxEnforce, SelinuxMls,
    SelinuxPolicyVers, StaticSource, validate_type_redundant,
};

// =============================================================================
// TPI: validate_type_redundant
// =============================================================================

#[test]
fn validate_type_redundant_valid() {
    // Full four-field context — both parse paths agree on "sshd_t"
    let result = validate_type_redundant("system_u:system_r:sshd_t:s0");
    assert!(result.is_ok(), "expected Ok, got: {:?}", result);
    assert_eq!(result.unwrap(), "sshd_t");
}

#[test]
fn validate_type_redundant_too_few_parts() {
    // Only two fields — path A (nom take_until) cannot find a third ":"
    let result = validate_type_redundant("user_u:role_r");
    assert!(result.is_err(), "expected Err for truncated context");
}

#[test]
fn validate_type_redundant_empty() {
    // Empty input — both paths fail closed
    let result = validate_type_redundant("");
    assert!(result.is_err(), "expected Err for empty context");
}

// =============================================================================
// SelinuxEnforce parsing
// =============================================================================

#[test]
fn enforce_state_parse_enforcing() {
    let result = SelinuxEnforce::parse(b"1");
    assert!(result.is_ok(), "expected Ok, got: {:?}", result);
    assert_eq!(result.unwrap(), EnforceState::Enforcing);
}

#[test]
fn enforce_state_parse_permissive() {
    let result = SelinuxEnforce::parse(b"0");
    assert!(result.is_ok(), "expected Ok, got: {:?}", result);
    assert_eq!(result.unwrap(), EnforceState::Permissive);
}

#[test]
fn enforce_state_parse_invalid() {
    let result = SelinuxEnforce::parse(b"x");
    assert!(result.is_err(), "expected Err for invalid enforce byte");
    assert_eq!(result.unwrap_err().kind(), std::io::ErrorKind::InvalidData);
}

// =============================================================================
// SelinuxMls parsing
// =============================================================================

#[test]
fn mls_parse_true() {
    let result = SelinuxMls::parse(b"1");
    assert!(result.is_ok(), "expected Ok, got: {:?}", result);
    assert!(result.unwrap(), "expected MLS enabled");
}

#[test]
fn mls_parse_false() {
    let result = SelinuxMls::parse(b"0");
    assert!(result.is_ok(), "expected Ok, got: {:?}", result);
    assert!(!result.unwrap(), "expected MLS disabled");
}

// =============================================================================
// GenericDualBool parsing
// =============================================================================

#[test]
fn dual_bool_parse_valid() {
    let result = GenericDualBool::parse(b"1 0");
    assert!(result.is_ok(), "expected Ok, got: {:?}", result);
    assert_eq!(
        result.unwrap(),
        DualBool {
            current: true,
            pending: false,
        }
    );
}

#[test]
fn dual_bool_parse_malformed() {
    // Only one token — parse must fail closed
    let result = GenericDualBool::parse(b"1");
    assert!(result.is_err(), "expected Err for single-token dual bool");
    assert_eq!(result.unwrap_err().kind(), std::io::ErrorKind::InvalidData);
}

// =============================================================================
// SelinuxPolicyVers parsing
// =============================================================================

#[test]
fn policy_vers_parse_valid() {
    let result = SelinuxPolicyVers::parse(b"33\n");
    assert!(result.is_ok(), "expected Ok, got: {:?}", result);
    assert_eq!(result.unwrap(), 33u32);
}

// =============================================================================
// AttributeCard Display
// =============================================================================

#[test]
fn attribute_card_display() {
    let card = AttributeCard::<SelinuxEnforce> {
        value: EnforceState::Enforcing,
        path: SelinuxEnforce::PATH,
        read_at: SystemTime::now(),
    };
    let output = format!("{card}");
    assert!(
        output.contains("selinuxfs"),
        "Display output missing kobject name"
    );
    assert!(
        output.contains("enforce"),
        "Display output missing attribute name"
    );
    assert!(
        output.contains("/sys/fs/selinux/enforce"),
        "Display output missing path"
    );
}

// =============================================================================
// KernelLockdown TPI parsing
// =============================================================================

#[test]
fn lockdown_parse_none() {
    // Leading bracket — active mode is "none"
    let result = KernelLockdown::parse(b"[none] integrity confidentiality\n");
    assert!(result.is_ok(), "expected Ok, got: {:?}", result);
    assert_eq!(result.unwrap(), LockdownMode::None);
}

#[test]
fn lockdown_parse_integrity() {
    // Middle bracket — active mode is "integrity"
    let result = KernelLockdown::parse(b"none [integrity] confidentiality\n");
    assert!(result.is_ok(), "expected Ok, got: {:?}", result);
    assert_eq!(result.unwrap(), LockdownMode::Integrity);
}

#[test]
fn lockdown_parse_confidentiality() {
    // Trailing bracket — active mode is "confidentiality"
    let result = KernelLockdown::parse(b"none integrity [confidentiality]\n");
    assert!(result.is_ok(), "expected Ok, got: {:?}", result);
    assert_eq!(result.unwrap(), LockdownMode::Confidentiality);
}

#[test]
fn lockdown_parse_no_bracket_fails_closed() {
    // No bracketed token — path A (nom take_until "[") cannot find "["; fails closed
    let result = KernelLockdown::parse(b"none integrity confidentiality\n");
    assert!(result.is_err(), "expected Err: no bracketed mode");
}

#[test]
fn lockdown_parse_unknown_token_fails_closed() {
    // Bracketed token is not a recognised mode
    let result = KernelLockdown::parse(b"none integrity [hardened]\n");
    assert!(result.is_err(), "expected Err: unrecognised lockdown mode");
}

#[test]
fn lockdown_parse_empty_fails_closed() {
    let result = KernelLockdown::parse(b"");
    assert!(result.is_err(), "expected Err for empty input");
}

#[test]
fn lockdown_parse_unclosed_bracket_fails_closed() {
    // `[` present but no closing `]` — nom take_until("]") fails; fails closed
    let result = KernelLockdown::parse(b"none [integrity\n");
    assert!(result.is_err(), "expected Err: unclosed bracket");
}

#[test]
fn lockdown_mode_ordering() {
    // Discriminants ascend with restrictiveness (None=0, Integrity=1, Confidentiality=2)
    assert!(LockdownMode::None < LockdownMode::Integrity);
    assert!(LockdownMode::Integrity < LockdownMode::Confidentiality);
    assert!(LockdownMode::None < LockdownMode::Confidentiality);
}

#[test]
fn lockdown_mode_display() {
    // Display emits the kernel-canonical lowercase string
    assert_eq!(format!("{}", LockdownMode::None), "none");
    assert_eq!(format!("{}", LockdownMode::Integrity), "integrity");
    assert_eq!(
        format!("{}", LockdownMode::Confidentiality),
        "confidentiality"
    );
}

#[test]
fn lockdown_attribute_card_display() {
    let card = AttributeCard::<KernelLockdown> {
        value: LockdownMode::Integrity,
        path: KernelLockdown::PATH,
        read_at: SystemTime::now(),
    };
    let output = format!("{card}");
    assert!(
        output.contains("securityfs"),
        "Display output missing kobject name: {output}"
    );
    assert!(
        output.contains("lockdown"),
        "Display output missing attribute name: {output}"
    );
    assert!(
        output.contains("/sys/kernel/security/lockdown"),
        "Display output missing path: {output}"
    );
}

// =============================================================================
// ModuleLoadLatch parsing
// =============================================================================

#[test]
fn module_load_latch_parse_true() {
    // b"1" → module loading is permanently disabled
    let result = ModuleLoadLatch::parse(b"1");
    assert!(result.is_ok(), "expected Ok, got: {:?}", result);
    assert!(result.unwrap(), "expected latch=true (modules disabled)");
}

#[test]
fn module_load_latch_parse_false() {
    // b"0" → module loading is currently enabled
    let result = ModuleLoadLatch::parse(b"0");
    assert!(result.is_ok(), "expected Ok, got: {:?}", result);
    assert!(!result.unwrap(), "expected latch=false (modules enabled)");
}

#[test]
fn module_load_latch_parse_invalid() {
    // Any byte other than '0' or '1' must fail closed
    let result = ModuleLoadLatch::parse(b"x");
    assert!(result.is_err(), "expected Err for invalid latch byte");
    assert_eq!(result.unwrap_err().kind(), std::io::ErrorKind::InvalidData);
}

#[test]
fn module_load_latch_parse_empty() {
    // Empty slice — no first byte; must fail closed
    let result = ModuleLoadLatch::parse(b"");
    assert!(result.is_err(), "expected Err for empty latch input");
}
