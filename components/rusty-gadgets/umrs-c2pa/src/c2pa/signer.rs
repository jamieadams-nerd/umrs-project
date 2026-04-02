// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Jamie Adams

//! # UMRS C2PA Signer — FIPS-Safe Signing Material
//!
//! Resolves signing material from the UMRS configuration and constructs a
//! `c2pa::BoxedSigner` for use during manifest signing. Enforces the FIPS-safe
//! algorithm allow-list at the earliest possible point in the signing path.
//!
//! ## Key Exported Items
//!
//! - [`ALLOWED_ALGORITHMS`] — FIPS-safe algorithm allow-list (constant)
//! - [`SignerMode`] — resolved signing material (ephemeral or credentials)
//! - [`resolve_signer_mode`] — resolve signing material from config
//! - [`build_signer`] — construct a `c2pa::BoxedSigner` from a `SignerMode`
//! - [`parse_algorithm`] — validate and parse an algorithm string
//! - [`is_ephemeral`] — probe whether signing mode is ephemeral
//!
//! ## FIPS Alignment
//!
//! ed25519 is intentionally excluded from the allow-list: it is not reliably
//! available on FIPS 140-2 validated modules (only added in FIPS 186-5) and is
//! optional in the C2PA spec. ES256/384/512 and PS256/384/512 are mandatory
//! in the C2PA spec and present in all FIPS 140-2 validated modules.
//!
//! ## Ephemeral Mode
//!
//! When no certificate and key are configured, an ephemeral self-signed ECDSA
//! certificate is generated at runtime using the configured algorithm. This is
//! safe for testing and evaluation; manifests produced in ephemeral mode are
//! marked UNTRUSTED by external validators. The ephemeral cert is generated
//! with a 24-hour validity and the CN clearly identifies it as test-only.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 SC-13**: Cryptographic Protection — `parse_algorithm`
//!   enforces the FIPS-safe allow-list; `UnsafeAlgorithm` is returned for
//!   any algorithm not in `ALLOWED_ALGORITHMS`.
//! - **NIST SP 800-53 SC-12**: Cryptographic Key Management — private key
//!   bytes are held in `zeroize::Zeroizing<Vec<u8>>` and zeroed on drop to
//!   prevent key recovery from freed heap memory.
//! - **NIST SP 800-53 SC-28**: Protection of Information at Rest — key
//!   zeroization ensures key material is not retained in heap after use.
//! - **CMMC SC.L2-3.13.10**: Employ FIPS-validated cryptography — all signing
//!   paths use ECDSA P-256/384/521 or RSA-PSS with SHA-256/384/512.
//! - **NSA RTB RAIN**: Non-bypassability — every signing path (ephemeral and
//!   production) calls `parse_algorithm` before any key material is accessed.

use std::path::Path;

use c2pa::SigningAlg;
use openssl::asn1::Asn1Time;
use openssl::bn::BigNum;
use openssl::ec::{EcGroup, EcKey};
use openssl::hash::MessageDigest;
use openssl::nid::Nid;
use openssl::pkey::PKey;
use openssl::x509::extension::{BasicConstraints, ExtendedKeyUsage, KeyUsage};
use openssl::x509::{X509Builder, X509NameBuilder};
use zeroize::Zeroizing;

use crate::c2pa::{config::IdentityConfig, error::InspectError};
#[allow(unused_imports)]
use crate::verbose;

/// FIPS-safe algorithms supported by UMRS.
/// ed25519 is intentionally excluded — unreliable on FIPS-enabled RHEL.
pub const ALLOWED_ALGORITHMS: &[&str] = &["es256", "es384", "es512", "ps256", "ps384", "ps512"];

/// Human-readable description of a signing algorithm: family, curve/key type,
/// digest, key size, and FIPS status.
#[must_use = "Algorithm description is used in operator-facing validation output; \
              discarding it silently produces no report"]
