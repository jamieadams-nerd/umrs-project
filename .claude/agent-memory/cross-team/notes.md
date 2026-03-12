# Cross-Team Notes

Shared across all agents. Any agent can write here to notify another agent of something
that crosses team boundaries — documentation gaps, new patterns, API changes that affect
docs, compliance findings that require new doc content.

**Read this file at session start.** Check for open entries addressed to your agent role.
Mark entries `resolved` when acted on. Do not delete entries.

## Format

```
## [YYYY-MM-DD] [from-agent] → [to-agent]: [topic]

**Status**: open | resolved

[Content — one concern per entry. Be specific: file paths, pattern names, crate names.]
```

## Agent Directory

| Agent | Writes about |
|---|---|
| `rust-developer` | New patterns implemented, API changes, doc gaps noticed in source, patterns needed but not yet in library |
| `security-engineer` | Compliance findings that require doc updates, new control mappings, audit gaps |
| `security-auditor` | Compliance audits: verifies control citations, identifies annotation debt, produces audit findings and reports |
| `tech-writer` | Questions about API or pattern intent, requests for source examples |
| `senior-tech-writer` | Architecture-level doc decisions, cross-module structural changes |
| `researcher` | RAG pipeline management, reference collection ingestion, standards research, research reports (`refs/reports/`) |
| `umrs-translator` | Text extractions from i18n-wrapped strings, language translations for active domains |
| `changelog-updater` | Structured changelog maintenance: tracks additions, changes, and fixes across crates, docs, and infrastructure in `.claude/CHANGELOG.md` |

---

<!-- Entries below, newest first -->

---

## [2026-03-12] coordinator → all-agents: DOCUMENTATION FREEZE — Antora restructure underway

**Status**: open

**DO NOT add new documentation pages to `docs/modules/` until the restructure is complete.**

Jamie has approved a full Antora documentation restructure to align with the project's
documentation vision (`.claude/jamies_brain/doc-vision.md`). The plan is at
`.claude/plans/antora-doc-restructure.md`.

**What's happening**:
- Phase 0: Audit every page, triage `_scratch/` and `_archive/`, produce migration manifest
- Phase 1: Move misplaced content, delete empty `security-compliance/`, create `glossary/` module, strengthen ROOT
- Phase 2: Patterns taxonomy (labeling, provenance badges, two-zone structure)
- Phase 3: New content (crypto section, glossary, ROOT pages, AI transparency)
- Phase 4: Curation, cross-linking, final validation

**Key decisions**:
- `security-compliance/` module: being deleted (empty, no purpose)
- `glossary/`: new module being created
- `_scratch`/`_archive` files: deleted after content is promoted into Antora pages
- `docs/new-stuff/crypto.md`: seed material for cryptography section

**Agent impacts**:
- **tech-writer, senior-tech-writer**: You are the primary executors. Check task board for
  Phase 0–4 tasks. Senior-tech-writer leads Phase 0 (audit) and Phase 4 (curation).
  Tech-writer leads Phase 1 (moves) and Phase 2 (patterns). Both collaborate on Phase 3.
- **rust-developer**: If your work produces new public API or patterns, create a `doc-sync:`
  task as usual — but note that the target module may change during restructure. Include
  the content description; the tech-writer will determine final placement.
- **security-engineer, security-auditor**: Compliance doc updates should be queued as tasks
  rather than written directly until the restructure settles.
- **researcher**: No impact on RAG work. Continue ingestion as normal.
- **umrs-translator**: No impact — i18n domains are tool binaries, not docs.
- **changelog-updater**: Log the restructure as a single entry when each phase completes.

**Task board**: Phase tasks are #2 (Phase 0), #4 (Phase 1), #5 (Phase 2), #1 (Phase 3), #3 (Phase 4) with blocking dependencies set.

---

## [2026-03-12] researcher → security-engineer, security-auditor: DoD 5200.01 series + CUI policy in RAG

**Status**: open

Five DoD Information Security Program documents downloaded from `esd.whs.mil` and ingested into RAG collection `dod-5200` (360 chunks):

| Document | Chunks | Key content |
|---|---|---|
| DoDI 5200.01 | 13 | Authorizing directive — collateral, SAP, SCI, CUI responsibilities |
| DoDM 5200.01 Vol 1 | 75 | Classification and declassification (Change 3, Jan 2025) |
| DoDM 5200.01 Vol 2 | 111 | **Marking procedures** — directly relevant to CUI label rendering in `cui-labels` and `mcs-setrans` |
| DoDM 5200.01 Vol 3 | 121 | Protection safeguards — storage, transmission, access controls |
| DoDI 5200.48 | 40 | **DoD CUI policy** — identification, marking, handling. Supersedes Vol 4. Requires NIST 800-171. |

Query: `rag-query --collection dod-5200`. Source PDFs at `refs/dod/`.

See Task #6 for full details.

---

## [2026-03-12] researcher → all-agents: RAG expansion complete — 5 new/expanded collections

**Status**: open

The following RAG collections were added or significantly expanded today. All are immediately
queryable via the `rag-query` skill.

**Updated collection (major expansion):**

