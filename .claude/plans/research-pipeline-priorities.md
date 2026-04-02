# Research Pipeline Priorities

**Date:** 2026-03-21
**Status:** in-progress — Priority 1 complete; Priority 2 Canada complete (Appendix J EN/FR downloaded 2026-03-31, AU/UK/NZ backburner); Priority 3 backburner; Priority 4 complete (all 3 docs downloaded + familiarized 2026-03-31, no RAG needed); FIPS 180-4 + 186-5 downloaded 2026-04-02; audit/logging research corpus (CC Part 2, IDMEF, CEF, DFXML, CASE, ITU X.721) acquisition started 2026-04-02
**Author:** The Librarian (researcher)
**ROADMAP Goals:** G3 (CUI & Five Eyes), G4 (Assessment Engine), G7 (Public Project)
**Milestones:** M2 (OSCAL), M3 (CUI/Five Eyes)
**Tech Lead:** The Librarian (researcher)
**LOE:** Medium (~3-4 sessions across all collections; OSCAL is ~1 session)

---

## Purpose

Track research acquisitions, RAG collection builds, and reference material needed
to unblock downstream work across the team.

---

## Priority 1 — OSCAL Schemas Collection

**Unblocks:** Assessment engine Phase 0
**LOE:** Small (~1 session)
**Source approval needed:** None (NIST GitHub and GSA GitHub already approved)

- Target: OSCAL v1.1.2 (confirmed current stable)
- Sources: `github.com/usnistgov/OSCAL`, `github.com/GSA/fedramp-automation`
- FedRAMP RFC-0024 mandates OSCAL packages by September 2026
- Deliverable: `oscal-schemas` RAG collection + familiarization pass

## Priority 2 — Five Eyes Classification Mappings

**Unblocks:** M3 CUI Ready, Simone's translation reference material
**LOE:** Medium (~1-2 sessions for acquisition + RAG build)
**Source approval needed from Jamie:**

| Site | Country | Content |
|---|---|---|
| `protectivesecurity.gov.au` | Australia | PSPF Policy 8 — classification equivalency table |
| `gov.uk` (Cabinet Office) | UK | UK classification policy |
| `tbs-sct.gc.ca` | Canada | Canadian security policy (bilingual — also serves Simone) |
| `nzism.gcsb.govt.nz` | New Zealand | NZ information security manual |

- Anchor document: Australia's PSPF Policy 8 Table 30 (maps all Five Eyes equivalents)
- Canada's TBS is bilingual — doubles as Simone's CUI terminology reference
- Deliverable: `five-eyes-classification` RAG collection

## Priority 3 — CUI Legal Corpus

**Unblocks:** CUI catalog authority, legal breach consequence documentation
**LOE:** Small (~1 session)
**Source approval needed:** `federalregister.gov`

- 32 CFR Part 2002 (CUI regulation)
- Executive Order 13556 (CUI establishment)
- ISOO Notice 2020-01 (CUI implementation guidance)
- Jamie's pre-Claude-Code legal/case study research on CUI breach consequences —
  needs to be located and integrated
- Deliverable: `cui-legal-corpus` RAG collection

## Priority 4 — Additional NIST Standards

**Unblocks:** Enhanced CUI posture, supply chain, information typing
**LOE:** Small (~1 session)
**Source approval needed:** None (nvlpubs.nist.gov already approved)

- NIST SP 800-172: Enhanced Security for CUI (advanced controls)
- NIST SP 800-161r1: Supply Chain Risk Management
- NIST SP 800-60v1r1: Information Type to Security Category Mapping
- Add to `.claude/references/refs-manifest.md` with version, date, SHA-256

---

## Standards We Should Be Tracking But Aren't

| Standard | Why |
|---|---|
| 32 CFR Part 2002 | CUI regulatory authority — the legal foundation |
| OSCAL v1.1.2 schemas | Assessment engine export target |
| NIST SP 800-172 | Enhanced CUI controls beyond 800-53 |
| NIST SP 800-161r1 | Supply chain risk (relevant to our dependency scrutiny) |
| NIST SP 800-60v1r1 | Maps information types to security categories |
| FedRAMP RFC-0024 | OSCAL package mandate timeline |

---

## Simone's Reference Material (from this pipeline)

Per Simone's briefing, she needs before the M3 translation sprint:

1. Canadian TBS Policy on Government Security (PGS) — French version (from Priority 2)
2. NATO/OTAN security policy docs (STANAG 4406) — separate acquisition TBD
3. ANSSI security vocabulary glossary — separate acquisition TBD

The TBS document from Priority 2 serves double duty: Five Eyes mapping AND French
CUI terminology reference.
