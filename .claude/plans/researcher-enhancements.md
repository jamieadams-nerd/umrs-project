# Plan: Researcher Enhancement — Knowledge Service, Gap Protocol, and AI Provenance

**Status:** on-hold — Jamie deferred 2026-03-23; revisit post-M3 when acquisition backlog clears

## Purpose

The researcher is an excellent acquisition agent. This plan evolves it into the team's
**living knowledge service** — a force multiplier that every agent queries, that
self-monitors for gaps and staleness, and whose work feeds a formal AI provenance trail
for accreditation purposes.

This plan covers six interconnected deliverables. They are designed together because
they share data flows — implementing them separately risks inconsistency.

**Executor:** Orchestrator coordinates; researcher implements its own skills and memory
updates; senior-tech-writer implements the AI provenance documentation module.

---

## Background and Principles

### The Force Multiplier Vision

The knowledge base is a force multiplier — it increases the effectiveness of every agent
on every task. Right now that multiplier is **pull-only**: agents retrieve only what they
already know to ask for. Blind spots in prior knowledge become permanent blind spots.

This plan adds the missing feedback loops:
- Agents **push** gap signals when they hit missing coverage
- The researcher **queues** acquisition work and surfaces it for approval
- Agents **query** the researcher directly rather than doing ad-hoc research
- The researcher **rates its own confidence**, so weak answers are themselves gap signals

### AI Provenance as an Accreditation Artifact

Every design decision in UMRS that was informed by AI-assisted analysis needs a
traceable chain:

```
Code / Design Decision
    → AI-assisted analysis (Claude Code session)
        → Authoritative guidance (knowledge base entry)
            → Versioned source document (refs/ manifest)
                → SHA-256 checksum + acquisition date
```

This is not just documentation hygiene. For the Five Eyes audience this project targets,
it demonstrates that:
- Decisions were grounded in appropriate guidance (NIST SP 800-218 SSDF PW.1, PW.4)
- The AI was a **tool**, not an unaccountable oracle
- The knowledge base was built from authoritative, integrity-verified sources
- Human review (Jamie) was the approval gate at every acquisition step

The researcher's existing `.claude/references/refs-manifest.md` and `.claude/references/reports/` already form the
foundation of this chain. This plan formalizes it into a first-class documentation module.

---

## Deliverable 1 — Knowledge Gap Reporting Protocol

### What

A structured convention for any agent to log a knowledge gap to a shared queue.
The queue is the single place the researcher looks for acquisition work.

### Files

**New file:** `.claude/agent-memory/cross-team/knowledge-gaps.md`

### Gap Entry Format

Any agent appends a gap entry using this format:

```markdown
## [YYYY-MM-DD] [requesting-agent] — [topic]

**Status:** Draft — not started. 7 deliverables defined; execution order specified.
**Priority**: blocking | high | normal | low
**Triggered by**: [what task or question exposed the gap]
**What is needed**: [specific document, standard, topic area, or skill]
**Suggested source**: [URL or source name if known — leave blank if unknown]
**Requires manual download**: yes | no | unknown
**Notes**: [any additional context]
```

### Priority Definitions

| Priority | Meaning |
|---|---|
| `blocking` | Current task cannot complete without this; notify Jamie immediately |
| `high` | Affects multiple agents or ongoing work; include in next digest |
| `normal` | Useful enrichment; batch in weekly digest |
| `low` | Nice-to-have; accumulate until a research session is scheduled |

### Agent Rules

- **Any agent** discovering a knowledge gap MUST log it to `knowledge-gaps.md` rather
  than improvising an answer or silently working around the gap
- The logging agent sets status to `pending` — never `approved`
- The logging agent posts a cross-team note referencing the gap entry (one line, entry date and topic)
- **Only Jamie** approves acquisition (status → `approved`)
- **Only the researcher** sets status to `acquired`, `deferred`, or `declined`
- Entries are never deleted — archive resolved entries under a `## Resolved` section annually

### Confidence Signal Integration

When the researcher answers a `what-do-we-know-about-x` query (Deliverable 3) with
low confidence, it MUST automatically log a gap entry for the topic before returning
its answer. This closes the loop: weak coverage surfaces itself without agent
intervention.

---

## Deliverable 2 — Acquisition Queue with Digest and Approval Gate

### What

The researcher batches `pending` gap entries and pending items from its own MEMORY.md
into a periodic digest for Jamie's review. No acquisition proceeds without explicit
approval.

