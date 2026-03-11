// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//!
//! # OS Detection Pipeline — `detect`
//!
//! Orchestrates a multi-phase, evidence-accumulating pipeline for determining
//! the identity and integrity of the running operating system. Each phase
//! builds on the previous one and can only upgrade the confidence model when
//! its verification gate passes.
//!
//! The pipeline is designed so that partial results are still useful: phase
//! failures downgrade confidence and are recorded in the `EvidenceBundle`,
//! but only two conditions abort the pipeline entirely (hard gates):
//!
//! 1. The procfs anchor fails (`DetectionError::ProcfsNotReal`) — if procfs
//!    is not real procfs, no kernel-anchored fact can be established.
//! 2. PID coherence is broken (`DetectionError::PidCoherenceFailed`) — the
//!    kernel channel is compromised.
//!
//! All other failures produce a downgraded `TrustLevel` and are reflected in
//! `DetectionResult::confidence`.
//!
//! ## Phase Sequence
//!
//! | Module | Phase | Gate |
//! |---|---|---|
//! | `kernel_anchor` | Procfs magic + PID coherence + boot_id + lockdown | Hard gate |
//! | `mount_topology` | Namespace IDs + mountinfo + statfs cross-check | Soft (downgrade) |
//! | `release_candidate` | os-release path resolution + perms + symlink policy | Soft (downgrade) |
//! | `pkg_substrate` | PackageProbe dispatch + SELinux enforce check for T3 | Soft (downgrade) |
//! | `file_ownership` | File ownership query via selected probe | Soft (downgrade) |
//! | `integrity_check` | Installed digest fetch + SHA-256 compute + compare | Soft (downgrade) |
//! | `release_parse` | Strict os-release field parsing + substrate corroboration | Soft (downgrade) |
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 SA-8**: Security Engineering Principles — the pipeline
//!   is fail-closed and layered; each phase must earn its trust tier.
//! - **NIST SP 800-53 CM-6**: Configuration Settings — the detection result
//!   provides a verified basis for configuration compliance decisions.
//! - **NIST SP 800-53 SI-7**: Software and Information Integrity — the kernel
//!   anchor and digest verification phases directly implement SI-7.

pub mod label_trust;
pub mod substrate;

mod kernel_anchor;
mod mount_topology;
mod release_candidate;
mod pkg_substrate;
mod file_ownership;
mod integrity_check;
mod release_parse;

use thiserror::Error;

use crate::confidence::ConfidenceModel;
use crate::evidence::EvidenceBundle;
use crate::os_identity::SubstrateIdentity;
use crate::os_release::OsRelease;
use label_trust::LabelTrust;

// ===========================================================================
// DetectionError
// ===========================================================================

/// Hard-gate errors that abort the detection pipeline.
///
/// These are the only conditions under which `OsDetector::detect` returns
/// `Err`. All other failures produce a downgraded `TrustLevel` within a
/// successful `DetectionResult`.
///
/// NIST SP 800-53 SA-8, SI-7 — fail-closed on kernel channel compromise.
#[derive(Debug, Error)]
pub enum DetectionError {
    /// procfs failed the `PROC_SUPER_MAGIC` check — it is not real procfs.
    ///
    /// This means no kernel-anchored fact can be established. Continuing would
    /// produce a result with no verifiable basis.
    #[error("procfs is not real procfs — cannot establish kernel anchor")]
    ProcfsNotReal,

    /// PID coherence check failed: the PID returned by `getpid(2)` does not
    /// match the PID parsed from `/proc/self/stat`.
    ///
    /// `syscall` and `procfs` values are process IDs — not sensitive data.
    ///
    /// NIST SP 800-53 SI-7 — kernel channel integrity failure.
    #[error("PID coherence broken: syscall={syscall} procfs={procfs}")]
    PidCoherenceFailed {
        /// PID reported by the `getpid(2)` syscall.
        syscall: u32,
        /// PID parsed from `/proc/self/stat`.
        procfs: u32,
    },

    /// An I/O error occurred during the kernel anchor phase before any
    /// recovery was possible.
    #[error("I/O error during kernel anchor: {0}")]
    KernelAnchorIo(#[from] std::io::Error),
}

// ===========================================================================
// DetectionResult
// ===========================================================================

/// The complete output of a successful detection run.
///
/// All fields that could not be determined are `None`. The `confidence` field
/// explains why, via `downgrade_reasons` and `contradictions`.
///
/// NIST SP 800-53 SA-8, CM-6, SI-7.
#[derive(Debug)]
pub struct DetectionResult {
    /// OS identity derived from the package substrate (T3+), independent of
    /// the `os-release` label.
    pub substrate_identity: Option<SubstrateIdentity>,

    /// Parsed and validated `os-release` fields, if the file was found and
    /// structurally valid. Present even when `label_trust` is `LabelClaim`.
    pub os_release: Option<OsRelease>,

    /// Trust classification assigned to the `os-release` label.
    pub label_trust: LabelTrust,

