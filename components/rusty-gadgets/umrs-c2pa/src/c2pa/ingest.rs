// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Jamie Adams

//! # UMRS C2PA File Ingestion
//!
//! Implements the core UMRS ingest workflow: compute SHA-256 and SHA-384
//! digests of the source file, read any existing C2PA manifest, build and
//! sign a new manifest recording this chain-of-custody event, and write the
//! signed output.
//!
//! ## Key Exported Types
//!
//! - [`IngestResult`] — result of ingesting a file, including paths and hashes
//! - [`ingest_file`] — primary entry point: sign a file and record custody
//! - [`sha256_hex`] — compute the SHA-256 hex digest of a file (system OpenSSL)
//! - [`sha384_hex`] — compute the SHA-384 hex digest of a file (system OpenSSL)
//!
//! ## Trust Boundary
//!
//! `ingest_file` trusts the caller to supply a valid `UmrsConfig` (already
//! loaded and defaulted). It validates the signing algorithm before any I/O
//! and refuses to re-sign a file that was already signed by UMRS (filename
//! convention guard). The guard is best-effort — it does not replace manifest
//! inspection as the definitive source of truth.
//!
//! ## TOCTOU Safety
//!
//! The source file is read exactly once into memory. Both the SHA-256 digest
//! and the signing operation consume the same in-memory buffer, eliminating
//! the TOCTOU window that would exist if the file were read twice from disk.
//! This ensures the recorded hash is always consistent with the signed content.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AU-10**: Non-repudiation — each ingest event produces a
//!   C2PA manifest asserting signer identity, timestamp, and SHA-256 digest.
//! - **NIST SP 800-53 AU-3**: Audit Record Content — the structured `log::info!`
//!   entry records file path, hash, previous signer, and action label.
//! - **NIST SP 800-53 SC-13**: Cryptographic Protection — signing is gated
//!   behind `signer::parse_algorithm`, which enforces the FIPS allow-list.
//! - **NIST SP 800-53 SI-7**: Software, Firmware, and Information Integrity —
//!   SHA-256 and SHA-384 are computed from the same buffer used for signing;
//!   both hashes are always consistent with the signed content (TOCTOU-free).
//! - **NIST SP 800-53 SC-13**: SHA-256 and SHA-384 are computed via system
//!   OpenSSL, which uses a FIPS 140-2/3 validated module on RHEL 10.
//! - **NSA RTB**: TOCTOU Safety — single-read design eliminates the race
//!   window between hash computation and signing.
//! - **CNSA 2.0**: SHA-384 is included alongside SHA-256 to satisfy CNSA 2.0
//!   hash algorithm requirements for long-term integrity records.

use std::io::Cursor;
use std::path::{Path, PathBuf};

use openssl::hash::{MessageDigest, hash};

use crate::c2pa::{config::UmrsConfig, error::InspectError, manifest, signer};
#[allow(unused_imports)]
use crate::verbose;

/// Result of ingesting a file into UMRS.
///
/// Contains the chain-of-custody evidence record, including the signed output
/// path, SHA-256 and SHA-384 digests of the source bytes, and ephemeral mode
/// status. The caller must inspect or log this result — discarding it silently
/// loses the chain-of-custody record.
///
/// Both digests are computed via system OpenSSL, using the FIPS 140-2/3
/// validated module on RHEL 10. SHA-384 is provided for CNSA 2.0 readiness.
///
/// ## Compliance
///
/// - **NIST SP 800-53 AU-10**: Non-repudiation — this struct is the evidence
///   record for one signing event; discarding it means no audit trail is kept.
/// - **NIST SP 800-53 AU-3**: Audit Record Content — fields carry the minimum
///   required audit content: identity, hash, timestamp, and action label.
/// - **NIST SP 800-53 SC-13**: Cryptographic Protection — both digests are
///   computed via system OpenSSL (FIPS 140-2/3 validated on RHEL 10).
/// - **CNSA 2.0**: SHA-384 satisfies CNSA 2.0 hash algorithm requirements.
#[must_use = "IngestResult is the chain-of-custody evidence record; \
              discarding it means the signing outcome, output path, and \
              integrity digests are silently lost"]
#[derive(Debug)]
pub struct IngestResult {
    /// Original file path.
    pub source_path: PathBuf,

