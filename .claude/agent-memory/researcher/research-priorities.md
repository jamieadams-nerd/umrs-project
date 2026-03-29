---
name: Research Priorities (Post-Roadmap Briefing)
description: Anticipated research tasks and RAG collection needs from roadmap M2/M3/M4 planning
type: project
---

# Research Priorities — Post-Roadmap Briefing (2026-03-21)

Derived from reading ROADMAP.md, sage-outreach-and-release-strategy.md, and umrs-assessment-engine.md.

## Immediate (no approval needed)

- **OSCAL v1.1.2 schemas** — Download from github.com/usnistgov/OSCAL (approved source).
  Target `.claude/references/nist/oscal/`. Build `oscal-schemas` RAG collection including OSCAL Reference docs
  from pages.nist.gov/OSCAL-Reference/ and FedRAMP OSCAL profile from github.com/GSA/fedramp-automation.
  **Why:** Assessment engine plan has explicit open question directed at researcher to confirm
  OSCAL version. Answer: target v1.1.2. FedRAMP RFC-0024 mandates OSCAL packages by Sept 2026.

- **Ingest `accreditation-artifacts` and `tui-cli`** — Downloaded 2026-03-15, still awaiting ingestion.
  These satisfy Phase 0 prerequisites for the assessment engine (FedRAMP SAR/SAP templates).

- **NIST SP 800-172, 800-161r1, 800-60v1r1** — All from nvlpubs.nist.gov (approved).
  - 800-172: Enhanced CUI requirements (above 800-171r3 high-water mark)
  - 800-161r1: Supply chain risk management (relevant to M4 crates.io release)
  - 800-60v1r1: Mapping information types to security categories (impact profile concept)

## Needs Source Approval (awaiting Jamie)

- **Five Eyes classification docs** — Need approval for:
  - protectivesecurity.gov.au (Australia PSPF Policy 8) — PSPF 2024 has Table 30 with cross-nation equivalency
  - gov.uk Cabinet Office (UK HMG Government Security Classifications Policy)
  - tbs-sct.gc.ca (Canada TBS Policy on Government Security, available EN + FR)
  - nzism.gcsb.govt.nz (New Zealand NZISM)
  All are official national government security policy sites (.gov.au, .gov.uk, .gc.ca, .govt.nz).
  **Why:** M3 requires Five Eyes interop mapping. Canada TBS is bilingual → source for Simone's FR translations.

- **CUI legal corpus** — Need approval for federalregister.gov OR use NARA-hosted versions:
  - 32 CFR Part 2002 (CUI regulation, the legal foundation)
  - Executive Order 13556 (established the CUI program)
  - ISOO CUI Notice 2020-01 (gap-filling implementation guidance)

## Research Without Downloads (can do now)

- **Five Eyes equivalency table** — Produce preliminary research report to `.claude/references/reports/`
  from domain knowledge + search. Lock citations once source approval granted.

- **Jamie's pre-Claude CUI legal research** — Needs to be located. Ask Jamie to check local
  files, bookmarks, notes from before Claude Code was in use. Content: real CUI breach
  consequences, case studies. Sage wants this for the "onsite security officer" blog angle.

## RAG Collections to Build (priority order)

1. `oscal-schemas` — OSCAL v1.1.2 JSON schemas + OSCAL Reference model docs + FedRAMP OSCAL profile
2. `five-eyes-classification` — PSPF/HMG/TBS/NZISM (needs source approval)
3. `cui-legal-corpus` — 32 CFR Part 2002, EO 13556, ISOO Notice 2020-01
4. `nist-800-161-supply-chain` — SP 800-161r1 (M4 supply chain risk)

## OSCAL Version Decision (for assessment engine plan)

**Confirmed: target OSCAL v1.1.2**
- Current stable release from usnistgov/OSCAL
- FedRAMP requires oscal-version >= 1.1.0 (RFC-0024, Sept 2026 mandate)
- JSON schema, XML schema, and YAML schema all available at same release tag
- Development snapshot exists (v1.2.0-dev) but not stable — do not target it
- This resolves the open architectural decision in umrs-assessment-engine.md

## Assessment Engine Phase 0 Gap Summary

Current status of Phase 0 prerequisites:
- SP 800-53A Rev 5: DONE (rmf-methodology collection, 1,132 chunks)
- SP 800-37 Rev 2: DONE (rmf-methodology collection)
- SP 800-30 Rev 1: DONE (rmf-methodology collection)
- FedRAMP SAR/SAP templates: DOWNLOADED, needs ingestion (accreditation-artifacts)
- OSCAL schema docs: NOT YET — this is the gap to close
