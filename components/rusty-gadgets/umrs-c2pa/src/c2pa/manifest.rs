// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Jamie Adams

//! # UMRS C2PA Manifest Reading and Chain-of-Custody Extraction
//!
//! Reads C2PA manifest stores from media files, walks the manifest ingredient
//! graph to reconstruct the full chain of custody, and derives trust status
//! from the c2pa SDK's `validation_status` codes.
//!
//! ## Key Exported Types
//!
//! - [`ChainEntry`] — a single entry in the chain of custody
//! - [`TrustStatus`] — trust evaluation result for a chain entry
//! - [`TrustFinding`] — structured diagnostic detail for UNVERIFIED entries
//! - [`read_chain`] — read and parse the full chain from a file
//! - [`manifest_json`] — raw manifest store as pretty-printed JSON
//! - [`chain_json`] — UMRS-parsed chain as JSON (for programmatic consumers)
//! - [`has_manifest`] — probe whether a file contains any C2PA data
//!
//! ## Chain Walk Algorithm
//!
//! The manifest store is a directed acyclic graph: the active manifest may
//! embed ingredient manifests, which in turn may embed their own ingredients.
//! `read_chain` walks this graph depth-first, oldest-first, with a cycle guard
//! to prevent infinite loops in malformed stores.
//!
//! ## Trust Finding Classification
//!
//! When a chain entry resolves to `TrustStatus::Untrusted`, the optional
//! `ChainEntry::trust_finding` field carries a [`TrustFinding`] that refines
//! the diagnosis. For example, [`TrustFinding::IssuerRotationMismatch`] fires
//! when the image was signed before the trust-list issuing CA's `Not Before`
//! date — a temporal certificate rotation event, not evidence of tampering.
//!
//! This classification is a **diagnostic signal only** — it does not change the
//! `UNVERIFIED` status. The operator must decide how to resolve the mismatch.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AU-10**: Non-repudiation — the chain walk surfaces every
//!   signing event across the full provenance history of a file.
//! - **NIST SP 800-53 AU-3**: Audit Record Content — `ChainEntry` carries
//!   signer identity, issuer, timestamp, algorithm, security marking, and
//!   structured trust findings for programmatic audit consumption.
//! - **NIST SP 800-53 SI-7**: Software, Firmware, and Information Integrity —
//!   `TrustStatus::Invalid` is set whenever the c2pa SDK reports a hash
//!   mismatch or signature verification failure, making tampering visible.

use std::path::Path;

use chrono::{DateTime, FixedOffset};
use serde::Serialize;

use crate::c2pa::{config::UmrsConfig, error::InspectError, trust::build_c2pa_settings};
#[allow(unused_imports)]
use crate::verbose;

/// Trust evaluation for a single entry in the chain of custody.
///
/// ## Variants:
///
/// | Status        | Display         | Meaning |
/// |---------------|-----------------|---------|
/// | `Trusted`     | `TRUSTED`       | Cert chain verified against a C2PA Trust List root CA |
/// | `Untrusted`   | `UNVERIFIED`    | Signature present but not validated against a trust list; the CA is not on the Trust List or no trust list was configured — not necessarily bad, just unverified |
/// | `Invalid`     | `INVALID`       | Signature verification failed or asset hash mismatch |
/// | `Revoked`     | `REVOKED`       | Signing certificate was revoked by the issuing CA |
/// | `NoTrustList` | `NO TRUST LIST` | No trust list configured — cannot evaluate trust; distinct from `Untrusted`: validation was not even attempted |
///
/// ## Compliance
///
/// - **NIST SP 800-53 AU-3**: Audit Record Content — `TrustStatus` is a
///   matchable, serializable, filterable value in the evidence record.
/// - **NSA RTB RAIN**: callers must not discard chain entries; the trust
///   status must be surfaced in reports or audit logs.
#[must_use = "TrustStatus carries the trust evaluation for a chain entry; \
              discarding it means trust decisions are silently bypassed"]
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum TrustStatus {
    #[serde(rename = "TRUSTED")]
    Trusted,
    #[serde(rename = "UNVERIFIED")]
    Untrusted,
    #[serde(rename = "INVALID")]
    Invalid,
    #[serde(rename = "REVOKED")]
    Revoked,
    #[serde(rename = "NO_TRUST_LIST")]
    NoTrustList,
}

impl std::fmt::Display for TrustStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Trusted => write!(f, "TRUSTED"),
            Self::Untrusted => write!(f, "UNVERIFIED"),
            Self::Invalid => write!(f, "INVALID"),
            Self::Revoked => write!(f, "REVOKED"),
            Self::NoTrustList => write!(f, "NO TRUST LIST"),
        }
    }
}

