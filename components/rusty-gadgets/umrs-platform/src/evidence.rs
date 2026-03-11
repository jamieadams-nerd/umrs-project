// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//!
//! # Evidence — Provenance Records for Platform Detection
//!
//! Every artifact consumed during OS detection is recorded as an
//! [`EvidenceRecord`] and accumulated in an [`EvidenceBundle`]. Together they
//! form an immutable, ordered audit trail that can be reviewed after detection
//! completes.
//!
//! The design follows the principle that no fact should be trusted without
//! provenance: each record captures how the data was obtained (fd-anchored or
//! path-based), from which source kind, what filesystem magic was observed,
//! file metadata at the time of access, and whether parsing succeeded.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AU-3**: Audit Record Content — records capture what was
//!   read, when, from where, and with what outcome.
//! - **NIST SP 800-53 AU-10**: Non-Repudiation — the evidence bundle is the
//!   authoritative record of every artifact the detection pipeline consumed.
//!   Callers cannot remove or modify records once pushed.
//! - **NSA RTB**: provenance must be traceable. Every `EvidenceRecord` carries
//!   the path requested, the resolved path (if different), and the filesystem
//!   magic observed — enough to reconstruct the decision chain post-incident.

// ===========================================================================
// SourceKind
// ===========================================================================

/// The origin class of a single piece of detection evidence.
///
/// Used to classify each [`EvidenceRecord`] so that audit reviewers can
/// immediately identify whether a value came from the kernel (procfs/sysfs),
/// a regular filesystem file, a package database, or a resolved symlink.
///
/// NIST SP 800-53 AU-3 — audit records must identify the source of the data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SourceKind {
    /// Data read from the procfs pseudo-filesystem (`/proc/`).
    /// Provenance-verified via `PROC_SUPER_MAGIC` before any bytes are consumed.
    Procfs,

    /// Data read from a regular file on a persistent filesystem
    /// (e.g., `/etc/os-release`, `/usr/lib/os-release`).
    RegularFile,

    /// Data read from a package manager database
    /// (e.g., RPM BDB/SQLite, dpkg status file).
    PackageDb,

    /// Data obtained by resolving a symbolic link target.
    /// The resolved target path is recorded in [`EvidenceRecord::path_resolved`].
    SymlinkTarget,

    /// Data read from the sysfs pseudo-filesystem (`/sys/`).
    /// Provenance-verified via `SYSFS_MAGIC` before any bytes are consumed.
    SysfsNode,

    /// Data obtained from a `statfs(2)` syscall on a directory or path.
    ///
    /// Not a file read — records filesystem-level metadata for a path.
    /// No file descriptor is opened.
    ///
    /// NIST SP 800-53 AU-3 — audit records must correctly identify data acquisition method.
    StatfsResult,
}

// ===========================================================================
// FileStat — grouped stat metadata
// ===========================================================================

/// File metadata collected from a single `statx(2)` call.
///
/// All fields are `Option<T>` because not all filesystems or kernel versions
/// populate every `statx` field. They are grouped here (rather than as eight
/// separate `Option<uXX>` fields on `EvidenceRecord`) to make it explicit that
/// these values arrive as a unit from one syscall — or are absent entirely.
///
/// NIST SP 800-53 AU-3 — metadata is part of the audit record.
#[derive(Debug, Clone)]
pub struct FileStat {
    /// Device ID of the filesystem containing the file.
    pub dev: Option<u64>,

    /// Inode number.
    pub ino: Option<u64>,

    /// File type and permission bits (same layout as `st_mode`).
    pub mode: Option<u32>,

    /// User ID of the file owner.
    pub uid: Option<u32>,

    /// Group ID of the file owner.
    pub gid: Option<u32>,

    /// Hard link count.
    pub nlink: Option<u64>,

    /// Size in bytes.
    pub size: Option<u64>,

    /// Last modification time (seconds since Unix epoch).
    pub mtime: Option<i64>,
}

// ===========================================================================
// DigestAlgorithm / PkgDigest
// ===========================================================================

/// Hash algorithm used for a package-database digest entry.
///
/// `Md5` is included for legacy RPM database compatibility only. RPM databases
/// on older RHEL versions store MD5 digests for installed files. These are
/// flagged here as weak — callers should record the weakness and not treat
/// MD5-verified files as having strong integrity guarantees.
///
/// NIST SP 800-53 SI-7 — software integrity; algorithm selection matters.
/// CMMC L2 SI.1.210 — integrity checking for software/firmware.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DigestAlgorithm {
    /// SHA-256 — preferred algorithm.
    Sha256,

    /// SHA-512 — acceptable strong algorithm.
    Sha512,

    /// MD5 — legacy only. Present in older RPM databases.
    ///
    /// **Weak**: MD5 is cryptographically broken and must not be relied upon
    /// for security decisions. Record the digest for audit completeness but
    /// treat any file with only an MD5 reference as having unverified integrity.
    Md5,

    /// An algorithm string the parser did not recognise, preserved verbatim.
    Unknown(String),
}

