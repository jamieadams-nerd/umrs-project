# Tech-Writer RAG Corpus Acquisition Plan

**Created:** 2026-03-15
**Status:** Phases 1-2 in progress (Phase 1.1 partial, Phase 1.2 complete, Phase 2 substantially complete)
**Source:** `.claude/jamies_brain/researcher-for-tw.md`
**ROADMAP Goals:** G10 (AI Transparency), G1 (Documentation Excellence)
**Agent:** researcher
**Depends on:** Nothing — can start immediately

---

## Purpose

Acquire style guides, government writing standards, and domain reference materials to build
a RAG corpus that makes the tech-writer and senior-tech-writer agents write with correct
terminology, register, and structure for the Five Eyes / federal security audience.

---

## Execution Order

Phases are ordered by priority. Within a phase, items can be fetched in parallel.

---

## Phase 1 — High-Priority Style Guides

| # | Source | Format | Notes |
|---|--------|--------|-------|
| 1.1 | Google Developer Documentation Style Guide | Web crawl (`developers.google.com/style`) | Crawl `/style/` prefix; exclude `/tech-writing/` |
| 1.2 | Microsoft Writing Style Guide | Web crawl (`learn.microsoft.com/en-us/style-guide/`) | Prioritize: word choice, procedures, bias-free, A-Z word list |

**Save to:** `.claude/references/tech-writer-corpus/style-guides/`

---

## Phase 2 — Government & Defense Writing Standards

| # | Source | Format | Notes |
|---|--------|--------|-------|
| 2.1 | MIL-STD-38784A (DoD Technical Manual Style) | PDF from everyspec.com | Governs warnings, cautions, notes in procedures |
| 2.2 | NIST Technical Series Author Instructions | Web (single page) | Plain Writing Act, Section 508, equation/figure formatting |
| 2.3 | Plain Language Guidelines (Federal) | Web crawl (`plainlanguage.gov/guidelines/`) + PDF | Mandated baseline for federal-adjacent documentation |

**Save to:** `.claude/references/tech-writer-corpus/gov-standards/`

---

## Phase 3 — Domain Reference Materials

These ensure correct terminology when writing about the security domain.

| # | Source | Format | Priority | Notes |
|---|--------|--------|----------|-------|
| 3.1 | NIST SP 800-53 Rev 5 | PDF | Critical | **CHECK:** may already exist at `refs/nist/sp800-53r5.pdf` — do NOT re-acquire if present |
| 3.2 | NIST SP 800-171 Rev 3 | PDF | High | **CHECK:** may already exist at `refs/nist/` |
| 3.3 | CMMC Level 2 Assessment Guide | PDF from `dodcio.defense.gov/CMMC/` | High | **CHECK:** may already exist at `refs/dod/` — may need manual browser download |
| 3.4 | RHEL 10 Security Guide | Web/PDF from Red Hat docs | High | SELinux, MLS, IMA, LUKS, audit chapters |
| 3.5 | SELinux Project Notebook | Web crawl (`selinuxproject.org`) | Medium | **CHECK:** already at `.claude/references/selinux-notebook/` — only ingest into tech-writer collection if not already there |
| 3.6 | Common Criteria Parts 1 & 2 | PDF from `commoncriteriaportal.org` | Medium | SFR naming conventions and formal register |

**Save to:** `.claude/references/tech-writer-corpus/domain-refs/`

**IMPORTANT:** Phase 3 items overlap with existing `refs/` and `.claude/references/` collections.
The researcher must CHECK existing material first and only acquire what's missing. For items
that already exist, create symlinks or reference pointers rather than duplicating files.

---

## Phase 4 — Medium-Priority Style Guides

| # | Source | Format | Notes |
|---|--------|--------|-------|
| 4.1 | Apple Style Guide | PDF + Web | International audience, writing for translation |
| 4.2 | DigitalOcean Writing Guidelines | Web + GitHub templates | Tutorial structure framework |
| 4.3 | Mailchimp Content Style Guide | Web crawl (`styleguide.mailchimp.com`) | Openly licensed; store attribution |

**Save to:** `.claude/references/tech-writer-corpus/style-guides/`

---

## Phase 5 — Supplemental Reference

| # | Source | Format | Priority | Notes |
|---|--------|--------|----------|-------|
| 5.1 | NASA Technical Report Writing Guidance | PDF from `ntrs.nasa.gov` | Low-Medium | Safety-critical procedure writing model |

**Save to:** `.claude/references/tech-writer-corpus/supplemental/`

---

## Phase 6 — RAG Ingestion & Familiarization

**Agent:** researcher (ingestion), then tech-writer + senior-tech-writer (familiarization)

1. Ingest all acquired materials into RAG collection `tech-writer-corpus`
2. Tech-writer agent runs corpus-familiarization skill
3. Senior-tech-writer agent runs corpus-familiarization skill
4. Both agents update their MEMORY.md with knowledge index entries

---

## Post-Phase Hygiene

- Log each phase to `.claude/logs/task-log.md`
- Update this plan with phase status after each completion
- Flag any sources requiring manual browser download (DoD portals, etc.)
- Notify Jamie when Phase 6 familiarization is complete

---

## Acquisition Checklist

| Item | Phase | Priority | Status |
|------|-------|----------|--------|
| Google Developer Style Guide | 1 | High | DONE — 10 pages fetched via curl+pandoc (2026-03-16); verbatim content replaces prior summaries |
| Microsoft Writing Style Guide | 1 | High | DONE — 13 files (2026-03-16) |
| MIL-STD-38784B | 2 | High | DONE — PDF downloaded from everyspec.com (Jamie-approved); 3.95 MB; SHA-256 recorded (2026-03-16) |
| NIST Author Instructions | 2 | High | DONE — 740-line markdown fetched via curl+pandoc (2026-03-16) |
| Plain Language Guidelines | 2 | High | PARTIAL — PDF URL dead (301 to digital.gov); 6 guideline pages fetched from GSA GitHub archive (CC0); original PDF no longer available via curl |
| NIST SP 800-53 Rev 5 | 3 | Critical | CHECK existing |
| NIST SP 800-171 Rev 3 | 3 | High | CHECK existing |
| CMMC Level 2 Assessment Guide | 3 | High | CHECK existing |
| RHEL 10 Security Guide | 3 | High | -- |
| SELinux Project Notebook | 3 | Medium | CHECK existing |
| Common Criteria Parts 1 & 2 | 3 | Medium | -- |
| Apple Style Guide | 4 | Medium | -- |
| DigitalOcean Guidelines | 4 | Medium | -- |
| Mailchimp Style Guide | 4 | Medium | -- |
| NASA Writing Guidance | 5 | Low-Med | -- |