/// Structured diagnostic detail for an `UNVERIFIED` chain entry.
///
/// When a chain entry resolves to `TrustStatus::Untrusted`, this enum carries
/// the specific reason, if one can be determined. This is a **diagnostic signal**
/// that helps operators distinguish benign mismatch conditions (e.g., certificate
/// rotation) from potentially concerning ones (e.g., unknown CA). The `UNVERIFIED`
/// status is not altered — only the explanation is refined.
///
/// ## Compliance
///
/// - **NIST SP 800-53 AU-3**: Audit Record Content — findings are structured
///   enum variants, not log strings, enabling programmatic filtering and
///   matching by audit consumers.
/// - **NIST SP 800-53 SI-7**: Software, Firmware, and Information Integrity —
///   the `IssuerRotationMismatch` variant makes temporal CA rotation visible
///   as a first-class finding rather than an opaque `UNVERIFIED` result.
#[must_use = "TrustFinding carries the diagnostic reason for UNVERIFIED; \
              discarding it means the operator sees no explanation of why trust failed"]
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "kind")]
pub enum TrustFinding {
    /// - `IssuerRotationMismatch { image_signed, trust_cert_not_before, subject_cn }` —
    ///   the image was signed before the trust-list issuing CA's `Not Before` date; this is a
    ///   temporal certificate rotation mismatch, not evidence of tampering. The CA in the trust
    ///   list is the replacement cert issued after the vendor (e.g., Adobe) rotated their issuing
    ///   CA; images signed before the rotation carry the old cert, which is not in the trust list.
    ///   `image_signed` is the signing timestamp from the manifest (RFC 3339 / UTC string);
    ///   `trust_cert_not_before` is the `Not Before` date of the matching trust-list cert
    ///   (human-readable); `subject_cn` is the subject CN of the trust-list cert that was
    ///   temporally mismatched.
    ///   **Operator action:** obtain images signed after the CA rotation date, or add the old
    ///   issuing CA to `user_anchors` to validate pre-rotation images.
    IssuerRotationMismatch {
        image_signed: String,
        trust_cert_not_before: String,
        subject_cn: String,
    },
}

impl std::fmt::Display for TrustFinding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IssuerRotationMismatch {
                image_signed,
                trust_cert_not_before,
                subject_cn,
            } => write!(
                f,
                "Image signed {image_signed} before trust list certificate \
                 became valid {trust_cert_not_before} (CN: {subject_cn}) \
                 — likely certificate rotation, not tampering"
            ),
        }
    }
}

/// A single entry in the chain of custody, extracted from one manifest
/// in the manifest store.
///
/// ## Fields:
///
/// - `signer_name` — signer identity from the cert CN or `claim_generator` field.
/// - `issuer` — CA that issued the signing certificate.
/// - `signed_at` — signing timestamp, if present.
/// - `trust_status` — trust evaluation for this entry.
/// - `trust_finding` — structured diagnostic for `UNVERIFIED` entries when a specific cause can
///   be determined (e.g., [`TrustFinding::IssuerRotationMismatch`]); `None` for `TRUSTED`,
///   `INVALID`, `REVOKED`, and `NO_TRUST_LIST` entries, and for `UNVERIFIED` entries where no
///   specific cause is identified.
/// - `algorithm` — signing algorithm used (e.g., `"es256"`).
/// - `generator` — claim generator name (e.g., `"ChatGPT"`, `"UMRS Reference System"`).
/// - `generator_version` — claim generator version, if available (e.g., `"0.67.1"`).
/// - `security_label` — security marking from a `umrs.security-label` assertion, if present.
#[derive(Debug, Clone, Serialize)]
pub struct ChainEntry {
    pub signer_name: String,
    pub issuer: String,
    pub signed_at: Option<String>,
    pub trust_status: TrustStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_finding: Option<TrustFinding>,
    pub algorithm: String,
    pub generator: String,
    pub generator_version: Option<String>,
    pub security_label: Option<String>,
}

/// A parsed trust-anchor certificate record — subject CN and `Not Before` date.
///
/// Used by [`classify_rotation_mismatches`] to compare signing timestamps
/// against the trust-list certs' validity windows.
#[derive(Debug, Clone)]
struct AnchorCertInfo {
    subject_cn: String,
    not_before_str: String,
    not_before_dt: Option<DateTime<FixedOffset>>,
}

