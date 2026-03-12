# Tech-Writer Agent Memory

## Project Context
- UMRS: Unclassified MLS Reference System — high-assurance SELinux/Rust platform for CUI and MLS
- Primary doc tool: Antora (AsciiDoc). All live docs are .adoc. Markdown files are legacy/quarantine.
- Quarantine directory: `docs/_scratch/` — Antora ignores directories starting with `_`
- Mermaid diagrams are supported in Antora .adoc files — use `[mermaid]\n....\n....` blocks

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

## Cross-Team Channel
`.claude/agent-memory/cross-team/notes.md` — shared with all agents.
Read at session start for entries addressed to tech-writer.
rust-developer and security-engineer leave doc requests here.

## Conversion Conventions (Markdown → AsciiDoc)
- `#` heading → `=`, `##` → `==`, etc.
- ` ```lang ``` ` → `[source,lang]\n----\n...\n----`
- `**bold**` → `*bold*`
- `[text](url)` → `https://url[text]` or `link:url[text]`
- Tables: `|===` with header row `|Col1 |Col2`
- NOTE/WARNING/CAUTION admonitions: `NOTE: text` or `[NOTE]\n====\ntext\n====`
- Standard header: `= Title\n:toc: left\n:description: ...`
- Mermaid: `[mermaid]\n....\n<diagram>\n....`

## Workflow Notes
- Source .md and .txt files in operations/ and architecture/ were raw AI conversation transcripts.
  These required restructuring into proper technical docs (not just format conversion) to be usable.
- When a file is modified externally between a Read and Edit call, re-read before editing.
- nav.adoc entries use `xref:filename.adoc[Display Text]` — cross-module refs use `module:file.adoc[Text]`

## MANDATORY: Build Verification Rule
- **`make docs` must pass cleanly** before any docs/ work is considered done. No exceptions.
- Run `make docs 2>&1` from the repo root and verify zero xref errors in the output.
- When moving pages into subdirectories, update ALL xrefs across ALL modules that reference the moved pages — not just the nav files.
- Cross-module xrefs (e.g., `reference:context.adoc`) must be updated when the target page moves to a subdirectory (e.g., `reference:selinux/context.adoc`).

## High-Assurance Pattern Library (`docs/modules/patterns/`)
Created 2026-03-11. Dedicated Antora module. Registered in `antora.yml`.
Each pattern from CLAUDE.md has its own .adoc page under `patterns/pages/`.
All pages have: Overview, Threat, Pattern, In the UMRS Codebase, When to Apply, Controls, Summary, See Also.

Pattern files:
- `pattern-tpi.adoc` — Two-Path Independence (Mermaid flowchart)
- `pattern-toctou.adoc` — TOCTOU Safety (Mermaid sequence diagram)
- `pattern-fail-closed.adoc` — Fail-Closed
- `pattern-provenance.adoc` — Provenance Verification (Mermaid flowchart)
- `pattern-loud-failure.adoc` — Loud Failure
- `pattern-non-bypassability.adoc` — Non-Bypassability (RAIN)
- `pattern-secure-arithmetic.adoc` — Secure Arithmetic
- `pattern-zeroize.adoc` — Zeroize Sensitive Data [⚠ not yet implemented]
- `pattern-constant-time.adoc` — Constant-Time Comparison (Mermaid sequence diagram) [⚠ not yet implemented]
- `pattern-error-discipline.adoc` — Error Information Discipline
- `pattern-bounds-safe.adoc` — Bounds-Safe Indexing
- `pattern-supply-chain.adoc` — Supply Chain Hygiene (cargo-audit is the tool; Mermaid flowchart)
- `pattern-sec.adoc` — Sealed Evidence Cache [✓ implemented in `umrs-platform/src/sealed_cache.rs`]

Index page: `patterns/pages/index.adoc`
- Full reference table with implementation status (✓ / ⚠)
- "Open — Awaiting Implementation" section for rust-developer visibility
- Patterns grouped by category

