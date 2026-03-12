# Plan: Antora Documentation Restructure

**Date**: 2026-03-11 (originated as pattern-library-taxonomy)
**Updated**: 2026-03-12
**Status**: Phase 0 complete, Phase 1 complete — Phase 2 next
**Author**: tech-writer, senior-tech-writer
**Reviewed by**: Jamie (2026-03-12)
**Vision source**: `.claude/jamies_brain/doc-vision.md`

---

## Decisions (locked)

1. **Remove `security-compliance/`** — empty module, no vision domain maps to it. Delete.
2. **Keep `architecture/`** — standalone module for now; revisit if it doesn't grow.
3. **Glossary gets its own module** — `docs/modules/glossary/`.
4. **Cryptography section** — use `docs/new-stuff/crypto.md` as seed material (FIPS tables,
   PQC, policy tiers, crypto glossary terms).
5. **Pattern taxonomy work** (original plan decisions) preserved as Phase 2 below.
6. **`_scratch` / `_archive` cleanup** — once content is promoted into Antora pages,
   delete the source file from `_scratch` or `_archive`. No stale copies.
7. **No hardening module or subsection** — IMA, kernel lockdown, `/tmp` isolation are
   high-assurance enhancements (vision §9) documented in `deployment/`. They stay where
   they are. OpenSCAP is simply a post-install step (like `dnf update`) — it's already
   in the existing deployment docs and needs no special treatment.
8. **Cryptography** — NOT a standalone module. Stays in `reference/` but promoted from a
   subdirectory to a first-class nav section ("Cryptographic Baseline"). Existing pages in
   `reference/pages/cryptography/` become top-level pages in `reference/pages/`. Seed new
   pages from `docs/new-stuff/crypto.md`. Organize, don't cross-link yet. Future nav:
   - FIPS-Approved Algorithms
   - Key Recommendations
   - Post-Quantum Cryptography
   - Cryptographic Policy Tiers
   - OpenSSL Vendoring Policy
   - CPU Cryptographic Extensions (placeholder — list of hardware extensions + how to
     audit an ELF binary for compiled-in use of available extensions; content TBD)
9. **`umrs-prog-lang.adoc`** — belongs in `devel/`, not `architecture/`. Delete the
   `architecture/` duplicate, keep `devel/` copy, fix xrefs.
10. **`vm-filesystem-layout.md`** and **`ISOLATED-TMP.md`** — remove without merging.
11. **Placeholder pages** — create stub pages for every vision domain gap so Jamie can see
    what's missing and where new content should go. Enables future gap analysis.

---

## Problem Statement

The Antora documentation has grown organically. Content is in wrong modules, foundational
pages are missing, and there is no coherent mapping from documentation domains (defined in
`doc-vision.md`) to Antora module structure. This must be fixed before adding more content,
otherwise every new page deepens the disorder.

---

## Phase 0 — Freeze, Audit, and Triage

**Goal**: Complete inventory of what exists and where it should live. No files move yet.

### 0a. Full page inventory

Map every `.adoc` page in `docs/modules/*/pages/` to its doc-vision domain (§3–§18).
Flag pages that are in the wrong module.

### 0b. Assess `docs/_scratch/` (46 files)

For each file, classify as:
- **Promote** — valuable content that maps to a vision domain; queue for conversion to `.adoc`
- **Delete** — superseded by existing Antora pages or no longer relevant

Known high-value candidates for promotion:

| File | Vision domain | Target module |
|---|---|---|
| `HACAMS.md` | §5 Historical background | ROOT or security-concepts |
| `RTB.md` | §6 Assurance principles, §12 Patterns | security-concepts |
| `HIGH_ASSURANCE_EXTRA.txt` | §6 Assurance principles | security-concepts |
| `RATIONALE_for_HA.adoc` | §4 What UMRS is, §5 History | ROOT or architecture |
| `mls-classified-talk.adoc` | §4 What UMRS is / isn't | ROOT |
| `rhel10-openscap.txt` | §8 Baseline hardening | deployment |
| `notes/terminology.txt` | §17 Glossary | glossary |
| `notes/umrs-concepts.txt` | §6 Security concepts | security-concepts |
| `notes/case-studies.txt` | §24 Outreach | ROOT or standalone |
| `logging-capacity.txt` | §15 Logging | Compare with `logging-audit/logging-capacity.adoc` |
| `log-lifecycle-model.txt` | §15 Logging | Compare with `logging-audit/log-lifecycle-model.adoc` |
| `log-tuning.txt` | §15 Logging | Compare with `logging-audit/log-tuning.adoc` |
| `chain-intro.txt` | §14 Operations | Compare with `operations/chain-intro.adoc` |
| `chain-verify-sign.txt` | §14 Operations | Compare with `operations/chain-verify-sign.adoc` |

