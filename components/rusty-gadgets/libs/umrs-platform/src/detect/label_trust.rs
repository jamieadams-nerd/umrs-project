// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//!
//! # Label Trust — Trust Classification for the `os-release` Label
//!
//! Defines the trust level assigned to the parsed `os-release` label after
//! all detection phases complete. This enum is the pipeline's final verdict
//! on how much the `os-release` content should be trusted for policy decisions.
//!
//! Unlike [`crate::TrustLevel`], which grades the overall confidence model,
//! `LabelTrust` is specific to the os-release file itself: it answers "given
//! everything the pipeline observed, how should callers treat what `os-release`
//! claims?"
//!
//! ## Compliance
//!
//! - **NSA RTB**: trust assertions must be explicit and graded. `LabelTrust`
//!   makes it impossible for callers to treat an unverified os-release as
//!   equivalent to a cryptographically verified one — the type system enforces
//!   the distinction.
//! - **NIST SP 800-53 CM-8**: component inventory accuracy depends on how
//!   much the ID fields in `os-release` can be trusted.
//! - **NIST SP 800-53 SI-7**: software integrity verification result is
//!   reflected here as `TrustedLabel`.

/// The trust classification assigned to the `os-release` label after detection.
///
/// Variants are ordered from least to most trustworthy. Callers must not use
/// a label for security policy decisions unless it has reached at least
/// `TrustedLabel`.
///
/// ## Variants:
///
/// - `UntrustedLabelCandidate` — permissions failed sanity check, or the file is unowned
///   by any package. The label may be parsed for informational/display use, but must never
///   be used for policy decisions.
/// - `LabelClaim` — parsed successfully, but integrity could not be verified. The package
///   substrate was not probed (T3 not reached), the package DB did not own this file, or
///   the digest was unavailable. Structurally valid but provenance unconfirmed.
/// - `TrustedLabel` — T4 reached: the file is owned by a package, and the on-disk SHA-256
///   digest matches the value recorded in the package database. The label content also
///   corroborates the substrate-derived identity. The only tier safe for policy decisions.
///   (NIST SP 800-53 SI-7; CMMC L2 SI.1.210)
/// - `IntegrityVerifiedButContradictory` — T4 integrity passed (digest verified) but the
///   label content contradicts the substrate-derived identity. Treated as untrusted for
///   policy decisions. Recorded as an anomaly in the `EvidenceBundle`.
///   (NIST SP 800-53 SI-7, AU-10)
///
/// ## Compliance
///
/// - **NSA RTB**: trust must be explicit and non-forgeable from context alone.
/// - **NIST SP 800-53 CM-8**, **SI-7**: component inventory accuracy and software
///   integrity verification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LabelTrust {
    UntrustedLabelCandidate,
    LabelClaim,
    TrustedLabel,
    IntegrityVerifiedButContradictory {
        /// Brief description of the contradiction (≤64 characters at log sites).
        /// Must not contain security labels, credentials, or file content
        /// (NIST SP 800-53 SI-12).
        contradiction: String,
    },
}
