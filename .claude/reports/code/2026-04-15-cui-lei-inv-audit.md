# CUI//LEI/INV Anti-Pattern Audit

Audit date: 2026-04-15
Depth: in-depth (exhaustive grep across full repo)
Scope: All files in `/DEVELOPMENT/umrs-project/` — source code, documentation, config/data, test fixtures, prototypes

---

## Background

`CUI//LEI/INV` incorrectly implies INV (Investigation) is a subcategory of LEI (General Law Enforcement). Both are independent categories under the NARA "Law Enforcement" index group. The index group name never appears in a banner. The correct forms are:

- `CUI//INV` — if the document contains only investigation data
- `CUI//INV/LEI` — alphabetized, if the document genuinely contains both General Law Enforcement and Investigation data

Rule source: `.claude/rules/cui-taxonomy-and-rules.md`, "Common Mistakes" section.

**Known-fixed occurrences (excluded from findings):**
- `components/rusty-gadgets/umrs-label/src/.../dirlist.rs` area (confirmed fixed)
- `components/rusty-gadgets/selinux/SETRANS.md` (confirmed fixed)

---

## Findings

### 1. Source Code (.rs)

No `.rs` files contain `CUI//LEI/INV` or `LEI/INV` as a marking string after the two confirmed fixes.

**Source code findings: 0**

---

### 2. Documentation (.adoc, .md, .page, .txt)

---

#### Finding D-01
**File:** `docs/sage/blogs/blog-cui-sign-lock.adoc`
**Lines:** 86, 90, 111, 149
**Offending string:** `LEI/INV` (used as a shorthand category descriptor, e.g., "Law Enforcement Investigative (LEI/INV)")

**Context:** The blog uses `LEI/INV` as a compound shorthand to describe the Investigation category colloquially (e.g., "Law Enforcement Investigative (LEI/INV) is not 'more sensitive' than..."). It does not appear as a banner marking string in these lines, but the shorthand reinforces the incorrect subordination of INV under LEI in the reader's mental model.

**Recommended replacement:**
- Line 86: `Law Enforcement Investigative (INV)` — drop the `LEI/` prefix entirely; INV is the standalone abbreviation
- Line 90: `INV, AGR/AMNT` (replace `LEI/INV`)
- Line 111: `INV:` (replace `LEI/INV:`)
- Line 149: `INV and AGR/AMNT` (replace `LEI/INV and AGR/AMNT`)

**Remediation owner:** tech-writer

---

#### Finding D-02
**File:** `docs/sage/blogs/blog-cui-sign-lock.md`
**Lines:** 119, 123, 144, 182
**Offending string:** `LEI/INV` (same shorthand usage as D-01 — this is the Markdown source of the same blog)

**Recommended replacement:** Same as D-01 (both files must be updated in sync).

**Remediation owner:** tech-writer

---

#### Finding D-03
**File:** `help/umrs-ls/C/concept-grouping.page`
**Line:** 50
**Offending string:** `CUI//LEI/INV` (used as a rendered example in user-facing help text)

**Context:** The example demonstrates what an MCS marking looks like in the TUI. This is a concrete banner marking string shown to operators — the incorrect form will train users to expect and produce the anti-pattern.

**Recommended replacement:** Replace `CUI//LEI/INV` with `CUI//INV` in the example. INV alone is the correct standalone marking for investigation data.

**Remediation owner:** tech-writer

---

#### Finding D-04
**File:** `help/umrs-ls/fr_CA/concept-grouping.page`
**Line:** 50
**Offending string:** `CUI//LEI/INV` (French locale help page, same example)

**Recommended replacement:** Replace `CUI//LEI/INV` with `CUI//INV`.

**Remediation owner:** tech-writer

---

#### Finding D-05
**File:** `docs/_scratch/unicode_symbols.txt`
**Line:** 34
**Offending string:** `CUI//LEI/INV` (used in a shell echo command demonstrating TUI color rendering)

**Context:** Scratch file, not published. The example will propagate the incorrect marking if copied as a template.

**Recommended replacement:** Replace `CUI//LEI/INV` with `CUI//INV`.

**Remediation owner:** tech-writer

---

#### Finding D-06
**File:** `docs/_scratch/HIGH_ASSURANCE_EXTRA.txt`
**Line:** 23
**Offending string:** `CUI//LEI/INV`

**Recommended replacement:** Replace `CUI//LEI/INV` with `CUI//INV`.

**Remediation owner:** tech-writer

---

*Note: `docs/_scratch/basic-versus-special.txt` line 42 contains `CUI//LEI/INV` but uses it explicitly as a labeled incorrect example ("Incorrect: CUI//LEI/INV"). This is intentional documentation of the anti-pattern and is NOT a finding.*

---

### 3. Config / Data (.json, .conf)

---

#### Finding C-01
**File:** `components/rusty-gadgets/umrs-label/config/us/labels-backup.json`
**Line:** 673
**Offending string:** `"CUI//LEI/INV"` (JSON object key; also `"parent_group": "LEI"` at line ~677)

