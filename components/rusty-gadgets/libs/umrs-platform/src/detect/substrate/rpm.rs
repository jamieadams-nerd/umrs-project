// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//!
//! # RPM Package Substrate Probe
//!
//! [`PackageProbe`] implementation for RPM-based distributions (RHEL, Fedora,
//! CentOS). When the `rpm-db` feature is enabled, this probe opens the SQLite
//! RPM database at `/var/lib/rpm/rpmdb.sqlite` and provides full file-ownership
//! and digest-query capability.
//!
//! ## Feature gate
//!
//! The `rpm-db` feature controls whether SQLite queries are available.
//! Without it, the probe reverts to stub behaviour: presence checks only,
//! `can_query_ownership = false`, `can_verify_digest = false`.
//!
//! ## Trust model
//!
//! The RPM database is treated as **untrusted input**. Every header blob is
//! parsed by the TPI parser in `rpm_header` (two independent paths; fail
//! closed on disagreement). The DB is opened read-only with no shared-cache
//! mutex. Ownership queries additionally verify `(dev, ino)` against a
//! re-stat of the path to detect TOCTOU races between the caller's open and
//! our DB query.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 CM-7**: Least Functionality — the probe exposes only
//!   the three operations the pipeline needs.
//! - **NIST SP 800-53 CM-8**: Component Inventory — file ownership queries
//!   establish which package owns each detected file.
//! - **NIST SP 800-53 SA-12**: Supply Chain Risk Management — the RPM
//!   database is the primary provenance record for RHEL/Fedora systems.
//! - **NIST SP 800-53 SI-7**: Software Integrity — digest lookup enables the
//!   pipeline to verify on-disk files against their package DB reference.
//! - **NSA RTB TOCTOU**: ownership queries re-verify `(dev, ino)` before
//!   returning a result.

use std::path::Path;
use std::sync::Mutex;

use nix::sys::statfs::statfs;

use crate::evidence::{EvidenceBundle, EvidenceRecord, SourceKind};
use crate::os_identity::{Distro, OsFamily, SubstrateIdentity};

use super::{FileOwnership, InstalledDigest, PackageProbe, PackageQueryError, ProbeResult};

#[cfg(feature = "rpm-db")]
use super::rpm_db::RpmDb;

// ===========================================================================
// Constants
// ===========================================================================

/// RPM database root directory.
const RPM_DB_ROOT: &str = "/var/lib/rpm";

/// RHEL 8/9 BDB Packages file path.
const RPM_PACKAGES_BDB: &str = "/var/lib/rpm/Packages";

/// RHEL 10+ SQLite database file path.
const RPM_PACKAGES_SQLITE: &str = "/var/lib/rpm/rpmdb.sqlite";

/// tmpfs filesystem magic number (linux/magic.h `TMPFS_MAGIC`).
const TMPFS_MAGIC: i64 = 0x0102_1994;

// ===========================================================================
// RpmProbe
// ===========================================================================

/// RPM package substrate probe.
///
/// ## Fields:
///
/// - `db` — lazily-opened RPM database handle. `Mutex` allows `&self` access from the
///   `PackageProbe` trait while still initialising on first use. `None` until `probe()`
///   succeeds in opening the DB. (feature-gated on `rpm-db`)
///
/// ## Compliance
///
/// - **NIST SP 800-53 CM-7**, **CM-8**, **SA-12**, **SI-7**.
pub struct RpmProbe {
    #[cfg(feature = "rpm-db")]
    db: Mutex<Option<RpmDb>>,
}

impl RpmProbe {
    /// Construct a new `RpmProbe`.
    #[must_use = "constructed probe must be used to run the RPM substrate detection phase"]
    pub const fn new() -> Self {
        Self {
            #[cfg(feature = "rpm-db")]
            db: Mutex::new(None),
        }
    }
}

impl Default for RpmProbe {
    fn default() -> Self {
        Self::new()
    }
}

// ===========================================================================
// PackageProbe implementation
// ===========================================================================

impl PackageProbe for RpmProbe {
    fn probe(&self, bundle: &mut EvidenceBundle) -> ProbeResult {
        probe_inner(self, bundle)
    }

