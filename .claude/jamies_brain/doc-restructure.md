# Documentation Restructure Instructions
## For: The Imprimatur (Senior Tech Writer)
## Type: Planning Only — No Execution Without Approval

---

## Your Mission

The UMRS documentation is currently a single monolithic Antora component.
Every module appears in one left navigation panel — this is the problem.
The goal is to restructure it into a **collection of standalone documentation
sets**, each serving a distinct audience, each with its own focused left
navigation containing only what that audience needs.

You will read the current structure, produce a complete migration plan, and
present it for approval. **You will not move, rename, delete, or modify any
file until the plan is explicitly approved.**

---

## Understand the Current Structure First

The current Antora structure is a single component:

```
docs/
├── antora.yml              ← single component descriptor (read this first)
├── modules/
│   ├── ROOT/               ← landing page, introduction
│   ├── architecture/       ← history, MLS model, library model
│   ├── ai-transparency/    ← AI agent roles, RAG, corpus, workflow
│   ├── cryptography/       ← FIPS, PQC, crypto policy, CPU extensions
│   ├── deployment/         ← RHEL 10, IMA/EVM, kernel lockdown, filesystem
│   ├── devel/              ← build tooling, patterns, cargo, compliance
│   ├── glossary/           ← single index.adoc
│   ├── logging-audit/      ← structured logging, boot-id, log lifecycle
│   ├── operations/         ← admin, IMA/EVM ops, key management, tools
│   ├── patterns/           ← high-assurance patterns library
│   ├── reference/          ← SELinux ref, CUI ref, rust style, secure bash
│   ├── security-concepts/  ← reference monitor, RTB, MLS security model
│   └── umrs-tools/         ← umrs-ls, umrs-state, umrs-logspace, shred
├── supplemental-ui/        ← custom CSS, header, footer, logo
├── images/                 ← logos and banners
├── new-stuff/              ← staging area, do not migrate
└── _scratch/               ← scratch notes, do not migrate
```

Read every `nav.adoc` and `index.adoc` in every module before forming
any opinions. The nav files are the authoritative record of what each
module contains and how it is organized.

Also read:
- `antora-playbook.yml` at the project root
- `docs/antora.yml`
- `docs/supplemental-ui/` — understand the custom UI before planning

---

## The Target Structure

The monolithic site becomes a **collection of five standalone components**
plus a collection home. Each component is a self-contained documentation
set with its own left navigation.

### Component 1 — The UMRS Project
**Audience:** Anyone new to UMRS, evaluators, Five Eyes community members
seeking background and context.
**Purpose:** Introduction, historical background, security concepts, and
the rationale for high-assurance design.
**Candidate modules:** `ROOT`, `architecture` (including `history/`),
`security-concepts`

### Component 2 — UMRS CUI Labeling
**Audience:** Security administrators, classification officers, operators
responsible for configuring CUI enforcement.
**Purpose:** Understanding SELinux MLS in the context of CUI, enabling
and configuring CUI labeling on RHEL 10.
**Candidate modules:** Portions of `reference` (selinux/, cui/ subdirs),
portions of `cryptography` (policy and configuration content)

### Component 3 — UMRS Operations
**Audience:** System administrators and operators deploying and running UMRS.
**Purpose:** Deployment, operational procedures, key management, tools,
and operational logging guidance.
**Candidate modules:** `deployment`, `operations`, `umrs-tools`,
portions of `logging-audit` (operational pages)

### Component 4 — UMRS High-Assurance Development
**Audience:** Rust developers working on UMRS or building high-assurance
systems using UMRS as a reference.
**Purpose:** Development guidelines, high-assurance patterns, software
library reference, crypto programming, developer logging guidance.
**Candidate modules:** `devel`, `patterns`, portions of `reference`
(rust-style-guide, secure-bash, secure-python), portions of
`cryptography` (programming content), portions of `logging-audit`
(structural/developer content)

### Component 5 — UMRS Use of AI
**Audience:** Technical leads, evaluators, and anyone assessing how AI
contributes to high-assurance development.
**Purpose:** Demonstrating how an AI with the right knowledge base
contributes to high-quality software with proper cross-referencing,
secure code, and excellent tooling.
**Candidate modules:** `ai-transparency`

### Collection Home
A lightweight landing page — not a documentation set itself. Contains
tiles/cards linking to each of the five components. No content of its own
beyond the directory and brief descriptions.

---

## The Hard Problems You Must Resolve in the Plan

Several modules contain content that serves multiple audiences. You must
make an explicit recommendation for each one. Do not leave these as
open questions in the plan — recommend and justify.

### Split Candidates

**`cryptography/`**
Contains both policy/configuration content (belongs in CUI Labeling)
and programming content (belongs in Development). Pages to evaluate:
- `crypto-policy-tiers.adoc` — likely CUI Labeling
- `fips-cryptography-cheat-sheet.adoc` — likely Development
- `crypto-cpu-extensions.adoc` — likely Development
- `crypto-post-quantum.adoc` — evaluate audience
- `crypto-usage-map.adoc` — evaluate audience
- `key-recommendation-list.adoc` — evaluate audience
- `openssl-no-vendoring.adoc` — likely Development

**`logging-audit/`**
Contains both operational content (belongs in Operations) and structural/
developer content (belongs in Development). Pages to evaluate:
- `auditing-noise.adoc` — likely Operations
- `boot-id.adoc` — evaluate (used by both)
- `how-to-structure-log.adoc` — likely Development
- `log-lifecycle-model.adoc` — likely Development
- `logging-capacity.adoc` — likely Operations
- `log-tuning.adoc` — likely Operations
- `structured-logging.adoc` — likely Development