Files likely superseded (verify before deleting):
- `kernel-files-TPI.md` → already `architecture/kernel-files-tpi.adoc`
- `nom_parser.md` → already `devel/nom-parser.adoc`
- `rust-must-use-contract.md` → already `devel/rust-must-use-contract.adoc`
- `TW0-NETIF-JUSTIFICATION.md` → already `deployment/dual-network-interface.adoc`
- `aide-README.md` → already `operations/aide-README.adoc`
- `umrs-signing-README.md` → already `operations/umrs-signing-README.adoc`

Remaining files need individual assessment (SELinux policy notes, CIL files, tool-specific
notes, unicode references, etc.).

### 0c. Assess `docs/modules/deployment/pages/_archive/` (5 files)

| File | Compare with | Likely disposition |
|---|---|---|
| `filesystem-layout.md` | Active `filesystem-layout.adoc` | Delete if superseded |
| `ISOLATED-TMP.md` | Active `tmp-security.adoc` | Delete if superseded |
| `umrs-tmp-filesystems-README.md` | Active `tmp-security.adoc` | Delete if superseded |
| `rhel10-README.md` | Active `rhel/` pages | Delete if superseded |
| `vm-filesystem-layout.md` | Active `filesystem-layout.adoc` | Delete if superseded |

### 0d. Assess `docs/new-stuff/crypto.md`

Rich FIPS-aligned crypto tables with PQC (ML-KEM, ML-DSA, SLH-DSA), 8 algorithm categories,
policy tiers, and a crypto glossary. This becomes the seed for the cryptography section
(vision §10). The crypto glossary terms also feed the glossary module (vision §17).
Delete `docs/new-stuff/crypto.md` after content is promoted.

### 0e. Assess top-level repo files

| File | Content summary | Disposition |
|---|---|---|
| `README.md` | Strong "what is high assurance" narrative, HACAMS history, real-world examples (military, nuclear, aviation, medical), key features table, HA vs traditional comparison | **Extract into** ROOT intro pages (§3, §4, §5). README stays at repo root but Antora pages become the authoritative version. |
| `UMRS-PROJECT.md` | Project identity, MLS label hierarchy table, CUI explanation, assurance philosophy, personal note | **Extract into** ROOT "What is UMRS" page (§3, §4). The MLS label table and CUI framing are particularly valuable. |
| `UMRS-PLAN.md` | Milestone roadmap (M1–M3). High-level feature backlog. | **Reference only** — useful for project context. Not Antora content. Keep as-is or move to `.claude/` as planning context. |

### 0f. Output

A **migration manifest** listing:
- Every page with its current location and target module
- Every `_scratch` file with promote/delete classification
- Every `_archive` file with its disposition
- Root-level files with their content extraction targets
- Gap list: vision domains with no existing content at all

---

## Phase 1 — Structural Foundation

**Goal**: Move misplaced content to correct modules, create missing module skeletons,
remove dead modules. No new prose writing — just moves, nav updates, and xref fixes.

### 1a. Delete `security-compliance/`

Empty module. Remove directory and any nav references.

### 1b. Create `glossary/` module

- `docs/modules/glossary/pages/index.adoc` — skeleton
- `docs/modules/glossary/nav.adoc`
- Seed initial terms from `_scratch/notes/terminology.txt` and crypto glossary
  in `docs/new-stuff/crypto.md`
- Register in `antora.yml` nav list

### 1c. Execute content migrations from manifest

Move pages that are in the wrong module to their correct home. Known candidates
(to be confirmed by Phase 0 audit):
- `deployment/structured-logging.adoc` → `logging-audit/` or `operations/`?
- `deployment/how-to-structure-log.adoc` → `logging-audit/`?
- Hardening content (IMA, kernel lockdown, tmp, OpenSCAP) — group coherently
  within `deployment/` as a "hardening" subsection, or promote to own module?

