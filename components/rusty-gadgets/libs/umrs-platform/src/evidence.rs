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
#![doc = include_str!("../docs/compliance-evidence.md")]

// ===========================================================================
// SourceKind
// ===========================================================================
/// The origin class of a single piece of detection evidence.
///
/// Used to classify each [`EvidenceRecord`] so that audit reviewers can
/// immediately identify whether a value came from the kernel (procfs/sysfs),
/// a regular filesystem file, a package database, or a resolved symlink.
///
/// ## Variants:
///
/// - `Procfs` — data read from the procfs pseudo-filesystem (`/proc/`); provenance-verified
///   via `PROC_SUPER_MAGIC` before any bytes are consumed.
/// - `RegularFile` — data read from a regular file on a persistent filesystem
///   (e.g., `/etc/os-release`, `/usr/lib/os-release`).
/// - `PackageDb` — data read from a package manager database
///   (e.g., RPM BDB/SQLite, dpkg status file).
/// - `SymlinkTarget` — data obtained by resolving a symbolic link target; the resolved path
///   is recorded in [`EvidenceRecord::path_resolved`].
/// - `SysfsNode` — data read from the sysfs pseudo-filesystem (`/sys/`); provenance-verified
///   via `SYSFS_MAGIC` before any bytes are consumed.
/// - `StatfsResult` — data obtained from a `statfs(2)` syscall on a directory or path.
///   Not a file read — records filesystem-level metadata for a path. No fd is opened.
///
/// ## Compliance
///
/// - **NIST SP 800-53 AU-3**: audit records must identify the source and acquisition method
///   of every data element.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SourceKind {
    Procfs,
    RegularFile,
    PackageDb,
    SymlinkTarget,
    SysfsNode,
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
/// ## Fields:
///
/// - `dev` — device ID of the filesystem containing the file.
/// - `ino` — inode number.
/// - `mode` — file type and permission bits (same layout as `st_mode`).
/// - `uid` — user ID of the file owner.
/// - `gid` — group ID of the file owner.
/// - `nlink` — hard link count.
/// - `size` — size in bytes.
/// - `mtime` — last modification time (seconds since Unix epoch).
///
/// ## Compliance
///
/// - **NIST SP 800-53 AU-3**: metadata is part of the audit record.
#[derive(Debug, Clone)]
pub struct FileStat {
    pub dev: Option<u64>,
    pub ino: Option<u64>,
    pub mode: Option<u32>,
    pub uid: Option<u32>,
    pub gid: Option<u32>,
    pub nlink: Option<u64>,
    pub size: Option<u64>,
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
/// ## Variants:
///
/// - `Sha256` — SHA-256; preferred algorithm.
/// - `Sha512` — SHA-512; acceptable strong algorithm.
/// - `Md5` — MD5; legacy only, present in older RPM databases. **Weak**: MD5 is
///   cryptographically broken and must not be relied upon for security decisions.
///   Record the digest for audit completeness but treat any file with only an MD5
///   reference as having unverified integrity.
/// - `Unknown(String)` — an algorithm string the parser did not recognise, preserved
///   verbatim.
///
/// ## Compliance
///
/// - **NIST SP 800-53 SI-7**: software integrity; algorithm selection matters.
/// - **CMMC L2 SI.1.210**: integrity checking for software/firmware.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DigestAlgorithm {
    Sha256,
    Sha512,
    Md5,
    Unknown(String),
}

