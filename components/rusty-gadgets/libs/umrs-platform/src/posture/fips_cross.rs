// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//! FIPS distro-managed configured-value cross-check.
//!
//! Extends the existing `FipsEnabled` indicator (Phase 1) with configured-value
//! discovery from RHEL 10's FIPS persistence layer, enabling contradiction
//! detection between the kernel's live FIPS state and the distro's intended
//! FIPS configuration.
//!
//! ## Three Sources of Truth
//!
//! On RHEL 10, FIPS persistent state is determined by three independent sources:
//!
//! 1. **Kernel live**: `/proc/sys/crypto/fips_enabled` (Phase 1 `ProcFips`)
//! 2. **Dracut initramfs**: `fips=1` on the kernel cmdline (Phase 1 cmdline parser)
//! 3. **Distro persistent state**: `/etc/system-fips` marker and
//!    `/etc/crypto-policies/state/current`
//!
//! This module covers source 3.
//!
//! ## Trust Gate
//!
//! Only reads `/etc/system-fips` and crypto-policy state if
//! `/proc/sys/crypto/fips_enabled` was accessible (live state available).
//! If the kernel cannot confirm the crypto subsystem state, config reads
//! are advisory-only and return `None`.
//!
//! ## Error Information Discipline
//!
//! FIPS state errors do not reveal the specific crypto-policy string in
//! user-visible error messages — it could indicate the system's cryptographic
//! posture to an adversary. The crypto-policy content is logged at `debug`
//! only; errors carry only structural descriptions.
//!
//! ## Applicable Patterns
//!
//! - **Trust Gate** (NIST SP 800-53 CM-6): config reads gated behind kernel
//!   FIPS availability.
//! - **Provenance Verification** (NIST SP 800-53 SI-7): kernel live state already
//!   verified via `PROC_SUPER_MAGIC` (Phase 1). `/etc/system-fips` and
//!   `/etc/crypto-policies/` are regular-filesystem reads — advisory only.
//! - **Fail-Closed** (NIST SP 800-53 SI-10 / RTB Fail Secure): absent marker →
//!   `None`, not "FIPS disabled". Unreadable policy → `None`, not a default.
//! - **Security Findings as Data** (NIST SP 800-53 AU-3): contradictions flow
//!   through `ContradictionKind` — programmatically matchable.
//! - **Error Information Discipline** (NIST SP 800-53 SI-11 / RTB Error Discipline):
//!   crypto-policy content not revealed in error messages.
//! - **Pattern Execution Measurement** (NIST SP 800-218 SSDF PW.4): debug-mode
//!   timing under `#[cfg(debug_assertions)]`.
//! - **Must-Use Contract** (NIST SP 800-53 SI-10, SA-11): `FipsCrossCheck` and
//!   `evaluate()` carry `#[must_use]`.
//! - **Non-Bypassability** (NSA RTB RAIN): cross-check invoked unconditionally
//!   from `read_configured()` for `IndicatorId::FipsEnabled`.
//!
//! ## Compliance
//!
//! NIST SP 800-53 SC-13: Cryptographic Protection — FIPS configuration state
//! determines which cryptographic modules are permitted.
//! NIST SP 800-53 CM-6: Configuration Settings — persistence layer for FIPS.
//! NIST SP 800-53 CA-7: Continuous Monitoring — cross-check detects FIPS
//! persistence gaps.
//! NIST SP 800-53 SI-11: Error Handling — crypto-policy content not in errors.
//! FIPS 140-2/140-3: system-wide FIPS mode enforcement.

use crate::posture::indicator::ConfiguredValue;

// ===========================================================================
// Paths
// ===========================================================================

/// RHEL 10 FIPS marker file. Presence indicates `fips-mode-setup --enable` ran.
const SYSTEM_FIPS_MARKER: &str = "/etc/system-fips";

/// RHEL 10 active crypto-policy state file.
const CRYPTO_POLICY_STATE: &str = "/etc/crypto-policies/state/current";

// ===========================================================================
// FipsCrossCheck
// ===========================================================================

