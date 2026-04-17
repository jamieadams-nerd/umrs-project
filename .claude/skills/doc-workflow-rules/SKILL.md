---
name: doc-workflow-rules
description: >
  Documentation workflow rules for UMRS: build verification, archive-first policy,
  inbox routing, Antora structure, scratch directory, xref safety, AsciiDoc
  conversion, and file format requirements. Use this skill when editing files
  under docs/, writing .adoc content, working with Antora modules, moving or
  deleting doc pages, or updating nav.adoc files. Trigger when the user or agent
  mentions docs/, .adoc, Antora, nav.adoc, xref, make docs, documentation
  workflow, archive, scratch, or any documentation structure work.
---

## Documentation Workflow Rules

Applies when editing files under `docs/`, writing `.adoc` content, or working with
Antora modules. All documentation agents (tech-writer, senior-tech-writer, sage) must
follow these rules.

## Build Verification Rule

- [RULE] Run `make docs` from the repo root after editing any `.adoc` file.
- Fix Antora warnings before considering the work complete.
- For draft-only modules: `make docs-draft`.

## Archive-First Rule

- [RULE] Never delete documentation or Jamie's write-ups. Archive instead.
- Agent inbox items (`.claude/agent-memory/<agent>/`) → agent manages lifecycle.
- Doc pages that don't fit the current structure → move to `docs/_scratch/`.
- If unsure whether to archive or delete, ask Jamie.

## Inbox Routing Rule

- Jamie delivers write-ups, research, and briefs to agent inboxes at `.claude/agent-memory/<agent>/`.
- Each agent owns their inbox. Process, distill, and archive items as they are consumed.
- Do not leave consumed items in the inbox — either archive to `docs/_scratch/` or delete after extracting value.

## Antora Structure Rule

- All published documentation lives under `docs/modules/` in Antora module layout.
- Module directories: `pages/`, `partials/`, `images/`, `examples/`.
- Navigation is defined in `docs/modules/*/nav.adoc`.
- New pages must be added to the relevant `nav.adoc` or they will not appear in the site.

## Scratch Directory Rule

- `docs/_scratch/` is for content that has value but no home yet.
- Anything in `_scratch/` is excluded from the Antora build.
- Periodically review `_scratch/` with Jamie to decide placement or retirement.

## Xref Safety Rule

- [RULE] When moving or deleting any `.adoc` page, grep ALL `.adoc` files across ALL modules for references to it.
- Nav files alone are not sufficient — inline xrefs in page bodies will break silently.
- Index pages (e.g., `reference/pages/index.adoc`) often mirror nav content — update both.

## Antora Examples Constraint

- Files in `examples/` directories are includable fragments only.
- They cannot be navigable pages or xref targets.

## AsciiDoc Conversion Rule

When converting Markdown (`.md`) or plain text (`.txt`) to AsciiDoc:

- `#` → `=`, `##` → `==`, etc.
- ` ```lang ``` ` → `[source,lang]\n----\n...\n----`
- `**bold**` → `*bold*`
- `[text](url)` → `https://url[text]` or `link:url[text]`
- Tables: `|===` with header row
- Admonitions: `NOTE:` inline or `[NOTE]\n====\n...\n====` block
- Mermaid: `[mermaid]\n....\n<diagram>\n....`
- Standard header: `= Title\n:toc: left\n:description: ...`

## File Format Rule

- `.md` and `.txt` files in `pages/` directories do NOT render in Antora.
- All publishable content must be `.adoc`.

## Shared Resources

- Approved terminology: `.claude/agent-memory/doc-team/approved_terminology.md`
- Doc team feedback log: `.claude/agent-memory/doc-team/feedback.md`
- Style corpus artifacts: `.claude/knowledge/tech-writer-corpus/`
