// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//!
//! # dpkg Package Substrate Probe — Stub
//!
//! Stub implementation of [`PackageProbe`] for dpkg-based distributions
//! (Debian, Ubuntu, Kali Linux).
//!
//! The stub validates that the dpkg status file (`/var/lib/dpkg/status`) is
//! present and returns a minimal substrate identity. Full dpkg status parsing
//! is deferred to a future iteration.
//!
//! ## What the stub does
//!
//! 1. Probes for the existence of `/var/lib/dpkg/` — the dpkg database root.
//! 2. Checks for the dpkg status file (`/var/lib/dpkg/status`). Presence is
//!    recorded as a second corroborating fact.
//! 3. Returns `facts_count = 1` for the DB root alone, or `facts_count = 2`
//!    if the status file is also present.
//!
//! `query_ownership` and `installed_digest` return `None` in this stub.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 CM-7**: Least Functionality — stub exposes only the
//!   minimum probe surface the pipeline requires.
//! - **NIST SP 800-53 SA-12**: Supply Chain Risk Management — the dpkg status
//!   file is the provenance record for installed software on Debian-family
//!   systems.
//! - **NIST SP 800-53 SI-7**: Software Integrity — digest capability is
//!   declared `false`; callers skip integrity verification rather than making
//!   false positive claims.

use std::path::Path;

use nix::sys::statfs::statfs;

use crate::evidence::{EvidenceBundle, EvidenceRecord, SourceKind};
use crate::os_identity::{OsFamily, SubstrateIdentity};

use super::{FileOwnership, InstalledDigest, PackageProbe, ProbeResult};

/// dpkg database root directory.
const DPKG_DB_ROOT: &str = "/var/lib/dpkg";

/// dpkg status file (primary package database).
const DPKG_STATUS: &str = "/var/lib/dpkg/status";

/// tmpfs filesystem magic number (linux/magic.h `TMPFS_MAGIC`).
/// A DB root on tmpfs may not be a real dpkg database.
const TMPFS_MAGIC: i64 = 0x0102_1994;

/// dpkg package substrate probe.
///
/// NIST SP 800-53 CM-7, SA-12, SI-7.
pub struct DpkgProbe;

impl DpkgProbe {
    /// Construct a new `DpkgProbe`.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for DpkgProbe {
    fn default() -> Self {
        Self::new()
    }
}

impl PackageProbe for DpkgProbe {
    fn probe(&self, bundle: &mut EvidenceBundle) -> ProbeResult {
        // Step 1: Check DB root existence.
        // Path-based check only — stub limitation; no fd-anchored open or DB parse.
        let root_present = Path::new(DPKG_DB_ROOT).exists();
        if !root_present {
            log::debug!(
                "dpkg_probe: /var/lib/dpkg not found — not a dpkg system"
            );
            let rec = EvidenceRecord {
                source_kind: SourceKind::PackageDb,
                opened_by_fd: false,
                path_requested: DPKG_DB_ROOT.to_owned(),
                path_resolved: None,
                stat: None,
                fs_magic: None,
                sha256: None,
                pkg_digest: None,
                parse_ok: false,
                notes: vec!["dpkg DB root not present".to_owned()],
            };
            bundle.push(rec.clone());
            return ProbeResult {
                probe_name: "dpkg",
                parse_ok: false,
                can_query_ownership: false,
                can_verify_digest: false,
                identity: None,
                evidence: rec,
            };
        }

        // statfs the DB root to detect tmpfs substitution.
        // Filesystem magic TMPFS_MAGIC (0x0102_1994) — not a real dpkg database location.
        let fs_magic_opt: Option<u64> = match statfs(DPKG_DB_ROOT) {
            Ok(stat) => {
                let magic = stat.filesystem_type().0;
                if magic == TMPFS_MAGIC {
                    log::warn!(
                        "dpkg_probe: {DPKG_DB_ROOT} is on tmpfs — \
                         may not be a real dpkg database"
                    );
                }
                // Cast i64 → u64; filesystem magic values are defined as positive constants.
                Some(magic.cast_unsigned())
            }
            Err(e) => {
                log::debug!("dpkg_probe: statfs({DPKG_DB_ROOT}) failed: {e}");
                None
            }
        };

        let mut identity = SubstrateIdentity {
            family: OsFamily::DpkgBased,
            distro: None,
            version_id: None,
            facts_count: 0,
            probe_used: "dpkg",
        };

        // Fact 1: dpkg DB root is present.
        identity.add_fact();

        let mut notes = vec!["dpkg DB root present: /var/lib/dpkg".to_owned()];
        if let Some(magic) = fs_magic_opt {
            notes.push(format!("db_root_fs_magic={magic:#x}"));
        }

        // Step 2: Check for status file (fact 2).
        // Path-based check only — stub limitation; no fd-anchored open or DB parse.
        if Path::new(DPKG_STATUS).exists() {
            identity.add_fact();
            notes.push("status file present: /var/lib/dpkg/status".to_owned());
        } else {
            notes.push("status file not found (partial probe)".to_owned());
        }

        log::debug!(
            "dpkg_probe: facts_count={}, parse_ok=true",
            identity.facts_count
        );

        let ev = EvidenceRecord {
            source_kind: SourceKind::PackageDb,
            opened_by_fd: false,
            path_requested: DPKG_DB_ROOT.to_owned(),
            path_resolved: None,
            stat: None,
            fs_magic: fs_magic_opt,
            sha256: None,
            pkg_digest: None,
            parse_ok: true,
            notes,
        };
        bundle.push(ev.clone());

        ProbeResult {
            probe_name: "dpkg",
            parse_ok: true,
            can_query_ownership: false,
            can_verify_digest: false,
            identity: Some(identity),
            evidence: ev,
        }
    }

    fn query_ownership(
        &self,
        _dev: u64,
        _ino: u64,
        _path: &Path,
    ) -> Option<FileOwnership> {
        // Stub: full dpkg ownership queries not yet implemented.
        None
    }

    fn installed_digest(&self, _path: &Path) -> Option<InstalledDigest> {
        // Stub: full dpkg digest lookup not yet implemented.
        None
    }
}
