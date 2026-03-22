// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams

//! # Indicators — Live Kernel Security Indicator and Header Context Reader
//!
//! Provides public functions:
//!
//! - [`read_security_indicators`] — queries kernel attribute nodes and returns
//!   a [`SecurityIndicators`] snapshot for the audit card header indicator rows.
//! - [`build_header_context`] — reads all system-identification fields (boot ID,
//!   kernel version, architecture, hostname, assessment timestamp) alongside the
//!   security indicators, returning a [`HeaderContext`] snapshot.
//! - [`read_system_uuid`] — reads the DMI system UUID from sysfs for display
//!   in the Kernel Security tab (requires root on most kernels).
//!
//! ## Fail-Closed Contract
//!
//! Every read operation wraps its result: success maps to `Enabled` or
//! `Disabled`; any I/O error or unimplemented source maps to `Unavailable`.
//! System-identification reads (boot ID, system UUID) fall back to the string
//! `"unavailable"` on failure — never to a fabricated or guessed value.
//!
//! ## Trust Boundary
//!
//! Values from kernel attribute nodes originate from provenance-verified reads
//! (`SecureReader` engine with fd-anchored `fstatfs` magic checks). They are
//! display-only in the header — they are not enforcement inputs or policy decisions.
//!
//! Hostname and kernel version come from `uname(2)` — display-only, not
//! trust-relevant assertions.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 SI-7**: Software and Information Integrity — all reads
//!   are provenance-verified via `SecureReader` before any bytes are parsed.
//! - **NIST SP 800-53 CM-6**: Configuration Settings — live kernel state is
//!   read directly; no static configuration file is trusted without a kernel
//!   gate (Trust Gate pattern).
//! - **NIST SP 800-53 CA-7**: Continuous Monitoring — `assessed_at` timestamps
//!   each collection event so records are datable.
//! - **NIST SP 800-53 SA-11**: Developer Testing — tool name and version provide
//!   traceability to the specific tool build that collected evidence.
//! - **NSA RTB RAIN**: Non-Bypassability — no raw `File::open` is used on
//!   `/proc/` or `/sys/`; all reads route through `ProcfsText`/`SysfsText`.

use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use umrs_platform::kattrs::{
    EnforceState, KernelLockdown, LockdownMode, ProcFips, ProcfsText,
    SecureReader, SelinuxEnforce, StaticSource, SysfsText,
};
use umrs_selinux::status::selinux_policy;

use crate::app::{HeaderContext, IndicatorValue, SecurityIndicators};

// ---------------------------------------------------------------------------
// Public entry point — security indicators only
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

    // No kattr type exists for /sys/kernel/security/lsm yet.
    // The path is under securityfs (not sysfs), and requires a dedicated
    // StaticSource implementation with SECURITYFS_MAGIC. Return Unavailable
    // until that type is added to umrs-platform.
    let active_lsm = IndicatorValue::Unavailable;

    let lockdown_mode = read_lockdown_mode();

    // Secure Boot state is platform-specific. On UEFI systems it is readable
    // from /sys/firmware/efi/efivars/SecureBoot-* but the path includes a
    // GUID suffix that must be determined at runtime via directory enumeration.
    // This cannot be expressed as a compile-time StaticSource path. A dedicated
    // probe is required; return Unavailable until implemented.
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
// Public entry point — full header context
// ---------------------------------------------------------------------------

/// Build a complete `HeaderContext` snapshot for the audit card header.
///
/// Reads all live security indicators (via [`read_security_indicators`])
/// plus system-identification fields: hostname, kernel version, architecture,
/// boot ID, and a formatted assessment timestamp.
///
/// `tool_name` and `tool_version` are supplied by the calling binary —
/// typically `env!("CARGO_PKG_NAME")` and `env!("CARGO_PKG_VERSION")`.
/// These are compile-time values; passing them from the caller avoids making
/// this library function depend on a specific binary's package metadata.
///
/// `os_name` is supplied by the calling binary. Binaries that run the OS
/// detection pipeline should pass `PRETTY_NAME` (or `NAME VERSION_ID`) from
/// the detected `OsRelease`. Binaries without OS detection should pass
/// `"unavailable"`.
///
/// All reads are fail-closed: if a field cannot be read, it is set to
/// `"unavailable"` — never to a guessed or fabricated value.
///
/// ## Pattern: Pattern Execution Measurement
///
/// In debug builds, the total build time is logged at `debug` level with the
/// pattern name and duration in microseconds.
///
/// NIST SP 800-53 AU-3 — the returned context carries sufficient identification
/// for every rendered card to serve as a standalone SP 800-53A Examine object.
/// NIST SP 800-53 CA-7 — `assessed_at` timestamps the collection event.
/// NIST SP 800-53 SA-11 — `tool_name` and `tool_version` provide traceability.
#[must_use = "HeaderContext must be passed to render_audit_card; discarding it hides \
              system identification and security posture from the audit card"]
