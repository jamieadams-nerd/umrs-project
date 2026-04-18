// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a, Imodium Operator)
//! # Kernel Attribute Value Types — Shared Domain Types
//!
//! Shared domain value types used across multiple kernel attribute structs.
//!
//! NIST SP 800-53 AC-3: Access Enforcement — `EnforceState` is the canonical
//! representation of the kernel LSM enforcement posture.

/// SELinux enforcement mode — the active security posture of the kernel LSM.
///
/// ## Compliance
///
/// - NIST SP 800-53 AC-3: Access Enforcement — this value directly determines
///   whether the kernel enforces or merely audits policy decisions.
/// - NSA RTB RAIN: the enforcement state is non-bypassable when read via `SecureReader`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnforceState {
    Permissive = 0,
    Enforcing = 1,
}

/// Dual-valued kernel boolean attribute (current value / pending value after commit).
///
/// Used for SELinux booleans where the kernel tracks a live state and an
/// uncommitted pending state separately. SELinux boolean nodes expose both the
/// current active value and a pending value that takes effect when
/// `commit_pending_bools` is written to selinuxfs.
///
/// ## Compliance
///
/// - NIST SP 800-53 AC-3: Access Enforcement — the dual boolean represents the
///   committed (current) and staged (pending) values of a kernel policy decision;
///   both must be examined to determine the full enforcement picture.
/// - NIST SP 800-53 AC-6: Least Privilege — the pending state must be checked to
///   detect uncommitted privilege changes that have not yet taken effect.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DualBool {
    pub current: bool,
    pub pending: bool,
}
