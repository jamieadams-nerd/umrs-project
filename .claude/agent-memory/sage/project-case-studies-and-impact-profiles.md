---
name: Case studies, impact profiles, and the SP 800-30 content engine
description: Synthesis of real-world CUI failure case studies, the IRS's impact profile recommendation (SP 800-30 Rev 1 Appendix H), and how these two bodies of work combine into Sage's highest-value content asset.
type: project
---

## Source Documents

- **Consolidated reference (primary):** `docs/modules/ROOT/pages/foundations/history/case-studies-consolidated.adoc`
  - Written 2026-04-03. Contains ALL cases from all three source files, merged and deduplicated.
  - Adds: Anthem, VW (full treatment), USDA/agriculture, COVID/public health, water rights,
    municipal police misuse, FRA/rail pattern, a full "Trust But Didn't Verify" section,
    and the "Quiet Killer" productivity erosion section.
  - This is now the authoritative case study document. Use this for blog source material.
- Original draft: `docs/modules/ROOT/pages/foundations/history/case-studies.adoc`
  - Kept per Archive-First Rule. Superseded by consolidated version for new content.
- Raw notes (Jamie's unedited input): `docs/_scratch/notes/case-studies.txt`
- IRS assessment: `.claude/reports/risk-domain-assessment-2026-03-21.md`
- CUI enrichment vision: `/home/jadams/.claude/projects/-media-psf-repos-umrs-project/memory/project_cui_enrichment_vision.md`

## The Case Study Inventory (by CUI category)

Each case maps to a CUI category and a specific failure class. Dollar amounts and conviction
details are from publicly available sources — suitable for citation.

| Case | CUI Category | Failure Class | Human Cost / Consequence |
|---|---|---|---|
| OPM Breach (2015) | CUI//PRIVACY | Over-access, no sensitivity segmentation | Millions of federal employee records exfiltrated; SSNs, financial history, foreign contacts |
| Equifax (2017) | CUI//PRIVACY | Insufficient segmentation; single vuln = broad access | ~147M individuals; $700M+ regulatory settlement |
| Twitter insider misuse (2019) | CUI//PRIVACY | Too much legitimate access; no compartment by purpose | Criminal charges; federal prosecution of employees |
| Flint water crisis | CUI//WATER, CUI//ENV | Integrity failure; sampling data altered; reports suppressed | Long-term public health harm; criminal convictions |
| Falsified drinking water reports | CUI//WATER | No tamper-evident provenance; corrections made without attribution | Federal conviction; 1,000+ falsified lab reports |
| Rail inspection falsification (WMATA, MBTA, LIRR) | CUI//TRAN, CUI//CRIT | Mutable records; no cryptographic binding to evidence | Federal investigations; active prosecutions |
| Boeing 737 MAX / FAA | CUI//CRIT | Provenance gaps in regulatory disclosures; no immutable record | Two crashes; 346 deaths; DOJ prosecution |
| Pipeline safety fraud | CUI//CRIT | Field test results not bound to device/user/time; retrospective fabrication | Federal conviction; infrastructure risk |
| Mars Climate Orbiter (1999) | (engineering context) | Data reused outside labeled context; units mismatch undetected | $125M spacecraft lost |
| Deepwater Horizon alarm bypass | CUI//CRIT | Safety config silently modified; no IMA/EVM protection | 11 deaths; $65B+ in costs |
| United States v. Vayner | (legal/evidence) | No provenance for digital evidence | Conviction vacated on appeal |
| Lorraine v. Markel | (legal/evidence) | Records existed but lacked authentication | Digital records excluded entirely |

**CRITICAL NOTE:** UMRS case studies cover Phase 1 (labeling, custody, audit) AND Phase 2 (IMA/EVM,
MLS enforcement) controls. When writing Sage content, always check phase1-phase2-positioning.md and
clearly separate which controls are available now vs. planned. IMA/EVM is NOT in Phase 1 release scope.

## The SP 800-30 Impact Profile Framework

The IRS assessed "risk domain" as a non-standard term with no authoritative anchor. Recommendation:
rename to **impact profile**, anchored to NIST SP 800-30 Rev 1, Appendix H, Table H-2.

Five authoritative impact types (short codes used in the JSON catalog):

| Code | Full name | When applicable |
|---|---|---|
| OPS | Harm to Operations | Mission degradation, noncompliance, reputational damage |
| ASSET | Harm to Assets | Loss of physical or information assets, intellectual property |
| IND | Harm to Individuals | Physical harm, PII exposure, identity theft |
| ORG | Harm to Other Organizations | Partner/contractor noncompliance, relational harms |
| NATION | Harm to the Nation | Critical infrastructure, national security, government continuity |

**Critical structural split (IRS finding):**
- `impact_profile` field — human awareness only; NOT machine-enforced; SP 800-30 source
- `handling_requirements` field — machine-verifiable by umrs-stat; NARA/DoD registry source
These two must remain separate. Mixing them implies that categories without an encrypted-at-rest
flag are exempt. That is a compliance defect, not a feature.

**Implementation sequence (per IRS):**
1. Fix NARA cross-reference accuracy problems (P1–P5 from earlier report) first
2. Add impact profiles (Phase 1 — human awareness)
3. Add handling_requirements enforcement (Phase 2 — umrs-stat verification)

## The UMRS CUI Enrichment Vision (Third Pillar)

Normal MLS systems: label the file, enforce access. Done.
UMRS adds: label + enforce + **GUIDE**.

The third pillar makes the catalog an advisory system:
- What happened historically when this category was mishandled (case studies)
- What class of damage occurs (SP 800-30 impact type)
- What handling is required (NARA-sourced requirements)
- What the kernel can verify (machine-verifiable subset)

Pitch formulation: "An onsite security officer in the filesystem."

## Why This Combination Is Novel

No framework has a construct that links:
- Authoritative CUI category identifiers (NARA)
- Adverse impact taxonomy (SP 800-30 Rev 1)
- Real case law and conviction records as demonstration
- Machine-verifiable handling requirements
- Kernel-level enforcement (Phase 2)

This is a genuine innovation. It is publishable. The case-to-category mapping alone is
conference-grade content — most CUI guidance is procedural; UMRS grounds it in outcomes.

## Why: How to apply in content

When writing any post that mentions CUI labels, the case studies are the proof layer.
The impact profile taxonomy is the framing layer. Together they move the narrative from
"marking files is compliance hygiene" to "this is what happens when the marking and the
enforcement aren't there."

Do not describe impact profiles or umrs-cui as complete or shipping — they are in design.
Describe the concept and the direction; note it is in development. Check release scope
before making capability claims.
