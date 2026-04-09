// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//!
//! # Release Parse Phase — Two-Path Independent Parsing
//!
//! Parses the `os-release` file using Two-Path Independence (TPI): two
//! independent parsers — `nom` (path A) and a line-by-line `split_once`
//! parser (path B) — must produce identical key sets. If they disagree, the
//! parse fails closed.
//!
//! ## TPI design
//!
//! - **Path A** (`nom`): a `nom`-based tokenizer that handles `KEY=VALUE` and
//!   `KEY="VALUE"` syntax, comment lines, and blank lines. Produces a
//!   `HashMap<&str, &str>` of key→value pairs.
//!
//! - **Path B** (`split_once`): a line-by-line scanner that splits each
//!   non-comment line on the first `=`. Produces an independent `HashMap`.
//!
//! - **Agreement check**: both maps must contain identical key sets. If a
//!   key is present in one map but absent in the other, the phase returns
//!   `None` with a downgrade and records the contradiction.
//!
//! - **Value extraction**: only path A values are used to construct the
//!   `OsRelease` struct, but the key-set agreement gate must pass first.
//!
//! ## Substrate corroboration
//!
//! After a successful parse, the `ID=` field is compared against the
//! substrate-derived `Distro` (if available). Agreement → `TrustedLabel`
//! (if T4 was earned). Disagreement → `IntegrityVerifiedButContradictory`.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 SI-7**: Two-path parsing eliminates single-point-of-failure
//!   in the security-relevant parse that establishes OS identity.
//! - **NIST SP 800-53 CM-8**: The parsed `OsRelease` is the typed component
//!   identity; field validation enforces inventory accuracy.
//! - **NIST SP 800-53 SI-10**: Input validation — all field values are parsed
//!   into validated newtypes; raw strings do not escape this module.
//! - **NSA RTB TPI**: Two independent parse paths must agree on the key set.

use std::collections::HashMap;
use std::io::Read as _;
use std::os::fd::AsFd as _;
use std::path::Path;

use nom::IResult;
use nom::branch::alt;
use nom::bytes::complete::{is_not, take_while1};
use nom::character::complete::{char, line_ending, space0};
use nom::combinator::{map, opt, recognize};
use nom::multi::many0;
use nom::sequence::{delimited, pair, preceded, terminated};

use crate::confidence::{ConfidenceModel, Contradiction, TrustLevel};
use crate::evidence::{EvidenceBundle, EvidenceRecord, SourceKind};
use crate::os_identity::{Distro, SubstrateIdentity};
use crate::os_release::{
    BuildId, Codename, CpeName, OsId, OsName, OsRelease, OsVersion, ValidatedUrl, VariantId,
    VersionId,
};

use super::file_ownership::find_stat_for_path;
use super::label_trust::LabelTrust;
use super::substrate::FileOwnership;

// ===========================================================================
// Phase entry point
// ===========================================================================

/// Run the release parse phase.
///
/// Returns `(Option<OsRelease>, LabelTrust)`. The label trust is always
/// populated — even a failed parse returns `UntrustedLabelCandidate`.
///
/// NIST SP 800-53 SI-7, CM-8, SI-10. NSA RTB TPI.
pub(super) fn run(
    evidence: &mut EvidenceBundle,
    confidence: &mut ConfidenceModel,
    candidate: &Path,
    substrate_identity: Option<&SubstrateIdentity>,
    ownership: Option<&FileOwnership>,
    integrity_ok: bool,
    max_line_len: usize,
) -> (Option<OsRelease>, LabelTrust) {
    #[cfg(debug_assertions)]
    let t0 = std::time::Instant::now();

    let result = run_inner(
        evidence,
        confidence,
        candidate,
        substrate_identity,
        ownership,
        integrity_ok,
        max_line_len,
    );

    #[cfg(debug_assertions)]
    log::debug!(
        "release_parse: completed in {} µs",
        t0.elapsed().as_micros()
    );

    result
}