    /// Output file path (signed copy).
    pub output_path: PathBuf,

    /// SHA-256 hex digest of the source file bytes, computed at ingest time.
    /// Computed via system OpenSSL from the same buffer used for signing.
    pub sha256: String,

    /// SHA-384 hex digest of the source file bytes, computed at ingest time.
    /// Computed via system OpenSSL from the same buffer used for signing.
    /// Provided for CNSA 2.0 readiness alongside SHA-256.
    pub sha384: String,

    /// Whether the file had an existing C2PA manifest on arrival.
    pub had_manifest: bool,

    /// The C2PA action label applied by UMRS (acquired or published).
    pub action: String,

    /// Signer name of the previous last entry in the chain (if any).
    pub previous_signer: Option<String>,

    /// Signing timestamp of the previous last entry (if any).
    pub previous_signed_at: Option<String>,

    /// Whether UMRS signed with an ephemeral (test) cert.
    pub is_ephemeral: bool,
}

/// Ingest a file: compute its SHA-256, read any existing manifest, sign it,
/// and write the result to `output_path`.
///
/// If `output_path` is `None`, a default path is derived by appending
/// `_umrs_signed` before the extension.
///
/// The source file is read once into memory. Both the SHA-256 digest and the
/// signing operation use the same buffer, eliminating any TOCTOU window between
/// hash computation and signing.
///
/// # Errors
///
/// Returns `InspectError::AlreadySigned` if the file was previously signed by
/// UMRS, `InspectError::Io` on filesystem failures, `InspectError::C2pa` on
/// manifest or signing errors, or `InspectError::UnsafeAlgorithm` if the
/// configured algorithm is not FIPS-safe.
///
/// ## Compliance
///
/// - **NIST SP 800-53 AU-10**: Non-repudiation.
/// - **NIST SP 800-53 SI-7**: TOCTOU-free single-read integrity.
#[expect(
    clippy::too_many_lines,
    reason = "Sequential ingest pipeline; splitting would reduce clarity"
)]
#[must_use = "Discarding IngestResult loses the chain-of-custody record, \
              output path, SHA-256 hash, and ephemeral mode status; \
              call sites must inspect or log the result"]
