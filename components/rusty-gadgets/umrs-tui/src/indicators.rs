// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams

//! # Indicators — Live Kernel Security Indicator Reader
//!
//! Provides [`read_security_indicators`], which queries kernel attribute
//! nodes via the `umrs-platform` `SecureReader` engine and returns a
//! [`SecurityIndicators`] snapshot for display in the audit card header.
//!
//! ## Fail-Closed Contract
//!
//! Every read operation wraps its result: success maps to `Active` or
//! `Inactive`; any I/O error or unimplemented source maps to `Unavailable`.
//! The caller is never handed a guess — degraded state is explicit.
//!
//! ## Trust Boundary
//!
//! Values originate from provenance-verified kernel pseudo-filesystem reads
//! (`SecureReader` engine with fd-anchored `fstatfs` magic checks). They are
//! display-only in the header — they are not enforcement inputs.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 SI-7**: Software and Information Integrity — all reads
//!   are provenance-verified via `SecureReader` before any bytes are parsed.
//! - **NIST SP 800-53 CM-6**: Configuration Settings — live kernel state is
//!   read directly; no static configuration file is trusted without a kernel
//!   gate (Trust Gate pattern).
//! - **NSA RTB RAIN**: Non-Bypassability — no raw `File::open` is used;
//!   all reads route through the `StaticSource::read()` path.

use umrs_platform::kattrs::{
    EnforceState, KernelLockdown, LockdownMode, ProcFips, SelinuxEnforce,
    StaticSource,
};

use crate::app::{IndicatorValue, SecurityIndicators};

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

/// Read all live kernel security indicators and return a snapshot.
///
/// Each field is populated from the corresponding kernel attribute node via
/// `SecureReader`. On any read error the field is set to
/// `IndicatorValue::Unavailable` and `log::warn!` is emitted — the caller
/// receives an explicit degraded-state marker, never a silent success.
///
/// Sources:
/// - `selinux_status` — `/sys/fs/selinux/enforce` (`SelinuxEnforce`)
/// - `fips_mode`      — `/proc/sys/crypto/fips_enabled` (`ProcFips`)
/// - `active_lsm`     — TODO: no kattr type exists yet; always `Unavailable`
/// - `lockdown_mode`  — `/sys/kernel/security/lockdown` (`KernelLockdown`)
/// - `secure_boot`    — TODO: platform-specific; always `Unavailable`
///
/// NIST SP 800-53 SI-7 / NSA RTB RAIN — provenance-verified reads only.
///
/// # Returns
///
/// A fully-populated `SecurityIndicators` where every field is one of
/// `Active`, `Inactive`, or `Unavailable`. Never panics.
#[must_use = "security indicators must be consumed by the render path; discarding them \
              hides kernel security state from the audit card"]
pub fn read_security_indicators() -> SecurityIndicators {
    #[cfg(debug_assertions)]
    let start = std::time::Instant::now();

    let selinux_status = read_selinux_status();
    let fips_mode = read_fips_mode();

    // TODO: No kattr type exists for /sys/kernel/security/lsm yet.
    // The path is under securityfs (not sysfs), and requires a dedicated
    // StaticSource implementation with SECURITYFS_MAGIC. Return Unavailable
    // until that type is added to umrs-platform.
    let active_lsm = IndicatorValue::Unavailable;

    let lockdown_mode = read_lockdown_mode();

    // TODO: Secure Boot state is platform-specific. On UEFI systems it is
    // readable from /sys/firmware/efi/efivars/SecureBoot-*.  A dedicated
    // sysfs kattr type is needed. Return Unavailable until implemented.
    let secure_boot = IndicatorValue::Unavailable;

    #[cfg(debug_assertions)]
    log::debug!(
        "Pattern: read_security_indicators completed in {} µs",
        start.elapsed().as_micros()
    );

    SecurityIndicators {
        selinux_status,
        fips_mode,
        active_lsm,
        lockdown_mode,
        secure_boot,
    }
}

// ---------------------------------------------------------------------------
// Private per-source readers
// ---------------------------------------------------------------------------

/// Read SELinux enforcement mode from `/sys/fs/selinux/enforce`.
///
/// Returns `Active("enforcing")`, `Inactive("permissive")`, or
/// `Unavailable` on read failure (selinux not mounted, kernel error).
fn read_selinux_status() -> IndicatorValue {
    match SelinuxEnforce::read() {
        Ok(EnforceState::Enforcing) => {
            IndicatorValue::Active("enforcing".to_owned())
        }
        Ok(EnforceState::Permissive) => {
            IndicatorValue::Inactive("permissive".to_owned())
        }
        Err(e) => {
            log::warn!("indicators: selinux_status read failed: {e}");
            IndicatorValue::Unavailable
        }
    }
}

/// Read FIPS mode from `/proc/sys/crypto/fips_enabled`.
///
/// Returns `Active("active")`, `Inactive("inactive")`, or `Unavailable`
/// on read failure.
fn read_fips_mode() -> IndicatorValue {
    match ProcFips::read() {
        Ok(true) => IndicatorValue::Active("active".to_owned()),
        Ok(false) => IndicatorValue::Inactive("inactive".to_owned()),
        Err(e) => {
            log::warn!("indicators: fips_mode read failed: {e}");
            IndicatorValue::Unavailable
        }
    }
}

/// Read kernel lockdown level from `/sys/kernel/security/lockdown`.
///
/// Returns `Active("<level>")` for non-None levels, `Inactive("none")`, or
/// `Unavailable` on read failure.
fn read_lockdown_mode() -> IndicatorValue {
    match KernelLockdown::read() {
        Ok(LockdownMode::None) => IndicatorValue::Inactive("none".to_owned()),
        Ok(mode) => IndicatorValue::Active(mode.to_string()),
        Err(e) => {
            log::warn!("indicators: lockdown_mode read failed: {e}");
            IndicatorValue::Unavailable
        }
    }
}
