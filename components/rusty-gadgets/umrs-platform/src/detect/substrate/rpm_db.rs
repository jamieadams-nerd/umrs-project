// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//!
//! # RPM SQLite Database Access Layer
//!
//! Read-only interface to `/var/lib/rpm/rpmdb.sqlite`, the RPM package
//! database used on RHEL 10+ and Fedora.
//!
//! This module is gated by the `rpm-db` feature flag. When the feature is
//! disabled, `rpm.rs` falls back to stub behaviour.
//!
//! ## Database Structure
//!
//! The RPM SQLite DB exposes three tables relevant to this implementation:
//!
//! - `Packages(hnum INTEGER PRIMARY KEY, blob BLOB)` — one row per installed
//!   package; `blob` is the binary RPM header.
//! - `Basenames(key TEXT, hnum INTEGER)` — inverted index: file basename →
//!   `hnum`. Multiple packages can own the same basename.
//! - `Name(key TEXT, hnum INTEGER)` — inverted index: package name → `hnum`.
//!
//! ## Trust Model
//!
//! This module treats the RPM database as **untrusted input**. Every blob is
//! parsed with the TPI parser in `rpm_header`. Queries are parameterized to
//! prevent SQL injection. Read-only flags are used on open.
//!
//! ## FFI Exception
//!
//! This module depends on `rusqlite`, which uses FFI (via `libsqlite3-sys`) to
//! bind to the C `libsqlite3` library. This is a documented, accepted exception
//! to the project's "prefer pure Rust / avoid FFI" policy. Justification:
//!
//! - No production-grade pure-Rust SQLite implementation reliably reads the
//!   RHEL 10 RPM DB wire format.
//! - On RHEL 10, `libsqlite3` is compiled with distro hardening flags and
//!   covered by Red Hat's CVE response pipeline.
//! - All `unsafe` code is encapsulated inside `rusqlite` / `libsqlite3-sys`;
//!   `#![forbid(unsafe_code)]` continues to hold at the Rust source level.
//! - The connection is opened with `SQLITE_OPEN_READ_ONLY` and all queries use
//!   parameterized placeholders, minimising the active FFI surface.
//!
//! Accepted by: Jamie Adams, 2026-03-11. See also `Cargo.toml` SA-12 note.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 CM-8**: Component Inventory — the RPM DB is the
//!   authoritative component inventory for RHEL systems.
//! - **NIST SP 800-53 SA-12**: Supply Chain Risk Management — ownership
//!   queries establish file provenance; FFI exception evaluated and accepted.
//! - **NIST SP 800-53 SI-7**: Software Integrity — digest queries return the
//!   reference value used in Phase 6 (integrity_check).
//! - **NIST SP 800-53 AU-3**: Audit Record Content — all queries record
//!   provenance in `EvidenceBundle`.
//! - **NSA RTB TOCTOU**: the DB connection is opened once and reused; no
//!   second open-by-path occurs.

use std::fmt;
use std::path::Path;

use rusqlite::{Connection, OpenFlags};

use crate::evidence::{
    DigestAlgorithm, EvidenceBundle, EvidenceRecord, SourceKind,
};

use super::rpm_header::{self, RpmDigestAlgo};

/// Path to the RHEL 10+ RPM SQLite database.
pub const RPM_DB_PATH: &str = "/var/lib/rpm/rpmdb.sqlite";

// ===========================================================================
// Error type
// ===========================================================================

/// Errors produced by RPM DB operations.
///
/// Variant payloads carry only structural or type information — never file
/// content, security labels, or user data. NIST SP 800-53 SI-12.
#[derive(Debug)]
pub enum RpmDbError {
    /// A SQLite error from `rusqlite`.
    Sqlite(rusqlite::Error),
    /// A header blob parse error from `rpm_header`.
    HeaderParse(rpm_header::RpmHeaderError),
    /// An I/O error (e.g., checking DB existence).
    Io(std::io::Error),
    /// A hex-decode error when converting a digest string to bytes.
    HexDecode,
}

impl fmt::Display for RpmDbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sqlite(e) => write!(
                f,
                "rpm db sqlite error (code {})",
                e.sqlite_error_code().map_or(-1_i32, |c| c as i32)
            ),
            Self::HeaderParse(e) => write!(f, "rpm db header parse error: {e}"),
            Self::Io(e) => write!(f, "rpm db I/O error: {e}"),
            Self::HexDecode => write!(f, "rpm db digest hex decode failed"),
        }
    }
}

impl From<rusqlite::Error> for RpmDbError {
    fn from(e: rusqlite::Error) -> Self {
        Self::Sqlite(e)
    }
}

