# Housekeeping Audit Report — 2026-03-18

**Executor:** Orchestrator (Claude Code)
**Phase:** 1 — Read-Only Audit
**Scope:** CLAUDE.md, rules/*.md, agents/*.md, agent-memory/*/MEMORY.md, agent-memory non-MEMORY files, team-collaboration.md, orchestrator MEMORY.md

---

## 1. Summary

| Category | Count |
|---|---|
| Duplicates | 14 |
| Contradictions | 4 |
| Orphaned bandaids (memory → rule) | 8 |
| Misplaced architecture | 4 sections |
| Agent Directory duplication | 3 locations |
| Skill overlap | 1 |
| Drop candidates | 3 |
| **Total findings** | **37** |

---

## 2. Reconciliation Table

### 2.1 Duplicates

| # | Content | Source A | Source B (+ C) | Recommendation |
|---|---|---|---|---|
| D-01 | Agent Directory / Team Structure table | CLAUDE.md §Agent Directory | team-collaboration.md §Team Structure + cross-team/notes.md §Agent Directory | **MERGE-WITH** team-collaboration.md. Replace CLAUDE.md table with one-line pointer. Remove duplicate from cross-team/notes.md. |
| D-02 | Tiered Annotation Expectations (modules/types/accessors) | CLAUDE.md §Compliance Annotations | agents/security-auditor.md §Tiered Annotation Expectations, agents/security-engineer.md §Tiered Annotation Expectations, agents/rust-developer.md §Documentation Standards | **MERGE-WITH** rules/rust_design_rules.md (add as a new rule). Remove from all agent .md files; they inherit via rules/. |
| D-03 | "No git commit/push" | CLAUDE.md §Claude will NEVER | rules/agent_behavior_rules.md §Repository Interaction Rule, every agent .md | **KEEP** in CLAUDE.md and rules/agent_behavior_rules.md. **DROP** from individual agent .md files (they inherit rules/). |
| D-04 | "No unsafe / forbid(unsafe_code)" | CLAUDE.md §Critical Coding Rules | agents/rust-developer.md §Mandatory Project Rules | **KEEP** in CLAUDE.md (project-wide). Agent file's mention is acceptable as role-specific reinforcement — no action. |
| D-05 | Protected Files Rule | CLAUDE.md §Critical Coding Rules (implicit) | rules/agent_behavior_rules.md §Protected Files Rule, agents/rust-developer.md line 47, agents/security-engineer.md line 154 | **KEEP** in rules/agent_behavior_rules.md only. Replace CLAUDE.md mention with pointer. Drop from agent .md files. |
| D-06 | Admonition Hierarchy (full table) | rules/admonition_hierarchy.md | rules/ste_mode.md §AsciiDoc Admonitions | **MERGE-WITH** rules/admonition_hierarchy.md as the single canonical location. Replace the full hierarchy in ste_mode.md with a one-line pointer: "See rules/admonition_hierarchy.md for the full hierarchy." |
| D-07 | Build commands (cargo xtask fmt/clippy/test) | CLAUDE.md §Build & Test Commands | agents/rust-developer.md §Module and Workspace Conventions | **KEEP** in CLAUDE.md (project-wide). Agent's brief mention is acceptable reinforcement — no action. |
| D-08 | "make docs must pass" | Orchestrator MEMORY.md (feedback_make_docs_mandatory) | agents/tech-writer.md §Doc-Ops Discipline, agents/senior-tech-writer.md §Doc-Ops Discipline, tech-writer MEMORY.md, senior-tech-writer MEMORY.md | **KEEP** in agents/tech-writer.md and senior-tech-writer.md (role-specific enforcement). **DROP** from both agent MEMORY.md files (already in their agent .md). Orchestrator memory entry is legitimate (Jamie preference tracking). |
| D-09 | "Never create local.settings.json" | CLAUDE.md §Settings Files — Hard Rule | rules/agent_behavior_rules.md §Settings and Data Location Rule, Orchestrator MEMORY.md | **KEEP** in CLAUDE.md and rules/. **DROP** from Orchestrator MEMORY.md (redundant with authoritative sources). |
| D-10 | Clippy aesthetic-vs-correctness philosophy | CLAUDE.md §Clippy Policy | agents/rust-developer.md §Rust Engineering Discipline + §Clippy suppressions | **KEEP** in CLAUDE.md (full suppressions table is project-level). Agent mention is brief reinforcement — no action. |
| D-11 | High-Assurance Pattern descriptions | CLAUDE.md §High-Assurance Design Patterns (brief) | agents/rust-developer.md §High-Assurance Patterns (detailed), rules/high_assurance_pattern_rules.md | **KEEP** CLAUDE.md pointer to docs/modules/patterns/. Agent .md keeps its full descriptions (role-specific operational guidance). rules/ keeps its measurement/must-use/validate rules. All three serve different purposes — no merge needed, but CLAUDE.md should slim to pointer only. |
| D-12 | Architectural Review Triggers table | CLAUDE.md §Architectural Review Triggers | agents/rust-developer.md §Architectural Review Triggers | CLAUDE.md table has 17 triggers. Agent table has 11 triggers with slightly different wording and some unique entries (e.g., TPI scope notes for kernel attributes). **MOVE-TO** ARCHITECTURE.md (CLAUDE.md table). Agent table is role-specific operational guidance — **KEEP** in agent .md but add a note: "See ARCHITECTURE.md for the full trigger list." |
| D-13 | RAG reference library description | agents/rust-developer.md §RAG Reference Library | agents/security-engineer.md §RAG Reference Library | Both have ~30 lines describing the same RAG collections. **MERGE-WITH** a shared pointer. Add a "RAG Reference Library" section to CLAUDE.md (3 lines: what it is, how to query, where to find details). Remove the per-agent collection inventories. Each agent keeps only its role-specific "When to invoke" guidance. |
| D-14 | Single .claude directory rule | rules/agent_behavior_rules.md §Single .claude Directory Rule | Orchestrator MEMORY.md feedback_single_claude_dir | **KEEP** in rules/. **DROP** from Orchestrator MEMORY.md (redundant). |

