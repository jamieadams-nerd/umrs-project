// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//!
//! # UMRS Platform — High-Assurance Kernel Attribute Access and OS Detection
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
//!
//! ## Compliance
//!
//! NIST 800-53 SI-7: Software and Information Integrity.
//! NSA RTB RAIN: Non-Bypassable security checks.
#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![deny(clippy::unwrap_used)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::missing_errors_doc)]
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
pub mod sealed_cache;

// ---------------------------------------------------------------------------
// Crate-root re-exports
// ---------------------------------------------------------------------------

// Confidence / trust model
pub use confidence::{ConfidenceModel, Contradiction, TrustLevel};

// Evidence / provenance records
pub use evidence::{
    DigestAlgorithm, EvidenceBundle, EvidenceRecord, FileStat, PkgDigest,
    SourceKind,
};

// OS identity (substrate-derived)
pub use os_identity::{
    CpuArch, Distro, KernelRelease, OsFamily, SubstrateIdentity,
};

// OS release types (validated newtypes + error)
pub use os_release::{
    BuildId, Codename, CpeName, OsId, OsName, OsRelease, OsReleaseParseError,
    OsVersion, ValidatedUrl, VariantId, VersionId,
};

// Sealed evidence cache (SEC pattern)
pub use sealed_cache::{
    CacheStatus, DEFAULT_TTL_SECS, MAX_TTL_SECS, SealedCache,
};