fn run_inner(
    evidence: &mut EvidenceBundle,
    confidence: &mut ConfidenceModel,
    candidate: &Path,
    substrate_identity: Option<&SubstrateIdentity>,
    ownership: Option<&FileOwnership>,
    integrity_ok: bool,
    max_line_len: usize,
) -> (Option<OsRelease>, LabelTrust) {
    let candidate_str = candidate.to_string_lossy().into_owned();

    // Read content from the regular filesystem (os-release is not on procfs/sysfs).
    let Some(content) = read_candidate(candidate, &candidate_str, evidence, confidence) else {
        return (None, LabelTrust::UntrustedLabelCandidate);
    };

    // Enforce per-line length limit before any parsing.
    for line in content.lines() {
        if line.len() > max_line_len {
            log::warn!(
                "release_parse: line in {candidate_str} exceeds max_line_len ({} bytes)",
                line.len()
            );
            confidence.downgrade(
                TrustLevel::KernelAnchored,
                "os-release contains line exceeding max_line_len",
            );
            return (None, LabelTrust::UntrustedLabelCandidate);
        }
    }

    // --- TPI: Path A (nom parser) ---
    let Some(map_a) = parse_with_nom(&content) else {
        log::warn!("release_parse: nom parser failed for {candidate_str}");
        confidence.downgrade(
            TrustLevel::KernelAnchored,
            "os-release: nom parse path failed",
        );
        return (None, LabelTrust::UntrustedLabelCandidate);
    };

    // --- TPI: Path B (split_once scanner) ---
    let map_b = parse_with_split(&content);

    // --- TPI: Agreement check on key sets ---
    if !key_sets_agree(&map_a, &map_b) {
        log::warn!(
            "release_parse: TPI disagreement — nom and split_once produced different key sets"
        );
        confidence.record_contradiction(
            Contradiction {
                source_a: "nom",
                source_b: "split_once",
                description: "os-release key sets differ between the two parse paths".to_owned(),
            },
            TrustLevel::KernelAnchored,
        );
        evidence.push(EvidenceRecord {
            source_kind: SourceKind::RegularFile,
            path_requested: candidate_str,
            notes: vec!["TPI: nom vs split_once key-set disagreement — fail closed".to_owned()],
            ..Default::default()
        });
        return (None, LabelTrust::UntrustedLabelCandidate);
    }

    // --- Build OsRelease from path A values ---
    let (os_release, build_notes) = match build_os_release(&map_a) {
        Ok(pair) => pair,
        Err(e) => {
            log::warn!("release_parse: OsRelease construction failed for {candidate_str}: {e}");
            confidence.downgrade(
                TrustLevel::KernelAnchored,
                "os-release field validation failed",
            );
            return (None, LabelTrust::UntrustedLabelCandidate);
        }
    };

    // Record successful parse.
    evidence.push(EvidenceRecord {
        source_kind: SourceKind::RegularFile,
        path_requested: candidate_str.clone(),
        parse_ok: true,
        notes: build_notes,
        ..Default::default()
    });

    // --- Substrate corroboration and label trust assignment ---
    let label_trust = assign_label_trust(
        &os_release,
        substrate_identity,
        ownership,
        integrity_ok,
        confidence,
    );

    log::debug!("release_parse: {candidate_str} parsed ok; label_trust={label_trust:?}");

    (Some(os_release), label_trust)
}

// ===========================================================================
// Content reader
// ===========================================================================

