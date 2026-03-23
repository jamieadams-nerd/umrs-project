---
name: french-lookup
description: >
  Search Canadian French terminology databases and GNU translation corpus for
  technical terms. Priority order: Termium Plus (federal) → OQLF GDT (Quebec)
  → GNU coreutils → GNU other. Use this before any translation task to find
  the authoritative fr_CA form. Trigger when translating UI strings, validating
  existing translations, or checking for "France French drift" in .po files.
---

# French Lookup Skill

Queries `.claude/corpus/` for authoritative Canadian French technical terminology.

## Search Priority

1. **Termium Plus** (.tsv) — Government of Canada federal terminology (highest authority)
2. **OQLF GDT** (.tsv) — Quebec official terminology
3. **CCCS Cyber Glossary** (.tsv) — Canadian Centre for Cyber Security
4. **GNU coreutils** (.po) — open-source baseline
5. **GNU other** (.po) — cryptsetup, util-linux, grep, sed, tar, findutils, bash

## Usage

```bash
bash .claude/commands/french_lookup.sh "security posture"
bash .claude/commands/french_lookup.sh "encryption"
bash .claude/commands/french_lookup.sh "audit"
```

## Rules

- Always check this skill before proposing a translation for a technical UI string.
- If a match is found in a higher-priority source, adopt that terminology.
- Termium Plus and GDT results are authoritative fr_CA — prefer them over GNU corpus.
- GNU corpus results may be fr-FR (France French) — flag when the register differs.
- If no match is found in any source, document the coinage in `vocabulary-fr_CA.md`
  with a note that it needs corpus validation.
