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
//! ## Phase Duration Timing
//!
//! Each phase boundary is timed via `umrs_hw::read_hw_timestamp()`. The result
//! is stored in `DetectionResult::phase_durations` as a `Vec<PhaseDuration>`,
//! one entry per phase in execution order.
//!
//! The unit of `PhaseDuration::duration_ns` is CPU cycles on x86_64 (RDTSCP)
//! and nanoseconds on other architectures (CLOCK_MONOTONIC_RAW). On x86_64
//! the value approximates nanoseconds at ~1 cycle/ns on modern processors.
//!
//! If the hardware clock returns an anomalous result (end < start), the
//! duration is recorded as `0` and a downgrade reason is added to the
//! `ConfidenceModel`. The phase result itself is unaffected.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 SA-8**: Security Engineering Principles — the pipeline
//!   is fail-closed and layered; each phase must earn its trust tier.
//! - **NIST SP 800-53 CM-6**: Configuration Settings — the detection result
//!   provides a verified basis for configuration compliance decisions.
//! - **NIST SP 800-53 SI-7**: Software and Information Integrity — the kernel
//!   anchor and digest verification phases directly implement SI-7.
//! - **NIST SP 800-53 AU-8**: Time Stamps — `phase_durations` records
//!   per-phase elapsed time for audit record temporal analysis.

pub mod label_trust;
pub mod substrate;

pub use substrate::rpm::is_installed;

mod file_ownership;
mod integrity_check;
mod kernel_anchor;
mod mount_topology;
mod pkg_substrate;
mod release_candidate;
mod release_parse;

use thiserror::Error;

use crate::confidence::{ConfidenceModel, TrustLevel};
use crate::evidence::EvidenceBundle;
use crate::os_identity::SubstrateIdentity;
use crate::os_release::OsRelease;
use label_trust::LabelTrust;

// ===========================================================================
// DetectionPhase
// ===========================================================================

/// Identifies one of the seven phases in the OS detection pipeline.
///
/// Used as the `phase` field in [`PhaseDuration`] to associate each timing
/// measurement with the phase that produced it. Variants appear in execution
/// order.
///
/// NIST SP 800-53 AU-8 — phase identification in audit timing records.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DetectionPhase {
    /// Phase 1 — procfs magic check, PID coherence, boot_id read, lockdown read.
    KernelAnchor,

    /// Phase 2 — mount namespace, `/proc/self/mountinfo`, statfs on `/etc`.
    MountTopology,

    /// Phase 3 — os-release path probe, statx metadata, symlink resolution.
    ReleaseCandidate,

    /// Phase 4 — RPM/dpkg substrate probe, SELinux enforce Biba pre-check.
    PkgSubstrate,

    /// Phase 5 — package ownership query for the os-release candidate.
    FileOwnership,

    /// Phase 6 — SHA-256 digest computation and comparison against package DB.
    IntegrityCheck,

    /// Phase 7 — TPI nom + split_once parsing, substrate corroboration, LabelTrust.
    ReleaseParse,
}

// ===========================================================================
// PhaseDuration
// ===========================================================================

/// Timing and evidence-record delta for one detection phase.
///
/// Collected by [`OsDetector::detect`] and attached to [`DetectionResult`].
/// The `duration_ns` field records how long the phase took; `record_count`
/// records how many new [`crate::evidence::EvidenceRecord`]s the phase pushed
/// to the bundle.
///
/// The unit of `duration_ns` is CPU cycles on x86_64 (from RDTSCP) and
/// nanoseconds on other architectures (from `CLOCK_MONOTONIC_RAW`). Both are
/// suitable for relative comparison within a single detection run.
///
/// A `duration_ns` of `0` indicates a clock anomaly (end < start) was
/// detected; the corresponding [`ConfidenceModel`] will contain a downgrade
/// reason describing the anomaly.
///
/// NIST SP 800-53 AU-8 — time stamps in audit records support temporal
/// ordering and phase-interval analysis.
#[derive(Debug, Clone, Copy)]
pub struct PhaseDuration {
    /// Which phase this record covers.
    pub phase: DetectionPhase,

    /// Elapsed time in CPU cycles (x86_64) or nanoseconds (other arches).
    ///
    /// Computed as `end_ts.saturating_sub(start_ts)` where both timestamps
    /// are from `umrs_hw::read_hw_timestamp()`. Saturating subtraction
    /// prevents underflow when a non-invariant TSC produces end < start.
    ///
    /// NIST SP 800-53 AU-8 — temporal precision in audit phase records.
    pub duration_ns: u64,

