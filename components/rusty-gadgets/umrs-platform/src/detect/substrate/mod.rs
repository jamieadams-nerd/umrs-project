// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//!
//! # Package Substrate Probes
//!
//! Defines the [`PackageProbe`] trait and its associated result types.
//! Concrete implementations (`rpm`, `dpkg`) are added as separate sub-modules.
//!
//! Each probe is responsible for:
//!
//! 1. Opening and validating its package database root (fail closed on I/O
//!    error or malformed header).
//! 2. Parsing minimally to prove the DB is a real package database.
//! 3. Requiring ≥2 independent facts before asserting a distribution identity.
//! 4. Never invoking external commands — all probing is pure file I/O.
//!
//! The [`PackageProbe`] trait is intentionally narrow: it expresses only the
//! three operations the pipeline needs (probe identity, query ownership, fetch
//! digest). Implementations must not exceed this surface.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 CM-7**: Least Functionality — pluggable, bounded probes
//!   that do only what the pipeline requires.
//! - **NIST SP 800-53 SA-12**: Supply Chain Risk Management — the package DB
//!   is the provenance record for installed software.
//! - **NIST SP 800-53 AU-10**: Non-Repudiation — `ProbeResult::evidence_trail`
//!   records the DB entries that proved ownership.

pub mod dpkg;
pub mod rpm;
#[cfg(feature = "rpm-db")]
pub mod rpm_db;
pub mod rpm_header;

pub(crate) use dpkg::DpkgProbe;
pub(crate) use rpm::RpmProbe;

use std::path::Path;

use thiserror::Error;

use crate::evidence::{DigestAlgorithm, EvidenceBundle, EvidenceRecord};
use crate::os_identity::SubstrateIdentity;

// ===========================================================================
// PackageQueryError
// ===========================================================================

/// Error returned by [`crate::detect::is_installed`] to distinguish failure
/// modes that a bare `bool` cannot express.
///
/// Callers that need to differentiate "package absent" from "database
/// unreadable" should match on this type. The `bool`-equivalent interpretation
/// is: `Ok(true)` = installed, `Ok(false)` = not installed,
/// `Err(_)` = query could not complete.
///
/// NIST SP 800-53 CM-8 — component inventory queries must surface read errors
/// separately from absent-package results so operators can distinguish a
/// missing package from a degraded database.
/// NIST SP 800-53 AU-3 — structured error types enable machine-readable audit
/// trail generation.
#[derive(Debug, Error)]
pub enum PackageQueryError {
    /// The package database could not be opened.
    ///
    /// The RPM database file is absent, unreadable, or the `rpm-db` feature
    /// is not compiled in. The query could not be attempted.
    #[error(
        "RPM database unavailable — cannot query package installation status"
    )]
    DatabaseUnavailable,

    /// The database opened successfully but the query itself failed.
    ///
    /// The database may be corrupt, locked, or the query encountered an
    /// unexpected schema. The result is indeterminate.
    #[error(
        "package installation query failed — database may be corrupt or locked"
    )]
    QueryFailed,
}

// ===========================================================================
// ProbeResult
// ===========================================================================

/// The outcome of a single package-probe attempt.
///
/// A `ProbeResult` is always produced — even for failed probes. The
/// `parse_ok` flag indicates whether the DB was readable and structurally
/// valid. Callers must check `parse_ok` before relying on `identity`.
///
/// NIST SP 800-53 CM-8, SA-12.
#[derive(Debug)]
pub struct ProbeResult {
    /// Short identifier for the probe implementation (e.g., `"rpm"`, `"dpkg"`).
    pub probe_name: &'static str,

    /// Whether the package database was opened and structurally validated.
    pub parse_ok: bool,

    /// Whether this probe can answer file ownership queries.
    pub can_query_ownership: bool,

    /// Whether this probe can return installed file digests.
    pub can_verify_digest: bool,

    /// Substrate-derived identity, if `parse_ok` is `true` and ≥2 facts were
    /// corroborated. `None` if the probe failed or insufficient facts found.
    pub identity: Option<SubstrateIdentity>,

    /// Provenance record for the probe attempt itself.
    pub evidence: EvidenceRecord,
}

// ===========================================================================
// FileOwnership
// ===========================================================================

/// A package's claimed ownership of a specific file.
///
/// Returned by `PackageProbe::query_ownership`. The `evidence_trail` records
/// which DB entries were used to establish the claim, enabling post-incident
/// reconstruction (NIST SP 800-53 AU-10).
#[derive(Debug, Clone)]
pub struct FileOwnership {
    /// Name of the package that claims ownership of the file.
    pub package_name: String,

    /// Version string of the owning package.
    pub package_version: String,

    /// DB record references that proved ownership. Must not contain file
    /// content or security labels (NIST SP 800-53 SI-12).
    pub evidence_trail: Vec<String>,
}

// ===========================================================================
// InstalledDigest
// ===========================================================================

/// The digest of a file as recorded in the package database.
///
/// This is the reference value used in Phase 5 (`integrity_check`) to verify
/// the on-disk file has not been tampered with.
///
/// NIST SP 800-53 SI-7, SC-28 — integrity at rest.
/// CMMC L2 SI.1.210 — integrity checking for software/firmware.
#[derive(Debug, Clone)]
pub struct InstalledDigest {
    /// The path this digest was recorded against in the package DB.
    pub path: String,

    /// The hash algorithm used to produce `value`.
    pub algorithm: DigestAlgorithm,

    /// Raw digest bytes as stored in the package database.
    pub value: Vec<u8>,
}

// ===========================================================================
// PackageProbe trait
// ===========================================================================

/// Contract for a pluggable package substrate probe.
///
/// Implementations must:
///
/// - Open their DB roots using fd-anchored I/O (or path-based only where no
///   fd API exists, with the limitation recorded in the `EvidenceRecord`).
/// - Parse minimally to prove the DB is structurally valid.
/// - Require ≥2 independent facts before asserting distribution identity.
/// - Never invoke external commands.
/// - Be `Send + Sync` to support future parallel probing.
///
/// NIST SP 800-53 CM-7: Least Functionality.
/// NIST SP 800-53 SA-12: Supply Chain Risk Management.
pub trait PackageProbe: Send + Sync {
    /// Attempt to open and validate the package database.
    ///
    /// Returns a `ProbeResult` regardless of success — the caller decides
    /// what to do with a failed probe. Evidence is always pushed to `bundle`.
    fn probe(&self, bundle: &mut EvidenceBundle) -> ProbeResult;

    /// Query which package owns the file identified by `(dev, ino, path)`.
    ///
    /// Using `dev` + `ino` rather than path alone prevents TOCTOU: the caller
    /// opens the file first, records `(dev, ino)` from `statx`, then calls
    /// this method. The probe must verify the path and inode agree.
    ///
    /// Returns `None` if the file is unowned, the probe lacks ownership
    /// query capability, or an error occurs — fail closed.
    fn query_ownership(
        &self,
        dev: u64,
        ino: u64,
        path: &Path,
    ) -> Option<FileOwnership>;

    /// Fetch the expected installed digest for `path` from the package DB.
    ///
    /// Returns `None` if the path has no digest record, the probe cannot
    /// provide digests, or an error occurs — fail closed.
    fn installed_digest(&self, path: &Path) -> Option<InstalledDigest>;
}