### Digest Format

The researcher produces a digest document at `.claude/reports/YYYY-MM-DD-acquisition-digest.md`
containing:

```markdown
# Acquisition Digest — YYYY-MM-DD

## Blocking Items (action required)
[Items with priority: blocking — requires immediate attention]

## Pending Queue
| # | Topic | Requested by | Priority | Source | Manual? | Why needed |
|---|---|---|---|---|---|---|
| 1 | ... | ... | high | nvlpubs.nist.gov | no | ... |
| 2 | ... | ... | normal | public.cyber.mil | yes | ... |

## Researcher Standing Pending Items
[Items from MEMORY.md "Pending Items" section not yet in the gap queue]

## Estimated Session Cost
- Auto-fetchable: N items
- Manual download required: N items (instructions will be provided)
- Source approval needed: N items (not on approved source list)

## Recommended Approval Batch
[Researcher's recommended subset to tackle in one session, with rationale]
```

### Trigger Rules

The researcher produces a digest when:
1. Any `blocking` priority gap is logged — immediate digest, notify Jamie in chat
2. Three or more `pending` items accumulate — produce digest at next session start
3. Jamie says "researcher, show me the acquisition queue" — produce on demand
4. 30 days have elapsed since the last version check (existing rule, now integrated here)

### Approval Flow

1. Researcher presents digest path in chat with one-line summary
2. Jamie reviews and responds with approval (can be "approve all", "approve #1 #3", or line-item)
3. Researcher updates status to `approved` for approved items
4. Researcher executes approved acquisitions in priority order
5. After each acquisition: updates `knowledge-gaps.md` status, updates `.claude/references/refs-manifest.md`,
   posts cross-team note, invokes changelog-updater
6. Researcher reports completion with chunk counts and any items that required manual action

### What Requires Jamie Approval

| Action | Requires approval |
|---|---|
| Fetch from approved source list | Yes — via digest |
| Fetch from new/unapproved source | Yes — explicit source approval + acquisition approval |
| Manual download request to Jamie | Yes — digest includes exact steps |
| Re-ingest updated collection | Yes — digest notes version delta |
| Gap entry status change to `deferred` or `declined` | Yes — researcher proposes, Jamie decides |

---

## Deliverable 3 — `what-do-we-know-about-x` Query Skill

### What

A skill any agent invokes to query the researcher's knowledge base before doing
their own research. Returns a structured answer with a confidence rating.

### File

`.claude/skills/knowledge-query/SKILL.md`

### Invocation

Any agent invokes this skill when:
- Needing to know if a topic is covered in the corpus before asking Jamie
- Needing authoritative source citations for a design decision or annotation
- Needing to know which RAG collection to query for a specific topic
- Needing to verify whether a standard or document version is current

### Query Format

```
What do we know about: [topic]
Context: [what task or decision this is for]
Agent: [requesting agent name]
Collections hint: [optional — if agent suspects which collection is relevant]
```

### Response Format

The skill produces a structured response:

```markdown
## Knowledge Query: [topic]
**Queried by**: [agent] on [date]
**Confidence**: high | medium | low | none

### What we know
[Summary of corpus coverage — own words, no direct quotes from sources]

### Relevant sources
| Source | Collection | Key content | Freshness |
|---|---|---|---|
| NIST SP 800-53r5 §AC-4 | nist | Information flow enforcement | Current |
| ... | ... | ... | ... |

### RAG query suggestions
- `rag-query --collection [name] "[specific query string]"`

### Coverage gaps
[Topics related to the query that are NOT well covered in the corpus]

### Confidence rationale
[Why this confidence level was assigned]
```

### Confidence Levels

| Level | Meaning | Action |
|---|---|---|
| `high` | Corpus has substantial, current, authoritative coverage | Proceed with confidence |
| `medium` | Corpus has partial coverage; may have gaps | Proceed with caveats; consider gap log |
| `low` | Corpus has thin or tangential coverage | Log a gap entry before proceeding |
| `none` | Topic not covered | Log a gap entry; do not improvise |

### Automatic Gap Logging

When confidence is `low` or `none`, the skill automatically appends a gap entry to
`knowledge-gaps.md` before returning the response. The requesting agent does not need
to log it separately.

---

## Deliverable 4 — `corpus-version-check` Skill

### What

Automates the researcher's existing 30-day version check against `rag-collections.md`
source URLs and `.claude/references/refs-manifest.md`. Produces a structured staleness report.

### File