/// A digest value from a package database, paired with its algorithm.
///
/// ## Fields:
///
/// - `algorithm` — the hash algorithm used to produce `value`.
/// - `value` — raw digest bytes, as stored in the package database.
///
/// ## Compliance
///
/// - **NIST SP 800-53 SI-7**, **SC-28**: integrity at rest — the package DB digest is the
///   reference value against which the on-disk file is compared.
#[derive(Debug, Clone)]
pub struct PkgDigest {
    pub algorithm: DigestAlgorithm,
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
/// ## Fields:
///
/// - `source_kind` — classification of the data source.
/// - `opened_by_fd` — whether the file was opened via an fd-anchored call (e.g., `openat2`
///   with `ResolveFlags`, or via `ProcfsText`/`SysfsText`). `false` means a path-based open
///   was used — callers should note this in `notes`.
/// - `path_requested` — the path as requested by the caller before any resolution.
/// - `path_resolved` — the resolved path if `path_requested` was a symlink; `None` if not a
///   symlink or resolution was not attempted.
/// - `stat` — file metadata from a `statx(2)` call on the open fd, if collected; `None` if
///   `statx` was not called for this record.
/// - `fs_magic` — filesystem magic observed via `fstatfs(2)` on the open fd, if verified;
///   `None` if provenance verification was not performed (e.g., package DB).
/// - `sha256` — SHA-256 digest of the file content, if computed (Phase 5 only).
/// - `pkg_digest` — digest from the package database for this path, if queried (Phase 5 only).
/// - `parse_ok` — whether the content of this record was successfully parsed by the consuming
///   phase. `false` means the read succeeded but the parse failed.
/// - `notes` — free-form notes added by the phase runner. Must not contain security labels,
///   credentials, or file content (NIST SP 800-53 SI-12). Strings longer than 64 characters
///   should be truncated at the call site.
/// - `duration_ns` — elapsed I/O time in CPU cycles (x86_64 RDTSCP) or nanoseconds (other
///   arches). Computed as `end_ts.saturating_sub(start_ts)`. `None` if per-record timing was
///   not captured. (NIST SP 800-53 AU-8)
///
/// ## Compliance
///
/// - **NIST SP 800-53 AU-3**, **AU-10**: audit record completeness and non-repudiation.
#[derive(Debug, Clone)]
pub struct EvidenceRecord {
    pub source_kind: SourceKind,
    pub opened_by_fd: bool,
    pub path_requested: String,
    pub path_resolved: Option<String>,
    pub stat: Option<FileStat>,
    pub fs_magic: Option<u64>,
    pub sha256: Option<[u8; 32]>,
    pub pkg_digest: Option<PkgDigest>,
    pub parse_ok: bool,
    pub notes: Vec<String>,
    pub duration_ns: Option<u64>,
}

impl Default for EvidenceRecord {
    /// Fail-closed default for `EvidenceRecord`.
    ///
    /// Sets `parse_ok` to `false` — callers must explicitly set it to `true`
    /// when the record represents a successful parse. `source_kind` defaults to
    /// `SourceKind::RegularFile` and `path_requested` to an empty string; in
    /// practice callers always override these two fields via struct update syntax:
    ///
    /// ```rust,ignore
    /// EvidenceRecord {
    ///     source_kind: SourceKind::Procfs,
    ///     path_requested: path.to_string(),
    ///     parse_ok: true,
    ///     ..Default::default()
    /// }
    /// ```
    ///
    /// `source_kind` and `path_requested` have no meaningful sentinel value —
    /// callers must always supply them explicitly.
    ///
    /// ## Compliance
    ///
    /// - NIST SP 800-53 AU-3 — audit record completeness: `parse_ok: false` is
    ///   the safe default; a record that forgets to set `parse_ok: true` is
    ///   conservatively treated as a failed parse rather than a silent success.
    fn default() -> Self {
        Self {
            source_kind: SourceKind::RegularFile,
            opened_by_fd: false,
            path_requested: String::new(),
            path_resolved: None,
            stat: None,
            fs_magic: None,
            sha256: None,
            pkg_digest: None,
            parse_ok: false,
            notes: Vec::new(),
            duration_ns: None,
        }
    }
}

// ===========================================================================
// EvidenceBundle
// ===========================================================================

/// Ordered, append-only collection of \[`EvidenceRecord`\]s for one detection run.
///
/// Records are pushed in the order the detection pipeline encounters each
/// artifact. The bundle is never reordered, deduplicated, or filtered — the
/// complete sequence is the audit trail.
///
/// The `records` field is private to enforce the AU-10 append-only invariant at
/// the type-system level: callers can only add records via \[`push`\], never remove,
/// reorder, or clear them. Use \[`records`\], \[`iter`\], or \[`is_empty`\]/\[`len`\] for
/// read access.
///
/// ## Fields:
///
/// - `records` — (private) all records accumulated during the detection run. Private to
///   enforce AU-10 append-only non-repudiation; no caller outside this module can clear,
///   pop, sort, or splice the inner `Vec`.
///
/// ## Compliance
///
/// - **NIST SP 800-53 AU-3**, **AU-10**: the bundle is the authoritative provenance record
///   for the detection run. Callers cannot remove or modify records once pushed.
#[derive(Debug, Default, Clone)]
pub struct EvidenceBundle {
    records: Vec<EvidenceRecord>,
}

impl EvidenceBundle {
    /// Construct a bundle pre-allocated for a typical detection pipeline run.
    ///
    /// Pre-allocates 32 slots — enough for all 7 phases without reallocation
    /// on a standard pipeline run (16 records at baseline). Eliminates the
    /// 4–5 incremental reallocations that `Vec::new()` would trigger.
    #[must_use = "constructed bundle must be used to accumulate evidence records"]
    pub fn new() -> Self {
        Self {
            records: Vec::with_capacity(32),
        }
    }

    /// Append a record to the bundle.
    ///
    /// Records are never reordered or removed after being pushed.
    ///
    /// ## Compliance
    ///
    /// - NIST SP 800-53 AU-10 — records are append-only; callers cannot remove or reorder
    ///   entries after push.
    pub fn push(&mut self, record: EvidenceRecord) {
        self.records.push(record);
    }

    /// Return an immutable slice of all records collected so far.
    ///
    /// ## Compliance
    ///
    /// - NIST SP 800-53 AU-10 — read-only access preserves the append-only invariant.
    #[must_use = "audit evidence records must be examined or stored — do not discard"]
    pub fn records(&self) -> &[EvidenceRecord] {
        &self.records
    }

    /// Return an iterator over all records collected so far.
    ///
    /// ## Compliance
    ///
    /// - NIST SP 800-53 AU-10 — read-only iteration; callers cannot mutate records.
    pub fn iter(&self) -> std::slice::Iter<'_, EvidenceRecord> {
        self.records.iter()
    }

    /// Return the number of records collected so far.
    #[must_use = "pure accessor — returns the number of evidence records in the bundle"]
    pub const fn len(&self) -> usize {
        self.records.len()
    }

    /// Return `true` if no records have been collected yet.
    #[must_use = "pure accessor — returns whether the evidence bundle has any records"]
    pub const fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}

impl<'a> IntoIterator for &'a EvidenceBundle {
    type Item = &'a EvidenceRecord;
    type IntoIter = std::slice::Iter<'a, EvidenceRecord>;

    fn into_iter(self) -> Self::IntoIter {
        self.records.iter()
    }
}
