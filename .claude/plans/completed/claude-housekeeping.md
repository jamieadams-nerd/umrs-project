# Plan: `.claude/` Housekeeping — Prune, Merge, Distill

**Status:** Completed — 2026-03-18. All three phases executed. 37 findings audited, 2,149 lines of context reduced. Report: `.claude/reports/2026-03-18-housekeeping-audit.md`

## Purpose

Over time, gaps in `CLAUDE.md` and agent `.md` files are patched through additions to
`agent-memory/*/MEMORY.md` and appended blocks in `CLAUDE.md`. These accumulate as
contradictions, duplicates, and misplaced content that slow agent context loading and
introduce ambiguity. This plan drives a periodic housekeeping pass to restore clean,
authoritative, non-redundant instruction files.

This is a **recurring maintenance task** — not a one-shot operation. Re-run whenever
`CLAUDE.md` or any `MEMORY.md` has grown noticeably since the last pass.

**Executor:** Orchestrator (Claude Code, no specialized agent needed)
**Role:** Technical editor only — no source code is touched.

---

## Scope

### In Scope

| Path | Action |
|---|---|
| `CLAUDE.md` | Refactor — slim down, extract misplaced sections, fix contradictions |
| `rules/*.md` | Audit against `CLAUDE.md` for overlap and contradiction |
| `agents/*.md` | Prune rules now covered by `rules/`; align to agent role boundary |
| `agent-memory/*/MEMORY.md` | Promote rule-like content up; retain project-specific learned context only |
| `agent-memory/*/*` (non-MEMORY) | Review — promote, archive, or retain as project context |
| `team-collaboration.md` | Absorb Agent Directory table from `CLAUDE.md` if duplicated there |
| `ARCHITECTURE.md` | **Create if it does not exist** — extract workspace layout, crate dependency rules, module maps, and architectural review triggers from `CLAUDE.md` |

### Out of Scope — Do Not Touch

- `knowledge/` — corpus familiarization artifacts, curated and stable
- `references/` — third-party standards documents
- `rag/` — RAG pipeline data
- `plans/` — project planning artifacts
- `plans/completed/` — historical record
- `reports/` — audit and review reports
- `jamies_brain/` — Jamie's scratch notes
- `ROADMAP.md` — Jamie's project roadmap, not agent instructions
- `CHANGELOG.md` — structured changelog, not instructions
- `skills/` — already clean and portable; review only if a skill duplicates a `CLAUDE.md` rule

---

## Phase 1 — Audit (Read-Only)

**Do not edit any file during this phase.**

Read files in this order:

1. `CLAUDE.md`
2. `rules/*.md` (all files)
3. `agents/*.md` (all files)
4. `agent-memory/*/MEMORY.md` (all agents)
5. `agent-memory/*/*` (non-MEMORY files, skim for rule-like content)
6. `team-collaboration.md`

For each file, produce a reconciliation table with four columns:

| Column | Meaning |
|---|---|
| `KEEP` | Content is in the right place, no changes needed |
| `MOVE-TO` | Content belongs in a different file — specify target |
| `MERGE-WITH` | Content duplicates something elsewhere — specify the canonical location |
| `DROP` | Content is stale, superseded, or no longer applicable |

### What to Flag

- **Duplicates** — same intent expressed in two or more places (even with different wording)
- **Contradictions** — two rules that conflict; cite both locations and recommend which wins
  (security-tightening always wins over brevity or convenience)
- **Orphaned bandaids** — `MEMORY.md` entries that are really rules and belong in `rules/` or an agent `.md`
- **Misplaced architecture** — workspace layout, crate dependency tables, module maps in `CLAUDE.md`
  (these belong in `ARCHITECTURE.md`)
- **Agent directory duplication** — Agent Directory table in `CLAUDE.md` vs `team-collaboration.md`
- **Skill overlap** — rules in `CLAUDE.md` that are fully covered by an existing `skills/*.md`

### Audit Report Output

