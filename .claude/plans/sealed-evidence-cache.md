# Plan: Sealed Evidence Cache (SEC) for OS Detection

---
status: draft
date: 2026-03-11
crate: umrs-platform
area: detect
phases: 3
pattern: SEC (NIST 800-53 SC-28, SC-12)
---

## Context

The OS detection pipeline (`OsDetector::detect()`) performs 14+ I/O operations across 7
phases to reach T4 (`IntegrityAnchored`). On a stable system, the result does not change
between consecutive calls within the same boot session. Callers who need the detection
result multiple times (e.g., `umrs-ls` listing hundreds of files, each checking OS
identity) currently re-run the entire pipeline every time.

The SEC pattern seals the T4 result with an HMAC tied to the boot session, caches it with
a configurable TTL, and returns the cached result on re-query after verifying the seal.
This avoids redundant I/O while maintaining the trust guarantee.

## Threat Model

| Threat | Mitigation |
|--------|------------|
| Attacker modifies cached `OsRelease` in memory | HMAC seal detects tampering |
| Attacker replays a cached result from a different boot | `boot_id` binding in the key |
| Attacker delays cache expiry to hide a change | TTL is enforced by the system clock |
| Cache key leaked to disk | Key is ephemeral, zeroized on drop, never serialized |
| FIPS mode: HMAC uses non-validated primitive | Disable caching when FIPS is active (or use system HMAC) |

## Phase 1: Sealing Key and Seal Type

### 1.1 — New dependency decision: `hmac` + `sha2`

`sha2` is already a dependency. Add `hmac 0.12` (RustCrypto family, pure Rust, minimal
transitive deps). Both crates are from the same RustCrypto ecosystem as the existing
`sha2 0.10`.

```toml
hmac = { version = "0.12", optional = true }

[features]
default = ["rpm-db", "detect-cache"]
detect-cache = ["dep:hmac"]
```

Feature-gated so builds without caching are possible.

### 1.2 — `src/detect/cache/seal_key.rs` (NEW)

```rust
/// Ephemeral sealing key derived from boot_id + process start time.
///
/// The key is generated once per `CachedDetector` instance and zeroized
/// on drop. It never leaves the process and is never serialized.
///
/// NIST SP 800-53 SC-12 — cryptographic key management.
pub(super) struct SealKey {
    /// 32-byte HMAC-SHA256 key material.
    /// Implements ZeroizeOnDrop.
    key: zeroize::Zeroizing<[u8; 32]>,
}
```

**Key derivation** (not KDF — just mixing entropy sources):
```
key = SHA-256(boot_id || process_start_nanos || pid)
```

- `boot_id`: from `DetectionResult::boot_id` (already read via provenance-verified procfs)
- `process_start_nanos`: `std::time::SystemTime::now()` at construction time
- `pid`: `std::process::id()`

This is NOT a cryptographic KDF — it's mixing locally-available entropy to produce a
per-instance key. The security property is that two different processes (or the same
process after restart) will produce different keys, preventing cross-process seal replay.

**Patterns applied**:
- Zeroize on drop (SSDF PW 4.1, NIST 800-53 MP-4)
- `Debug` impl prints `[REDACTED]` (secrecy discipline)

### 1.3 — `src/detect/cache/seal.rs` (NEW)

```rust
/// A sealed detection result.
///
/// Contains the `DetectionResult` plus an HMAC tag that covers:
/// - The `TrustLevel` discriminant
/// - The `LabelTrust` variant (discriminant only, not contradiction text)
/// - A SHA-256 digest of the serialized `OsRelease` fields
/// - The evidence record count
/// - The `boot_id`
/// - The creation timestamp
///
/// NIST SP 800-53 SC-28 — protection of information at rest (in memory).
pub(super) struct SealedResult {
    result: DetectionResult,
    tag: [u8; 32],
    created_at: std::time::Instant,
    ttl: std::time::Duration,
}
```

**Methods**:
- `seal(result, key, ttl) -> Self` — compute HMAC, store result + tag + timestamp
- `verify_and_get(key) -> Option<&DetectionResult>` — check TTL, verify HMAC, return ref
- `is_expired() -> bool` — `created_at.elapsed() > ttl`

**HMAC input construction**: Deterministic byte sequence built from the result's key
fields. Does NOT serialize the entire `DetectionResult` — only the security-relevant
fields that an attacker would want to modify. This keeps the seal computation fast.

**Constant-time comparison**: HMAC tag verified using `subtle::ConstantTimeEq` (already
a pattern in CLAUDE.md). Prevents timing side-channels on seal verification.

## Phase 2: Cached Detector

### 2.1 — `src/detect/cache/mod.rs` (NEW)