    /// Boot session UUID from `/proc/sys/kernel/random/boot_id`.
    ///
    /// All evidence in this result is bound to this boot session.
    /// `None` if the kernel anchor phase could not read it.
    pub boot_id: Option<String>,

    /// Final confidence tier and any recorded contradictions or downgrade reasons.
    pub confidence: ConfidenceModel,

    /// Full provenance record for every artifact consumed during detection.
    pub evidence: EvidenceBundle,
}

// ===========================================================================
// OsDetector
// ===========================================================================

/// Configurable orchestrator for the OS detection pipeline.
///
/// Construct via `OsDetector::default()` for standard operating limits, or
/// set individual limits explicitly for constrained or extended environments.
///
/// NIST SP 800-53 SA-8, CM-6, SI-7.
pub struct OsDetector {
    /// Maximum bytes for a single bounded file read (default: 65536 / 64 KiB).
    ///
    /// Applies to os-release content and other single-value files. The limit
    /// prevents unbounded allocation if a malformed or replaced file is
    /// encountered. 64 KiB is far larger than any legitimate `os-release`.
    pub max_read_bytes: usize,

    /// Maximum bytes for the mountinfo read (default: 4,194,304 / 4 MiB).
    ///
    /// `/proc/self/mountinfo` can be large on systems with many bind mounts or
    /// inside containers. 4 MiB provides headroom while bounding allocation.
    pub max_mountinfo_bytes: usize,

    /// Maximum length of a single line in `os-release` (default: 512 bytes).
    ///
    /// No legitimate `os-release` field value exceeds 512 bytes. Lines longer
    /// than this are rejected with `OsReleaseParseError::LineTooLong`.
    pub max_line_len: usize,
}

impl Default for OsDetector {
    fn default() -> Self {
        Self {
            max_read_bytes: 65_536,
            max_mountinfo_bytes: 4_194_304,
            max_line_len: 512,
        }
    }
}

impl OsDetector {
    /// Run the full OS detection pipeline.
    ///
    /// Returns `Ok(DetectionResult)` in all cases except the two hard-gate
    /// failures defined in [`DetectionError`]. Soft failures (phase errors,
    /// contradictions) are reflected in `DetectionResult::confidence`.
    ///
    /// NIST SP 800-53 SA-8, CM-6, SI-7.
    /// NIST SP 800-53 SA-8 — orchestrates layered, fail-closed platform verification pipeline.
    pub fn detect(&self) -> Result<DetectionResult, DetectionError> {
        let mut evidence = EvidenceBundle::new();
        let mut confidence = ConfidenceModel::new();

        // ── Phase 1: Kernel Anchor (hard gate) ─────────────────────────────
        // Verifies procfs magic, PID coherence, reads boot_id, reads lockdown.
        // Returns Err on hard failure; Ok(boot_id) otherwise.
        let boot_id = kernel_anchor::run(&mut evidence, &mut confidence)?;

        // ── Phase 2: Mount Topology (soft) ─────────────────────────────────
        // Reads mount namespace, mountinfo, statfs /etc.
        // Upgrades to T2 (EnvAnchored) on success.
        mount_topology::run(&mut evidence, &mut confidence, self.max_mountinfo_bytes);

        // ── Phase 3: Release Candidate (soft) ──────────────────────────────
        // Locates os-release, records statx metadata, resolves symlink.
        let candidate = release_candidate::run(&mut evidence, &mut confidence);

        // ── Phase 4: Package Substrate (soft) ──────────────────────────────
        // Probes RPM/dpkg DB, SELinux enforce pre-check, T3 gate.
        let (substrate_identity, probe_box) =
            pkg_substrate::run(&mut evidence, &mut confidence);

        // Borrow the probe as a trait object reference for subsequent phases.
        let probe: Option<&dyn substrate::PackageProbe> =
            probe_box.as_deref();

        // ── Phase 5: File Ownership (soft) ─────────────────────────────────
        // Queries selected probe for package ownership of the candidate.
        let ownership = candidate.as_ref().and_then(|c| {
            file_ownership::run(&mut evidence, &mut confidence, c, probe)
        });

        // ── Phase 6: Integrity Check (soft) ────────────────────────────────
        // Computes SHA-256, compares against package DB digest, T4 gate.
        let integrity_ok = match candidate.as_ref() {
            Some(c) => integrity_check::run(
                &mut evidence,
                &mut confidence,
                c,
                probe,
                ownership.as_ref(),
                self.max_read_bytes,
            ),
            None => false,
        };

        // ── Phase 7: Release Parse (soft) ───────────────────────────────────
        // TPI nom + split_once parsing, substrate corroboration, LabelTrust.
        let (os_release, label_trust) = match candidate.as_ref() {
            Some(c) => release_parse::run(
                &mut evidence,
                &mut confidence,
                c,
                substrate_identity.as_ref(),
                ownership.as_ref(),
                integrity_ok,
                self.max_line_len,
            ),
            None => (None, label_trust::LabelTrust::UntrustedLabelCandidate),
        };

        Ok(DetectionResult {
            substrate_identity,
            os_release,
            label_trust,
            boot_id,
            confidence,
            evidence,
        })
    }
}