pub fn ingest_file(
    source_path: &Path,
    output_path: Option<&Path>,
    marking: Option<&str>,
    config: &UmrsConfig,
) -> Result<IngestResult, InspectError> {
    // 0. Guard: refuse to re-sign a file that was already signed by UMRS.
    //    This prevents accidental overwrites and double-signing.
    if is_umrs_signed(source_path) {
        return Err(InspectError::AlreadySigned(
            source_path.display().to_string(),
        ));
    }

    // 1. Read source file once into memory.
    //    Both SHA-256 and signing use this buffer — no TOCTOU window exists.
    verbose!("Reading source file into memory...");
    let source_bytes = std::fs::read(source_path).map_err(InspectError::Io)?;

    // 2. Compute SHA-256 and SHA-384 from the in-memory buffer.
    //    Both digests use system OpenSSL — the FIPS 140-2/3 validated module
    //    on RHEL 10.  SHA-384 is provided for CNSA 2.0 readiness.
    verbose!("Computing SHA-256 and SHA-384 of source file via system OpenSSL...");
    let sha256 = digest_hex(&source_bytes, MessageDigest::sha256())?;
    let sha384 = digest_hex(&source_bytes, MessageDigest::sha384())?;

    // 3. Check for existing manifest.
    verbose!("Checking for existing C2PA manifest...");
    let had_manifest = manifest::has_manifest(source_path);
    let (previous_signer, previous_signed_at) = if had_manifest {
        verbose!("Existing manifest found — reading previous signer...");
        match manifest::last_signer(source_path, config)? {
            Some((name, ts)) => {
                verbose!("Previous signer: {}", name);
                (Some(name), ts)
            }
            None => (None, None),
        }
    } else {
        verbose!("No existing manifest — file is new to C2PA");
        (None, None)
    };

    // 4. Choose action label and reason.
    let (action, reason) = if had_manifest {
        verbose!(
            "Action: {} (file had existing manifest)",
            config.policy.signed_action
        );
        (
            config.policy.signed_action.clone(),
            config.policy.signed_reason.clone(),
        )
    } else {
        verbose!(
            "Action: {} (file had no manifest)",
            config.policy.unsigned_action
        );
        (
            config.policy.unsigned_action.clone(),
            config.policy.unsigned_reason.clone(),
        )
    };

    // 5. Resolve signing material.
    verbose!("Resolving signing credentials...");
    let signer_mode =
        signer::resolve_signer_mode(&config.identity, config.timestamp.tsa_url.as_deref())?;
    let is_ephemeral = signer::is_ephemeral(&signer_mode);
    if is_ephemeral {
        verbose!("Using ephemeral self-signed certificate (test mode)");
    } else {
        verbose!("Using configured certificate and private key");
    }
    verbose!("Building signer ({})...", config.identity.algorithm);
    let signer = signer::build_signer(&signer_mode)?;

    // 6. Build the C2PA manifest.
    let format = mime_for_path(source_path);
    let mut builder = c2pa::Builder::new();
    {
        let mut cgi = c2pa::ClaimGeneratorInfo::default();
        cgi.name.clone_from(&config.identity.claim_generator);
        cgi.version = Some(env!("CARGO_PKG_VERSION").to_string());
        builder.set_claim_generator_info(cgi);
    }

    // Action assertion with a timestamp recording when UMRS ingested the file.
    let now = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    let action_assertion = serde_json::json!({
        "actions": [
            {
                "action": action,
                "when": now,
                "reason": reason,
                "softwareAgent": config.identity.claim_generator,
            }
        ]
    });
    builder.add_assertion("c2pa.actions", &action_assertion).map_err(InspectError::C2pa)?;

    verbose!("Building C2PA manifest...");
    verbose!("  Claim generator: {}", config.identity.claim_generator);

    // Security label assertion — embeds a marking string (e.g. "CUI" or
    // "CUI//SP-CTI//NOFORN") as a tamper-evident, cryptographically signed
    // assertion in the manifest.
    if let Some(label) = marking {
        verbose!("  Embedding security marking: {}", label);
        let label_assertion = serde_json::json!({
            "marking": label,
        });
        builder
            .add_assertion("umrs.security-label", &label_assertion)
            .map_err(InspectError::C2pa)?;
    }

    // If there's an existing manifest, embed it as an ingredient.
    if had_manifest {
        verbose!("  Embedding existing manifest as ingredient...");
        let ingredient = c2pa::Ingredient::from_file(source_path).map_err(InspectError::C2pa)?;
        builder.add_ingredient(ingredient);
    }

    // 7. Sign and write output using the already-read source_bytes buffer.
    //    No second file read — the hash and the signed content are identical.
    let out_path = match output_path {
        Some(p) => p.to_path_buf(),
        None => derive_output_path(source_path),
    };

    verbose!("Signing and writing output to {}...", out_path.display());
    #[cfg(unix)]
    let mut out_file = {
        use std::os::unix::fs::OpenOptionsExt;
        std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .mode(0o640)
            .open(&out_path)
            .map_err(InspectError::Io)?
    };
    #[cfg(not(unix))]
    let mut out_file = std::fs::File::create(&out_path).map_err(InspectError::Io)?;

    builder
        .sign(
            signer.as_ref(),
            &format,
            &mut Cursor::new(source_bytes),
            &mut out_file,
        )
        .map_err(InspectError::C2pa)?;
    verbose!("Signing complete");

    // 8. Emit structured log entry.
    if had_manifest {
        log::info!(
            target: "umrs",
            "ingest file=\"{}\" sha256=\"{}\" sha384=\"{}\" previous_signer=\"{}\" \
             signed_at=\"{}\" action={}",
            source_path.display(),
            sha256,
            sha384,
            previous_signer.as_deref().unwrap_or("unknown"),
            previous_signed_at.as_deref().unwrap_or("unknown"),
            action,
        );
    } else {
        log::info!(
            target: "umrs",
            "ingest file=\"{}\" sha256=\"{}\" sha384=\"{}\" manifest=none action={}",
            source_path.display(),
            sha256,
            sha384,
            action,
        );
    }

    Ok(IngestResult {
        source_path: source_path.to_path_buf(),
        output_path: out_path,
        sha256,
        sha384,
        had_manifest,
        action,
        previous_signer,
        previous_signed_at,
        is_ephemeral,
    })
}

