# Senior Tech Writer — Persistent Memory

## Cross-Team Channel
`.claude/agent-memory/cross-team/notes.md` — shared with all agents.
Read at session start for entries addressed to senior-tech-writer.
Write here to notify rust-developer or security-engineer of doc-driven questions.

## Project: UMRS Documentation (Antora)

**Docs root**: `docs/modules/`
**Modules**: ROOT, architecture, security-concepts, devel, deployment, operations, logging-audit, reference, umrs-tools, patterns

**antora.yml** registers all 10 module nav files — updated 2026-03-11.
**security-concepts** and **logging-audit** are now registered; both have nav.adoc and index.adoc.

---

## Structural Decisions (confirmed by Jamie, 2026-03-10)

- `umrs-tools/` stays as a separate module; wire into main nav and cross-reference from operations
- `admin/` is merged into `operations/`; admin/ pages move, module disappears
- `release-notes.adoc` is a placeholder for milestones/features — not tied to codebase versioning
- Prototype crates (umrs-logspace, umrs-state, vaultmgr) get placeholder pages only — no content until stable
- Use cases are user-story format: "As a developer, I want to [goal] — here is how"
- "Five Eyes" section = multi-national classification interoperability architecture page (US/UK/CA/AU/NZ)
- Project origin narrative: third person, factual — no first-person authorship voice

## Structural Decisions (confirmed by Jamie, 2026-03-11) — Reorganization

- `security/` module retired; all pages moved to `architecture/` or `devel/`
- `historical/` module retired; all pages moved to `architecture/`
- `architecture/` is now the sole owner of all design rationale, history, and security model content
- `security-model.adoc` in ROOT deleted (was a redirect stub with no content)
- `_scratch/pdf-security-model.adoc` deleted (approved)
- `selinux-registry.txt` is a byte-for-byte duplicate of `umrs-mls-registry.txt` — flagged to Jamie

---

## Architecture Module — Complete Page List (2026-03-11)

All pages now in `docs/modules/architecture/pages/`:

| Page | Source |
|---|---|
| index.adoc | STW-2 (written 2026-03-10) |
| five-eyes-interop.adoc | STW-3 (written 2026-03-10) |
| mls-history.adoc | from historical/ |
| selinux-history.adoc | from historical/ |
| trusted-path-orange.adoc | from historical/ |
| HACAMS.adoc | from historical/ |
| ring-based-security.adoc | from historical/ |
| ibm-zos-os390.adoc | from historical/ |
| microsoft-nt-orange.adoc | from historical/ |
| one-way-hashes.adoc | from historical/ |
| openssl-no-vendoring.adoc | from historical/ |
| umrs-prog-lang.adoc | from devel/ (copy; devel/ retains its copy) |
| reference-monitor.adoc | from security/ (converted S-1) |
| rtb-vnssa.adoc | from security/ (converted S-1) |
| kernel-files-tpi.adoc | from security/ (converted S-1) |
| library-model.adoc | from security/ (converted S-1) |
| rationale-strongly-typed.adoc | from security/ (converted S-1, major expansion) |
| mls-label-model.adoc | Phase 2 A-1 (new) |
| integrity-and-provenance.adoc | Phase 2 A-2 (new) |
| case-studies.adoc | Phase 2 A-3 (new) |
| cui-structure.adoc | Phase 2 A-4 (new) |
| truth-concepts.adoc | Phase 2 A-5 (stub) |

---

## Reference Module — New Pages (2026-03-11)

All converted from `.txt` or `.md` source files:
- `mls-colors.adoc` — from mls-COLORS.md
- `rhel-selinux-users.adoc` — from RHEL_SELINUX_USERS.md
- `setrans-technical.adoc` — from SETRANS.md
- `cui-category-abbreviations.adoc` — from cui-category-abbreviations.txt
- `example-setrans-conf.adoc` — from example-setrans-conf.txt
- `fips-cryptography-cheat-sheet.adoc` — from fips-cryptography-cheat-sheet.txt
- `umrs-mls-registry.adoc` — from umrs-mls-registry.txt
- `cui-descriptions.adoc` — from cui-descriiptions.txt (fixed typo in filename)
- `key-recommendation-list.adoc` — from key-recommendation-list.md

Original .txt and .md files remain in place (not deleted per policy).

---

## Key Structural Facts

- `.md` files in `pages/` dirs do NOT render in Antora without a plugin — convert to `.adoc`
- `.txt` files in `pages/` dirs are inert — convert to `.adoc`
- `antora.yml` at `docs/antora.yml` is the single component descriptor — no module-level antora.yml files
- `security/` and `historical/` were NOT registered in antora.yml — no removal needed
- `umrs-tools/` was already wired into ROOT/nav.adoc before 2026-03-11 session

### File locations after Phase 1 reorganization (canonical, 2026-03-11)

