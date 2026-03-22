---
name: HCI/IA/KO Corpus Status
description: Status of knowledge acquisition corpus for HCI, Information Architecture, Knowledge Organization, and Technical Communication collections — ingestion progress and priority order
type: project
---

# HCI/IA/KO Corpus Status

**Last updated:** 2026-03-22
**Why:** Familiarization session; green-lit by Jamie 2026-03-22.

## Ingestion Status

All Tier 1 collections from the Librarian's plan are DOWNLOADED but NOT YET INGESTED.

| Collection path | Contents | Ingestion status |
|---|---|---|
| `.claude/references/hci-courses/mit-6831/` | 26 files (MIT OCW 6.831, all 23 lectures) | NOT INGESTED |
| `.claude/references/hci-courses/stanford-cs147/` | 23 PDFs (Stanford CS147 Autumn 2023) | NOT INGESTED |
| `.claude/references/hci-courses/theory-papers/` | Blackwell/Green Cognitive Dimensions; Vannevar Bush 1945 | NOT INGESTED |
| `.claude/references/information-architecture/theory/` | Pirolli Ch. 1; Precision Content IA white paper | NOT INGESTED |
| `.claude/references/information-architecture/standards/` | NISO Z39.19-2005(R2010) controlled vocabularies | NOT INGESTED |
| `.claude/references/knowledge-organization/ieko/` | 120 IEKO encyclopedia articles (HTML) | NOT INGESTED |
| `.claude/references/knowledge-organization/texts/` | Svenonius Ch. 5; Hjørland KO theories | NOT INGESTED |
| `.claude/references/technical-communication/theory/` | Miller 1984; Bazerman 2011 genre theory papers | NOT INGESTED |

**Only current queryable doc-domain RAG corpus:** `doc-structure` (Diataxis, Antora, Red Hat modular, Google/GitLab style).

## Ingestion Priority Order (for Librarian request)

1. IEKO articles (120 HTML files) — highest density, broadest KO coverage
2. Blackwell/Green Cognitive Dimensions — directly applicable to UMRS display design
3. Stanford CS147 PDFs — practical HCI, most immediately applicable
4. Miller + Bazerman genre theory (2 PDFs) — foundational TC theory
5. NISO Z39.19 — controlled vocabulary standard, applicable to CUI taxonomy
6. MIT 6.831 PDFs — broader HCI depth (second tier urgency)
7. Pirolli Ch. 1 — IA/information foraging
8. Svenonius Ch. 5 + Hjørland — KO theory texts
9. Vannevar Bush 1945 — historical context only

## Outstanding Tier 2 Gap

Rosenfeld/Morville/Arango IA 4th ed. (polar bear book) — NOT available.
Most significant missing resource. Purchase decision needed from Jamie.
O'Reilly subscription would also unlock other texts.

## Key Concepts (pre-RAG, from document descriptions)

**For umrs-uname documentation:**
- Gulf of Evaluation (Norman): can operator determine system state? Trust tier table fails this.
- Information scent (Pirolli): section headers must match what operator is looking for.
- Patch value (Pirolli): help sections must satisfy the information need, not just describe it.
- User warrant vs. system warrant (NISO/Svenonius): tier labels have system warrant, not user warrant.
- Genre theory (Miller/Bazerman): help overlay is a distinct genre — completion criterion is operator action, not understanding.
- Cognitive Dimensions (Blackwell/Green): "role-expressiveness" dimension diagnoses trust tier opacity.
- Principle of User Convenience (Svenonius): structure around operator decision workflow, not system architecture.
