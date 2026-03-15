# Plan: `ai-transparency` Antora Module

**Goal**: G10 — AI Transparency
**Milestone**: M5 — AI Transparency
**Date**: 2026-03-15
**Status**: Draft — awaiting Jamie approval before execution
**Blocked by**: Antora doc restructure freeze (lift before executing)

---

## Why This Module Exists

UMRS is a high-assurance project. Auditors, adopters, and security reviewers must be able
to trace every substantive claim — in code comments, documentation, and design decisions —
back to its source. When AI agents participate in that work, the same traceability requirement
applies to the AI pipeline itself.

This module documents the AI methodology transparently. It is not a general introduction to
AI-assisted development. It is a specific, honest account of how the UMRS team uses AI: what
agents do, how they acquire knowledge, where that knowledge comes from, and how human judgment
remains the final authority.

**Auditor value**: An auditor seeing a NIST control citation in UMRS documentation can follow
the chain: the citation exists because a security-auditor agent found it; that agent's
knowledge came from a RAG corpus built from a specific NIST SP PDF; that PDF is checksummed
in `refs/manifest.md`. The chain is complete and reproducible.

**Adopter value**: Organizations considering a similar AI-assisted workflow can use this module
as a concrete reference. UMRS documents the full pipeline — not just "we used AI" but how the
pipeline is structured to maintain accuracy and traceability.

**Contributor value**: New contributors understand how the team works and can participate
without disrupting established workflows or knowledge provenance.

---

## Audience

| Reader | What they need | Priority |
|---|---|---|
| Security auditors | Understand the AI pipeline scope, review requirements, and knowledge provenance | High |
| Potential adopters | Evaluate whether this approach is sound and replicable | High |
| New contributors | Understand team structure, roles, and how to participate | Medium |
| Engineers curious about the methodology | Background on RAG, corpus familiarization, structured agent design | Medium |

---

## Scope Boundary

This module documents the AI **process** — methodology, workflow, knowledge sourcing, and
transparency justification. It does not document:

- AI-generated Rust code (that lives in crate source and rustdoc)
- AI-generated documentation content (that lives in its target module)
- The underlying Claude Code platform or Anthropic APIs (external)

---

## Relationship to Existing Content

The ROOT-level `ai-transparency.adoc` page (written in Phase 3) is a single-page summary.
It covers: why AI is used, agent roles table, what AI does not do, and an auditor section.
This module expands that summary into a full documentation set. The ROOT page becomes the
entry point that links here.

**Navigation change required** (after freeze lifts):
The ROOT nav entry `* xref:ai-transparency.adoc[AI in This Project]` becomes a cross-module
reference pointing to `ai-transparency:index.adoc`. The ROOT page itself either becomes a
redirect stub or is folded into the new module's index.

---

## Module Structure

### Antora module name: `ai-transparency`

**Location**: `docs/modules/ai-transparency/`

**antora.yml registration**: Add `modules/ai-transparency/nav.adoc` to `docs/antora.yml`
after the `glossary` nav entry (last in the list — this module is project-meta, not
subject-matter documentation).

**ROOT nav placement**: After `xref:glossary:index.adoc[Glossary]`, before Legal Notices.
Label: `AI in This Project`.

---

### Page List

All pages use `.adoc` format. File naming follows the project convention (lowercase, hyphens).

#### Group 1: Overview (Explanation — Diataxis)

| File | Title | Diataxis type | Priority |
|---|---|---|---|
| `index.adoc` | AI in This Project | Explanation / module index | Phase 1 |
| `why-transparency.adoc` | Why We Document the AI Pipeline | Explanation | Phase 1 |
| `what-ai-does-and-does-not-do.adoc` | What AI Does and Does Not Do | Explanation | Phase 1 |

#### Group 2: Team and Workflow (Explanation + Reference)

