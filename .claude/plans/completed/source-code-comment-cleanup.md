# Plan: Source Code Comment Cleanup

**Date:** 2026-03-15
**Status:** COMPLETE (2026-03-17)
**ROADMAP Goals:** G7 (Public Project), G8 (High-Assurance Patterns)
**Owner:** rust-developer (Rusty)
**Reviewer:** security-auditor (The IRS), tech-writer (Von Neumann)

---

## Purpose

Improve API documentation quality across all crates by removing clippy lint
suppressions and fixing the resulting violations. This is prerequisite work for
M4 (Public Release) — source code comments must be reviewed before crates.io
publication.

---

## Task 1 — Remove `missing_errors_doc` suppression

Remove `#![allow(clippy::missing_errors_doc)]` and add proper `# Errors` sections
to all public `Result`-returning functions.

| Crate | Violations fixed | Status |
|---|---|---|
| umrs-platform | 41 | DONE (2026-03-17) |
| umrs-selinux | 21 + 2 panics | DONE (2026-03-17) |
| umrs-hw | 0 (suppression removed) | DONE (2026-03-17) |
| umrs-core | 0 (no suppression) | N/A |
| umrs-tui | suppression remains | Deferred — TUI has its own enhancement plan |
| umrs-ls | 0 (no suppression) | N/A |

**Approach:** One crate at a time. Remove suppression, fix all violations, clippy clean,
separate commit per crate.

## Task 2 — Remove `missing_panics_doc` suppression

Remove `#![allow(clippy::missing_panics_doc)]` where present and add `# Panics` sections.
Currently suppressed in umrs-tui. Check other crates.

## Task 3 — Module-level comment review (Von Neumann)

Von Neumann (tech-writer) reviews every module-level `//!` comment block across all crates for:
- **Clarity** — is the module's purpose immediately understandable?
- **Accuracy** — do the comments match what the code actually does?
- **Cross-references** — are related modules, types, and patterns properly linked?
- **Control citations** — are NIST/CMMC/RTB references correct and complete?
- **Terminology** — consistent with OSCAL, SP 800-53A, and project glossary?

Von Neumann produces a findings report per crate. Rusty fixes all findings.
The IRS reviews the final result for compliance annotation completeness.

| Crate | Review status | Findings | Fixed |
|---|---|---|---|
| umrs-selinux | DONE | 35 (12H, 17M, 6L) | All HIGH+MEDIUM fixed (2026-03-17) |
| umrs-platform | DONE | 10 (1H, 3M, 6L) | All fixed incl FIPS path bug (2026-03-17) |
| umrs-core | DONE | ~30 (2H, ~10M, ~18L) | Code bugs + MEDIUM doc fixes (2026-03-17) |
| umrs-tui | DONE | 8 (2H, 1M, 5L) | All HIGH+MEDIUM fixed (2026-03-17) |
| umrs-ls | DONE | 1 (1H) | Fixed (2026-03-17) |

**Reports:** `.claude/reports/2026-03-17-module-comment-review.md`

**Code bugs found during review:**
- FIPS path corrected: `/proc/sys/kernel/fips_enabled` → `/proc/sys/crypto/fips_enabled` (umrs-platform)
- `fs/mod.rs` undefined symbols fixed, module left unwired (raw procfs reads need migration)
- `audit/events.rs` wired into `audit/mod.rs`
- `keymap.rs` wrong control: `AC-2` → `AC-12` (Session Termination)

## Task 4 — (reserved for future Jamie tasks)

Jamie will add additional source code comment cleanup tasks here.

---

## Constraints

- Each crate is a separate pass — don't batch across crates
- `# Errors` sections should describe the actual error conditions, not just "returns Err"
- `# Panics` sections only needed where panics are reachable (not for unreachable panics)
- Von Neumann reviews the doc text for clarity after Rusty writes it
- The IRS reviews for completeness — are all error paths documented?
