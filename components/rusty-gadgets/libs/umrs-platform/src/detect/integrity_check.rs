// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//!
//! # Integrity Check Phase
//!
//! Soft-gate phase that computes the SHA-256 digest of the os-release candidate
//! file and compares it against the reference digest stored in the package DB.
//! On a match, upgrades confidence to `TrustLevel::IntegrityAnchored` (T4).
//!
//! ## FIPS 140-2/140-3 Posture Statement
//!
//! The SHA-256 implementation used here is `sha2 0.10` from the RustCrypto
//! family. **This crate has not been independently validated under FIPS 140-2
//! or FIPS 140-3 by NIST/CMVP.** It is used here exclusively for file
//! integrity verification — comparing the on-disk content of `os-release`
//! against a reference digest stored in the package DB. It is NOT used for
//! key derivation, MAC generation, digital signatures, or any authentication
//! operation. On systems where FIPS-validated SHA-256 is mandatory for this
//! operation, this phase must be replaced with a call to the system's
//! FIPS-validated cryptographic provider (e.g., via OpenSSL FIPS module or
//! `kcapi`). On RHEL 10 with FIPS mode active, callers should verify this
//! posture satisfies their policy before relying on T4 integrity results.
//!
//! ## Steps performed
//!
//! 1. Fetch the reference digest from the probe: `probe.installed_digest(path)`.
//!    Only `DigestAlgorithm::Sha256` and `DigestAlgorithm::Sha512` digests are
//!    accepted. An `Md5` reference digest is recorded but triggers a downgrade
//!    rather than a T4 upgrade.
//!
//! 2. Open the candidate file for reading. The read is bounded by
//!    `max_read_bytes` to prevent unbounded allocation. A file larger than the
//!    limit is rejected with a downgrade.
//!
//! 3. Compute SHA-256 using `sha2::Sha256`. The resulting digest is compared
//!    against the package DB reference value. Match → upgrade to T4 and
//!    populate `sha256` in the evidence record. Mismatch → downgrade and
//!    record the deviation.
//!
//! All failures are soft — they produce a downgrade but never abort.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 SI-7**: Software and Information Integrity — on-disk
//!   content verified against a trusted reference (package DB digest).
//! - **NIST SP 800-53 SC-28**: Protection of Information at Rest — the digest
//!   comparison detects at-rest tampering of a security-critical configuration
//!   file.
//! - **CMMC L2 SI.1.210**: Identify, report, and correct information and
//!   information system flaws in a timely manner — digest mismatch is a
//!   reportable integrity deviation.
//! - **NSA RTB TOCTOU**: The file is opened once; all reads happen on the same
//!   open `File` handle. Re-opening by path is not performed.

use std::fs::File;
use std::io::{self, Read};
use std::os::fd::AsFd as _;
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

use crate::confidence::{ConfidenceModel, TrustLevel};
use crate::evidence::{DigestAlgorithm, EvidenceBundle, EvidenceRecord, PkgDigest, SourceKind};
use crate::kattrs::{ProcfsText, SecureReader};

use super::file_ownership::{find_resolved_path, find_stat_for_path};
use super::substrate::{FileOwnership, InstalledDigest, PackageProbe};

// ===========================================================================
// Phase entry point
// ===========================================================================

/// Run the integrity check phase.
///
/// Returns `true` if the on-disk SHA-256 digest matches the package DB
/// reference digest (T4 earned). Returns `false` in all other cases.
///
/// NIST SP 800-53 SI-7, SC-28. CMMC L2 SI.1.210. NSA RTB TOCTOU.
pub(super) fn run(
    evidence: &mut EvidenceBundle,
    confidence: &mut ConfidenceModel,
    candidate: &Path,
    probe: Option<&dyn PackageProbe>,
    ownership: Option<&FileOwnership>,
    max_read_bytes: usize,
) -> bool {
    #[cfg(debug_assertions)]
    let t0 = std::time::Instant::now();

    let result = run_inner(
        evidence,
        confidence,
        candidate,
        probe,
        ownership,
        max_read_bytes,
    );

    #[cfg(debug_assertions)]
    log::debug!(
        "integrity_check: completed in {} µs",
        t0.elapsed().as_micros()
    );

    result
}