### 1d. Strengthen ROOT module

- Add "What is UMRS" page — draw from `UMRS-PROJECT.md` and `README.md`
- Add "What is High Assurance" page — draw from `README.md` high-assurance narrative
- Improve `introduction.adoc` — connect to new pages
- Add AI transparency page skeleton (vision §18) — content written in Phase 3

### 1e. Delete promoted `_scratch` / `_archive` files

After content is migrated into Antora pages, delete the source files.
No stale copies left behind.

### 1f. Nav and xref repair

- Update every `nav.adoc` affected by moves
- Fix all broken xrefs
- `make docs` must pass clean

---

## Phase 2 — Patterns Taxonomy (self-contained, original plan)

**Goal**: Label, tag, and cross-link the patterns module. No structural changes
outside `patterns/`.

### Preserved decisions from original plan (locked 2026-03-12)

1. Single kind: "Implementation Pattern" with sub-labels (*architectural* / *technique*)
2. High-assurance provenance tag via NOTE admonitions
3. Two-zone page template: "Why This Pattern Exists" + "Implementation"
4. "Concept basis" column in index table linking to `security-concepts/`
5. "Process Discipline" kept as a kind (one member: Supply Chain Hygiene)

### 2a. Update Pattern Reference Table in `index.adoc`

Add "Sub-group", "Provenance", and "Concept basis" columns.

### 2b. Add provenance badges to high-assurance pattern pages

NOTE admonitions with control citations:
- TPI → `NIST 800-53 SI-10`
- TOCTOU Safety → `NSA RTB RAIN`
- Provenance Verification → `NIST 800-53 SI-7`
- Non-Bypassability → `NSA RTB RAIN`
- Fail-Closed → `NSA RTB (Fail Secure)`
- Zeroize → `NIST 800-53 SC-28`

### 2c. Add two-zone structure markers to pattern pages

```asciidoc
== Why This Pattern Exists
// Zone 1: concept — threat, consequence, control basis

== The Pattern
// Zone 2: implementation — invariant, rule, code recipe

== In the UMRS Codebase
// Zone 2 continued: types, traits, file paths

== When to Apply
// Trigger conditions from Architectural Review Triggers
```

### 2d. Reorganize `patterns/nav.adoc`

```
* Architectural Patterns
** Fail-Closed
** Loud Failure
** Non-Bypassability / RAIN
** Error Information Discipline
** Sealed Evidence Cache / SEC
* Coding Techniques
** Two-Path Independence (TPI)
** TOCTOU Safety
** Provenance Verification
** Secure Arithmetic
** Bounds-Safe Indexing
** Zeroize Sensitive Data
** Constant-Time Comparison
* Process
** Supply Chain Hygiene
```

### 2e. Cross-linking audit

Audit 5 existing `security-concepts/` pages against all pattern pages.
Populate "Concept basis" cells. Report gaps.

---

## Phase 3 — Fill Structural Gaps

**Goal**: Write the missing foundational content identified in Phase 0.

### 3a. Cryptography section