pub fn build_header_context(
    tool_name: impl Into<String>,
    tool_version: impl Into<String>,
    os_name: impl Into<String>,
) -> HeaderContext {
    #[cfg(debug_assertions)]
    let start = std::time::Instant::now();

    let indicators = read_security_indicators();
    let assessed_at = format_assessed_at();
    let (hostname, kernel_version, architecture) = read_uname_fields();
    let boot_id = read_boot_id();
    let system_uuid = read_system_uuid();

    #[cfg(debug_assertions)]
    log::debug!(
        "Pattern: build_header_context completed in {} µs",
        start.elapsed().as_micros()
    );

    HeaderContext {
        indicators,
        tool_name: tool_name.into(),
        tool_version: tool_version.into(),
        assessed_at,
        hostname,
        kernel_version,
        architecture,
        boot_id,
        system_uuid,
        os_name: os_name.into(),
    }
}

// ---------------------------------------------------------------------------
// Security indicator readers (private)
// ---------------------------------------------------------------------------

/// Read SELinux enforcement mode from `/sys/fs/selinux/enforce`.
///
/// Returns `Enabled("Enforcing (Targeted)")`, `Disabled("Permissive (Targeted)")`,
/// or `Unavailable` on read failure (SELinux not mounted, kernel error).
///
/// When SELinux is active, the policy type is read from `/etc/selinux/config`
/// via `selinux_policy()` — gated behind kernel confirmation (Trust Gate pattern,
/// NIST SP 800-53 CM-6). If the policy type cannot be read, the mode string
/// is shown without a parenthetical (e.g., `"Enforcing"`).
fn read_selinux_status() -> IndicatorValue {
    // Capitalise the policy name for display (e.g., "targeted" → "Targeted").
    fn policy_label() -> Option<String> {
        let p = selinux_policy()?;
        let raw = p.as_str();
        let mut chars = raw.chars();
        let first = chars.next()?.to_uppercase().to_string();
        Some(format!("{first}{}", chars.as_str()))
    }

    match SelinuxEnforce::read() {
        Ok(EnforceState::Enforcing) => {
            let label = if let Some(pol) = policy_label() {
                format!("Enforcing ({pol})")
            } else {
                "Enforcing".to_owned()
            };
            IndicatorValue::Enabled(label)
        }
        Ok(EnforceState::Permissive) => {
            let label = if let Some(pol) = policy_label() {
                format!("Permissive ({pol})")
            } else {
                "Permissive".to_owned()
            };
            IndicatorValue::Disabled(label)
        }
        Err(e) => {
            log::warn!("indicators: selinux_status read failed: {e}");
            IndicatorValue::Unavailable
        }
    }
}

/// Read FIPS mode from `/proc/sys/crypto/fips_enabled`.
///
/// Returns `Enabled("Enabled")`, `Disabled("Disabled")`, or `Unavailable`
/// on read failure.
fn read_fips_mode() -> IndicatorValue {
    match ProcFips::read() {
        Ok(true) => IndicatorValue::Enabled("Enabled".to_owned()),
        Ok(false) => IndicatorValue::Disabled("Disabled".to_owned()),
        Err(e) => {
            log::warn!("indicators: fips_mode read failed: {e}");
            IndicatorValue::Unavailable
        }
    }
}

/// Read kernel lockdown level from `/sys/kernel/security/lockdown`.
///
/// Returns `Enabled("<level>")` for non-None levels, `Disabled("none")`, or
/// `Unavailable` on read failure.
fn read_lockdown_mode() -> IndicatorValue {
    match KernelLockdown::read() {
        Ok(LockdownMode::None) => IndicatorValue::Disabled("none".to_owned()),
        Ok(mode) => IndicatorValue::Enabled(mode.to_string()),
        Err(e) => {
            log::warn!("indicators: lockdown_mode read failed: {e}");
            IndicatorValue::Unavailable
        }
    }
}

// ---------------------------------------------------------------------------
// System identification readers (private)
// ---------------------------------------------------------------------------

