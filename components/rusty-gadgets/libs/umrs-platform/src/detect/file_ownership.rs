// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//!
//! # File Ownership Phase
//!
//! Soft-gate phase that queries the selected package substrate probe to
//! determine whether the os-release candidate is owned by a known package.
//!
//! ## Steps performed
//!
//! 1. Extract `(dev, ino)` from the `FileStat` recorded by `release_candidate`
//!    for the chosen candidate path. If no stat record is available, the phase
//!    returns `None` immediately with a downgrade.
//!
//! 2. Call `probe.query_ownership(dev, ino, path)`. The probe must verify both
//!    the path and the `(dev, ino)` pair — this prevents TOCTOU substitution
//!    between the candidate selection phase and the ownership query.
//!
//! 3. Record the ownership result in `evidence`. A `None` return from the
//!    probe means the file is unowned — this is not an error, but it does
//!    prevent T4 (`IntegrityAnchored`) from being reached.
//!
//! All failures are soft — they downgrade confidence and return `None`, but
//! they never abort the pipeline.
//!
//! ## TOCTOU note
//!
//! The `(dev, ino)` pair is obtained from the `release_candidate` phase's
//! `statx` call. When the probe's `query_ownership` implementation receives
//! this pair, it can cross-check the path's current on-disk `(dev, ino)` to
//! detect substitution. The stub implementations do not perform this check
//! (they return `None`) — full implementations must perform it.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 SI-7**: Software and Information Integrity — confirming
//!   package ownership is a prerequisite for integrity verification.
//! - **NIST SP 800-53 AU-10**: Non-Repudiation — the ownership record names
//!   the package responsible for the file; recorded in evidence for audit.
//! - **NSA RTB TOCTOU**: `(dev, ino)` anchoring prevents path-substitution
//!   between candidate selection and ownership query.

use std::path::{Path, PathBuf};

use crate::confidence::{ConfidenceModel, TrustLevel};
use crate::evidence::{EvidenceBundle, EvidenceRecord, SourceKind};

use super::substrate::{FileOwnership, PackageProbe};

// ===========================================================================
// Phase entry point
// ===========================================================================

/// Run the file ownership phase.
///
/// Returns the ownership record if the file is owned by a known package.
/// Returns `None` if the probe is absent, the stat record is missing, or
/// the file is unowned.
///
/// ## Compliance
///
/// - NIST SP 800-53 SI-7, AU-10.
/// - NSA RTB TOCTOU.
pub(super) fn run(
    evidence: &mut EvidenceBundle,
    confidence: &mut ConfidenceModel,
    candidate: &Path,
    probe: Option<&dyn PackageProbe>,
) -> Option<FileOwnership> {
    #[cfg(debug_assertions)]
    let t0 = std::time::Instant::now();

    let result = run_inner(evidence, confidence, candidate, probe);

    #[cfg(debug_assertions)]
    log::debug!(
        "file_ownership: completed in {} µs",
        t0.elapsed().as_micros()
    );

    result
}

