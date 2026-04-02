// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//! Integration tests for the RPM SQLite database access layer.
//!
//! All tests skip gracefully if `/var/lib/rpm/rpmdb.sqlite` is absent —
//! this file will not exist in CI environments or on non-RHEL systems.
//!
//! ## Compliance
//!
//! - NIST SP 800-53 CM-8 — ownership queries are validated against a real DB.
//! - NIST SP 800-53 SA-12 — supply chain provenance established from real data.
//! - NIST SP 800-53 SI-7 — digest queries return reference values for
//!   integrity verification.

#[cfg(feature = "rpm-db")]
mod rpm_db_tests {
    use std::path::Path;

    use umrs_platform::detect::substrate::rpm_db::{RPM_DB_PATH, RpmDb};
    use umrs_platform::evidence::{DigestAlgorithm, EvidenceBundle};

    /// Return `true` if the RPM SQLite database is absent on this system.
    fn skip_if_no_rpmdb() -> bool {
        !Path::new(RPM_DB_PATH).exists()
    }

    /// Open the RPM database read-only — should succeed on RHEL 10+.
    #[test]
    fn open_rpmdb() {
        if skip_if_no_rpmdb() {
            eprintln!("SKIP: {RPM_DB_PATH} not present");
            return;
        }
        let mut bundle = EvidenceBundle::new();
        let result = RpmDb::open(&mut bundle);
        assert!(result.is_ok(), "RpmDb::open failed: {result:?}");
        assert!(!bundle.is_empty(), "evidence must be recorded on open");
    }

    /// `/usr/lib/os-release` must be owned by an *-release package.
    #[test]
    fn query_os_release_ownership() {
        if skip_if_no_rpmdb() {
            eprintln!("SKIP: {RPM_DB_PATH} not present");
            return;
        }
        let mut bundle = EvidenceBundle::new();
        let db = RpmDb::open(&mut bundle).expect("db should open");
        let path = Path::new("/usr/lib/os-release");
        let result = db.query_file_owner(path).expect("query should not error");
        match result {
            Some((name, _ver, trail)) => {
                // Package name should contain "release" on RHEL systems.
                assert!(
                    name.contains("release"),
                    "/usr/lib/os-release not owned by a -release package: {name}"
                );
                assert!(!trail.is_empty(), "evidence trail must not be empty");
            }
            None => {
                // On some containers /usr/lib/os-release may be unowned — not a failure.
                eprintln!("INFO: /usr/lib/os-release has no owner in RPM DB");
            }
        }
    }

    /// `/usr/lib/os-release` should have a non-empty SHA-256 digest.
    #[test]
    fn query_os_release_digest() {
        if skip_if_no_rpmdb() {
            eprintln!("SKIP: {RPM_DB_PATH} not present");
            return;
        }
        let mut bundle = EvidenceBundle::new();
        let db = RpmDb::open(&mut bundle).expect("db should open");
        let path = Path::new("/usr/lib/os-release");
        let result = db.query_file_digest(path).expect("digest query should not error");
        match result {
            Some((algo, bytes)) => {
                assert!(!bytes.is_empty(), "digest bytes must not be empty");
                // RHEL 10 uses SHA-256 for file digests.
                match algo {
                    DigestAlgorithm::Sha256 | DigestAlgorithm::Sha512 => {}
                    DigestAlgorithm::Md5 => {
                        // MD5 is allowed on older DBs — just log.
                        eprintln!("INFO: os-release digest uses MD5 (legacy DB)");
                    }
                    DigestAlgorithm::Unknown(ref s) => {
                        panic!("unexpected digest algorithm: {s}");
                    }
                }
            }
            None => {
                // Some containers may have no digest for this file.
                eprintln!("INFO: /usr/lib/os-release has no digest in RPM DB");
            }
        }
    }

    /// A path that definitely does not exist → `None` from ownership query.
    #[test]
    fn query_nonexistent_file() {
        if skip_if_no_rpmdb() {
            eprintln!("SKIP: {RPM_DB_PATH} not present");
            return;
        }
        let mut bundle = EvidenceBundle::new();
        let db = RpmDb::open(&mut bundle).expect("db should open");
        let path = Path::new("/nonexistent/path/xyzzy-not-a-real-file");
        let result = db.query_file_owner(path).expect("query should not error");
        assert!(result.is_none(), "nonexistent path must return None");
    }

    /// `bash` is always installed on RHEL systems.
    #[test]
    fn is_installed_known_package() {
        if skip_if_no_rpmdb() {
            eprintln!("SKIP: {RPM_DB_PATH} not present");
            return;
        }
        let result = umrs_platform::detect::is_installed("bash")
            .expect("RPM DB must be queryable on an RHEL system");
        assert!(result, "bash must be installed on an RHEL system");
    }

    /// A definitely-not-installed package must return `Ok(false)`.
    #[test]
    fn is_installed_missing_package() {
        if skip_if_no_rpmdb() {
            eprintln!("SKIP: {RPM_DB_PATH} not present");
            return;
        }
        let result = umrs_platform::detect::is_installed("nonexistent-pkg-xyz-umrs-test")
            .expect("RPM DB must be queryable on an RHEL system");
        assert!(!result, "nonexistent package must return false");
    }

    /// Querying a non-existent digest path returns None (not an error).
    #[test]
    fn query_nonexistent_digest() {
        if skip_if_no_rpmdb() {
            eprintln!("SKIP: {RPM_DB_PATH} not present");
            return;
        }
        let mut bundle = EvidenceBundle::new();
        let db = RpmDb::open(&mut bundle).expect("db should open");
        let path = Path::new("/nonexistent/path/xyzzy-not-a-real-file");
        let result = db.query_file_digest(path).expect("digest query should not error");
        assert!(result.is_none(), "nonexistent path must return None");
    }
}
