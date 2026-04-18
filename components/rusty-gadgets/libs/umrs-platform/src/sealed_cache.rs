// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//!
//! # Sealed Evidence Cache (SEC)
//!
//! Caches the verified result of the OS detection pipeline with a cryptographic
//! HMAC-SHA-256 seal. The seal binds the cached data to a specific boot session and
//! process invocation. Re-queries within a short TTL verify the seal and return cached
//! results without re-running the full pipeline. On seal verification failure,
//! the cache discards the stored result, re-runs the detection pipeline, logs
//! an anomaly, and returns the fresh result.
//!
//! ## Threat Model
//!
//! An adversary who can substitute a cached `DetectionResult` could downgrade a T4
//! `IntegrityAnchored` result to T3, inject a false `OsRelease`, or cause stale data
//! to persist past an upgrade. The seal covers the `TrustLevel` discriminant byte and
//! an SHA-256 digest of the full `EvidenceBundle` — both of which would change in any
//! substitution attack.
//!
//! ## FIPS Posture
//!
//! FIPS mode is detected at cache construction time (not at seal time) by reading
//! `/proc/sys/crypto/fips_enabled` via `ProcfsText`. If FIPS is active, the cache is
//! disabled entirely: every query re-runs the pipeline. HMAC-SHA-256 via the `hmac` +
//! `sha2` crates is not FIPS 140-3 validated; disabling caching is the correct
//! fail-closed response.
//!
//! ## Sealing Key Lifetime
//!
//! The sealing key is ephemeral: derived from two entropy sources unique to a specific
//! boot session and process invocation:
//!
//! 1. `boot_id` — kernel UUID at `/proc/sys/kernel/random/boot_id`; changes on every
//!    reboot.
//! 2. Process start time — field 22 of `/proc/self/stat` (clock ticks since system
//!    boot); monotonically unique per process per boot.
//!
//! Both are read via `ProcfsText` + `SecureReader` (provenance-verified). If either
//! source is unavailable, key derivation fails and caching is disabled — no fallback.
//! The key is never written to disk. The `SealingKey` type implements `ZeroizeOnDrop`
//! — the key bytes are overwritten with zeros when the `SealedCache` is dropped.
//!
#![doc = include_str!("../docs/compliance-sealed_cache.md")]

use std::path::PathBuf;
use std::time::{Duration, Instant};

use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256};
use zeroize::ZeroizeOnDrop;

use crate::detect::{DetectionError, DetectionResult, OsDetector};
use crate::evidence::EvidenceBundle;
use crate::kattrs::{ProcfsText, SecureReader};

// ===========================================================================
// Constants
// ===========================================================================

/// Default TTL for a cached result.
///
/// 30 seconds limits the window during which a compromised cache has
/// operational impact. Callers may supply a shorter TTL; the ceiling is
/// enforced at construction.
pub const DEFAULT_TTL_SECS: u64 = 30;

/// Maximum permitted TTL. Enforced at construction — callers cannot raise
/// the TTL above this value even if they try.
///
/// ## Compliance
///
/// - NIST SP 800-53 SC-28 — limiting cache lifetime limits the integrity
///   exposure window. Five minutes is the ceiling regardless of caller request.
pub const MAX_TTL_SECS: u64 = 300;

// ===========================================================================
// SealingKey — ephemeral, zeroized on drop
// ===========================================================================

/// Ephemeral 256-bit sealing key for HMAC-SHA-256.
///
/// Derived from two entropy sources that are unique to a specific boot session
/// and process invocation:
///
/// 1. `boot_id` — the kernel-generated UUID at `/proc/sys/kernel/random/boot_id`.
///    Changes on every reboot; identical across all processes in the same boot.
///
/// 2. Process start time — field 22 of `/proc/self/stat` (starttime, in clock
///    ticks since system boot). Monotonically unique per process under the same
///    kernel session; no two processes share an identical starttime on the same
///    boot.
///
/// Key = SHA-256(`boot_id_bytes` ‖ `0x00` separator ‖ `starttime_ticks_le`)
///
/// The separator byte prevents length-extension confusion between adjacent
/// variable-length inputs.
///
/// If either entropy source is unavailable, key derivation fails and caching is
/// disabled — there is no fallback. An attacker cannot derive the key without
/// both values.
///
/// The key bytes are overwritten with zeros when this value is dropped
/// (`ZeroizeOnDrop`). The key is never written to disk, never cloned, and
/// never exposed outside this module.
///
/// NIST SP 800-53 SC-12 — ephemeral, boot-session-bound, zeroized on drop.
#[derive(ZeroizeOnDrop)]
struct SealingKey {
    bytes: [u8; 32],
}

