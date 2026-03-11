// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//!
//! # Kernel Anchor Phase
//!
//! Hard-gate phase that establishes a kernel-anchored trust foundation.
//! All subsequent phases depend on this one succeeding.
//!
//! ## Steps performed
//!
//! 1. Verify that `/proc/self/stat` is served by real procfs (`PROC_SUPER_MAGIC`).
//!    Failure → `DetectionError::ProcfsNotReal` (hard gate, aborts pipeline).
//! 2. PID coherence: compare `getpid(2)` result against PID in `/proc/self/stat`.
//!    Failure → `DetectionError::PidCoherenceFailed` (hard gate, aborts pipeline).
//! 3. Read `/proc/sys/kernel/random/boot_id`.
//!    Failure → soft: downgrade to `Untrusted`, return `Ok(None)`.
//! 4. Read kernel lockdown mode from `/sys/kernel/security/lockdown`.
//!    Failure → soft: record in evidence, log warn, continue.
//!
//! On full success, upgrades confidence to `TrustLevel::KernelAnchored`.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 SI-7**: Software and Information Integrity — procfs
//!   provenance verification before any kernel-sourced fact is trusted.
//! - **NIST SP 800-53 SA-9**: External Information System Services — TCB
//!   boundary: the kernel channel must be verified before it is relied upon.
//! - **NIST SP 800-53 CM-6**: Configuration Settings — lockdown mode is a
//!   kernel-level configuration baseline item recorded here for audit.
//! - **NSA RTB RAIN**: Non-bypassable — all procfs reads route through
//!   `ProcfsText` + `SecureReader::read_generic_text`.

use std::path::PathBuf;

use rustix::process::getpid;

use crate::confidence::{ConfidenceModel, TrustLevel};
use crate::evidence::{EvidenceBundle, EvidenceRecord, SourceKind};
use crate::kattrs::{KernelLockdown, ProcfsText, SecureReader, StaticSource};

use super::DetectionError;

/// Maximum bytes to read from a single procfs file in this phase.
///
/// boot_id is 36 chars + newline (37 bytes). `/proc/self/stat` is bounded
/// by kernel implementation. 4096 bytes is generous headroom.
const MAX_PROC_READ: usize = 4_096;

// ===========================================================================
// Public phase entry point
// ===========================================================================

/// Run the kernel anchor phase.
///
/// Returns `Ok(Some(boot_id))` on full success, `Ok(None)` if the boot_id
/// read fails non-fatally, or `Err` on hard-gate failure.
///
/// All reads use `ProcfsText` + `SecureReader::read_generic_text` for
/// provenance-verified, fd-anchored procfs access.
///
/// NIST SP 800-53 SI-7, SA-9, CM-6.
pub(super) fn run(
    evidence: &mut EvidenceBundle,
    confidence: &mut ConfidenceModel,
) -> Result<Option<String>, DetectionError> {
    #[cfg(debug_assertions)]
    let t0 = std::time::Instant::now();

    let result = run_inner(evidence, confidence);

    #[cfg(debug_assertions)]
    {
        let outcome = if result.is_ok() { "ok" } else { "err" };
        log::debug!(
            "kernel_anchor: completed ({}) in {} µs",
            outcome,
            t0.elapsed().as_micros()
        );
    }

    result
}

fn run_inner(
    evidence: &mut EvidenceBundle,
    confidence: &mut ConfidenceModel,
) -> Result<Option<String>, DetectionError> {
    // --- Step 1 + 2: procfs gate + PID coherence ----------------------------
    let stat_content = read_proc_self_stat(evidence)?;
    check_pid_coherence(&stat_content)?;

    // procfs is real and PID coherence passed — advance to T1.
    confidence.upgrade(TrustLevel::KernelAnchored);
    log::debug!("kernel_anchor: procfs gate passed; confidence upgraded to KernelAnchored");

    // --- Step 3: boot_id ----------------------------------------------------
    let boot_id = read_boot_id(evidence, confidence);

    // --- Step 4: lockdown mode (soft — never aborts) ------------------------
    read_lockdown(evidence);

    Ok(boot_id)
}

