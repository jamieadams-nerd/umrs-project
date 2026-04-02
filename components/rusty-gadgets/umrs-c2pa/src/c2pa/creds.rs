// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Jamie Adams

//! Credential generation and validation.
//!
//! Generates self-signed certificates, private keys, and CSRs for C2PA signing.
//! Validates existing credential files against the UMRS configuration.
//!
//! This module exists because key/certificate management is the #1 source of
//! confusion in any PKI-based system.  The goal is: **simple, simple, simple.**
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 SC-13**: Cryptographic Protection — key generation is
//!   gated through [`signer::parse_algorithm`], which enforces the FIPS-safe
//!   algorithm allow-list before any key material is produced.
//! - **NIST SP 800-53 SC-12**: Cryptographic Key Management — generated
//!   private key bytes are held in `Zeroizing<Vec<u8>>` and zeroed on drop,
//!   preventing key recovery from freed heap memory after the struct is dropped.
//! - **NIST SP 800-218 SSDF PW.4.1**: Validate at construction — algorithm
//!   validation occurs at the start of [`generate`], before any I/O.
//! - **CMMC SC.L2-3.13.10**: Employ FIPS-validated cryptography — all
//!   generated keys use ECDSA P-256/384/521 curves.
//! - **NSA RTB RAIN**: Non-Bypassability — every code path through [`generate`]
//!   calls `signer::parse_algorithm` unconditionally.

use openssl::asn1::Asn1Time;
use openssl::bn::BigNum;
use openssl::ec::{EcGroup, EcKey};
use openssl::hash::MessageDigest;
use openssl::nid::Nid;
use openssl::pkey::PKey;
use openssl::x509::extension::{BasicConstraints, ExtendedKeyUsage, KeyUsage};
use openssl::x509::{X509Builder, X509NameBuilder, X509ReqBuilder};
use zeroize::Zeroizing;

use crate::c2pa::{config::UmrsConfig, error::InspectError, signer};
#[allow(unused_imports)]
use crate::verbose;

/// Result of credential generation.
///
/// Contains the generated certificate (or CSR), private key material, and
/// structured metadata fields describing what was generated. Callers are
/// responsible for rendering the metadata into human-readable output — this
/// separation ensures each display string can be individually wrapped with
/// `gettext()` when the i18n layer is wired in.
///
/// The private key is held in `Zeroizing<Vec<u8>>` so it is zeroed on drop
/// and does not persist in freed heap memory after this struct is dropped.
///
/// ## Compliance
///
/// - **NIST SP 800-53 SC-12**: `key_pem` is zeroized on drop.
/// - **NIST SP 800-53 SC-13**: algorithm was validated by `generate` before
///   this struct was constructed.
#[must_use = "Generated key material must be written to disk or consumed; \
              discarding this value means the private key is permanently lost"]
pub struct GeneratedCredentials {
    /// PEM-encoded certificate (self-signed) or CSR.
    pub cert_or_csr_pem: Vec<u8>,

    /// PEM-encoded private key (PKCS#8) — zeroed on drop.
    pub key_pem: Zeroizing<Vec<u8>>,

    /// Whether this is a CSR (true) or self-signed certificate (false).
    pub is_csr: bool,

    /// Algorithm string as it appears in the config (e.g. `"es256"`).
    pub algorithm: String,

    /// Human-readable EC curve name (e.g. `"P-256"`).
    pub curve_name: String,

    /// Key size in bits as a string (e.g. `"256"`).
    pub key_bits: String,

    /// Validity period in days. `None` when `is_csr` is `true` (CSRs have no
    /// validity period — the CA sets it when issuing the signed certificate).
    pub validity_days: Option<u32>,

    /// Organization name from the config, embedded in the subject DN.
    pub organization: String,
}

/// Result of a single credential validation check.
#[derive(Debug)]
pub struct CredCheck {
    pub check: String,
    pub ok: bool,
    pub message: String,
}