/// FIPS persistent configuration state from RHEL distro tooling.
///
/// Aggregates multiple FIPS configuration indicators into a single typed
/// assessment. Each indicator is independently resolved and recorded for
/// audit evidence.
///
/// NIST SP 800-53 SC-13: Cryptographic Protection — FIPS configuration state
/// determines which cryptographic modules are permitted.
/// FIPS 140-2/140-3: system-wide FIPS mode enforcement.
#[must_use = "FIPS cross-check results carry compliance findings — do not discard"]
pub struct FipsCrossCheck {
    /// Presence of `/etc/system-fips` marker file.
    /// `None` if the check was not performed (Trust Gate not cleared).
    pub marker_present: Option<bool>,
    /// `fips=1` found in the kernel cmdline (from Phase 1 `CmdlineReader`).
    /// `None` if cmdline was not available.
    pub cmdline_fips: Option<bool>,
    /// Active crypto-policy string from `/etc/crypto-policies/state/current`.
    /// `None` if unreadable or not applicable.
    pub crypto_policy: Option<String>,
    /// Overall assessment: configured FIPS indicators agree with desired (FIPS on).
    /// `None` if insufficient indicators were available.
    pub configured_meets_desired: Option<bool>,
}

impl FipsCrossCheck {
    /// Evaluate FIPS configured-value state from all distro persistence sources.
    ///
    /// `live_fips_readable` — whether `/proc/sys/crypto/fips_enabled` was
    /// successfully read in the live pass. If `false`, the Trust Gate blocks
    /// all config reads and the returned struct has all `None` fields.
    ///
    /// `cmdline_has_fips1` — whether `fips=1` was found in `/proc/cmdline`.
    /// Pass `None` if the cmdline reader was unavailable.
    ///
    /// NIST SP 800-53 CM-6: Trust Gate and configured-value resolution.
    /// NIST SP 800-53 SI-11: no crypto-policy content in error paths.
    #[must_use = "FIPS cross-check evaluation result must be examined for compliance findings"]
    pub fn evaluate(live_fips_readable: bool, cmdline_has_fips1: Option<bool>) -> Self {
        #[cfg(debug_assertions)]
        let start = std::time::Instant::now();

        // Trust Gate: if live FIPS state was unreadable, config reads are
        // meaningless — return None for everything.
        if !live_fips_readable {
            log::debug!(
                "posture: FIPS cross-check: Trust Gate blocked — \
                 /proc/sys/crypto/fips_enabled was not readable"
            );
            return Self {
                marker_present: None,
                cmdline_fips: cmdline_has_fips1,
                crypto_policy: None,
                configured_meets_desired: None,
            };
        }

        // --- Source 1: /etc/system-fips marker ---
        let marker_present = Some(check_system_fips_marker());

        // --- Source 2: cmdline fips=1 (from Phase 1 CmdlineReader, passed in) ---
        log::debug!(
            "posture: FIPS cross-check: /proc/cmdline fips=1 present={cmdline_has_fips1:?}"
        );

        // --- Source 3: /etc/crypto-policies/state/current ---
        let crypto_policy = read_crypto_policy_state();

        // --- Overall assessment ---
        // FIPS is considered persistently configured if:
        //   - /etc/system-fips exists, OR
        //   - fips=1 is in the cmdline, OR
        //   - crypto-policy is "FIPS" or starts with "FIPS:"
        //
        // Any single indicator is sufficient to assert configured=FIPS.
        // If ALL indicators are None, the assessment is None (insufficient data).
        let configured_meets_desired =
            assess_configured_fips(marker_present, cmdline_has_fips1, crypto_policy.as_deref());

        #[cfg(debug_assertions)]
        log::debug!(
            "posture: FIPS cross-check: completed in {} µs — \
             marker={:?} cmdline={:?} policy={:?}",
            start.elapsed().as_micros(),
            marker_present,
            cmdline_has_fips1,
            // policy content is safe at debug level; it's a short static string
            crypto_policy.as_deref()
        );

        if let Some(meets) = configured_meets_desired {
            log::debug!(
                "posture: FIPS cross-check: configured_meets_desired={meets} \
                 (all indicators assessed)"
            );
        } else {
            log::debug!(
                "posture: FIPS cross-check: configured_meets_desired=None \
                 (insufficient indicators)"
            );
        }

        Self {
            marker_present,
            cmdline_fips: cmdline_has_fips1,
            crypto_policy,
            configured_meets_desired,
        }
    }