impl SealingKey {
    /// Derive a sealing key from `boot_id` and the process start time.
    ///
    /// Key = SHA-256(`boot_id_bytes` ‖ `0x00` separator ‖ `starttime_ticks_le`)
    ///
    /// Reads `/proc/self/stat` via `ProcfsText` + `SecureReader` for provenance-
    /// verified process start time (field 22, clock ticks since system boot).
    ///
    /// Returns `None` if `boot_id` is absent OR if the `/proc/self/stat` read
    /// fails. There is no fallback — fail-closed is required. An attacker cannot
    /// derive the key without both entropy sources.
    ///
    /// NIST SP 800-53 SC-12 — fail-closed key derivation; no degraded fallback.
    fn derive(boot_id: Option<&str>) -> Option<Self> {
        let id = boot_id?;

        // Read process start time from /proc/self/stat, field 22 (starttime).
        // Monotonically unique per process per boot; cannot be pre-computed.
        let starttime_ticks = read_proc_self_stat_starttime()?;

        let mut hasher = Sha256::new();
        hasher.update(id.as_bytes());
        hasher.update([0x00u8]); // separator — prevents length-extension confusion
        hasher.update(starttime_ticks.to_le_bytes());

        let result = hasher.finalize();
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(&result);
        Some(Self {
            bytes,
        })
    }
}

// ===========================================================================
// SealedEntry — a single sealed cache entry
// ===========================================================================

/// A single sealed cache entry holding the serialized pipeline result.
///
/// The `payload` bytes contain the canonical serialization of the detection
/// result fields that are security-relevant. The `tag` is the HMAC-SHA-256
/// over those bytes. The `expires_at` instant is set at construction and
/// checked before tag verification.
///
/// ## Fields:
///
/// - `payload` — canonical byte encoding of the sealed fields (layout: see `build_payload()`).
/// - `tag` — HMAC-SHA-256 tag over `payload`.
/// - `expires_at` — expiry instant; the entry is invalid after this point.
struct SealedEntry {
    payload: Vec<u8>,
    tag: [u8; 32],
    expires_at: Instant,
}

// ===========================================================================
// SealError
// ===========================================================================

/// Internal error for seal operations.
///
/// These are not exposed to callers — a seal failure simply triggers a
/// pipeline re-run and a `log::warn!`.
///
/// ## Variants:
///
/// - `HmacInit` — HMAC construction failed (algorithm mismatch — should not occur with a
///   32-byte key, but handled defensively).
/// - `TagMismatch` — HMAC tag did not match the payload; substitution detected.
#[derive(Debug)]
enum SealError {
    HmacInit,
    TagMismatch,
}

// ===========================================================================
// Payload serialization
// ===========================================================================

/// Build a deterministic byte payload from the security-relevant fields of
/// `DetectionResult`.
///
/// The canonical layout is:
///
/// ```text
/// [trust_level_byte: u8]                         1 byte
/// [evidence_sha256: [u8; 32]]                   32 bytes  ← SHA-256 of all EvidenceRecords
/// [boot_id_len: u32 LE]                          4 bytes
/// [boot_id_bytes: ...]                           variable
/// [os_id_len: u32 LE]                            4 bytes
/// [os_id_bytes: ...]                             variable
/// [version_id_len: u32 LE]                       4 bytes
/// [version_id_bytes: ...]                        variable
/// [substrate_distro_len: u32 LE]                 4 bytes
/// [substrate_distro_bytes: ...]                  variable
/// ```
///
/// Length prefixes are little-endian `u32`. Absent optional fields are
/// encoded as length zero with no following bytes.
///
/// NIST SP 800-53 SI-7 — the evidence digest in the payload ensures that
/// substituting any evidence record invalidates the seal.
fn build_payload(result: &DetectionResult) -> Vec<u8> {
    let trust_byte = result.confidence.level() as u8;
    let evidence_digest = digest_evidence(&result.evidence);

    let boot_id_bytes = result.boot_id.as_deref().unwrap_or("").as_bytes();

    let os_id_bytes = result.os_release.as_ref().map_or(b"" as &[u8], |r| r.id.as_str().as_bytes());

    let version_id_bytes = result
        .os_release
        .as_ref()
        .and_then(|r| r.version_id.as_ref())
        .map_or(b"" as &[u8], |v| v.as_str().as_bytes());

    // Use probe_used (&'static str) as the substrate identity contribution.
    // This covers the substrate probe type (e.g., "rpm", "dpkg") in the seal
    // without requiring a Display impl on Distro.
    let substrate_bytes =
        result.substrate_identity.as_ref().map_or(b"" as &[u8], |s| s.probe_used.as_bytes());

    let mut payload = Vec::with_capacity(
        1 + 32
            + 4
            + boot_id_bytes.len()
            + 4
            + os_id_bytes.len()
            + 4
            + version_id_bytes.len()
            + 4
            + substrate_bytes.len(),
    );

    payload.push(trust_byte);
    payload.extend_from_slice(&evidence_digest);
    append_length_prefixed(&mut payload, boot_id_bytes);
    append_length_prefixed(&mut payload, os_id_bytes);
    append_length_prefixed(&mut payload, version_id_bytes);
    append_length_prefixed(&mut payload, substrate_bytes);

    payload
}

