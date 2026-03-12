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

- `README.md` — defines high-assurance engineering, HACAMS lineage, HA examples
- `UMRS-PROJECT.md` — authoritative project description, MLS label hierarchy, CUI handling

---

## Terminology Preferences (confirmed)
See `.claude/approved_terminology.md` for the full list.
Preferred: `security label`, `audit event`, `system mediator`, `policy enforcement`,
`sensitivity level`, `category set`, `MLS range`, `reference monitor`, `security context`

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

## OS Detection Pipeline Docs (2026-03-11)

Two new pages written for the `umrs-platform` OS detection pipeline:

- `docs/modules/patterns/pages/pattern-os-detection.adoc` — "OS Detection: A Trust
  Ladder". Patterns module. Multi-audience (auditors, newcomers, adopters). No code.
  Wired into `patterns/nav.adoc` under Patterns — Verification Pipelines.

- `docs/modules/devel/pages/os-detection-deep-dive.adoc` — "OS Detection Pipeline —
  Deep Dive". Devel module. Engineers and security auditors. Full code references,
  per-phase threat model, confidence model, EvidenceBundle design.
  Wired into `devel/nav.adoc` under Platform Internals.

Key conventions used in these docs (reuse for future pipeline docs):
- Code snippets are representative; use `// Conceptual` comment if not yet implemented
- Every phase section: What / Why / Code Reference / Security Controls sub-sections
- `(device, inode)` is the canonical TOCTOU defense term — not "dev/ino" or "inode pair"
- FIPS gate explanation: sha2 crate is not validated; ceiling becomes T3 on FIPS systems
- EvidenceBundle rationale always links AU-3 and the "silent rejection = audit gap" argument
- Trust level comparisons use `<`/`>=` on the `TrustLevel` enum (PartialOrd), not integer checks
