---
name: french-lookup
description: Search the GNU translation corpus for technical French terms in priority order (coreutils first). Use this before any translation task.
---

# French Lookup Skill
Use this skill to query the `.claude/corpus/gnu-fr/` directory for existing technical translations.

## Usage
Run the command with the English term you want to find:
`bash .claude/commands/french_lookup.sh [term]`

## Rules
- Always check this skill before proposing a translation for a technical UI string.
- If a match is found in a higher-priority file (e.g., coreutils), you must adopt that terminology.
- Use the `msgstr` value from the output as your primary source of truth.

