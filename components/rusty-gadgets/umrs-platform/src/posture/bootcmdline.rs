// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//! Bootloader configured-cmdline reader — Phase 2b.
//!
//! Reads the **configured** kernel command line from bootloader entries on
//! disk, providing the "intended" cmdline for contradiction detection against
//! the live `/proc/cmdline`.
//!
//! ## Architecture
//!
//! On RHEL 10, the Boot Loader Specification (BLS) is the standard boot entry
//! format. BLS entries live under `/boot/loader/entries/*.conf` and are used
//! by both GRUB2 (via the `blscfg` module) and systemd-boot. Each entry
//! contains an `options` line that becomes the kernel cmdline.
//!
//! This module reads BLS entries only — it does not parse raw `grub.cfg` files,
//! which have a complex template language (shell-like conditionals, variables,
//! loops). Parsing raw `grub.cfg` is out of scope for Phase 2b; BLS entries
//! provide the structured, machine-readable form that RHEL 10 uses.
//!
//! ## Entry Selection Heuristic
//!
//! BLS does not designate a single "active" entry at rest. The bootloader
//! selects the entry at boot time based on the `default` value in
//! `/boot/loader/loader.conf` (systemd-boot) or GRUB2's environment block.
//! Neither of these is trivially parseable in a way that reliably maps to an
//! entry file.
//!
//! This module uses the following selection strategy (in priority order):
//!
//! 1. If only one BLS entry exists, use it.
//! 2. Try to match an entry whose `version` field matches the running kernel
//!    version from `/proc/version_signature` or `/proc/sys/kernel/osrelease`.
//! 3. Fall back to the lexicographically last entry (highest version by name).
//!
//! This heuristic is correct for standard RHEL 10 deployments with a single
//! installed kernel. If multiple kernels are installed, the heuristic prefers
//! the newest entry, which is the RHEL default.
//!
//! ## Trust Boundary
//!
//! `/boot/loader/entries/` is a regular filesystem path, not a pseudo-filesystem.
//! These are **advisory** configured values — the live `/proc/cmdline` is
//! always authoritative. If entries are absent or unreadable, `None` is returned
//! silently (best-effort, no error propagated to caller).
//!
//! ## Compliance
//!
//! NIST SP 800-53 CM-6: Configuration Settings — the BLS `options` line is the
//! persistence layer for cmdline security tokens.
//! NIST SP 800-53 CA-7: Continuous Monitoring — enables `EphemeralHotfix` and
//! `BootDrift` detection for cmdline signals (`ModuleSigEnforce`, `Mitigations`,
//! `Pti`, `Lockdown`, `RandomTrustCpu`, `RandomTrustBootloader`).
//! NIST SP 800-53 SI-10: Input Validation — BLS entry content is validated
//! line-by-line; malformed entries are skipped with debug logging.

use std::path::{Path, PathBuf};

// ===========================================================================
// Constants
// ===========================================================================

/// Directory containing BLS boot entries on RHEL 10 / systemd-boot systems.
const BLS_ENTRIES_DIR: &str = "/boot/loader/entries";

/// Kernel osrelease node for current kernel version matching.
const KERNEL_OSRELEASE: &str = "/proc/sys/kernel/osrelease";

// ===========================================================================
// Public API
// ===========================================================================

/// Read the configured kernel cmdline from the most likely active BLS entry.
///
/// Returns the trimmed `options` line from the selected BLS entry, or `None`
/// if no BLS entries are found, no `options` line is present in the selected
/// entry, or files cannot be read.
///
/// This is a **best-effort** read. Failures are logged at `debug` and `None`
/// is returned. The caller must not treat `None` as an error condition — it
/// means the configured cmdline is unavailable in this environment.
///
/// NIST SP 800-53 CM-6: boot-persistence layer for cmdline signals.
/// NIST SP 800-53 CA-7: enables contradiction detection for cmdline signals.
#[must_use = "configured cmdline result must be examined — None means bootloader config unavailable"]
pub fn read_configured_cmdline() -> Option<String> {
    #[cfg(debug_assertions)]
    let start = std::time::Instant::now();

    let entries = collect_bls_entries()?;
    if entries.is_empty() {
        log::debug!(
            "posture: bootcmdline: no BLS entries found in {BLS_ENTRIES_DIR}"
        );
        return None;
    }

    let selected = select_entry(&entries)?;
    let options = parse_bls_options(selected)?;

    #[cfg(debug_assertions)]
    log::debug!(
        "posture: bootcmdline: read configured cmdline in {} µs from {}",
        start.elapsed().as_micros(),
        selected.display()
    );

    Some(options)
}

// ===========================================================================
// Entry collection
// ===========================================================================

/// Collect all `.conf` files from the BLS entries directory in sorted order.
///
/// Returns `None` if the directory is absent or unreadable (graceful degrade).
/// Returns `Some(empty)` if the directory exists but contains no `.conf` files.
fn collect_bls_entries() -> Option<Vec<PathBuf>> {
    let dir = Path::new(BLS_ENTRIES_DIR);
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(ref e) if e.kind() == std::io::ErrorKind::NotFound => {
            log::debug!(
                "posture: bootcmdline: {BLS_ENTRIES_DIR} absent — \
                 BLS not available on this system"
            );
            return None;
        }
        Err(e) => {
            log::debug!(
                "posture: bootcmdline: cannot read {BLS_ENTRIES_DIR}: {e}"
            );
            return None;
        }
    };

    let mut paths: Vec<PathBuf> = entries
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("conf"))
        .collect();

    paths.sort();
    Some(paths)
}