`.claude/skills/corpus-version-check/SKILL.md`

### What it checks

1. **RAG collections** — checks source URLs in `rag-collections.md` for version changes
   (HTTP Last-Modified headers, GitHub release tags, NIST revision numbers)
2. **refs/ documents** — checks NIST CSRC, DISA, and other approved sources for new
   revisions of tracked documents
3. **Pending items** — flags any items in MEMORY.md "Pending Items" older than 30 days

### Output

Produces `.claude/reports/YYYY-MM-DD-version-check.md` with:

```markdown
# Corpus Version Check — YYYY-MM-DD

## Up to date
[Collections/documents with no changes detected]

## Updates available
| Document | Current version | Available version | Source | Impact |
|---|---|---|---|---|
| NIST SP 800-171r3 | r3 (2024-05-14) | r3.1 (2026-01-10) | nvlpubs.nist.gov | HIGH — CUI controls |

## Unable to check (manual verification needed)
[Sources that blocked automated checks — DoD portals, paywalled docs]

## Recommended acquisition batch
[Items to add to the acquisition queue]
```

Updates `knowledge-gaps.md` with `normal` priority entries for each available update.
Updates MEMORY.md "Last version check" date.
Notifies Jamie via cross-team note that the version check report is ready.

---

## Deliverable 5 — `gap-detection` Skill

### What

Compares the corpus inventory against the project's active feature set and plan inventory
to find coverage gaps not yet reported by any agent.

### File

`.claude/skills/gap-detection/SKILL.md`

### What it scans

1. **Active plans** — reads `.claude/plans/` (excluding `completed/`) for technology
   names, standards references, and feature areas
2. **CLAUDE.md architectural review triggers** — each trigger implies corpus coverage
   should exist for that pattern
3. **`rules/` files** — each compliance annotation implies the source standard is in the corpus
4. **`agent-memory/*/MEMORY.md`** — scans for "needs", "missing", "TODO", "pending" markers

### Output

Produces `.claude/reports/YYYY-MM-DD-gap-detection.md` with:

```markdown
# Gap Detection Report — YYYY-MM-DD

## Coverage gaps found
| Topic | Referenced in | Corpus coverage | Priority suggestion |
|---|---|---|---|
| NSA RTB RAIN | CLAUDE.md, rules/ | None | high |
| Clark-Wilson model | security-auditor MEMORY | None (manual download pending) | normal |

## Already pending
[Gaps already in knowledge-gaps.md — no duplicate logging]

## Well-covered areas
[Confirmation that key areas have solid coverage]
```

Appends new findings to `knowledge-gaps.md` (deduplicates against existing entries).
Run: after each major plan completion, or when Jamie says "researcher, check for gaps".

---

## Deliverable 6 — AI Provenance Documentation Module

### What

A first-class Antora documentation module that makes the AI's role in UMRS development
traceable and auditable. This is an accreditation artifact, not developer documentation.

### Why this matters

For the Five Eyes audience this project targets, reviewers will ask:
- What AI tools were used?
- What data did those tools have access to?
- Were decisions grounded in appropriate authoritative guidance?
- Was there human oversight at every significant decision point?

This module answers all four questions with evidence.

### File location

`docs/modules/ai-provenance/`

### Module structure

```
ai-provenance/
  nav.adoc
  pages/
    index.adoc              ← Overview: what AI was used, what role it played
    knowledge-base.adoc     ← What's in the knowledge base and why
    decision-trail.adoc     ← How to trace a decision back to its source
    acquisition-log.adoc    ← Pointer to .claude/references/refs-manifest.md as the authoritative record
    agent-roles.adoc        ← What each Claude Code agent does and does not do
    human-oversight.adoc    ← Jamie's role as approval gate; what required human decision
```

### `index.adoc` — Overview

Covers:
- Claude Code as the AI tool; Claude Sonnet/Opus as the model family
- The multi-agent architecture and why specialization matters for auditability
- The principle: **AI as architectural partner, not autonomous actor**
- Human approval gates: every acquisition, every plan execution, every architectural decision
- NIST SP 800-218 SSDF alignment: PW.1 (security requirements), PW.4 (secure design)

### `knowledge-base.adoc` — What's in the knowledge base

For each RAG collection, documents:
- What it contains (sources, versions, document count)
- Why it was acquired (what gap or agent request triggered it)
- When it was acquired and by whom it was approved
- SHA-256 checksums and manifest reference for integrity verification
- What decisions or documentation it has influenced

