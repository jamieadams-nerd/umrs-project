## Task Board Rule

- The Task system is the shared cross-channel communication board for all agents.
- All agents MUST check the task board (TaskList) at the start of every session.
- Use TaskGet to read task details and comments relevant to your role.
- Leave status updates, findings, and hand-off notes as task comments (TaskUpdate).
- Create new tasks when work items emerge that cross agent boundaries.
- The task board supplements (does not replace) `.claude/agent-memory/cross-team/notes.md`.
- Use the task board for actionable work items; use cross-team notes for contextual advisories.

## Settings and Data Location Rule

- NEVER create a `local.settings.json` file anywhere in the repository.
- Always use `.claude/settings.json` for project-level settings.
- All Claude-specific data (agent memory, plans, reports, references, RAG, rules, skills, commands) belongs under `.claude/`.
- Do not create Claude configuration or data files at the repository root or outside `.claude/`.

## Single .claude Directory Rule

- There is exactly ONE `.claude/` directory in this project: the one at the repository root.
- Agents MUST NOT create `.claude/` directories anywhere else — not in subdirectories, not in crate roots, not in docs/, not relative to their CWD.
- When writing to agent-memory, reports, or settings, always use absolute paths rooted at the project's `.claude/` directory (e.g., `.claude/agent-memory/<agent>/MEMORY.md`, `.claude/reports/`).
- If an agent detects a `.claude/` directory outside the repo root, it must flag it to the user — never write into it.

## Agent Memory Hygiene Rule

- Each agent is responsible for pruning, distilling, and reviewing their own memory file in `.claude/agent-memory/<agent>/MEMORY.md`.
- Remove stale, redundant, or session-specific information.
- Consolidate related entries.
- Keep memory files concise and efficient — they are loaded into context every session.
- If unsure whether information is still relevant, ask the user before deleting.

## Documentation Sync Rule

- When the `rust-developer` agent modifies a **public API surface**, **phase logic**, or **type definition** in any documented crate (currently: `umrs-platform`, `umrs-selinux`, `umrs-core`, `umrs-ls`), it MUST create a task for the `tech-writer` describing what changed and which documentation pages are affected.
- The task title should follow the pattern: `doc-sync: <crate> — <brief description of change>`.
- The task body should list: files changed, types/functions added or modified, and the documentation pages that reference them.
- If the change affects code snippets in `docs/modules/devel/pages/` or `docs/modules/patterns/pages/`, flag this explicitly — illustrative snippets drift silently.
- The `tech-writer` checks for `doc-sync:` tasks at the start of every session and prioritizes them.
- This rule also applies to the `security-engineer` when changes affect deployment or operations documentation.

## Repository Interaction Rule

- Never execute git commit.
- Never execute git push.
- Never create or modify branches.
- The agent may modify working files only.
- The agent must not alter repository history.
- Never modify production configuration unless explicitly instructed.

## Agent Permissions Pre-Grant Rule

- Before launching background agents, pre-grant all Write/Edit/Bash permissions they will need in `.claude/settings.json`.
- Background agents cannot recover from permission denials — they block silently.
- This applies to ALL agent launches, not just the first one in a session.

## Output Directory Pre-Creation Rule

- Before launching background agents, run `mkdir -p` on ALL output paths the agent will write to.
- Background agents cannot create missing directories and will fail silently.
- Check: reports/, admin-reports/, api-reports/, logs/, agent-memory subdirectories.

## Post-Ingestion Familiarization Rule

- After RAG ingestion of a new collection, the target agent MUST run the `corpus-familiarization` skill before using the material.
- Do not skip this step — without familiarization, the agent has passive retrieval but no active knowledge of what the collection contains.

## Plan Status Header Rule

- Every plan file in `.claude/plans/` MUST have a `**Status:**` line near the top.
- At session start, check active plans for missing status headers and add them.
- Valid statuses: `draft`, `approved`, `in-progress`, `blocked`, `completed`.

## ROADMAP Reference Rule

- Every new plan MUST reference which ROADMAP goals (G1–G10) and milestones (M1–M5) it serves.
- Plans without ROADMAP alignment require explicit justification.
- See `.claude/ROADMAP.md` for the current goal and milestone definitions.

## Tool Usage Verification Rule

- Before invoking any script or CLI tool, read its usage/help first.
- Do not guess CLI arguments — verify syntax from the script source or `--help` output.
- When delegating to subagents, provide exact invocation syntax rather than letting the agent discover it.

## Protected Files Rule

The following file patterns must never be edited unless the user explicitly instructs it:

- `**/*.json` — configuration files (CUI labels, MLS state, package manifests)
- `**/setrans.conf` — SELinux MCS translation configuration
- `**/.gitignore` — repository ignore rules

These files affect deployed system behavior or repository integrity.
Changes must be intentional and user-directed.

