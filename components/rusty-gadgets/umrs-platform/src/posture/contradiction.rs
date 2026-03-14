// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//! Contradiction classification for kernel security posture signals.
//!
//! A contradiction arises when the live (kernel-effective) value of a signal
//! disagrees with its configured (sysctl.d or cmdline) value. Contradictions
//! indicate configuration management problems that may affect security posture
//! across reboots.
//!
//! ## Contradiction Taxonomy
//!
//! | Scenario | Classification |
//! |---|---|
//! | Live hardened, configured not hardened | `EphemeralHotfix` |
//! | Configured hardened, live not hardened | `BootDrift` |
//! | Live unreadable, configured value present | `SourceUnavailable` |
//! | Both hardened or both not hardened | No contradiction |
//!
//! ## Compliance
//!
//! NIST 800-53 CM-6: Configuration Settings — contradictions between the
//! configured and effective state indicate a configuration management gap.
//! NIST 800-53 CA-7: Continuous Monitoring — contradiction detection is a
//! key output of the posture probe's monitoring function.
//! NIST 800-53 AU-3: Audit Record Content — `ContradictionKind` is a typed
//! enum so audit consumers can programmatically classify findings.

// ===========================================================================
// ContradictionKind
// ===========================================================================

/// Classification of a live-vs-configured value contradiction.
///
/// Used in `SignalReport::contradiction` when both values are available and
/// they disagree. The variant tells the operator what kind of configuration
/// management problem is present.
///
/// NIST 800-53 CM-6: distinguishes between ephemeral hotfixes (live is better
/// than config) and boot drift (config is better than live).
/// NIST 800-53 AU-3: typed enum enables machine-readable audit classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContradictionKind {
    /// Live value is hardened; configured value is not.
    ///
    /// Interpretation: a manual runtime `sysctl` write applied a hardening
    /// setting that is not persisted. The hardening will be lost on reboot.
    /// Action: persist the setting in sysctl.d.
    EphemeralHotfix,

    /// Configured value is hardened; live value is not.
    ///
    /// Interpretation: the sysctl.d configuration says the setting should be
    /// hardened, but the running kernel disagrees. Possible causes: sysctl.d
    /// was not applied at boot, the value was overwritten at runtime, or the
    /// kernel does not support this parameter.
    /// Action: investigate whether sysctl -p was run and check kernel version.
    BootDrift,

    /// The live value could not be read but a configured value exists.
    ///
    /// Interpretation: the kernel node is absent (unsupported feature, missing
    /// kernel module) or a read error occurred. The configured value cannot be
    /// verified against the live state.
    /// Action: check kernel version and module availability.
    SourceUnavailable,
}

impl std::fmt::Display for ContradictionKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EphemeralHotfix => {
                f.write_str("EphemeralHotfix (live hardened, config not)")
            }
            Self::BootDrift => {
                f.write_str("BootDrift (config hardened, live not)")
            }
            Self::SourceUnavailable => {
                f.write_str("SourceUnavailable (live unreadable)")
            }
        }
    }
}

// ===========================================================================
// classify — contradiction detection logic
// ===========================================================================

/// Classify the relationship between a live hardening check and a configured
/// value's hardening check.
///
/// `live_meets` — whether the live value meets the desired value (`Some(true)`
/// = hardened, `Some(false)` = not hardened, `None` = unreadable/unavailable).
///
/// `configured_meets` — whether the configured value (if present) meets the
/// desired value. `None` if no configured value was found.
///
/// Returns `Some(ContradictionKind)` when a contradiction is detected,
/// or `None` when no contradiction exists (both agree, or one is absent).
///
/// NIST 800-53 CM-6: contradiction detection logic.
/// NIST 800-53 CA-7: produces typed findings for continuous monitoring.
#[must_use = "contradiction classification result must be examined"]
pub const fn classify(
    live_meets: Option<bool>,
    configured_meets: Option<bool>,
) -> Option<ContradictionKind> {
    match (live_meets, configured_meets) {
        // Both agree or no configured value — no contradiction.
        (Some(true), Some(true)) | (Some(false), Some(false)) | (_, None) => {
            None
        }
        // Configured exists but live is unreadable.
        (None, Some(_)) => Some(ContradictionKind::SourceUnavailable),
        // Live hardened, configured not → ephemeral hotfix.
        (Some(true), Some(false)) => Some(ContradictionKind::EphemeralHotfix),
        // Live not hardened, configured hardened → boot drift.
        (Some(false), Some(true)) => Some(ContradictionKind::BootDrift),
    }
}

// ===========================================================================
// evaluate_configured_meets — parse configured value against desired
// ===========================================================================

/// Test whether a raw configured string value meets the desired value.
///
/// Returns `Some(true)` if the configured value parses as an integer and
/// satisfies the desired integer constraint. Returns `None` for non-integer
/// desired values (cmdline signals), for unparseable strings, or for
/// `DesiredValue::Custom`.
///
/// NIST 800-53 SI-10: Input Validation — non-parseable configured values
/// produce `None` rather than a silent failure.
#[must_use = "configured value evaluation result must be examined"]
pub fn evaluate_configured_meets(
    raw: &str,
    desired: &crate::posture::signal::DesiredValue,
) -> Option<bool> {
    match raw.trim().parse::<u32>() {
        Ok(v) => desired.meets_integer(v),
        Err(_) => None,
    }
}
