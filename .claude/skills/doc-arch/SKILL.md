---
name: doc-arch
description: >
  Searches the doc-structure RAG collection for documentation architecture
  guidance: Diataxis taxonomy, Antora structure, modular documentation, style
  guides, and docs-as-code practices. Trigger when the senior-tech-writer or
  tech-writer needs to decide where content belongs, how to structure navigation,
  how to apply topic types (concept/procedure/reference/tutorial), how to set up
  documentation testing, or when reviewing whether a page fits its current Antora
  module. Also trigger when any agent asks "where should this doc go?", "what
  type of doc is this?", "how does Antora handle X?", or similar documentation
  architecture questions.
---

# Doc-Arch Skill

Semantic search over the `doc-structure` RAG collection for documentation
architecture and style guidance.

## Path

```
.claude/rag/query.py
```

## Usage

```bash
# Search the doc-structure collection
python .claude/rag/query.py "your query" --collection doc-structure

# Return more results
python .claude/rag/query.py "your query" --collection doc-structure --top-k 10

# JSON output for programmatic use
python .claude/rag/query.py "your query" --collection doc-structure --json
```

## When to Use

Query `doc-structure` when you need guidance on:

| Decision area | Example queries |
|---|---|
| **Content classification** | "diataxis tutorial vs how-to distinction" |
| **Page placement** | "which diataxis type is explanation vs reference" |
| **Antora structure** | "antora component version descriptor nav key" |
| **Navigation** | "antora navigation file registration order" |
| **Cross-references** | "antora xref macro cross-module resource ID" |
| **Modular docs** | "red hat modular concept procedure reference module types" |
| **Assembly structure** | "modular documentation assembly include leveloffset" |
| **Topic types** | "gitlab concept task reference troubleshooting CTRT" |
| **Style rules** | "google style procedures introductory sentence sub-steps" |
| **Voice and tone** | "google developer documentation conversational tone avoid simply" |
| **API docs** | "api reference code comments class method documentation" |
| **Doc testing** | "gitlab documentation CI testing vale markdownlint" |
| **Docs-as-code** | "docs as code version control plain text markup review" |
| **File naming** | "red hat modular docs file naming con proc ref prefix" |
| **Playbook config** | "antora playbook content sources UI bundle site properties" |

## Query Writing Guidelines

Search is semantic (meaning-based). Write queries as natural phrases, not keywords.

| Instead of | Use |
|---|---|
| `diataxis types` | "four documentation types tutorial how-to reference explanation" |
| `antora nav` | "antora navigation file registration component menu" |
| `modular docs` | "modular documentation concept procedure reference assembly structure" |
| `style guide tone` | "conversational documentation tone avoid condescending language" |

## Interpreting Results

Each result includes:
- **Score** — 0.0-1.0. Above 0.75 is relevant. Below 0.6 may be tangential.
- **Source** — which reference subdirectory (diataxis, antora, redhat-modular, etc.)
- **Section** — the heading the chunk falls under
- **Text preview** — first 300 chars

## Example Queries by Situation

**Deciding where content belongs:**
```bash
python .claude/rag/query.py "diataxis compass action cognition acquisition application" --collection doc-structure
```

**Setting up Antora navigation:**
```bash
python .claude/rag/query.py "antora navigation files unordered lists registration component version" --collection doc-structure
```

**Writing a procedure:**
```bash
python .claude/rag/query.py "google style procedures introductory sentence numbered steps sub-steps" --collection doc-structure
```

**Reviewing modular doc structure:**
```bash
python .claude/rag/query.py "red hat modular documentation module types assembly user story" --collection doc-structure
```

**Understanding Antora build pipeline:**
```bash
python .claude/rag/query.py "antora build pipeline playbook content sources component version" --collection doc-structure
```

## Source Material

The `doc-structure` collection contains material from 7 authoritative sources:

| Source | Coverage |
|---|---|
| Diataxis Framework | Four documentation types, compass, map, content classification |
| Divio Documentation System | Original four-type taxonomy, tutorials, reference, explanation |
| Antora Official Docs | Component descriptors, navigation, xrefs, playbook, build pipeline |
| Red Hat Modular Docs | Concept/procedure/reference modules, assemblies, snippets, file naming |
| Write the Docs | Docs-as-code, style guides, documentation principles |
| Google Developer Style | Voice/tone, procedures, API reference comments |
| GitLab Docs Practices | Topic types (CTRT), style guide, CI testing, docs-first methodology |
