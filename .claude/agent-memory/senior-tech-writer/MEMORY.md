# Senior Tech Writer — Persistent Memory

## Cross-Team Channel
`.claude/agent-memory/cross-team/notes.md` — shared with all agents.
Read at session start for entries addressed to senior-tech-writer.
Write here to notify rust-developer or security-engineer of doc-driven questions.

## Project: UMRS Documentation (Antora)

**Docs root**: `docs/modules/`
**Modules**: ROOT, architecture, security-concepts, devel, deployment, operations, logging-audit, cryptography, reference, umrs-tools, patterns

**antora.yml** registers all 11 module nav files — updated 2026-03-13.
**cryptography** module added 2026-03-13 — nav.adoc, index.adoc, crypto-usage-map.adoc, and 6 pages moved from reference/pages/.
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
| openssl-no-vendoring, key-recommendation-list, fips-cryptography-cheat-sheet, crypto-post-quantum, crypto-policy-tiers, crypto-cpu-extensions, crypto-usage-map | cryptography | pages/ |
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

## Multi-Component Split Plan (review complete 2026-03-17)

Plan: `.claude/plans/antora-multi-component-split.md` — Draft, awaiting Jamie approval.
Review report: `.claude/reports/multi-component-split-review.md`

Key facts from review:
- Cross-module xref count: **272 occurrences across 72 files** — scripted substitution is mandatory
- 5-component structure is correct; no boundary changes recommended
- `umrs-development` nav will be 46+ items — requires deliberate sectioning before migration
- `crypto-usage-map.adoc` and `crypto-post-quantum.adoc` are dual-audience; plan assignments acceptable but need cross-component xrefs
- Glossary Option C (umrs-project) is correct
- Doc-theme plan should execute before the component split
- 4 open feedback items are pre-migration prerequisites: `rhel10-install.adoc` duplicate, `admin/` originals, `security-model.adoc` stub, `apparmor/` empty dir
- Nav drafts for all 5 components are required before any files move

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

truth-concepts.adoc: fully written 2026-03-14 — Runtime Source of Truth + Non-Bypassability. No longer a stub.
high-availability-history.adoc: new page in architecture/pages/history/ — HA vs high-assurance distinction. Added to architecture nav.

IMPORTANT: `architecture:rationale.adoc` does NOT exist. Correct filename is `architecture:rationale-strongly-typed.adoc`.

Operations module NO LONGER has a Logging section — all logging content is in deployment/ (setup) or logging-audit/ (operations).

---

## Patterns Module — Phase 2 Taxonomy (2026-03-12)

All 16 standard pattern pages updated. Two-zone structure (`== Why This Pattern Exists` + `== The Pattern`).
Nav groups: Architectural / Coding Techniques / Observability / Process / Deep Dives.
Security-concepts xrefs added to 8 pattern pages' See Also sections.

## OS Detection Pipeline Docs (2026-03-11)
- `patterns/pages/pattern-os-detection.adoc` — concept/architecture. Multi-audience.
- `devel/pages/os-detection-deep-dive.adoc` — engineer deep dive, full code references.
- `(device, inode)` is the canonical TOCTOU defense term. FIPS gate: sha2 not validated; ceiling T3.

## Phases 3–4 Complete (2026-03-12)

ROOT stubs populated: `what-is-high-assurance.adoc`, `what-is-umrs.adoc`, `ai-transparency.adoc`.
Glossary populated: 25+ definitions (Assurance/Integrity, SELinux/MLS, Cryptography).
Crypto reference pages complete: `crypto-post-quantum.adoc`, `crypto-policy-tiers.adoc` (both at `reference/pages/` root).
`crypto-cpu-extensions.adoc` remains stub — requires research.
SELinux reference pages rewritten to AsciiDoc (sensitivity, category_set, user, role, security_type — all in `reference/selinux/`).
Phase 4 plans archived: `.claude/plans/completed/`.
Build status: 2 pre-existing errors in ubuntu.adoc only.

## PQC Documentation Expansion (2026-03-13, round 1)

