// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a, Imodium Operator)
//! # SELinux Runtime Status
//!
//! Queries the live SELinux kernel state and, when SELinux is confirmed active,
//! reads the policy type from `/etc/selinux/config`.
//!
//! ## Trust Gate Pattern
//!
//! Config-file reads are gated behind kernel confirmation. The file
//! `/etc/selinux/config` is only consulted when the kernel reports that
//! SELinux is enabled. If the kernel says SELinux is inactive, `selinux_policy()`
//! returns `None` — the config file cannot be trusted without kernel corroboration.
//!
//! ## Primary Types
//!
//! - `SelinuxStatus` — live kernel state: enable/enforce flags and policy name.
//! - `SelinuxPolicy` — policy type parsed from `SELINUXTYPE=` in the config.
//!
//! ## Primary Functions
//!
//! - `is_selinux_enabled()` — returns `true` if the kernel confirms SELinux is loaded.
//! - `is_selinux_mls_enabled()` — returns `true` if the kernel confirms MLS policy.
//! - `selinux_policy()` — returns the active `SelinuxPolicy` when SELinux is enabled.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 CM-6**: Configuration Settings — status checks gate all
//!   config-file reads; config is never trusted without kernel corroboration.
//! - **NIST SP 800-53 AC-3**: Access Enforcement — enforcement mode must be
//!   confirmed at the kernel level before any access decision is made.
//! - **NSA RTB Trust Gate**: kernel is authoritative; config file is advisory.
//!
use std::fs::File;
use std::io::{BufRead, BufReader};

use umrs_platform::kattrs::{EnforceState, SelinuxEnforce, SelinuxMls, StaticSource};

const SELINUX_MAGIC: i64 = 0xf97c_ff8c;

/// The active SELinux policy type, sourced from `/etc/selinux/config`.
///
/// Only populated when SELinux is enabled — the config file is not
/// trustworthy when the kernel reports no active policy.
///
/// ## Variants:
///
/// - `Targeted` — `SELINUXTYPE=targeted`; type enforcement only.
/// - `Mls` — `SELINUXTYPE=mls`; Multi-Level Security with full MLS enforcement.
/// - `Minimum` — `SELINUXTYPE=minimum`; reduced targeted policy for constrained systems.
/// - `Unknown(String)` — an unrecognised `SELINUXTYPE=` value, preserved verbatim.
///
/// ## Compliance
///
/// - **NIST SP 800-53 CM-6**: security configuration baseline item.
/// - **CMMC Level 2 CM.L2-3.4.1**: establish baseline configurations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SelinuxPolicy {
    Targeted,
    Mls,
    Minimum,
    Unknown(String),
}

impl SelinuxPolicy {
    /// Return the canonical lowercase string for this policy type.
    ///
    /// Matches the value written in `/etc/selinux/config` and used in
    /// policy file paths (e.g., `/etc/selinux/targeted/setrans.conf`).
    #[must_use = "policy type string is used to construct config file paths; discarding it silently bypasses policy routing"]
    pub const fn as_str(&self) -> &str {
        match self {
            Self::Targeted => "targeted",
            Self::Mls => "mls",
            Self::Minimum => "minimum",
            Self::Unknown(s) => s.as_str(),
        }
    }
}

/// Read the `SELINUXTYPE=` field from `/etc/selinux/config`.
///
/// Returns `None` if the file is absent, unreadable, or the field is not
/// present. This function must only be called when SELinux is enabled —
/// the config file reflects boot-time configuration, not the running
/// kernel policy, so it must not be trusted when the kernel is inactive.
///
/// NIST SP 800-53 CM-6: provenance-checked configuration source.
pub(crate) fn read_policy_from_config() -> Option<SelinuxPolicy> {
    let file = File::open("/etc/selinux/config").ok()?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.ok()?;
        let trimmed = line.trim();
        if trimmed.starts_with('#') || trimmed.is_empty() {
            continue;
        }
        if let Some(value) = trimmed.strip_prefix("SELINUXTYPE=") {
            let value = value.trim();
            let policy = match value {
                "targeted" => SelinuxPolicy::Targeted,
                "mls" => SelinuxPolicy::Mls,
                "minimum" => SelinuxPolicy::Minimum,
                other => SelinuxPolicy::Unknown(other.to_owned()),
            };
            return Some(policy);
        }
    }
    None
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelinuxStatus {
    enabled: bool,
    enforcing: bool,
    policy: Option<SelinuxPolicy>,
}

impl SelinuxStatus {
    #[must_use = "SELinux status captures the live kernel enforcement state; ignoring it may lead to unenforced security decisions"]
    pub fn current() -> Self {
        let enabled = is_selinux_enabled();

        let enforcing = if enabled {
            // CALL ON THE STRUCT, NOT THE TRAIT
            SelinuxEnforce::read().map(|state| state == EnforceState::Enforcing).unwrap_or(false)
        } else {
            false
        };

        // Only trust the config file when the kernel confirms SELinux is active.
        let policy = if enabled {
            read_policy_from_config()
        } else {
            None
        };

        Self {
            enabled,
            enforcing,
            policy,
        }
    }

    #[must_use = "pure accessor; ignoring the enabled flag may allow code to proceed as if SELinux is active when it is not"]
    pub const fn enabled(&self) -> bool {
        self.enabled
    }

    #[must_use = "pure accessor; ignoring enforcement mode may allow decisions to proceed without MAC enforcement"]
    pub const fn enforcing(&self) -> bool {
        self.enforcing
    }

    #[must_use = "pure accessor; callers that discard the policy type cannot route config reads correctly"]
    pub const fn policy(&self) -> Option<&SelinuxPolicy> {
        self.policy.as_ref()
    }
}

// ===========================================================================
// Some legacy "looking" functions
// ===========================================================================
#[must_use = "security gate: ignoring whether SELinux is enabled may allow bypass of MAC enforcement checks"]
pub fn is_selinux_enabled() -> bool {
    let path = "/sys/fs/selinux";
    match nix::sys::statfs::statfs(path) {
        Ok(stats) => stats.filesystem_type().0 == SELINUX_MAGIC,
        Err(_) => false,
    }
}

#[must_use = "security gate: ignoring MLS mode may lead to incorrect lattice dominance assumptions in access decisions"]
pub fn is_selinux_mls_enabled() -> bool {
    SelinuxMls::read().unwrap_or(false)
}

//  True  - Enforcing,  False - Permissive
#[must_use = "security gate: discarding the enforce/permissive result may allow operations to proceed without verifying MAC is active"]
pub fn security_getenforce() -> bool {
    SelinuxStatus::current().enforcing()
}

/// Return the active SELinux policy type, or `None` if SELinux is not enabled
/// or the config file cannot be read.
///
/// NIST SP 800-53 CM-6: security configuration baseline inspection.
#[must_use = "policy type gates config-file routing; discarding it means all subsequent config reads are unanchored"]
pub fn selinux_policy() -> Option<SelinuxPolicy> {
    if is_selinux_enabled() {
        read_policy_from_config()
    } else {
        None
    }
}