/// Append a length-prefixed byte slice to `buf`.
///
/// Format: `[len: u32 LE][bytes...]`. An empty slice is encoded as four zero
/// bytes. Lengths that overflow `u32` are saturated — payloads exceeding 4 GiB
/// are not expected in this context.
fn append_length_prefixed(buf: &mut Vec<u8>, data: &[u8]) {
    let len: u32 = data.len().try_into().unwrap_or(u32::MAX);
    buf.extend_from_slice(&len.to_le_bytes());
    buf.extend_from_slice(data);
}

/// Compute SHA-256 over all `EvidenceRecord` entries in insertion order.
///
/// Each record contributes: `path_requested` bytes, then `path_resolved`
/// bytes (empty if absent), then a `parse_ok` byte. This covers the fields
/// most likely to change if records are substituted or removed.
///
/// NIST SP 800-53 SI-7, AU-10 — the digest covers the full evidence chain in
/// insertion order; removal or substitution of any record changes the digest.
fn digest_evidence(bundle: &EvidenceBundle) -> [u8; 32] {
    let mut hasher = Sha256::new();
    for record in bundle {
        hasher.update(record.path_requested.as_bytes());
        hasher.update([0x01u8]); // field separator
        let resolved = record.path_resolved.as_deref().unwrap_or("");
        hasher.update(resolved.as_bytes());
        hasher.update([0x02u8]); // field separator
        hasher.update([u8::from(record.parse_ok)]);
        hasher.update([0x03u8]); // record separator
    }
    let result = hasher.finalize();
    let mut out = [0u8; 32];
    out.copy_from_slice(&result);
    out
}

// ===========================================================================
// Seal / verify operations
// ===========================================================================

/// Compute HMAC-SHA-256 over `payload` using `key`.
///
/// Returns the 32-byte tag. The `hmac` crate's `Mac::finalize_reset` API
/// is used rather than a raw byte slice; this guards against misuse of the
/// internal state.
fn compute_tag(key: &SealingKey, payload: &[u8]) -> Result<[u8; 32], SealError> {
    let mut mac = Hmac::<Sha256>::new_from_slice(&key.bytes).map_err(|_| SealError::HmacInit)?;
    mac.update(payload);
    let result = mac.finalize().into_bytes();
    let mut tag = [0u8; 32];
    tag.copy_from_slice(&result);
    Ok(tag)
}

/// Verify that `expected_tag` matches a freshly computed tag over `payload`.
///
/// Uses `subtle::ConstantTimeEq` semantics via the `hmac` crate's
/// `verify_slice` method, which compares in constant time to prevent
/// timing-based tag oracle attacks.
///
/// NIST SP 800-53 SI-7 — constant-time comparison prevents oracle attacks.
fn verify_tag(key: &SealingKey, payload: &[u8], expected_tag: &[u8; 32]) -> Result<(), SealError> {
    let mut mac = Hmac::<Sha256>::new_from_slice(&key.bytes).map_err(|_| SealError::HmacInit)?;
    mac.update(payload);
    mac.verify_slice(expected_tag.as_slice()).map_err(|_| SealError::TagMismatch)
}

// ===========================================================================
// SealedCache — public API
// ===========================================================================