| File | Title | Diataxis type | Priority |
|---|---|---|---|
| `agent-roles.adoc` | Agent Roles and Responsibilities | Reference | Phase 1 |
| `workflow-mechanics.adoc` | How Work Flows Through the Team | Explanation | Phase 1 |
| `feedback-and-standing-rules.adoc` | How Feedback Becomes Permanent Rules | Explanation | Phase 2 |

#### Group 3: Knowledge Pipeline (Explanation + Procedure)

| File | Title | Diataxis type | Priority |
|---|---|---|---|
| `knowledge-pipeline.adoc` | How Knowledge Enters the System | Explanation | Phase 1 |
| `rag-collections.adoc` | RAG Collections and Their Sources | Reference | Phase 1 |
| `corpus-familiarization.adoc` | Corpus Familiarization — Building Active Knowledge | Explanation | Phase 2 |
| `knowledge-provenance.adoc` | Knowledge Provenance and Claim Traceability | Explanation | Phase 1 |

#### Group 4: Skills Catalog (Reference)

| File | Title | Diataxis type | Priority |
|---|---|---|---|
| `skills-catalog.adoc` | Skills Catalog | Reference | Phase 2 |

#### Group 5: For Auditors (Explanation)

| File | Title | Diataxis type | Priority |
|---|---|---|---|
| `auditor-guide.adoc` | AI Transparency — Auditor's Guide | Explanation | Phase 1 |

#### Group 6: Case Study (Tutorial-adjacent Explanation)

| File | Title | Diataxis type | Priority |
|---|---|---|---|
| `case-study-rmf-corpus.adoc` | Case Study: Building the RMF Methodology Corpus | Explanation | Phase 2 |

---

### Navigation (nav.adoc structure)

```asciidoc
* xref:index.adoc[AI in This Project]

* Overview
** xref:why-transparency.adoc[Why We Document the AI Pipeline]
** xref:what-ai-does-and-does-not-do.adoc[What AI Does and Does Not Do]

* Team and Workflow
** xref:agent-roles.adoc[Agent Roles and Responsibilities]
** xref:workflow-mechanics.adoc[How Work Flows Through the Team]
** xref:feedback-and-standing-rules.adoc[How Feedback Becomes Permanent Rules]

* Knowledge Pipeline
** xref:knowledge-pipeline.adoc[How Knowledge Enters the System]
** xref:rag-collections.adoc[RAG Collections and Their Sources]
** xref:corpus-familiarization.adoc[Corpus Familiarization]
** xref:knowledge-provenance.adoc[Knowledge Provenance and Claim Traceability]

* xref:skills-catalog.adoc[Skills Catalog]

* xref:auditor-guide.adoc[Auditor's Guide]

* xref:case-study-rmf-corpus.adoc[Case Study: RMF Methodology Corpus]
```

---

## Content Outlines

### `index.adoc` — AI in This Project

**Diataxis**: Explanation (module index)
**Audience**: All

This page is the entry point. It replaces (or demotes) the ROOT-level `ai-transparency.adoc`.

Content:
- One paragraph: what this module covers and why it exists
- Brief statement on the transparency requirement in high-assurance projects
- Navigation table: what each group of pages covers and who should read it
- Cross-reference to `auditor-guide.adoc` for auditors coming directly here
- Cross-reference to `ROOT:getting-started.adoc` for broader project orientation

---

### `why-transparency.adoc` — Why We Document the AI Pipeline

**Diataxis**: Explanation
**Audience**: Auditors, adopters

Content:
- The traceability argument: high-assurance documentation must be as auditable as the code
- Why "we used AI" is insufficient disclosure — scope, knowledge sources, review gates all matter
- The asymmetry between AI confidence and accuracy: RAG grounding as the mitigation
- The human-judgment requirement: no AI decision is final without engineer review
- Why this module is itself a compliance artifact — it demonstrates the project's commitment
  to the same evidence discipline it asks of the code

Control reference: NIST SP 800-53 SA-11 (Developer Security Testing and Evaluation),
AU-2 (Event Logging), SI-7 (Software, Firmware, and Information Integrity).

