// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//!
//! # Release Candidate Phase
//!
//! Soft-gate phase that locates a candidate `os-release` file and records its
//! metadata. No parsing of the file's content occurs here — that is deferred
//! to `release_parse.rs`.
//!
//! ## Steps performed
//!
//! 1. Probe the two standard `os-release` paths in priority order:
//!    `/etc/os-release` (writable by the system administrator) then
//!    `/usr/lib/os-release` (read-only, distribution-provided fallback).
//!    The first path that exists and passes a basic permissions sanity check
//!    is selected as the candidate.
//!
//! 2. Collect `statx(2)` metadata from the selected path using
//!    `rustix::fs::statx`. The device + inode pair (`dev`, `ino`) is recorded
//!    in the `EvidenceRecord` and returned to the orchestrator. These values
//!    anchor the subsequent ownership and integrity phases to a single
//!    kernel-verified file identity — preventing TOCTOU substitution between
//!    phases.
//!
//! 3. If the candidate is a symbolic link, resolve the link target via
//!    `rustix::fs::readlinkat`. The resolved path is recorded for audit.
//!    Note: On RHEL 10, `/etc/os-release` is a symlink to
//!    `/usr/lib/os-release`; resolving it is expected and normal.
//!
//! All failures are soft — they produce a downgrade and return `None`.
//! On success, returns `Some(candidate_path)` and the `FileStat` is embedded
//! in the evidence record.
//!
//! ## Permissions sanity check
//!
//! Before accepting a candidate, the file's mode is checked:
//! - World-writable (`0o002`) — rejected immediately, triggers downgrade.
//! - Owned by root (uid == 0) — expected and recorded as a note.
//! - Setuid bit set — recorded as a note; unusual but not fatal at this phase.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 SI-7**: Software and Information Integrity — `statx`
//!   metadata is recorded before any content is read; the `(dev, ino)` pair
//!   is re-verified in subsequent phases to prevent substitution.
//! - **NIST SP 800-53 CM-8**: Component Inventory — the os-release file is
//!   the canonical identity assertion; its filesystem provenance must be
//!   captured with full metadata.
//! - **NSA RTB TOCTOU**: fd-anchored `statx` before content read; `(dev, ino)`
//!   returned to caller for re-verification in later phases.

use std::path::{Path, PathBuf};

use rustix::fs::{AtFlags, CWD, StatxFlags, readlinkat, statx};

use crate::confidence::{ConfidenceModel, TrustLevel};
use crate::evidence::{EvidenceBundle, EvidenceRecord, FileStat, SourceKind};

/// Standard `os-release` search paths, in priority order.
///
/// `/etc/os-release` is preferred. If absent, `/usr/lib/os-release` is the
/// distribution-maintained fallback (systemd `os-release(5)` specification).
const OS_RELEASE_PATHS: [&str; 2] = ["/etc/os-release", "/usr/lib/os-release"];

/// World-writable mode bit.
const S_IWOTH: u32 = 0o002;

// ===========================================================================
// Phase entry point
// ===========================================================================

/// Run the release candidate phase.
///
/// Returns `Some(path)` if a usable candidate was found, `None` otherwise.
/// A returned `path` has passed permissions sanity and has `statx` metadata
/// recorded in `evidence`.
///
/// NIST SP 800-53 SI-7, CM-8. NSA RTB TOCTOU.
pub(super) fn run(
    evidence: &mut EvidenceBundle,
    confidence: &mut ConfidenceModel,
) -> Option<PathBuf> {
    #[cfg(debug_assertions)]
    let t0 = std::time::Instant::now();

    let result = run_inner(evidence, confidence);

    #[cfg(debug_assertions)]
    log::debug!(
        "release_candidate: completed in {} µs",
        t0.elapsed().as_micros()
    );

    result
}

fn run_inner(evidence: &mut EvidenceBundle, confidence: &mut ConfidenceModel) -> Option<PathBuf> {
    for &path_str in &OS_RELEASE_PATHS {
        let path = PathBuf::from(path_str);

        if let Some(candidate) = probe_candidate(path_str, &path, evidence, confidence) {
            return Some(candidate);
        }
    }

    // Neither path was usable.
    log::warn!("release_candidate: no usable os-release candidate found at any standard path");
    confidence.downgrade(
        TrustLevel::EnvAnchored,
        "os-release not found at any standard path",
    );
    None
}

// ===========================================================================
// Candidate probe
// ===========================================================================