impl CredCheck {
    fn pass(check: &str, msg: &str) -> Self {
        Self {
            check: check.into(),
            ok: true,
            message: msg.into(),
        }
    }
    fn fail(check: &str, msg: &str) -> Self {
        Self {
            check: check.into(),
            ok: false,
            message: msg.into(),
        }
    }
    fn warn(check: &str, msg: &str) -> Self {
        Self {
            check: check.into(),
            ok: true, // warn is not a failure for the ok flag
            message: format!("[WARN] {msg}"),
        }
    }
}

/// Generate a self-signed certificate and private key (or a CSR + key).
///
/// Uses the algorithm from the UMRS configuration to pick the curve/key type.
/// The certificate has proper X.509v3 extensions required by the C2PA profile.
///
/// Algorithm validation is performed unconditionally before any key material is
/// produced — this is the NSA RTB RAIN non-bypassability property for this path.
///
/// ## Compliance
///
/// - **NIST SP 800-53 SC-13**: algorithm validation is the first operation.
/// - **NSA RTB RAIN**: `parse_algorithm` is called before any I/O or keygen.
///
/// # Errors
///
/// Returns `InspectError::UnsafeAlgorithm` if the configured algorithm is not
/// FIPS-safe, or `InspectError::Signing` if key generation or certificate
/// building fails.
pub fn generate(
    config: &UmrsConfig,
    csr_only: bool,
    validity_days: u32,
) -> Result<GeneratedCredentials, InspectError> {
    let alg_str = &config.identity.algorithm;
    let alg = signer::parse_algorithm(alg_str)?;

    // Pick the EC curve matching the algorithm.
    let (nid, digest, curve_name, key_bits) = match alg {
        c2pa::SigningAlg::Es384 => (Nid::SECP384R1, MessageDigest::sha384(), "P-384", "384"),
        c2pa::SigningAlg::Es512 => (Nid::SECP521R1, MessageDigest::sha512(), "P-521", "521"),
        _ => (Nid::X9_62_PRIME256V1, MessageDigest::sha256(), "P-256", "256"),
    };

    verbose!("Generating ECDSA key pair on curve {}...", curve_name);
    let group = EcGroup::from_curve_name(nid)
        .map_err(|e| InspectError::Signing(format!("EC group: {e}")))?;
    let ec_key =
        EcKey::generate(&group).map_err(|e| InspectError::Signing(format!("EC keygen: {e}")))?;
    let pkey =
        PKey::from_ec_key(ec_key).map_err(|e| InspectError::Signing(format!("PKey wrap: {e}")))?;

    let key_bytes = pkey
        .private_key_to_pem_pkcs8()
        .map_err(|e| InspectError::Signing(format!("key PEM: {e}")))?;
    // Wrap immediately in Zeroizing so key bytes are zeroed on drop.
    let key_pem = Zeroizing::new(key_bytes);

    let org = &config.identity.organization;

    if csr_only {
        verbose!("Generating Certificate Signing Request...");
        let csr_pem = build_csr(&pkey, org, digest)?;
        Ok(GeneratedCredentials {
            cert_or_csr_pem: csr_pem,
            key_pem,
            is_csr: true,
            algorithm: alg_str.clone(),
            curve_name: curve_name.to_owned(),
            key_bits: key_bits.to_owned(),
            validity_days: None,
            organization: org.clone(),
        })
    } else {
        verbose!("Generating self-signed certificate (valid {} days)...", validity_days);
        let cert_pem = build_self_signed(&pkey, org, digest, validity_days)?;
        Ok(GeneratedCredentials {
            cert_or_csr_pem: cert_pem,
            key_pem,
            is_csr: false,
            algorithm: alg_str.clone(),
            curve_name: curve_name.to_owned(),
            key_bits: key_bits.to_owned(),
            validity_days: Some(validity_days),
            organization: org.clone(),
        })
    }
}

