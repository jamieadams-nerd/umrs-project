# UMRS New Zealand English (en_NZ) Vocabulary Reference

Canonical spelling and vocabulary decisions for UMRS New Zealand English localization.
Maintained by the `umrs-translator` agent.

**Scope**: Formal technical and government/security documentation register.
Follows New Zealand Government Web Standards and GCSB/NCSC-NZ published guidance.

**Update policy**: Add or revise an entry whenever a new term decision is made.
When a term has an established NZ equivalent, use it. Retain technical
acronyms and proper nouns unchanged (SELinux, MLS, CUI, NIST, etc.).

---

## Overview

New Zealand English follows British English spelling conventions closely,
essentially identical to Australian English in the formal/government register.

Key rules:
- **-ise** (not -ize): organise, authorise, recognise
- **-our** (not -or): colour, behaviour, honour
- **-re** (not -er): centre, fibre, calibre
- **-ll** doubling: labelled, cancelled, travelled

### Te Reo Māori considerations

New Zealand English may incorporate Te Reo Māori words and macrons (tohutō):
ā, ē, ī, ō, ū. For UMRS tooling, no Te Reo Māori integration is required at
this time. However:

- Ensure UTF-8 encoding is used throughout (already required by UMRS i18n policy)
- Do not strip macrons from proper nouns if they appear in NZ-context output
- Example: "Aotearoa" not "Aotearoa" with macrons stripped

---

## Spelling Conventions

NZ spelling rules are identical to en_GB and en_AU for all terms relevant to UMRS.
Refer to `vocabulary-en_GB.md` for the full spelling table.

### NZ-specific notes

| Topic | NZ Rule |
|-------|---------|
| -ise/-ize | Always -ise in formal NZ Government usage |
| program | "program" (not "programme") in computing contexts |
| organisation | -ise: "organisation" |
| licence/license | Licence (noun), license (verb) |

---

## Security and Access Control

| English (en_US)     | en_NZ equivalent    | Notes |
|---------------------|---------------------|-------|
| access denied       | access denied       | Identical |
| access control      | access control      | Identical |
| color-coded         | colour-coded        | Spelling change |
| labeled             | labelled            | Spelling change |
| authorized          | authorised          | Spelling change |
| recognized          | recognised          | Spelling change |
| behavior            | behaviour           | Spelling change |

---

## New Zealand Cybersecurity Terminology

The Government Communications Security Bureau (GCSB) and the National Cyber
Security Centre (NCSC-NZ) publish guidance in New Zealand English.

| Term | Notes |
|------|-------|
| GCSB | Government Communications Security Bureau — Five Eyes member |
| NCSC-NZ | National Cyber Security Centre (New Zealand) — distinct from UK NCSC |
| Protective Security Requirements (PSR) | NZ Government security policy framework |
| New Zealand Information Security Manual (NZISM) | Authoritative NZ cybersecurity standard |

---

## Locale Settings

| Field          | Value        |
|----------------|--------------|
| Locale code    | en_NZ        |
| Date format    | DD/MM/YYYY   |
| Decimal sep.   | . (period)   |
| Thousands sep. | , (comma)    |
| Currency       | NZ$ (NZD) — not relevant for UMRS tools |

---

## Sources

- New Zealand Government Web Standards (digital.govt.nz)
- GCSB / NCSC-NZ published advisories
- New Zealand Oxford Dictionary — authoritative NZ English reference
