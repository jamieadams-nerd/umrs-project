# Rust Developer Agent Memory

## Topic Files

- [Posture module history](posture_module_history.md) — Signal→Indicator rename, Phase 2b architecture, security review findings
- [SCAP/STIG familiarization](scap_familiarization.md) — CCE mappings, new indicator candidates
- [Doc placement feedback](feedback_doc_placement.md) — `# Errors` section placement rules
- [TUI reference](tui_reference.md) — umrs-tui architecture, binaries, tests, i18n, patterns
- [Detect pipeline reference](detect_reference.md) — dev encoding, RPM fixes, SEC pattern, EvidenceBundle
- [TUI phases](tui_phase1.md) / [phase2](tui_phase2.md) / [phase3](tui_phase3.md) / [phase45](tui_phase45.md)
- [Timestamp module](timestamp_module.md) — BootSessionTimestamp/Duration, CLOCK_MONOTONIC_RAW

## Key Architectural Facts

- Primary workspace: `components/rusty-gadgets/` — production crates only
- Prototype workspace: `components/rust-prototypes/` — out of scope; no xtask
- Primary crate: `umrs-selinux`; Platform crate: `umrs-platform`
- HW crate: `umrs-hw` — ONLY crate WITHOUT `#![forbid(unsafe_code)]`
- All tests in `tests/`, never inline
- Run via `cargo xtask {fmt,clippy,test}` from `components/rusty-gadgets/`

## Toolchain

- Rust 1.92+; `is_multiple_of()` stable; let-chain guards stable

## System Prerequisites

- `sqlite-devel` required (`sudo dnf install sqlite-devel`)
- Without it, `umrs-ls`/`umrs-platform` fail with `-lsqlite3 not found`

## FFI Exception — rusqlite

Accepted exception (Jamie 2026-03-11). System `libsqlite3` on RHEL; no bundled feature.
Documented in `umrs-platform/Cargo.toml` and `rpm_db.rs`.

## detect/ Pipeline

Phase order: release_candidate → release_parse → pkg_substrate → file_ownership → integrity_check

Key types:
- `EvidenceBundle` — append-only; `records` PRIVATE. Access: `.records()`, `.iter()`, `.push()`
- `EvidenceRecord` — `path_requested` = original, `path_resolved` = symlink target

## Symlink Path Resolution (RPM-16)

RHEL 10 `/etc/os-release` → `../usr/lib/os-release` (relative symlink).
RPM DB has absolute path. Fix: `find_resolved_path` → canonicalize → use for DB queries.

## umrs-selinux Import Paths

- `observations` module is **private** — import via root: `umrs_selinux::ObservationKind`
- `SelinuxCtxState` re-exported at root
- `fs_encrypt` and `posix` are `pub mod`
- `SecurityContext`: `.user()`, `.role()`, `.security_type()`, `.level()` → `Option<&MlsLevel>`
- `FileSize` is `Copy` — pass by value. Has `.as_u64()`.
- `SecureDirent::access_denied` is a `pub bool` field

## Clippy Style Preferences

- Explicit `match` over `.map().unwrap_or_else()`
- `let x && let y` chains for if-let guards
- `is_multiple_of(2)` over `% 2 == 0`
- `let-else` for early-return: `let Some(x) = expr else { return None; };`
- `missing_const_for_fn`: make helpers const, never suppress
- `unnecessary_literal_bound`: use `&'static str` for literal returns
- `cast_possible_truncation`: `#[allow]` with range-safety comment

## Outstanding Audit Findings

Reports in `.claude/reports/` with coder-assigned items:
- `2026-03-11-rpm-db-security-audit.md`: RPM-01, -02, -04, -06, -07, -11, -12, -15, -16, -19, -20, -22, -27, -28
- `2026-03-11-os-detection-umrs-platform-surface-audit.md`: multiple HIGH/MEDIUM
- `sec-audit-2026-03-11.md`: SEC-05, -07, -09, -10 remaining

## posture Module

37 signals, `PostureSnapshot::collect()`, contradiction detection, FipsCrossCheck.
Tests: posture_tests.rs (79+), posture_modprobe_tests.rs (50), posture_bootcmdline_tests.rs (10).
See [posture_module_history.md](posture_module_history.md) for full details.

## Error Information Discipline Pattern

Canonical: `posture/configured.rs`. Debug logs: key + line number, suppress raw values in release.
`snapshot.rs collect_one()`: dual `cfg(debug_assertions)` blocks.
