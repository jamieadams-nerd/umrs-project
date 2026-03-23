# Corpus Delta Report — umrs-uname fr_CA.po
## Pre-corpus vs Post-corpus Translation Quality

**Date:** 2026-03-23
**Baseline:** `fr_CA.po.pre-corpus-baseline` (GNU corpus only)
**New sources:** Termium Plus (32,210 entries), OQLF GDT (25,881 entries)
**Translations reviewed:** 95 msgid/msgstr pairs

---

## Summary Statistics

| Metric | Count | % |
|---|---|---|
| Total translations reviewed | 95 | 100% |
| Corpus confirmed (exact or near match) | 12 | 13% |
| Corpus suggests change | 3 | 3% |
| No corpus match (UMRS-specific phrases) | 80 | 84% |

The 84% "no match" rate is expected and healthy — most UMRS translations are
compound security phrases ("Kernel lockdown LSM restricts...") that no
terminology database would contain. The databases validate *individual terms
within* those phrases, not the phrases themselves.

---

## Confirmed Terms (Simone got it right)

These translations matched the authoritative corpus without changes:

| English | Simone's Translation | Source |
|---|---|---|
| Configuration file | Fichier de configuration | Termium |
| Assessment | Évaluation | Termium |
| Host | Hôte | GDT |
| Tool | Outil | GDT |
| OS | SE (système d'exploitation) | GDT |
| Indicator | Indicateur | GDT + Termium |
| Integrity | Intégrité | GDT + Termium |
| Module | Module | GDT + Termium |
| Vulnerability | Vulnérabilité | GDT + Termium |
| Encryption | Chiffrement | GDT + Termium |
| Contradictions | Contradictions | — (same in both languages) |
| Description | Description | — (same in both languages) |

## Suggested Changes

| msgid | Old msgstr | Corpus recommendation | Source | Action |
|---|---|---|---|---|
| Status | État | statut | GDT | KEEP "État" — "statut" is generic; "état" is standard for system state in GC IT docs |
| not available | non disponible | pas disponible | Termium | CHANGE — Termium preferred form |
| unavailable | indisponible | non disponible | Termium | REVIEW — "indisponible" is valid fr_CA; Termium prefers "non disponible" |

## Simone's Coined Terms — Corpus Validation

| Coined Term | Simone Used | Termium | GDT | Verdict |
|---|---|---|---|---|
| hard gate | verrou dur | no match | no match | UMRS coinage stands — no precedent exists |
| trust tier | palier de confiance | no match | no match | UMRS coinage stands — novel concept |
| security posture | posture de sécurité | **Posture de sécurité** | posture en matière de sécurité | **CONFIRMED by Termium** — exact match |
| posture indicator | indicateur de posture | no match | no match | UMRS coinage stands — compound of confirmed terms |

**Key finding:** "posture de sécurité" was confirmed by Termium Plus as the federal
standard. Simone's instinct matched the Government of Canada's official terminology
without access to the database. The GDT alternative ("posture en matière de sécurité")
is more formal but not preferred for operator-facing text.

## Notable Divergences Between Databases

| English | Termium (federal) | GDT (Quebec) | UMRS Decision |
|---|---|---|---|
| hardening | renforcement | bastionnage | Use "durcissement" (Simone's choice) — UMRS-specific, clearer for operators |
| hash/digest | condensé numérique | condensé | Use "condensé" (Simone's choice) — shorter, matches ANSSI usage |
| audit trail | journal d'audit | piste de contrôle | Use "journal d'audit" (Termium) when this term appears |
| evidence | donnée probante (Termium implied) | donnée probante | Simone uses "preuves" — acceptable, more direct |
| baseline | ligne de base | ligne de fond | Simone uses "base de référence" — better for UMRS context |
| kernel | (not in Termium IT) | noyau | Confirmed: "noyau" is standard |

## Observations

1. **Simone's pre-corpus accuracy was high.** Of the terms that had corpus matches,
   most were confirmed. The 3 suggested changes are minor (register preference, not errors).

2. **The databases are most valuable for future work**, not corrections. When Simone
   translates new strings for umrs-platform, umrs-ls, umrs-stat, she can now look up
   every technical term before coining.

3. **"verrou dur" and "palier de confiance" are genuine UMRS coinages** — no precedent
   in either database. These should be documented in the UMRS glossary as project
   terminology with a note that they are not attested in GC or OQLF sources.

4. **Termium confirmed "posture de sécurité"** — this is the headline finding. Simone's
   most-used security term matched the federal standard exactly.

---

## Process Improvement

The `french-lookup` skill now searches Termium Plus and GDT before the GNU corpus.
For all future translation work, Simone should run lookups for every technical term
before translating. The databases are the authority; GNU corpus is the fallback.