---

### `what-ai-does-and-does-not-do.adoc` — What AI Does and Does Not Do

**Diataxis**: Explanation
**Audience**: All (especially auditors and adopters)

This page is a structured expansion of the table already in `ROOT/pages/ai-transparency.adoc`.
It replaces that table with prose plus a table, and adds the rationale for each boundary.

Content:

**What AI does** (with rationale for each):
- Standards research and RAG-grounded citation identification
- Documentation drafting against an established architecture
- Code review for pattern compliance (missing `#[must_use]`, unsafe blocks, annotation gaps)
- Compliance annotation proposals (candidates, not final claims)
- Test case proposals for security-relevant code paths
- Structural planning for documentation and feature phases
- Acting as reviewer personas (new-engineer, security-auditor) to surface blind spots

**What AI does not do** (with rationale for each boundary):
- Make architectural decisions (the engineer decides; AI proposes)
- Author SELinux policy modules
- Make final compliance claims (AI identifies candidates; engineer confirms)
- Commit to the repository (hard rule — all commits are human)
- Access production systems or classified environments

**The review gate**: Every AI-generated artifact passes through engineer review before
acceptance. The commit record is the audit trail.

---

### `agent-roles.adoc` — Agent Roles and Responsibilities

**Diataxis**: Reference
**Audience**: Contributors, auditors

Content: A reference table expanded from the summary in `ROOT/pages/ai-transparency.adoc`.

For each agent: name, primary responsibility, typical outputs, what it defers to other agents,
what it does not do.

Agents to cover: `rust-developer`, `security-engineer`, `security-auditor`, `tech-writer`,
`senior-tech-writer`, `researcher`, `umrs-translator`, `changelog-updater`.

Also cover the communication channels each agent uses:
- Task board (actionable work items, status, hand-offs)
- Cross-team notes (`.claude/agent-memory/cross-team/notes.md` — contextual advisories)
- Doc-team feedback log (`.claude/agent-memory/doc-team/feedback.md`)
- Agent memory (`.claude/agent-memory/<agent>/MEMORY.md` — persistent per-agent state)

Note on specialization rationale: explain why role boundaries matter in a high-assurance
context — conflicting priorities dilute both, and shallow reasoning across too many domains
misses critical concerns.

---

### `workflow-mechanics.adoc` — How Work Flows Through the Team

**Diataxis**: Explanation
**Audience**: Contributors, curious engineers

Content:

**The review pipeline** — mirror of the human engineering review chain:
1. Developer implements
2. Security review (trust, threats, policy)
3. Architecture review (system coherence, Jamie)
4. Documentation review (clarity, completeness)

**How work enters the system**:
- Jamie identifies a gap or new feature offline
- Research phase: researcher agent acquires authoritative sources
- RAG ingestion: sources chunked and indexed in ChromaDB
- Corpus familiarization: target agent reads the corpus, builds active knowledge artifacts
- Active work: agent produces grounded output
- Review and acceptance: Jamie reviews; engineer commits

**The inbox locations**:
- `docs/new_stuff/` — research material waiting for placement in the docs tree
- `.claude/jamies_brain/` — Jamie's private research area (not an agent input queue)
- `.claude/plans/` — the active work queue (pending, in-progress, completed/)

**Plan lifecycle**: pending → in-progress → completed/archived.
Plans reference ROADMAP goals (G1–G10) to justify existence.

**Task board**: cross-agent actionable items, status tracking, hand-off notes.

---

### `feedback-and-standing-rules.adoc` — How Feedback Becomes Permanent Rules

**Diataxis**: Explanation
**Audience**: Contributors, auditors interested in process quality

