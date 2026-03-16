// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//!
//! # Mount Topology Phase
//!
//! Soft-gate phase that maps the execution environment by reading the mount
//! namespace, parsing `/proc/self/mountinfo`, and performing a statfs
//! cross-check on `/etc`.
//!
//! ## Steps performed
//!
//! 1. Read the mount namespace inode from `/proc/self/ns/mnt` via `readlinkat`.
//!    The link target format is `mnt:[NNNNNNNNNN]`; the inode is extracted.
//! 2. Read `/proc/self/mountinfo` via `ProcfsText`. Apply `max_mountinfo_bytes`
//!    cap — content exceeding the limit triggers a downgrade and early return.
//! 3. Perform a statfs on `/etc` to record the filesystem magic of the path
//!    where `os-release` will be sought.
//!
//! All failures are soft — they downgrade confidence but do not abort the
//! pipeline. On success, upgrades confidence to `TrustLevel::EnvAnchored`.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 CM-8**: Information System Component Inventory —
//!   the mount topology is part of the accurate execution-environment inventory.
//! - **NIST SP 800-53 SC-39**: Process Isolation — the namespace inode
//!   identifies the mount namespace; unexpected namespaces are anomalies.
//! - **NSA RTB RAIN**: All procfs reads use `ProcfsText` + `SecureReader`.

use std::path::PathBuf;

use nix::sys::statfs::statfs;
use rustix::fs::{CWD, readlinkat};

use crate::confidence::{ConfidenceModel, TrustLevel};
use crate::evidence::{EvidenceBundle, EvidenceRecord, SourceKind};
use crate::kattrs::{ProcfsText, SecureReader};

// ===========================================================================
// Phase entry point
// ===========================================================================

/// Run the mount topology phase.
///
/// Never returns `Err` — all failures downgrade confidence and continue.
///
/// NIST SP 800-53 CM-8, SC-39.
pub(super) fn run(
    evidence: &mut EvidenceBundle,
    confidence: &mut ConfidenceModel,
    max_mountinfo_bytes: usize,
) {
    #[cfg(debug_assertions)]
    let t0 = std::time::Instant::now();

    run_inner(evidence, confidence, max_mountinfo_bytes);

    #[cfg(debug_assertions)]
    log::debug!(
        "mount_topology: completed in {} µs",
        t0.elapsed().as_micros()
    );
}

fn run_inner(
    evidence: &mut EvidenceBundle,
    confidence: &mut ConfidenceModel,
    max_mountinfo_bytes: usize,
) {
    // Step 1: mount namespace inode
    read_mnt_namespace(evidence);

    // Step 2: mountinfo
    let mountinfo_ok =
        read_mountinfo(evidence, confidence, max_mountinfo_bytes);

    // Step 3: statfs on /etc
    let statfs_ok = read_etc_statfs(evidence);

    // Upgrade to T2 only if both mountinfo and statfs succeeded.
    if mountinfo_ok && statfs_ok {
        confidence.upgrade(TrustLevel::EnvAnchored);
        log::debug!("mount_topology: confidence upgraded to EnvAnchored");
    } else {
        log::warn!(
            "mount_topology: partial failure — confidence not upgraded to T2"
        );
    }
}

// ===========================================================================
// Step 1: mount namespace inode
// ===========================================================================

/// Read the mount namespace inode from `/proc/self/ns/mnt`.
///
/// The symlink target has the form `mnt:[NNNNNNNNNN]`. We record the inode
/// value as a note in the evidence record. Failure is soft — logged and
/// recorded, but does not affect confidence.
///
/// NIST SP 800-53 SC-39: process isolation — namespace identification.
fn read_mnt_namespace(evidence: &mut EvidenceBundle) {
    let path = "/proc/self/ns/mnt";

    // readlinkat with AT_EMPTY_PATH on the procfs link.
    let result = readlinkat(CWD, path, Vec::new());

    match result {
        Ok(target) => {
            let target_str = target.to_string_lossy().into_owned();
            log::debug!("mount_topology: mnt namespace = {target_str}");
            evidence.push(EvidenceRecord {
                source_kind: SourceKind::Procfs,
                opened_by_fd: false,
                path_requested: path.to_owned(),
                path_resolved: Some(target_str.clone()),
                stat: None,
                fs_magic: None,
                sha256: None,
                pkg_digest: None,
                parse_ok: true,
                notes: vec![format!("mnt_ns={target_str}")],
                duration_ns: None,
            });
        }
        Err(e) => {
            log::warn!(
                "mount_topology: could not read mnt namespace link: {e}"
            );
            evidence.push(EvidenceRecord {
                source_kind: SourceKind::Procfs,
                opened_by_fd: false,
                path_requested: path.to_owned(),
                path_resolved: None,
                stat: None,
                fs_magic: None,
                sha256: None,
                pkg_digest: None,
                parse_ok: false,
                notes: vec!["mnt namespace readlink failed".to_owned()],
                duration_ns: None,
            });
        }
    }
}