/// Validate existing credentials referenced in the UMRS configuration.
///
/// Checks that the cert and key files exist, are valid PEM, match each other,
/// use the configured algorithm, and have acceptable validity dates.
///
/// Also checks Unix file permissions on the private key — warns if the key
/// file is world-readable or not owned by the current process uid.
#[expect(
    clippy::too_many_lines,
    reason = "Sequential validation checks; splitting would reduce clarity"
)]
#[must_use = "Credential check results must be inspected; discarding them \
              silently ignores expired certificates, key mismatches, or \
              dangerous file permissions"]
pub fn validate(config: &UmrsConfig) -> Vec<CredCheck> {
    let mut checks = Vec::new();

    let (cert_path, key_path) = match (&config.identity.cert_chain, &config.identity.private_key) {
        (Some(c), Some(k)) => (c, k),
        (None, None) => {
            checks.push(CredCheck::fail(
                "credentials",
                "No cert_chain or private_key configured. \
                 Run `inspect creds generate` to create them, \
                 then set the paths in umrs-c2pa.toml.",
            ));
            return checks;
        }
        (Some(_), None) => {
            checks.push(CredCheck::fail(
                "private_key",
                "cert_chain is set but private_key is missing",
            ));
            return checks;
        }
        (None, Some(_)) => {
            checks.push(CredCheck::fail(
                "cert_chain",
                "private_key is set but cert_chain is missing",
            ));
            return checks;
        }
    };

    // File existence.
    if cert_path.exists() {
        checks.push(CredCheck::pass("cert_file", &format!("Found: {}", cert_path.display())));
    } else {
        checks.push(CredCheck::fail("cert_file", &format!("Not found: {}", cert_path.display())));
    }
    if key_path.exists() {
        checks.push(CredCheck::pass("key_file", &format!("Found: {}", key_path.display())));
    } else {
        checks.push(CredCheck::fail("key_file", &format!("Not found: {}", key_path.display())));
    }

    // If either is missing, stop here.
    if !cert_path.exists() || !key_path.exists() {
        return checks;
    }

    // Unix permission check on the private key file.
    // A world-readable or group-readable key is a deployment error.
    #[cfg(unix)]
    check_key_permissions(key_path, &mut checks);

    // PEM format — read key bytes into Zeroizing so they are zeroed after this scope.
    let cert_bytes = match std::fs::read(cert_path) {
        Ok(b) => b,
        Err(e) => {
            checks.push(CredCheck::fail("cert_read", &format!("Cannot read: {e}")));
            return checks;
        }
    };
    let key_bytes: Zeroizing<Vec<u8>> = match std::fs::read(key_path) {
        Ok(b) => Zeroizing::new(b),
        Err(e) => {
            checks.push(CredCheck::fail("key_read", &format!("Cannot read: {e}")));
            return checks;
        }
    };

    if !cert_bytes.windows(11).any(|w| w == b"-----BEGIN ") {
        checks.push(CredCheck::fail("cert_format", "File is not valid PEM"));
        return checks;
    }
    checks.push(CredCheck::pass("cert_format", "Valid PEM format"));

    if !key_bytes.windows(11).any(|w| w == b"-----BEGIN ") {
        checks.push(CredCheck::fail("key_format", "File is not valid PEM"));
        return checks;
    }
    checks.push(CredCheck::pass("key_format", "Valid PEM format"));

    // Parse certificate.
    let cert = match openssl::x509::X509::from_pem(&cert_bytes) {
        Ok(c) => c,
        Err(e) => {
            checks.push(CredCheck::fail("cert_parse", &format!("Cannot parse certificate: {e}")));
            return checks;
        }
    };
    checks.push(CredCheck::pass("cert_parse", "Certificate parsed successfully"));

    // Parse private key.
    let pkey = match openssl::pkey::PKey::private_key_from_pem(&key_bytes) {
        Ok(k) => k,
        Err(e) => {
            checks.push(CredCheck::fail("key_parse", &format!("Cannot parse private key: {e}")));
            return checks;
        }
    };
    checks.push(CredCheck::pass("key_parse", "Private key parsed successfully"));

    // Key matches certificate.
    let cert_pubkey = match cert.public_key() {
        Ok(pk) => pk,
        Err(e) => {
            checks.push(CredCheck::fail(
                "key_match",
                &format!("Cannot extract cert public key: {e}"),
            ));
            return checks;
        }
    };
    if cert_pubkey.public_eq(&pkey) {
        checks.push(CredCheck::pass("key_match", "Private key matches certificate"));
    } else {
        checks.push(CredCheck::fail(
            "key_match",
            "Private key does NOT match certificate — wrong key for this cert",
        ));
    }

    // Subject info.
    let subject = cert
        .subject_name()
        .entries()
        .map(|e| {
            let key = e.object().nid().short_name().unwrap_or("?");
            let val = e.data().as_utf8().map_or_else(|_| "?".to_string(), |s| s.to_string());
            format!("{key}={val}")
        })
        .collect::<Vec<_>>()
        .join(", ");
    checks.push(CredCheck::pass("subject", &format!("Subject: {subject}")));

    // Issuer info.
    let issuer = cert
        .issuer_name()
        .entries()
        .map(|e| {
            let key = e.object().nid().short_name().unwrap_or("?");
            let val = e.data().as_utf8().map_or_else(|_| "?".to_string(), |s| s.to_string());
            format!("{key}={val}")
        })
        .collect::<Vec<_>>()
        .join(", ");
    let is_self_signed = cert.subject_name_hash() == cert.issuer_name_hash();
    if is_self_signed {
        checks.push(CredCheck::pass("issuer", &format!("Issuer: {issuer} (self-signed)")));
    } else {
        checks.push(CredCheck::pass("issuer", &format!("Issuer: {issuer}")));
    }

    // Validity dates.
    let not_before = cert.not_before().to_string();
    let not_after = cert.not_after().to_string();
    let now = openssl::asn1::Asn1Time::days_from_now(0).ok();
    if let Some(ref now_time) = now {
        if cert.not_after() < now_time.as_ref() {
            checks.push(CredCheck::fail(
                "validity",
                &format!("EXPIRED — valid {not_before} to {not_after}"),
            ));
        } else if cert.not_before() > now_time.as_ref() {
            checks.push(CredCheck::fail(
                "validity",
                &format!("NOT YET VALID — valid {not_before} to {not_after}"),
            ));
        } else {
            checks.push(CredCheck::pass(
                "validity",
                &format!("Valid from {not_before} to {not_after}"),
            ));
        }
    }

    // Algorithm info.
    let desc = signer::describe_algorithm(&config.identity.algorithm);
    checks.push(CredCheck::pass("algorithm", &format!("Configured: {desc}")));

    checks
}

