# Rust Developer Agent Memory

## Key Files
- See `project_structure.md` for workspace layout and module map.
- See `patterns.md` for confirmed high-assurance pattern application notes.

## User Preferences
- Jamie Adams (Jamie). Concise communication.
- End of session: brief summary, flag anything unusual, "your turn to review, commit, and push."

## Active Work
- Phase 2a of Kernel Security Posture Probe: REVIEWED AND FIXED (2026-03-14)
  - Plan: `.claude/plans/kernel-security-posture-probe.md`
  - 180 tests pass (40 modprobe, 60 posture, 26 kattrs, 14 fips, 16 rpm_header, 7 rpm_db, 16 sealed_cache)
  - Security-engineer F1/F2, auditor F-06/F-07/F-08/F-09 + others, tech-writer sweep all resolved.
  - Doc-sync task #1 still pending for tech-writer.
  - Phase 2b is next.

## Known Patterns — Blacklist Sentinel
- `evaluate_configured_meets("blacklisted", desired)` now handles the sentinel.
- Blacklist signals use `DesiredValue::Exact(1)` — "blacklisted" maps to integer 1.
- Without this fix, `BootDrift` was silently suppressed for all four DMA-surface blacklist signals.

## Confirmed Patterns (umrs-platform posture module)
- All `/proc/` reads: `ProcfsText` + `SecureReader::read_generic_text`
- All `/sys/` reads: `SysfsText` + `SecureReader::read_generic_text` with SYSFS_MAGIC
- Merge-tree pattern: load dirs in precedence order, last-writer-wins, lexicographic sort within dir
- Debug timing: `#[cfg(debug_assertions)]` block with `std::time::Instant`, log µs at completion
- Non-debug suppression: `#[cfg(not(debug_assertions))] let _ = var;`
- SignalId `Ok(None)` arm in `read_live_sysctl`: must include ALL non-sysctl variants explicitly
- `ParsedDirective` and `parse_modprobe_line` are `pub` (not pub(crate)) — integration tests need visibility

## Clippy Suppressions Active (lib.rs)
- `option_if_let_else`, `redundant_closure`, `module_name_repetitions`
- `missing_errors_doc`, `missing_panics_doc`, `unreadable_literal`, `doc_markdown`
- Never add suppressions without asking Jamie.