// ===========================================================================
// Entry selection heuristic
// ===========================================================================

/// Select the most likely active BLS entry from the collected list.
///
/// Strategy (in priority order):
/// 1. Single entry — use it directly.
/// 2. Version match — find the entry whose `version` field matches the
///    running kernel's osrelease string from `/proc/sys/kernel/osrelease`.
/// 3. Fallback — use the lexicographically last entry (typically the newest).
///
/// Returns `None` only if the entries list is empty.
fn select_entry(entries: &[PathBuf]) -> Option<&PathBuf> {
    if entries.is_empty() {
        return None;
    }

    // Single entry — no selection needed.
    if entries.len() == 1 {
        return entries.first();
    }

    // Try to match by kernel osrelease.
    if let Some(osrelease) = read_kernel_osrelease() {
        for entry in entries {
            if let Some(version) = parse_bls_field(entry, "version")
                && version.trim() == osrelease.trim()
            {
                log::debug!(
                    "posture: bootcmdline: matched entry by osrelease '{}': {}",
                    osrelease.trim(),
                    entry.display()
                );
                return Some(entry);
            }
        }
        log::debug!(
            "posture: bootcmdline: no entry matched osrelease '{}', \
             falling back to last entry",
            osrelease.trim()
        );
    }

    // Fallback: lexicographically last entry.
    entries.last()
}

/// Read the running kernel's osrelease string from
/// `/proc/sys/kernel/osrelease`.
///
/// This is NOT a provenance-verified read because the osrelease value is used
/// only for heuristic BLS entry selection — not for a security assertion. The
/// live cmdline (from `/proc/cmdline` via `CmdlineReader`) is the authoritative
/// source; the osrelease value only influences which BLS entry is compared.
///
/// # Security note — unverified procfs read
///
/// This read does not go through `SecureReader` / `PROC_SUPER_MAGIC`. On an
/// enforcing SELinux system, `/proc/sys/kernel/osrelease` is labeled `proc_t`
/// and is writable only by the kernel — userspace cannot inject a false value
/// through the procfs VFS layer. The unverified read is therefore safe under
/// the project's target deployment (SELinux enforcing, integrity lockdown).
/// If the posture probe is ever deployed without SELinux enforcing, this read
/// should be upgraded to `ProcfsText` + `PROC_SUPER_MAGIC` to remove the
/// deployment assumption. This dependency is documented here so reviewers have
/// explicit context.
///
/// NIST SP 800-53 SI-7; NIST SP 800-218 SSDF PW.4; NSA RTB RAIN.
///
/// Returns `None` if unreadable (permission, absent node).
fn read_kernel_osrelease() -> Option<String> {
    match std::fs::read_to_string(KERNEL_OSRELEASE) {
        Ok(s) => Some(s.trim().to_owned()),
        Err(e) => {
            log::debug!(
                "posture: bootcmdline: cannot read {KERNEL_OSRELEASE}: {e}"
            );
            None
        }
    }
}

// ===========================================================================
// BLS entry parsing
// ===========================================================================

/// Read the `options` line from a BLS entry file.
///
/// Returns the trimmed options string, or `None` if the file cannot be read
/// or contains no `options` line.
fn parse_bls_options(entry_path: &Path) -> Option<String> {
    parse_bls_field(entry_path, "options")
}

/// Read a named field from a BLS entry file.
///
/// BLS entry format: each line is `<key>  <value>` (one or more spaces/tabs
/// between key and value). Lines starting with `#` are comments. Empty lines
/// are ignored.
///
/// Returns the trimmed value for the first occurrence of `field`, or `None`
/// if the file cannot be read or the field is absent.
///
/// # File size assumption
///
/// BLS entry files are expected to be small (well under 64 KiB in practice —
/// a typical entry is under 1 KiB). `std::fs::read_to_string` reads the entire
/// file into memory without a size cap. A crafted or corrupted entry that is
/// extremely large would cause a proportional allocation before this function
/// returns `None` via the error path. This is an availability concern only (no
/// security failure); the worst outcome is a transient memory spike. For
/// correctness on realistic BLS files, the current approach is sufficient.
///
/// NIST SP 800-53 SI-10: Input Validation — malformed lines are skipped;
/// the field match is exact (not a substring match).
fn parse_bls_field(entry_path: &Path, field: &str) -> Option<String> {
    let content = match std::fs::read_to_string(entry_path) {
        Ok(c) => c,
        Err(e) => {
            log::debug!(
                "posture: bootcmdline: cannot read {}: {e}",
                entry_path.display()
            );
            return None;
        }
    };

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        // BLS lines are: `<key>[ \t]+<value>`
        // Split on the first run of whitespace.
        let mut parts = trimmed.splitn(2, |c: char| c.is_ascii_whitespace());
        let key = parts.next()?;
        if key != field {
            continue;
        }
        let value = parts.next().unwrap_or("").trim();
        return Some(value.to_owned());
    }

    None
}
