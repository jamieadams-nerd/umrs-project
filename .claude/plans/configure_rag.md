# UMRS Agent & RAG Infrastructure Setup Guide

## Overview

This document captures the complete setup procedure for the UMRS project's
AI agent infrastructure, RAG reference library, and associated skills. Follow
these steps to reproduce the setup from scratch on any machine.

---

## Prerequisites

- RHEL 10 VM (or compatible Linux) with Python 3.12+
- Claude Code CLI installed
- `pandoc` installed: `sudo dnf install pandoc`
- Project repository cloned at your working path

---

## Part 1 — Directory Structure

Create the following layout under your project root:

```
.claude/
├── agents/           ← agent .md definition files
├── skills/           ← skill SKILL.md files
│   ├── rag-ingest/
│   └── rag-query/
├── references/       ← raw source documents (READ ONLY — never modified)
│   ├── selinux-notebook/
│   ├── linux-fhs-2-3/
│   └── kernel-docs/
└── rag/              ← RAG pipeline scripts and database
    ├── ingest.py
    ├── query.py
    ├── chroma/       ← ChromaDB database directory (auto-created)
    ├── converted/    ← converted .md files (auto-created)
    └── manifest.json ← ingestion state tracking (auto-created)
```

---

## Part 2 — Python Dependencies

```bash
pip install chromadb sentence-transformers tiktoken --break-system-packages
```

---

## Part 3 — RAG Scripts

### ingest.py

Place at `.claude/rag/ingest.py`. Key behaviors:

- Reads raw files from `.claude/references/<collection>/`
- Converts supported formats to Markdown using pandoc
- `.txt` and `.md` files are copied as-is (no pandoc needed)
- Binary files (images, archives, compiled objects) are skipped entirely
- Chunks each Markdown file into ~512-token passages respecting heading
  and code block boundaries
- Embeds each chunk using `all-MiniLM-L6-v2` (downloads ~90MB on first run,
  cached at `~/.cache/huggingface/` after that)
- Stores chunks and vectors in ChromaDB at `.claude/rag/chroma/`
- Records every processed file in `manifest.json` with its SHA-256 hash
- Saves manifest after each file — safe to interrupt and resume
- Re-running on an unchanged file is safe — hash check skips it

Usage:

```bash
# Process all collections
python .claude/rag/ingest.py

# Process one collection
python .claude/rag/ingest.py --collection selinux-notebook

# Force reprocess everything
python .claude/rag/ingest.py --force

# Show database summary
python .claude/rag/ingest.py --summary
```

### query.py

Place at `.claude/rag/query.py`. Key behaviors:

- Accepts a natural language query
- Embeds the query using the same model as ingest
- Searches one, several, or all ChromaDB collections
- Merges and re-ranks results by cosine similarity across collections
- Returns ranked chunks with source attribution (file, section, score)

Usage:

```bash
# Search all collections
python .claude/rag/query.py "your query here"

# Search a specific collection
python .claude/rag/query.py "your query" --collection selinux-notebook

# Search multiple collections
python .claude/rag/query.py "your query" --collection kernel-docs linux-fhs-2-3

# Return more results (default: 5)
python .claude/rag/query.py "your query" --top-k 10

# JSON output for agent consumption
python .claude/rag/query.py "your query" --json

# List available collections and chunk counts
python .claude/rag/query.py --list-collections
```

---

## Part 4 — Agents

Create each file in `.claude/agents/`. The `model:` field in frontmatter
controls which Claude model the agent uses.

### Model Assignment

| Agent              | Model  | Reason                                      |
|--------------------|--------|---------------------------------------------|
| researcher         | haiku  | Mechanical tasks — runs scripts, compares lists |
| security-auditor   | sonnet | Judgment required — code review, citations  |
| umrs-translator    | sonnet | French technical vocabulary requires nuance |
| guest-coder        | sonnet | API feedback requires genuine reasoning     |
| guest-admin        | sonnet | Documentation review requires judgment      |

### Agent Frontmatter Pattern

```markdown
---
name: researcher
description: >
  [agent description here]
model: haiku
---

[agent system prompt here]
```

### Agents Created

**researcher** — Maintains the reference library. Detects new collections
in `.claude/references/`, runs ingest, monitors for document updates,
compares versions and notifies the team of changes.

**security-auditor** — Read-only reviewer. Checks source code comments and
documentation for proper NIST/CMMC/RTB control citations. Flags uncited
security claims. Makes no code changes.

**umrs-translator** — Owns the i18n/l10n pipeline using xgettext/gettext.
Identifies unwrapped strings, recommends text-domain strategy, manages
.pot/.po/.mo lifecycle. French is the primary target language with accurate
Five Eyes francophone technical vocabulary.

**guest-coder** — Writes example code in `examples/` directories as a
developer new to UMRS. Reviews rustdoc and provides feedback on API clarity,
naming, and usability. Does not modify library source.

