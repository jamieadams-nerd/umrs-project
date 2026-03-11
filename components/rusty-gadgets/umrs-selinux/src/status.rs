// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a, Imodium Operator)
//! SELinux runtime status inspection.
//!
use std::fs::File;
use std::io::{BufRead, BufReader};

use umrs_platform::kattrs::{
    EnforceState, SelinuxEnforce, SelinuxMls, StaticSource,
};

const SELINUX_MAGIC: i64 = 0xf97c_ff8c;

/// The active SELinux policy type, sourced from `/etc/selinux/config`.
///
/// Only populated when SELinux is enabled — the config file is not
/// trustworthy when the kernel reports no active policy.
///
/// NIST SP 800-53 CM-6: security configuration baseline item.
/// CMMC Level 2 — CM.L2-3.4.1: establish baseline configurations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SelinuxPolicy {
    /// `SELINUXTYPE=targeted` — type enforcement only.
    Targeted,
    /// `SELINUXTYPE=mls` — Multi-Level Security with full MLS enforcement.
    Mls,
    /// `SELINUXTYPE=minimum` — reduced targeted policy for constrained systems.
    Minimum,
    /// An unrecognised `SELINUXTYPE=` value, preserved verbatim.
    Unknown(String),
}

impl SelinuxPolicy {
    /// Return the canonical lowercase string for this policy type.
    ///
    /// Matches the value written in `/etc/selinux/config` and used in
    /// policy file paths (e.g., `/etc/selinux/targeted/setrans.conf`).
    #[must_use]
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
    #[must_use]
    pub fn current() -> Self {
        let enabled = is_selinux_enabled();

        let enforcing = if enabled {
            // CALL ON THE STRUCT, NOT THE TRAIT
            SelinuxEnforce::read()
                .map(|state| state == EnforceState::Enforcing)
                .unwrap_or(false)
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

    #[must_use]
    pub const fn enabled(&self) -> bool {
        self.enabled
    }

    #[must_use]
    pub const fn enforcing(&self) -> bool {
        self.enforcing
    }

    #[must_use]
    pub const fn policy(&self) -> Option<&SelinuxPolicy> {
        self.policy.as_ref()
    }
}

// ===========================================================================
// Some legacy "looking" functions
// ===========================================================================
#[must_use]
pub fn is_selinux_enabled() -> bool {
    let path = "/sys/fs/selinux";
    match nix::sys::statfs::statfs(path) {
        Ok(stats) => stats.filesystem_type().0 == SELINUX_MAGIC,
        Err(_) => false,
    }
}

#[must_use]
pub fn is_selinux_mls_enabled() -> bool {
    SelinuxMls::read().unwrap_or(false)
}

//  True  - Enforcing,  False - Permissive
#[must_use]
pub fn security_getenforce() -> bool {
    SelinuxStatus::current().enforcing()
}

/// Return the active SELinux policy type, or `None` if SELinux is not enabled
/// or the config file cannot be read.
///
/// NIST SP 800-53 CM-6: security configuration baseline inspection.
#[must_use]
pub fn selinux_policy() -> Option<SelinuxPolicy> {
    if is_selinux_enabled() {
        read_policy_from_config()
    } else {
        None
    }
}
