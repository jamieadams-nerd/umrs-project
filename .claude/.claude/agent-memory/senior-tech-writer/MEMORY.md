# Senior Tech Writer Persistent Memory

## Project Context
- UMRS: Unclassified MLS Reference System — high-assurance SELinux platform on RHEL 10
- Primary doc format: Antora/AsciiDoc at `docs/modules/`
- Build gate: `make docs` (must pass zero errors before any docs task is complete)
- Vision source: `.claude/jamies_brain/doc-vision.md` — 25 sections, §3–§18 are the domains

## Antora Module Map (current as of 2026-03-12)
- `ROOT/` — project entry points (§3, §4 What UMRS is)
- `architecture/` — design rationale, history subdir (§4, §5, §6)
- `security-concepts/` — assurance principles, reference monitor, RTB (§6)
- `deployment/` — OS config, STIG layout, HA enhancements (§7, §8, §9)
- `devel/` — developer guide, Rust style, patterns guide (§11)
- `patterns/` — 17 high-assurance pattern pages (§12)
- `umrs-tools/` — tool documentation (§13)
- `operations/` — day-to-day operations, signing, AIDE (§14)
- `logging-audit/` — log lifecycle, capacity, tuning, auditing (§15)
- `reference/` — SELinux types, CUI, crypto tables (§16)
- `glossary/` — DOES NOT EXIST YET; planned Phase 1b (§17)
- `security-compliance/` — EMPTY; delete in Phase 1a

## Known Issues (from Phase 0 audit 2026-03-12)
- `deployment/structured-logging.adoc` belongs in `logging-audit/` — WRONG MODULE
- `deployment/how-to-structure-log.adoc` belongs in `logging-audit/` — WRONG MODULE
- `architecture/umrs-prog-lang.adoc` and `devel/umrs-prog-lang.adoc` are IDENTICAL DUPLICATES — delete devel copy
- `security-compliance/` module is empty — delete it
- `glossary/` module does not exist — create in Phase 1b

## _scratch File Pattern
All `docs/_scratch/*.txt` and `*.md` files are raw AI conversation transcripts.
Every transcript that became a doc has a polished Antora page that supersedes it.
Pattern is consistent: compare before deciding, but expect the .adoc to be the complete version.

## Phase 0 Manifest Location
`/media/psf/repos/umrs-project/.claude/reports/phase0-migration-manifest.md`
Full audit of all 130+ pages, 50 scratch files, 5 archive files.

## Duplicate-detection Rule
`umrs-prog-lang.adoc` exists in both architecture/ and devel/ with identical content.
Architecture copy is authoritative. Delete devel copy in Phase 1c.

## Gap List (domains with no content)
- §17 Glossary: no module
- §18 AI transparency: no page
- §4 "What is UMRS" dedicated page: partial only
- §3 "What is High Assurance" dedicated page: partial only
- Post-quantum cryptography: not in Antora yet (content in `docs/new-stuff/crypto.md`)
- Protocol Break / Data Sanitization pattern: not in patterns/