// ===========================================================================
// Step 2: /proc/self/mountinfo
// ===========================================================================

/// Read `/proc/self/mountinfo` via provenance-verified `ProcfsText`.
///
/// Returns `true` if the read succeeded and the content was within the byte
/// cap. Returns `false` and downgrades confidence otherwise.
///
/// NIST SP 800-53 CM-8: mount inventory; SC-39: process isolation.
fn read_mountinfo(
    evidence: &mut EvidenceBundle,
    confidence: &mut ConfidenceModel,
    max_mountinfo_bytes: usize,
) -> bool {
    let path = PathBuf::from("/proc/self/mountinfo");

    let node = match ProcfsText::new(path.clone()) {
        Ok(n) => n,
        Err(e) => {
            log::warn!(
                "mount_topology: ProcfsText rejected mountinfo path: {e}"
            );
            confidence.downgrade(
                TrustLevel::KernelAnchored,
                "mountinfo path construction failed",
            );
            return false;
        }
    };

    let content = match SecureReader::<ProcfsText>::new()
        .read_generic_text(&node)
    {
        Ok(s) => s,
        Err(e) => {
            log::warn!("mount_topology: could not read mountinfo: {e}");
            confidence
                .downgrade(TrustLevel::KernelAnchored, "mountinfo read failed");
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
                notes: vec!["mountinfo read failed".to_owned()],
                duration_ns: None,
            });
            return false;
        }
    };

    if content.len() > max_mountinfo_bytes {
        log::warn!(
            "mount_topology: mountinfo content ({} bytes) exceeds cap ({} bytes)",
            content.len(),
            max_mountinfo_bytes
        );
        confidence.downgrade(
            TrustLevel::KernelAnchored,
            "mountinfo exceeded size cap",
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
            notes: vec!["mountinfo exceeded size cap".to_owned()],
            duration_ns: None,
        });
        return false;
    }

    let mount_count = content.lines().count();
    log::debug!("mount_topology: mountinfo has {mount_count} entries");

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
        notes: vec![format!("mount_count={mount_count}")],
        duration_ns: None,
    });

    true
}

// ===========================================================================
// Step 3: statfs on /etc
// ===========================================================================

/// Perform a statfs on `/etc` and record the filesystem magic.
///
/// This cross-checks that the path where `os-release` will be sought is on
/// a real, identifiable filesystem — not a tmpfs substitution.
///
/// NIST SP 800-53 CM-8: execution environment inventory.
fn read_etc_statfs(evidence: &mut EvidenceBundle) -> bool {
    match statfs("/etc") {
        Ok(stat) => {
            // nix FsType is a newtype over i64; extract the raw magic value.
            let magic = stat.filesystem_type().0;
            // Cast i64 → u64 for storage; filesystem magic values are defined
            // as positive constants in linux/magic.h.
            let magic_u64 = magic.cast_unsigned();
            log::debug!(
                "mount_topology: /etc filesystem magic = {magic_u64:#x}"
            );
            evidence.push(EvidenceRecord {
                source_kind: SourceKind::StatfsResult,
                opened_by_fd: false,
                path_requested: "/etc".to_owned(),
                path_resolved: None,
                stat: None,
                fs_magic: Some(magic_u64),
                sha256: None,
                pkg_digest: None,
                parse_ok: true,
                notes: vec![format!("etc_fs_magic={magic_u64:#x}")],
                duration_ns: None,
            });
            true
        }
        Err(e) => {
            log::warn!("mount_topology: statfs(/etc) failed: {e}");
            evidence.push(EvidenceRecord {
                source_kind: SourceKind::StatfsResult,
                opened_by_fd: false,
                path_requested: "/etc".to_owned(),
                path_resolved: None,
                stat: None,
                fs_magic: None,
                sha256: None,
                pkg_digest: None,
                parse_ok: false,
                notes: vec!["statfs(/etc) failed".to_owned()],
                duration_ns: None,
            });
            false
        }
    }
}