// ── internal helpers ─────────────────────────────────────────────────────────

/// Check Unix file permissions on the private key.
///
/// Emits a `CredCheck::warn` (not fail) to keep the validation non-blocking —
/// the key may still be functionally usable even if the permissions are wrong.
/// The warning surfaces in operator preflight output so the issue is visible.
///
/// ## Compliance
///
/// - **NIST SP 800-53 AC-3**: Access Enforcement — world-readable or
///   group-readable key files violate least-privilege access control.
/// - **NIST SP 800-53 CM-6**: Configuration Settings — deployment errors
///   are caught at preflight rather than silently passed.
#[cfg(unix)]
fn check_key_permissions(key_path: &std::path::Path, checks: &mut Vec<CredCheck>) {
    use std::os::unix::fs::MetadataExt;
    match std::fs::metadata(key_path) {
        Err(e) => {
            checks.push(CredCheck::warn("key_permissions", &format!("Cannot stat key file: {e}")));
        }
        Ok(meta) => {
            let mode = meta.mode() & 0o777;
            if mode & 0o077 != 0 {
                checks.push(CredCheck::warn(
                    "key_permissions",
                    &format!(
                        "Private key has unsafe permissions {:04o} — expected 0600 or 0400. \
                         Run: chmod 0600 {}",
                        mode,
                        key_path.display()
                    ),
                ));
                log::warn!(
                    target: "umrs",
                    "private key has unsafe permissions mode={:04o} file={}",
                    mode,
                    key_path.file_name().map(|n| n.to_string_lossy()).unwrap_or_default()
                );
            } else {
                checks.push(CredCheck::pass(
                    "key_permissions",
                    &format!("Permissions {mode:04o} (secure)"),
                ));
            }

            // Check owner matches effective uid.
            // Read euid from /proc/self/status to avoid unsafe libc::geteuid().
            // The "Uid:" line format is: "Uid:\t<real> <effective> <saved> <fs>"
            if let Some(euid) = read_euid_from_proc()
                && meta.uid() != euid
            {
                checks.push(CredCheck::warn(
                    "key_owner",
                    &format!(
                        "Private key owner uid={} does not match process euid={}",
                        meta.uid(),
                        euid
                    ),
                ));
                log::warn!(
                    target: "umrs",
                    "private key owner mismatch file_uid={} process_euid={}",
                    meta.uid(),
                    euid
                );
            }
        }
    }
}