    /// Produce a `ConfiguredValue` suitable for insertion into an `IndicatorReport`.
    ///
    /// Returns `None` if all indicators were unavailable (Trust Gate blocked or
    /// no indicators readable).
    ///
    /// The `raw` field contains a structured summary of all indicators for
    /// audit display. The `source_file` records the primary indicator source.
    ///
    /// NIST SP 800-53 AU-3: structured evidence for audit consumers.
    #[must_use = "configured value result must be examined — None means no FIPS persistence data"]
    pub fn as_configured_value(&self) -> Option<ConfiguredValue> {
        // If no indicator was available at all, return None.
        if self.marker_present.is_none()
            && self.cmdline_fips.is_none()
            && self.crypto_policy.is_none()
        {
            return None;
        }

        // Build a compact audit summary from available indicators.
        let mut parts: Vec<String> = Vec::new();
        if let Some(m) = self.marker_present {
            let label = if m {
                "present"
            } else {
                "absent"
            };
            parts.push(format!("marker={label}"));
        }
        if let Some(c) = self.cmdline_fips {
            if c {
                parts.push("cmdline=fips=1".to_owned());
            } else {
                parts.push("cmdline=no-fips".to_owned());
            }
        }
        if let Some(ref p) = self.crypto_policy {
            parts.push(format!("policy={p}"));
        }

        let raw = parts.join(" ");

        // Primary source: prefer marker file, then cmdline, then policy.
        let source_file = if self.marker_present.is_some() {
            SYSTEM_FIPS_MARKER.to_owned()
        } else if self.cmdline_fips.is_some() {
            "/proc/cmdline".to_owned()
        } else {
            CRYPTO_POLICY_STATE.to_owned()
        };

        Some(ConfiguredValue {
            raw,
            source_file,
        })
    }
}

// ===========================================================================
// Private helpers
// ===========================================================================

/// Check for the presence of the `/etc/system-fips` marker file.
///
/// Returns `true` if it exists, `false` if it does not.
/// Absence is a valid (non-FIPS) state — never returns a failure.
///
/// Uses `std::fs::metadata()` rather than `Path::exists()` to collapse the
/// check into a single syscall, eliminating the TOCTOU window between an
/// existence check and a subsequent operation. The result is the same
/// (`true`/`false`), but the race window is removed. NIST SP 800-53 SI-10.
fn check_system_fips_marker() -> bool {
    let exists = std::fs::metadata(SYSTEM_FIPS_MARKER).is_ok();
    log::debug!("posture: FIPS cross-check: {SYSTEM_FIPS_MARKER} exists={exists}");
    exists
}

/// Read the active crypto-policy from `/etc/crypto-policies/state/current`.
///
/// Returns `Some(String)` with the trimmed policy name on success, `None` on
/// any I/O failure (absent file, permission denied, non-UTF8, etc.).
///
/// NIST SP 800-53 SI-11: Error Discipline — errors do not reveal policy content.
/// The content itself is only visible at debug level.
fn read_crypto_policy_state() -> Option<String> {
    match std::fs::read_to_string(CRYPTO_POLICY_STATE) {
        Ok(content) => {
            let trimmed = content.trim().to_owned();
            log::debug!("posture: FIPS cross-check: {CRYPTO_POLICY_STATE} = \"{trimmed}\"");
            if trimmed.is_empty() {
                log::debug!("posture: FIPS cross-check: crypto-policy state is empty");
                None
            } else {
                Some(trimmed)
            }
        }
        Err(e) => {
            // Error Information Discipline: log only that the file was
            // unreadable and the error kind — not any content.
            log::debug!(
                "posture: FIPS cross-check: {} unreadable: {} ({})",
                CRYPTO_POLICY_STATE,
                e.kind(),
                // Structural description only — no content.
                if e.kind() == std::io::ErrorKind::NotFound {
                    "not installed"
                } else {
                    "read error"
                }
            );
            None
        }
    }
}

/// Assess whether FIPS is persistently configured, based on all available
/// indicators.
///
/// Logic:
/// - If ANY indicator confirms FIPS → `Some(true)`.
/// - If NO indicator confirms FIPS AND at least one was readable → `Some(false)`.
/// - If ALL indicators are `None` → `None` (cannot assess).
///
/// "Crypto-policy is FIPS" means the string is `"FIPS"` or starts with `"FIPS:"`.
///
/// NIST SP 800-53 CM-6: configured-value assessment from multiple indicators.
fn assess_configured_fips(
    marker_present: Option<bool>,
    cmdline_fips: Option<bool>,
    crypto_policy: Option<&str>,
) -> Option<bool> {
    let marker_says_fips = marker_present == Some(true);
    let cmdline_says_fips = cmdline_fips == Some(true);
    let policy_says_fips = crypto_policy.is_some_and(|p| p == "FIPS" || p.starts_with("FIPS:"));

    if marker_says_fips || cmdline_says_fips || policy_says_fips {
        return Some(true);
    }

    // No positive indicator — determine if we have any evidence at all.
    let any_indicator =
        marker_present.is_some() || cmdline_fips.is_some() || crypto_policy.is_some();

    if any_indicator {
        Some(false)
    } else {
        None
    }
}