`crypto-post-quantum.adoc` expanded with:
- New "The Quantum Threat" section: Shor's/Grover's algorithms, harvest-now threat, CRQC
  timeline (5–15 years expert consensus), NIST standardization history (Dec 2016 – Aug 2024)
- KEM vs NIKE NOTE in ML-KEM section
- CNSA 2.0 NOTE in SLH-DSA section (CNSA 2.0 includes LMS/XMSS, excludes SLH-DSA)
- New "Algorithm Replacement Mapping" table (between SLH-DSA and Migration sections)
- NIST IR 8547 deprecation note (2035 deadline) and FIPS 206/HQC mention in Migration
- FIPS provider + hybrid deployment IMPORTANT block in Migration
- SI-7 added to control mapping
All facts verified via nist-pqc RAG collection. Build passes clean.

## PQC Documentation Expansion (2026-03-13, round 2 — RHEL 10 availability)

Source: `docs/new-stuff/latest-on-pqc.txt`. File incorporated and noted as resolved in cross-team notes.

Changes:
- `crypto-post-quantum.adoc`: FALCON/FIPS 206 language tightened; HQC draft timeline noted;
  14-candidate signature on-ramp (CROSS, FAEST, MAYO) added; FIPS IMPORTANT block updated
  with explicit FIPS/PQC mutual exclusion and CMVP pointer; new `== RHEL 10 PQC Availability`
  section with 3-row status table and FIPS constraint narrative; 3 Red Hat source URLs as footnotes.
- `crypto-usage-map.adoc`: Planned umrs-crypto goals list updated with FIPS gate bullet.
- `glossary/pages/index.adoc`: New `=== Crypto Policy (RHEL System-Wide Cryptographic Policy)` entry.
- `deployment/pages/rhel/rhel10-packages.adoc`: New `== Cryptographic Policy` cross-reference section.

## FIPS / PQC Constraint (authoritative — confirmed 2026-03-13)

FIPS mode and PQC are mutually exclusive on current RHEL 10.
NIST has not completed FIPS 140-3 validation for ML-KEM, ML-DSA, or SLH-DSA.
CUI systems (NIST 800-171, CMMC) must use FIPS mode → no PQC until validation completes.
RHEL 10.0 (May 2025): PQC was Technology Preview — required crypto-policies-pq-preview + DEFAULT:TEST-PQ.
RHEL 10.1+: PQC GA under DEFAULT policy for non-FIPS systems.
State this constraint explicitly whenever documenting PQC or umrs-crypto design.

## SELinux Source Module Reference

Key design deviations: CategorySet: dense [u64;16] vs kernel sparse ebitmap.
SelinuxType: mixed-case allowed. SelinuxUser/Role: lowercase [a-z0-9_] only.
dominates(): `(subject & object) == object`, word-by-word across 16 u64 words.

## Devel Guide Restructure (2026-03-13)

`rust-style-guide.adoc`, `secure-bash.adoc`, `secure-python.adoc` moved from `devel/pages/` to `reference/pages/`.
Reference `nav.adoc` now has a "Language & Style Guides" section at the bottom.
`devel/pages/index.adoc` rewritten — links to reference module for those guides.
`devel/nav.adoc` restructured — Language Guides section removed; `umrs-prog-lang.adoc` promoted to top level.
`os-detection-deep-dive.adoc` stays in `devel/pages/` (Antora `examples/` family is for includes only, not navigable pages).

**Antora `examples/` constraint**: Files in `examples/` cannot be navigable pages or xref targets. They are includable fragments accessed via `include::example$filename[]`. Never put navigable documents in `examples/`.

## AI Transparency Module (2026-03-15)

`ai-transparency` module created and registered. 13 pages written (all Phase 1 + Phase 2).

- All pages in `docs/modules/ai-transparency/pages/`
- nav.adoc covers all 13 pages — index, why-transparency, what-ai-does-and-does-not-do, agent-roles, workflow-mechanics, knowledge-pipeline, rag-collections, knowledge-provenance, auditor-guide, feedback-and-standing-rules, corpus-familiarization, skills-catalog, case-study-rmf-corpus
- Registered in `docs/antora.yml` after glossary entry
- ROOT nav updated: `xref:ai-transparency:index.adoc[AI in This Project]`
- ROOT page `ai-transparency.adoc` retained as summary stub; "Full Documentation" section added pointing to module
- Build verified: zero new errors (pre-existing ubuntu.adoc errors only)