    /// Number of new `EvidenceRecord`s pushed by this phase.
    ///
    /// Computed as `evidence.len()` after the phase minus `evidence.len()`
    /// before it. Uses `saturating_sub` to prevent underflow.
    ///
    /// NIST SP 800-53 AU-3 — audit record completeness: documents how many
    /// I/O events each phase produced.
    pub record_count: usize,
}

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
/// NIST SP 800-53 SA-8, CM-6, SI-7, AU-8.
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

    /// Per-phase timing and evidence-record counts for this detection run.
    ///
    /// Always contains exactly seven entries in phase-execution order when
    /// `detect()` returns `Ok`. The `duration_ns` field of each entry is the
    /// elapsed time (CPU cycles on x86_64, nanoseconds on other arches) for
    /// that phase boundary.
    ///
    /// NIST SP 800-53 AU-8 — phase duration records support temporal ordering
    /// and interval analysis in audit trails.
    pub phase_durations: Vec<PhaseDuration>,
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

// ===========================================================================
// Phase timing helper
// ===========================================================================

/// Record the duration and evidence delta for one completed phase.
///
/// If `end_ts < start_ts` (clock anomaly — e.g., non-invariant TSC after
/// core migration), records `duration_ns = 0` and adds a downgrade reason to
/// `confidence`. The phase result is unaffected; only timing data is degraded.
///
/// NIST SP 800-53 AU-8 — phase timing in audit records; clock anomaly recorded
/// in confidence model for operator visibility.
fn record_phase(
    phase: DetectionPhase,
    start_ts: u64,
    end_ts: u64,
    evidence_before: usize,
    evidence_after: usize,
    confidence: &mut ConfidenceModel,
    phase_durations: &mut Vec<PhaseDuration>,
) {
    let duration_ns = if end_ts >= start_ts {
        end_ts.saturating_sub(start_ts)
    } else {
        log::warn!(
            "detect: clock anomaly in phase {phase:?} — end_ts ({end_ts}) < start_ts ({start_ts}); \
             duration recorded as 0",
        );
        confidence.downgrade(
            TrustLevel::KernelAnchored,
            format!(
                "hardware clock anomaly in phase {phase:?}: end < start — TSC may not be invariant"
            ),
        );
        0
    };
    let record_count = evidence_after.saturating_sub(evidence_before);
    phase_durations.push(PhaseDuration {
        phase,
        duration_ns,
        record_count,
    });
}

// ===========================================================================
// OsDetector::detect
// ===========================================================================