    fn query_ownership(&self, dev: u64, ino: u64, path: &Path) -> Option<FileOwnership> {
        query_ownership_inner(self, dev, ino, path)
    }

    fn installed_digest(&self, path: &Path) -> Option<InstalledDigest> {
        installed_digest_inner(self, path)
    }
}

// ===========================================================================
// Implementation functions (extracted to stay under the line limit)
// ===========================================================================

/// Execute the probe phase — check presence, statfs, optionally open DB.
fn probe_inner(probe: &RpmProbe, bundle: &mut EvidenceBundle) -> ProbeResult {
    // Cache result once to avoid micro-TOCTOU from double `.exists()`.
    let root_present = Path::new(RPM_DB_ROOT).exists();
    if !root_present {
        log::debug!("rpm_probe: /var/lib/rpm not found — not an RPM system");
        let rec = no_db_record(RPM_DB_ROOT, "RPM DB root not present");
        bundle.push(rec.clone());
        return ProbeResult {
            probe_name: "rpm",
            parse_ok: false,
            can_query_ownership: false,
            can_verify_digest: false,
            identity: None,
            evidence: rec,
        };
    }

    // statfs DB root to detect tmpfs substitution.
    let fs_magic_opt: Option<u64> = match statfs(RPM_DB_ROOT) {
        Ok(stat) => {
            let magic = stat.filesystem_type().0;
            if magic == TMPFS_MAGIC {
                log::warn!("rpm_probe: {RPM_DB_ROOT} is on tmpfs — may not be a real RPM database");
            }
            Some(magic.cast_unsigned())
        }
        Err(e) => {
            log::debug!("rpm_probe: statfs({RPM_DB_ROOT}) failed: {e}");
            None
        }
    };

    let mut identity = SubstrateIdentity {
        family: OsFamily::RpmBased,
        distro: None,
        version_id: None,
        facts_count: 0,
        probe_used: "rpm",
    };

    // Fact 1: RPM DB root is present.
    identity.add_fact();

    let mut notes = vec!["RPM DB root present: /var/lib/rpm".to_owned()];
    if let Some(magic) = fs_magic_opt {
        notes.push(format!("db_root_fs_magic={magic:#x}"));
    }

    // Check for Packages database file (fact 2).
    let sqlite_present = Path::new(RPM_PACKAGES_SQLITE).exists();
    let bdb_present = Path::new(RPM_PACKAGES_BDB).exists();

    if sqlite_present || bdb_present {
        identity.add_fact();
        let which = if sqlite_present {
            "rpmdb.sqlite (RHEL10+)"
        } else {
            "Packages (BDB, RHEL8/9)"
        };
        notes.push(format!("Packages file present: {which}"));

        if sqlite_present {
            // Tentative — refined by infer_distro() after DB opens.
            identity.distro = Some(Distro::Rhel);
            notes.push("distro tentative: RPM-based (SQLite DB)".to_owned());
        }
    } else {
        notes.push("Packages DB file not found (partial probe)".to_owned());
    }

    // Attempt to open the DB when rpm-db feature is enabled.
    let (can_query, can_digest) =
        try_open_db(probe, bundle, sqlite_present, &mut identity, &mut notes);

    // Emit a loud warning if T3 upgrade gate is reached without ownership capability.
    if !can_query {
        log::warn!(
            "rpm_probe: T3 upgrade gate reached with can_query_ownership=false — \
             ownership and digest verification unavailable"
        );
    }

    log::debug!(
        "rpm_probe: facts_count={}, can_query={can_query}, can_digest={can_digest}",
        identity.facts_count,
    );

    let ev = EvidenceRecord {
        source_kind: SourceKind::PackageDb,
        path_requested: RPM_DB_ROOT.to_owned(),
        fs_magic: fs_magic_opt,
        parse_ok: true,
        notes,
        ..Default::default()
    };
    bundle.push(ev.clone());

    ProbeResult {
        probe_name: "rpm",
        parse_ok: true,
        can_query_ownership: can_query,
        can_verify_digest: can_digest,
        identity: Some(identity),
        evidence: ev,
    }
}

