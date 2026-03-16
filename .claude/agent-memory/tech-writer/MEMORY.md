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

## Module Structure (as of 2026-03-13 crypto module added)
Active modules: ROOT, architecture, security-concepts, deployment, devel, patterns,
umrs-tools, operations, logging-audit, reference, cryptography, glossary
- Crypto pages: cryptography/pages/ (MOVED from reference/pages/ — update plans accordingly)
  - fips-cryptography-cheat-sheet.adoc, crypto-policy-tiers.adoc, crypto-post-quantum.adoc,
    key-recommendation-list.adoc, crypto-cpu-extensions.adoc
- reference/ still exists for compliance-frameworks, SELinux context, MLS, CUI pages
- Logging architecture pages: logging-audit/ (NOT deployment/)
- umrs-prog-lang.adoc: devel/ (NOT architecture/)

## Completed Work (2026-03-13 CTW tasks)
- CTW-NAV-1: ROOT/nav.adoc restructured with groupings; architecture/nav.adoc "Historical
  Context" consolidation done; security-concepts/nav.adoc "Foundations"/"Integrity and Trust"
  groupings added; what-is-umrs.adoc content merged into introduction.adoc and removed from nav
- CTW-CRYPTO-1: Three crypto enhancements applied to cryptography/ pages:
  1. fips-cryptography-cheat-sheet.adoc: referenced standards preamble added
  2. fips-cryptography-cheat-sheet.adoc: NOTE about audit log chaining added after SHA-1 entry
  3. crypto-policy-tiers.adoc: KDF "Typical use" column improved
- docs/new-stuff/used/ receives processed source files (crypto.md, left-navbar.md moved there)

## Cross-Module Xref Gotchas
- Grep ALL .adoc files when moving/deleting pages — nav files alone are not enough
- reference/pages/index.adoc mirrors nav.adoc as a body list — update both when moving pages
- architecture/pages/index.adoc had an inline xref that a nav-only grep would miss

## umrs-platform Source Doc Standards (established 2026-03-14)

- Correct citation: `NIST SP 800-53` (with `SP`). ~30 occurrences in umrs-platform use bare `NIST 800-53` — sweep needed.
  See: [feedback_citation_format.md](feedback_citation_format.md)
- All `//!` blocks should open with `# Module Name — Subtitle` heading. ~15 modules missing this.
- Best-in-class doc examples: `sealed_cache.rs` (threat model), `posture/modprobe.rs` (patterns section)
- Needs high-priority rewrite: `kattrs/mod.rs` (all-caps prose style, no Markdown structure)
- `lib.rs` module table missing row for `sealed_cache`
- Internal review references ("Finding N", "RAG Finding N") in source comments should be replaced with self-contained rationale

## Future Antora Pages Needed (from 2026-03-14 review)
- detect pipeline walkthrough (devel/)
- Confidence model / T0-T4 trust tiers (devel/ or architecture/)
- posture module developer guide (devel/)
- Provenance verification pattern page (patterns/)
- Trust Gate pattern page (patterns/)
- umrs-platform architecture overview (architecture/)

## Glossary State (as of 2026-03-15)
Single-page file at `docs/modules/glossary/pages/index.adoc`.
Sections: Assurance and Integrity, SELinux and MLS, Cryptography, UMRS Patterns.
Added 2026-03-15: AVC, DAC, EVM, IMA, LSM, MLS (dedicated entry), RBAC, TCB,
Ground Truth, TPI, Fail-Closed, TOCTOU Safety.
In-page xref anchor format: `xref:#_term_name_abbrev[...]` — lowercase, underscores, auto-generated.

## CPU Extensions Reference (added 2026-03-16)

`docs/modules/reference/pages/cpu-extensions.adoc` — new reference page.
Three-layer activation model (hardware / OS / software), Mermaid activation flow,
6 extension groups: Cryptographic Acceleration, Hardware RNG, Memory Protection,
Speculation Mitigations, Vector/Compute (security-adjacent), Trusted Execution.
Detection quick-reference table covers all extensions. Compliance notes: SC-13,
SI-7, CM-8, CA-7, SC-39. Written for i18n (complete sentences, no fragments).
Nav: added "Platform Security" section to `reference/nav.adoc`.
Source: `.claude/plans/umrs-platform-expansion.md` + `.claude/plans/cpu-security-corpus-plan.md`.
Feeds future `CpuSignalId` enum. `ContradictionKind::CapabilityUnused` surfaced as a future
finding concept (Layer 1 present, Layer 2 enabled, Layer 3 not used).
Note: "crypto-cpu-extensions.adoc" in MEMORY.md line 129 is a phantom stub that was never
on disk. This new `cpu-extensions.adoc` fulfills that intent.

## Platform Update Checklists (added 2026-03-16)

`docs/modules/devel/pages/update-checklists.adoc` — new developer guide.
Covers: kernel version update (7-step signal addition procedure), CPU extension
update (three-layer activation model, `CpuSignalId` add checklist), signal
deprecation (never delete variants, use `#[deprecated]` + graceful `None`).
Nav: added under "Platform Internals" in `devel/nav.adoc`.
Source: `signal.rs`, `catalog.rs`, `kattrs/mod.rs`, `umrs-platform-expansion.md`,
`platform-api-enrichment.md`.
Two reference pages are forward-referenced but do not yet exist:
`reference/pages/kernel-probe-signals.adoc` (created by security-auditor
2026-03-16 — confirmed exists), `reference/pages/cpu-extensions.adoc`
(created 2026-03-16 — confirmed exists). Both xrefs use forward references in the
See Also section to avoid dead xrefs at build time.

## AI Transparency — Agent Aliases (added 2026-03-15)
Section "== Agent Aliases" added to `ai-transparency/agent-roles.adoc`.
Table maps alias → agent → rationale.
Aliases: Boss (orchestrator), Rusty (rust-developer), The IRS (security-auditor),
The Librarian (researcher), The Imprimatur (senior-tech-writer), Von Neumann (tech-writer),
Simone (umrs-translator), Summer Intern (guest-coder).
Formal task board entries always use canonical identifier, not alias.
