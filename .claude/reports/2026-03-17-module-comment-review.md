# Module-Level Comment Review — All Crates

**Date:** 2026-03-17
**Reviewer:** Von Neumann (tech-writer) x4 parallel agents
**Task:** source-code-comment-cleanup Task 3

## Summary

| Crate | HIGH | MEDIUM | LOW | Total |
|-------|------|--------|-----|-------|
| umrs-selinux | 12 | 17 | 6 | 35 |
| umrs-platform | 1 | 3 | 6 | 10 |
| umrs-core | 2 | ~10 | ~18 | ~30 |
| umrs-ls | 1 | 0 | 0 | 1 |
| umrs-tui | 2 | 1 | 5 | 8 |
| **Total** | **18** | **~31** | **~35** | **~84** |

## Code Bugs (not just doc issues)

1. **umrs-platform** `sealed_cache.rs` + `detect/integrity_check.rs` — wrong FIPS path
   `/proc/sys/kernel/fips_enabled` should be `/proc/sys/crypto/fips_enabled`
   FAIL-OPEN on FIPS detection on real RHEL 10 systems.

2. **umrs-core** `fs/mod.rs` — references `HashSet` (not imported) and `ENCRYPTED_FS_TYPES` (not defined)
   File will not compile if reached.

3. **umrs-core** `audit/events.rs` — exists but `audit/mod.rs` does not declare `pub mod events`
   Module is unreachable/dead code.

## HIGH Doc Findings (by crate)

### umrs-selinux (12 HIGH)
- F-004: TPI parse architecture not documented in `context.rs`
- F-005: Duplicate `MlsLevel` in `context.rs:31` and `mls/level.rs:45` — not acknowledged
- F-008: Typo "Catagories" in `category.rs:5` (renders in rustdoc title)
- F-013: `status.rs` single-line module doc for Trust Gate module
- F-018: `mls/range.rs` — no `//!` block at all
- F-020: `mcs/translator.rs:6` — "this crate" should be "this module"
- F-021: `mcs/translator.rs:11` — `NIST 800-53` missing `SP`
- F-022: `mcs/translator.rs:12` — `NSA Raise-the-Bar` should be `NSA RTB`
- F-027: `xattrs.rs` — no `//!` block; TPI guarantee not in rustdoc
- F-028: `xattrs.rs:6` — `NIST 800-53` missing `SP` in `//` comment
- F-029: `posix/mod.rs` — no `//!` block

### umrs-platform (1 HIGH)
- F-01: Wrong FIPS procfs path in `sealed_cache.rs` + `detect/integrity_check.rs` (CODE BUG)

### umrs-core (2 HIGH)
- F-17: `fs/mod.rs` references undefined symbols
- F-07: `audit/events.rs` unreachable (missing `pub mod events`)

### umrs-ls (1 HIGH)
- LS-1: `main.rs` has no `//!` block — tool invisible to `cargo doc`

### umrs-tui (2 HIGH)
- TUI-1: `lib.rs` usage steps 3-4 point to wrong function (`read_security_indicators` vs `build_header_context`)
- TUI-2: `main.rs` `//!` lists 2 tabs but binary has 3; Kernel Security tab undocumented

## MEDIUM Highlights

### umrs-selinux
- Many modules missing compliance citations entirely (category, sensitivity, mls/*, mcs/*, utils/*)
- `context.rs` `dominates()` is `todo!()` but doc implies enforcement is active
- `posix/identity.rs:1` — stale filename `posix.rs` in heading
- `mcs/colors.rs` — range-matching string equality limitation underplayed

### umrs-platform
- `kattrs/procfs.rs` — doesn't mention `ProcfsText`
- `posture/mod.rs` Quick Start calls `snap.findings()` without defining findings
- `lib.rs` module table omits `FipsCrossCheck`, `ModprobeConfig`, `ContradictionKind`

### umrs-core
- `console/macros.rs` — primary console API with no module doc
- `cui/mod.rs` and `cui/catalog.rs` — CUI-handling code with zero documentation
- `selinux/mod.rs` — empty `//!` block on security-critical module
- Zero compliance control citations in any `//!` block across the entire crate

### umrs-tui
- TUI-3: `keymap.rs` cites `NIST SP 800-53 AC-2` for session termination — should be `AC-12`

## LOW Highlights
- Grammar: "a indicator" → "an indicator" (platform, 2 locations)
- Missing heading conventions (5 platform files)
- `dirlist.rs` "Fail-open" phrasing misleading to security readers
- Various inline citation formatting inconsistencies
- Copyright name spelling inconsistency in category.rs

## Remediation Notes

- Code bugs (items 1-3 above) must be fixed first — they affect correctness
- NIST citation normalization (`NIST 800-53` → `NIST SP 800-53`) was partially done in a prior pass;
  ~30 item-level occurrences remain in umrs-selinux `///` comments (out of scope for this review
  but should be caught in the same fix pass)
- `secure_dirent.rs` and `posix/primitives.rs` are exemplary — use as templates