impl From<rpm_header::RpmHeaderError> for RpmDbError {
    fn from(e: rpm_header::RpmHeaderError) -> Self {
        Self::HeaderParse(e)
    }
}

impl From<std::io::Error> for RpmDbError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

// ===========================================================================
// RpmDb
// ===========================================================================

/// Read-only handle to the RPM SQLite database.
///
/// Opened with `SQLITE_OPEN_READONLY | SQLITE_OPEN_NO_MUTEX`. The connection
/// is opened once and reused for all queries — no second open-by-path.
///
/// NIST SP 800-53 CM-8, SA-12 — component inventory queries.
/// NSA RTB TOCTOU — single open, fd reuse.
pub struct RpmDb {
    // Connection does not implement Debug; we implement it manually.
    conn: Connection,
}

impl std::fmt::Debug for RpmDb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RpmDb").field("path", &RPM_DB_PATH).finish()
    }
}

impl RpmDb {
    /// Open the RPM database read-only, recording the attempt in `bundle`.
    ///
    /// Records a `PackageDb` evidence entry on both success and failure so
    /// the audit trail is complete.
    ///
    /// # Errors
    ///
    /// Returns `RpmDbError::Sqlite` if the file cannot be opened as a SQLite
    /// database, or if the `Packages` table is absent (indicating the path
    /// does not point to a valid RPM database).
    ///
    /// NIST SP 800-53 AU-3 — evidence pushed regardless of outcome.
    /// NIST SP 800-53 CM-8, SA-12.
    pub fn open(bundle: &mut EvidenceBundle) -> Result<Self, RpmDbError> {
        let flags =
            OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX;
        let conn = match Connection::open_with_flags(RPM_DB_PATH, flags) {
            Ok(c) => c,
            Err(e) => {
                log::warn!("rpm_db: failed to open {RPM_DB_PATH}: {e}");
                let rec = evidence_record(
                    RPM_DB_PATH,
                    false,
                    vec!["open failed".to_owned()],
                );
                bundle.push(rec);
                return Err(RpmDbError::Sqlite(e));
            }
        };

        // Basic structural check: the `Packages` table must exist.
        let table_exists = conn
            .query_row(
                "SELECT 1 FROM sqlite_master \
                 WHERE type='table' AND name='Packages' LIMIT 1",
                [],
                |row| row.get::<_, i64>(0),
            )
            .is_ok();

        if !table_exists {
            log::warn!("rpm_db: Packages table not found in {RPM_DB_PATH}");
            let rec = evidence_record(
                RPM_DB_PATH,
                false,
                vec!["Packages table missing — not a valid RPM db".to_owned()],
            );
            bundle.push(rec);
            return Err(RpmDbError::Sqlite(
                rusqlite::Error::QueryReturnedNoRows,
            ));
        }

        log::debug!("rpm_db: opened {RPM_DB_PATH} read-only");
        let rec = evidence_record(
            RPM_DB_PATH,
            true,
            vec!["rpm db opened read-only".to_owned()],
        );
        bundle.push(rec);

        Ok(Self {
            conn,
        })
    }

    /// Query which package owns `path`.
    ///
    /// Splits `path` into dirname + basename, queries `Basenames`, fetches
    /// and parses the owning header, then confirms the full path appears in
    /// the header's file list.
    ///
    /// Returns `(name, version, evidence_trail)` or `None` if unowned.
    ///
    /// # Errors
    ///
    /// Returns `RpmDbError::Sqlite` if the `Basenames` query fails due to a
    /// SQLite error.
    ///
    /// NIST SP 800-53 CM-8 — ownership query.
    /// NIST SP 800-53 SA-12 — provenance establishment.
    pub fn query_file_owner(
        &self,
        path: &Path,
    ) -> Result<Option<(String, String, Vec<String>)>, RpmDbError> {
        let (dirname, basename) = split_path(path);

        let mut stmt = self
            .conn
            .prepare_cached("SELECT hnum FROM Basenames WHERE key = ?1")?;
        let hnums: Vec<i64> = stmt
            .query_map([&basename], |row| row.get::<_, i64>(0))?
            .filter_map(std::result::Result::ok)
            .collect();

        if hnums.is_empty() {
            return Ok(None);
        }

        for hnum in hnums {
            let blob: Vec<u8> = match self.conn.query_row(
                "SELECT blob FROM Packages WHERE hnum = ?1",
                [hnum],
                |row| row.get::<_, Vec<u8>>(0),
            ) {
                Ok(b) => b,
                Err(e) => {
                    log::debug!(
                        "rpm_db: blob fetch for hnum={hnum} failed: {e}"
                    );
                    continue;
                }
            };

            let header = match rpm_header::parse_rpm_header(&blob) {
                Ok(h) => h,
                Err(e) => {
                    log::debug!(
                        "rpm_db: header parse for hnum={hnum} failed: {e}"
                    );
                    continue;
                }
            };

            let owns = header.files.iter().any(|f| {
                let p = Path::new(&f.full_path);
                let fdir = p
                    .parent()
                    .and_then(|d| d.to_str())
                    .map_or_else(|| "/".to_owned(), ensure_trailing_slash);
                let fbase =
                    p.file_name().and_then(|n| n.to_str()).unwrap_or("");
                fdir == dirname && fbase == basename
            });

            if owns {
                let pkg_name = header.name.unwrap_or_default();
                let pkg_ver = header.version.unwrap_or_default();
                let trail =
                    vec![format!("hnum={hnum}"), format!("pkg={pkg_name}")];
                return Ok(Some((pkg_name, pkg_ver, trail)));
            }
        }

        Ok(None)
    }