**Context:** This is a backup of the US CUI label catalog. The key `"CUI//LEI/INV"` encodes the incorrect subcategory relationship directly into the catalog structure. The `parent_group: "LEI"` field compounds the error by asserting subordination.

**Recommended replacement:** Rename the key to `"CUI//INV"`. Remove or nullify the `parent_group` field (or set it to the NARA index group name for display purposes only, clearly marked as non-marking). This is a protected JSON file — change requires explicit user instruction.

**Remediation owner:** coder (requires Jamie approval per protected-files rule)

---

#### Finding C-02
**File:** `components/rusty-gadgets/libs/umrs-selinux/config/setrans.conf`
**Line:** 136
**Offending string:** `s0:c90,c99   = CUI//LEI/INV       #  Investigation`

**Context:** This is the active setrans.conf used by the production codebase under `rusty-gadgets`. The MCS translation table maps `s0:c90,c99` to the incorrect banner string. Any operator viewing a file labeled at this MCS tuple will see `CUI//LEI/INV` — a marking they may copy onto physical documents or enter into other systems.

**Recommended replacement:** `s0:c90,c99   = CUI//INV       #  Investigation`

This is a protected file — change requires explicit user instruction.

**Remediation owner:** coder (requires Jamie approval per protected-files rule)

---

#### Finding C-03
**File:** `components/rust-prototypes/umrs-cui/data/TARGETED-setrans.conf`
**Line:** 133
**Offending string:** `s0:c90,c99   = CUI//LEI/INV       #  Investigation`

**Recommended replacement:** `s0:c90,c99   = CUI//INV       #  Investigation`

**Remediation owner:** coder

---

#### Finding C-04
**File:** `components/rust-prototypes/umrs-cui/data/MLS-setrans.conf`
**Line:** 166
**Offending string:** `s1:c90,c99   = CUI//LEI/INV       #  Investigation`

**Recommended replacement:** `s1:c90,c99   = CUI//INV       #  Investigation`

**Remediation owner:** coder

---

#### Finding C-05
**File:** `components/rust-prototypes/umrs-cui/data/us/US-CUI-LABELS.json`
**Line:** 285
**Offending string:** `"CUI//LEI/INV"`

**Recommended replacement:** Key → `"CUI//INV"`. Remove or recast `parent_group` field.

**Remediation owner:** coder

---

#### Finding C-06
**File:** `components/rust-prototypes/umrs-cui/cui-labels.json`
**Line:** 260
**Offending string:** `"CUI//LEI/INV"`

**Recommended replacement:** Key → `"CUI//INV"`.

**Remediation owner:** coder

---

#### Finding C-07
**File:** `components/rust-prototypes/vaultmgr/cui-labels.json`
**Line:** 260
**Offending string:** `"CUI//LEI/INV"`

**Recommended replacement:** Key → `"CUI//INV"`.

**Remediation owner:** coder

---

#### Finding C-08
**File:** `components/rust-prototypes/mcs-setrans/setrans.conf`
**Line:** 136
**Offending string:** `s0:c90,c99   = CUI//LEI/INV       #  Investigation`

**Recommended replacement:** `s0:c90,c99   = CUI//INV       #  Investigation`

**Remediation owner:** coder

---

#### Finding C-09
**File:** `components/rust-prototypes/mcs-setrans/examples/data/setrans.conf`
**Line:** 136
**Offending string:** `s0:c90,c99   = CUI//LEI/INV       #  Investigation`

**Recommended replacement:** `s0:c90,c99   = CUI//INV       #  Investigation`

**Remediation owner:** coder

---

#### Finding C-10
**File:** `components/rust-prototypes/cui-labels/data/json/US-CUI-LABELS.json`
**Line:** 470
**Offending string:** `"CUI//LEI/INV"`

**Recommended replacement:** Key → `"CUI//INV"`.

**Remediation owner:** coder

---

#### Finding C-11
**File:** `components/rust-prototypes/cui-labels/data/BASE-FULL-setrans.conf`
**Line:** 133
**Offending string:** `s0:c90,c99   = CUI//LEI/INV       #  Investigation`

**Recommended replacement:** `s0:c90,c99   = CUI//INV       #  Investigation`

**Remediation owner:** coder

---

#### Finding C-12
**File:** `components/rust-prototypes/cui-labels/cui-labels.json`
**Line:** 260
**Offending string:** `"CUI//LEI/INV"`

**Recommended replacement:** Key → `"CUI//INV"`.

**Remediation owner:** coder

---

#### Finding C-13
**File:** `components/platforms/rhel10/targeted/setrans.conf-TARGETED`
**Line:** 133
**Offending string:** `s0:c90,c99   = CUI//LEI/INV       #  Investigation`

**Recommended replacement:** `s0:c90,c99   = CUI//INV       #  Investigation`

**Remediation owner:** coder

---

#### Finding C-14
**File:** `components/platforms/rhel10/mls/setrans.conf-MLS`
**Line:** 208
**Offending string:** `s3:c90,c99  = "CUI//LEI/INV"    # Investigation`

**Recommended replacement:** `s3:c90,c99  = "CUI//INV"    # Investigation`

**Remediation owner:** coder

---