| Files | Module | Subdirectory |
|---|---|---|
| rtb-vnssa, integrity-and-provenance, truth-concepts, reference-monitor | security-concepts | pages/ |
| selinux-history, five-eyes-interop, HACAMS, ibm-zos-os390, microsoft-nt-orange, ring-based-security, mls-history, one-way-hashes, trusted-path-orange | architecture | pages/history/ |
| category_set, security_type, role, user, context, sensitivity, booleans, secolor, mcs, mls-colors, rhel-selinux-users, setrans-technical, example-setrans-conf, umrs-mls-registry | reference | pages/selinux/ |
| openssl-no-vendoring, key-recommendation-list, fips-cryptography-cheat-sheet | reference | pages/cryptography/ |
| cui-category-abbreviations, cui-descriptions | reference | pages/cui/ |
| logging-capacity, log-lifecycle-model, log-tuning | logging-audit | pages/ |
| rhel10-installation, rhel10-openscap, rhel10-packages, rhel10-setrans, rhel10-directory-structure | deployment | pages/rhel/ |
| ubuntu.adoc | deployment | pages/ubuntu/ |
| git-commit-signing | devel | pages/ |
| umrs-tooling, umrs-tool-shred, umrs-tool-shred-usage | umrs-tools | pages/ |

---

## Primary Source Documents for Introduction and Architecture Content

- `README.md` — defines high-assurance engineering, HACAMS lineage, real-world examples
- `UMRS-PROJECT.md` — authoritative project description, MLS label hierarchy, CUI handling

---

## Terminology Decisions (confirmed by Jamie, 2026-03-12)
See `.claude/approved_terminology.md` for the full list and translator cross-references.

Key decisions:
- `security context` — PREFERRED. Full five-part label: `user:role:type:sensitivity_level:category_set`
- `security label` — COLLOQUIAL. Do not use as a primary term; it means "security context" generically
- `sensitivity level` — PREFERRED (the s0–s15 hierarchical component)
- `sensitivity label` — NON-PREFERRED colloquial form; translator vocabulary corrected
- `HA` abbreviation — NEVER. "HA" = high-availability; always spell out "high-assurance"
- "HA-Sign" — correct product name; the "HA-" is part of the name, not an abbreviation

CategorySet glossary entry marked TODO for expansion when SELinux reference pages are more complete.

---

## MANDATORY: Build Verification Rule
- **`make docs` must pass cleanly** before any docs/ work is considered done. No exceptions.
- Run `make docs 2>&1` from the repo root and verify zero xref errors in the output.
- When moving pages into subdirectories, update ALL xrefs across ALL modules that reference the moved pages — not just the nav files.
- Cross-module xrefs (e.g., `reference:context.adoc`) must be updated when the target page moves to a subdirectory (e.g., `reference:selinux/context.adoc`).

## Writing Mode Defaults
- Architecture Mode for explanatory content
- STE Mode for procedures
- Load rules file before writing (`.claude/architecture_mode.md`, `.claude/ste_mode.md`)

---

## Phase 2 Reorganization (2026-03-11)

File locations changed from Phase 1:

| Pages | From | To |
|---|---|---|
| security-model.adoc | ROOT/pages/ | security-concepts/pages/ |
| case-studies.adoc, mls-classified-talk.adoc | architecture/pages/ | architecture/pages/history/ |
| structured-logging.adoc, how-to-structure-log.adoc | operations/pages/ | deployment/pages/ |
| auditing-noise.adoc | operations/pages/ | logging-audit/pages/ |
| TW0-NETIF-JUSTIFICATION.adoc | deployment/pages/ | deployment/pages/dual-network-interface.adoc |

truth-concepts.adoc content merged into reference-monitor.adoc (new section). truth-concepts.adoc is now a redirect stub — keep, do not edit.

IMPORTANT: `architecture:rationale.adoc` does NOT exist. Correct filename is `architecture:rationale-strongly-typed.adoc`.

Operations module NO LONGER has a Logging section — all logging content is in deployment/ (setup) or logging-audit/ (operations).

---

## Patterns Module — Phase 2 Taxonomy (2026-03-12)

Phase 2 complete. All 16 standard pattern pages updated (not pattern-os-detection.adoc):

**2c (Two-zone structure)**: All pages renamed `== Overview` → `== Why This Pattern Exists`
with `// Zone 1:` comment; `== The Threat` demoted to `===`; `// Zone 2:` comments added
to `== The Pattern` and `== In the UMRS Codebase`.

**2b (Provenance badges)**: NOTE admonitions added to TPI, TOCTOU, Provenance, Non-Bypassability,
Fail-Closed, Zeroize citing the mandating framework.

**2d (Nav)**: Reorganized into Architectural Patterns / Coding Techniques / Observability / Process / Deep Dives.

**2a (Index table)**: Added Sub-group, Concept basis columns; renamed Status column.

**2e (Cross-links)**: Security-concepts xrefs added to 8 pattern pages' See Also sections.
Reverse links added to reference-monitor.adoc, integrity-and-provenance.adoc, rtb-vnssa.adoc.

