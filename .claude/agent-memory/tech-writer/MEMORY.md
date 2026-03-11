# Tech-Writer Agent Memory

## Project Context
- UMRS: Unclassified MLS Reference System — high-assurance SELinux/Rust platform for CUI and MLS
- Primary doc tool: Antora (AsciiDoc). All live docs are .adoc. Markdown files are legacy/quarantine.
- Quarantine directory: `docs/_scratch/` — Antora ignores directories starting with `_`

## Antora Module Map
- `docs/modules/ROOT/` — project entry points, release notes, nav root
- `docs/modules/architecture/` — design rationale, history, background
- `docs/modules/devel/` — developer guides, Rust style, high-assurance patterns
- `docs/modules/deployment/` — OS configuration, SELinux policy setup
- `docs/modules/operations/` — day-to-day operation, admin tasks (admin/ merged here)
- `docs/modules/reference/` — API references, compliance registry
- `docs/modules/umrs-tools/` — tool-level docs (umrs-ls, prototype crates)
- `docs/modules/admin/` — DEPRECATED; originals still present pending Jamie's cleanup sign-off

## Feedback Log Location
`.claude/agent-memory/doc-team/feedback.md` — shared between tech-writer and senior-tech-writer.
Read at session start. Append resolved entries when tasks complete. Never delete entries.

## Conversion Conventions (Markdown → AsciiDoc)
- `#` heading → `=`, `##` → `==`, etc.
- ` ```lang ``` ` → `[source,lang]\n----\n...\n----`
- `**bold**` → `*bold*`
- `[text](url)` → `https://url[text]` or `link:url[text]`
- Tables: `|===` with header row `|Col1 |Col2`
- NOTE/WARNING/CAUTION admonitions: `NOTE: text` or `[NOTE]\n====\ntext\n====`
- Standard header: `= Title\n:toc: left\n:description: ...`

## Workflow Notes
- Source .md and .txt files in operations/ and architecture/ were raw AI conversation transcripts.
  These required restructuring into proper technical docs (not just format conversion) to be usable.
- When a file is modified externally between a Read and Edit call, re-read before editing.
- nav.adoc entries use `xref:filename.adoc[Display Text]` — cross-module refs use `module:file.adoc[Text]`

## Open Items for Jamie (as of 2026-03-10)
- admin/ module cleanup: originals in admin/pages/ still present; see feedback.md open entry
- rhel10-install.adoc vs deployment/rhel10-installation.adoc: potential duplicate, needs review
- security-model.adoc: redirect stub — decide to remove or replace with genuine content
- i18n.md in docs/_scratch/: confirm safe to delete (i18n.adoc is the complete version)

## Completed Work (2026-03-10 batch)
- Task 0: deployment/rhel10-packages.adoc — Post-Install Packages section added
- TW-1: umrs-tools/ wired into ROOT/nav.adoc; xref from operations/index.adoc
- TW-2: admin/ pages converted and copied to operations/pages/; Administration section in ops nav
- TW-3: 6 architecture/ .md files converted to .adoc; nav entries activated
- TW-4: 8 operations/ .txt/.md files converted to .adoc; nav entries added
- TW-6: umrs-logspace.adoc and umrs-state.adoc stub pages created in umrs-tools/
- TW-7: release-notes.adoc structured template added (3 sections with comment placeholders)
- TW-9: i18n.md quarantined; rust-must-use-contract.md → .adoc, nav entry added
- TW-10: security-model.adoc assessed as redirect stub; recommendation filed in feedback.md
