---
name: Jamie's answers to Henri's six onboarding questions
description: Received 2026-03-23. Establishes scope, authority level, and action items.
type: directive
---

## Jamie's Answers (2026-03-23)

### Q1: Deployment intent

UMRS is a **reference system**, not intended for production deployment on Canadian
government systems. It demonstrates that labeling and some enforcement CAN happen,
showcases high-assurance coding techniques, and encourages exploration and collaboration.
People take what they like and use as they see fit.

**Impact on my work:** All Canadian findings are **advisory/informational**, not blockers.
The Protected A/B/C mapping is a reference contribution, not a deployment prerequisite.

### Q2: TBS as compliance gate

Answered by Q1. TBS is a reference framework for UMRS purposes, not a compliance gate.
Canadian alignment is advisory. Divergences are informational findings, not blockers.

### Q3: TBS Standard ingestion

Jamie defers to my judgment. His rationale for the Canadian work: if UMRS was pre-wired
with i18n, Canada might appreciate the thoughtfulness. Forward-looking positioning.

**My decision:** I do NOT recommend RAG ingestion of the full TBS Standard at this time.
Rationale:
- The corpus files already in my agent-memory/corpus/ directory contain the authoritative
  text of the TBS Standard on Security Categorization (Appendix J) in both languages
- The key sections (J.2.4.2 Protected A/B/C definitions) are compact enough to reference
  directly from the corpus files
- RAG is most valuable for large, frequently-queried collections. The TBS documents are
  small and I know exactly which sections matter.
- If the scope expands beyond Protected A/B/C into the full TBS security management
  framework, revisit this decision.

### Q4: Canadian government contacts

No contacts. Jamie's background: many years on classified MLS systems, Five Eyes
CDS/MLS collaboration. The Canadian angle is forward-looking positioning, not a
current engagement pipeline.

### Q5: Category scope for Canadian program

Jamie confirms the sensitivity ladder:
- s1:c200-c299 = Protected A (confirmed range, room to breathe)
- s2:c200-c299 = Protected B
- s3:c200-c299 = Protected C

He does NOT have pre-existing Canadian category requirements. My assignments:
1. Identify what Canadian categories are needed (within c200-c299 range per tier)
2. Identify which categories are allowed together, which are mutually exclusive
3. Identify handling requirements (Canadian equivalent of dissemination controls)
4. Write it all up in a report
5. Build the CANADIAN-PROTECTED.json catalog (structural template: US CUI catalog)
6. Get Knox's US CUI catalog as a structural reference

### Q6: Termium Plus Military/Security download

Jamie approved. Route through the Librarian. The Librarian has used curl and other
acquisition techniques before.

## Recalibrated Operating Posture

| Before | After |
|---|---|
| Uncertain whether findings are blocking | All findings are advisory/informational |
| TBS compliance might be a hard gate | TBS is a reference framework |
| Uncertain about deployment context | Reference system only |
| Waiting on Jamie for TBS RAG decision | My call: not needed now, corpus files suffice |
| Waiting on Canadian category direction | I own the category design; Jamie reviews |

## Action Items (ordered)

1. [DONE] Save these decisions to agent memory
2. [DONE] Create tasks on task board
3. [DONE] Route Termium Plus Military/Security to Librarian (cross-team note)
4. [DONE] Write Canadian category requirements report (`.claude/reports/2026-03-23-canadian-protected-category-requirements.md`)
5. [DONE] Build CANADIAN-PROTECTED.json catalog (`components/rusty-gadgets/umrs-cui/canadian-protected.json`)
6. [ONGOING] Calibrate all future findings as advisory/informational

### Additional Decisions (2026-03-23, second round)

7. **Category range: c200-c299 per tier.** Jamie wants room to breathe. c200 = tier marker, c201-c299 reserved for future subcategories.
8. **Protected C: include in Phase 1.** Catalog and docs will need updating later with correct sX definitions. Phase 1 reads markings only. Include PC with caveat.
9. **UTF-8: confirmed.** All JSON files must use UTF-8 encoding. French accented characters stored directly.