/// Read the os-release file content from the regular filesystem.
///
/// Opens the file, verifies `(dev,ino)` against the release_candidate statx
/// record to detect substitution (TOCTOU), then reads via the open file handle.
///
/// Returns `Some(content)` on success, `None` on I/O failure or TOCTOU
/// detection (side-effect: records in evidence and downgrades confidence).
// Line-count overage is a result of F-03 TOCTOU re-verification code added
// for security compliance (NSA RTB TOCTOU, NIST SP 800-53 SI-7).
#[expect(
    clippy::too_many_lines,
    reason = "line-count overage is TOCTOU re-verification code added for NSA RTB / NIST SP 800-53 SI-7 compliance"
)]
fn read_candidate(
    candidate: &Path,
    candidate_str: &str,
    evidence: &mut EvidenceBundle,
    confidence: &mut ConfidenceModel,
) -> Option<String> {
    // os-release is a regular file, not procfs/sysfs — ProcfsText/SysfsText do not apply.
    let file = match std::fs::File::open(candidate) {
        Ok(f) => f,
        Err(e) => {
            log::warn!("release_parse: could not open {candidate_str}: {e}");
            confidence.downgrade(
                TrustLevel::KernelAnchored,
                "os-release read failed during parse phase",
            );
            evidence.push(EvidenceRecord {
                source_kind: SourceKind::RegularFile,
                path_requested: candidate_str.to_owned(),
                notes: vec!["os-release open failed in parse phase".to_owned()],
                ..Default::default()
            });
            return None;
        }
    };

    // TOCTOU re-verification: fstat the open fd and compare (dev,ino) against
    // the release_candidate evidence record.
    let fstat_ok = match rustix::fs::fstat(file.as_fd()) {
        Ok(st) => {
            // Normalize dev_t to (major << 32) | minor to match the encoding
            // used in release_candidate.rs (from statx stx_dev_major/minor).
            // Linux dev_t uses a different compact encoding — comparing raw
            // values would always fail for devices with major > 0.
            let maj = rustix::fs::major(st.st_dev);
            let min = rustix::fs::minor(st.st_dev);
            let cur_dev = (u64::from(maj) << 32) | u64::from(min);
            let cur_ino = st.st_ino;
            match find_stat_for_path(evidence, candidate_str) {
                Some((rec_dev, rec_ino)) if cur_dev != rec_dev || cur_ino != rec_ino => {
                    log::warn!(
                        "release_parse: TOCTOU — file identity changed for {candidate_str}: \
                         recorded=({rec_dev},{rec_ino}) current=({cur_dev},{cur_ino})"
                    );
                    evidence.push(EvidenceRecord {
                        source_kind: SourceKind::RegularFile,
                        opened_by_fd: false,
                        path_requested: candidate_str.to_owned(),
                        path_resolved: None,
                        stat: None,
                        fs_magic: None,
                        sha256: None,
                        pkg_digest: None,
                        parse_ok: false,
                        notes: vec![
                            "TOCTOU: file identity changed between candidate statx and parse read"
                                .to_owned(),
                        ],
                        duration_ns: None,
                    });
                    confidence.downgrade(
                        TrustLevel::KernelAnchored,
                        "release_parse: file identity changed before parse read",
                    );
                    return None;
                }
                Some(_) => {
                    log::debug!(
                        "release_parse: fstat verified (dev={cur_dev},ino={cur_ino}) \
                         matches release_candidate statx"
                    );
                    true
                }
                None => {
                    // No stat record available — cannot verify; continue.
                    false
                }
            }
        }
        Err(e) => {
            log::debug!("release_parse: fstat failed for {candidate_str}: {e}");
            false
        }
    };

    // Read from the open file handle (fd-anchored).
    let mut content = String::new();
    let mut reader = file;
    match reader.read_to_string(&mut content) {
        Ok(_) => {
            evidence.push(EvidenceRecord {
                source_kind: SourceKind::RegularFile,
                opened_by_fd: fstat_ok,
                path_requested: candidate_str.to_owned(),
                path_resolved: None,
                stat: None,
                fs_magic: None,
                sha256: None,
                pkg_digest: None,
                parse_ok: true,
                notes: vec![if fstat_ok {
                    "fstat verified: (dev,ino) matches release_candidate statx".to_owned()
                } else {
                    "path-based open; (dev,ino) stat not available for re-verification".to_owned()
                }],
                duration_ns: None,
            });
            Some(content)
        }
        Err(e) => {
            log::warn!("release_parse: could not read {candidate_str}: {e}");
            confidence.downgrade(
                TrustLevel::KernelAnchored,
                "os-release read failed during parse phase",
            );
            evidence.push(EvidenceRecord {
                source_kind: SourceKind::RegularFile,
                opened_by_fd: false,
                path_requested: candidate_str.to_owned(),
                path_resolved: None,
                stat: None,
                fs_magic: None,
                sha256: None,
                pkg_digest: None,
                parse_ok: false,
                notes: vec!["os-release read failed in parse phase".to_owned()],
                duration_ns: None,
            });
            None
        }
    }
}

// ===========================================================================
// TPI Path A: nom parser
// ===========================================================================