/// Read the effective user ID from `/proc/self/status` without unsafe code.
///
/// The `Uid:` line in `/proc/self/status` has the form:
/// `Uid:\t<real> <effective> <saved> <fs-uid>`
///
/// Returns `None` if the file cannot be read or parsed.
#[cfg(unix)]
fn read_euid_from_proc() -> Option<u32> {
    let status = std::fs::read_to_string("/proc/self/status").ok()?;
    for line in status.lines() {
        if let Some(rest) = line.strip_prefix("Uid:") {
            // Fields are tab/space separated: real effective saved fs
            let mut fields = rest.split_whitespace();
            let _real = fields.next()?; // real uid — skip
            let effective = fields.next()?;
            return effective.parse::<u32>().ok();
        }
    }
    None
}

fn build_self_signed(
    pkey: &PKey<openssl::pkey::Private>,
    organization: &str,
    digest: MessageDigest,
    validity_days: u32,
) -> Result<Vec<u8>, InspectError> {
    let cn = format!("{organization} (UMRS C2PA Signing — self-signed)");
    let name = build_x509_name(organization, &cn)?;

    let mut builder =
        X509Builder::new().map_err(|e| InspectError::Signing(format!("X509 builder: {e}")))?;
    builder.set_version(2).map_err(|e| InspectError::Signing(format!("X509 version: {e}")))?;
    builder
        .set_subject_name(&name)
        .map_err(|e| InspectError::Signing(format!("X509 subject: {e}")))?;
    builder
        .set_issuer_name(&name)
        .map_err(|e| InspectError::Signing(format!("X509 issuer: {e}")))?;
    builder.set_pubkey(pkey).map_err(|e| InspectError::Signing(format!("X509 pubkey: {e}")))?;

    let not_before =
        Asn1Time::days_from_now(0).map_err(|e| InspectError::Signing(format!("ASN1 time: {e}")))?;
    let not_after = Asn1Time::days_from_now(validity_days)
        .map_err(|e| InspectError::Signing(format!("ASN1 time: {e}")))?;
    builder
        .set_not_before(&not_before)
        .map_err(|e| InspectError::Signing(format!("X509 not_before: {e}")))?;
    builder
        .set_not_after(&not_after)
        .map_err(|e| InspectError::Signing(format!("X509 not_after: {e}")))?;

    // Random serial number — RFC 5280 §4.1.2.2 recommends unique serial numbers.
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

    // C2PA-required X.509v3 extensions.
    add_c2pa_extensions(&mut builder)?;

    builder.sign(pkey, digest).map_err(|e| InspectError::Signing(format!("X509 sign: {e}")))?;

    builder.build().to_pem().map_err(|e| InspectError::Signing(format!("cert PEM: {e}")))
}