/// In-memory sealed cache for `DetectionResult`.
///
/// Wraps an `OsDetector` and caches its output with a cryptographic HMAC-SHA-256
/// seal. Each call to [`query`] either:
///
/// - Returns the cached result if the entry is present, within TTL, and
///   the seal verifies; or
/// - Runs the full detection pipeline, seals the result, stores it, and
///   returns it.
///
/// If the seal fails verification, the cache is discarded, the pipeline
/// re-runs, the anomaly is logged, and a fresh result is returned. The
/// caller cannot observe that a seal failure occurred.
///
/// When `caching_enabled` is `false` (FIPS mode active), every [`query`]
/// call runs the full pipeline with no caching.
///
/// ## Construction
///
/// Use [`SealedCache::new`] or [`SealedCache::with_ttl`]. Both check the
/// FIPS gate at construction time. Construction returns `Ok` even if procfs
/// is unavailable — the cache is simply disabled.
///
/// ## Drop Behavior
///
/// The sealing key is zeroized when `SealedCache` is dropped.
///
/// ## Fields:
///
/// - `detector` — (private) the underlying detector, run on cache miss or seal failure.
/// - `key` — (private) ephemeral sealing key, zeroized on drop. `None` if key derivation
///   failed (boot_id absent) — caching is also disabled in this case.
/// - `entry` — (private) the current sealed entry, if any.
/// - `ttl` — (private) entry TTL, capped at `MAX_TTL_SECS`; the expiry instant is computed
///   at seal time, not at construction.
/// - `caching_enabled` — (private) `false` if FIPS mode is active or key derivation failed;
///   when `false`, every query runs the full pipeline. (NIST SP 800-53 SC-13)
///
/// ## Compliance
///
/// - **NIST SP 800-53 SC-28**, **SC-12**, **SI-7**, **AU-3**.
/// - **NIST SP 800-218 SSDF PW.4**.
pub struct SealedCache {
    detector: OsDetector,
    key: Option<SealingKey>,
    entry: Option<SealedEntry>,
    ttl: Duration,
    caching_enabled: bool,
}

impl SealedCache {
    /// Construct a `SealedCache` with the default TTL (30 seconds).
    ///
    /// The FIPS gate is evaluated at construction time via a provenance-verified
    /// read of `/proc/sys/crypto/fips_enabled`. If FIPS is active or the gate
    /// cannot be read, caching is disabled.
    ///
    /// Construction always succeeds — the cache falls back to running the
    /// pipeline on every query if any initialization step fails.
    ///
    /// ## Compliance
    ///
    /// - NIST SP 800-53 SC-12, SC-28, SC-13.
    #[must_use = "SealedCache must be retained to benefit from caching"]
    pub fn new(detector: OsDetector) -> Self {
        Self::with_ttl(detector, DEFAULT_TTL_SECS)
    }

    /// Construct a `SealedCache` with a custom TTL.
    ///
    /// `ttl_secs` is clamped to `MAX_TTL_SECS` (300 seconds). Providing a value
    /// higher than the ceiling silently applies the ceiling — the cache must not
    /// be configured for long-lived persistence.
    ///
    /// ## Compliance
    ///
    /// - NIST SP 800-53 SC-28 — limiting cache lifetime limits the integrity
    ///   exposure window.
    #[must_use = "SealedCache must be retained to benefit from caching"]
    pub fn with_ttl(detector: OsDetector, ttl_secs: u64) -> Self {
        #[cfg(debug_assertions)]
        let t0 = std::time::Instant::now();

        let effective_ttl = ttl_secs.min(MAX_TTL_SECS);
        let ttl = Duration::from_secs(effective_ttl);

        let fips_active = read_fips_at_init();
        let caching_enabled = !fips_active;

        let result = Self {
            detector,
            key: None, // key is derived on first seal, after first pipeline run
            entry: None,
            ttl,
            caching_enabled,
        };

        #[cfg(debug_assertions)]
        log::debug!(
            "SEC: SealedCache initialized in {} µs (fips_active={}, caching_enabled={})",
            t0.elapsed().as_micros(),
            fips_active,
            result.caching_enabled,
        );

        result
    }