/// Extract the CN (Common Name) from an `X509NameRef` via its entry iterator.
///
/// Uses the structured openssl API to find entries with the `CN` NID,
/// avoiding the need to parse a formatted subject string. Returns the
/// first CN value found, or `None` if the name has no CN entry.
fn extract_cn_from_name(name: &openssl::x509::X509NameRef) -> Option<String> {
    use openssl::nid::Nid;
    let entries = name.entries_by_nid(Nid::COMMONNAME);
    for entry in entries {
        if let Ok(data) = entry.data().as_utf8() {
            return Some(data.to_string());
        }
    }
    None
}

/// Parse an OpenSSL-format `Not Before` string (e.g. `"Nov 19 18:29:44 2025 GMT"`)
/// into a `DateTime<FixedOffset>`.
///
/// Returns `None` if the string cannot be parsed — callers treat `None` as
/// "unable to compare" and skip the rotation check for this cert.
fn parse_openssl_time(s: &str) -> Option<DateTime<FixedOffset>> {
    // OpenSSL Asn1Time::to_string() produces "Mmm DD HH:MM:SS YYYY GMT"
    // e.g. "Nov 19 18:29:44 2025 GMT"
    let s_trimmed = s.trim().trim_end_matches(" GMT").trim();
    // Parse with a known format
    DateTime::parse_from_str(&format!("{s_trimmed} +0000"), "%b %e %T %Y %z")
        .or_else(|_| DateTime::parse_from_str(&format!("{s_trimmed} +0000"), "%b  %e %T %Y %z"))
        .ok()
}

/// Extract `AnchorCertInfo` records from a PEM bundle string.
///
/// Iterates each certificate in the bundle and extracts the subject CN and
/// `Not Before` date. Certs that cannot be parsed are silently skipped —
/// the trust anchor file has already been validated by OpenSSL when the
/// SDK loaded it; any cert we cannot parse here is simply unavailable for
/// rotation mismatch classification (a diagnostic gap, not a security gap).
fn extract_anchor_certs(pem: &str) -> Vec<AnchorCertInfo> {
    let mut result = Vec::new();
    let Ok(certs) = openssl::x509::X509::stack_from_pem(pem.as_bytes()) else {
        return result;
    };
    for cert in &certs {
        let subject_cn = extract_cn_from_name(cert.subject_name()).unwrap_or_default();
        if subject_cn.is_empty() {
            continue;
        }
        let not_before_str = cert.not_before().to_string();
        let not_before_dt = parse_openssl_time(&not_before_str);
        result.push(AnchorCertInfo {
            subject_cn,
            not_before_str,
            not_before_dt,
        });
    }
    result
}

/// Classify rotation mismatches for `Untrusted` chain entries.
///
/// For each entry with `TrustStatus::Untrusted` and a known `signed_at`
/// timestamp, checks whether any trust-anchor cert shares the same issuer
/// CN and has a `Not Before` date **after** the signing timestamp. If so,
/// the entry's `trust_finding` is set to
/// [`TrustFinding::IssuerRotationMismatch`].
///
/// This is a post-processing pass that runs after the full chain walk
/// and trust status assignment. It does not alter `trust_status` — only
/// the diagnostic field.
///
/// ## Fail-closed behaviour
///
/// If any step in the classification fails (timestamp unparseable, no matching
/// cert), the entry's `trust_finding` remains `None`. The operator still sees
/// `UNVERIFIED`; they just get no additional explanation. This is the correct
/// fail-closed posture: unknown does not become trusted.
///
/// ## Compliance
///
/// - **NIST SP 800-53 AU-3**: Audit Record Content — structured findings
///   enable audit consumers to distinguish rotation mismatches from genuinely
///   unknown CAs without parsing log strings.
/// - **NIST SP 800-53 SI-7**: Software, Firmware, and Information Integrity —
///   the classification never changes `UNVERIFIED` to `TRUSTED`; it only
///   adds context.
fn classify_rotation_mismatches(entries: &mut [ChainEntry], anchor_certs: &[AnchorCertInfo]) {
    #[cfg(debug_assertions)]
    let start = std::time::Instant::now();

    for entry in entries.iter_mut() {
        if entry.trust_status != TrustStatus::Untrusted {
            continue;
        }
        let Some(ref signed_at_str) = entry.signed_at else {
            continue;
        };

        // Parse the signing timestamp from the manifest (RFC 3339).
        let Ok(signed_dt) = DateTime::parse_from_rfc3339(signed_at_str) else {
            continue;
        };

        // Look for a trust-anchor cert whose CN resembles the issuer
        // and whose Not Before is after the signing date.
        for anchor in anchor_certs {
            let Some(not_before_dt) = anchor.not_before_dt else {
                continue;
            };

            // A rotation mismatch requires:
            //   1. The trust cert's Not Before is after the signing date
            //   2. The entry's issuer CN overlaps the trust cert's CN
            //      (exact or substring match, since the entry issuer comes from
            //      the manifest's parsed issuer string, which may be abbreviated)
            let is_after_signing = not_before_dt > signed_dt;
            let cn_overlaps = entry.issuer.contains(anchor.subject_cn.as_str())
                || anchor.subject_cn.contains(entry.issuer.as_str());

            if is_after_signing && cn_overlaps {
                log::warn!(
                    target: "umrs",
                    "IssuerRotationMismatch: image signed {signed_at_str} \
                     before trust cert Not Before {} (CN: {})",
                    anchor.not_before_str,
                    anchor.subject_cn,
                );
                entry.trust_finding = Some(TrustFinding::IssuerRotationMismatch {
                    image_signed: signed_at_str.clone(),
                    trust_cert_not_before: anchor.not_before_str.clone(),
                    subject_cn: anchor.subject_cn.clone(),
                });
                break;
            }
        }
    }

    #[cfg(debug_assertions)]
    log::debug!(
        target: "umrs",
        "IssuerRotationMismatch classification completed in {} µs",
        start.elapsed().as_micros()
    );
}