### 2.2 Contradictions

| # | Rule A | Rule B | Resolution |
|---|---|---|---|
| C-01 | **CLAUDE.md §Settings Files — Hard Rule**: "NEVER create `.claude/settings.local.json`" + rules/agent_behavior_rules.md §Settings and Data Location Rule: same | **Reality**: `.claude/settings.local.json` EXISTS on disk (119 bytes, dated 2026-03-18) | **ESCALATE to Jamie.** The file exists. Either (a) merge its contents into settings.json and delete it, or (b) update the rule. Security-tightening says: merge and delete. |
| C-02 | **CLAUDE.md §Workspace Layout**: Lists `cui-labels/`, `kernel-files/`, `mcs-setrans/`, `vaultmgr/` under `components/rusty-gadgets/` | **rust-developer MEMORY.md**: "Prototype workspace: `components/rust-prototypes/` — moved 2026-03-11" + cross-team/notes.md 2026-03-11 confirms move | **CLAUDE.md is stale.** Update workspace layout to reflect the move. Remove the four crates from the rusty-gadgets listing; add a note about `components/rust-prototypes/` being out of scope. |
| C-03 | **rules/ste_mode.md §Admonition hierarchy scope**: "Unlike the other STE rules... applies to ALL UMRS documentation" | **rules/admonition_hierarchy.md §Scope**: "This rule applies to ALL UMRS documentation" | **Not a true contradiction** — both agree the hierarchy is universal. But having the full hierarchy in TWO files is confusing. Resolve via D-06 (single canonical location). |
| C-04 | **senior-tech-writer MEMORY.md SDR-005**: "Second person throughout; third person only when describing system behavior" | **senior-tech-writer MEMORY.md SDR-009**: "Third person for architecture/security-concepts; second person for devel/deployment/operations" | **SDR-009 supersedes SDR-005.** SDR-005 should be updated to note it was refined by SDR-009. This is internal to the agent's MEMORY — not a cross-file conflict, but worth flagging for cleanup. |

### 2.3 Orphaned Bandaids (Memory entries that are really rules)

