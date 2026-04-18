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

pub use substrate::PackageQueryError;
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
use crate::os_identity::{KernelRelease, SubstrateIdentity};
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
/// ## Variants:
///
/// - `KernelAnchor` — Phase 1: procfs magic check, PID coherence, boot_id read,
///   lockdown read.
/// - `MountTopology` — Phase 2: mount namespace, `/proc/self/mountinfo`, statfs on `/etc`.
/// - `ReleaseCandidate` — Phase 3: os-release path probe, statx metadata, symlink
///   resolution.
/// - `PkgSubstrate` — Phase 4: RPM/dpkg substrate probe, SELinux enforce Biba pre-check.
/// - `FileOwnership` — Phase 5: package ownership query for the os-release candidate.
/// - `IntegrityCheck` — Phase 6: SHA-256 digest computation and comparison against
///   package DB.
/// - `ReleaseParse` — Phase 7: TPI nom + split_once parsing, substrate corroboration,
///   LabelTrust.
///
/// ## Compliance
///
/// - **NIST SP 800-53 AU-8**: phase identification in audit timing records.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DetectionPhase {
    KernelAnchor,
    MountTopology,
    ReleaseCandidate,
    PkgSubstrate,
    FileOwnership,
    IntegrityCheck,
    ReleaseParse,
}

impl DetectionPhase {
    /// Returns a human-readable label for this phase, suitable for audit log
    /// entries and debug output.
    ///
    /// The returned string is a `'static` slice — no allocation occurs. This
    /// method is designed for use in `log::debug!()` instrumentation where
    /// phase names must be emitted without carrying kernel attribute values or
    /// security label data (NIST SP 800-53 SI-11 — Error Information Discipline).
    ///
    /// ## Compliance
    ///
    /// - NIST SP 800-53 AU-8 — phase label appears in per-phase timing records
    ///   to associate each duration measurement with its originating pipeline stage.
    #[must_use = "phase name is used to label audit timing records"]
    pub const fn name(self) -> &'static str {
        match self {
            Self::KernelAnchor => "kernel_anchor",
            Self::MountTopology => "mount_topology",
            Self::ReleaseCandidate => "release_candidate",
            Self::PkgSubstrate => "pkg_substrate",
            Self::FileOwnership => "file_ownership",
            Self::IntegrityCheck => "integrity_check",
            Self::ReleaseParse => "release_parse",
        }
    }
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
/// ## Fields:
///
/// - `phase` — which phase this record covers.
/// - `duration_ns` — elapsed time in CPU cycles (x86_64) or nanoseconds (other arches).
///   Computed as `end_ts.saturating_sub(start_ts)` from `umrs_hw::read_hw_timestamp()`.
///   Saturating subtraction prevents underflow on a non-invariant TSC.
///   (NIST SP 800-53 AU-8)
/// - `record_count` — number of new `EvidenceRecord`s pushed by this phase.
///   Computed as `evidence.len()` delta using `saturating_sub` to prevent underflow.
///   (NIST SP 800-53 AU-3)
///
/// ## Compliance
///
/// - **NIST SP 800-53 AU-8**: time stamps in audit records support temporal ordering and
///   phase-interval analysis.
#[derive(Debug, Clone, Copy)]
pub struct PhaseDuration {
    pub phase: DetectionPhase,
    pub duration_ns: u64,
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
/// ## Variants:
///
/// - `ProcfsNotReal` — procfs failed the `PROC_SUPER_MAGIC` check. No kernel-anchored
///   fact can be established; continuing would produce a result with no verifiable basis.
/// - `PidCoherenceFailed` — the PID from `getpid(2)` does not match the PID parsed from
///   `/proc/self/stat`. Fields `syscall` (getpid result) and `procfs` (parsed PID) are
///   process IDs only — not sensitive data. (NIST SP 800-53 SI-7)
/// - `KernelAnchorIo` — an I/O error occurred during the kernel anchor phase before
///   any recovery was possible.
///
/// ## Compliance
///
/// - **NIST SP 800-53 SA-8**, **SI-7**: fail-closed on kernel channel compromise.
#[derive(Debug, Error)]
pub enum DetectionError {
    #[error("procfs is not real procfs — cannot establish kernel anchor")]
    ProcfsNotReal,

