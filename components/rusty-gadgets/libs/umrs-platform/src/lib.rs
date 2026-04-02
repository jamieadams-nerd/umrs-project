// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//!
//! # UMRS Platform — High-Assurance Kernel Attribute Access and OS Detection
//!
//! For file-level security context operations (reading SELinux labels, parsing
//! security contexts, MLS/MCS label handling), see the `umrs-selinux` crate.
//! This crate handles kernel attribute access and OS detection; `umrs-selinux`
//! handles the SELinux policy and label layer.
//!
//! This crate provides platform-level infrastructure for:
//!
//! - Interacting with Linux kernel pseudo-filesystems (`/sys/fs/selinux`,
//!   `/proc`, `/sys/kernel/security`) in a provenance-verified, fail-closed
//!   manner.
//! - Detecting and verifying OS identity through a multi-phase pipeline that
//!   anchors to the kernel, cross-checks mount topology, probes the package
//!   substrate, and verifies file integrity against package DB digests.
//!
//! ## Module Layout
//!
//! | Module | Contents |
//! |---|---|
//! | `kattrs` | Kernel attribute reader engine, selinuxfs/procfs/sysfs/securityfs types |
//! | `confidence` | `TrustLevel`, `ConfidenceModel`, `Contradiction` |
//! | `evidence` | `EvidenceRecord`, `EvidenceBundle`, `FileStat`, `SourceKind` |
//! | `os_identity` | `OsFamily`, `Distro`, `KernelRelease`, `CpuArch`, `SubstrateIdentity` |
//! | `os_release` | `OsRelease` and all validated field newtypes |
//! | `detect` | `OsDetector`, `DetectionResult`, `DetectionError`, phase modules |
//! | `posture` | `PostureSnapshot`, `IndicatorReport`, `IndicatorId`, `AssuranceImpact`, `ContradictionKind`, `FipsCrossCheck`, `ModprobeConfig` |
//! | `sealed_cache` | `SealedCache`, `CacheStatus`, `DEFAULT_TTL_SECS`, `MAX_TTL_SECS` — SEC pattern |
//! | `timestamp` | `BootSessionTimestamp`, `BootSessionDuration`, `TimestampError` — nanosecond audit ordering |
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 SI-7**: Software and Information Integrity — provenance
//!   verification via fd-anchored `fstatfs` before every kernel attribute read.
//! - **NIST SP 800-53 CA-7**: Continuous Monitoring — `PostureSnapshot` provides
//!   point-in-time kernel security posture with typed contradiction detection.
//! - **NIST SP 800-53 CM-6**: Configuration Settings — posture probe detects
//!   live-vs-configured divergence; Trust Gate prevents reads when subsystem
//!   is inactive.
//! - **NIST SP 800-53 SC-12, SC-28**: Key Management and Protection at Rest —
//!   `SealedCache` uses HMAC-SHA-256 with an ephemeral, boot-session-bound key
//!   that is zeroized on drop.
//! - **NIST SP 800-53 AU-3**: Audit Record Content — `EvidenceBundle` records
//!   what was read, from where, and under what filesystem magic; `ContradictionKind`
//!   is a typed enum enabling machine-readable audit classification.
//! - **NIST SP 800-218 SSDF PW.4**: Secure Coding — compile-time path and magic
//!   binding prevents runtime parameterization of security-critical constants.
//! - **NSA RTB RAIN**: Non-Bypassable — all kernel attribute reads route through
//!   `SecureReader`; the magic check cannot be skipped.
#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![deny(clippy::unwrap_used)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::option_if_let_else)]

// ---------------------------------------------------------------------------
// Module declarations
// ---------------------------------------------------------------------------

pub mod confidence;
pub mod detect;
pub mod evidence;
pub mod kattrs;
pub mod os_identity;
pub mod os_release;
pub mod posture;
pub mod sealed_cache;
pub mod timestamp;

// ---------------------------------------------------------------------------
// Crate-root re-exports
// ---------------------------------------------------------------------------

// Confidence / trust model
pub use confidence::{ConfidenceModel, Contradiction, TrustLevel};

// Evidence / provenance records
pub use evidence::{
    DigestAlgorithm, EvidenceBundle, EvidenceRecord, FileStat, PkgDigest, SourceKind,
};

// Detection phase timing
pub use detect::{DetectionPhase, PackageQueryError, PhaseDuration};

// OS identity (substrate-derived)
pub use os_identity::{
    CpuArch, Distro, KernelRelease, KernelVersion, KernelVersionParseError, OsFamily,
    SubstrateIdentity,
};

// OS release types (validated newtypes + error)
pub use os_release::{
    BuildId, Codename, CpeName, OsId, OsName, OsRelease, OsReleaseParseError, OsVersion,
    ValidatedUrl, VariantId, VersionId,
};

// Sealed evidence cache (SEC pattern)
pub use sealed_cache::{CacheStatus, DEFAULT_TTL_SECS, MAX_TTL_SECS, SealedCache};

// Posture probe
pub use posture::{AssuranceImpact, IndicatorId, IndicatorReport, PostureSnapshot};

// Boot-session timestamps (nanosecond-precision monotonic ordering)
pub use timestamp::{BootSessionDuration, BootSessionTimestamp, TimestampError};