    /// Query the installed digest for `path` from the package database.
    ///
    /// Returns `(DigestAlgorithm, bytes)` or `None` if no digest record exists.
    ///
    /// # Errors
    ///
    /// Returns `RpmDbError::Sqlite` if the `Basenames` query fails, or
    /// `RpmDbError::HexDecode` if the stored digest hex string is malformed.
    ///
    /// NIST SP 800-53 SI-7 — reference digest for integrity verification.
    /// NIST SP 800-53 CM-8 — provenance record.
    pub fn query_file_digest(
        &self,
        path: &Path,
    ) -> Result<Option<(DigestAlgorithm, Vec<u8>)>, RpmDbError> {
        let (dirname, basename) = split_path(path);

        let mut stmt = self
            .conn
            .prepare_cached("SELECT hnum FROM Basenames WHERE key = ?1")?;
        let hnums: Vec<i64> = stmt
            .query_map([&basename], |row| row.get::<_, i64>(0))?
            .filter_map(std::result::Result::ok)
            .collect();

        if hnums.is_empty() {
            return Ok(None);
        }

        for hnum in hnums {
            let blob: Vec<u8> = match self.conn.query_row(
                "SELECT blob FROM Packages WHERE hnum = ?1",
                [hnum],
                |row| row.get::<_, Vec<u8>>(0),
            ) {
                Ok(b) => b,
                Err(e) => {
                    log::debug!(
                        "rpm_db: blob fetch for hnum={hnum} failed: {e}"
                    );
                    continue;
                }
            };

            let header = match rpm_header::parse_rpm_header(&blob) {
                Ok(h) => h,
                Err(e) => {
                    log::debug!(
                        "rpm_db: header parse for hnum={hnum} failed: {e}"
                    );
                    continue;
                }
            };

            for entry in &header.files {
                let p = Path::new(&entry.full_path);
                let fdir = p
                    .parent()
                    .and_then(|d| d.to_str())
                    .map_or_else(|| "/".to_owned(), ensure_trailing_slash);
                let fbase =
                    p.file_name().and_then(|n| n.to_str()).unwrap_or("");

                if fdir == dirname && fbase == basename {
                    if entry.digest_hex.is_empty() {
                        return Ok(None);
                    }
                    let algo = rpm_algo_to_digest_algo(entry.digest_algo);
                    let bytes = hex_decode(&entry.digest_hex)?;
                    return Ok(Some((algo, bytes)));
                }
            }
        }

        Ok(None)
    }

    /// Infer the specific distribution from installed release packages.
    ///
    /// Checks the `Name` table for well-known release package names in a
    /// deterministic order (most specific first). Returns the first match.
    /// Returns `None` if no recognised release package is installed.
    ///
    /// This is evidence-driven inference — it uses the actual package name
    /// rather than assuming a distro from the DB format alone.
    ///
    /// # Errors
    ///
    /// Returns `RpmDbError::Sqlite` if the `Name` table query fails.
    ///
    /// NIST SP 800-53 CM-8 — component inventory, evidence-based identity.
    pub fn infer_distro(
        &self,
    ) -> Result<Option<(crate::os_identity::Distro, String)>, RpmDbError> {
        use crate::os_identity::Distro;

        // Order: most specific first. Each entry: (package_name, Distro).
        // The first installed match wins.
        let candidates: &[(&str, Distro)] = &[
            ("centos-stream-release", Distro::CentOs),
            ("centos-release", Distro::CentOs),
            ("redhat-release", Distro::Rhel),
            ("fedora-release", Distro::Fedora),
            ("almalinux-release", Distro::AlmaLinux),
            ("rocky-release", Distro::RockyLinux),
        ];

        for &(pkg, ref distro) in candidates {
            if self.is_installed(pkg)? {
                return Ok(Some((distro.clone(), pkg.to_owned())));
            }
        }
        Ok(None)
    }