| # | Source | Content | Recommendation |
|---|---|---|---|
| B-01 | Orchestrator MEMORY.md §User Preferences | "All Rust development — delegate to rust-developer agent; do not write Rust directly" | **MOVE-TO** CLAUDE.md §General Workflow (add a line: "Delegate all Rust implementation to the rust-developer agent.") Then remove from MEMORY.md. |
| B-02 | Orchestrator MEMORY.md §User Preferences | "Clippy style: Prefer explicit if/else/match over .map_or_else(); use #[allow(...)] rather than rewriting" | **Already in** CLAUDE.md §Clippy Policy (option_if_let_else suppression). **DROP** from MEMORY.md. |
| B-03 | Orchestrator MEMORY.md §User Preferences | "End of session: Brief summary of changes, flag anything unusual, then 'your turn to review, commit, and push'" | **MOVE-TO** CLAUDE.md (add to §General Workflow as session wrap-up protocol). Then trim from MEMORY.md. |
| B-04 | Orchestrator MEMORY.md §Critical Rules | Entire "Critical Rules (from CLAUDE.md — quick reference)" section | **DROP.** This is explicitly a copy of CLAUDE.md content and adds no value. MEMORY.md should not mirror CLAUDE.md. |
| B-05 | Orchestrator MEMORY.md §Documentation Rules | "make docs must pass cleanly" + "Subdirectory reorganizations require updating xrefs" | First rule: **DROP** (redundant with tech-writer/senior-tech-writer agent .md files). Second rule: **MOVE-TO** rules/ or CLAUDE.md as a general doc rule if not already there. |
| B-06 | Orchestrator MEMORY.md §Hard Rules — Feedback | 10 linked feedback files covering operational rules | **REVIEW each.** Some are Jamie-specific preferences (legitimate memory). Others are universal rules that belong in rules/. Specific recommendations below in §2.5. |
| B-07 | security-auditor MEMORY.md §Control Mapping Conventions | "TPI claims → cite SI-7", "Fail-closed → cite SI-10", etc. | **KEEP as agent memory.** This is operational knowledge specific to the auditor role — it's a conventions reference, not a rule for all agents. |
| B-08 | security-engineer MEMORY.md §ProcfsText / SysfsText mandatory | Repeats the ProcfsText/SysfsText rule | **DROP.** Already in CLAUDE.md §Critical Coding Rules, agents/rust-developer.md, and rules would be the right place. The agent MEMORY.md copy is pure redundancy. |

### 2.4 Misplaced Architecture (CLAUDE.md → ARCHITECTURE.md)

| # | CLAUDE.md Section | Lines (approx) | Notes |
|---|---|---|---|
| A-01 | Workspace Layout (directory tree) | ~25 | Stale — needs C-02 fix during extraction |
| A-02 | Crate Dependency Rules table | ~15 | Stable, well-defined |
| A-03 | umrs-selinux Module Map table | ~20 | Stable reference |
| A-04 | Architectural Review Triggers table | ~20 | Master list; agent .md has role-specific subset |

### 2.5 Orchestrator MEMORY.md Feedback Files — Triage