/// Inspect a single (signing timestamp, issuer, trust anchor PEM) combination
/// for a certificate rotation mismatch.
///
/// This function exposes the rotation mismatch detection logic as a standalone
/// public API. It is the primary entry point for integration tests and for
/// callers that want to classify a known-untrusted result without re-reading a
/// full chain.
///
/// Returns `Some(TrustFinding::IssuerRotationMismatch)` when:
/// - `signed_at_rfc3339` is a valid RFC 3339 timestamp
/// - `anchor_pem` contains at least one cert whose CN overlaps `issuer_cn`
/// - That cert's `Not Before` is after `signed_at_rfc3339`
///
/// Returns `None` if:
/// - The timestamp cannot be parsed
/// - No matching cert is found in `anchor_pem`
/// - The matching cert's `Not Before` is before the signing date
///   (no temporal mismatch)
///
/// ## Compliance
///
/// - **NIST SP 800-53 AU-3**: Audit Record Content — the returned finding is
///   a structured enum variant, not a log string, enabling programmatic audit
///   consumption.
/// - **NIST SP 800-53 SI-7**: Software, Firmware, and Information Integrity —
///   the function is fail-closed: `None` never elevates trust.
#[must_use = "The returned TrustFinding explains why trust validation failed; \
              discarding it means the operator sees no explanation for UNVERIFIED"]
pub fn inspect_rotation_mismatch(
    signed_at_rfc3339: &str,
    issuer_cn: &str,
    anchor_pem: &str,
) -> Option<TrustFinding> {
    let signed_dt = DateTime::parse_from_rfc3339(signed_at_rfc3339).ok()?;
    let anchor_certs = extract_anchor_certs(anchor_pem);

    for anchor in &anchor_certs {
        let Some(not_before_dt) = anchor.not_before_dt else {
            continue;
        };
        let is_after_signing = not_before_dt > signed_dt;
        let cn_overlaps =
            issuer_cn.contains(anchor.subject_cn.as_str()) || anchor.subject_cn.contains(issuer_cn);

        if is_after_signing && cn_overlaps {
            return Some(TrustFinding::IssuerRotationMismatch {
                image_signed: signed_at_rfc3339.to_string(),
                trust_cert_not_before: anchor.not_before_str.clone(),
                subject_cn: anchor.subject_cn.clone(),
            });
        }
    }
    None
}

/// Read the chain of custody from a file's C2PA manifest store.
///
/// Builds c2pa SDK `Settings` from the configured trust list paths, creates a
/// `Context` with those settings, and reads the manifest store through that
/// context so that trust validation is performed against the configured CAs.
///
/// Returns entries ordered oldest-first (deepest ingredient → active manifest).
/// Returns an empty `Vec` if the file has no manifest.
///
/// For `Untrusted` entries, the `trust_finding` field is populated when a
/// specific cause can be identified (e.g., [`TrustFinding::IssuerRotationMismatch`]).
///
/// # Errors
///
/// Returns `InspectError::C2pa` if the manifest store cannot be read, or
/// `InspectError::Config` if the manifest JSON is malformed or if trust
/// settings cannot be assembled.  Returns `InspectError::Io` if any trust
/// anchor PEM file cannot be read from disk.
///
/// ## Compliance
///
/// - **NIST SP 800-53 SC-13**: Cryptographic Protection — trust validation
///   against configured CA anchors is performed on every read.
/// - **NSA RTB RAIN**: Non-Bypassability — all manifest reads flow through
///   this function, which always calls `build_c2pa_settings`.
#[must_use = "Trust validation results are returned in ChainEntry::trust_status; \
              discarding the chain means trust decisions are silently bypassed"]