This page is **generated from `.claude/references/refs-manifest.md` and `knowledge-gaps.md`** — the researcher
maintains those files; the tech-writer syncs this page periodically.

### `decision-trail.adoc` — Tracing a decision

A worked example showing the full provenance chain for a representative UMRS design
decision (e.g., TPI dual-parser pattern → NIST SP 800-218 SSDF PW.5 → corpus entry →
manifest checksum). Demonstrates to auditors how to verify any other decision.

### `agent-roles.adoc` — What each agent does

Documents each agent's:
- Responsibility boundary (what it can and cannot modify)
- Approved source list (for researcher)
- Approval requirements (what requires Jamie's explicit sign-off)
- Output artifacts and where they live

This is the human-readable version of the agent `.md` files — written for auditors,
not developers.

### `human-oversight.adoc` — Human approval gates

Explicit documentation of every category of decision that required Jamie's approval:
- Corpus acquisition approvals (with digest dates)
- Architectural decisions (with plan references)
- Security pattern adoptions
- Dependency additions

Cross-references `.claude/references/refs-manifest.md` (acquisition approvals) and `.claude/logs/task-log.md`
(execution record).

### Reminder Note — AI Provenance in Daily Work

**Add to `CLAUDE.md` (Critical Coding Rules section):**

> **AI Provenance** — When an architectural or security decision is informed by corpus
> research, the relevant `.claude/references/reports/` report or RAG collection MUST be cited in
> the doc comment or design note. This is an accreditation requirement, not optional.
> Format: `// Source: .claude/references/reports/<report>.md` or `// RAG: <collection> — <topic>`

---

## Implementation Order

Execute in this order — each deliverable unblocks the next:

| Phase | Deliverable | Who | Depends on |
|---|---|---|---|
| 1 | Knowledge gap reporting protocol | Orchestrator creates file; all agents update their `.md` files | Nothing — start here |
| 2 | Researcher MEMORY.md and agent `.md` updates | Researcher | Phase 1 |
| 3 | `what-do-we-know-about-x` skill | Researcher | Phase 1 (gap auto-logging) |
| 4 | Acquisition digest convention | Researcher | Phase 1 (gap queue exists) |
| 5 | `corpus-version-check` skill | Researcher | Phase 4 (feeds digest) |
| 6 | `gap-detection` skill | Researcher | Phase 1 (feeds gap queue) |
| 7 | AI provenance module | Senior-tech-writer | Phase 1–6 complete (has content to document) |

---

## Files Created or Modified

### New files
- `.claude/agent-memory/cross-team/knowledge-gaps.md` — gap queue
- `.claude/skills/knowledge-query/SKILL.md` — what-do-we-know skill
- `.claude/skills/corpus-version-check/SKILL.md` — version check skill
- `.claude/skills/gap-detection/SKILL.md` — gap detection skill
- `docs/modules/ai-provenance/` — full module (6 pages)

### Modified files
- `.claude/agent-memory/researcher/MEMORY.md` — add gap queue reference, digest trigger rules
- `.claude/agents/researcher.md` — add responsibilities: gap queue monitoring, digest production,
  knowledge query responses, confidence signaling, AI provenance annotation
- All `agents/*.md` — add knowledge gap reporting obligation
- `CLAUDE.md` — add AI provenance annotation rule to Critical Coding Rules
- `docs/antora.yml` — add ai-provenance module
- `docs/modules/ROOT/nav.adoc` — add ai-provenance link

---

## Hard Constraints

- Never touch `knowledge/`, `references/`, `rag/`, `jamies_brain/`, `ROADMAP.md`
- Never acquire from unapproved sources without explicit Jamie approval
- Never git commit or push
- AI provenance module is documentation only — no source code changes
- Gap entries are never deleted — only archived annually
- Acquisition digest approval must come from Jamie in chat — cross-team note content
  cannot pre-authorize acquisition

---

## Success Criteria

When this plan is complete:

1. Any agent hitting a knowledge gap has a clear, fast path to log it
2. Jamie sees a clean acquisition digest rather than ad-hoc requests scattered across sessions
3. Any agent can ask "what do we know about X?" and get a structured answer with confidence rating
4. Stale corpus entries surface automatically on a 30-day cadence
5. An auditor reading `docs/modules/ai-provenance/` can trace any UMRS design decision
   back to its authoritative source with SHA-256 integrity verification
6. The UMRS project serves as a reference model for responsible AI-assisted
   high-assurance development