| File | Content | Verdict |
|---|---|---|
| `feedback_agent_permissions.md` | Pre-grant perms before launching background agents | **RULE** — move to rules/agent_behavior_rules.md |
| `feedback_corpus_familiarization.md` | After RAG ingestion, run corpus-familiarization | **RULE** — move to rules/agent_behavior_rules.md |
| `feedback_single_claude_dir.md` | Never create .claude/.claude/ | **DROP** — already in rules/agent_behavior_rules.md |
| `feedback_nav_scrutiny.md` | Audit all navs before declaring done | **RULE** — move to agents/senior-tech-writer.md (role-specific) or rules/ |
| `feedback_make_docs_mandatory.md` | Run make docs after every change | **DROP** — already in both writer agent .md files |
| `feedback_move_after_read.md` | Ask Jamie to move/archive processed files | **KEEP** — Jamie-specific preference |
| `feedback_know_our_tools.md` | Read script usage before invoking | **RULE** — move to rules/agent_behavior_rules.md |
| `feedback_preflght_agent_dirs.md` | mkdir -p output paths before background agents | **RULE** — move to rules/agent_behavior_rules.md |
| `feedback_plan_status_headers.md` | Every plan needs a Status: line | **RULE** — move to rules/agent_behavior_rules.md |
| `feedback_scratch_cleanup.md` | Delete _scratch/_archive after promoting | **KEEP** — project-specific context |
| `feedback_roadmap_reference.md` | Plans must reference ROADMAP goals | **RULE** — move to rules/agent_behavior_rules.md |
| `feedback_archive_after_plan.md` | Ask Jamie to archive jamies_brain files | **KEEP** — Jamie-specific preference |
| `feedback_human_centered_design.md` | Security without usability is shelfware | **KEEP** — design philosophy, project-specific |
| `feedback_doc_tone.md` | Public docs: energetic, detailed, blog-ready | **KEEP** — Jamie preference |
| `feedback_mermaid_diagrams.md` | Use Mermaid liberally | **KEEP** — Jamie preference (also in doc-team feedback.md) |
| `feedback_thin_tui_principle.md` | Platform is expert, consumers are viewports | **KEEP** — architectural decision |
| `feedback_vendor_terminology.md` | Library owns canonical names | **KEEP** — architectural decision |
| `feedback_tui_helpers_to_platform.md` | Move TUI helpers to platform | **KEEP** — project-specific planned work |

**Summary:** 7 feedback files should be promoted to rules/. 3 should be dropped (redundant). 8 should stay (legitimate project context or Jamie preferences).

### 2.6 Skill Overlap

| # | CLAUDE.md Section | Skill | Recommendation |
|---|---|---|---|
| S-01 | ASM Usage Policy (~80 lines) | `asm-guidance` skill | **Slim CLAUDE.md** to the three-gate test (~15 lines) + pointer to the skill for full guidance, permitted use cases, templates, and verification checklists. Move the detailed tables and templates to the skill if not already there. |

### 2.7 Drop Candidates

| # | Source | Content | Reason |
|---|---|---|---|
| X-01 | cross-team/notes.md | DOCUMENTATION FREEZE entry (2026-03-12) | Restructure is COMPLETE per orchestrator MEMORY.md. This entry is stale/misleading — mark resolved. |
| X-02 | cross-team/notes.md | Multiple "open" entries from 2026-03-12 that are informational (not actionable) | These are reference announcements, not actionable items. They clutter the file. **Archive** entries older than 2 weeks that are purely informational. |
| X-03 | doc-team/feedback.md | ~20 "open" TW-* and STW-* entries from 2026-03-10 that are marked resolved in their body | The "open" status in some headers conflicts with "resolved" noted in the content. Clean up status fields. |

---

## 3. Contradiction Register

| # | Location A | Location B | Conflict | Recommended Resolution |
|---|---|---|---|---|
| C-01 | CLAUDE.md + rules/agent_behavior_rules.md | `.claude/settings.local.json` on disk | Policy forbids file; file exists | **ESCALATE:** Jamie merges contents into settings.json, then deletes the file |
| C-02 | CLAUDE.md §Workspace Layout | rust-developer MEMORY.md + cross-team notes | Layout lists crates in wrong workspace | **Fix CLAUDE.md** during ARCHITECTURE.md extraction |
| C-03 | rules/ste_mode.md | rules/admonition_hierarchy.md | Same content in two files | **Deduplicate:** ste_mode.md points to admonition_hierarchy.md |
| C-04 | senior-tech-writer MEMORY.md SDR-005 | SDR-009 | Person guidance refined | **Agent self-cleanup:** SDR-005 notes superseded by SDR-009 |

---

## 4. Proposed ARCHITECTURE.md Outline

```
# UMRS Architecture Reference

## Workspace Layout
[From CLAUDE.md — updated to reflect rust-prototypes move]

## Crate Dependency Rules
[Table from CLAUDE.md — verbatim]

## umrs-selinux Module Map
[Table from CLAUDE.md — verbatim]

## Architectural Review Triggers
[Full 17-trigger table from CLAUDE.md]
[Note: agents/rust-developer.md has a role-specific subset]

## Environment Context
[Consider moving from CLAUDE.md — describes RHEL 10, FIPS, network posture, audit exposure, data sensitivity]
```