pub fn read_chain(path: &Path, config: &UmrsConfig) -> Result<Vec<ChainEntry>, InspectError> {
    verbose!("Building trust settings...");
    let settings = build_c2pa_settings(config)?;
    let context = c2pa::Context::new()
        .with_settings(settings)
        .map_err(|e| InspectError::Config(format!("c2pa context settings error: {e}")))?;

    verbose!("Opening C2PA manifest store...");
    let reader = match c2pa::Reader::from_context(context).with_file(path) {
        Ok(r) => r,
        Err(c2pa::Error::JumbfNotFound | c2pa::Error::ProvenanceMissing) => {
            verbose!("No C2PA manifest found in file");
            return Ok(Vec::new());
        }
        Err(e) => return Err(InspectError::C2pa(e)),
    };

    verbose!("Parsing manifest store JSON...");
    let store_json: serde_json::Value = serde_json::from_str(&reader.json())
        .map_err(|e| InspectError::Config(format!("manifest JSON parse error: {e}")))?;

    verbose!("Walking chain of custody...");
    let mut entries: Vec<ChainEntry> = Vec::new();
    collect_entries(&store_json, &mut entries);
    verbose!("Found {} chain entries", entries.len());

    // Post-process: classify rotation mismatches for any Untrusted entries.
    // Only attempt if trust is configured — without a trust list we have no
    // anchor certs to compare against.
    if config.trust.verify_trust && config.has_trust_config() {
        let anchor_pem = read_anchor_pem_for_classification(config);
        if !anchor_pem.is_empty() {
            let anchor_certs = extract_anchor_certs(&anchor_pem);
            classify_rotation_mismatches(&mut entries, &anchor_certs);
        }
    }

    Ok(entries)
}

/// Read the trust anchor PEM content for rotation mismatch classification.
///
/// Attempts to read `trust_anchors` (and `user_anchors`) from disk. Returns
/// an empty string on any I/O error — classification silently degrades to
/// "no finding" rather than failing the whole chain read, which has already
/// succeeded. The PEM is used only for CN/date comparisons; it is never
/// passed back to the SDK.
///
/// This is distinct from `trust::read_pem` because we do not need `O_NOFOLLOW`
/// security here: we are not injecting this content into a cryptographic
/// trust decision. The SDK has already performed that decision using the
/// content read via `trust::build_c2pa_settings`. This secondary read is
/// purely diagnostic — it cannot elevate trust.
fn read_anchor_pem_for_classification(config: &UmrsConfig) -> String {
    let mut pem = String::new();
    if let Some(path) = &config.trust.trust_anchors
        && let Ok(content) = std::fs::read_to_string(path)
    {
        pem.push_str(&content);
    }
    if let Some(path) = &config.trust.user_anchors
        && let Ok(content) = std::fs::read_to_string(path)
    {
        if !pem.is_empty() && !pem.ends_with('\n') {
            pem.push('\n');
        }
        pem.push_str(&content);
    }
    pem
}

/// Returns `true` if the file contains any C2PA manifest data.
///
/// This probe uses the raw c2pa reader without trust settings — its purpose
/// is only to detect manifest presence, not to validate trust. All callers
/// that need trust-validated results must use `read_chain`. The intentional
/// bypass here is safe because no trust decision flows from this function:
/// it returns a bool, not a `TrustStatus` or `ChainEntry`.
#[must_use = "The presence/absence of a manifest determines whether trust \
              validation and chain walking should proceed"]
pub fn has_manifest(path: &Path) -> bool {
    c2pa::Reader::from_file(path).is_ok()
}

/// Returns the full manifest store as a pretty-printed JSON string.
///
/// This is the **raw c2pa SDK output** — the complete manifest store as the
/// crate emits it, including all assertions, ingredients, and signature info.
/// Trust settings are applied from `config` so the JSON includes real
/// `validation_status` entries when anchors are configured.
///
/// # Errors
///
/// Returns `InspectError::C2pa` if the manifest store cannot be read, or
/// `InspectError::Config` if JSON parsing, serialization, or trust settings
/// assembly fails.  Returns `InspectError::Io` if any trust PEM file cannot
/// be read.
#[must_use = "Manifest JSON is the raw evidence output; discarding it silently \
              loses the operator's requested inspection data"]