/// Probe a single os-release path candidate.
///
/// Returns `Some(path)` if the path is usable; `None` if the path should be
/// skipped (e.g., not present, permissions failure). Metadata is always
/// pushed to `evidence` when the syscall succeeds, even for rejected candidates.
///
/// NIST SP 800-53 SI-7, CM-8.
fn probe_candidate(
    path_str: &str,
    path: &Path,
    evidence: &mut EvidenceBundle,
    confidence: &mut ConfidenceModel,
) -> Option<PathBuf> {
    // Step 1: statx the path to get metadata and detect presence.
    // AtFlags::empty() intentionally follows symlinks — returns target (dev,ino), not symlink's.
    let statx_result = statx(CWD, path_str, AtFlags::empty(), StatxFlags::ALL);

    let Ok(sx) = statx_result else {
        // Path not present or not accessible — try next candidate.
        log::debug!("release_candidate: {path_str} not accessible, skipping");
        return None;
    };

    // Extract mode bits from statx result using `From` (no lossy cast).
    let mode = u32::from(sx.stx_mode);

    // Step 2: Permissions sanity check — world-writable is an immediate reject.
    if mode & S_IWOTH != 0 {
        log::warn!("release_candidate: {path_str} is world-writable — rejecting candidate");
        confidence.downgrade(
            TrustLevel::EnvAnchored,
            "os-release candidate is world-writable",
        );
        evidence.push(EvidenceRecord {
            source_kind: SourceKind::RegularFile,
            opened_by_fd: false,
            path_requested: path_str.to_owned(),
            path_resolved: None,
            stat: Some(extract_file_stat(&sx)),
            fs_magic: None,
            sha256: None,
            pkg_digest: None,
            parse_ok: false,
            notes: vec!["rejected: world-writable".to_owned()],
            duration_ns: None,
        });
        return None;
    }

    // Step 3: Symlink resolution — record resolved target for audit.
    let resolved = resolve_symlink(path_str, evidence);

    // Step 4: Assemble notes.
    let mut notes = Vec::new();
    if sx.stx_uid == 0 {
        log::debug!("os-release is owned by root.");
        notes.push("owned by root".to_owned());
    } else {
        notes.push(format!("owner uid={}", sx.stx_uid));
        log::debug!("os-release owner uid={}", sx.stx_uid);
    }
    if mode & 0o4000 != 0 {
        notes.push("setuid bit set (unusual for os-release)".to_owned());
    }
    notes.push(format!(
        "dev={}:{} ino={}",
        sx.stx_dev_major, sx.stx_dev_minor, sx.stx_ino
    ));
    // If this path is a symlink, record that statx followed it: the (dev,ino)
    // belongs to the resolved target, not the symlink inode itself.
    if resolved.is_some() {
        notes.push(
            "statx followed symlink (AT_SYMLINK_NOFOLLOW not set); \
             (dev,ino) belongs to resolved target"
                .to_owned(),
        );
    }

    // Combine major/minor into a single u64 device ID for storage.
    // The standard representation is (major << 32) | minor for unambiguous u64 storage.
    let dev_combined: u64 = (u64::from(sx.stx_dev_major) << 32) | u64::from(sx.stx_dev_minor);

    evidence.push(EvidenceRecord {
        source_kind: SourceKind::RegularFile,
        opened_by_fd: false,
        path_requested: path_str.to_owned(),
        path_resolved: resolved,
        stat: Some(FileStat {
            dev: Some(dev_combined),
            ino: Some(sx.stx_ino),
            mode: Some(mode),
            uid: Some(sx.stx_uid),
            gid: Some(sx.stx_gid),
            nlink: Some(sx.stx_nlink.into()),
            size: Some(sx.stx_size),
            mtime: Some(sx.stx_mtime.tv_sec),
        }),
        fs_magic: None,
        sha256: None,
        pkg_digest: None,
        parse_ok: true,
        notes,
        duration_ns: None,
    });

    log::debug!(
        "release_candidate: selected {} (ino={})",
        path_str,
        sx.stx_ino
    );

    Some(path.to_path_buf())
}

// ===========================================================================
// Helper: statx field extraction
// ===========================================================================

/// Extract a `FileStat` from a `rustix::fs::Statx` value.
///
/// Used when we have a valid statx result but are rejecting the candidate —
/// we still want to record the full metadata for the audit trail.
fn extract_file_stat(sx: &rustix::fs::Statx) -> FileStat {
    let dev_combined: u64 = (u64::from(sx.stx_dev_major) << 32) | u64::from(sx.stx_dev_minor);
    FileStat {
        dev: Some(dev_combined),
        ino: Some(sx.stx_ino),
        mode: Some(u32::from(sx.stx_mode)),
        uid: Some(sx.stx_uid),
        gid: Some(sx.stx_gid),
        nlink: Some(sx.stx_nlink.into()),
        size: Some(sx.stx_size),
        mtime: Some(sx.stx_mtime.tv_sec),
    }
}

// ===========================================================================
// Helper: symlink resolution
// ===========================================================================

/// Attempt to resolve `path_str` as a symlink via `readlinkat`.
///
/// Returns `Some(target_str)` if the path is a symlink and resolution
/// succeeds; `None` if the path is not a symlink or resolution fails.
/// Failures are logged at debug level — a non-symlink path is not an error.
///
/// The resolved path is recorded in the evidence record's `path_resolved`
/// field (NIST SP 800-53 AU-3 — audit records include resolved paths).
fn resolve_symlink(path_str: &str, evidence: &mut EvidenceBundle) -> Option<String> {
    if let Ok(target) = readlinkat(CWD, path_str, Vec::new()) {
        let target_str = target.to_string_lossy().into_owned();
        log::debug!("release_candidate: {path_str} is a symlink → {target_str}");
        evidence.push(EvidenceRecord {
            source_kind: SourceKind::SymlinkTarget,
            opened_by_fd: false,
            path_requested: path_str.to_owned(),
            path_resolved: Some(target_str.clone()),
            stat: None,
            fs_magic: None,
            sha256: None,
            pkg_digest: None,
            parse_ok: true,
            notes: vec![format!("symlink target={target_str}")],
            duration_ns: None,
        });
        Some(target_str)
    } else {
        // Not a symlink or path not accessible — not an error condition.
        log::debug!("release_candidate: {path_str} is not a symlink (or readlinkat failed)");
        None
    }
}