ROOT nav disposition: The ROOT `ai-transparency.adoc` page is kept as a brief summary. The ROOT nav now points directly to `ai-transparency:index.adoc` (the module). If Jamie wants the ROOT page kept separately from the nav, the nav entry can be reverted.

## Navigation Restructure Plan (2026-03-13)

Plan: `.claude/plans/docs-new-stuff-crypto-and-navbar.md`
Tasks assigned to tech-writer in feedback.md: CTW-NAV-1 (ROOT nav rewrite), CTW-CRYPTO-1 (minor crypto enhancements).
crypto.md from new-stuff: almost entirely duplicate of existing pages. Three minor enhancements only.
Open questions for Jamie (in plan): Q1 page label disambiguation, Q2 historical section consolidation, Q3 API orientation page.

---

## Tech-Writer Corpus — Style Knowledge (2026-03-16)

**Location**: `.claude/knowledge/tech-writer-corpus/`
**RAG collection**: `tech-writer-corpus`
**Artifacts**: concept-index.md, cross-reference-map.md, style-decision-record.md, term-glossary.md, README.md

### Source priority order (highest → lowest)
1. MIL-STD-38784B — DoD TM structure, WARNING/CAUTION/NOTE hierarchy, modal vocabulary
2. Federal Plain Language Guidelines — statutory (Plain Writing Act 2010); active voice, 15-20 word sentences
3. NIST Author Instructions — NIST pub structure, Section 508, inclusive terminology (CANONICAL authority)
4. Google Developer Documentation Style Guide — developer docs primary commercial authority
5. Microsoft Writing Style Guide — secondary commercial authority; warm/relaxed voice

### Key resolved decisions (see style-decision-record.md for full SDRs)
- **Headings**: Sentence case always (Google + Microsoft agree); ALL-CAPS only in formal DoD TM submissions
- **Admonitions**: WARNING = security/data-loss risk; CAUTION = recoverable degradation; NOTE = informational; IMPORTANT = prerequisite
- **Inclusive terms**: allowlist/denylist (not whitelist/blacklist); primary/subordinate (not master/slave) — NIST is highest authority
- **Modals**: "shall/should/may" only in normative spec docs; use "must"/present tense in all other UMRS docs
- **Procedures**: Introductory sentence required; numbered steps, one action each; 7-step limit; sub-steps a/b/c then i/ii/iii
- **Oxford comma**: Required (both Google and Microsoft agree)
- **SDR-009 RESOLVED**: Third person for architecture/security-concepts; second person for devel/deployment/operations
- **SDR-010 RESOLVED**: Inclusive terms in narrative; standard terms in specs; verbatim quotes get editorial note

### Common Criteria Knowledge (CC:2022, added 2026-03-16)
- **Part 1** (Introduction): 94 formal definitions; evaluation chain: assets → threats → objectives → SFRs → SARs
- **Part 2** (SFR classes): 11 classes — FAU, FCS, FDP, FIA, FMT, FPR, FPT, FRU, FTA, FTP, FCO
- **UMRS mappings**: FAU=audit, FCS=FIPS/crypto, FDP=MLS/Bell-LaPadula, FIA=security context, FPT=fail-closed/self-test
- **Writing rule**: SFR element text uses "shall" (not "must") per CC convention
- **SFR operations**: assignment, selection, refinement, iteration — bracket/bold notation
- **Conformance types**: strict, demonstrable, exact (new in CC:2022)
- 6 CC terms added to term-glossary.md: EAL, PP, SFR, ST, TOE, TSF

### Remaining gaps
- GPO Style Manual — cited by MIL-STD for formal TM capitalization/punctuation
- Google word-list.md (2766 lines) — deferred; query RAG for specific terms