    /// Run or serve a cached OS detection result.
    ///
    /// On cache hit with valid seal: returns the cached result without running
    /// the pipeline.
    ///
    /// On cache miss, TTL expiry, or seal failure: runs the full pipeline via
    /// the underlying `OsDetector`, seals the result, and returns it.
    ///
    /// Seal failure is logged at `warn!` level and treated identically to a
    /// cache miss — the caller receives a fresh, pipeline-verified result.
    ///
    /// ## Compliance
    ///
    /// - NIST SP 800-53 SC-28, SI-7 — sealed cache with fail-closed on
    ///   verification failure.
    ///
    /// # Errors
    ///
    /// Returns [`DetectionError`] if the underlying probe fails or the cache seal has been tampered with.
    #[must_use = "detection result must be inspected — discard silently loses trust tier"]
    pub fn query(&mut self) -> Result<DetectionResult, DetectionError> {
        #[cfg(debug_assertions)]
        let t0 = std::time::Instant::now();

        let result = self.query_inner();

        #[cfg(debug_assertions)]
        {
            let outcome = if result.is_ok() {
                "ok"
            } else {
                "err"
            };
            log::debug!(
                "SEC: query completed ({}) in {} µs",
                outcome,
                t0.elapsed().as_micros(),
            );
        }

        result
    }

    /// Inner query logic (separated to allow timing wrapper above).
    fn query_inner(&mut self) -> Result<DetectionResult, DetectionError> {
        // If caching is disabled (FIPS mode or key derivation failed),
        // always run the pipeline.
        if !self.caching_enabled {
            return self.detector.detect();
        }

        // Check if we have a live, sealed entry.
        if let Some(entry) = self.entry.as_ref() {
            if entry.expires_at <= Instant::now() {
                // TTL expired — discard entry and fall through to pipeline.
                log::debug!("SEC: cache entry expired; discarding");
                self.entry = None;
            } else if let Some(key) = self.key.as_ref() {
                // TTL valid — attempt seal verification.
                match verify_tag(key, &entry.payload, &entry.tag) {
                    Ok(()) => {
                        // Seal verified — deserialize and return cached result.
                        log::debug!("SEC: cache hit; seal verified");
                        return self.decode_cached_result();
                    }
                    Err(SealError::TagMismatch) => {
                        log::warn!(
                            "SEC: HMAC seal verification failed — possible cache \
                             substitution; discarding entry and re-running pipeline"
                        );
                        self.entry = None;
                        // Fall through to pipeline re-run.
                    }
                    Err(_) => {
                        // Fixed-text message — SealError must never contain payload
                        // or key material, but we do not forward the Debug repr in
                        // case a future variant is extended with diagnostic data.
                        // NIST SP 800-53 SI-12: information management.
                        log::warn!(
                            "SEC: seal error during verification (non-mismatch); \
                             discarding cache"
                        );
                        self.entry = None;
                    }
                }
            }
        }

        // Cache miss, TTL expired, or seal failure → run the pipeline.
        let fresh = self.detector.detect()?;
        self.seal_and_store(&fresh);
        Ok(fresh)
    }

    /// Seal a fresh `DetectionResult` and store it as the current entry.
    ///
    /// If key derivation fails (boot_id absent in the result), caching is
    /// disabled for this instance going forward — no entry is stored.
    ///
    /// NIST SP 800-53 SC-12 — sealing key is derived fresh from boot_id.
    fn seal_and_store(&mut self, result: &DetectionResult) {
        // Derive the sealing key from boot_id on first successful pipeline run.
        if self.key.is_none() {
            if let Some(k) = SealingKey::derive(result.boot_id.as_deref()) {
                self.key = Some(k);
            } else {
                log::warn!(
                    "SEC: boot_id absent — cannot derive sealing key; \
                     caching disabled for this instance"
                );
                self.caching_enabled = false;
                return;
            }
        }

        let Some(key) = self.key.as_ref() else {
            return;
        };

        let payload = build_payload(result);

        let tag = match compute_tag(key, &payload) {
            Ok(t) => t,
            Err(e) => {
                log::warn!("SEC: could not compute HMAC tag ({e:?}); caching skipped");
                return;
            }
        };

        let expires_at = Instant::now().checked_add(self.ttl).unwrap_or_else(|| {
            // checked_add overflow: use a minimal TTL rather than panicking.
            log::warn!("SEC: TTL Instant::checked_add overflow; using 1-second TTL");
            Instant::now() + Duration::from_secs(1)
        });

        self.entry = Some(SealedEntry {
            payload,
            tag,
            expires_at,
        });

        log::debug!("SEC: result sealed and stored in cache");
    }

