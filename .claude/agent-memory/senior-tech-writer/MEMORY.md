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

---

## Senior Tech Writer Backlog — COMPLETED 2026-03-10

All STW-1 through STW-7 items resolved. See feedback.md for per-item resolution notes.

Files created:
- `docs/modules/ROOT/pages/introduction.adoc`
- `docs/modules/ROOT/pages/getting-started.adoc`
- `docs/modules/architecture/pages/five-eyes-interop.adoc`
- `docs/modules/reference/pages/compliance-frameworks.adoc`
- `docs/modules/devel/pages/compliance-annotations.adoc`

Files expanded / revised:
- `docs/modules/architecture/pages/index.adoc` (stub → full overview)
- `docs/modules/architecture/pages/mls-history.adoc` (MAC definition, Biba model added)
- `docs/modules/architecture/pages/selinux-history.adoc` (security server defined, hooks defined, Biba named, accreditation defined)
- `docs/modules/architecture/pages/trusted-path-orange.adoc` (TCB defined before use, TCSEC formal name added, NIST SC-11 cited)

Nav entries added:
- `docs/modules/ROOT/nav.adoc` — introduction.adoc, getting-started.adoc
- `docs/modules/architecture/nav.adoc` — five-eyes-interop.adoc
- `docs/modules/reference/nav.adoc` — compliance-frameworks.adoc
- `docs/modules/devel/nav.adoc` — compliance-annotations.adoc

## Next Senior Tech Writer Tasks (no backlog item yet — flag for Jamie)

- `docs/modules/ROOT/pages/security-model.adoc` is still a redirect stub; now that
  architecture/index.adoc is substantive, this stub can likely be removed or replaced.
  Needs Jamie's decision.
- When developer use-case pages are ready to write (TW-8 depends on STW-2 being done —
  it is now done), senior-tech-writer should review TW-8 output before it is published.

---

## Primary Source Documents for Introduction and Architecture Content

Two project-level documents contain authoritative, Jamie-authored prose that should be
used as primary source material — especially for STW-1 (introduction) and STW-2
(architecture overview). Read these before writing those pages.

- `README.md` — defines high-assurance engineering, the HACAMS lineage, real-world HA
  system examples, and the key differences between HA and traditional systems. Contains
  Jamie's CC BY 4.0 licensed prose. Use it, don't duplicate it — link or excerpt.
- `UMRS-PROJECT.md` — the authoritative project description. Covers: what UMRS is,
  the MLS label hierarchy, CUI handling, the goal of demonstrating (not just describing)
  HA engineering, the component roadmap, and Jamie's personal note on adoption. This is
  the single best source for the introduction page.

Both are at the repo root. Jamie's own wording from UMRS-PROJECT.md should be preserved
or paraphrased closely in introductory content — do not invent the project description
when a vetted one already exists.

---

## Key Structural Facts

- `.md` files in `pages/` dirs do NOT render in Antora without a plugin — convert to `.adoc`
- `.txt` files in `pages/` dirs are inert — convert to `.adoc`
- Pending `.md` files in architecture that need conversion (tech-writer task):
  HACAMS.md, nom_parser.md, RTB.md, reference_monitor.md, kernel-files-TPI.md,
  TW0-NETIF-JUSTIFICATION.md
- `umrs-tools/` was already wired into ROOT/nav.adoc before this session started
- `security-model.adoc` in ROOT is a redirect stub — assess for removal after STW-2 exists

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