/// Format an ISO-8601 UTC timestamp for the current moment.
///
/// Uses `SystemTime::now()` — display-only, not a trust assertion.
/// Falls back to `"unavailable"` if the clock is behind UNIX_EPOCH
/// (should not occur on a sane system, but handled for correctness).
fn format_assessed_at() -> String {
    let Ok(dur) = SystemTime::now().duration_since(UNIX_EPOCH) else {
        return "unavailable".to_owned();
    };
    let total_secs = dur.as_secs();
    let days_since_epoch: i64 = match i64::try_from(total_secs / 86400) {
        Ok(d) => d,
        Err(_) => return "unavailable".to_owned(),
    };
    let rem = total_secs % 86400;
    let hours = rem / 3600;
    let minutes = (rem % 3600) / 60;
    let seconds = rem % 60;

    // Days since 1970-01-01 → calendar date via Howard Hinnant's Gregorian
    // algorithm (public domain). All arithmetic stays in i64 to avoid casts.
    // Algorithm invariant: `doe` is always in [0, 146096] after subtraction,
    // and `yoe` is always in [0, 399]; `mp` and `d` are always positive.
    let z: i64 = days_since_epoch + 719_468;
    let era: i64 = if z >= 0 {
        z
    } else {
        z - 146_096
    } / 146_097;
    let doe: i64 = z - era * 146_097; // [0, 146096]
    let yoe: i64 = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365; // [0, 399]
    let y: i64 = yoe + era * 400;
    let doy: i64 = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp: i64 = (5 * doy + 2) / 153;
    let d: i64 = doy - (153 * mp + 2) / 5 + 1;
    let m: i64 = if mp < 10 {
        mp + 3
    } else {
        mp - 9
    };
    let y: i64 = if m <= 2 {
        y + 1
    } else {
        y
    };

    format!("{y:04}-{m:02}-{d:02} {hours:02}:{minutes:02}:{seconds:02} UTC")
}

/// Read hostname, kernel release, and CPU architecture from `uname(2)`.
///
/// Returns `("(unknown)", "(unknown)", "(unknown)")` if any field is non-UTF-8.
/// All values are display-only — not trust-relevant assertions.
fn read_uname_fields() -> (String, String, String) {
    let uname = rustix::system::uname();
    let hostname = uname.nodename().to_str().unwrap_or("(unknown)").to_owned();
    let kernel = uname.release().to_str().unwrap_or("(unknown)").to_owned();
    let arch = uname.machine().to_str().unwrap_or("(unknown)").to_owned();
    (hostname, kernel, arch)
}

/// Read the kernel boot ID from `/proc/sys/kernel/random/boot_id`.
///
/// Uses `ProcfsText` + `SecureReader` (PROC_SUPER_MAGIC verification).
/// Returns `"unavailable"` on any read error. The value is trimmed of
/// trailing whitespace before return.
///
/// NSA RTB RAIN — raw `File::open` on `/proc/` is prohibited; all reads
/// route through `ProcfsText`.
fn read_boot_id() -> String {
    let Ok(node) =
        ProcfsText::new(PathBuf::from("/proc/sys/kernel/random/boot_id"))
    else {
        log::warn!("indicators: boot_id path rejected by ProcfsText");
        return "unavailable".to_owned();
    };

    match SecureReader::<ProcfsText>::new().read_generic_text(&node) {
        Ok(raw) => raw.trim().to_owned(),
        Err(e) => {
            log::warn!("indicators: boot_id read failed: {e}");
            "unavailable".to_owned()
        }
    }
}

/// Read the system UUID from `/sys/class/dmi/id/product_uuid`.
///
/// Uses `SysfsText` + `SecureReader` (SYSFS_MAGIC verification).
/// Returns `"unavailable"` on any read error (non-UEFI systems,
/// permission denied — readable only by root on many kernels).
/// The value is trimmed of trailing whitespace before return.
///
/// This function is public so that binaries can surface the system UUID in
/// the Kernel Security tab without storing it in `HeaderContext` (where it
/// is not displayed in the header proper).
///
/// NSA RTB RAIN — raw `File::open` on `/sys/` is prohibited; all reads
/// route through `SysfsText`.
#[must_use = "system UUID string must be used; discarding it omits hardware identity from the display"]
pub fn read_system_uuid() -> String {
    let Ok(node) =
        SysfsText::new(PathBuf::from("/sys/class/dmi/id/product_uuid"))
    else {
        log::warn!("indicators: system_uuid path rejected by SysfsText");
        return "unavailable".to_owned();
    };

    match SecureReader::<SysfsText>::new().read_generic_text(&node) {
        Ok(raw) => raw.trim().to_owned(),
        Err(e) => {
            // Permission denied is expected on non-root; debug level is appropriate.
            log::debug!(
                "indicators: system_uuid read failed (may require root): {e}"
            );
            "unavailable".to_owned()
        }
    }
}