Convert `docs/new-stuff/crypto.md` to AsciiDoc pages. Location TBD (expand
`reference/cryptography/` or standalone module — see open question #2). Include:
- FIPS-aligned algorithm tables (8 categories)
- Policy tiers (preferred / baseline / legacy / disallowed)
- PQC landscape (ML-KEM, ML-DSA, SLH-DSA — FIPS 203/204/205)
- UMRS crypto usage cross-references (SEC sealing, IMA, dm-crypt)
- Control mappings (SC-12, SC-13, IA-7)
- Delete `docs/new-stuff/crypto.md` after promotion

### 3b. Glossary — initial population

Seed terms from:
- `_scratch/notes/terminology.txt` (delete after promotion)
- `docs/new-stuff/crypto.md` glossary section
- Security concepts vocabulary (assurance, provenance, ground truth, reference monitor, etc.)
- SELinux/MLS terms used across docs

### 3c. ROOT content — "What is UMRS" / "What is High Assurance"

Full narrative pages drawing from `README.md`, `UMRS-PROJECT.md`, `_scratch/HACAMS.md`.
Cross-reference to security-concepts for deeper treatment.
Delete `_scratch/HACAMS.md` after content is promoted.

### 3d. AI transparency page

Explain AI agent roles in the project per vision §18.

### 3e. Historical context

Sprinkle into security-concepts pages where it supports understanding (vision §5).
Draw from `_scratch/HACAMS.md`, `README.md` HACAMS section, `_scratch/RTB.md`.
Delete source files after promotion.

### 3f. High-assurance enhancements coherence

Ensure IMA/EVM, kernel lockdown, `/tmp` isolation, AIDE, OpenSCAP content is
grouped coherently — either as a deployment subsection or its own module
(see open question #1).

---

## Phase 4 — Curation and Cross-Linking

**Goal**: Clean up reference module, wire everything together, validate the
"where does new content go?" workflow.

### 4a. Reference module curation

Review every page in `reference/`. Move narrative content to its proper module.
Keep only true reference material (tables, specs, detailed type docs).

### 4b. Cross-reference audit

Every module should link to related modules where appropriate:
- devel ↔ patterns
- security-concepts ↔ patterns
- deployment ↔ operations
- logging-audit ↔ operations
- glossary ← all modules

### 4c. Final `_scratch` cleanup

Process any remaining `_scratch` files per Phase 0 classifications.
Delete promoted files, flag remaining files for Jamie's decision.
Goal: `_scratch/` should be empty or nearly empty when done.

### 4d. Validate the vision §21 goal

Test with 3–5 real topics: "Here is a new topic — determine where it fits."
The documentation architecture should make placement obvious.

### 4e. `make docs` final validation

Full clean build. Zero broken xrefs. All nav files consistent.

---

## Source Material Inventory

| Source | Location | Phase used | Post-use action |
|---|---|---|---|
| Doc vision | `.claude/jamies_brain/doc-vision.md` | All | Keep (authoritative) |
| Crypto tables + PQC + glossary | `docs/new-stuff/crypto.md` | 3a, 3b | Delete |
| README.md | repo root | 1d, 3c | Keep (extract, don't move) |
| UMRS-PROJECT.md | repo root | 1d, 3c | Keep (extract, don't move) |
| UMRS-PLAN.md | repo root | Reference only | Keep |
| HACAMS.md | `docs/_scratch/` | 3c, 3e | Delete |
| RTB.md | `docs/_scratch/` | 3e | Delete |
| terminology.txt | `docs/_scratch/notes/` | 3b | Delete |
| umrs-concepts.txt | `docs/_scratch/notes/` | 3b, 3e | Delete |
| 46 total _scratch files | `docs/_scratch/` | 0b, 4c | Delete after promotion |
| 5 _archive files | `deployment/pages/_archive/` | 0c | Delete after verification |

---

## Open Questions

1. **Hardening as module or subsection?** — IMA, kernel lockdown, OpenSCAP, `/tmp` isolation.
   Currently scattered in `deployment/`. Promote to own module (`hardening/`) or keep as a
   coherent subsection within `deployment/`? (vision §8-§9)
2. **Cryptography module vs reference subdir?** — `docs/new-stuff/crypto.md` is rich enough
   for its own module. But is it better as `reference/cryptography/` expanded, or standalone
   `cryptography/` module? (vision §10)
3. **`architecture/` long-term** — if it stays small, consider merging into ROOT or devel.
   Revisit after Phase 1.

---

## Resolved Questions (from original pattern-taxonomy plan)

| # | Question | Decision |
|---|---|---|
| 1 | Kind label format? | NOTE admonitions — theme-independent |
| 2 | First concept pages? | Deferred — audit existing `security-concepts/` first |
| 3 | Blog conversion? | Not a near-term priority; taxonomy supports it when ready |
| 4 | Non-Bypassability split? | Leave as Implementation Pattern (architectural); future concept page links to it |
| 5 | `security-concepts/` coverage? | Deferred — Jamie will revisit with a fuller list |
| 6 | Remove `security-compliance/`? | Yes — delete empty module |
| 7 | Glossary location? | Own module: `docs/modules/glossary/` |
| 8 | `_scratch`/`_archive` after use? | Delete — no stale copies |
