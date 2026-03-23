# Translation Visual Verification Checklist

**Applies to:** All umrs-* binary domains
**Run after:** Every translation update pass before compilation
**Owned by:** umrs-translator (Simone)

This checklist enforces the three rules established 2026-03-23 after live TUI
testing identified column-width and display-width regressions in umrs-uname.

---

## Rule 1 — Key Column Width

**Threshold:** French key must not exceed 150% of the English msgid length
for strings used as row keys in the audit card data panel.

**Why:** The TUI key/value split is computed at render time. Oversized keys
push the value column off-screen or force line wrapping that corrupts
table layout.

**Check method:** For every msgid that appears as a `DataRow` key argument,
measure the character lengths. Flag any msgstr that exceeds 150% of the msgid.

| msgid (English key) | len | msgstr (French key) | len | Ratio | Status |
|---|---|---|---|---|---|
| `Label Trust` | 11 | `Confiance` | 9 | 0.82 | OK |
| `Trust Tier` | 10 | `Palier` | 6 | 0.60 | OK |
| `Downgrade Reasons` | 17 | `Motifs` | 6 | 0.35 | OK |
| `Evidence Records` | 16 | `Preuves` | 7 | 0.44 | OK |
| `Description` | 11 | `Description` | 11 | 1.00 | OK |
| `Contradictions` | 14 | `Contradictions` | 14 | 1.00 | OK |
| `Status` | 6 | `État` | 4 | 0.67 | OK |
| `Reason` | 6 | `Raison` | 6 | 1.00 | OK |
| `Trust Level` | 11 | `Niveau de confiance` | 20 | 1.82 | WARN — used only in error-state rows; verify display |
| `Kernel Version` | 14 | `Version du noyau` | 17 | 1.21 | OK |
| `Indicators` | 10 | `Indicateurs` | 12 | 1.20 | OK |
| `No Assessment` | 13 | `Sans évaluation` | 16 | 1.23 | OK |
| `Catalog Baseline` | 16 | `Base de référence du catalogue` | 30 | 1.88 | WARN — verify display in summary pane |
| `Platform Family` | 15 | `Famille de plateforme` | 22 | 1.47 | OK |
| `Platform Distro` | 15 | `Distribution` | 13 | 0.87 | OK |
| `Platform Version` | 16 | `Version de plateforme` | 22 | 1.38 | OK |
| `Platform Facts` | 14 | `Faits de plateforme` | 20 | 1.43 | OK |
| `Probe Used` | 10 | `Sonde utilisée` | 15 | 1.50 | OK (at limit) |
| `Platform Identity` | 17 | `Identité de plateforme` | 23 | 1.35 | OK |

**WARN entries** require live TUI verification at next test session. They are
used in contexts where the display pane may be wider, but Jamie should confirm
they do not jam the value column in the actual render.

---

## Rule 2 — Multi-line Help Text Width

**Threshold:** No translated line in a help text block may exceed 66 characters.

**Why:** The TUI dialog renders help text in a fixed-width overlay. Lines that
exceed 66 characters are clipped or wrap in ways that break the ASCII
column-alignment of the navigation block.

**Check method:** After translating any help text block (the multi-line raw
string msgids), scan every line of the msgstr for character count. Flag any
line that exceeds 66 characters.

The width comments in the .po source (e.g., `# WIDTH: 66-char limit`) record
the per-line verification status. Translator must update these comments when
editing help text.

**Current status (2026-03-23):**

All help text msgstr blocks were verified at or under 66 characters per line
during the initial translation session. Flagged lines in the .po source carry
explicit WIDTH FLAG comments.

Lines to re-verify after any edit to Tab 0, Tab 1, or Tab 2 help text:

- Tab 0 (OS Information): longest French line is 66 chars — at limit
- Tab 1 (Kernel Security): longest French line is 64 chars — OK
- Tab 2 (Trust / Evidence): longest French line noted at 67 chars in one
  entry — review `"   ◼ Toute contradiction nécessite une révision manuelle. Une\n"`

---

## Rule 3 — Single-line Display String Length Ratio

**Threshold:** No French translation of a single-line display string may
exceed 150% of its English msgid character count.

**Why:** Single-line display strings appear in status bars, column headers,
row values, and short labels. Excessive expansion causes truncation or
wrapping in fixed-width contexts.

**Scope:** This rule applies to all msgids that are NOT multi-line help text
blocks. Multi-line blocks are governed by Rule 2 (66-char per-line limit).

**Check method:** `len(msgstr) / len(msgid) <= 1.50` for every non-multi-line
entry.

**Exceptions recorded (pre-existing, reviewed 2026-03-23):**

These entries exceed 150% but are NOT key-column strings. Each has a context
justification. Flagged for live TUI verification at next test session.

| msgid | len | msgstr | len | Ratio | Context | Action |
|---|---|---|---|---|---|---|
| `Trust Level` | 11 | `Niveau de confiance` | 19 | 1.73x | Error-state row only (T0 hard gate). Never in normal flow. | Accept |
| `Catalog Baseline` | 16 | `Base de référence du catalogue` | 30 | 1.88x | Key in kernel security summary pane — pane is wider than trust pane. | Live verify |
| `Catalog baseline matches running kernel` | 39 | `La base de référence du catalogue correspond au noyau actif` | 59 | 1.51x | Value column string, not a key. | Accept |
| `BOOT INTEGRITY` | 14 | `INTÉGRITÉ AU DÉMARRAGE` | 22 | 1.57x | Group title — full-width display, no column split. | Accept |
| `FILESYSTEM HARDENING` | 20 | `DURCISSEMENT DU SYSTÈME DE FICHIERS` | 35 | 1.75x | Group title — full-width display. | Accept |
| `Package database` | 16 | `Base de données des paquets` | 27 | 1.69x | Evidence table cell value, not a key. | Accept |
| `Symlink target` | 14 | `Cible du lien symbolique` | 24 | 1.71x | Evidence table cell value, not a key. | Accept |
| `Filesystem identity` | 19 | `Identité du système de fichiers` | 31 | 1.63x | Evidence table cell value, not a key. | Accept |
| `(no data)` | 9 | `(aucune donnée)` | 15 | 1.67x | Fallback — never shown in normal operation. | Accept |

---

## How to Run This Check

1. After editing the .po file, scan all non-multiline msgstr entries:

```
python3 -c "
import re, sys
with open('resources/i18n/umrs-uname/fr_CA.po') as f:
    content = f.read()
pairs = re.findall(r'msgid \"([^\n\"]+)\"\nmsgstr \"([^\n\"]+)\"', content)
for en, fr in pairs:
    if len(en) > 0 and len(fr) / len(en) > 1.50:
        print(f'OVER 150%: {len(fr)/len(en):.2f}x  [{en}] -> [{fr}]')
"
```

2. Run `msgfmt --check resources/i18n/umrs-uname/fr_CA.po` and verify exit 0.

3. Run `make i18n-compile-umrs-uname` and verify clean output.

4. Test in the live TUI on the target system. Focus on:
   - Trust / Evidence tab pinned summary pane — key column widths
   - Kernel Security tab summary pane — "Catalog Baseline" key
   - Any red (unhardened) indicator row — "[ Recommandé : <value> ]" line

---

## Changelog

| Date | Change |
|---|---|
| 2026-03-23 | Checklist created after live TUI test identified key column jamming in Trust / Evidence tab. Shortened Label Trust, Trust Tier, Downgrade Reasons, Evidence Records. Added "Recommended" to umrs-uname domain (from umrs-ui/data_panel.rs). |
