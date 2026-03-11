# UMRS Australian English (en_AU) Vocabulary Reference

Canonical spelling and vocabulary decisions for UMRS Australian English localization.
Maintained by the `umrs-translator` agent.

**Scope**: Formal technical and government/security documentation register.
Follows the Australian Government Style Manual (style.gov.au) and ASD/ACSC guidance.

**Update policy**: Add or revise an entry whenever a new term decision is made.
When a term has an established Australian equivalent, use it. Retain technical
acronyms and proper nouns unchanged (SELinux, MLS, CUI, NIST, etc.).

---

## Overview

Australian English spelling follows British English conventions in formal and
government contexts. The Australian Government Style Manual (updated 2021) mandates:

- **-ise** (not -ize): organise, authorise, recognise
- **-our** (not -or): colour, behaviour, honour
- **-re** (not -er): centre, fibre, calibre
- **-ll** doubling: labelled, cancelled, travelled

These rules apply consistently in UMRS tool output and documentation targeting
Australian audiences.

---

## Spelling Conventions

The en_AU spelling rules are identical to en_GB in all cases relevant to UMRS tooling.
Refer to `vocabulary-en_GB.md` for the full spelling table.

### AU-specific notes

| Topic | Australian Style Manual Rule |
|-------|------------------------------|
| -ise/-ize | Always -ise in formal Australian Government usage |
| program | "program" (not "programme") in computing contexts |
| organisation | -ise: "organisation" is correct (not "organization") |
| licence/license | Licence (noun), license (verb) — same as en_GB |

---

## Security and Access Control

| English (en_US)     | en_AU equivalent    | Notes |
|---------------------|---------------------|-------|
| access denied       | access denied       | Identical |
| access control      | access control      | Identical |
| color-coded         | colour-coded        | Spelling change |
| labeled             | labelled            | Spelling change |
| authorized          | authorised          | Spelling change |
| recognized          | recognised          | Spelling change |
| behavior            | behaviour           | Spelling change |

---

## Australian Government Cybersecurity Terminology

The Australian Signals Directorate (ASD) and Australian Cyber Security Centre (ACSC)
publish guidance in Australian English. Key terms from ACSC publications:

| Term | Notes |
|------|-------|
| Essential Eight | ASD's eight mitigation strategies — use exact name |
| Information Security Manual (ISM) | ASD's cybersecurity framework — use exact name |
| Protective Security Policy Framework (PSPF) | Australian Government policy — use exact name |
| security classification | Australian classification terms differ from US; do not translate US-specific terms to AU classification vocabulary |

---

## Locale Settings

| Field          | Value        |
|----------------|--------------|
| Locale code    | en_AU        |
| Date format    | DD/MM/YYYY   |
| Decimal sep.   | . (period)   |
| Thousands sep. | , (comma)    |
| Currency       | A$ (AUD) — not relevant for UMRS tools |

---

## Sources

- Australian Government Style Manual (style.gov.au, 2021 edition)
- ASD/ACSC Information Security Manual (ISM) — terminology reference
- Macquarie Dictionary — authoritative Australian English reference
