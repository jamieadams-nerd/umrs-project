// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//!
//! # Confidence Model — Trust Tiers for Platform Detection
//!
//! Provides the typed trust-tier system used across the OS detection pipeline
//! and any future platform module that reasons about evidence quality.
//!
//! Trust levels are monotonically ordered: `Untrusted < KernelAnchored <
//! EnvAnchored < SubstrateAnchored < IntegrityAnchored`. Confidence can only
//! decrease, never silently increase. The `ConfidenceModel::downgrade` method
//! enforces this invariant at every call site.
//!
#![doc = include_str!("../docs/compliance-confidence.md")]

//#[doc = include_str!("../docs/resolve-user.md")]
// ===========================================================================
// TrustLevel
// ===========================================================================
/// Monotonically ordered trust tier for platform detection evidence.
///
/// Confidence accumulates as successive verification gates pass. Each tier
/// represents a stronger, independently verifiable anchor. Discriminant values
/// ascend with trust, enabling `Ord`-based comparisons such as
/// `"has the model reached at least SubstrateAnchored?"`.
///
/// ## Variants:
///
/// - `Untrusted` (T0) — no kernel anchor established; default start state.
/// - `KernelAnchored` (T1) — procfs verified via `PROC_SUPER_MAGIC` + PID coherence gate.
/// - `EnvAnchored` (T2) — mount topology cross-checked (mountinfo vs statfs);
///   execution environment known.
/// - `SubstrateAnchored` (T3) — package substrate parsed; identity derived from ≥2
///   independent facts.
/// - `IntegrityAnchored` (T4) — os-release ownership + installed digest verified against
///   package DB.
///
/// ## Compliance
///
/// - **NIST SP 800-53 SA-9**, **CM-6**: trust must be explicit and graded.
/// - **NSA RTB**: every claim must be traceable to a kernel-anchored evidence source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TrustLevel {
    Untrusted = 0,
    KernelAnchored = 1,
    EnvAnchored = 2,
    SubstrateAnchored = 3,
    IntegrityAnchored = 4,
}

impl std::fmt::Display for TrustLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            Self::Untrusted => "T0 — Untrusted",
            Self::KernelAnchored => "T1 — KernelAnchored",
            Self::EnvAnchored => "T2 — EnvAnchored",
            Self::SubstrateAnchored => "T3 — SubstrateAnchored",
            Self::IntegrityAnchored => "T4 — IntegrityAnchored",
        };
        f.write_str(label)
    }
}

// ===========================================================================
// Contradiction
// ===========================================================================

/// A recorded disagreement between two independent evidence sources.
///
/// Contradictions are non-fatal — they are recorded in `ConfidenceModel` and
/// trigger a downgrade, but they do not abort the detection pipeline. The
/// caller receives both the contradiction record and the downgraded trust
/// level, enabling full audit reconstruction.
///
/// ## Compliance
///
/// - **NIST SP 800-53 AU-10**: non-repudiation — contradictions are preserved in
///   the evidence trail exactly as observed.
///
/// ## Fields:
///
/// - `source_a` — short label identifying the first source (e.g., `"SelinuxEnforce"`).
/// - `source_b` — short label identifying the second source (e.g., `"pkg_substrate"`).
/// - `description` — human-readable description of the disagreement. Must not contain
///   security labels, credentials, or file content (NIST SP 800-53 SI-12).
#[derive(Debug, Clone)]
pub struct Contradiction {
    pub source_a: &'static str,
    pub source_b: &'static str,
    pub description: String,
}

// ===========================================================================
// ConfidenceModel
// ===========================================================================

/// Accumulates trust level and recorded anomalies across all detection phases.
///
/// The model starts at `TrustLevel::Untrusted` and is upgraded phase by phase
/// as verification gates pass. It can only be downgraded via `downgrade()` —
/// never silently upgraded after the fact.
///
/// ## Compliance
///
/// - **NIST SP 800-53 SA-9**, **CM-6**: trust must be explicit, graded, and auditable.
/// - **NSA RTB RAIN**: non-bypassable — the downgrade-only invariant is enforced by
///   the API; callers cannot set `level` directly.
///
/// ## Fields:
///
/// - `level` — current trust tier; starts at `Untrusted`, raised by phase runners,
///   lowered by `downgrade()`. Read-only from outside this module.
/// - `contradictions` — all contradictions observed during detection, in encounter order.
/// - `downgrade_reasons` — human-readable reasons for each downgrade, in order;
///   paired with `level` to reconstruct the confidence trajectory.
#[derive(Debug, Clone)]
pub struct ConfidenceModel {
    level: TrustLevel,
    pub contradictions: Vec<Contradiction>,
    pub downgrade_reasons: Vec<String>,
}

impl ConfidenceModel {
    /// Construct a new model at `TrustLevel::Untrusted`.
    #[must_use = "constructed confidence model must be used to track trust tier across detection phases"]
    pub const fn new() -> Self {
        Self {
            level: TrustLevel::Untrusted,
            contradictions: Vec::new(),
            downgrade_reasons: Vec::new(),
        }
    }

    /// Return the current trust tier.
    #[must_use = "pure accessor — returns the current trust tier; discarding it loses the security posture signal"]
    pub const fn level(&self) -> TrustLevel {
        self.level
    }

    /// Raise the trust tier to `to`, if `to` is strictly higher than the current level.
    ///
    /// This is the only path that increases trust, and it only applies when the
    /// caller has just passed the verification gate that earns `to`. Calling this
    /// with a value lower than the current level is a no-op — use `downgrade` to
    /// decrease trust.
    ///
    /// Phase runner modules call this after each gate passes. It must not be
    /// called speculatively or before the gate has been verified.
    pub fn upgrade(&mut self, to: TrustLevel) {
        if to > self.level {
            self.level = to;
        }
    }

    /// Downgrade the trust tier to `to` and record the reason.
    ///
    /// If `to` is greater than or equal to the current level this is a no-op —
    /// confidence never silently improves via a downgrade call. The `reason`
    /// string is appended to `downgrade_reasons` only when an actual downgrade
    /// occurs.
    ///
    /// Must not include security labels, credentials, or file content in
    /// `reason` (NIST SP 800-53 SI-12 — information management and retention).
    pub fn downgrade(&mut self, to: TrustLevel, reason: impl Into<String>) {
        if to < self.level {
            self.level = to;
            self.downgrade_reasons.push(reason.into());
        }
    }

    /// Record a contradiction and downgrade trust.
    ///
    /// Convenience wrapper: records the `Contradiction` struct and then calls
    /// `downgrade`. The `reason` passed to `downgrade` is the same as
    /// `contradiction.description`.
    ///
    /// ## Compliance
    ///
    /// - NIST SP 800-53 AU-10 — contradictions are preserved for non-repudiation.
    pub fn record_contradiction(&mut self, contradiction: Contradiction, downgrade_to: TrustLevel) {
        let reason = contradiction.description.clone();
        self.contradictions.push(contradiction);
        self.downgrade(downgrade_to, reason);
    }
}

impl Default for ConfidenceModel {
    fn default() -> Self {
        Self::new()
    }
}