/// Attempt to open the RPM DB. Returns `(can_query, can_digest)`.
///
/// When the `rpm-db` feature is disabled both values are always `false`.
fn try_open_db(
    #[cfg(feature = "rpm-db")] probe: &RpmProbe,
    #[cfg(not(feature = "rpm-db"))] _probe: &RpmProbe,
    bundle: &mut EvidenceBundle,
    sqlite_present: bool,
    identity: &mut SubstrateIdentity,
    notes: &mut Vec<String>,
) -> (bool, bool) {
    #[cfg(feature = "rpm-db")]
    if sqlite_present {
        if let Ok(mut guard) = probe.db.lock() {
            if guard.is_none() {
                match RpmDb::open(bundle) {
                    Ok(db) => {
                        // Refine distro from release package evidence.
                        if let Ok(Some((distro, pkg))) = db.infer_distro() {
                            notes.push(format!("distro refined: {distro:?} (from {pkg})"));
                            identity.distro = Some(distro);
                        }
                        *guard = Some(db);
                        identity.add_fact();
                        notes.push("RPM SQLite DB opened and validated".to_owned());
                        return (true, true);
                    }
                    Err(e) => {
                        log::warn!("rpm_probe: DB open failed: {e}");
                        notes.push("RPM SQLite DB open failed".to_owned());
                        return (false, false);
                    }
                }
            }
            // Already opened in a prior call.
            return (true, true);
        }
        log::warn!("rpm_probe: mutex poisoned");
    }

    #[cfg(not(feature = "rpm-db"))]
    {
        let _ = (bundle, sqlite_present, identity, notes);
    }

    (false, false)
}

/// Handle a `query_ownership` call.
fn query_ownership_inner(
    #[cfg(feature = "rpm-db")] probe: &RpmProbe,
    #[cfg(not(feature = "rpm-db"))] _probe: &RpmProbe,
    dev: u64,
    ino: u64,
    path: &Path,
) -> Option<FileOwnership> {
    #[cfg(feature = "rpm-db")]
    {
        let Ok(guard) = probe.db.lock() else {
            log::warn!("rpm_probe: mutex poisoned in query_ownership");
            return None;
        };
        let db = guard.as_ref()?;

        match db.query_file_owner(path) {
            Ok(Some((pkg_name, pkg_ver, trail))) => {
                // TOCTOU verification: re-stat the path and confirm (dev, ino) match.
                // Detects races between the caller's open and our DB query.
                //
                // NSA RTB TOCTOU — re-verify inode identity after query.
                //
                // BUG EXPLANATION — device number encoding mismatch:
                //
                // `release_candidate.rs` calls `rustix::fs::statx`, which returns the
                // device number as separate `stx_dev_major` and `stx_dev_minor` u32 fields.
                // `FileStat.dev` stores them combined as `(major as u64) << 32 | (minor as u64)`.
                //
                // `nix::sys::stat::stat()` here returns `st_dev` as a Linux `dev_t` — the
                // kernel's compact encoding: `makedev(major, minor)` uses bit-packing
                // (e.g., for 253:0 → `(253 << 8) | 0 = 64768`), NOT the same layout.
                //
                // Comparing `st_dev` directly against `FileStat.dev` would always fail for
                // any device with major > 0, producing a spurious TOCTOU rejection every time.
                //
                // Fix: decompose `st_dev` back into major/minor using `nix::sys::stat::major`
                // and `nix::sys::stat::minor`, then reassemble in the same `(major << 32) |
                // minor` layout that `release_candidate.rs` uses for `FileStat.dev`.
                if let Ok(stat) = nix::sys::stat::stat(path) {
                    let st_major = nix::sys::stat::major(stat.st_dev);
                    let st_minor = nix::sys::stat::minor(stat.st_dev);
                    // Normalise to the statx-based encoding: (major << 32) | minor.
                    // This matches the layout stored in FileStat.dev by release_candidate.rs.
                    let fdev = (st_major << 32) | st_minor;
                    let fino = stat.st_ino;
                    if fdev != dev || fino != ino {
                        log::warn!(
                            "rpm_probe: TOCTOU: (dev, ino) mismatch for {}",
                            path.display()
                        );
                        return None;
                    }
                } else {
                    log::warn!(
                        "rpm_probe: could not re-stat {} for TOCTOU check",
                        path.display()
                    );
                    return None;
                }
                Some(FileOwnership {
                    package_name: pkg_name,
                    package_version: pkg_ver,
                    evidence_trail: trail,
                })
            }
            Ok(None) => None,
            Err(e) => {
                log::debug!("rpm_probe: query_file_owner failed: {e}");
                None
            }
        }
    }

    #[cfg(not(feature = "rpm-db"))]
    {
        let _ = (dev, ino, path);
        None
    }
}