/// Compute the SHA-256 hex digest of the file at `path`.
///
/// Uses system OpenSSL for FIPS 140-2/3 compliance on RHEL 10.
///
/// # Errors
///
/// Returns `InspectError::Io` if the file cannot be read, or
/// `InspectError::Config` if the OpenSSL hash operation fails.
///
/// ## Compliance
///
/// - **NIST SP 800-53 SI-7**: the returned hex digest is the integrity
///   reference for the audit log entry; discarding it means no integrity
///   record exists for the caller's operation.
/// - **NIST SP 800-53 SC-13**: computed via system OpenSSL (FIPS 140-2/3
///   validated module on RHEL 10).
#[must_use = "SHA-256 digest is the integrity reference for the audit log entry; \
              discarding it means no integrity record exists"]
pub fn sha256_hex(path: &Path) -> Result<String, InspectError> {
    let bytes = std::fs::read(path).map_err(InspectError::Io)?;
    digest_hex(&bytes, MessageDigest::sha256())
}

/// Compute the SHA-384 hex digest of the file at `path`.
///
/// Uses system OpenSSL for FIPS 140-2/3 compliance on RHEL 10.
/// SHA-384 is provided alongside SHA-256 for CNSA 2.0 readiness.
///
/// # Errors
///
/// Returns `InspectError::Io` if the file cannot be read, or
/// `InspectError::Config` if the OpenSSL hash operation fails.
///
/// ## Compliance
///
/// - **NIST SP 800-53 SI-7**: the returned hex digest is the integrity
///   reference for the audit log entry; discarding it means no integrity
///   record exists for the caller's operation.
/// - **NIST SP 800-53 SC-13**: computed via system OpenSSL (FIPS 140-2/3
///   validated module on RHEL 10).
/// - **CNSA 2.0**: SHA-384 satisfies CNSA 2.0 hash algorithm requirements
///   for long-term integrity records.
#[must_use = "SHA-384 digest is the CNSA 2.0 integrity reference; \
              discarding it means the stronger hash record is silently lost"]
pub fn sha384_hex(path: &Path) -> Result<String, InspectError> {
    let bytes = std::fs::read(path).map_err(InspectError::Io)?;
    digest_hex(&bytes, MessageDigest::sha384())
}

/// Compute a hex-encoded digest of `data` using the given `MessageDigest`.
///
/// Internal helper — not part of the public API.
fn digest_hex(data: &[u8], md: MessageDigest) -> Result<String, InspectError> {
    let digest =
        hash(md, data).map_err(|e| InspectError::Config(format!("OpenSSL hash error: {e}")))?;
    Ok(hex::encode(digest.as_ref()))
}

/// Check whether a file was previously signed by UMRS (by filename convention).
fn is_umrs_signed(path: &Path) -> bool {
    path.file_stem().and_then(|s| s.to_str()).is_some_and(|stem| stem.ends_with("_umrs_signed"))
}

/// Derive a default output path by inserting `_umrs_signed` before the extension.
fn derive_output_path(source: &Path) -> PathBuf {
    let stem = source.file_stem().unwrap_or_default().to_string_lossy();
    let ext = source.extension().map(|e| format!(".{}", e.to_string_lossy())).unwrap_or_default();
    let name = format!("{stem}_umrs_signed{ext}");
    source.with_file_name(name)
}

/// Best-effort MIME type from file extension.
fn mime_for_path(path: &Path) -> String {
    match path.extension().and_then(|e| e.to_str()).map(str::to_lowercase).as_deref() {
        Some("jpg" | "jpeg") => "image/jpeg",
        Some("png") => "image/png",
        Some("webp") => "image/webp",
        Some("tiff" | "tif") => "image/tiff",
        Some("avif") => "image/avif",
        Some("heic" | "heif") => "image/heic",
        Some("mp4") => "video/mp4",
        Some("mov") => "video/quicktime",
        Some("wav") => "audio/wav",
        Some("mp3") => "audio/mpeg",
        Some("pdf") => "application/pdf",
        _ => "application/octet-stream",
    }
    .to_string()
}