    /// Check whether a named package is installed.
    ///
    /// Queries the `Name` table directly for an exact match.
    ///
    /// # Errors
    ///
    /// Returns `RpmDbError::Sqlite` if the `Name` table query fails with an
    /// error other than `QueryReturnedNoRows` (which is treated as `false`).
    ///
    /// NIST SP 800-53 CM-8 — component inventory check.
    /// NIST SP 800-53 SA-12 — supply chain provenance.
    pub fn is_installed(&self, pkgname: &str) -> Result<bool, RpmDbError> {
        match self.conn.query_row(
            "SELECT 1 FROM Name WHERE key = ?1 LIMIT 1",
            [pkgname],
            |_row| Ok(()),
        ) {
            Ok(()) => Ok(true),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(false),
            Err(e) => Err(RpmDbError::Sqlite(e)),
        }
    }
}

// ===========================================================================
// Internal helpers
// ===========================================================================

/// Split a `Path` into (dirname_with_trailing_slash, basename).
///
/// Operates entirely on the path components — no filesystem access.
fn split_path(path: &Path) -> (String, String) {
    let basename =
        path.file_name().and_then(|n| n.to_str()).unwrap_or("").to_owned();
    let dirname = path
        .parent()
        .and_then(|d| d.to_str())
        .map_or_else(|| "/".to_owned(), ensure_trailing_slash);
    (dirname, basename)
}

/// Ensure a directory string ends with '/'. RPM DIRNAMES entries always do.
fn ensure_trailing_slash(s: &str) -> String {
    if s.ends_with('/') {
        s.to_owned()
    } else {
        format!("{s}/")
    }
}

/// Convert an `RpmDigestAlgo` to the codebase's `DigestAlgorithm`.
fn rpm_algo_to_digest_algo(algo: RpmDigestAlgo) -> DigestAlgorithm {
    match algo {
        RpmDigestAlgo::Sha256 => DigestAlgorithm::Sha256,
        RpmDigestAlgo::Sha512 => DigestAlgorithm::Sha512,
        RpmDigestAlgo::Md5 => DigestAlgorithm::Md5,
        RpmDigestAlgo::Unknown(n) => {
            DigestAlgorithm::Unknown(format!("rpm-algo-{n}"))
        }
    }
}

/// Decode a lowercase hex string into bytes. Returns `Err(HexDecode)` on any
/// non-hex character or odd-length input.
///
/// NIST SP 800-218 SSDF PW.4.1 — checked arithmetic on cursor advancement.
fn hex_decode(hex: &str) -> Result<Vec<u8>, RpmDbError> {
    if !hex.len().is_multiple_of(2) {
        return Err(RpmDbError::HexDecode);
    }
    let pair_count = hex.len() / 2;
    let mut bytes = Vec::with_capacity(pair_count);
    let chars: Vec<char> = hex.chars().collect();
    let mut i = 0usize;
    while i < chars.len() {
        let hi = *chars.get(i).ok_or(RpmDbError::HexDecode)?;
        let lo = *chars.get(i + 1).ok_or(RpmDbError::HexDecode)?;
        let hi_nibble = hex_nibble(hi).ok_or(RpmDbError::HexDecode)?;
        let lo_nibble = hex_nibble(lo).ok_or(RpmDbError::HexDecode)?;
        bytes.push((hi_nibble << 4) | lo_nibble);
        i = i.checked_add(2).ok_or(RpmDbError::HexDecode)?;
    }
    Ok(bytes)
}

/// Convert a single hex character to its nibble value.
const fn hex_nibble(c: char) -> Option<u8> {
    match c {
        '0'..='9' => Some(c as u8 - b'0'),
        'a'..='f' => Some(c as u8 - b'a' + 10),
        'A'..='F' => Some(c as u8 - b'A' + 10),
        _ => None,
    }
}

/// Build a minimal `PackageDb` evidence record for DB open results.
fn evidence_record(path: &str, ok: bool, notes: Vec<String>) -> EvidenceRecord {
    EvidenceRecord {
        source_kind: SourceKind::PackageDb,
        opened_by_fd: false,
        path_requested: path.to_owned(),
        path_resolved: None,
        stat: None,
        fs_magic: None,
        sha256: None,
        pkg_digest: None,
        parse_ok: ok,
        notes,
        duration_ns: None,
    }
}