#### Finding C-15
**File:** `components/platforms/rhel10/good-labels.json`
**Line:** 263
**Offending string:** `"CUI//LEI/INV"`

**Recommended replacement:** Key → `"CUI//INV"`.

**Remediation owner:** coder

---

#### Finding C-16
**File:** `components/platforms/rhel10/CUI-LABELS.json`
**Line:** 470
**Offending string:** `"CUI//LEI/INV"`

**Recommended replacement:** Key → `"CUI//INV"`.

**Remediation owner:** coder

---

#### Finding C-17
**File:** `components/platforms/rhel10/CLEANUP/CUI-LABELS.json`
**Line:** 283
**Offending string:** `"CUI//LEI/INV"`

**Recommended replacement:** Key → `"CUI//INV"`.

**Remediation owner:** coder

---

### 4. Test Fixtures

No dedicated test fixture files (under `tests/` or `fixtures/`) were found to contain the pattern. The prototype JSON files above (C-05 through C-12) function as data fixtures in prototype crates and are covered under section 3.

**Test fixture findings: 0 additional**

---

### 5. Excluded: Reference and Memory Files

The following contain `CUI//LEI/INV` but are excluded from actionable findings. They are historical records, audit evidence, reference corpus material, or correctly-labeled anti-pattern documentation:

| File | Reason excluded |
|---|---|
| `.claude/rules/cui-taxonomy-and-rules.md:175` | Authoritative anti-pattern rule — intentional |
| `.claude/reports/code/2026-03-30-us-cui-labels-audit.md` | Prior audit report — historical record |
| `.claude/reports/2026-03-23-canadian-protected-category-requirements.md` | Prior report — historical record |
| `.claude/references/cui-registry/reconciliation.json` | Reference corpus — read-only by rule |
| `.claude/references/cui-registry/index.json` | Reference corpus — read-only by rule |
| `.claude/references/cui-registry/categories/INV.json` | Reference corpus — read-only by rule |
| `.claude/references/cui-legal-corpus/cui-banner-marking-rules.md` | Reference corpus — read-only by rule |
| `.claude/references/sage-outreach-corpus/content-strategy-operational-spec.md` | Checklist item correctly showing the pattern for validation — review needed but not an audit finding |
| `.claude/jamies_brain/cui_cleanup/session-notes.md` | Notes correctly documenting the fix direction |
| `.claude/jamies_brain/new_material_review.txt` | Research notes |
| `.claude/jamies_brain/archive/next-big-steps.md` | Archive |
| `.claude/jamies_brain/archive/nara-info.txt` | Archive |
| `.claude/logs/task-log.md` | Task log — historical record |
| `.claude/logs/CHANGELOG.md` | Changelog — historical record |
| `.claude/agent-memory/sage/project-next-big-steps.md` | Agent memory — stale note |
| `.claude/agent-memory/researcher/sp800-60-familiarization.md` | Agent memory |
| `.claude/agent-memory/henri/orientation-notes.md` | Agent memory |
| `.claude/agent-memory/henri/finding-canadian-program-name.md` | Agent memory |
| `docs/sage/reviews/2026-03-19-blog-cui-sign-lock.md` | Prior review report — historical |

**Note on `.claude/references/sage-outreach-corpus/content-strategy-operational-spec.md:219`:** The checklist item reads `- [ ] CUI marking syntax correct (CUI//LEI/INV)`. This is unchecked and appears to use the incorrect form as the reference standard for validation. This should be corrected to `CUI//INV` so the checklist validates against the correct pattern — but as a reference file it is read-only. Flag for Jamie.

---

## Gap Analysis Summary

```
Files reviewed: full repo scan
Total actionable findings: 21

  Source code (.rs):        0
  Documentation:            6  (D-01 through D-06; D-01 and D-02 are the same blog in two formats)
  Config / data:           17  (C-01 through C-17)
  Test fixtures:            0  additional

  Of the 17 config/data findings:
    - 2 are in the active rusty-gadgets codebase (C-01, C-02) — highest priority
    - 5 are in components/platforms/ reference configs (C-13 through C-17)
    - 10 are in rust-prototypes/ (C-03 through C-12)

  Protected files requiring Jamie approval before change: C-01, C-02

Priority order for remediation:
  1. C-02 — active setrans.conf; operators see this marking today
  2. C-01 — active labels-backup.json; also carries incorrect parent_group field
  3. D-03, D-04 — user-facing help text trains operators on the incorrect form
  4. D-01, D-02 — blog content (public-facing)
  5. C-03 through C-17 — prototypes and platform configs
  6. D-05, D-06 — scratch files
```

**Blog "LEI/INV" shorthand note:** The blog (D-01, D-02) uses `LEI/INV` as a parenthetical shorthand abbreviation for the Investigation category, not as a formatted banner string. The notation still implies subordination and should be corrected to the standalone `INV` abbreviation with the full category name "Law Enforcement — Investigation" on first use if disambiguation from General Law Enforcement is needed.

**No occurrences found** of `CUI//INV/LEI` or any other variant that correctly combines both categories — all instances use the incorrect `CUI//LEI/INV` form.