**`reference/`**
Contains SELinux/CUI reference (belongs in CUI Labeling) and developer
reference (belongs in Development). The `selinux/` and `cui/`
subdirectories go to CUI Labeling. The following go to Development:
- `rust-style-guide.adoc`
- `secure-bash.adoc`
- `secure-python.adoc`
- `compliance-frameworks.adoc` — evaluate audience
- `cpu-extensions.adoc` — evaluate (overlaps with cryptography)
- `kernel-probe-signals.adoc` — likely Development

**`glossary/`**
Currently a single `index.adoc`. Recommendation options:
- Option A: Shared partial — one canonical glossary included into
  each component. Single source of truth, no duplication.
- Option B: Each component carries its own scoped glossary containing
  only terms relevant to its audience.
- Option C: Glossary lives in The UMRS Project component and all other
  components cross-reference it.

Evaluate the current glossary content and recommend one option with
justification. Note that Option A requires understanding Antora partials.

---

## Antora Concepts the Plan Must Address

### Component Structure
Each new component requires:
```
docs/<component-name>/
├── antora.yml          ← name, title, version, start_page, nav list
└── modules/
    └── ROOT/           ← or named modules if needed
        ├── nav.adoc    ← scoped navigation for this component only
        ├── pages/      ← all .adoc content pages
        ├── images/     ← images used by this component
        └── partials/   ← reusable fragments (if needed)
```

### The `antora.yml` for each component must include:
```yaml
name: umrs-<component>      # e.g. umrs-project, umrs-cui, umrs-operations
title: <Human Title>
version: latest
start_page: index.adoc
nav:
  - modules/ROOT/nav.adoc
```

### The playbook must list all components:
```yaml
content:
  sources:
    - url: .
      branches: main
      start_paths:
        - docs/umrs-home
        - docs/umrs-project
        - docs/umrs-cui
        - docs/umrs-operations
        - docs/umrs-development
        - docs/umrs-ai
```

### Cross-component links use this syntax:
```asciidoc
xref:umrs-development:high-assurance-patterns.adoc[High-Assurance Patterns]
xref:umrs-cui:understanding-selinux.adoc[Understanding SELinux]
```
The format is `xref:<component-name>:<page>.adoc[Link Text]`

Existing cross-references within the current monolithic structure will
break after migration and must be updated. The plan must include a
strategy for finding and updating all cross-references.

### Partials (shared content):
A partial is a reusable AsciiDoc fragment stored in `modules/ROOT/partials/`
and included into pages with:
```asciidoc
include::partial$filename.adoc[]
```
Partials are component-scoped — a partial in one component cannot be
directly included by another component. For truly shared content across
components, the recommendation is to maintain it in one component and
cross-reference it rather than include it.

---

## What the Plan Must Contain

Deliver the plan as a structured document. It must contain all of the
following sections. Do not omit any.

### Section 1 — Current State Assessment
Summary of what you found after reading all nav.adoc and index.adoc
files. Note anything unexpected, inconsistent, or that affects the
migration approach.

### Section 2 — Component Definitions
For each of the five components plus the collection home:
- Component name and `antora.yml` values
- Audience statement (one sentence)
- Purpose statement (two sentences maximum)
- Complete list of pages assigned to this component with their
  source path in the current structure

### Section 3 — Split Decisions
For every module identified as a split candidate:
- Your recommendation (which pages go where)
- Justification (audience, purpose, cross-reference implications)
- Whether any content needs to be duplicated, split, or shared via
  a different mechanism

### Section 4 — Glossary and Reference Strategy
Your recommendation for glossary and reference handling with full
justification. If recommending partials, explain the mechanism clearly.

### Section 5 — Cross-Reference Audit
Strategy for finding all existing cross-references (xref:: and
include:: directives) and updating them after migration. Include
the grep/rg command you will use to find them all before migration
begins.

### Section 6 — New Navigation Design
For each component, draft the `nav.adoc` showing the proposed
left-navigation structure. This is the most important deliverable
for review — the nav defines the reader experience.

### Section 7 — Migration Sequence
The order in which components will be created and content moved.
Must be sequenced to minimize broken references at each step.
Identify which component migrates first and why.

### Section 8 — Rollback Strategy
How to recover the current working state if the migration goes wrong.
The current site must remain buildable throughout the migration.
Recommend whether to use a feature branch or another mechanism.

### Section 9 — Risks and Open Questions
Anything that could go wrong, anything that requires a decision from
the project owner before proceeding, and anything discovered during
research that changes the scope or approach.

---

## Constraints

- **Do not touch `_scratch/` or `new-stuff/`** — these are staging
  areas and scratch notes, not production content.
- **Do not modify `supplemental-ui/`** — the custom UI carries over
  as-is; note in the plan where it goes in the new structure.
- **Do not modify `images/`** — inventory which images are used by
  which components; each component may need its own copy or a shared
  reference.
- **The current site must remain buildable** until migration is
  complete and the new structure is verified.
- **One component at a time** — the migration sequence must move one
  complete component at a time, verify it builds, then proceed to
  the next.
- **Nothing gets deleted** until the new structure is verified to
  build cleanly and all cross-references resolve.

---

## Definition of Done for the Plan

The plan is complete when:
1. Every existing page has an explicit assigned destination
2. Every split decision has a recommendation and justification
3. Every component has a drafted nav.adoc
4. The cross-reference audit strategy is defined
5. The migration sequence is ordered and justified
6. A rollback strategy exists
7. All open questions are listed

Present the completed plan. Wait for explicit approval before
touching any file.