pub fn manifest_json(path: &Path, config: &UmrsConfig) -> Result<String, InspectError> {
    let settings = build_c2pa_settings(config)?;
    let context = c2pa::Context::new()
        .with_settings(settings)
        .map_err(|e| InspectError::Config(format!("c2pa context settings error: {e}")))?;
    let reader = c2pa::Reader::from_context(context).with_file(path).map_err(InspectError::C2pa)?;
    let val: serde_json::Value = serde_json::from_str(&reader.json())
        .map_err(|e| InspectError::Config(format!("manifest JSON parse: {e}")))?;
    serde_json::to_string_pretty(&val)
        .map_err(|e| InspectError::Config(format!("JSON serialize: {e}")))
}

/// Returns the UMRS-parsed chain of custody as a JSON string.
///
/// Unlike `manifest_json()` which returns the raw c2pa SDK manifest store,
/// this function returns the **parsed evidence chain** — the same data
/// displayed in the human-readable report, serialized as JSON for
/// programmatic consumption by other tools.
///
/// The returned JSON is an array of objects ordered oldest-first:
///
/// ```json
/// [
///   {
///     "signer_name": "Truepic Lens CLI in Sora",
///     "issuer": "OpenAI",
///     "signed_at": null,
///     "trust_status": "NO_TRUST_LIST",
///     "algorithm": "Es256",
///     "generator": "ChatGPT",
///     "generator_version": null
///   }
/// ]
/// ```
///
/// Returns an empty array `[]` if the file has no C2PA manifest.
///
/// # Errors
///
/// Returns `InspectError::C2pa` if the manifest store cannot be read, or
/// `InspectError::Config` if JSON serialization or trust settings assembly
/// fails.  Returns `InspectError::Io` if any trust PEM file cannot be read.
#[must_use = "Chain JSON carries the full custody record; discarding it silently \
              loses the programmatic evidence output"]
pub fn chain_json(path: &Path, config: &UmrsConfig) -> Result<String, InspectError> {
    let chain = read_chain(path, config)?;
    serde_json::to_string_pretty(&chain)
        .map_err(|e| InspectError::Config(format!("JSON serialize: {e}")))
}

/// Returns the UMRS-parsed chain of custody as a JSON string, including
/// SHA-256 and SHA-384 file integrity digests.
///
/// Produces a JSON object with `sha256`, `sha384`, and `chain` fields.
/// The `chain` array contains the same entries as `chain_json()`, ordered
/// oldest-first. Both hash digests must be pre-computed by the caller from
/// the source file so that the hashes and the chain describe the same bytes.
///
/// ```json
/// {
///   "sha256": "abcdef...",
///   "sha384": "123456...",
///   "chain": [ ... ]
/// }
/// ```
///
/// Returns `chain: []` if the file has no C2PA manifest.
///
/// # Errors
///
/// Returns `InspectError::C2pa` if the manifest store cannot be read, or
/// `InspectError::Config` if JSON serialization or trust settings assembly
/// fails.  Returns `InspectError::Io` if any trust PEM file cannot be read.
///
/// ## Compliance
///
/// - **NIST SP 800-53 AU-10**: Non-repudiation — the JSON output carries
///   both integrity digests alongside the full custody chain for forensic use.
/// - **NIST SP 800-53 SC-13**: SHA-256 and SHA-384 are computed via system
///   OpenSSL (FIPS 140-2/3 validated module on RHEL 10).
/// - **CNSA 2.0**: SHA-384 satisfies CNSA 2.0 hash algorithm requirements.
#[must_use = "Chain report JSON carries integrity digests and the full custody record; \
              discarding it silently loses the programmatic evidence output"]
pub fn chain_report_json(
    path: &Path,
    sha256: &str,
    sha384: &str,
    config: &UmrsConfig,
) -> Result<String, InspectError> {
    let chain = read_chain(path, config)?;
    let report = serde_json::json!({
        "sha256": sha256,
        "sha384": sha384,
        "chain": chain,
    });
    serde_json::to_string_pretty(&report)
        .map_err(|e| InspectError::Config(format!("JSON serialize: {e}")))
}

/// Returns the most recent signer name and timestamp from the active manifest.
///
/// Used for the ingest log entry in the "has manifest" case.  Trust settings
/// from `config` are applied so the SDK performs real validation.
///
/// # Errors
///
/// Returns `InspectError::C2pa` if the manifest store cannot be read, or
/// `InspectError::Config` if the manifest JSON is malformed or trust settings
/// assembly fails.  Returns `InspectError::Io` if any trust PEM file cannot
/// be read.
pub fn last_signer(
    path: &Path,
    config: &UmrsConfig,
) -> Result<Option<(String, Option<String>)>, InspectError> {
    let chain = read_chain(path, config)?;
    Ok(chain.last().map(|e| (e.signer_name.clone(), e.signed_at.clone())))
}