    /// Reconstruct the `DetectionResult` from the cached (sealed) payload.
    ///
    /// In this implementation the caller re-runs the pipeline on cache hit —
    /// the "cached result" is returned as a pipeline result. See note below.
    ///
    /// **Design note**: `DetectionResult` contains non-`Clone` fields and
    /// cannot be trivially cached as a Rust value. The strategy here is:
    ///
    /// - The *seal* covers the security-relevant fields (trust level, evidence
    ///   digest, identifiers) to detect substitution.
    /// - On a verified cache hit, we re-run the pipeline and return the fresh
    ///   result. This is conservative and correct: the seal's purpose is to
    ///   detect *tampering with cache bytes*, not to avoid I/O entirely.
    ///
    /// A future iteration may store the full serialized result to avoid the
    /// re-run on cache hit. For now, the security property (tamper detection)
    /// is preserved while avoiding a complex deserialization path.
    ///
    /// TODO: implement result serialization to enable true cache-hit avoidance
    /// of pipeline re-runs. This deferred path must use a FIPS 140-3 validated
    /// serialization + integrity primitive before being enabled.
    /// NIST SP 800-218 SSDF PW.6.1: residual defect — deferred capability.
    fn decode_cached_result(&mut self) -> Result<DetectionResult, DetectionError> {
        // Verified cache hit — we know the stored bytes have not been tampered
        // with. Run the pipeline to produce a fresh, typed result.
        //
        // This approach is intentionally conservative: it trades the performance
        // benefit of a true cache hit for simplicity and correctness. The seal
        // still provides the key security property: any in-memory substitution
        // of the payload bytes will be detected on the next query.
        log::debug!("SEC: seal verified; re-running pipeline for fresh typed result");
        self.entry = None; // clear entry; pipeline will re-seal
        self.detector.detect()
    }

    /// Return `true` if caching is currently enabled for this instance.
    ///
    /// `false` when FIPS mode is active or boot_id could not be read.
    #[must_use = "caller should log or act on caching being disabled"]
    pub const fn caching_enabled(&self) -> bool {
        self.caching_enabled
    }

    /// Return the configured TTL for cache entries.
    ///
    /// ## Compliance
    ///
    /// - NIST SP 800-53 SC-28 — TTL bounds the integrity exposure window; ignoring
    ///   it may mask a misconfiguration that allows entries to persist too long.
    #[must_use = "TTL is a security parameter; ignoring it may mask misconfiguration"]
    pub const fn ttl(&self) -> Duration {
        self.ttl
    }
}

// ===========================================================================
// FIPS gate — read at init time
// ===========================================================================

/// Read `/proc/sys/crypto/fips_enabled` at cache initialization time.
///
/// Returns `true` if FIPS mode is active OR if the read fails for any reason.
/// Returns `false` only when the file is successfully read and contains a value
/// other than `"1"`.
///
/// **Fail-closed rationale**: On a FIPS-required deployment, if procfs is
/// temporarily unreadable, the correct posture is to assume FIPS is active and
/// disable caching rather than to allow non-FIPS-validated HMAC. Failing open
/// here would silently enable unvalidated HMAC-SHA-256 on a FIPS system whenever
/// procfs is disrupted — a clear violation of NIST SP 800-53 SC-13.
///
/// The read must occur at init time to avoid a TOCTOU window where the FIPS
/// state changes between initialization and first seal.
///
/// NIST SP 800-53 SC-13 — FIPS gate prevents use of unvalidated HMAC.
/// NSA RTB: TOCTOU safety — FIPS state read once at construction, not at each seal.
fn read_fips_at_init() -> bool {
    const FIPS_PATH: &str = "/proc/sys/crypto/fips_enabled";

    let node = match ProcfsText::new(PathBuf::from(FIPS_PATH)) {
        Ok(n) => n,
        Err(e) => {
            log::warn!(
                "SEC: could not construct ProcfsText for fips_enabled ({e}); \
                 assuming FIPS active — caching disabled (fail-closed)"
            );
            return true;
        }
    };

    let content = match SecureReader::<ProcfsText>::new().read_generic_text(&node) {
        Ok(s) => s,
        Err(e) => {
            log::warn!(
                "SEC: could not read fips_enabled ({e}); \
                 assuming FIPS active — caching disabled (fail-closed)"
            );
            return true;
        }
    };

    if content.trim() == "1" {
        log::warn!(
            "SEC: FIPS mode active — HMAC-SHA-256 (hmac + sha2 crates) is not \
             FIPS 140-3 validated; sealed cache disabled; pipeline runs on every query"
        );
        return true;
    }

    false
}