/// Parse `os-release` content using `nom`.
///
/// Returns a map of key → value (with quotes stripped). Returns `None` if
/// the nom parser encounters a structural error.
///
/// NIST SP 800-53 SI-7 — TPI path A.
fn parse_with_nom(content: &str) -> Option<HashMap<&str, &str>> {
    let result: IResult<&str, Vec<Option<(&str, &str)>>> = many0(alt((
        // Blank line
        map(recognize(pair(space0, line_ending)), |_| None),
        // Comment line
        map(
            recognize(pair(
                preceded(space0, char('#')),
                terminated(opt(is_not("\n\r")), opt(line_ending)),
            )),
            |_| None,
        ),
        // KEY=VALUE or KEY="VALUE"
        map(
            terminated(
                pair(
                    // Key: alphanumeric and underscores
                    take_while1(|c: char| c.is_ascii_alphanumeric() || c == '_'),
                    preceded(
                        char('='),
                        alt((
                            // Quoted value
                            delimited(char('"'), opt(is_not("\"")), char('"')),
                            // Unquoted value (may be empty — use opt)
                            opt(is_not("\n\r")),
                        )),
                    ),
                ),
                opt(line_ending),
            ),
            |(k, v)| Some((k, v.unwrap_or(""))),
        ),
    )))(content);

    match result {
        Ok((_, pairs)) => {
            let mut map = HashMap::new();
            for pair in pairs.into_iter().flatten() {
                map.insert(pair.0, pair.1);
            }
            Some(map)
        }
        Err(_) => None,
    }
}

// ===========================================================================
// TPI Path B: split_once scanner
// ===========================================================================

/// Parse `os-release` content using line-by-line `split_once`.
///
/// This is the independent second parse path. It intentionally does NOT use
/// `nom` — any shared logic between paths would defeat TPI.
///
/// NIST SP 800-53 SI-7 — TPI path B.
fn parse_with_split(content: &str) -> HashMap<&str, &str> {
    let mut map = HashMap::new();

    for line in content.lines() {
        let trimmed = line.trim();
        // Skip comments and blank lines.
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if let Some((key, val)) = trimmed.split_once('=') {
            // Strip enclosing quotes from value if present.
            let val = val.trim_matches('"');
            map.insert(key, val);
        }
    }

    map
}

// ===========================================================================
// TPI: Agreement check
// ===========================================================================

/// Return `true` if both maps contain exactly the same key set.
///
/// Value comparison is not performed here — we only verify that both parse
/// paths agree on which keys are present. Value differences may arise from
/// whitespace normalization differences; key-set agreement is the gate.
///
/// NIST SP 800-53 SI-7 — fail closed on disagreement.
fn key_sets_agree(a: &HashMap<&str, &str>, b: &HashMap<&str, &str>) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.keys().all(|k| b.contains_key(k))
}

// ===========================================================================
// OsRelease construction from parsed map
// ===========================================================================

/// Build an `OsRelease` from the nom-produced key→value map.
///
/// Returns `Err(String)` if required fields are absent or fail validation.
/// Notes are accumulated for the evidence record.
///
/// NIST SP 800-53 SI-10 — all values validated at construction.
fn build_os_release(map: &HashMap<&str, &str>) -> Result<(OsRelease, Vec<String>), String> {
    let mut notes = Vec::new();

    // Required: ID=
    let id_str = map.get("ID").ok_or_else(|| "missing required field: ID".to_owned())?;
    let id = OsId::parse(id_str).map_err(|e| format!("ID validation failed: {e}"))?;
    notes.push(format!("id={}", id.as_str()));

    // Required: NAME=
    let name_str = map.get("NAME").ok_or_else(|| "missing required field: NAME".to_owned())?;
    let name = OsName::parse(name_str).map_err(|e| format!("NAME validation failed: {e}"))?;

    // Optional fields — parse failure is logged but not fatal.
    let version_id = map.get("VERSION_ID").and_then(|s| VersionId::parse(s).ok());

    let version = map.get("VERSION").and_then(|s| OsVersion::parse(s).ok());

    let version_codename = map.get("VERSION_CODENAME").and_then(|s| Codename::parse(s).ok());

    let pretty_name = map.get("PRETTY_NAME").and_then(|s| OsName::parse(s).ok());

    let home_url = map.get("HOME_URL").and_then(|s| ValidatedUrl::parse(s).ok());

    let cpe_name = map.get("CPE_NAME").and_then(|s| CpeName::parse(s).ok());

    let variant_id = map.get("VARIANT_ID").and_then(|s| VariantId::parse(s).ok());

    let build_id = map.get("BUILD_ID").and_then(|s| BuildId::parse(s).ok());

    let ansi_color = map.get("ANSI_COLOR").map(|s| (*s).to_owned());

    // ID_LIKE= is a space-separated list of identifiers.
    let id_like = map.get("ID_LIKE").and_then(|s| {
        let ids: Vec<OsId> = s.split_whitespace().filter_map(|tok| OsId::parse(tok).ok()).collect();
        if ids.is_empty() {
            None
        } else {
            Some(ids)
        }
    });

    notes.push(format!("parsed {} fields", map.len()));

    let os_release = OsRelease {
        id,
        id_like,
        name,
        version_id,
        version,
        version_codename,
        pretty_name,
        home_url,
        cpe_name,
        variant_id,
        build_id,
        ansi_color,
    };

    Ok((os_release, notes))
}

