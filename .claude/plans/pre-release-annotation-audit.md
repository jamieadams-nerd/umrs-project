# Pre-Release Annotation Audit Scope

**Date:** 2026-03-21
**Status:** draft — Herb to author full scope document
**Author:** Herb (The IRS)
**ROADMAP Goals:** G7 (Public Project)
**Milestones:** M4 (Public Release) — must be complete before any public release
**Tech Lead:** Herb (security-auditor)
**LOE:** Medium (~2-3 sessions for full audit pass across all crates)

---

## Purpose

Define which crates get annotation review, in what order, and what "clean" means
before M4 public release. Every NIST citation in UMRS code is a factual claim that
security professionals will evaluate. Wrong, vague, or unsupported citations damage
credibility worse than having none.

---

## Scope

### Crates to audit (priority order)

1. `umrs-selinux` — highest public visibility, most compliance annotations
2. `umrs-platform` — kernel posture, trust tiers, detection pipeline
3. `umrs-core` — shared types and utilities
4. `umrs-assess` — assessment engine (when implemented)
5. `umrs-mcs` — CUI labeling (when implemented)
6. `umrs-ls` — tool binary, user-facing output
7. `umrs-state` — tool binary
8. `umrs-logspace` — tool binary

### What constitutes "clean"

- Every module has a `//!` block with `## Compliance` section
- All citations use canonical form (`NIST SP 800-53`, not `NIST 800-53`)
- Security-critical types and functions have explicit control citations
- No internal reference document citations (Finding 1, RAG Finding, etc.)
- CCE identifiers include STIG version and date
- No orphaned or incorrect control references
- `#[must_use]` annotations include message strings (not bare)

### What does NOT need annotation (per tiered rule)

- Simple accessors and Display impls where parent type is annotated
- Internal utility functions with no security surface
- Test files

---

## Audit Artifacts

- Per-crate audit report in `.claude/reports/code/`
- Using ACCURATE/CONCERN/ERROR tiering (Jamie's preferred format)
- Summary table: crate, files audited, findings by tier, pass/fail

---

## Dependencies

- Must complete before M4 public release
- Tech-writer reviews source comments after Herb's audit pass
- Contributor guide (outreach plan Phase 3.4) references these standards