// Walk the manifest store JSON and collect chain entries.
// The store JSON has an `active_manifest` key and a `manifests` map.
// We walk from the active manifest back through ingredients.
fn collect_entries(store: &serde_json::Value, out: &mut Vec<ChainEntry>) {
    let Some(manifests) = store.get("manifests").and_then(|m| m.as_object()) else {
        return;
    };
    let Some(active_id) = store.get("active_manifest").and_then(|v| v.as_str()) else {
        return;
    };

    // Walk the chain recursively: ingredients first, then the active manifest.
    walk_manifest(
        active_id,
        manifests,
        out,
        &mut std::collections::HashSet::new(),
    );

    // The c2pa SDK places validation_status at the **store level**, not inside
    // individual manifests.  Per-manifest validation_status is absent for most
    // real-world files.  We must derive trust from the store-level array and
    // propagate it to chain entries that have no per-manifest status.
    //
    // Priority: tamper detection first (mismatch/failed → Invalid), then trust
    // evaluation (trusted/untrusted/revoked → override NoTrustList entries).
    if let Some(statuses) = store.get("validation_status").and_then(|v| v.as_array()) {
        let codes: Vec<&str> =
            statuses.iter().filter_map(|s| s.get("code").and_then(|v| v.as_str())).collect();

        // Tamper detection — affects all entries unconditionally.
        let store_tampered = codes.iter().any(|c| c.contains("mismatch") || c.contains("failed"));

        if store_tampered {
            for entry in out.iter_mut() {
                if entry.trust_status != TrustStatus::Invalid {
                    entry.trust_status = TrustStatus::Invalid;
                }
            }
        } else {
            // Trust evaluation — apply store-level trust to entries that had no
            // per-manifest validation_status (currently showing NoTrustList).
            let store_trust = if codes.iter().any(|c| c.contains("revoked")) {
                Some(TrustStatus::Revoked)
            } else if codes.contains(&"signingCredential.trusted") {
                Some(TrustStatus::Trusted)
            } else if codes.iter().any(|c| c.contains("untrusted")) {
                Some(TrustStatus::Untrusted)
            } else {
                None
            };

            if let Some(ref trust) = store_trust {
                for entry in out.iter_mut() {
                    if entry.trust_status == TrustStatus::NoTrustList {
                        entry.trust_status = trust.clone();
                    }
                }
            }
        }
    }
}

fn walk_manifest(
    id: &str,
    manifests: &serde_json::Map<String, serde_json::Value>,
    out: &mut Vec<ChainEntry>,
    visited: &mut std::collections::HashSet<String>,
) {
    if !visited.insert(id.to_string()) {
        return; // cycle guard
    }

    let Some(manifest) = manifests.get(id) else {
        return;
    };

    // Recurse into ingredients first (oldest-first ordering).
    if let Some(ingredients) = manifest.get("ingredients").and_then(|v| v.as_array()) {
        for ingredient in ingredients {
            if let Some(manifest_ref) = ingredient.get("active_manifest").and_then(|v| v.as_str()) {
                walk_manifest(manifest_ref, manifests, out, visited);
            }
        }
    }

    // Extract this manifest's entry.
    let entry = extract_entry(manifest);
    out.push(entry);
}

fn extract_entry(manifest: &serde_json::Value) -> ChainEntry {
    let sig_info = manifest.get("signature_info");

    // Signer identity: prefer common_name (cert CN, e.g. "Truepic Lens CLI
    // in Sora"), fall back to issuer, then claim_generator.
    let signer_name = sig_info
        .and_then(|s| s.get("common_name"))
        .and_then(|v| v.as_str())
        .or_else(|| sig_info.and_then(|s| s.get("issuer")).and_then(|v| v.as_str()))
        .or_else(|| manifest.get("claim_generator").and_then(|v| v.as_str()))
        .unwrap_or("Unknown")
        .to_string();

    // Issuer: the organization that issued the signing certificate.
    let issuer = sig_info
        .and_then(|s| s.get("issuer"))
        .and_then(|v| v.as_str())
        .unwrap_or("Unknown")
        .to_string();

    // Timestamp: prefer TSA timestamp from signature_info, fall back to the
    // "when" field from the first action assertion (e.g. UMRS ingest time).
    let signed_at = sig_info
        .and_then(|s| s.get("time"))
        .and_then(|v| v.as_str())
        .map(std::string::ToString::to_string)
        .or_else(|| extract_action_when(manifest));

    let algorithm = sig_info
        .and_then(|s| s.get("alg"))
        .and_then(|v| v.as_str())
        .unwrap_or("unknown")
        .to_string();

    // Extract claim_generator info — prefer structured claim_generator_info
    // array, fall back to parsing the claim_generator string.
    let (generator, generator_version) = extract_generator_info(manifest);

    // Derive trust status from validation_status codes if present.
    let trust_status = derive_trust(manifest);

    // Extract security label from umrs.security-label assertion.
    let security_label = extract_security_label(manifest);

    ChainEntry {
        signer_name,
        issuer,
        signed_at,
        trust_status,
        trust_finding: None,
        algorithm,
        generator,
        generator_version,
        security_label,
    }
}