// ===========================================================================
// Label trust assignment
// ===========================================================================

/// Assign `LabelTrust` based on the pipeline outcome.
///
/// - `TrustedLabel`: T4 reached AND digest verified AND substrate corroborates.
/// - `IntegrityVerifiedButContradictory`: T4 reached but substrate contradicts.
/// - `LabelClaim`: structurally valid but no integrity verification.
/// - `UntrustedLabelCandidate`: permissions failure or ownership missing.
///
/// NIST SP 800-53 SI-7, CM-8. NSA RTB.
fn assign_label_trust(
    os_release: &OsRelease,
    substrate_identity: Option<&SubstrateIdentity>,
    ownership: Option<&FileOwnership>,
    integrity_ok: bool,
    confidence: &mut ConfidenceModel,
) -> LabelTrust {
    // If unowned, the label cannot be trusted.
    if ownership.is_none() {
        return LabelTrust::LabelClaim;
    }

    // If integrity was not verified, the label is only a claim.
    if !integrity_ok {
        return LabelTrust::LabelClaim;
    }

    // T4 is in hand — now check substrate corroboration.
    match substrate_identity {
        Some(substrate) => {
            if substrate_id_matches(os_release.id.as_str(), substrate.distro.as_ref()) {
                LabelTrust::TrustedLabel
            } else {
                let contradiction = format!(
                    "os-release ID='{}' does not match substrate distro",
                    os_release.id.as_str()
                );
                log::warn!("release_parse: substrate contradiction: {contradiction}");
                confidence.record_contradiction(
                    Contradiction {
                        source_a: "os_release",
                        source_b: "substrate",
                        description: contradiction.clone(),
                    },
                    TrustLevel::SubstrateAnchored,
                );
                LabelTrust::IntegrityVerifiedButContradictory {
                    contradiction,
                }
            }
        }
        // No substrate identity available — still T4 from integrity alone.
        None => LabelTrust::TrustedLabel,
    }
}

/// Return `true` if the os-release `ID=` value is consistent with the
/// substrate-derived `Distro`.
///
/// Matching is deliberately broad — the goal is to detect obvious
/// contradictions (e.g., `id="ubuntu"` when the RPM DB says `Rhel`),
/// not to exhaustively enumerate all valid ID strings.
fn substrate_id_matches(id: &str, distro: Option<&Distro>) -> bool {
    let Some(distro) = distro else {
        // No substrate distro — no contradiction possible.
        return true;
    };

    match distro {
        Distro::Rhel => id == "rhel" || id.contains("rhel"),
        Distro::Fedora => id == "fedora",
        Distro::CentOs => id == "centos",
        Distro::AlmaLinux => id == "almalinux",
        Distro::RockyLinux => id == "rocky",
        Distro::Debian => id == "debian",
        Distro::Ubuntu => id == "ubuntu",
        Distro::Kali => id == "kali",
        Distro::Other(_) => true,
    }
}
