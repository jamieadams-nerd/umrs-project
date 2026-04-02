# Appendix J French Terminology Report

**Date:** 2026-03-31
**Source:** TBS DOSM Appendix J — Catégorisation de sécurité
**Agent:** Simone (umrs-translator)

## Overall Assessment

The current UMRS `CANADIAN-PROTECTED.json` (v0.2.0) French terminology is **substantively accurate**.
Core descriptions align with TBS Appendix J verbatim text. Three items require attention.

## Findings

### FR-J-05 — Non-Breaking Space in `name_fr` (RECOMMENDED CORRECTION)

The `name_fr` fields use regular spaces between "Protégé" and the tier letter.
TBS HTML uses `&nbsp;` (U+00A0) throughout. The correct forms are:
- `"Protégé\u00a0A"`
- `"Protégé\u00a0B"`
- `"Protégé\u00a0C"`

This affects any UI rendering that might line-break within the designation.
JSON files are protected — flagged for Jamie's decision.

### FR-J-04 — Non-Breaking Space in `marking_banner_fr` (LOW PRIORITY)

Same non-breaking space issue applies to `marking_banner_fr` fields
(currently `"PROTÉGÉ A"` etc.). Low priority since all-caps banner rendering
is standard and the regular space is unlikely to cause line-breaking in practice.

### FR-J-03 — Protected C `description_fr` Redundancy (FUTURE)

Protected C `description_fr` includes the injury example inline
("par exemple, la perte de vie"), creating redundancy with `injury_examples_fr`.
PA and PB keep the example separate. Alignment is cosmetic but worth tracking.

## Critical Vocabulary Distinction

**Confirmed correct:** "préjudice extrêmement grave" (Protected C) vs
"préjudice exceptionnellement grave" (Top Secret) are two distinct TBS terms.
The UMRS file correctly uses "extrêmement grave" for Protected C.

## Core Terminology Table (TBS Authoritative)

| English (TBS) | French (TBS) | Notes |
|---|---|---|
| Protected A | Protégé A | Non-breaking space before letter |
| Protected B | Protégé B | Non-breaking space before letter |
| Protected C | Protégé C | Non-breaking space before letter |
| Security categorization | Catégorisation de sécurité | |
| Injury | Préjudice | |
| Low injury | Préjudice faible | Protected A threshold |
| Serious injury | Préjudice grave | Protected B threshold |
| Extremely grave injury | Préjudice extrêmement grave | Protected C threshold |
| National interest | Intérêt national | Classified vs Protected boundary |
| Confidentiality | Confidentialité | |
| Integrity | Intégrité | |
| Availability | Disponibilité | |
| Information asset | Bien d'information | |
| Safeguarding | Protection | |
| Classified | Classifié | |
| Unclassified | Non classifié | |
| Compromise | Compromission | |

## New Vocabulary Count

28 terms from TBS Appendix J need to be added to `vocabulary-fr_CA.md` —
primarily injury-level terminology and categorization process vocabulary.