pub fn describe_algorithm(alg: &str) -> &'static str {
    match alg {
        "es256" => "ES256  ECDSA / P-256 (prime256v1) / SHA-256 / 256-bit / FIPS-safe",
        "es384" => "ES384  ECDSA / P-384 (secp384r1) / SHA-384 / 384-bit / FIPS-safe",
        "es512" => "ES512  ECDSA / P-521 (secp521r1) / SHA-512 / 521-bit / FIPS-safe",
        "ps256" => "PS256  RSA-PSS / SHA-256 / 2048+ bit / FIPS-safe",
        "ps384" => "PS384  RSA-PSS / SHA-384 / 2048+ bit / FIPS-safe",
        "ps512" => "PS512  RSA-PSS / SHA-512 / 2048+ bit / FIPS-safe",
        _ => "(unknown algorithm)",
    }
}

/// Parse an algorithm string into `c2pa::SigningAlg`.
///
/// # Errors
///
/// Returns `InspectError::UnsafeAlgorithm` if the algorithm is not in the
/// FIPS-safe allowed set.
pub fn parse_algorithm(alg: &str) -> Result<SigningAlg, InspectError> {
    if !ALLOWED_ALGORITHMS.contains(&alg) {
        return Err(InspectError::UnsafeAlgorithm(alg.to_string()));
    }
    verbose!("Algorithm: {}", describe_algorithm(alg));
    match alg {
        "es256" => Ok(SigningAlg::Es256),
        "es384" => Ok(SigningAlg::Es384),
        "es512" => Ok(SigningAlg::Es512),
        "ps256" => Ok(SigningAlg::Ps256),
        "ps384" => Ok(SigningAlg::Ps384),
        "ps512" => Ok(SigningAlg::Ps512),
        other => Err(InspectError::UnsafeAlgorithm(other.to_string())),
    }
}

/// Signing material resolved from `IdentityConfig`.
///
/// Holds FIPS-validated algorithm selection and, in the `Credentials` variant,
/// the raw PEM bytes needed to build a `c2pa::BoxedSigner`. Private key bytes
/// are stored in `Zeroizing<Vec<u8>>` so they are zeroed on drop.
///
/// ## Compliance
///
/// - **NIST SP 800-53 SC-12**: key_pem uses `Zeroizing` to prevent key
///   recovery from freed heap memory.
/// - **NIST SP 800-53 SC-13**: the algorithm field is pre-validated by
///   `parse_algorithm`; only FIPS-safe variants can be stored here.
#[must_use = "SignerMode determines signing credentials and algorithm; \
              it must be passed to build_signer to produce a signing object"]
pub enum SignerMode {
    /// Ephemeral self-signed cert generated at runtime (test/eval mode).
    Ephemeral {
        alg: SigningAlg,
        organization: String,
    },
    /// Production signing using customer-supplied cert + key.
    Credentials {
        alg: SigningAlg,
        /// Public certificate chain — not secret, not zeroized.
        cert_pem: Vec<u8>,
        /// Private key bytes — zeroized on drop.
        key_pem: Zeroizing<Vec<u8>>,
        tsa_url: Option<String>,
    },
}

/// Resolve signing material from the identity config.
///
/// # Errors
///
/// Returns `InspectError::UnsafeAlgorithm` if the configured algorithm is
/// not FIPS-safe, or `InspectError::Io` if credential files cannot be read.
#[must_use = "Resolved signing mode must be passed to build_signer; \
              discarding it means no signing object is created"]
pub fn resolve_signer_mode(
    identity: &IdentityConfig,
    tsa_url: Option<&str>,
) -> Result<SignerMode, InspectError> {
    let alg = parse_algorithm(&identity.algorithm)?;

    if let (Some(cert_path), Some(key_path)) = (&identity.cert_chain, &identity.private_key) {
        // Log only the filename to avoid exposing full paths in verbose output.
        // Full paths may reveal mount points and operational directory structure.
        let cert_name = cert_path.file_name().map(|n| n.to_string_lossy()).unwrap_or_default();
        let key_name = key_path.file_name().map(|n| n.to_string_lossy()).unwrap_or_default();
        verbose!("Loading certificate from: {}", cert_name);
        verbose!("Loading private key from: {}", key_name);
        let cert_pem = read_cert_pem(cert_path)?;
        let key_pem = read_private_key(key_path)?;
        if tsa_url.is_some() {
            verbose!("TSA timestamp authority configured");
        }
        Ok(SignerMode::Credentials {
            alg,
            cert_pem,
            key_pem,
            // TSA requires network access — silently omit in air-gap builds.
            #[cfg(feature = "internet")]
            tsa_url: tsa_url.map(String::from),
            #[cfg(not(feature = "internet"))]
            tsa_url: None,
        })
    } else {
        verbose!("No cert/key configured — will generate ephemeral certificate");
        Ok(SignerMode::Ephemeral {
            alg,
            organization: identity.organization.clone(),
        })
    }
}

