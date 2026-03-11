# Rust Developer Agent Memory

## Key Architectural Facts

- Workspace root: `components/rusty-gadgets/`
- Primary crate: `umrs-selinux` (under `src/`)
- Platform crate: `umrs-platform` (detect pipeline, kattrs, evidence)
- All tests in `tests/`, never inline
- Run everything via `cargo xtask {fmt,clippy,test}` from `components/rusty-gadgets/`

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

## Outstanding Audit Findings (next session — start here)

Two reports in `.claude/reports/` with findings assigned to "coder":
- `components/rusty-gadgets/.claude/reports/2026-03-11-rpm-db-security-audit.md`
  29 findings total; coder-assigned: RPM-01, RPM-02, RPM-04, RPM-06, RPM-07, RPM-11,
  RPM-12, RPM-15, RPM-16, RPM-19, RPM-20, RPM-22, RPM-27, RPM-28
- `.claude/reports/2026-03-11-os-detection-umrs-platform-surface-audit.md`
  22 findings total; multiple coder-assigned HIGH/MEDIUM items

Session start checklist will surface these automatically. Work through HIGH findings first.

## Clippy Style Preferences

- Use explicit `match` over `.map().unwrap_or_else()` — clearer and avoids lint
- Use `let x && let y` chains for if-let guards
- `is_multiple_of(2)` is preferred over `% 2 == 0` by clippy 1.92