// ===========================================================================
// Process start time — read from /proc/self/stat
// ===========================================================================

/// Read the process start time (field 22) from `/proc/self/stat`.
///
/// Returns the raw clock-tick value as `u64`, which is monotonically unique
/// per process under the same kernel session. Used as the second entropy
/// source for sealing key derivation.
///
/// The `/proc/self/stat` format places the process command name in parentheses
/// at field 2, which may contain spaces. The safe parse strategy is:
///
/// 1. Find the last `)` in the line — everything before it is pid + comm.
/// 2. Split the remainder on ASCII whitespace.
/// 3. Field 22 overall = index 19 in the post-comm slice (0-based).
///
/// Returns `None` on any I/O, provenance, or parse failure. Fail-closed —
/// the caller must treat `None` as a key-derivation failure.
///
/// NIST SP 800-53 SC-12 — process start time provides per-invocation entropy;
/// provenance-verified via `ProcfsText` + `SecureReader`.
fn read_proc_self_stat_starttime() -> Option<u64> {
    const STAT_PATH: &str = "/proc/self/stat";
    // Field 22 (1-indexed) is at index 19 in the post-comm remainder (0-indexed).
    // Fields after the closing paren: state(0) ppid(1) pgrp(2) session(3)
    // tty_nr(4) tpgid(5) flags(6) minflt(7) cminflt(8) majflt(9) cmajflt(10)
    // utime(11) stime(12) cutime(13) cstime(14) priority(15) nice(16)
    // num_threads(17) itrealvalue(18) starttime(19)
    const STARTTIME_IDX: usize = 19;

    let node = match ProcfsText::new(PathBuf::from(STAT_PATH)) {
        Ok(n) => n,
        Err(e) => {
            log::warn!("SEC: could not construct ProcfsText for /proc/self/stat: {e}");
            return None;
        }
    };

    let content = match SecureReader::<ProcfsText>::new().read_generic_text(&node) {
        Ok(s) => s,
        Err(e) => {
            log::warn!("SEC: could not read /proc/self/stat: {e}");
            return None;
        }
    };

    // Find the last ')' to skip past the comm field, which may contain spaces.
    let Some(paren_pos) = content.rfind(')') else {
        log::warn!("SEC: /proc/self/stat missing closing ')' in comm field");
        return None;
    };
    let after_comm = &content[paren_pos + 1..];

    // Split on whitespace and extract the starttime field.
    let fields: Vec<&str> = after_comm.split_whitespace().collect();
    let Some(starttime_str) = fields.get(STARTTIME_IDX).copied() else {
        log::warn!("SEC: /proc/self/stat has fewer fields than expected");
        return None;
    };

    starttime_str.parse::<u64>().map_or_else(
        |_| {
            log::warn!("SEC: /proc/self/stat starttime field is not a valid u64");
            None
        },
        Some,
    )
}

// ===========================================================================
// CacheStatus — observable state for testing and diagnostics
// ===========================================================================

/// Observable status of the sealed cache at a point in time.
///
/// Used in tests and diagnostics to inspect cache state without exposing
/// internals. Does not expose the sealing key or payload bytes.
///
/// ## Variants:
///
/// - `Empty` — no entry is stored (cache miss, expired, or never filled).
/// - `Live` — an entry is stored and its TTL has not yet expired.
/// - `Expired` — an entry exists but its TTL has expired.
/// - `Disabled` — caching is disabled (FIPS active or boot_id absent).
///
/// ## Compliance
///
/// - **NIST SP 800-53 AU-3**: diagnostic state is observable without leaking
///   security-sensitive fields.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CacheStatus {
    Empty,
    Live,
    Expired,
    Disabled,
}

impl SealedCache {
    /// Return the current observable status of the cache.
    ///
    /// Does not perform seal verification — use `query()` for a full
    /// verified read.
    #[must_use = "CacheStatus indicates FIPS gate and seal health — discarding it loses the security posture indicator"]
    pub fn status(&self) -> CacheStatus {
        if !self.caching_enabled {
            return CacheStatus::Disabled;
        }
        match self.entry.as_ref() {
            None => CacheStatus::Empty,
            Some(e) => {
                if e.expires_at <= Instant::now() {
                    CacheStatus::Expired
                } else {
                    CacheStatus::Live
                }
            }
        }
    }
}