// run_inner performs a sequential verification pipeline; splitting it would
// obscure the flow. The line-count overage is a result of F-02 TOCTOU
// re-verification code added for security compliance.
#[expect(clippy::too_many_lines, reason = "line-count overage is TOCTOU re-verification code for NSA RTB / NIST SP 800-53 SI-7 compliance")]
fn run_inner(
    evidence: &mut EvidenceBundle,
    confidence: &mut ConfidenceModel,
    candidate: &Path,
    probe: Option<&dyn PackageProbe>,
    ownership: Option<&FileOwnership>,
    max_read_bytes: usize,
) -> bool {
    let candidate_str = candidate.to_string_lossy().into_owned();

    // Guards: probe and prior ownership record both required.
    let Some(probe) = probe else {
        log::debug!("integrity_check: no probe available — skipping");
        return false;
    };

    if ownership.is_none() {
        log::debug!("integrity_check: file is unowned — T4 cannot be reached for {candidate_str}");
        return false;
    }

    // Resolve the query path for RPM DB lookups.
    //
    // On RHEL 10, `/etc/os-release` is a symlink to `/usr/lib/os-release`. The RPM
    // database records the real path. Use the resolved path so that the digest query
    // can succeed. The candidate_str (symlink path) remains in the audit record's
    // path_requested field. NIST SP 800-53 SI-7.
    //
    // `readlinkat` may return a relative symlink target (e.g., `../usr/lib/os-release`).
    // Resolve against the candidate's parent directory and canonicalize to an absolute path.
    let query_path: PathBuf = match find_resolved_path(evidence, &candidate_str) {
        Some(resolved) => {
            let p = PathBuf::from(&resolved);
            if p.is_absolute() {
                p
            } else {
                let parent = candidate.parent().unwrap_or_else(|| Path::new("/"));
                std::fs::canonicalize(parent.join(&p)).unwrap_or_else(|_| parent.join(p))
            }
        }
        None => candidate.to_path_buf(),
    };

    if query_path != candidate {
        log::debug!(
            "integrity_check: using resolved path '{}' (symlink target of '{candidate_str}') \
             for RPM DB digest query",
            query_path.display()
        );
    }

    // Fetch the reference digest from the probe.
    let Some(installed) = probe.installed_digest(&query_path) else {
        log::warn!(
            "integrity_check: no installed digest available for {candidate_str} — T4 not earned"
        );
        evidence.push(no_digest_record(&candidate_str));
        return false;
    };

    // Reject weak or unknown digest algorithms.
    if let Some(rejection) =
        check_algorithm_policy(&installed, &candidate_str, evidence, confidence)
    {
        return rejection;
    }

    // NIST SP 800-53 SC-13, CMMC L2 SC.3.177 — gate T4 on FIPS; sha2 crate is not FIPS 140-3 validated.
    if fips_mode_active(evidence, confidence) {
        return false;
    }

    // Open the candidate file (single File handle — TOCTOU safe).
    // path-based open; (dev,ino) re-verified below via fstat before trusting content.
    let mut file = match File::open(candidate) {
        Ok(f) => f,
        Err(e) => {
            log::warn!("integrity_check: could not open {candidate_str}: {e}");
            confidence.downgrade(
                TrustLevel::SubstrateAnchored,
                "integrity: could not open os-release for hashing",
            );
            return false;
        }
    };

    // TOCTOU re-verification: fstat the open fd and compare (dev,ino) against
    // the release_candidate statx record. Detects substitution between phases.
    //
    // BUG EXPLANATION — device number encoding mismatch:
    //
    // `release_candidate.rs` calls `rustix::fs::statx`, which returns the device
    // number as separate `stx_dev_major` and `stx_dev_minor` u32 fields.
    // `FileStat.dev` stores them combined as `(major as u64) << 32 | (minor as u64)`.
    //
    // `rustix::fs::fstat()` returns `st_dev` as a Linux `dev_t` — the kernel's compact
    // encoding via `makedev(major, minor)` (e.g., device 253:0 → 64768, not
    // `(253u64 << 32) | 0`). These two encodings differ for any device with major > 0.
    //
    // Comparing `st.st_dev` directly against `FileStat.dev` would always fail for real
    // block devices, producing a spurious TOCTOU rejection on every non-loopback system.
    //
    // Fix: decompose `st_dev` back into major/minor using `rustix::fs::major` and
    // `rustix::fs::minor`, then reassemble in the same `(major << 32) | minor` layout
    // that `release_candidate.rs` uses for `FileStat.dev`.
    let fstat_result = rustix::fs::fstat(file.as_fd());
    let current_dev_ino: Option<(u64, u64)> = match fstat_result {
        Ok(st) => {
            // Decompose the kernel dev_t compact encoding, then reassemble using the
            // same `(major << 32) | minor` layout stored in FileStat.dev.
            let maj = u64::from(rustix::fs::major(st.st_dev));
            let min = u64::from(rustix::fs::minor(st.st_dev));
            Some(((maj << 32) | min, st.st_ino))
        }
        Err(e) => {
            log::warn!("integrity_check: fstat failed for open fd on {candidate_str}: {e}");
            None
        }
    };

    // Look up the (dev,ino) from the release_candidate evidence record.
    let candidate_recorded = find_stat_for_path(evidence, &candidate_str);

    // If both are available, verify identity.
    if let (Some((cur_dev, cur_ino)), Some((rec_dev, rec_ino))) =
        (current_dev_ino, candidate_recorded)
    {
        if cur_dev != rec_dev || cur_ino != rec_ino {
            log::warn!(
                "integrity_check: TOCTOU — file identity changed for {candidate_str}: \
                 recorded=({rec_dev},{rec_ino}) current=({cur_dev},{cur_ino})"
            );
            evidence.push(EvidenceRecord {
                source_kind: SourceKind::RegularFile,
                opened_by_fd: false,
                path_requested: candidate_str,
                path_resolved: None,
                stat: None,
                fs_magic: None,
                sha256: None,
                pkg_digest: None,
                parse_ok: false,
                notes: vec![
                    "path-based open; (dev,ino) not re-verified against release_candidate statx"
                        .to_owned(),
                    "TOCTOU: file identity changed between statx and open; aborting hash"
                        .to_owned(),
                ],
                duration_ns: None,
            });
            confidence.downgrade(
                TrustLevel::SubstrateAnchored,
                "integrity: file identity changed between candidate selection and hashing",
            );
            return false;
        }
        log::debug!(
            "integrity_check: fstat verified: (dev={cur_dev},ino={cur_ino}) matches \
             release_candidate statx"
        );
    }

    // Read bounded content into a buffer.
    let content = match read_bounded(&mut file, max_read_bytes) {
        Ok(buf) => buf,
        Err(e) => {
            log::warn!("integrity_check: read failed for {candidate_str}: {e}");
            confidence.downgrade(
                TrustLevel::SubstrateAnchored,
                "integrity: os-release read failed during hashing",
            );
            return false;
        }
    };

    // Compute SHA-256.
    let computed: [u8; 32] = {
        let mut hasher = Sha256::new();
        hasher.update(&content);
        hasher.finalize().into()
    };

    // For SHA-512 reference digests, cross-algorithm comparison is not supported.
    if installed.algorithm == DigestAlgorithm::Sha512 {
        log::warn!(
            "integrity_check: package DB uses SHA-512 but we computed SHA-256 — \
             cross-algorithm comparison not supported; T4 not earned"
        );
        evidence.push(EvidenceRecord {
            source_kind: SourceKind::RegularFile,
            opened_by_fd: false,
            path_requested: candidate_str,
            path_resolved: None,
            stat: None,
            fs_magic: None,
            sha256: Some(computed),
            pkg_digest: Some(PkgDigest {
                algorithm: installed.algorithm,
                value: installed.value,
            }),
            parse_ok: false,
            notes: vec![
                "path-based open; (dev,ino) not re-verified against release_candidate statx"
                    .to_owned(),
                "SHA-512 vs SHA-256: cross-algorithm comparison unsupported".to_owned(),
            ],
            duration_ns: None,
        });
        return false;
    }

    // Determine whether fstat verification succeeded (both sides available and matching).
    let fstat_verified = matches!(
        (current_dev_ino, candidate_recorded),
        (Some((cd, ci)), Some((rd, ri))) if cd == rd && ci == ri
    );

    // Both are SHA-256 at this point.
    compare_and_record(
        computed,
        installed,
        candidate_str,
        fstat_verified,
        evidence,
        confidence,
    )
}