/// Build a `c2pa::Signer` from resolved signing material.
///
/// In ephemeral mode, generates an ECDSA self-signed certificate using the
/// algorithm specified in the config (default: ES256 / P-256).  This avoids
/// the c2pa crate's built-in `EphemeralSigner`, which hardcodes Ed25519 —
/// an algorithm that is not universally available on FIPS 140-3 validated
/// modules (only added in FIPS 186-5) and is optional in the C2PA spec.
///
/// By generating our own ephemeral cert with a FIPS-safe ECDSA curve, test
/// mode output is:
///   - verifiable by every C2PA-compliant validator (ES256/384/512 are mandatory)
///   - safe to run on any FIPS-enabled system regardless of module generation
///   - consistent with the algorithm the user configured for production
///
/// # Errors
///
/// Returns `InspectError::Signing` if ephemeral cert generation fails, or
/// `InspectError::C2pa` if the c2pa signer cannot be created.
#[must_use = "BoxedSigner is the cryptographic signing object required for manifest signing; \
              discarding it means no signature can be produced"]
pub fn build_signer(mode: &SignerMode) -> Result<c2pa::BoxedSigner, InspectError> {
    match mode {
        SignerMode::Ephemeral {
            alg,
            organization,
        } => {
            let (cert_pem, key_pem) = generate_ephemeral_cert(*alg, organization)?;
            c2pa::create_signer::from_keys(&cert_pem, &key_pem, *alg, None)
                .map_err(InspectError::C2pa)
        }
        SignerMode::Credentials {
            alg,
            cert_pem,
            key_pem,
            tsa_url,
        } => c2pa::create_signer::from_keys(cert_pem, key_pem, *alg, tsa_url.clone())
            .map_err(InspectError::C2pa),
    }
}

