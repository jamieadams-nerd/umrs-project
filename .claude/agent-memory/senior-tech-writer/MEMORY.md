# Senior Tech Writer — Persistent Memory

## Cross-Team Channel
`.claude/agent-memory/cross-team/notes.md` — shared with all agents.
Read at session start for entries addressed to senior-tech-writer.

## Topic Files
- [Restructure history](restructure_history.md) — page lists, reorganization details, PQC expansion notes
- [SCAP familiarization](scap_familiarization.md) — CCE citation format, STIG patterns, Phase 3 blockers

## Project: UMRS Documentation (Antora)

**Docs root**: `docs/modules/`
**Modules** (13): ROOT, architecture, security-concepts, devel, deployment, operations, logging-audit, cryptography, reference, umrs-tools, patterns, glossary, ai-transparency
**antora.yml** registers all module nav files. Single component descriptor at `docs/antora.yml`.

---

## Key Structural Facts

- `.md`/`.txt` files in `pages/` do NOT render in Antora — convert to `.adoc`
- `security/` and `historical/` modules retired (2026-03-11) — content in architecture/ and devel/
- `admin/` merged into `operations/` — originals still present pending Jamie cleanup
- Operations module has NO Logging section — all logging in deployment/ (setup) or logging-audit/ (ops)
- `architecture:rationale.adoc` does NOT exist — correct filename is `architecture:rationale-strongly-typed.adoc`
- **Antora `examples/` constraint**: Files in `examples/` cannot be navigable pages or xref targets. They are includable fragments only.

---

## Structural Decisions (confirmed by Jamie)

- `umrs-tools/` stays as separate module
- Use cases: user-story format ("As a developer, I want to [goal]")
- Project origin narrative: third person, factual
- "Five Eyes" = multi-national classification interop (US/UK/CA/AU/NZ)
- Prototype crates get placeholder pages only

---

## Terminology
See `.claude/agent-memory/doc-team/approved_terminology.md` for the full list.
Key: `security context` (not label), `sensitivity level` (not label), never abbreviate `high-assurance` as `HA`.

---

## Build Verification
`make docs` must pass cleanly before any docs/ work is done. No exceptions.
When moving pages, update ALL xrefs across ALL modules.

---

## Multi-Component Split Plan (pending Jamie approval)

Plan: `.claude/plans/antora-multi-component-split.md`
Review: `.claude/reports/multi-component-split-review.md`
Key: 272 xrefs across 72 files — scripted substitution mandatory. Nav drafts required before files move.
4 pre-migration prerequisites: rhel10-install.adoc duplicate, admin/ originals, security-model.adoc stub, apparmor/ empty dir.

---

## Writing Modes
- Architecture Mode for explanatory content
- STE Mode for procedures
- Load rules file before writing

---

## Primary Source Documents
- `README.md` — high-assurance engineering, HACAMS lineage
- `UMRS-PROJECT.md` — project description, MLS label hierarchy, CUI handling

---

## FIPS / PQC Constraint (authoritative)

FIPS mode and PQC are mutually exclusive on current RHEL 10.
CUI systems must use FIPS mode → no PQC until FIPS 140-3 validation completes.
RHEL 10.1+: PQC GA under DEFAULT policy for non-FIPS systems only.

---

## SELinux Source Reference
CategorySet: dense [u64;16] vs kernel sparse ebitmap.
SelinuxType: mixed-case allowed. SelinuxUser/Role: lowercase [a-z0-9_] only.
dominates(): `(subject & object) == object`, word-by-word across 16 u64 words.

---

## Style Corpus — Quick Reference

**Location**: `.claude/knowledge/tech-writer-corpus/`
**RAG collection**: `tech-writer-corpus`
**Artifacts**: concept-index.md, cross-reference-map.md, style-decision-record.md, term-glossary.md

**Source priority** (highest → lowest):
1. MIL-STD-38784B — DoD TM structure, WARNING/CAUTION/NOTE
2. Federal Plain Language Guidelines — statutory; active voice, 15-20 word sentences
3. NIST Author Instructions — Section 508, inclusive terminology (CANONICAL)
4. Google Developer Documentation Style Guide
5. Microsoft Writing Style Guide

**Key SDRs**: SDR-001 contractions by module, SDR-007 sentence length, SDR-009 person by module (3rd for architecture/security-concepts, 2nd elsewhere), SDR-010 inclusive terms.
Full SDR list in `style-decision-record.md`.

**Common Criteria**: CC:2022 Parts 1+2 ingested. SFR uses "shall". 6 CC terms in glossary.

---

## SCAP/STIG Corpus (Phase 2 complete — 2026-03-17)

CCE format: `NIST SP 800-53 CM-6 | CCE-89232-3 (RHEL 10 STIG)` — NIST leads, CCE is index key.
CCE only in reference/ and devel/ — never in operations/ or deployment/.
Phase 3b blocked until rust-developer adds `cce` field (Phase 3a).
RAG chunking fix needed before Phase 3 integration.
Full details: [scap_familiarization.md](scap_familiarization.md)

---

## CPU Extensions Reference (2026-03-16)

`docs/modules/reference/pages/cpu-extensions.adoc` — three-layer activation model.
Nav: "Platform Security" section in reference/nav.adoc.
`crypto-cpu-extensions.adoc` was a phantom stub — `cpu-extensions.adoc` fulfills that intent.

## Platform Update Checklists (2026-03-16)

`docs/modules/devel/pages/update-checklists.adoc` — kernel version, CPU extension, signal deprecation.
Nav: "Platform Internals" in devel/nav.adoc.

## Navigation Plan (2026-03-13)

Plan: `.claude/plans/docs-new-stuff-crypto-and-navbar.md`
Open questions for Jamie: Q1 page label disambiguation, Q2 historical consolidation, Q3 API orientation page.

## Pattern Library Notes

16 standard patterns in `patterns/pages/`. Two-zone structure. Nav groups: Architectural / Coding / Observability / Process / Deep Dives.
Zeroize and Constant-Time: not yet implemented. SEC: implemented.

## OS Detection Docs

`patterns/pages/pattern-os-detection.adoc` + `devel/pages/os-detection-deep-dive.adoc`.
FIPS gate: sha2 not validated; ceiling T3.