// ===========================================================================
// Algorithm policy check
// ===========================================================================

/// Check the algorithm of the installed digest against policy.
///
/// Returns `Some(false)` if the algorithm is rejected (caller should return
/// `false`). Returns `None` if the algorithm is acceptable and processing
/// should continue.
fn check_algorithm_policy(
    installed: &InstalledDigest,
    candidate_str: &str,
    evidence: &mut EvidenceBundle,
    confidence: &mut ConfidenceModel,
) -> Option<bool> {
    match &installed.algorithm {
        DigestAlgorithm::Md5 => {
            log::warn!(
                "integrity_check: package DB digest for {candidate_str} uses MD5 — \
                 algorithm is cryptographically weak; T4 not earned"
            );
            confidence.downgrade(
                TrustLevel::SubstrateAnchored,
                "integrity: package DB uses MD5 digest — weak algorithm",
            );
            evidence.push(EvidenceRecord {
                source_kind: SourceKind::PackageDb,
                opened_by_fd: false,
                path_requested: candidate_str.to_owned(),
                path_resolved: None,
                stat: None,
                fs_magic: None,
                sha256: None,
                pkg_digest: Some(PkgDigest {
                    algorithm: installed.algorithm.clone(),
                    value: installed.value.clone(),
                }),
                parse_ok: false,
                notes: vec!["MD5 digest rejected: weak algorithm".to_owned()],
                duration_ns: None,
            });
            Some(false)
        }
        DigestAlgorithm::Unknown(alg) => {
            log::warn!(
                "integrity_check: unknown digest algorithm '{alg}' for {candidate_str} — \
                 T4 not earned"
            );
            confidence.downgrade(
                TrustLevel::SubstrateAnchored,
                "integrity: unknown digest algorithm in package DB",
            );
            Some(false)
        }
        DigestAlgorithm::Sha256 | DigestAlgorithm::Sha512 => None,
    }
}

