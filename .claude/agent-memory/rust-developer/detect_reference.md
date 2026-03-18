# Detect Pipeline Implementation Reference

Moved from MEMORY.md 2026-03-18. Detailed detection pipeline implementation notes.

## Dev Encoding Mismatch — FIXED

`release_candidate.rs` uses `(stx_dev_major << 32) | stx_dev_minor`.
`nix`/`rustix` fstat returns compact `dev_t`. Fix: decompose with major/minor, reassemble.
Applied in `rpm.rs::query_ownership_inner` and `integrity_check.rs` fstat block.

## Error Display Discipline (RPM-02)

rusqlite::Error Display can leak paths. Use:
`e.sqlite_error_code().map_or(-1_i32, |c| c as i32)` for error code only.

## RPM-07: fail-closed on ArrayLengthMismatch

`RpmHeaderError::ArrayLengthMismatch` added. `extract_file_list` returns error instead
of silently returning `Vec::new()`.

## SEC Pattern — Sealed Evidence Cache

- Module: `umrs-platform/src/sealed_cache.rs`
- Tests: 16 passing. Example: `sealed_cache_demo.rs`
- Public: `SealedCache`, `CacheStatus`, `DEFAULT_TTL_SECS`, `MAX_TTL_SECS`
- Deps: `hmac = "0.12"`, `zeroize = { version = "1", features = ["derive"] }`
- FIPS gate: fail-CLOSED on ANY read failure
- Key derivation: SHA-256(boot_id || 0x00 || starttime_ticks_le)
- Conservative: re-runs pipeline on verified hit (full deser deferred)
- map_or pattern: `.map_or(b"" as &[u8], |x| ...)` for byte literal default

## EvidenceBundle IntoIterator

`IntoIterator for &EvidenceBundle` — `for rec in &bundle` works.

## mcs_translator Test Fixture

Fixture: `umrs-selinux/data/setrans.conf`. All 5 tests pass.
