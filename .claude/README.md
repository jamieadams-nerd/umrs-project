# .claude/ Directory Guide

This directory contains everything Claude Code needs to work on the UMRS project:
the team, their knowledge, their rules, and their output.

## Directory Map

```
.claude/
  agents/              Team: who they are, what they do, how they collaborate
  agent-memory/        Working memory each agent carries between sessions
  rules/               Governance: behavioral rules loaded every session
  skills/              Capabilities: invocable tools agents can use
  commands/            Custom slash commands
  hooks/               Event-driven automation (pre/post tool execution)

  plans/               Strategy and design
    ARCHITECTURE.md      Workspace layout, crate map, architectural review triggers
    ROADMAP.md           Goals (G1-G10), milestones (M1-M5)
    completed/           Finished plans (kept for context)
    long-term/           Backlog and future design

  references/          The library: everything the team reads FROM
    nist/                NIST SPs and FIPS standards (PDFs)
    dod-5200/            DoD directives, CMMC assessment guides
    fedramp/             FedRAMP templates
    cui-registry/        NARA CUI category data (scraped JSON)
    cui-legal-corpus/    CUI marking rules, ISOO transcripts
    corpus/              Terminology databases (Termium Plus, OQLF, GNU .po files)
    scripts/             Python scrapers for refreshing source material
    reports/             Researcher synthesis reports
    refs-manifest.md     Provenance: version, source URL, SHA-256 for every standard
    (30+ other collections: kernel-docs, selinux-notebook, etc.)

  knowledge/           What agents LEARNED from references (familiarization output)
    oscal-schemas/       concept-index, cross-ref map, glossary, style decisions
    tui-cli/
    tech-writer-corpus/
    performance-corpus/

  rag/                 Search engine tooling (indexes references/)
    ingest.py            Ingestion pipeline (ChromaDB)
    query.py             Query interface
    manifest.json        Per-file chunk tracking
                         Actual DB is external: RAG_CHROMA_PATH env var

  reports/             Output: audits, reviews, session summaries
  logs/                Operational records
    task-log.md          Agent task completions (one line per task)
    CHANGELOG.md         Session-level changelog
    tracked-issues.txt   Upstream Claude Code issues we are watching

  jamies_brain/        Jamie's personal workspace and notes
  archive/             Retired content (never deleted, just moved here)
  projects/            Claude Code's own cross-project memory (managed by Claude)
  settings.json        Claude Code permissions and configuration
```

## The Pipeline

```
references/  -->  rag/ + knowledge/  -->  agents do work  -->  reports/ + logs/
  (inputs)        (index + learn)         (using rules/)       (outputs)
```

## What Lives at the Project Root

The project root contains only things that ship or build:

- `CLAUDE.md` -- Claude Code entry point (must be at root)
- `README.md`, `LICENSE*`, `CONTRIBUTORS.md` -- standard repo files
- `Makefile`, `antora-playbook*.yml` -- build config
- `build-tools/` -- scripts and templates for builds
- `components/` -- Rust workspace (the actual code)
- `docs/` -- Antora documentation (published content)
- `resources/i18n/` -- gettext .po/.pot/.mo files (ships with binaries)
- `man/` -- man pages
- `help/` -- Mallard/Yelp desktop help pages
- `build/` -- generated Antora site output

Everything else -- team infrastructure, references, plans, reports -- lives
under `.claude/` so it travels together and stays out of the source tree.