---

## 5. Proposed CLAUDE.md Slim Outline

After extraction, CLAUDE.md retains:

```
## Build & Test Commands                    [KEEP — operational]
## General Workflow                         [KEEP + add B-01, B-03 session protocol]
## Claude will NEVER                        [KEEP — hard constraints]
## Technology Stack                         [KEEP — brief]
## Environment Context                      [KEEP or MOVE to ARCHITECTURE.md — Jamie's call]
## Critical Coding Rules                    [KEEP — slim, no duplication of rules/]
## Shell Tools — Hard Rule                  [KEEP]
## Path Rules — Hard Rule                   [KEEP]
## Shell Conventions                        [KEEP]
## Settings Files — Hard Rule               [KEEP]
## Workspace Layout → ARCHITECTURE.md       [POINTER ONLY]
## Crate Dependency Rules → ARCHITECTURE.md [POINTER ONLY]
## Module Map → ARCHITECTURE.md             [POINTER ONLY]
## Clippy Policy                            [KEEP — suppressions table is project-level]
## Compliance Annotations                   [SLIM — pointer to rules/rust_design_rules.md]
## High-Assurance Design Patterns           [SLIM — pointer to docs/modules/patterns/]
## Architectural Review Triggers → ARCH.md  [POINTER ONLY]
## Agent Directory → team-collaboration.md  [POINTER ONLY]
## Code Navigation & Metadata               [KEEP]
## TUI/CLI Design Principles                [KEEP]
## Role of Claude Code in This Project      [KEEP — mission statement]
## Reference Documents                      [KEEP]
## ASM Usage Policy                         [SLIM — three-gate test + pointer to skill]
## Performance & Task Tracking              [KEEP]
```

**Estimated reduction:** ~120 lines removed from CLAUDE.md (replaced with ~15 lines of pointers).

---

## 6. Cross-Team Notes Cleanup

The cross-team/notes.md file is 675 lines and growing. Many entries are informational announcements, not actionable work items. Recommended:

1. **Mark stale entries as resolved:** Documentation freeze (2026-03-12), all 2026-03-12 informational announcements that have been consumed.
2. **Archive pattern:** Entries older than 2 weeks with status "open" that are purely informational should be archived to a `cross-team/archive/` file.
3. **Trim Agent Directory from cross-team/notes.md** — it duplicates team-collaboration.md.

---

## 7. Doc-Team Feedback Cleanup

The feedback.md file is 975 lines. Most TW-* and STW-* entries from 2026-03-10 are resolved. The file should be trimmed:

1. All entries with `**Status**: resolved` can be moved to an archive section at the bottom or a separate file.
2. Open entries awaiting Jamie (security-model.adoc, admin/ cleanup, i18n.md, selinux-registry.txt, rhel10-install.adoc duplicate) should be consolidated into a "Pending Jamie Decision" block at the top.

---

## 8. Agent MEMORY.md Health

| Agent | Lines | Health | Action Needed |
|---|---|---|---|
| rust-developer | 252 | **Over limit** (200 line soft cap) | Distill: move TUI implementation details to topic files; trim resolved audit findings |
| security-auditor | 269 | **Over limit** | Move CPU matrix, RMF, accreditation sections to topic files (links already exist for some) |
| security-engineer | 130 | OK | Drop B-08 (ProcfsText redundancy) |
| senior-tech-writer | 329 | **Well over limit** | Aggressively distill: Phase completion histories → archive; structural decisions → topic file; only current state in MEMORY.md |
| tech-writer | 206 | Slightly over | Trim conversion history (2026-03-10/11 work is done) |
| researcher | 228 | Over limit | Move retrieval patterns to topic file; keep collection table and pipeline notes |
| umrs-translator | 207 | Slightly over | Move vocabulary decisions to vocabulary.md (already partially done) |
| Orchestrator | 129 | OK (line count) | Content quality: 7 entries to promote, 3 to drop (see §2.5) |

---

*End of Phase 1 audit. Do not proceed to Phase 3 until Jamie has reviewed this report.*