impl OsDetector {
    /// Run the full OS detection pipeline.
    ///
    /// Returns `Ok(DetectionResult)` in all cases except the two hard-gate
    /// failures defined in [`DetectionError`]. Soft failures (phase errors,
    /// contradictions) are reflected in `DetectionResult::confidence`.
    ///
    /// `DetectionResult::phase_durations` always contains exactly seven entries
    /// on success, one per phase in execution order.
    ///
    /// NIST SP 800-53 SA-8, CM-6, SI-7, AU-8 — orchestrates a layered,
    /// fail-closed platform verification pipeline; hard gates abort on kernel
    /// channel compromise (SA-8); phase durations recorded per AU-8.
    // Line count exceeds the 100-line clippy limit because each of the seven
    // phases requires a timestamp pair, evidence-length delta, and a
    // record_phase() call — six lines of instrumentation per phase. The
    // underlying logic is still sequential and linear; the overage is
    // mechanical AU-8 instrumentation, not structural complexity.
    #[allow(clippy::too_many_lines)]
    pub fn detect(&self) -> Result<DetectionResult, DetectionError> {
        let mut evidence = EvidenceBundle::new();
        let mut confidence = ConfidenceModel::new();
        let mut phase_durations: Vec<PhaseDuration> = Vec::with_capacity(7);

        // Check TSC invariance once at pipeline start. A non-invariant TSC will
        // produce clock anomaly warnings when individual phase spans are computed.
        if !umrs_hw::tsc_is_invariant() {
            log::warn!(
                "detect: hardware TSC is not invariant — phase duration measurements may be \
                 unreliable across core migrations or C-state transitions"
            );
        }

        // ── Phase 1: Kernel Anchor (hard gate) ─────────────────────────────
        // Verifies procfs magic, PID coherence, reads boot_id, reads lockdown.
        // Returns Err on hard failure; Ok(boot_id) otherwise.
        {
            let ev_before = evidence.len();
            let t0 = umrs_hw::read_hw_timestamp();
            let boot_id_result =
                kernel_anchor::run(&mut evidence, &mut confidence);
            let t1 = umrs_hw::read_hw_timestamp();
            let ev_after = evidence.len();
            // Record timing before propagating the error — if Phase 1 fails we
            // return Err and there is no DetectionResult to attach it to, so
            // we discard the timing. This matches the spec: hard-gate abort has
            // no DetectionResult and no phase_durations.
            let _ = (t0, t1, ev_before, ev_after); // timing captured; may be discarded on Err
            let boot_id = boot_id_result?;
            record_phase(
                DetectionPhase::KernelAnchor,
                t0,
                t1,
                ev_before,
                ev_after,
                &mut confidence,
                &mut phase_durations,
            );

            // ── Phase 2: Mount Topology (soft) ─────────────────────────────
            // Reads mount namespace, mountinfo, statfs /etc.
            // Upgrades to T2 (EnvAnchored) on success.
            let ev_before2 = evidence.len();
            let t2 = umrs_hw::read_hw_timestamp();
            mount_topology::run(
                &mut evidence,
                &mut confidence,
                self.max_mountinfo_bytes,
            );
            let t3 = umrs_hw::read_hw_timestamp();
            let ev_after2 = evidence.len();
            record_phase(
                DetectionPhase::MountTopology,
                t2,
                t3,
                ev_before2,
                ev_after2,
                &mut confidence,
                &mut phase_durations,
            );

            // ── Phase 3: Release Candidate (soft) ──────────────────────────
            // Locates os-release, records statx metadata, resolves symlink.
            let ev_before3 = evidence.len();
            let t4 = umrs_hw::read_hw_timestamp();
            let candidate =
                release_candidate::run(&mut evidence, &mut confidence);
            let t5 = umrs_hw::read_hw_timestamp();
            let ev_after3 = evidence.len();
            record_phase(
                DetectionPhase::ReleaseCandidate,
                t4,
                t5,
                ev_before3,
                ev_after3,
                &mut confidence,
                &mut phase_durations,
            );

            // ── Phase 4: Package Substrate (soft) ──────────────────────────
            // Probes RPM/dpkg DB, SELinux enforce pre-check, T3 gate.
            let ev_before4 = evidence.len();
            let t6 = umrs_hw::read_hw_timestamp();
            let (substrate_identity, probe_box) =
                pkg_substrate::run(&mut evidence, &mut confidence);
            let t7 = umrs_hw::read_hw_timestamp();
            let ev_after4 = evidence.len();
            record_phase(
                DetectionPhase::PkgSubstrate,
                t6,
                t7,
                ev_before4,
                ev_after4,
                &mut confidence,
                &mut phase_durations,
            );

            // Borrow the probe as a trait object reference for subsequent phases.
            let probe: Option<&dyn substrate::PackageProbe> =
                probe_box.as_deref();

            // ── Phase 5: File Ownership (soft) ─────────────────────────────
            // Queries selected probe for package ownership of the candidate.
            let ev_before5 = evidence.len();
            let t8 = umrs_hw::read_hw_timestamp();
            let ownership = candidate.as_ref().and_then(|c| {
                file_ownership::run(&mut evidence, &mut confidence, c, probe)
            });
            let t9 = umrs_hw::read_hw_timestamp();
            let ev_after5 = evidence.len();
            record_phase(
                DetectionPhase::FileOwnership,
                t8,
                t9,
                ev_before5,
                ev_after5,
                &mut confidence,
                &mut phase_durations,
            );

            // ── Phase 6: Integrity Check (soft) ────────────────────────────
            // Computes SHA-256, compares against package DB digest, T4 gate.
            let ev_before6 = evidence.len();
            let t10 = umrs_hw::read_hw_timestamp();
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
            let t11 = umrs_hw::read_hw_timestamp();
            let ev_after6 = evidence.len();
            record_phase(
                DetectionPhase::IntegrityCheck,
                t10,
                t11,
                ev_before6,
                ev_after6,
                &mut confidence,
                &mut phase_durations,
            );

            // ── Phase 7: Release Parse (soft) ───────────────────────────────
            // TPI nom + split_once parsing, substrate corroboration, LabelTrust.
            let ev_before7 = evidence.len();
            let t12 = umrs_hw::read_hw_timestamp();
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
                None => {
                    (None, label_trust::LabelTrust::UntrustedLabelCandidate)
                }
            };
            let t13 = umrs_hw::read_hw_timestamp();
            let ev_after7 = evidence.len();
            record_phase(
                DetectionPhase::ReleaseParse,
                t12,
                t13,
                ev_before7,
                ev_after7,
                &mut confidence,
                &mut phase_durations,
            );

            Ok(DetectionResult {
                substrate_identity,
                os_release,
                label_trust,
                boot_id,
                confidence,
                evidence,
                phase_durations,
            })
        }
    }
}