Write the full reconciliation report to:

```
.claude/reports/YYYY-MM-DD-housekeeping-audit.md
```

The report must include:

1. **Summary** — total findings by category (duplicates, contradictions, misplaced, orphaned, drop candidates)
2. **Reconciliation table** — one row per finding with source file, target file, and recommended action
3. **Contradiction register** — each conflict listed with both citations and the recommended resolution
4. **Proposed `ARCHITECTURE.md` outline** — sections to be created and their sources
5. **`CLAUDE.md` proposed slim outline** — what remains after extraction

---

## Phase 2 — Review

**Stop. Do not proceed to Phase 3 until Jamie has reviewed and approved the audit report.**

Present the report path and a brief summary in the chat. Highlight any contradictions that
require a judgment call — do not silently resolve these.

---

## Phase 3 — Execute (Approved Changes Only)

Execute approved changes in this order to minimize broken references:

1. **Create `ARCHITECTURE.md`** from extracted `CLAUDE.md` sections:
   - Workspace layout
   - Crate dependency rules table
   - `umrs-selinux` module map
   - Architectural review triggers table
   - Add pointer in `CLAUDE.md`: `See .claude/plans/ARCHITECTURE.md for workspace layout and architectural constraints.`

2. **Refactor `CLAUDE.md`** to slim, pointer-based format:
   - Identity, environment context, and hard rules stay
   - Each extracted section replaced with a one-line pointer to its new home
   - No content deleted — only moved

3. **Merge Agent Directory** into `team-collaboration.md` if duplicated:
   - Ensure `team-collaboration.md` is the single source of truth
   - Replace the table in `CLAUDE.md` with a pointer

4. **Promote `MEMORY.md` bandaids** to appropriate `rules/` files or agent `.md` files:
   - Rule-like content (how to do X, always do Y) → `rules/` or agent `.md`
   - Project-specific learned context (this crate does X, last session we decided Y) → stays in `MEMORY.md`
   - Trim promoted content from `MEMORY.md` after promotion is confirmed

5. **Update `agents/*.md`** to remove content now fully covered by `rules/`:
   - Agent files should define role, responsibility, and agent-specific constraints only
   - Generic rules belong in `rules/`, not duplicated per-agent

6. **Archive, never delete**:
   - Any content being dropped moves to `archive/` with a one-line comment explaining why
   - Format: `<!-- Archived YYYY-MM-DD: superseded by rules/rust_design_rules.md -->`

7. **Append task-log entry** on completion:
   ```
   [YYYY-MM-DD HH:MM] orchestrator  .claude housekeeping pass  cat,rg  success  N findings resolved
   ```

---

## Conflict Resolution Rules

When two rules contradict each other, apply these tiebreakers in order:

1. **Security-tightening wins** — the more restrictive rule is canonical
2. **Newer wins** — if security posture is equal, the more recently written rule wins
3. **Escalate to Jamie** — if neither tiebreaker is clear-cut, do not silently resolve; flag it

---

## Hard Constraints

These apply throughout all phases without exception:

- **Never touch** `knowledge/`, `references/`, `rag/`, `plans/`, `reports/`, `jamies_brain/`, `ROADMAP.md`
- **Never delete** — archive with a reason comment if dropping content
- **Never git commit or push**
- **Phase 2 approval is mandatory** — do not skip directly from audit to execution
- **No source code changes** — this task is documentation and instruction files only
- **Security-tightening always wins** when resolving contradictions

---

## Portability Goal

The end state of each housekeeping pass should move the `.claude/` directory closer to
being portable to a new project. The test is:

> Could a new team member clone this repo, read `CLAUDE.md` → `ARCHITECTURE.md` →
> `team-collaboration.md` → `rules/` → `agents/` in order and have a complete, unambiguous
> picture of how to work in this codebase?

`agent-memory/` and `jamies_brain/` are explicitly **not portable** — they are
project-specific learned context. Everything else should be.