// ===========================================================================
// Digest comparison and evidence recording
// ===========================================================================

/// Compare the computed SHA-256 digest against the installed reference.
///
/// `fstat_verified` is `true` if the open fd's `(dev,ino)` was confirmed to
/// match the `release_candidate` statx record — i.e., the TOCTOU check passed.
///
/// Returns `true` if they match and T4 was earned; `false` otherwise.
fn compare_and_record(
    computed: [u8; 32],
    installed: InstalledDigest,
    candidate_str: String,
    fstat_verified: bool,
    evidence: &mut EvidenceBundle,
    confidence: &mut ConfidenceModel,
) -> bool {
    let digest_matches = computed.as_ref() == installed.value.as_slice();

    // Build common notes reflecting the open method and fstat verification status.
    let open_note = if fstat_verified {
        "fstat verified: (dev,ino) matches release_candidate statx".to_owned()
    } else {
        "path-based open; (dev,ino) not re-verified against release_candidate statx".to_owned()
    };

    if digest_matches {
        log::debug!("integrity_check: SHA-256 digest verified for {candidate_str}");
        confidence.upgrade(TrustLevel::IntegrityAnchored);
        evidence.push(EvidenceRecord {
            source_kind: SourceKind::RegularFile,
            // The file was opened via File::open (path-based), not via an fd-anchored
            // call. opened_by_fd is always false here. Fstat verification status is
            // recorded in the notes field. (NIST SP 800-53 AU-3 — audit records must
            // accurately reflect the actual open method.)
            opened_by_fd: false,
            path_requested: candidate_str,
            path_resolved: None,
            stat: None,
            fs_magic: None,
            sha256: Some(computed),
            pkg_digest: Some(PkgDigest {
                algorithm: DigestAlgorithm::Sha256,
                value: installed.value,
            }),
            parse_ok: true,
            notes: vec![open_note, "SHA-256 digest verified (T4 earned)".to_owned()],
            duration_ns: None,
        });
        true
    } else {
        log::warn!(
            "integrity_check: SHA-256 digest MISMATCH for {candidate_str} — \
             file may have been modified"
        );
        confidence.downgrade(
            TrustLevel::SubstrateAnchored,
            "integrity: os-release SHA-256 digest does not match package DB",
        );
        evidence.push(EvidenceRecord {
            source_kind: SourceKind::RegularFile,
            opened_by_fd: false,
            path_requested: candidate_str,
            path_resolved: None,
            stat: None,
            fs_magic: None,
            sha256: Some(computed),
            pkg_digest: Some(PkgDigest {
                algorithm: DigestAlgorithm::Sha256,
                value: installed.value,
            }),
            parse_ok: false,
            notes: vec![
                open_note,
                "SHA-256 digest mismatch — integrity deviation recorded".to_owned(),
            ],
            duration_ns: None,
        });
        false
    }
}