/// Extract the `when` timestamp from the first action in `c2pa.actions` or
/// `c2pa.actions.v2`. Returns `None` if no action has a `when` field.
fn extract_action_when(manifest: &serde_json::Value) -> Option<String> {
    let assertions = manifest.get("assertions").and_then(|v| v.as_array())?;
    for assertion in assertions {
        let label = assertion.get("label").and_then(|v| v.as_str()).unwrap_or("");
        if label == "c2pa.actions" || label == "c2pa.actions.v2" {
            let actions =
                assertion.get("data").and_then(|d| d.get("actions")).and_then(|a| a.as_array())?;
            for action in actions {
                if let Some(when) = action.get("when").and_then(|v| v.as_str()) {
                    return Some(when.to_string());
                }
            }
        }
    }
    None
}

/// Extract a security label from a `umrs.security-label` assertion, if present.
fn extract_security_label(manifest: &serde_json::Value) -> Option<String> {
    let assertions = manifest.get("assertions").and_then(|v| v.as_array())?;
    for assertion in assertions {
        let label = assertion.get("label").and_then(|v| v.as_str()).unwrap_or("");
        if label == "umrs.security-label" {
            return assertion
                .get("data")
                .and_then(|d| d.get("marking"))
                .and_then(|v| v.as_str())
                .map(String::from);
        }
    }
    None
}

/// Extract generator name and version from the manifest.
///
/// Prefers `claim_generator_info` (structured array). Looks for:
///   1. `version` field (standard C2PA, e.g. UMRS sets this)
///   2. `org.contentauth.c2pa_rs` vendor extension (used by OpenAI/ChatGPT
///      to record the c2pa-rs SDK version — not the app version, but still
///      useful for forensics)
///
/// Falls back to parsing the `claim_generator` string, which often has the
/// form `"Name/Version"`.
fn extract_generator_info(manifest: &serde_json::Value) -> (String, Option<String>) {
    // Try claim_generator_info array first (C2PA 2.x style).
    if let Some(info_arr) = manifest.get("claim_generator_info").and_then(|v| v.as_array())
        && let Some(first) = info_arr.first()
    {
        let name = first.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown");

        // Only use the explicit "version" field — vendor extensions like
        // "org.contentauth.c2pa_rs" are internal SDK version numbers,
        // not meaningful to end users.
        let version = first.get("version").and_then(|v| v.as_str()).map(String::from);

        return (name.to_string(), version);
    }

    // Fall back to claim_generator string — split on "/" for name/version.
    if let Some(cg) = manifest.get("claim_generator").and_then(|v| v.as_str()) {
        if let Some((name, version)) = cg.split_once('/') {
            return (name.trim().to_string(), Some(version.trim().to_string()));
        }
        return (cg.to_string(), None);
    }

    ("Unknown".to_string(), None)
}

fn derive_trust(manifest: &serde_json::Value) -> TrustStatus {
    let Some(statuses) = manifest.get("validation_status").and_then(|v| v.as_array()) else {
        // No validation_status array — common for ingredient manifests and
        // self-signed output.  No trust list was evaluated.
        return TrustStatus::NoTrustList;
    };

    if statuses.is_empty() {
        return TrustStatus::NoTrustList;
    }

    let codes: Vec<&str> =
        statuses.iter().filter_map(|s| s.get("code").and_then(|v| v.as_str())).collect();

    if codes.iter().any(|c| c.contains("revoked")) {
        return TrustStatus::Revoked;
    }
    if codes.iter().any(|c| c.contains("mismatch") || c.contains("failed")) {
        return TrustStatus::Invalid;
    }
    if codes.contains(&"signingCredential.trusted") {
        return TrustStatus::Trusted;
    }
    if codes.iter().any(|c| c.contains("untrusted")) {
        return TrustStatus::Untrusted;
    }
    // Catch-all: status codes were present but none matched a recognized pattern.
    // Returning Untrusted (not Trusted) is the correct fail-closed behavior here:
    // an unrecognized status code must never be treated as a trust grant.
    // This satisfies NSA RTB Fail Secure — when in doubt, deny.
    TrustStatus::Untrusted
}