**guest-admin** — Reviews admin and deployment documentation as a
non-specialist operator. Runs CLI tools and evaluates argument naming,
help output, and error messages. Provides feedback only, no changes.

### Bootstrapping Agents

After creating agent files, run this in Claude Code to have each agent
review and refine its own prompt:

```
For each agent in .claude/agents/, read its system prompt, summarize
your understanding of its role, ask me all clarifying questions at once
for anything ambiguous or underspecified. Do one agent at a time, wait
for my answers, incorporate them, and overwrite the file in place before
moving to the next.
```

---

## Part 5 — Skills

Create each skill directory under `.claude/skills/` with a `SKILL.md` file.

### rag-ingest

Location: `.claude/skills/rag-ingest/SKILL.md`

Assigned to: `researcher` agent

Triggers when: user asks to add a collection to the RAG, ingest new
documents, or update the reference library.

Procedure the skill follows:
1. Compare `.claude/references/` directories against `--list-collections`
2. Report gap to user
3. Run `ingest.py --collection <name>` for each missing collection
4. Verify with `--list-collections` and report chunk counts

### rag-query

Location: `.claude/skills/rag-query/SKILL.md`

Assigned to: all technical agents (rust-developer, security-engineer,
security-auditor, tech-writer, guest-coder, guest-admin)

Triggers when: any agent works on a topic touching SELinux, MLS policy,
Linux kernel internals, filesystem standards, IMA, dm-crypt, Linux
capabilities, extended attributes, or CUI handling.

Procedure the skill follows:
1. Formulate a semantic query (natural phrase, not keywords)
2. Run `query.py` against relevant collection(s)
3. Review scores — above 0.85 highly relevant, below 0.6 tangential
4. Cite source file and section heading in the answer

---

## Part 6 — Initial Ingestion

Run the first ingest manually for each collection:

```bash
cd /path/to/project

# Start with the smallest collection to verify the pipeline
python .claude/rag/ingest.py --collection selinux-notebook

# Then the others
python .claude/rag/ingest.py --collection linux-fhs-2-3

# kernel-docs is large (~12,942 files, ~85 minutes at 0.4s/file)
# Run in background or be prepared to wait
python .claude/rag/ingest.py --collection kernel-docs
```

The manifest checkpoint means any interrupted run can be safely resumed
by re-running the same command — already-processed files are skipped.

### Verify

```bash
python .claude/rag/query.py --list-collections
```

Expected output after full ingest:

```
Available collections:
  selinux-notebook   (691 chunks)
  linux-fhs-2-3      (45 chunks)
  kernel-docs        (growing during ingest)
```

---

## Part 7 — Adding New Collections

When new reference material becomes available:

1. Create a directory under `.claude/references/new-collection-name/`
2. Drop raw documents into it (PDF, RST, HTML, TXT, MD all supported)
3. Tell the researcher agent:

```
Check .claude/references/ for any collections not yet in the RAG
database and ingest any that are missing. Then show me a summary
of all collections and chunk counts.
```

The researcher agent will detect the new directory, ingest it, and report
the results.

---

## Part 8 — Permanent Architectural Constraints

The following constraints are recorded in `CLAUDE.md` and must never be
violated during coding or refactoring:

### Crate Dependency Order

- `umrs-platform` — no dependencies on the other two crates
- `umrs-selinux` — depends on `umrs-platform` only
- `umrs-core` — depends on both `umrs-platform` and `umrs-selinux`

These dependency directions are fixed and must never be reversed or added to.

### Adding New Permanent Constraints

To add a new architectural constraint to `CLAUDE.md` via Claude Code:

```
Add the following permanent constraint to CLAUDE.md: [your rule]
```

Good candidates for permanent constraints:
- Dependency rules
- API stability rules
- Safety annotation requirements
- Naming conventions

---

## Part 9 — Ongoing Maintenance

| Task | Who | How |
|---|---|---|
| New reference material | researcher | Drop in `references/`, ask agent to ingest |
| Document updated upstream | researcher | Hash change triggers automatic re-ingest |
| New collection | researcher | Create directory, agent detects and ingests |
| Query the library | any agent | `@agent-name` + question, skill triggers automatically |
| Check DB health | anyone | `python .claude/rag/query.py --list-collections` |

---

## Known Issues Fixed During Setup

| Issue | Fix |
|---|---|
| `sqlite-vec` wrong ELF class (32-bit) | Switched to ChromaDB — no native binaries |
| `datetime.UTC` crash on Python 3.12 | Use `from datetime import timezone` then `timezone.utc` |
| pandoc hangs on `.png` files | Added `SKIP_EXTENSIONS` set in `ingest.py` |
| pandoc fails on `.txt` files | Moved `.txt` to `PASSTHROUGH` — copied as-is |
| Interrupted ingest loses progress | Manifest now saved after each file, not end of run |
