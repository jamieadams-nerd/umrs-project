# Rust Developer Agent Memory

## Key Architectural Facts

- Primary workspace: `components/rusty-gadgets/` — production crates only
- Prototype workspace: `components/rust-prototypes/` — moved 2026-03-11; no xtask, use `cargo build/test` directly
- Primary crate: `umrs-selinux` (under `src/`)
- Platform crate: `umrs-platform` (detect pipeline, kattrs, evidence)
- All tests in `tests/`, never inline
- Run everything via `cargo xtask {fmt,clippy,test}` from `components/rusty-gadgets/`

## rust-prototypes workspace members (as of 2026-03-11)

- `cui-labels` — depends on `umrs-core` via `../../rusty-gadgets/umrs-core`
- `kernel-files` — no workspace deps
- `mcs-setrans` — depends on `umrs-selinux` via `../../rusty-gadgets/umrs-selinux`
- `vaultmgr` — depends on `umrs-core` via `../../rusty-gadgets/umrs-core`
- These were moved from rusty-gadgets; rustfmt.toml is an exact copy of rusty-gadgets/rustfmt.toml

## Toolchain

- Rust 1.92+ is the active toolchain on this system
- `is_multiple_of()` is stable in 1.92 — clippy enforces it over `% 2 == 0`
- `let ... && let ...` (let-chain guards) are stable in this toolchain

## System Prerequisites

- `sqlite-devel` must be installed on the build host (`sudo dnf install sqlite-devel`)
- Without it, `umrs-ls` and anything linking `umrs-platform` will fail with `-lsqlite3 not found`
- This is the RHEL package; Ubuntu equivalent is `libsqlite3-dev` (but target is RHEL)

## FFI Exception — rusqlite

`rusqlite` uses FFI (via `libsqlite3-sys`) to bind to C `libsqlite3`. This is a documented,
accepted exception to the "prefer pure Rust / avoid FFI" policy. Accepted by Jamie Adams
2026-03-11. Documented in `umrs-platform/Cargo.toml` and `rpm_db.rs` module doc.
Rationale: no pure-Rust SQLite handles RHEL 10 RPM DB reliably; system lib is RHEL-patched;
unsafe is fully encapsulated in rusqlite/libsqlite3-sys.

## umrs-platform detect/ Pipeline

Phase order: release_candidate → release_parse → pkg_substrate → file_ownership → integrity_check

Key types:
- `EvidenceBundle` — append-only; `records` field is PRIVATE (RPM-22 fix). Access via `.records()`, `.iter()`, `.push()`, `.len()`, `.is_empty()`
- `EvidenceRecord` — one per I/O event; `path_requested` = original path, `path_resolved` = symlink target if any
- `find_stat_for_path` — pub(super) in file_ownership.rs; searches bundle in reverse for (dev,ino) by path
- `find_resolved_path` — pub(super) in file_ownership.rs; searches bundle for symlink target by path_requested

## Symlink Path Resolution (RPM-16)

On RHEL 10, `/etc/os-release` is a symlink → `../usr/lib/os-release` (relative!).
The RPM DB records `/usr/lib/os-release` (absolute). Fix pattern:
1. `find_resolved_path(evidence, candidate_str)` returns the raw readlinkat result
2. If relative: join with candidate's parent dir, then `std::fs::canonicalize()`
3. Use canonicalized path for DB queries; keep original `candidate_str` in audit records
Both `file_ownership.rs` and `integrity_check.rs` import and apply this pattern.

## Dev Encoding Mismatch — FIXED

`release_candidate.rs` records `dev_combined = (stx_dev_major << 32) | stx_dev_minor`.
`statx` exposes major/minor as separate u32 fields; this encodes them as `(major as u64) << 32 | minor`.

`nix::sys::stat::stat()` and `rustix::fs::fstat()` return `st_dev` as the kernel's compact
`dev_t` encoding (e.g., device 253:0 → 64768, not `(253 << 32) | 0`).

Fix applied in two places:
- `rpm.rs::query_ownership_inner`: decompose with `nix::sys::stat::major/minor` (return `u64`),
  reassemble as `(st_major << 32) | st_minor`.
