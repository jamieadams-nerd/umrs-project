// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//!
//! # RPM Package Substrate Probe — Stub
//!
//! Stub implementation of [`PackageProbe`] for RPM-based distributions.
//!
//! The stub validates that the RPM database root (`/var/lib/rpm/`) is present
//! and returns a minimal substrate identity to allow the pipeline to proceed
//! to `file_ownership` and `integrity_check`. Full RPM database parsing
//! (via `rpmdb` BDB/SQLite bindings) is deferred to a future iteration.
//!
//! ## What the stub does
//!
//! 1. Probes for the existence of `/var/lib/rpm/` — a necessary (not
//!    sufficient) condition for an RPM-based system.
//! 2. Checks for the presence of the Packages database file
//!    (`/var/lib/rpm/Packages` on RHEL 8/9 or `/var/lib/rpm/rpmdb.sqlite`
//!    on RHEL 10+). Presence of either file is recorded as a corroborating
//!    fact.
//! 3. Returns `facts_count = 1` for the DB root alone, or `facts_count = 2`
//!    if a Packages file is also present.
//!
//! `query_ownership` and `installed_digest` return `None` in this stub —
//! full implementation requires RPM DB parsing.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 CM-7**: Least Functionality — the stub exposes only
//!   the operations the pipeline needs; no extra functionality.
//! - **NIST SP 800-53 SA-12**: Supply Chain Risk Management — the RPM
//!   database is the primary provenance record for installed software on
//!   RHEL/Fedora systems.
//! - **NIST SP 800-53 SI-7**: Software Integrity — digest lookup capability
//!   is declared `false` in this stub; the caller will skip integrity
//!   verification rather than making a false positive claim.

use std::path::Path;

use crate::evidence::{EvidenceBundle, EvidenceRecord, SourceKind};
use crate::os_identity::{Distro, OsFamily, SubstrateIdentity};

use super::{FileOwnership, InstalledDigest, PackageProbe, ProbeResult};

/// RPM database root directory.
const RPM_DB_ROOT: &str = "/var/lib/rpm";

/// RHEL 8/9 BDB Packages file path.
const RPM_PACKAGES_BDB: &str = "/var/lib/rpm/Packages";

/// RHEL 10+ SQLite database file path.
const RPM_PACKAGES_SQLITE: &str = "/var/lib/rpm/rpmdb.sqlite";

/// RPM package substrate probe.
///
/// NIST SP 800-53 CM-7, SA-12, SI-7.
pub struct RpmProbe;

impl RpmProbe {
    /// Construct a new `RpmProbe`.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for RpmProbe {
    fn default() -> Self {
        Self::new()
    }
}

impl PackageProbe for RpmProbe {
    fn probe(&self, bundle: &mut EvidenceBundle) -> ProbeResult {
        // Step 1: Check DB root existence.
        let root_present = Path::new(RPM_DB_ROOT).exists();
        if !root_present {
            log::debug!("rpm_probe: /var/lib/rpm not found — not an RPM system");
            let rec = EvidenceRecord {
                source_kind: SourceKind::PackageDb,
                opened_by_fd: false,
                path_requested: RPM_DB_ROOT.to_owned(),
                path_resolved: None,
                stat: None,
                fs_magic: None,
                sha256: None,
                pkg_digest: None,
                parse_ok: false,
                notes: vec!["RPM DB root not present".to_owned()],
            };
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

        // Step 2: Check for Packages database (fact 2).
        let packages_present = Path::new(RPM_PACKAGES_SQLITE).exists()
            || Path::new(RPM_PACKAGES_BDB).exists();

        if packages_present {
            identity.add_fact();
            let which = if Path::new(RPM_PACKAGES_SQLITE).exists() {
                "rpmdb.sqlite (RHEL10+)"
            } else {
                "Packages (BDB, RHEL8/9)"
            };
            notes.push(format!("Packages file present: {which}"));

            // Infer RHEL family from SQLite DB presence (RHEL 10+ indicator).
            if Path::new(RPM_PACKAGES_SQLITE).exists() {
                identity.distro = Some(Distro::Rhel);
                notes.push("distro inferred: RHEL (SQLite RPM DB)".to_owned());
            }
        } else {
            notes.push("Packages DB file not found (partial probe)".to_owned());
        }

        log::debug!(
            "rpm_probe: facts_count={}, parse_ok=true",
            identity.facts_count
        );

        let ev = EvidenceRecord {
            source_kind: SourceKind::PackageDb,
            opened_by_fd: false,
            path_requested: RPM_DB_ROOT.to_owned(),
            path_resolved: None,
            stat: None,
            fs_magic: None,
            sha256: None,
            pkg_digest: None,
            parse_ok: true,
            notes,
        };
        bundle.push(ev.clone());

        ProbeResult {
            probe_name: "rpm",
            parse_ok: true,
            can_query_ownership: false,
            can_verify_digest: false,
            identity: Some(identity),
            evidence: ev,
        }
    }

    fn query_ownership(&self, _dev: u64, _ino: u64, _path: &Path) -> Option<FileOwnership> {
        // Stub: full RPM DB ownership queries not yet implemented.
        None
    }

    fn installed_digest(&self, _path: &Path) -> Option<InstalledDigest> {
        // Stub: full RPM DB digest lookup not yet implemented.
        None
    }
}