| Collection | Old chunks | New chunks | What was added |
|---|---|---|---|
| nist | 461 | 1,447 | sp800-171r2, sp800-171r3, **sp800-171Ar3** (new), sp800-218-ssdf, sp800-53r5, fips140-2, fips140-3 |

**New collections:**

| Collection | Chunks | What's in it |
|---|---|---|
| rustdoc-book | 194 | Rustdoc reference book (doc.rust-lang.org/rustdoc/print.html) — doc comment syntax, attributes, intra-doc links, test harness |
| asciidoctor-ref | 67 | AsciiDoc syntax quick reference + document structure guide |
| dita-spec | 100 | OASIS DITA 1.3 Part 2 Technical Content — concept/task/reference topic type definitions |

**New document added to refs/:**
- `refs/nist/sp800-171Ar3.pdf` — NIST SP 800-171A Rev 3 (Assessment procedures for CUI controls).
  SHA-256: `946d963707cdaba19901c49d5c89517adb00844fe5d101e9dac7febc68e34cfa`
  Manifest entry added to `refs/manifest.md`.

**Notes for specific agents:**
- **tech-writer / senior-tech-writer**: `rustdoc-book` and `asciidoctor-ref` are now searchable —
  useful when writing Rust doc comments or Antora AsciiDoc content.
- **security-engineer / security-auditor**: `nist` collection now includes 800-171A Rev 3 assessment
  procedures and FIPS 140-2/3. Query with `rag-query --collection nist` for control assessment details.
- **rust-developer**: `rustdoc-book` collection is available for rustdoc syntax questions.

**Pending (requires user decision):**
- DoD 5200.01 (Information Security Program, 4 volumes): URLs located at esd.whs.mil (official .mil
  DoD Issuances site, not currently on approved source list). Flagged for user confirmation.
- IEEE 829 (Software Test Documentation): paywalled; requires manual download if desired.

---

---

## [2026-03-12] researcher → all-agents: Reference document locations — know where to look

**Status**: open

There are **three layers** of reference material. Know which to use:

| Layer | Path | What's there | When to use |
|---|---|---|---|
| **RAG database** | Queried via `rag-query` skill | Chunked, searchable text from all ingested collections | **Default for any technical question** — fast semantic search, use routinely when writing code or docs |
| **Source documents** | `.claude/references/<collection>/` | Original PDFs, HTML, markdown used to build the RAG | When you need to read the full original document, verify page numbers, or check context around a RAG result |
| **Official refs** | `refs/nist/`, `refs/dod/`, `refs/reports/` | Canonical copies of standards (NIST SPs, FIPS, CMMC, DoD docs) and researcher reports | Authoritative source of record; integrity-verified via `refs/manifest.md` SHA-256 checksums |

**Key distinctions:**
- `refs/` is the **permanent, auditable archive** — checksummed, manifested, never modified after download
- `.claude/references/` is the **RAG source staging area** — may contain copies from `refs/` plus additional material (academic papers, man pages, etc.) organized by collection
- The RAG database is the **search index** — derived from `.claude/references/`, not from `refs/` directly
- Some documents exist in both `refs/` and `.claude/references/` (e.g., CMMC PDFs, NIST SPs) — `refs/` is the authoritative copy

**Current RAG collections** (query any via `rag-query`):

| Collection | Chunks | Topics |
|---|---|---|
| kernel-docs | 22,738 | Linux kernel documentation tree |
| access-control | 1,360 | Bell-LaPadula, Biba, Brewer-Nash, Saltzer-Schroeder, ABAC, ZTA, SELinux, capabilities, POSIX ACL |
| selinux-notebook | 691 | SELinux reference (policy, TE, MLS/MCS, labeling, xattrs) |
| cmmc | 545 | CMMC Final Rule (32 CFR 170) + Assessment Guide L2 v2.13 |
| dod-5200 | 360 | DoD 5200.01 (Info Security Program, Vols 1-3) + DoDI 5200.48 (CUI policy) |
| nist | 1,447 | NIST SPs (800-53r5, 800-171r2, 800-171r3, 800-171Ar3, 800-218) + FIPS 140-2/3 + others |
| doc-structure | 102 | Diataxis, Antora, modular docs, style guides |
| rust-security | 73 | Rust security patterns and references |
| linux-fhs-2-3 | 45 | Filesystem Hierarchy Standard |

**Rule of thumb**: Start with `rag-query`. If you need more context, read the source in `.claude/references/`. If you need the checksummed authoritative copy, go to `refs/`.

---

## [2026-03-12] researcher → security-auditor, security-engineer: CMMC documents downloaded and in RAG

**Status**: open

Two critical CMMC documents are now downloaded, verified, and searchable in the RAG (collection: `cmmc`, 545 chunks):

| Document | Version | Published | Chunks | Path |
|---|---|---|---|---|
| CMMC Final Rule (32 CFR Part 170) | 89 FR 83092 | Oct 15, 2024 | 282 | `refs/dod/cmmc-32cfr170-final-rule.pdf` |
| CMMC Assessment Guide Level 2 | v2.13 | Sep 2024 | 263 | `refs/dod/cmmc-assessment-guide-l2.pdf` |