- `integrity_check.rs` fstat block: decompose with `rustix::fs::major/minor` (return `u32`),
  reassemble as `(u64::from(maj) << 32) | u64::from(min)`.

After fix: `os_detect` example shows `owner=centos-stream-release` and T4 digest verified.

## mcs_translator Test Fixture Path

Test fixture is at `umrs-selinux/data/setrans.conf` (moved from crate root in a prior session).
`tests/mcs_translator.rs` and `examples/color_demo.rs` both use `"data/setrans.conf"`.
All 5 mcs_translator tests pass as of 2026-03-11.

## EvidenceBundle IntoIterator

`IntoIterator for &EviduxBundle` is implemented (required by clippy `iter_without_into_iter`).
`for rec in bundle.iter()` and `for rec in &bundle` both work.

## Error Display Discipline (RPM-02)

rusqlite::Error Display can leak paths. Use:
`e.sqlite_error_code().map_or(-1_i32, |c| c as i32)` to extract only the error code.

## RPM-07: fail-closed on ArrayLengthMismatch

`RpmHeaderError::ArrayLengthMismatch { expected, dirindexes, digests }` added.
`extract_file_list` now returns this error instead of silently returning `Vec::new()`.

## SEC Pattern — Sealed Evidence Cache (implemented 2026-03-11)

- Module: `umrs-platform/src/sealed_cache.rs`
- Tests: `umrs-platform/tests/sealed_cache_tests.rs` (16 tests, all passing)
- Example: `umrs-platform/examples/sealed_cache_demo.rs`
- Public re-exports from `lib.rs`: `SealedCache`, `CacheStatus`, `DEFAULT_TTL_SECS`, `MAX_TTL_SECS`
- New deps: `hmac = "0.12"`, `zeroize = { version = "1", features = ["derive"] }`
- FIPS gate: read at construction via ProcfsText; caching disabled if FIPS active
- Seal covers: TrustLevel byte + SHA-256(EvidenceBundle) + boot_id + os_id + version_id + probe_used
- Key derivation: SHA-256(boot_id || 0x00 || starttime_ticks_le) — two entropy sources
  - boot_id: /proc/sys/kernel/random/boot_id (session binding)
  - starttime: /proc/self/stat field 22 (process invocation binding)
  - Both read via ProcfsText + SecureReader; fail-closed if either is unavailable (no fallback)
- FIPS gate: fail-CLOSED — returns true (disable caching) on ANY read failure, not just "1"
- Conservative cache hit: re-runs pipeline on verified hit (full deserialization deferred)
- `OsDetector::detect()` is NOT integrated — caller uses `SealedCache::query()` instead
- map_or pattern: `.map_or(b"" as &[u8], |x| ...)` is required when default is a byte literal

## Outstanding Audit Findings

Reports in `.claude/reports/` with findings assigned to "coder":
- `2026-03-11-rpm-db-security-audit.md`: coder-assigned RPM-01, -02, -04, -06, -07, -11, -12, -15, -16, -19, -20, -22, -27, -28
- `2026-03-11-os-detection-umrs-platform-surface-audit.md`: multiple HIGH/MEDIUM items
- `sec-audit-2026-03-11.md`: SEC-01, SEC-02 (HIGH) resolved. SEC-04, SEC-06, SEC-08 (LOW) resolved.
  Remaining coder items: SEC-05 (evidence digest coverage), SEC-07 (tamper test), SEC-09 (Cargo.toml citation), SEC-10 (lib.rs citations).

## Clippy Style Preferences

- Use explicit `match` over `.map().unwrap_or_else()` — clearer and avoids lint
- Use `let x && let y` chains for if-let guards
- `is_multiple_of(2)` is preferred over `% 2 == 0` by clippy 1.92
- `match` with one Some/Ok arm and one None/Err arm fires `single_match_else` — use `if let ... else` or `let Some(...) = ... else { return ... }` (let-else)
- `let-else` form: `let Some(x) = expr else { return None; };` — preferred for early-return error paths
