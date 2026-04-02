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
//! ## Compliance
//!
//! - **NIST SP 800-53 AU-10**: Non-repudiation — the chain walk surfaces every
//!   signing event across the full provenance history of a file.
//! - **NIST SP 800-53 AU-3**: Audit Record Content — `ChainEntry` carries
//!   signer identity, issuer, timestamp, algorithm, and security marking.
//! - **NIST SP 800-53 SI-7**: Software, Firmware, and Information Integrity —
//!   `TrustStatus::Invalid` is set whenever the c2pa SDK reports a hash
//!   mismatch or signature verification failure, making tampering visible.

use std::path::Path;

use serde::Serialize;

use crate::c2pa::{config::UmrsConfig, error::InspectError, trust::build_c2pa_settings};
#[allow(unused_imports)]
use crate::verbose;

/// Trust evaluation for a single entry in the chain of custody.
///
/// | Status        | Display         | Meaning |
/// |---------------|-----------------|---------|
/// | `Trusted`     | `TRUSTED`       | Cert chain verified against a C2PA Trust List root CA |
/// | `Untrusted`   | `UNVERIFIED`    | Signature present but not validated against a trust list |
/// | `Invalid`     | `INVALID`       | Signature verification failed or asset hash mismatch |
/// | `Revoked`     | `REVOKED`       | Signing certificate was revoked by the issuing CA |
/// | `NoTrustList` | `NO TRUST LIST` | No trust list configured — cannot evaluate trust |
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
    /// Cert chain leads to a root CA in the C2PA Trust List.
    #[serde(rename = "TRUSTED")]
    Trusted,
    /// Signature is present but the CA is not on the Trust List,
    /// or no trust list was configured. The signature has not been
    /// validated — it is not necessarily bad, just unverified.
    #[serde(rename = "UNVERIFIED")]
    Untrusted,
    /// Signature verification failed, or asset hash does not match.
    #[serde(rename = "INVALID")]
    Invalid,
    /// Certificate was revoked by the issuing CA.
    #[serde(rename = "REVOKED")]
    Revoked,
    /// No trust list is configured, so trust cannot be evaluated.
    /// Distinct from Untrusted: this means we did not even attempt validation.
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

/// A single entry in the chain of custody, extracted from one manifest
/// in the manifest store.
#[derive(Debug, Clone, Serialize)]
pub struct ChainEntry {
    /// Signer identity — from the cert CN or `claim_generator` field.
    pub signer_name: String,

    /// CA that issued the signing certificate.
    pub issuer: String,

    /// Signing timestamp, if present.
    pub signed_at: Option<String>,

    /// Trust evaluation for this entry.
    pub trust_status: TrustStatus,

    /// Signing algorithm used (e.g. "es256").
    pub algorithm: String,

    /// Claim generator name (e.g. "`ChatGPT`", "UMRS Reference System").
    pub generator: String,

    /// Claim generator version, if available (e.g. "0.67.1").
    pub generator_version: Option<String>,

    /// Security label / marking from a `umrs.security-label` assertion, if present.
    pub security_label: Option<String>,
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
    Ok(entries)
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
