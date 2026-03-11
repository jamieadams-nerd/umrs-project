# Senior Tech Writer — Persistent Memory

## Project: UMRS Documentation (Antora)

**Docs root**: `docs/modules/`
**Modules**: ROOT, architecture, devel, deployment, operations, reference, umrs-tools

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
- `antora.yml` already references `modules/architecture/nav.adoc` — nav file now exists
- `security/` and `historical/` were NOT registered in antora.yml — no removal needed
- `umrs-tools/` was already wired into ROOT/nav.adoc before 2026-03-11 session

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

## Writing Mode Defaults
- Architecture Mode for explanatory content
- STE Mode for procedures
- Load rules file before writing (`.claude/architecture_mode.md`, `.claude/ste_mode.md`)