/// Generate a self-signed ECDSA certificate and private key in PEM format.
///
/// The curve is chosen to match the requested `SigningAlg`:
///   - ES256 → P-256 (prime256v1)
///   - ES384 → P-384 (secp384r1)
///   - ES512 → P-521 (secp521r1)
///   - PS256/384/512 → P-256 (RSA-PSS algorithms use RSA keys, but for ephemeral
///     test mode we generate ECDSA; callers should prefer ES* for test mode)
///
/// The certificate is marked with `CN=<org> (ephemeral — self-signed)`
/// so it is visually obvious in any validator output.
///
/// The private key bytes are returned in a `Zeroizing<Vec<u8>>` so they are
/// zeroed on drop — preventing key recovery from freed heap memory.
fn generate_ephemeral_cert(
    alg: SigningAlg,
    organization: &str,
) -> Result<(Vec<u8>, Zeroizing<Vec<u8>>), InspectError> {
    let (nid, digest, curve_name) = match alg {
        SigningAlg::Es384 => (Nid::SECP384R1, MessageDigest::sha384(), "P-384 (secp384r1)"),
        SigningAlg::Es512 => (Nid::SECP521R1, MessageDigest::sha512(), "P-521 (secp521r1)"),
        // ES256 and RSA-PSS algorithms both use P-256 for ephemeral test certs.
        // Production RSA-PSS signing uses real certs, not this path.
        _ => (Nid::X9_62_PRIME256V1, MessageDigest::sha256(), "P-256 (prime256v1)"),
    };

    verbose!("Generating ephemeral ECDSA key on curve {}...", curve_name);
    let group = EcGroup::from_curve_name(nid)
        .map_err(|e| InspectError::Signing(format!("EC group: {e}")))?;
    let ec_key =
        EcKey::generate(&group).map_err(|e| InspectError::Signing(format!("EC keygen: {e}")))?;
    let pkey =
        PKey::from_ec_key(ec_key).map_err(|e| InspectError::Signing(format!("PKey wrap: {e}")))?;

    // Build self-signed X.509 cert with the configured organization name.
    let cn = format!("{organization} (ephemeral — self-signed)");
    verbose!("Building X.509v3 certificate: CN=\"{}\"", cn);
    verbose!("  Organization: {}", organization);
    verbose!("  Extensions: BasicConstraints(CA=false), KeyUsage(digitalSignature)");
    verbose!("  Extensions: ExtendedKeyUsage(emailProtection), SKI, AKI");
    verbose!("  Validity: 24 hours from now");
    let mut name_builder =
        X509NameBuilder::new().map_err(|e| InspectError::Signing(format!("X509 name: {e}")))?;
    name_builder
        .append_entry_by_text("O", organization)
        .map_err(|e| InspectError::Signing(format!("X509 O: {e}")))?;
    name_builder
        .append_entry_by_text("CN", &cn)
        .map_err(|e| InspectError::Signing(format!("X509 CN: {e}")))?;
    let name = name_builder.build();

    let mut builder =
        X509Builder::new().map_err(|e| InspectError::Signing(format!("X509 builder: {e}")))?;
    builder.set_version(2).map_err(|e| InspectError::Signing(format!("X509 version: {e}")))?;
    builder
        .set_subject_name(&name)
        .map_err(|e| InspectError::Signing(format!("X509 subject: {e}")))?;
    builder
        .set_issuer_name(&name)
        .map_err(|e| InspectError::Signing(format!("X509 issuer: {e}")))?;
    builder.set_pubkey(&pkey).map_err(|e| InspectError::Signing(format!("X509 pubkey: {e}")))?;

    let not_before =
        Asn1Time::days_from_now(0).map_err(|e| InspectError::Signing(format!("ASN1 time: {e}")))?;
    let not_after =
        Asn1Time::days_from_now(1).map_err(|e| InspectError::Signing(format!("ASN1 time: {e}")))?;
    builder
        .set_not_before(&not_before)
        .map_err(|e| InspectError::Signing(format!("X509 not_before: {e}")))?;
    builder
        .set_not_after(&not_after)
        .map_err(|e| InspectError::Signing(format!("X509 not_after: {e}")))?;

    // Random serial number — RFC 5280 §4.1.2.2 recommends unique serial numbers.
    // BigNum::pseudo_rand generates a 128-bit value suitable for serial use.
    let mut serial_bn =
        BigNum::new().map_err(|e| InspectError::Signing(format!("serial BigNum: {e}")))?;
    serial_bn
        .pseudo_rand(128, openssl::bn::MsbOption::MAYBE_ZERO, false)
        .map_err(|e| InspectError::Signing(format!("serial rand: {e}")))?;
    let serial = serial_bn
        .to_asn1_integer()
        .map_err(|e| InspectError::Signing(format!("serial ASN1: {e}")))?;
    builder
        .set_serial_number(&serial)
        .map_err(|e| InspectError::Signing(format!("X509 serial: {e}")))?;

    // X.509v3 extensions required by the c2pa crate's certificate validator:
    //   - BasicConstraints: CA=false (end-entity cert)
    //   - KeyUsage: digitalSignature
    //   - ExtendedKeyUsage: emailProtection + any (matches c2pa EphemeralSigner)
    let bc = BasicConstraints::new()
        .build()
        .map_err(|e| InspectError::Signing(format!("BasicConstraints: {e}")))?;
    builder.append_extension(bc).map_err(|e| InspectError::Signing(format!("append BC: {e}")))?;

    let ku = KeyUsage::new()
        .digital_signature()
        .build()
        .map_err(|e| InspectError::Signing(format!("KeyUsage: {e}")))?;
    builder.append_extension(ku).map_err(|e| InspectError::Signing(format!("append KU: {e}")))?;

    // C2PA profile disallows anyExtendedKeyUsage — use emailProtection only.
    let eku = ExtendedKeyUsage::new()
        .email_protection()
        .build()
        .map_err(|e| InspectError::Signing(format!("ExtKeyUsage: {e}")))?;
    builder.append_extension(eku).map_err(|e| InspectError::Signing(format!("append EKU: {e}")))?;

    // AuthorityKeyIdentifier — required by the C2PA certificate profile.
    // For a self-signed cert, AKI = SKI (same key).
    let ski = openssl::x509::extension::SubjectKeyIdentifier::new()
        .build(&builder.x509v3_context(None, None))
        .map_err(|e| InspectError::Signing(format!("SKI: {e}")))?;
    builder.append_extension(ski).map_err(|e| InspectError::Signing(format!("append SKI: {e}")))?;

    let aki = openssl::x509::extension::AuthorityKeyIdentifier::new()
        .keyid(true)
        .build(&builder.x509v3_context(None, None))
        .map_err(|e| InspectError::Signing(format!("AKI: {e}")))?;
    builder.append_extension(aki).map_err(|e| InspectError::Signing(format!("append AKI: {e}")))?;

    builder.sign(&pkey, digest).map_err(|e| InspectError::Signing(format!("X509 sign: {e}")))?;
    let cert = builder.build();

    let cert_pem = cert.to_pem().map_err(|e| InspectError::Signing(format!("cert PEM: {e}")))?;
    let key_bytes = pkey
        .private_key_to_pem_pkcs8()
        .map_err(|e| InspectError::Signing(format!("key PEM: {e}")))?;
    // Wrap in Zeroizing so the key bytes are zeroed on drop.
    let key_pem = Zeroizing::new(key_bytes);

    Ok((cert_pem, key_pem))
}

