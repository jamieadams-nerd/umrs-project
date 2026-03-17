// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//! Contradiction classification for kernel security posture indicators.
//!
//! A contradiction arises when the live (kernel-effective) value of a indicator
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
//! ## Blacklist Sentinel Semantics
//!
//! For modprobe.d blacklist indicators, `ConfiguredValue::raw` is set to the
//! sentinel string `"blacklisted"` when a `blacklist <module>` entry exists
//! in modprobe.d. This string cannot be parsed as a `u32`, so
//! `evaluate_configured_meets` handles it explicitly:
//!
//! - `"blacklisted"` → the module is explicitly blacklisted in modprobe.d.
//!   The desired value for blacklist indicators is `Exact(1)` ("blacklist
//!   effective"). A configured blacklist entry meets the desired value
//!   (`configured_meets = Some(true)`).
//! - Any other non-integer string → `None` (no configured value for
//!   contradiction purposes).
//!
//! The FIPS cross-check configured value is a human-readable audit summary
//! string (e.g., `"marker=present cmdline=fips=1"`). This always returns
//! `None` from `evaluate_configured_meets` — the FIPS path does not
//! participate in `classify()` by construction.
//!
//! ## Compliance
//!
//! NIST SP 800-53 CM-6: Configuration Settings — contradictions between the
//! configured and effective state indicate a configuration management gap.
//! NIST SP 800-53 CA-7: Continuous Monitoring — contradiction detection is a
//! key output of the posture probe's monitoring function.
//! NIST SP 800-53 AU-3: Audit Record Content — `ContradictionKind` is a typed
//! enum so audit consumers can programmatically classify findings.

// ===========================================================================
// ContradictionKind
// ===========================================================================

/// Classification of a live-vs-configured value contradiction.
///
/// Used in `IndicatorReport::contradiction` when both values are available and
/// they disagree. The variant tells the operator what kind of configuration
/// management problem is present.
///
/// NIST SP 800-53 CM-6: distinguishes between ephemeral hotfixes (live is better
/// than config) and boot drift (config is better than live).
/// NIST SP 800-53 AU-3: typed enum enables machine-readable audit classification.
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
/// NIST SP 800-53 CM-6: contradiction detection logic.
/// NIST SP 800-53 CA-7: produces typed findings for continuous monitoring.
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
/// desired values (cmdline indicators), for unparseable strings, or for
/// `DesiredValue::Custom`.
///
/// ## Blacklist Sentinel
///
/// The sentinel string `"blacklisted"` is handled as a special case for
/// modprobe.d blacklist indicators. Blacklist indicators use `DesiredValue::Exact(1)`;
/// a configured `"blacklisted"` entry represents the equivalent of integer `1`
/// (blacklist effective = hardened). This produces `Some(true)` when the desired
/// value is `Exact(1)`, enabling contradiction detection for the critical case
/// where a module is blacklisted in modprobe.d but loaded at runtime
/// (`live_meets = Some(false)`, `configured_meets = Some(true)` → `BootDrift`).
///
/// ## FIPS Cross-Check
///
/// The FIPS configured value is a human-readable audit summary string (e.g.,
/// `"marker=present cmdline=fips=1"`). This always returns `None` from this
/// function — the FIPS path does not participate in `classify()` by construction.
///
/// ## KernelCmdline (BLS Options String)
///
/// For `KernelCmdline`-class indicators, the configured raw value is the full BLS
/// options string (e.g., `"root=UUID=abc fips=1 module.sig_enforce=1"`). This
/// function returns `None` for such values — token-based evaluation for these
/// indicators is handled via a dedicated path in `collect_one()` (see
/// `snapshot.rs`). `DesiredValue::meets_cmdline()` is called directly on the
/// BLS options string rather than routing through this function. This design
/// is intentional: the BLS options string is not an integer and cannot be
/// evaluated here without knowing which token to search for.
///
/// NIST SP 800-53 SI-10: Input Validation — non-parseable configured values
/// produce `None` rather than a silent failure. The `"blacklisted"` sentinel
/// is an explicitly recognised non-integer value, not an error.
/// NIST SP 800-53 CM-6: Configuration Settings — blacklist contradiction
/// detection requires the sentinel to participate in `classify()`.
/// NIST SP 800-53 AU-3: Security Findings as Data — `BootDrift` must be
/// producible for blacklist indicators; suppressing it silently is a defect.
#[must_use = "configured value evaluation result must be examined"]
pub fn evaluate_configured_meets(
    raw: &str,
    desired: &crate::posture::indicator::DesiredValue,
) -> Option<bool> {
    // Blacklist sentinel: a modprobe.d `blacklist <module>` entry sets raw to
    // "blacklisted". The desired value for blacklist indicators is Exact(1) —
    // meaning "blacklist effective" (module not loaded = hardened).
    // Treat "blacklisted" as the integer 1 for the purposes of meets_integer.
    // This allows classify() to emit BootDrift when the module is loaded
    // despite a modprobe.d blacklist entry.
    if raw.trim() == "blacklisted" {
        return desired.meets_integer(1);
    }

    match raw.trim().parse::<u32>() {
        Ok(v) => desired.meets_integer(v),
        Err(_) => {
            // Fall back to signed parse for sysctl nodes that legitimately
            // store negative configured values (e.g., `kernel.perf_event_paranoid = -1`
            // means "unrestricted for all users"). Without this path, a sysctl.d file
            // with `perf_event_paranoid = -1` produces `None`, suppressing
            // `EphemeralHotfix` detection when the live value was hotfixed to 2.
            //
            // The signed path routes through `meets_signed_integer`, which compares
            // in `i64` to avoid overflow, and correctly returns `Some(false)` for
            // a negative value against `AtLeast(2)`.
            //
            // NIST SP 800-53 CA-7: must not silently suppress EphemeralHotfix
            // when the configured and live values legitimately disagree.
            raw.trim()
                .parse::<i32>()
                .ok()
                .and_then(|v| desired.meets_signed_integer(v))
        }
    }
}