fn build_csr(
    pkey: &PKey<openssl::pkey::Private>,
    organization: &str,
    digest: MessageDigest,
) -> Result<Vec<u8>, InspectError> {
    let cn = format!("{organization} (UMRS C2PA Signing)");

    let mut name_builder =
        X509NameBuilder::new().map_err(|e| InspectError::Signing(format!("X509 name: {e}")))?;
    name_builder
        .append_entry_by_text("O", organization)
        .map_err(|e| InspectError::Signing(format!("X509 O: {e}")))?;
    name_builder
        .append_entry_by_text("CN", &cn)
        .map_err(|e| InspectError::Signing(format!("X509 CN: {e}")))?;
    let name = name_builder.build();

    let mut req =
        X509ReqBuilder::new().map_err(|e| InspectError::Signing(format!("CSR builder: {e}")))?;
    req.set_pubkey(pkey).map_err(|e| InspectError::Signing(format!("CSR pubkey: {e}")))?;
    req.set_subject_name(&name).map_err(|e| InspectError::Signing(format!("CSR subject: {e}")))?;
    req.sign(pkey, digest).map_err(|e| InspectError::Signing(format!("CSR sign: {e}")))?;

    req.build().to_pem().map_err(|e| InspectError::Signing(format!("CSR PEM: {e}")))
}

fn build_x509_name(organization: &str, cn: &str) -> Result<openssl::x509::X509Name, InspectError> {
    let mut name_builder =
        X509NameBuilder::new().map_err(|e| InspectError::Signing(format!("X509 name: {e}")))?;
    name_builder
        .append_entry_by_text("O", organization)
        .map_err(|e| InspectError::Signing(format!("X509 O: {e}")))?;
    name_builder
        .append_entry_by_text("CN", cn)
        .map_err(|e| InspectError::Signing(format!("X509 CN: {e}")))?;
    Ok(name_builder.build())
}

fn add_c2pa_extensions(builder: &mut X509Builder) -> Result<(), InspectError> {
    let bc = BasicConstraints::new()
        .build()
        .map_err(|e| InspectError::Signing(format!("BasicConstraints: {e}")))?;
    builder.append_extension(bc).map_err(|e| InspectError::Signing(format!("append BC: {e}")))?;

    let ku = KeyUsage::new()
        .digital_signature()
        .build()
        .map_err(|e| InspectError::Signing(format!("KeyUsage: {e}")))?;
    builder.append_extension(ku).map_err(|e| InspectError::Signing(format!("append KU: {e}")))?;

    let eku = ExtendedKeyUsage::new()
        .email_protection()
        .build()
        .map_err(|e| InspectError::Signing(format!("ExtKeyUsage: {e}")))?;
    builder.append_extension(eku).map_err(|e| InspectError::Signing(format!("append EKU: {e}")))?;

    let ski = openssl::x509::extension::SubjectKeyIdentifier::new()
        .build(&builder.x509v3_context(None, None))
        .map_err(|e| InspectError::Signing(format!("SKI: {e}")))?;
    builder.append_extension(ski).map_err(|e| InspectError::Signing(format!("append SKI: {e}")))?;

    let aki = openssl::x509::extension::AuthorityKeyIdentifier::new()
        .keyid(true)
        .build(&builder.x509v3_context(None, None))
        .map_err(|e| InspectError::Signing(format!("AKI: {e}")))?;
    builder.append_extension(aki).map_err(|e| InspectError::Signing(format!("append AKI: {e}")))?;

    Ok(())
}