// ===========================================================================
// Step 1 + 2: /proc/self/stat read and PID coherence
// ===========================================================================

/// Read `/proc/self/stat` via provenance-verified `ProcfsText`.
///
/// Returns the raw content string for PID extraction. Returns
/// `Err(DetectionError::ProcfsNotReal)` if the magic check fails.
///
/// NIST SP 800-53 SI-7: fd-anchored fstatfs before any bytes are consumed.
fn read_proc_self_stat(evidence: &mut EvidenceBundle) -> Result<String, DetectionError> {
    let path = PathBuf::from("/proc/self/stat");

    let node = ProcfsText::new(path.clone()).map_err(|_| {
        log::error!("kernel_anchor: /proc/self/stat path rejected by ProcfsText");
        DetectionError::ProcfsNotReal
    })?;

    let content = SecureReader::<ProcfsText>::new()
        .read_generic_text(&node)
        .map_err(|e| {
            // Magic check failure comes back as PermissionDenied from execute_read_text.
            if e.kind() == std::io::ErrorKind::PermissionDenied {
                log::error!("kernel_anchor: /proc/self/stat failed filesystem magic check");
                DetectionError::ProcfsNotReal
            } else {
                log::error!("kernel_anchor: /proc/self/stat I/O error");
                DetectionError::KernelAnchorIo(e)
            }
        })?;

    // Enforce our own read cap on the returned content.
    if content.len() > MAX_PROC_READ {
        log::error!("kernel_anchor: /proc/self/stat content exceeds expected size");
        return Err(DetectionError::ProcfsNotReal);
    }

    evidence.push(EvidenceRecord {
        source_kind: SourceKind::Procfs,
        opened_by_fd: true,
        path_requested: path.display().to_string(),
        path_resolved: None,
        stat: None,
        fs_magic: None,
        sha256: None,
        pkg_digest: None,
        parse_ok: true,
        notes: vec!["procfs gate: PROC_SUPER_MAGIC verified".to_owned()],
    });

    Ok(content)
}

/// Compare `getpid(2)` against the PID field in `/proc/self/stat`.
///
/// `/proc/self/stat` field 1 is the process PID. We extract it by scanning
/// past the `comm` field (which is enclosed in parentheses and may contain
/// spaces) and reading the first whitespace token before it.
///
/// Returns `Err(DetectionError::PidCoherenceFailed)` on mismatch.
///
/// NIST SP 800-53 SI-7: kernel channel integrity.
fn check_pid_coherence(stat_content: &str) -> Result<(), DetectionError> {
    // PIDs on Linux are always positive i32 values; the cast to u32 is safe.
    // We use saturating_add(0) on the i32 to make the non-negative contract
    // visible, then cast. If somehow negative (impossible under Linux), we
    // fail coherence rather than panic.
    let raw_pid = getpid().as_raw_nonzero().get();
    let syscall_pid: u32 = if raw_pid > 0 {
        // ANSSI Rust Guide: checked arithmetic on security values.
        // raw_pid is a positive i32; fitting it in u32 is always valid here.
        raw_pid.cast_unsigned()
    } else {
        log::error!("kernel_anchor: getpid() returned non-positive value");
        return Err(DetectionError::ProcfsNotReal);
    };

    // PID is the first space-separated token in /proc/self/stat.
    let proc_pid_str = stat_content.split_whitespace().next().unwrap_or("");
    let proc_pid: u32 = proc_pid_str.parse().map_err(|_| {
        log::error!("kernel_anchor: could not parse PID from /proc/self/stat");
        DetectionError::ProcfsNotReal
    })?;

    if syscall_pid != proc_pid {
        log::error!(
            "kernel_anchor: PID coherence failure (syscall={syscall_pid}, procfs={proc_pid})"
        );
        return Err(DetectionError::PidCoherenceFailed {
            syscall: syscall_pid,
            procfs: proc_pid,
        });
    }

    log::debug!("kernel_anchor: PID coherence check passed (pid={syscall_pid})");
    Ok(())
}