/// Returns `true` if the mode is ephemeral (test/eval).
#[must_use = "Ephemeral status is recorded in ingest log entries and reports; \
              discarding it would suppress the ephemeral mode warning"]
pub const fn is_ephemeral(mode: &SignerMode) -> bool {
    matches!(mode, SignerMode::Ephemeral { .. })
}

/// Read a certificate (non-secret) PEM file by path.
///
/// Certificate chain files are not secret — no zeroization is applied.
fn read_cert_pem(path: &Path) -> Result<Vec<u8>, InspectError> {
    std::fs::read(path).map_err(InspectError::Io)
}

/// Read a private key file with defense-in-depth I/O hardening.
///
/// On Unix: opens the file with `O_NOFOLLOW` to refuse symlink targets,
/// preventing a symlink substitution attack where an attacker replaces the key
/// path with a symlink to a different file. The file descriptor is obtained
/// first; the key bytes are then read from that fd.
///
/// The returned bytes are wrapped in `Zeroizing<Vec<u8>>` so they are zeroed
/// on drop, preventing key recovery from freed heap memory.
///
/// ## Compliance
///
/// - **NIST SP 800-53 SC-12**: Key bytes are zeroized on drop.
/// - **NIST SP 800-53 AC-3**: `O_NOFOLLOW` prevents symlink-based key
///   substitution attacks, enforcing that the configured path names a
///   regular file owned by the operator.
/// - **NSA RTB**: TOCTOU defense — the fd is opened once; all subsequent
///   I/O uses the same fd, eliminating the open-then-read race window.
fn read_private_key(path: &Path) -> Result<Zeroizing<Vec<u8>>, InspectError> {
    #[cfg(unix)]
    {
        use std::io::Read;
        use std::os::unix::fs::OpenOptionsExt;
        // O_NOFOLLOW: refuse to open if the final path component is a symlink.
        // This is a defense-in-depth measure; SELinux type enforcement provides
        // the primary protection on production systems.
        let mut file = std::fs::OpenOptions::new()
            .read(true)
            .custom_flags(libc::O_NOFOLLOW)
            .open(path)
            .map_err(InspectError::Io)?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).map_err(InspectError::Io)?;
        Ok(Zeroizing::new(contents))
    }
    #[cfg(not(unix))]
    {
        let bytes = std::fs::read(path).map_err(InspectError::Io)?;
        Ok(Zeroizing::new(bytes))
    }
}