    #[error("PID coherence broken: syscall={syscall} procfs={procfs}")]
    PidCoherenceFailed {
        syscall: u32,
        procfs: u32,
    },

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
/// ## Fields:
///
/// - `substrate_identity` — OS identity derived from the package substrate (T3+),
///   independent of the `os-release` label.
/// - `os_release` — parsed and validated `os-release` fields, if the file was found
///   and structurally valid. Present even when `label_trust` is `LabelClaim`.
/// - `label_trust` — trust classification assigned to the `os-release` label.
/// - `boot_id` — boot session UUID from `/proc/sys/kernel/random/boot_id`. All evidence
///   is bound to this session. `None` if the kernel anchor phase could not read it.
/// - `kernel_release` — kernel release string from `/proc/sys/kernel/osrelease`. Present
///   at T1+; `None` only if the read failed after the anchor was established.
///   (NIST SP 800-53 CM-8)
/// - `confidence` — final confidence tier and any recorded contradictions or downgrade
///   reasons.
/// - `evidence` — full provenance record for every artifact consumed during detection.
/// - `phase_durations` — per-phase timing and evidence-record counts; always seven entries
///   in phase-execution order when `detect()` returns `Ok`.
///   (NIST SP 800-53 AU-8)
///
/// ## Compliance
///
/// - **NIST SP 800-53 SA-8**, **CM-6**, **SI-7**, **AU-8**.
#[derive(Debug)]
pub struct DetectionResult {
    pub substrate_identity: Option<SubstrateIdentity>,
    pub os_release: Option<OsRelease>,
    pub label_trust: LabelTrust,
    pub boot_id: Option<String>,
    pub kernel_release: Option<KernelRelease>,
    pub confidence: ConfidenceModel,
    pub evidence: EvidenceBundle,
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
/// ## Fields:
///
/// - `max_read_bytes` — maximum bytes for a single bounded file read (default: 65536 / 64 KiB).
///   Prevents unbounded allocation if a malformed or replaced file is encountered.
/// - `max_mountinfo_bytes` — maximum bytes for the mountinfo read (default: 4,194,304 / 4 MiB).
///   `/proc/self/mountinfo` can be large on systems with many bind mounts or inside containers.
/// - `max_line_len` — maximum length of a single line in `os-release` (default: 512 bytes).
///   Lines longer than this are rejected with `OsReleaseParseError::LineTooLong`.
///
/// ## Compliance
///
/// - **NIST SP 800-53 SA-8**, **CM-6**, **SI-7**.
pub struct OsDetector {
    pub max_read_bytes: usize,
    pub max_mountinfo_bytes: usize,
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
    /// ## Compliance
    ///
    /// - NIST SP 800-53 SA-8, CM-6, SI-7, AU-8 — orchestrates a layered,
    ///   fail-closed platform verification pipeline; hard gates abort on kernel
    ///   channel compromise (SA-8); phase durations recorded per AU-8.
    ///
    /// # Errors
    ///
    /// Returns `DetectionError` if a hard-gate phase (Phase 1: Kernel Anchor
    /// or Phase 2: SELinux Channel) fails — for example, if `/proc` or
    /// `/sys/fs/selinux` cannot be opened, the filesystem magic does not match,
    /// or the kernel reports an unexpected state. Later phases (OS Release,
    /// RPM substrate) degrade confidence but do not return `Err`.
    // Line count exceeds the 100-line clippy limit because each of the seven
    // phases requires a timestamp pair, evidence-length delta, and a
    // record_phase() call — six lines of instrumentation per phase. The
    // underlying logic is still sequential and linear; the overage is
    // mechanical AU-8 instrumentation, not structural complexity.
    #[expect(
        clippy::too_many_lines,
        reason = "line-count overage is AU-8 instrumentation (timestamp pairs, evidence deltas, phase records) — six lines per phase"
    )]
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
            let boot_id_result = kernel_anchor::run(&mut evidence, &mut confidence);
            let t1 = umrs_hw::read_hw_timestamp();
            let ev_after = evidence.len();
            // Record timing before propagating the error — if Phase 1 fails we
            // return Err and there is no DetectionResult to attach it to, so
            // we discard the timing. This matches the spec: hard-gate abort has
            // no DetectionResult and no phase_durations.
            let _ = (t0, t1, ev_before, ev_after); // timing captured; may be discarded on Err
            let (boot_id, kernel_release) = boot_id_result?;
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
            mount_topology::run(&mut evidence, &mut confidence, self.max_mountinfo_bytes);
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
            let candidate = release_candidate::run(&mut evidence, &mut confidence);
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
            let probe: Option<&dyn substrate::PackageProbe> = probe_box.as_deref();

            // ── Phase 5: File Ownership (soft) ─────────────────────────────
            // Queries selected probe for package ownership of the candidate.
            let ev_before5 = evidence.len();
            let t8 = umrs_hw::read_hw_timestamp();
            let ownership = candidate
                .as_ref()
                .and_then(|c| file_ownership::run(&mut evidence, &mut confidence, c, probe));
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
                None => (None, label_trust::LabelTrust::UntrustedLabelCandidate),
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

            // ── Debug instrumentation — per-phase timing summary ────────────
            // Emits phase name, duration, and evidence-record count at debug
            // level. release_max_level_info compiles these to no-ops in release
            // builds (log feature flag in Cargo.toml).
            //
            // Error Information Discipline (NIST SP 800-53 SI-11): only static
            // phase name strings and numeric timing values are logged — no
            // kernel attribute values, configuration file contents, or security
            // label data appear here.
            for pd in &phase_durations {
                log::debug!(
                    "[detect_pipeline] phase={phase} duration_ns={dur} records={rec}",
                    phase = pd.phase.name(),
                    dur = pd.duration_ns,
                    rec = pd.record_count,
                );
            }
            let total_ns: u64 =
                phase_durations.iter().map(|pd| pd.duration_ns).fold(0u64, u64::saturating_add);
            log::debug!(
                "[detect_pipeline] total_duration_ns={total} phases={count}",
                total = total_ns,
                count = phase_durations.len(),
            );

            Ok(DetectionResult {
                substrate_identity,
                os_release,
                label_trust,
                boot_id,
                kernel_release,
                confidence,
                evidence,
                phase_durations,
            })
        }
    }
}