```rust
/// A caching wrapper around `OsDetector` that seals and caches results.
///
/// On first call to `detect()`, runs the full pipeline and seals the result.
/// On subsequent calls within the TTL, verifies the seal and returns the
/// cached result without re-running the pipeline.
///
/// NIST SP 800-53 SC-28, SC-12. Pattern: SEC.
pub struct CachedDetector {
    inner: OsDetector,
    key: SealKey,
    cache: Mutex<Option<SealedResult>>,
    ttl: Duration,
}
```

**Methods**:
- `new(detector: OsDetector, ttl: Duration) -> Self` — derives sealing key
- `detect() -> Result<&DetectionResult, DetectionError>` — check cache, verify seal,
  return cached or re-run pipeline
- `invalidate()` — explicitly clear the cache (e.g., after a package update)
- `cache_status() -> CacheStatus` — `Hit`, `Miss`, `Expired`, `SealFailed`

**Thread safety**: `Mutex<Option<SealedResult>>` — same pattern as `RpmProbe::db`.
Never contended in practice (sequential pipeline calls).

**FIPS gate**: If `ProcFips` reads `1`, caching is disabled (HMAC would need
FIPS-validated implementation). `detect()` always runs the full pipeline.
Log a debug message: "SEC: caching disabled — FIPS mode active".

### 2.2 — Re-export

```rust
// In src/detect/mod.rs:
#[cfg(feature = "detect-cache")]
pub mod cache;
#[cfg(feature = "detect-cache")]
pub use cache::CachedDetector;
```

## Phase 3: Tests and Example

### 3.1 — `tests/detect_cache_tests.rs` (NEW)

| Test | Purpose |
|------|---------|
| `seal_and_verify_roundtrip` | Seal a mock result, verify succeeds |
| `seal_verify_wrong_key` | Verify with different key → None |
| `seal_expired` | Create with 0ms TTL, verify → None (expired) |
| `seal_tampered_trust_level` | Modify trust level after sealing → verify fails |
| `cached_detector_first_call` | First detect() runs pipeline |
| `cached_detector_second_call` | Second detect() returns cached |
| `cached_detector_after_invalidate` | invalidate() forces re-run |
| `fips_mode_disables_cache` | When FIPS=1, every call runs pipeline |

### 3.2 — Update `examples/os_detect.rs`

Add a section demonstrating cached detection:

```rust
// --- Cached Detection Demo ---
let cached = CachedDetector::new(OsDetector::default(), Duration::from_secs(30));
let r1 = cached.detect()?;  // full pipeline
let r2 = cached.detect()?;  // cache hit
println!("Cache status: {:?}", cached.cache_status());
```

## Dependencies Summary

| Crate | Version | Feature | Purpose | Supply chain |
|-------|---------|---------|---------|-------------|
| `hmac` | 0.12 | `detect-cache` | HMAC-SHA256 seal | RustCrypto, pure Rust, minimal deps |
| `subtle` | 2.5 | `detect-cache` | Constant-time tag comparison | RustCrypto, widely audited |
| `zeroize` | 1.7 | `detect-cache` | Key zeroization on drop | RustCrypto, mandatory for key material |

All three are from the RustCrypto family and have zero `unsafe` in their public API.

## Files Summary

| File | Action | Phase |
|------|--------|-------|
| `Cargo.toml` | Add `hmac`, `subtle`, `zeroize` (optional) | 1 |
| `src/detect/cache/mod.rs` | NEW — `CachedDetector` | 2 |
| `src/detect/cache/seal_key.rs` | NEW — ephemeral key derivation | 1 |
| `src/detect/cache/seal.rs` | NEW — seal/verify logic | 1 |
| `src/detect/mod.rs` | Add `cache` module, re-export | 2 |
| `tests/detect_cache_tests.rs` | NEW — seal + cache tests | 3 |
| `examples/os_detect.rs` | Add cached detection demo section | 3 |

All paths relative to `components/rusty-gadgets/umrs-platform/`.

## Verification

```bash
cd components/rusty-gadgets

# Full verification
cargo xtask fmt
cargo xtask clippy
cargo xtask test
cargo test -p umrs-platform -- cache

# Without cache feature
cargo clippy -p umrs-platform --no-default-features --features rpm-db -- -D warnings

# Example
cargo run -p umrs-platform --example os_detect
```

## Open Questions

1. **TTL default**: 30 seconds? 60 seconds? Should it be configurable per-caller or
   fixed at construction?
2. **FIPS fallback**: Disable caching entirely, or provide an option to use the system's
   FIPS-validated HMAC via `kcapi` (kernel crypto API)? Using `kcapi` would add an FFI
   dependency.
3. **Evidence chain in seal**: Include full evidence digest, or just the record count?
   Full digest is more thorough but slower to compute on large bundles.