Nav groups: Architectural (Fail-Closed, Loud Failure, Non-Bypassability, Error Discipline, SEC, Layered Separation)
Coding Techniques (TPI, TOCTOU, Provenance, Secure Arithmetic, Bounds-Safe, Zeroize, Constant-Time)
Observability (Execution Measurement, Audit Cards) | Process (Supply Chain Hygiene) | Deep Dives (OS Detection)

## OS Detection Pipeline Docs (2026-03-11)
- `patterns/pages/pattern-os-detection.adoc` — concept/architecture. Multi-audience.
- `devel/pages/os-detection-deep-dive.adoc` — engineer deep dive, full code references.
- `(device, inode)` is the canonical TOCTOU defense term. FIPS gate: sha2 not validated; ceiling T3.

## Phase 3 Content — Filled Structural Gaps (2026-03-12)

ROOT stubs populated (all were placeholder skeletons):
- `ROOT/pages/what-is-high-assurance.adoc` — full explanation from README.md
- `ROOT/pages/what-is-umrs.adoc` — full explanation from UMRS-PROJECT.md
- `ROOT/pages/ai-transparency.adoc` — AI agent roles, review requirements, auditor guidance

Glossary populated:
- `glossary/pages/index.adoc` — 25+ definitions across 3 groups (Assurance/Integrity, SELinux/MLS, Cryptography)
- Sources: terminology.txt, crypto.md glossary, in-codebase SELinux concepts

Crypto reference stubs filled:
- `reference/pages/crypto-post-quantum.adoc` — ML-KEM/ML-DSA/SLH-DSA tables, migration approach
- `reference/pages/crypto-policy-tiers.adoc` — 4-tier framework (Preferred/Approved/Baseline/Disallowed), 8 algorithm categories
- `crypto-cpu-extensions.adoc` remains stub — no source material; requires research
- `fips-cryptography-cheat-sheet.adoc` and `key-recommendation-list.adoc` were already complete

3f (high-assurance enhancements coherence): deployment/nav.adoc already has "Assurance Enhancements" section — no changes needed.

## Phase 4 Complete — Antora Doc Restructure (2026-03-12)

**Plan archived**: `.claude/plans/completed/antora-doc-restructure.md`
**Vision archived**: `.claude/plans/completed/doc-vision.md`

4a — SELinux reference pages rewritten from Markdown to proper AsciiDoc using source `///` comments:
- `reference/selinux/sensitivity.adoc` — SensitivityLevel, Bell-LaPadula, UMRS type model
- `reference/selinux/category_set.adoc` — CategorySet bitmask, dominance math, kernel ebitmap deviation
- `reference/selinux/user.adoc` — SelinuxUser, naming conventions, POSIX vs SELinux identity
- `reference/selinux/role.adoc` — SelinuxRole, RBAC layer, domain authorization
- `reference/selinux/security_type.adoc` — SelinuxType, TE mechanism, domain vs type distinction

4b — Cross-reference audit complete. `deployment/pages/index.adoc` now links to operations, logging-audit, glossary.

4c — Scratch file cleanup: dispositions identified (30 files), no deletions made (awaiting Jamie approval).
Group 1 (15 files): DELETED 2026-03-12.
Group 2 (awaiting Jamie): mls-classified-talk (444L vs 249L promoted), category_set_math, MLS_CATEGORIES_SET_MATH, HIGH_ASSURANCE_EXTRA, TPI_DUAL_LOGIC_FLOW, UMRS.cil, UMRS_CUI.cil, UMRS_LABELS-tool.txt, umrs-core-unicode.txt, notes/umrs-concepts, notes/umrs-levels-cui, notes/case-studies, notes/i18n, notes/CQRS, notes/encrypt-icon-verification, plus ~15 tool/logspace notes (umrs-logspace-*, umrs-state-*, umrs-signing-*, umrs-shred-notes, umrs-selinux-doc-README, AGR-NOTES, apache-mls-project, chain-script, chrome-mls-extension, fgexattr, IVM-SYSTEMD, logging_notrs, LS_HA_RESTRICTED_NOTES, rhel10-scripts-JSON, selinux-policy-junk-NOTES, unicode_symbols).

4d — Vision §21 validation: 5 topics tested, all placements unambiguous.

4e — Build: 2 errors total, both pre-existing in ubuntu.adoc. Zero new errors.

## SELinux Source Module Reference (for future SELinux doc work)

Key design deviations documented in source:
- CategorySet: dense [u64;16] vs kernel sparse ebitmap — for performance/determinism
- SelinuxType: mixed-case allowed (e.g., NetworkManager_etc_t) — matches kernel policy parser
- SelinuxUser/Role: lowercase only — [a-z0-9_] character set
- dominates(): `(subject & object) == object` — implemented word-by-word across 16 u64 words