`devel/pages/high-assurance-patterns.adoc` — kept as consolidated narrative; Pattern Reference
section now redirects to `patterns:index.adoc` instead of listing pages inline.
`devel/nav.adoc` — pattern subsections removed; single "Pattern Library →" cross-module link added.

## Status of Unimplemented Patterns
- Zeroize: vaultmgr needs `zeroize` crate + `ZeroizeOnDrop` on secret types
- Constant-Time: `subtle` crate not in workspace; needed before any credential comparison
- SEC (Sealed Evidence Cache): IMPLEMENTED — `umrs-platform/src/sealed_cache.rs`;
  zeroize crate is in workspace; SealingKey derives ZeroizeOnDrop

## Open Items for Jamie (as of 2026-03-10)
- admin/ module cleanup: originals in admin/pages/ still present; see feedback.md open entry
- rhel10-install.adoc vs deployment/rhel10-installation.adoc: potential duplicate, needs review
- security-model.adoc: redirect stub — decide to remove or replace with genuine content
- i18n.md in docs/_scratch/: confirm safe to delete (i18n.adoc is the complete version)

## Completed Work (2026-03-10 batch)
- Task 0: deployment/rhel10-packages.adoc — Post-Install Packages section added
- TW-1 through TW-10: see feedback.md for details

## Completed Work (2026-03-11 batch)
- 12 individual high-assurance pattern pages created in devel/pages/
- high-assurance-patterns.adoc updated with Pattern Reference table linking to all 12
- devel/nav.adoc updated with pattern subsections
- SEC pattern docs updated for implementation:
  - pattern-sec.adoc: WARNING block removed; "In the UMRS Codebase" rewritten for actual types
  - patterns/index.adoc: SEC status ⚠→✓; "Open — Awaiting Implementation" SEC block removed
  - os-detection-deep-dive.adoc: "Future: Sealed Memory Cache" section replaced with
    full "Sealed Evidence Cache" section covering SealedCache API, seal payload layout,
    FIPS gate, decode_cached_result() design note, CacheStatus, compliance table

## Completed Work (2026-03-12 Phase 1 Antora restructure)
- security-compliance/ module deleted (was empty, not in antora.yml)
- architecture/umrs-prog-lang.adoc deleted; devel/ copy is canonical; all xrefs updated
  - Fixed in: architecture/nav.adoc, devel/nav.adoc, architecture/pages/index.adoc
- deployment/structured-logging.adoc moved to logging-audit/
- deployment/how-to-structure-log.adoc moved to logging-audit/
  - Fixed xref in: ROOT/pages/getting-started.adoc (deployment: → logging-audit:)
- deployment/pages/_archive/ directory deleted (5 .md files, all superseded)
- glossary/ module created: pages/index.adoc + nav.adoc; registered in antora.yml
- reference/pages/cryptography/ subdir promoted to top-level reference/pages/
  - Nav heading renamed: "Cryptography" → "Cryptographic Baseline"
  - Fixed xrefs in: reference/pages/index.adoc, devel/pages/high-assurance-patterns.adoc,
    architecture/pages/cui-structure.adoc
- Placeholder pages created: ROOT/pages/what-is-umrs.adoc, what-is-high-assurance.adoc,
  ai-transparency.adoc; reference/pages/crypto-post-quantum.adoc, crypto-policy-tiers.adoc,
  crypto-cpu-extensions.adoc
- ROOT/nav.adoc: orientation pages first; glossary and release-notes added

## Module Structure (as of 2026-03-12 Phase 1 complete)
Active modules: ROOT, architecture, security-concepts, deployment, devel, patterns,
umrs-tools, operations, logging-audit, reference, glossary
- Crypto pages: reference/pages/ top-level (NOT cryptography/ subdir)
- Logging architecture pages: logging-audit/ (NOT deployment/)
- umrs-prog-lang.adoc: devel/ (NOT architecture/)

## Cross-Module Xref Gotchas
- Grep ALL .adoc files when moving/deleting pages — nav files alone are not enough
- reference/pages/index.adoc mirrors nav.adoc as a body list — update both when moving pages
- architecture/pages/index.adoc had an inline xref that a nav-only grep would miss