Content:
- The feedback loop: engineer corrections during work become standing rules
- Where rules live: `.claude/rules/` (project-level, loaded into every agent context)
- Where per-agent memory lives: `.claude/agent-memory/<agent>/MEMORY.md`
- Example: how a clippy style preference became a rule in `rust_design_rules.md`
- Example: how a doc navigation problem became a standing nav-scrutiny rule
- Why this matters: without persistent rules, agents repeat the same mistakes; the loop
  closes the gap between correction and prevention
- Shared feedback channel: `doc-team/feedback.md` — used by tech writers to track
  decisions across sessions

---

### `knowledge-pipeline.adoc` — How Knowledge Enters the System

**Diataxis**: Explanation
**Audience**: All (especially auditors and adopters)

This is one of the most important pages in the module. It explains the full pipeline from
knowledge gap to grounded agent output.

Content structure:

**The problem with raw AI knowledge**:
- Large language models have a training cutoff; standards evolve
- Models can confabulate citations — a plausible-sounding NIST control number that
  does not correspond to a real control
- Without grounding, confidence in AI-produced security claims is low

**The solution: the RAG pipeline**:
- Jamie identifies a knowledge gap (e.g., "the security-auditor agent does not know
  current RMF assessment methodology deeply enough")
- Researcher agent acquires authoritative source documents (NIST SPs, FIPS standards,
  DISA STIGs, CMMC guides) from approved sources
- Sources are checksummed and recorded in `refs/manifest.md`
- Ingestion: `ingest.py` chunks documents and stores them in ChromaDB
- At query time, `query.py` performs semantic search and returns relevant chunks with
  source attribution

**Corpus familiarization** (brief summary, full treatment in its own page):
- After ingestion, the target agent runs a structured familiarization pass
- This produces four knowledge artifacts loaded into always-on context:
  concept index, cross-reference map, style decision record, term glossary
- Without familiarization, the agent knows what to retrieve but not what to ask for

**The result**:
- When an agent produces a NIST citation, it traces to a specific chunk in a specific
  document that is itself checksummed in `refs/manifest.md`
- The chain: agent output → RAG chunk → source document → manifest entry → SHA-256

Include a Mermaid flowchart of the full pipeline (gap → research → ingest → familiarize
→ work → review → commit).

---

### `rag-collections.adoc` — RAG Collections and Their Sources

**Diataxis**: Reference
**Audience**: All (especially auditors)

Content: A reference table of all current RAG collections. For each collection:
- Collection name (as used in `rag-query`)
- Document count / chunk count
- What it covers (one sentence)
- Source documents (titles, publication dates, where obtained)
- Primary use cases — which agents query it and for what

Collections to document (from current cross-team notes):

| Collection | Key content |
|---|---|
| `kernel-docs` | Linux kernel documentation tree |
| `access-control` | Bell-LaPadula, Biba, SELinux, capabilities, POSIX ACL, ZTA |
| `selinux-notebook` | SELinux policy, TE, MLS/MCS, labeling, xattrs |
| `cmmc` | CMMC Final Rule (32 CFR 170) + Assessment Guide L2 v2.13 |
| `dod-5200` | DoD 5200.01 (Info Security Program) + DoDI 5200.48 (CUI policy) |
| `nist` | NIST SP 800-53r5, 800-171r2/r3, 800-171Ar3, 800-218 SSDF, FIPS 140-2/3 |
| `nist-pqc` | FIPS 203/204/205 + 11 web articles on PQC migration |
| `rmf-methodology` | NIST SP 800-53A r5, 800-37 r2, 800-30 r1, 800-39 |
| `doc-structure` | Diataxis, Antora, Red Hat modular docs, Google style, GitLab docs |
| `rustdoc-book` | Rustdoc reference (doc comment syntax, intra-doc links) |
| `asciidoctor-ref` | AsciiDoc syntax and document structure |
| `accreditation-artifacts` | FedRAMP playbooks, SSP/SAP/SAR templates (staged, pending download) |

Add a note on the three-layer reference architecture:
- RAG database: searchable, semantic
- Source documents (`.claude/references/<collection>/`): original files, readable
- Official refs (`refs/`): checksummed, manifested, authoritative archive

---

### `corpus-familiarization.adoc` — Corpus Familiarization

**Diataxis**: Explanation
**Audience**: Contributors, curious engineers

Content:
- Why RAG is pull-only and why that creates a blind-spot problem
- The corpus familiarization skill: what it does and when it runs
- The four knowledge artifacts and why each exists:
  1. Concept index — what the agent knows and where to look
  2. Cross-reference map — where documents agree, conflict, or defer to each other
  3. Style decision record — project-specific resolutions to documentation tensions
  4. Term glossary — canonical terminology from authoritative sources
- Where artifacts live: `.claude/knowledge/<collection>/`
- When to re-run: after any significant corpus update or new collection
- Example: the security-auditor agent after RMF corpus ingestion (2026-03-15)

This page explains the mechanism that turns passive RAG retrieval into active, reliable
agent knowledge.

---

### `knowledge-provenance.adoc` — Knowledge Provenance and Claim Traceability

**Diataxis**: Explanation
**Audience**: Auditors (primary), adopters

Content:
- The traceability chain for a security claim in UMRS documentation:
  1. A doc page contains a NIST control citation (e.g., `NIST SP 800-53 AC-4`)
  2. That citation was proposed by an agent during documentation work
  3. The agent queried the `nist` RAG collection and retrieved a matching chunk
  4. That chunk came from `refs/nist/sp800-53r5.pdf`
  5. That PDF has a SHA-256 checksum in `refs/manifest.md`
  6. The engineer reviewed the citation before accepting it in a git commit

- The manifest as an audit artifact: `refs/manifest.md` records every reference
  document — title, version, download date, source URL, SHA-256. This makes the
  knowledge base inspectable and verifiable.

- Where the chain can be inspected:
  - `refs/manifest.md` — document inventory
  - `.claude/references/<collection>/` — source documents used for ingestion
  - Git history — engineer acceptance of specific content

- Limitations and honest gaps:
  - RAG retrieval is semantic, not exact — a retrieved chunk may be adjacent to the
    right answer rather than the answer itself; engineer review closes this gap
  - The training data of the underlying model (Claude) is not fully auditable;
    RAG grounding reduces but does not eliminate model-level knowledge uncertainty
  - Claims that have no RAG grounding path are explicitly flagged by agents as
    requiring manual verification

- Why this matters for CUI and DoD contexts:
  Reference NIST SP 800-53 AU-10 (Non-Repudiation) as the applicable control
  principle — the documentation record is itself an artifact of the security posture.

---

### `skills-catalog.adoc` — Skills Catalog

**Diataxis**: Reference
**Audience**: Contributors, engineers

Content: A reference table of all skills. For each skill:
- Name (as invoked)
- Purpose (one sentence)
- When it triggers (conditions or phrases that activate it)
- Primary user (which agents use it)
- Output or effect

Skills to document:

| Skill | Purpose |
|---|---|
| `rag-query` | Semantic search over the reference library |
| `rag-ingest` | Ingests new or updated reference documents into ChromaDB |
| `corpus-familiarization` | Builds active knowledge from newly ingested collections |
| `doc-arch` | Searches the doc-structure collection for documentation guidance |
| `new-crate` | Scaffolds a new Rust crate in the workspace |
| `project-cleanup` | Removes build artifacts and temporary files |
| `french-lookup` | Looks up translated strings in the French corpus |
| `rust-uml` | Generates UML diagrams from Rust source code |
| `seccomp-engineering` | Generates seccomp profiles for Rust binaries |
| `changelog-updater` | (Agent skill) Structured changelog maintenance |

For each skill, note whether it produces a persistent artifact (e.g., corpus
familiarization writes to `.claude/knowledge/`), modifies the workspace, or only
produces transient output.

---

### `auditor-guide.adoc` — Auditor's Guide

**Diataxis**: Explanation
**Audience**: Security auditors (primary)

Content:
- What AI involvement means for audit scope: AI accelerates but does not replace
  human judgment; the auditable artifacts are the same
- Where to look for evidence of AI scope:
  - `ROOT/pages/ai-transparency.adoc` — high-level summary (single page)
  - This module — full methodology
  - `.claude/rules/` — standing rules that govern agent behavior
  - `.claude/agent-memory/` — per-agent persistent state (not a production artifact,
    but shows how agents evolve their knowledge)
- The commit record as the authoritative audit trail: AI does not commit; every
  incorporated artifact went through engineer review
- Compliance control citations in UMRS documentation:
  - Citations are identified by AI agents and confirmed by engineer review
  - Citation format follows `.claude/rules/rust_design_rules.md` (Citation Format Rule)
  - If an auditor finds an incorrect citation, they should flag it — corrections will
    be applied and the correction process is itself transparent
- How to query the RAG for source verification:
  - Brief instruction: RAG is not externally accessible; auditors can request that
    specific claims be traced to their source document and section on request
- The `refs/manifest.md` as a reference inventory: all source documents are recorded
  with SHA-256 checksums; auditors can verify document integrity

Control citations for this page:
- NIST SP 800-53 SA-11 (Developer Security Testing and Evaluation)
- NIST SP 800-53 AU-10 (Non-Repudiation)
- NIST SP 800-53 SI-7 (Software, Firmware, and Information Integrity)

---

### `case-study-rmf-corpus.adoc` — Case Study: Building the RMF Methodology Corpus

**Diataxis**: Explanation (case study format)
**Audience**: Adopters, contributors

This page documents a specific, real pipeline run as a concrete example of the methodology.

Content:
- The gap identified: the security-auditor agent lacked deep knowledge of NIST RMF
  assessment procedures (SP 800-53A, 800-37, 800-30, 800-39)
- The research phase: researcher agent located authoritative PDFs from `nvlpubs.nist.gov`
- The manifest update: SHA-256 checksums recorded, source URLs documented
- The ingestion: `ingest.py` processed the four PDFs into the `rmf-methodology` collection
  (1,132 chunks)
- The familiarization: corpus-familiarization skill run on the collection; four knowledge
  artifacts written to `.claude/knowledge/rmf-methodology/`
- The result: security-auditor agent can now classify its review activities by RMF method
  type (Examine / Interview / Test), cite specific assessment procedures, and identify
  evidence gaps rather than just annotation gaps
- The cross-team notification: researcher posted to `cross-team/notes.md`, security-auditor
  alerted to run a feedback pass on active plans using new methodology grounding

This case study makes the abstract pipeline concrete and shows the full chain from
knowledge gap to grounded agent capability. Date of actual pipeline run: 2026-03-15.

---

## Cross-References to Other Modules

| This page links to | Target module/page | Reason |
|---|---|---|
| `index.adoc` | `ROOT:getting-started.adoc` | Project orientation for new readers |
| `index.adoc` | `ROOT:what-is-umrs.adoc` | Context for what UMRS is |
| `agent-roles.adoc` | `devel:index.adoc` | Developer guide for contributors |
| `agent-roles.adoc` | `patterns:index.adoc` | Pattern library (rust-developer agent output) |
| `knowledge-provenance.adoc` | `reference:compliance-frameworks.adoc` | Framework context for control citations |
| `auditor-guide.adoc` | `ROOT:getting-started.adoc` (audit path) | Auditor orientation |
| `auditor-guide.adoc` | `reference:compliance-frameworks.adoc` | Framework inventory |
| `skills-catalog.adoc` | `devel:index.adoc` | Skills used in development workflow |
| `rag-collections.adoc` | `reference:compliance-frameworks.adoc` | Source frameworks for RAG content |
| `what-ai-does-and-does-not-do.adoc` | `ROOT:ai-transparency.adoc` (ROOT stub) | The ROOT page becomes the short summary |

---

## Phasing

### Phase 1 — Core (write first; highest auditor/adopter value)

Priority: these pages answer the most important questions and unblock cross-references.

1. `index.adoc` — module entry point
2. `why-transparency.adoc` — the rationale
3. `what-ai-does-and-does-not-do.adoc` — scope definition
4. `agent-roles.adoc` — team structure reference
5. `workflow-mechanics.adoc` — how work flows
6. `knowledge-pipeline.adoc` — the RAG pipeline explanation (includes Mermaid diagram)
7. `rag-collections.adoc` — reference table of all collections
8. `knowledge-provenance.adoc` — traceability chain
9. `auditor-guide.adoc` — auditor-specific entry point

Deliverables from Phase 1 also include:
- `nav.adoc` wired for Phase 1 pages
- Registration in `docs/antora.yml`
- ROOT nav updated to point to new module index
- `make docs` clean (zero new errors)

### Phase 2 — Depth (write after Phase 1 is complete and stable)

1. `feedback-and-standing-rules.adoc` — feedback loop mechanics
2. `corpus-familiarization.adoc` — familiarization skill explanation
3. `skills-catalog.adoc` — full skills reference
4. `case-study-rmf-corpus.adoc` — concrete pipeline example (best written while the
   2026-03-15 pipeline run is fresh; may move to Phase 1 if Jamie wants it sooner)

### What needs more project maturity before writing

- An "adoption guide" version of this module (for other projects wanting to replicate
  the methodology) — valuable but requires the methodology to be stable across several
  more cycles first
- Formal evidence mapping (AI pipeline output → NIST AU controls) — premature until
  Phase 2 is complete

---

## Unique Value Proposition

Most AI-in-software disclosures say: "We used AI tools. All output was reviewed."

This module says: here is how the pipeline works, what sources the agents read, how
knowledge is grounded in checksummed reference documents, how human judgment gates every
accepted artifact, and how standing rules prevent repeated mistakes.

For auditors: this is the documentation equivalent of a reproducible build. You can
inspect the knowledge sources the same way you can inspect the code dependencies.

For adopters: this is a concrete, replicable methodology — not a vague claim that AI
helps. The RAG pipeline, corpus familiarization pattern, and standing-rules mechanism
are all directly reproducible.

For contributors: this is the onboarding documentation that explains not just what agents
do, but why the design is structured to maintain accuracy and traceability in a domain
(high-assurance security) where errors have real consequences.

---

## Execution Notes

**Do not execute until the Antora restructure freeze is lifted.**

When the freeze lifts:
1. Create `docs/modules/ai-transparency/` with `nav.adoc` and `pages/` directory
2. Write Phase 1 pages in the order listed
3. Add `modules/ai-transparency/nav.adoc` to `docs/antora.yml` (after glossary)
4. Update `docs/modules/ROOT/nav.adoc` to point to `ai-transparency:index.adoc`
5. Run `make docs 2>&1` after each page — fix immediately if broken
6. Confirm ROOT page (`ai-transparency.adoc`) status with Jamie:
   - Option A: demote to a short redirect stub pointing to the module
   - Option B: keep as a standalone summary; module is "more depth" rather than a replacement
7. Write Phase 2 pages

**Source material for writing**:
- `.claude/team-collaboration.md` — authoritative on workflow, roles, Jamie's working style
- `.claude/ROADMAP.md` — G10/M5 goals
- `docs/modules/ROOT/pages/ai-transparency.adoc` — existing Phase 3 page (expand, do not duplicate)
- Cross-team notes entries from 2026-03-12 and 2026-03-15 — RAG collections and workflow details
- Skill SKILL.md files in `.claude/skills/` — authoritative on what each skill does
- Agent `.md` files in `.claude/agents/` — authoritative on agent roles
- `refs/manifest.md` — source for the RAG collections reference table

**MEMORY.md update needed**: After execution, add `ai-transparency` module to the module
list in MEMORY.md and note the ROOT page disposition decision.