fn run_inner(
    evidence: &mut EvidenceBundle,
    confidence: &mut ConfidenceModel,
    candidate: &Path,
    probe: Option<&dyn PackageProbe>,
) -> Option<FileOwnership> {
    // Step 1: Probe must be present (T3 was reached).
    let Some(probe) = probe else {
        log::debug!("file_ownership: no probe available — skipping ownership query");
        return None;
    };

    let candidate_str = candidate.to_string_lossy().into_owned();

    // Step 2: Extract (dev, ino) from the most recent EvidenceRecord for the candidate path.
    // We search the bundle in reverse order — the release_candidate record is the most recent
    // stat record for this path.
    let Some((dev, ino)) = find_stat_for_path(evidence, &candidate_str) else {
        log::warn!(
            "file_ownership: no stat record found for candidate — cannot anchor ownership query"
        );
        confidence.downgrade(
            TrustLevel::SubstrateAnchored,
            "file ownership: stat record missing for os-release candidate",
        );
        return None;
    };

    // Step 3: Resolve the query path.
    //
    // On RHEL 10, `/etc/os-release` is a symlink to `/usr/lib/os-release`. The RPM
    // database records the real path, not the symlink. Use the resolved path for the
    // DB query so that ownership can be established. The candidate_str (symlink path)
    // is still used for the audit record's path_requested field to preserve the
    // original request path in the evidence chain.
    //
    // `readlinkat` may return a relative symlink target (e.g., `../usr/lib/os-release`).
    // Resolve against the candidate's parent directory and canonicalize to produce an
    // absolute path that matches what the RPM DB records.
    //
    // NIST SP 800-53 SI-7 — integrity verification must complete on the target platform.
    let query_path: PathBuf = match find_resolved_path(evidence, &candidate_str) {
        Some(resolved) => {
            let p = PathBuf::from(&resolved);
            if p.is_absolute() {
                p
            } else {
                // Relative symlink: join with the candidate's parent directory, then
                // canonicalize to an absolute path.
                let parent = candidate.parent().unwrap_or_else(|| Path::new("/"));
                std::fs::canonicalize(parent.join(&p)).unwrap_or_else(|_| parent.join(p))
            }
        }
        None => candidate.to_path_buf(),
    };

    if query_path != candidate {
        log::debug!(
            "file_ownership: using resolved path '{}' (symlink target of '{candidate_str}') \
             for RPM DB query",
            query_path.display()
        );
    }

    // Step 4: Query ownership via the probe.
    let ownership = probe.query_ownership(dev, ino, &query_path);

    if let Some(o) = &ownership {
        log::debug!(
            "file_ownership: {candidate_str} owned by package '{}' version '{}'",
            o.package_name,
            o.package_version
        );
        evidence.push(EvidenceRecord {
            source_kind: SourceKind::PackageDb,
            path_requested: candidate_str,
            parse_ok: true,
            notes: vec![
                format!("owner={}", o.package_name),
                format!("version={}", o.package_version),
            ],
            ..Default::default()
        });
    } else {
        log::warn!("file_ownership: {candidate_str} has no package owner — T4 cannot be reached");
        evidence.push(EvidenceRecord {
            source_kind: SourceKind::PackageDb,
            path_requested: candidate_str,
            notes: vec!["file unowned by any package".to_owned()],
            ..Default::default()
        });
    }

    ownership
}

// ===========================================================================
// Helper: locate (dev, ino) from evidence bundle
// ===========================================================================

/// Search the evidence bundle (in reverse order) for a `FileStat` record
/// matching `path_str`.
///
/// Returns `(dev, ino)` if found; `None` if no stat record exists for the path.
///
/// `pub(super)` so that sibling phase modules (e.g., `integrity_check`,
/// `release_parse`) can re-verify `(dev, ino)` without duplicating the
/// search logic. NIST SP 800-53 SI-7 — TOCTOU re-verification.
pub(super) fn find_stat_for_path(evidence: &EvidenceBundle, path_str: &str) -> Option<(u64, u64)> {
    for record in evidence.iter().rev() {
        if record.path_requested == path_str
            && let Some(ref stat) = record.stat
            && let (Some(dev), Some(ino)) = (stat.dev, stat.ino)
        {
            return Some((dev, ino));
        }
    }
    None
}

/// Search the evidence bundle (in reverse order) for a symlink resolution record
/// where `path_requested` matches `candidate_str` and `path_resolved` is present.
///
/// Returns the resolved (real) path string if found; `None` if the candidate was
/// not recorded as a symlink. Used to redirect RPM DB queries from the symlink path
/// (e.g., `/etc/os-release`) to the real path that the DB owns (e.g.,
/// `/usr/lib/os-release`).
///
/// `pub(super)` so that `integrity_check` can perform the same redirection.
/// NIST SP 800-53 SI-7 — integrity verification must use the path the package DB
/// tracks, not the symlink.
pub(super) fn find_resolved_path(evidence: &EvidenceBundle, candidate_str: &str) -> Option<String> {
    for record in evidence.iter().rev() {
        if record.path_requested == candidate_str
            && let Some(ref resolved) = record.path_resolved
        {
            return Some(resolved.clone());
        }
    }
    None
}