// ===========================================================================
// Evidence helpers
// ===========================================================================

/// Build an evidence record for a missing digest entry.
fn no_digest_record(candidate_str: &str) -> EvidenceRecord {
    EvidenceRecord {
        source_kind: SourceKind::PackageDb,
        opened_by_fd: false,
        path_requested: candidate_str.to_owned(),
        path_resolved: None,
        stat: None,
        fs_magic: None,
        sha256: None,
        pkg_digest: None,
        parse_ok: false,
        notes: vec!["no installed digest in package DB".to_owned()],
        duration_ns: None,
    }
}

// ===========================================================================
// FIPS gate
// ===========================================================================

/// Check whether FIPS mode is active by reading `/proc/sys/crypto/fips_enabled`.
///
/// Returns `true` if FIPS is active and T4 should be blocked. Returns `false`
/// if FIPS is disabled or the read fails (fail open — missing fips_enabled
/// usually means FIPS is not configured).
///
/// NIST SP 800-53 SC-13, CMMC L2 SC.3.177 — gate T4 on FIPS;
/// sha2 crate is not FIPS 140-3 validated.
fn fips_mode_active(evidence: &mut EvidenceBundle, confidence: &mut ConfidenceModel) -> bool {
    const FIPS_PATH: &str = "/proc/sys/crypto/fips_enabled";

    let node = match ProcfsText::new(PathBuf::from(FIPS_PATH)) {
        Ok(n) => n,
        Err(e) => {
            log::debug!("integrity_check: could not construct ProcfsText for fips_enabled: {e}");
            return false;
        }
    };

    let content = match SecureReader::<ProcfsText>::new().read_generic_text(&node) {
        Ok(s) => s,
        Err(e) => {
            log::debug!("integrity_check: could not read fips_enabled: {e}");
            return false;
        }
    };

    let trimmed = content.trim();
    if trimmed == "1" {
        log::warn!(
            "integrity_check: FIPS mode active — sha2::Sha256 is not FIPS 140-3 validated; \
             T4 not earned; capping at T3"
        );
        evidence.push(EvidenceRecord {
            source_kind: SourceKind::Procfs,
            opened_by_fd: true,
            path_requested: FIPS_PATH.to_owned(),
            path_resolved: None,
            stat: None,
            fs_magic: None,
            sha256: None,
            pkg_digest: None,
            parse_ok: false,
            notes: vec![
                "FIPS mode active: sha2 is not FIPS 140-3 validated; T4 not earned".to_owned(),
            ],
            duration_ns: None,
        });
        confidence.downgrade(
            TrustLevel::SubstrateAnchored,
            "FIPS mode: sha2 not FIPS-validated; integrity check skipped",
        );
        return true;
    }

    false
}

// ===========================================================================
// Bounded read helper
// ===========================================================================

/// Read at most `max_bytes` from an open file handle.
///
/// Returns the content as a `Vec<u8>`. Returns `Err` if the file is larger
/// than `max_bytes` — an oversize os-release is a security anomaly.
///
/// NSA RTB: bounded reads prevent unbounded allocation on malformed inputs.
fn read_bounded(file: &mut File, max_bytes: usize) -> io::Result<Vec<u8>> {
    let mut buf = Vec::new();
    let bytes_read = file.take((max_bytes as u64).saturating_add(1)).read_to_end(&mut buf)?;

    if bytes_read > max_bytes {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "os-release file exceeds maximum read limit",
        ));
    }

    Ok(buf)
}