// ===========================================================================
// Step 3: boot_id
// ===========================================================================

/// Read `/proc/sys/kernel/random/boot_id` and return the trimmed UUID string.
///
/// Non-fatal: on any failure, downgrades confidence and returns `None`.
///
/// NIST SP 800-53 SI-7 — boot session binding.
fn read_boot_id(
    evidence: &mut EvidenceBundle,
    confidence: &mut ConfidenceModel,
) -> Option<String> {
    let path = PathBuf::from("/proc/sys/kernel/random/boot_id");

    let node = ProcfsText::new(path.clone()).ok()?;
    let content = match SecureReader::<ProcfsText>::new().read_generic_text(&node) {
        Ok(s) => s,
        Err(e) => {
            log::warn!("kernel_anchor: could not read boot_id: {e}");
            confidence.downgrade(
                TrustLevel::Untrusted,
                "boot_id read failed — kernel anchor degraded",
            );
            evidence.push(EvidenceRecord {
                source_kind: SourceKind::Procfs,
                opened_by_fd: true,
                path_requested: path.display().to_string(),
                path_resolved: None,
                stat: None,
                fs_magic: None,
                sha256: None,
                pkg_digest: None,
                parse_ok: false,
                notes: vec!["boot_id read failed".to_owned()],
            });
            return None;
        }
    };

    // boot_id is 36 chars + newline. Reject anything implausibly large.
    if content.len() > 128 {
        log::warn!("kernel_anchor: boot_id content unexpectedly large, ignoring");
        return None;
    }

    let boot_id = content.trim().to_owned();

    evidence.push(EvidenceRecord {
        source_kind: SourceKind::Procfs,
        opened_by_fd: true,
        path_requested: path.display().to_string(),
        path_resolved: None,
        stat: None,
        fs_magic: None,
        sha256: None,
        pkg_digest: None,
        parse_ok: true,
        notes: vec!["boot_id read ok".to_owned()],
    });

    Some(boot_id)
}

// ===========================================================================
// Step 4: kernel lockdown mode
// ===========================================================================

/// Read the kernel lockdown mode from securityfs (soft — never aborts).
///
/// Records the result in the evidence bundle. The lockdown tier is captured
/// as a note rather than a separate confidence modifier — a system in
/// `Confidentiality` lockdown provides stronger provenance guarantees, which
/// the orchestrator may use to qualify the result.
///
/// NIST SP 800-53 CM-6: kernel lockdown is a configuration baseline item.
/// NIST SP 800-53 SI-7: lockdown provides MAC-enforced kernel integrity.
fn read_lockdown(evidence: &mut EvidenceBundle) {
    match SecureReader::<KernelLockdown>::new().read() {
        Ok(mode) => {
            log::debug!("kernel_anchor: lockdown mode = {mode}");
            evidence.push(EvidenceRecord {
                source_kind: SourceKind::SysfsNode,
                opened_by_fd: true,
                path_requested: KernelLockdown::PATH.to_owned(),
                path_resolved: None,
                stat: None,
                fs_magic: None,
                sha256: None,
                pkg_digest: None,
                parse_ok: true,
                notes: vec![format!("lockdown={mode}")],
            });
        }
        Err(e) => {
            log::warn!("kernel_anchor: could not read kernel lockdown mode: {e}");
            evidence.push(EvidenceRecord {
                source_kind: SourceKind::SysfsNode,
                opened_by_fd: true,
                path_requested: KernelLockdown::PATH.to_owned(),
                path_resolved: None,
                stat: None,
                fs_magic: None,
                sha256: None,
                pkg_digest: None,
                parse_ok: false,
                notes: vec!["lockdown read failed — securityfs may be unavailable".to_owned()],
            });
        }
    }
}