**Important corrections made to `refs/manifest.md`:**
- The previously listed Final Rule URL was **wrong** (pointed to an OMB submission, not the CMMC rule). Corrected to the actual final rule (document 2024-22905).
- The Assessment Guide URL was **stale** (404); corrected to the v2.13 filename (`AssessmentGuideL2v2.pdf`).
- The Final Rule is from **October 2024** (not December 2023) — this is the legally binding final rule, effective December 16, 2024, not the proposed rule.

**security-auditor**: See Task #2 for detailed action items — verify control citations, check for v2.0→v2.13 deltas, confirm FR citation accuracy in docs.

**security-engineer**: See Task #1 for action items — map Final Rule requirements to UMRS architecture, identify CMMC-specific gaps beyond NIST 800-171.

Use `rag-query` skill with collection `cmmc` to search these documents.

---

## [2026-03-12] coordinator → all-agents: RAG collections fully ingested

**Status**: open

All RAG collections are now ingested and available for querying via the `rag-query` skill:

| Collection | Chunks |
|---|---|
| kernel-docs | 22,738 |
| access-control | 1,360 |
| selinux-notebook | 691 |
| nist | 461 |
| doc-structure | 102 |
| rust-security | 73 |
| linux-fhs-2-3 | 45 |

Use the `rag-query` skill to search any of these collections. The `access-control` collection
covers foundational papers, OS security models, rule catalogs, and standards.

---

## [2026-03-12] coordinator → senior-tech-writer, tech-writer: doc-arch skill and internalized knowledge

**Status**: open

Two changes to the documentation writing agents:

1. **senior-tech-writer.md** — expanded with internalized knowledge distilled from 7 documentation architecture sources (Diataxis, Divio, Antora, Red Hat modular docs, Write the Docs, Google style, GitLab docs). Key additions: Diataxis taxonomy with UMRS module mapping, compass test, modular documentation rules, Antora mechanics, style/voice guidelines, procedural writing rules, content classification quick reference.

2. **New `doc-arch` skill** — searches the `doc-structure` RAG collection (102 chunks). Use this skill for deeper queries about documentation architecture, Antora structure, modular doc patterns, style guide rules, and docs-as-code practices. The senior-tech-writer has the framework internalized; the skill provides backup for specific questions.

The tech-writer agent definition has also been updated to reference the `doc-arch` skill.

---

## [2026-03-11] coordinator → all-agents: rust-prototypes workspace is out of scope

**Status**: open

The prototype crates (`cui-labels`, `kernel-files`, `mcs-setrans`, `vaultmgr`) have been
moved from `components/rusty-gadgets/` to a new workspace at `components/rust-prototypes/`.

**All agents**: ignore `components/rust-prototypes/` unless explicitly asked to work on it.
Do not audit, document, translate, or include it in reports. It is a parking lot for
experimental code and is not part of the active development surface.

---

## [2026-03-11] coordinator → all-agents: Plan completion workflow

**Status**: open

When a plan (in `.claude/plans/`) has been fully implemented:
1. Confirm completion with the user (Jamie).
2. Once confirmed, mark the plan as completed (add a `## Status: Completed` header and date).
3. Move the file to `.claude/plans/archive/`.

Do not archive plans without user confirmation.

---

## [2026-03-11] coordinator → all-agents: Notify umrs-translator for new i18n strings

**Status**: open

If your work introduces new or updated code that contains i18n-wrapped strings (e.g.,
`gettext!`, `tr!`, or any localization macro), you MUST notify the **umrs-translator**
agent when your work is complete.

The umrs-translator will then:
1. Perform text extractions from the updated source.
2. Perform language translations for all active domains.

Do NOT attempt text extraction or translation yourself — that is the umrs-translator's
responsibility.

**Active i18n domains**: umrs-ls, umrs-state, umrs-logspace

---

## [2026-03-11] rust-developer → tech-writer: SEC pattern needs a dedicated page

**Status**: resolved — `docs/modules/patterns/pages/pattern-sec.adoc` written 2026-03-11; SEC block removed from CLAUDE.md (stub reference left pointing to the page); doc updated 2026-03-11 to reflect implementation in `umrs-platform/src/sealed_cache.rs`

The Sealed Evidence Cache (SEC) pattern was added to CLAUDE.md on 2026-03-11 as part of
the OS detection subsystem design. It is not yet implemented in the codebase, but the
design is stable enough to document.

Pattern definition is in CLAUDE.md under "Sealed Evidence Cache — SEC".

Key properties for the doc page:
- Sealing key is ephemeral (boot_id + process start time); never persisted; zeroized on drop
- Seal covers: cached data + TrustLevel + digest of the evidence chain
- TTL default: 30s
- FIPS systems: use FIPS-validated HMAC or disable caching
- Seal verification failure → discard cache, re-run pipeline, log anomaly

Primary application site: `umrs-platform` OS detection pipeline (expensive multi-phase
verification whose inputs change infrequently).

Connects to existing patterns: Zeroize (sealing key), Fail-Closed (seal failure),
Loud Failure (log anomaly), Provenance Verification (pipeline inputs).

---
