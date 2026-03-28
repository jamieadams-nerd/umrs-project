# Tech-Writer Agent Memory — "Von Neumann"
# Alias: Von Neumann. No "The". Bio and portrait pending.
# Named for John von Neumann — literally wrote the architecture document that defined how computers work.

## Project Context
- UMRS: Unclassified MLS Reference System — high-assurance SELinux/Rust platform for CUI and MLS
- Mermaid diagrams supported in Antora — use `[mermaid]\n....\n....` blocks

## Antora Module Map
- `docs/modules/ROOT/` — project entry points, release notes, nav root
- `docs/modules/architecture/` — design rationale, history, background
- `docs/modules/devel/` — developer guides, Rust style, high-assurance patterns
- `docs/modules/deployment/` — OS configuration, SELinux policy setup
- `docs/modules/operations/` — day-to-day operation, admin tasks (admin/ merged here)
- `docs/modules/reference/` — API references, compliance registry
- `docs/modules/umrs-tools/` — tool-level docs (umrs-ls, prototype crates)
- `docs/modules/admin/` — DEPRECATED; originals still present pending Jamie's cleanup sign-off

## Communication Channels
- Feedback log: `.claude/agent-memory/doc-team/feedback.md`
- Cross-team channel: `.claude/agent-memory/cross-team/notes.md`

## Workflow Notes
- Source .md and .txt files in operations/ and architecture/ were raw AI conversation transcripts.
  These required restructuring into proper technical docs (not just format conversion).
- nav.adoc entries use `xref:filename.adoc[Display Text]` — cross-module refs use `module:file.adoc[Text]`

## High-Assurance Pattern Library (`docs/modules/patterns/`)
Created 2026-03-11. Each pattern has its own .adoc page under `patterns/pages/`.
All pages have: Overview, Threat, Pattern, In the UMRS Codebase, When to Apply, Controls, Summary, See Also.
Index page: `patterns/pages/index.adoc` — grouped by category with implementation status (✓ / ⚠).

## Status of Unimplemented Patterns
- Zeroize: vaultmgr needs `zeroize` crate + `ZeroizeOnDrop` on secret types
- Constant-Time: `subtle` crate not in workspace; needed before any credential comparison
- SEC (Sealed Evidence Cache): IMPLEMENTED — `umrs-platform/src/sealed_cache.rs`

## Open Items for Jamie (as of 2026-03-10)
- admin/ module cleanup: originals in admin/pages/ still present; see feedback.md open entry
- rhel10-install.adoc vs deployment/rhel10-installation.adoc: potential duplicate, needs review
- security-model.adoc: redirect stub — decide to remove or replace with genuine content
- i18n.md in docs/_scratch/: confirm safe to delete (i18n.adoc is the complete version)

## Module Structure (as of 2026-03-13)
Active modules: ROOT, architecture, security-concepts, deployment, devel, patterns,
umrs-tools, operations, logging-audit, reference, cryptography, glossary
- Crypto pages: cryptography/pages/ (MOVED from reference/pages/)
- Logging architecture pages: logging-audit/ (NOT deployment/)
- umrs-prog-lang.adoc: devel/ (NOT architecture/)

## umrs-platform Source Doc Standards (established 2026-03-14)
- ~30 occurrences use bare `NIST 800-53` — sweep needed. See [feedback_citation_format.md](feedback_citation_format.md)
- ~15 modules missing `# Module Name — Subtitle` heading in `//!` blocks
- Best-in-class examples: `sealed_cache.rs` (threat model), `posture/modprobe.rs` (patterns section)
- Needs rewrite: `kattrs/mod.rs` (all-caps prose style, no Markdown structure)
- `lib.rs` module table missing row for `sealed_cache`

## Future Antora Pages Needed (from 2026-03-14 review)
- detect pipeline walkthrough (devel/)
- Confidence model / T0-T4 trust tiers (devel/ or architecture/)
- posture module developer guide (devel/)
- Provenance verification pattern page (patterns/)
- Trust Gate pattern page (patterns/)
- umrs-platform architecture overview (architecture/)

## Glossary State (as of 2026-03-15)
Single-page at `docs/modules/glossary/pages/index.adoc`.
Sections: Assurance and Integrity, SELinux and MLS, Cryptography, UMRS Patterns.
Anchor format: `xref:#_term_name_abbrev[...]` — lowercase, underscores, auto-generated.

## SCAP/STIG Corpus (familiarized 2026-03-17)
Detailed findings at [scap_familiarization.md](scap_familiarization.md).
- CCE citation format established (Rust, Antora, CLI/TUI)
- Approved terms added: CCE, SCAP, STIG, RHEL 10 STIG, XCCDF

## Style Corpus
Artifacts at `.claude/knowledge/tech-writer-corpus/`. RAG collection: `tech-writer-corpus`.
Style decisions (SDR-001 through SDR-009) in `style-decision-record.md`.
Authority hierarchy: MIL-STD-38784B > Federal Plain Language > NIST Author Instructions > Google > Microsoft.

## ROOT Module Editorial Notes (2026-03-22)
See [feedback_raw_notes_in_pages.md](feedback_raw_notes_in_pages.md).
- `foundations/NOTE-TO-JAMIE.txt` — flagged for Jamie to remove
- `foundations/history/mls-classified-talk.adoc` — placeholder stub, content pending
- "targeted policy" is the consistent term (not "targeted mode")