/// Handle an `installed_digest` call.
fn installed_digest_inner(
    #[cfg(feature = "rpm-db")] probe: &RpmProbe,
    #[cfg(not(feature = "rpm-db"))] _probe: &RpmProbe,
    path: &Path,
) -> Option<InstalledDigest> {
    #[cfg(feature = "rpm-db")]
    {
        let Ok(guard) = probe.db.lock() else {
            log::warn!("rpm_probe: mutex poisoned in installed_digest");
            return None;
        };
        let db = guard.as_ref()?;

        match db.query_file_digest(path) {
            Ok(Some((algorithm, value))) => Some(InstalledDigest {
                path: path.to_string_lossy().into_owned(),
                algorithm,
                value,
            }),
            Ok(None) => None,
            Err(e) => {
                log::debug!("rpm_probe: query_file_digest failed: {e}");
                None
            }
        }
    }

    #[cfg(not(feature = "rpm-db"))]
    {
        let _ = path;
        None
    }
}

// ===========================================================================
// Public standalone function
// ===========================================================================

/// Check whether a named RPM package is installed on this system.
///
/// Opens `/var/lib/rpm/rpmdb.sqlite` read-only, queries the `Name` table
/// for an exact match, and returns the result.
///
/// `Ok(true)` means installed; `Ok(false)` means not installed. `Err`
/// means the query could not be completed — callers should treat this as
/// distinct from a definitive "not installed" result.
///
/// A bare `bool` cannot distinguish "package absent" from "query failed";
/// callers that need to surface database degradation separately from absent
/// packages must match on the `Err` variant.
///
/// ## Compliance
///
/// - NIST SP 800-53 CM-8 — component inventory query; structured error enables
///   callers to detect degraded database state separately from absent packages.
/// - NIST SP 800-53 SA-12 — supply chain provenance verification.
///
/// # Errors
///
/// - [`PackageQueryError::DatabaseUnavailable`] — the RPM database could not
///   be opened (absent, unreadable, or `rpm-db` feature not compiled in).
/// - [`PackageQueryError::QueryFailed`] — the database opened but the query
///   itself failed (corruption, schema mismatch).
#[cfg(feature = "rpm-db")]
#[must_use = "package query result must be examined — Ok(false) and Err differ in meaning"]
pub fn is_installed(pkgname: &str) -> Result<bool, PackageQueryError> {
    let mut bundle = EvidenceBundle::new();
    let db = RpmDb::open(&mut bundle).map_err(|e| {
        log::debug!("is_installed({pkgname}): db open failed: {e}");
        PackageQueryError::DatabaseUnavailable
    })?;
    db.is_installed(pkgname).map_err(|e| {
        log::debug!("is_installed({pkgname}): query failed: {e}");
        PackageQueryError::QueryFailed
    })
}

/// Stub — `rpm-db` feature is disabled; always returns `DatabaseUnavailable`.
///
/// ## Compliance
///
/// - NIST SP 800-53 CM-8, SA-12.
///
/// # Errors
///
/// Always returns [`PackageQueryError::DatabaseUnavailable`] when compiled
/// without the `rpm-db` feature.
#[cfg(not(feature = "rpm-db"))]
#[must_use = "package query result must be examined — Ok(false) and Err differ in meaning"]
pub fn is_installed(_pkgname: &str) -> Result<bool, PackageQueryError> {
    Err(PackageQueryError::DatabaseUnavailable)
}

// ===========================================================================
// Internal helper
// ===========================================================================

/// Build a minimal failed-probe evidence record.
fn no_db_record(path: &str, note: &str) -> EvidenceRecord {
    EvidenceRecord {
        source_kind: SourceKind::PackageDb,
        path_requested: path.to_owned(),
        notes: vec![note.to_owned()],
        ..Default::default()
    }
}