/// A digest value from a package database, paired with its algorithm.
///
/// NIST SP 800-53 SI-7, SC-28 — integrity at rest: the package DB digest is
/// the reference value against which the on-disk file is compared.
#[derive(Debug, Clone)]
pub struct PkgDigest {
    /// The hash algorithm used to produce `value`.
    pub algorithm: DigestAlgorithm,

    /// Raw digest bytes, as stored in the package database.
    pub value: Vec<u8>,
}

// ===========================================================================
// EvidenceRecord
// ===========================================================================

/// Full provenance record for one artifact consumed during detection.
///
/// An `EvidenceRecord` is created for every significant I/O event in the
/// detection pipeline: each procfs read, each sysfs query, each package DB
/// probe, each symlink resolution, and each regular file read. The record is
/// pushed to the [`EvidenceBundle`] immediately after the I/O completes,
/// regardless of whether the read succeeded.
///
/// Fields that could not be populated (e.g., `stat` when `statx` was not
/// called, or `sha256` when Phase 5 did not run) are `None` — absence is
/// always explicit.
///
/// NIST SP 800-53 AU-3, AU-10 — audit record completeness and non-repudiation.
#[derive(Debug, Clone)]
pub struct EvidenceRecord {
    /// Classification of the data source.
    pub source_kind: SourceKind,

    /// Whether the file was opened via an fd-anchored call (e.g., `openat2`
    /// with `ResolveFlags`, or via `ProcfsText`/`SysfsText`). `false` means a
    /// path-based open was used — callers should note this in `notes`.
    pub opened_by_fd: bool,

    /// The path as requested by the caller before any resolution.
    pub path_requested: String,

    /// The resolved path, if `path_requested` was a symlink. `None` if the
    /// path was not a symlink or resolution was not attempted.
    pub path_resolved: Option<String>,

    /// File metadata from a `statx(2)` call on the open fd, if collected.
    /// `None` if `statx` was not called for this record.
    pub stat: Option<FileStat>,

    /// Filesystem magic observed via `fstatfs(2)` on the open fd, if verified.
    /// `None` if provenance verification was not performed (e.g., package DB).
    pub fs_magic: Option<u64>,

    /// SHA-256 digest of the file content, if computed (Phase 5 only).
    pub sha256: Option<[u8; 32]>,

    /// Digest from the package database for this path, if queried (Phase 5 only).
    pub pkg_digest: Option<PkgDigest>,

    /// Whether the content of this record was successfully parsed by the
    /// consuming phase. `false` means the read succeeded but the parse failed.
    pub parse_ok: bool,

    /// Free-form notes added by the phase runner. Must not contain security
    /// labels, credentials, or file content (NIST SP 800-53 SI-12). Strings
    /// longer than 64 characters should be truncated at the call site.
    pub notes: Vec<String>,
}

// ===========================================================================
// EvidenceBundle
// ===========================================================================

/// Ordered, append-only collection of [`EvidenceRecord`]s for one detection run.
///
/// Records are pushed in the order the detection pipeline encounters each
/// artifact. The bundle is never reordered, deduplicated, or filtered — the
/// complete sequence is the audit trail.
///
/// NIST SP 800-53 AU-3, AU-10 — the bundle is the authoritative provenance
/// record for the detection run.
#[derive(Debug, Default, Clone)]
pub struct EvidenceBundle {
    /// All records accumulated during the detection run.
    pub records: Vec<EvidenceRecord>,
}

impl EvidenceBundle {
    /// Construct an empty bundle.
    #[must_use]
    pub const fn new() -> Self {
        Self { records: Vec::new() }
    }

    /// Append a record to the bundle.
    ///
    /// Records are never reordered or removed after being pushed.
    ///
    /// NIST SP 800-53 AU-10 — records are append-only; callers cannot remove or reorder
    /// entries after push.
    pub fn push(&mut self, record: EvidenceRecord) {
        self.records.push(record);
    }

    /// Return the number of records collected so far.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.records.len()
    }

    /// Return `true` if no records have been collected yet.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
