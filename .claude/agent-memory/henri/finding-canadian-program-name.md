---
name: Canadian program canonical naming analysis
description: Henri's formal analysis of the canonical short name for the Canadian Protected information program — PROTECTED/PROTÉGÉ, not an abbreviation
type: reference
---

## Finding: No Official Canadian Abbreviation

Canada does not have a branded program abbreviation equivalent to "CUI."
The word "Protected" / "Protégé" IS the designation, not an abbreviation of a longer program name.

Source: TBS Directive on Security Management, Appendix J (Standard on Security Categorization), 2019-07-01

## UMRS Convention (Approved by Jamie — pending)

| Context | US | Canadian (EN) | Canadian (FR) |
|---|---|---|---|
| Program reference | CUI | Protected | Protégé |
| JSON catalog keys | `"cui"` | `"protected"` | `"protege"` |
| Code enum/type | `Cui` | `Protected` | — (code is English) |
| Marking banner (EN) | `CUI//LEI/INV` | `PROTECTED B` | — |
| Marking banner (FR) | — | — | `PROTÉGÉ B` |
| setrans.conf | `CUI` | `PROTECTED` | `PROTÉGÉ` |
| MCS shorthand | `s0`–`s3` | `PA`, `PB`, `PC` | `PA`, `PB`, `PC` |
| Conversation | "CUI" | "Protected" or "PB" | "Protégé" or "PB" |

Note: PA/PB/PC are bilingual by nature — same letters, same levels in both languages.

## Formal Findings Filed

1. **Informational** — No official abbreviation; PA/PB/PC is UMRS convention, must document as such
2. **High** — CUI and Protected are structurally different systems; ROADMAP M3 language needs correction
3. **Medium** — Bilingual marking display: recommend display as-applied, locale-appropriate in explanatory output
