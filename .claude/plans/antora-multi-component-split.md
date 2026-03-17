# Plan: Antora Multi-Component Documentation Split

**Status:** Approved — execute after antora-doc-theme is complete

**Sequencing:** Execute `antora-doc-theme.md` FIRST, then this plan. Theme is independent and benefits the split (build verification runs against final UI).

**ROADMAP alignment:** G8 (Human-Centered Design), G10 (AI Transparency)

**Source:** `.claude/jamies_brain/doc-restructure.md` (Jamie Adams via Claude Chat)

**Prerequisite:** Documentation restructure (module cleanup) is COMPLETE (2026-03-12). This plan is the next phase: splitting the monolithic site into audience-focused components.

---

## Problem

The UMRS documentation is a **single monolithic Antora component** (`name: umrs`). All 13 modules appear in one left navigation panel. This means:

- A security administrator configuring CUI sees developer pattern libraries in their nav
- A Rust developer sees deployment procedures they'll never run
- An evaluator assessing AI transparency wades through kernel lockdown steps
- The nav is overwhelming — 13 modules, no audience filtering

The goal is **audience-focused documentation sets** — each reader sees only what they need, in a nav designed for their tasks.

---

## Current State

Single component at `docs/` with `antora.yml` listing 13 module navs:

```
docs/
├── antora.yml          ← single component: name=umrs
├── modules/
│   ├── ROOT/           ← landing page, introduction
│   ├── architecture/   ← history, MLS model, library model
│   ├── ai-transparency/← AI agent roles, RAG, corpus, workflow
│   ├── cryptography/   ← FIPS, PQC, crypto policy, CPU extensions
│   ├── deployment/     ← RHEL 10, IMA/EVM, kernel lockdown, filesystem
│   ├── devel/          ← build tooling, patterns, cargo, compliance
│   ├── glossary/       ← single index.adoc
│   ├── logging-audit/  ← structured logging, boot-id, log lifecycle
│   ├── operations/     ← admin, IMA/EVM ops, key management, tools
│   ├── patterns/       ← high-assurance patterns library
│   ├── reference/      ← SELinux ref, CUI ref, rust style, secure bash
│   ├── security-concepts/ ← reference monitor, RTB, MLS security model
│   └── umrs-tools/     ← umrs-ls, umrs-state, umrs-logspace, shred
├── supplemental-ui/    ← custom CSS, header, footer, logo
├── images/             ← logos and banners
├── new-stuff/          ← staging area (do not migrate)
└── _scratch/           ← scratch notes (do not migrate)
```

Playbook: single source at `start_path: docs`, single component `umrs`.

---

## Target: 5 Components + Collection Home

```
docs/
├── umrs-home/          ← collection landing page (tiles to each component)
├── umrs-project/       ← Component 1: intro, history, security concepts
├── umrs-cui/           ← Component 2: SELinux MLS, CUI labeling, crypto policy
├── umrs-operations/    ← Component 3: deployment, ops, tools, operational logging
├── umrs-development/   ← Component 4: dev guide, patterns, reference, dev logging
├── umrs-ai/            ← Component 5: AI transparency, agent roles, RAG, workflow
├── supplemental-ui/    ← shared theme (unchanged)
├── images/             ← shared images (inventory per-component usage)
├── new-stuff/          ← staging area (unchanged)
└── _scratch/           ← scratch notes (unchanged)
```

Each component is self-contained with its own `antora.yml`, `modules/ROOT/`, `nav.adoc`, and `pages/`.

---

## Component Definitions

### Collection Home (`umrs-home`)

**Audience:** Everyone — first point of contact.
**Purpose:** Lightweight landing page with tiles/cards linking to each component. No content of its own beyond directory and descriptions.

```yaml
name: umrs-home
title: UMRS Documentation
version: latest
start_page: index.adoc
```

### Component 1 — The UMRS Project (`umrs-project`)

**Audience:** Anyone new to UMRS, evaluators, Five Eyes community members.
**Purpose:** Introduction, historical background, security concepts, and the rationale for high-assurance design.

**Source modules:**
- `ROOT/` (landing page, introduction)
- `architecture/` (history, MLS model, library model)
- `security-concepts/` (reference monitor, RTB, MLS security model)

```yaml
name: umrs-project
title: The UMRS Project
version: latest
start_page: index.adoc
```

### Component 2 — CUI Labeling (`umrs-cui`)

**Audience:** Security administrators, classification officers, operators configuring CUI enforcement.
**Purpose:** Understanding SELinux MLS in the context of CUI, enabling and configuring CUI labeling on RHEL 10.

**Source modules:**
- `reference/` pages: `selinux/` subdirectory, `cui/` subdirectory
- `cryptography/` pages: policy/configuration content (crypto-policy-tiers, key-recommendation-list, crypto-usage-map)

```yaml
name: umrs-cui
title: UMRS CUI Labeling
version: latest
start_page: index.adoc
```

### Component 3 — Operations (`umrs-operations`)

**Audience:** System administrators and operators deploying and running UMRS.
**Purpose:** Deployment, operational procedures, key management, tools, and operational logging guidance.

**Source modules:**
- `deployment/` (all pages)
- `operations/` (all pages)
- `umrs-tools/` (all pages)
- `logging-audit/` pages: operational content (auditing-noise, logging-capacity, log-tuning)

```yaml
name: umrs-operations
title: UMRS Operations
version: latest
start_page: index.adoc
```

### Component 4 — Development (`umrs-development`)

**Audience:** Rust developers working on UMRS or building high-assurance systems.
**Purpose:** Development guidelines, high-assurance patterns, software library reference, crypto programming, developer logging guidance.

**Source modules:**
- `devel/` (all pages)
- `patterns/` (all pages)
- `reference/` pages: rust-style-guide, secure-bash, secure-python, compliance-frameworks, kernel-probe-signals, cpu-extensions
- `cryptography/` pages: programming content (fips-cheat-sheet, crypto-cpu-extensions, openssl-no-vendoring, crypto-post-quantum)
- `logging-audit/` pages: structural/developer content (how-to-structure-log, log-lifecycle-model, structured-logging, boot-id)

```yaml
name: umrs-development
title: UMRS Development
version: latest
start_page: index.adoc
```

### Component 5 — AI Transparency (`umrs-ai`)

**Audience:** Technical leads, evaluators, anyone assessing AI's contribution to high-assurance development.
**Purpose:** Demonstrating how AI with the right knowledge base contributes to quality software.

**Source modules:**
- `ai-transparency/` (all pages)

```yaml
name: umrs-ai
title: UMRS Use of AI
version: latest
start_page: index.adoc
```

---

## Split Decisions Required

These modules contain content serving multiple audiences. Each page must be explicitly assigned.

### `cryptography/` — split between CUI Labeling and Development

| Page | Destination | Rationale |
|---|---|---|
| `crypto-policy-tiers.adoc` | **umrs-cui** | Policy configuration for admins |
| `key-recommendation-list.adoc` | **umrs-cui** | Key management for admins |
| `crypto-usage-map.adoc` | **umrs-cui** | Which crypto is used where — admin context |
| `fips-cryptography-cheat-sheet.adoc` | **umrs-development** | Developer reference |
| `crypto-cpu-extensions.adoc` | **umrs-development** | Developer/implementer reference |
| `crypto-post-quantum.adoc` | **umrs-development** | Forward-looking dev guidance |
| `openssl-no-vendoring.adoc` | **umrs-development** | Build decision for developers |

**Senior-tech-writer must verify:** Read each page and confirm the assignment. Some pages may need a different split or may need to be duplicated with audience-specific framing.

### `logging-audit/` — split between Operations and Development

| Page | Destination | Rationale |
|---|---|---|
| `auditing-noise.adoc` | **umrs-operations** | Operational tuning |
| `logging-capacity.adoc` | **umrs-operations** | Capacity planning for admins |
| `log-tuning.adoc` | **umrs-operations** | Operational tuning |
| `how-to-structure-log.adoc` | **umrs-development** | Developer guidance |
| `log-lifecycle-model.adoc` | **umrs-development** | Architecture for developers |
| `structured-logging.adoc` | **umrs-development** | Developer guidance |
| `boot-id.adoc` | **umrs-development** | Used by both, but primarily dev reference |

### `reference/` — split between CUI Labeling and Development

| Page/Directory | Destination | Rationale |
|---|---|---|
| `selinux/` subdirectory | **umrs-cui** | SELinux reference for admins |
| `cui/` subdirectory | **umrs-cui** | CUI reference for classification officers |
| `rust-style-guide.adoc` | **umrs-development** | Developer reference |
| `secure-bash.adoc` | **umrs-development** | Developer reference |
| `secure-python.adoc` | **umrs-development** | Developer reference |
| `compliance-frameworks.adoc` | **umrs-project** | Cross-cutting — evaluator audience |
| `cpu-extensions.adoc` | **umrs-development** | Developer/implementer reference |
| `kernel-probe-signals.adoc` | **umrs-development** | Developer reference |

### `glossary/` — strategy decision

**Recommendation: Option C** — Glossary lives in The UMRS Project component (`umrs-project`). All other components cross-reference it via `xref:umrs-project:glossary.adoc[Glossary]`. Single source of truth, no duplication, no partial complexity.

**Rationale:** The glossary is small (single `index.adoc`). Antora partials are component-scoped and cannot cross component boundaries. Duplicating the glossary into each component creates maintenance burden. A cross-reference is simple and always up to date.

---

## Cross-Reference Migration Strategy

All existing `xref::` and `include::` directives will break after migration because page locations change.

### Pre-migration audit

```bash
cd docs && rg --no-heading -n 'xref:|include::' modules/
```

This produces the complete list of cross-references to update.

### Update pattern

| Old (monolithic) | New (multi-component) |
|---|---|
| `xref:devel:high-assurance-patterns.adoc[...]` | `xref:umrs-development:high-assurance-patterns.adoc[...]` |
| `xref:security-concepts:reference-monitor.adoc[...]` | `xref:umrs-project:reference-monitor.adoc[...]` |
| `include::partial$shared-fragment.adoc[]` | Cross-reference instead (partials don't cross components) |

### Post-migration verification

```bash
cd docs && rg --no-heading -n 'xref:[^u]' */modules/  # find any xrefs NOT using new component names
```

---

## Playbook Update

```yaml
content:
  sources:
    - url: .
      branches: HEAD
      start_paths:
        - docs/umrs-home
        - docs/umrs-project
        - docs/umrs-cui
        - docs/umrs-operations
        - docs/umrs-development
        - docs/umrs-ai
```

The old `start_path: docs` line is removed. Each component is a separate start path.

---

## Migration Sequence

One component at a time. Build must pass after each step.

| Step | Action | Verification |
|---|---|---|
| 0 | **Feature branch** — all work on a branch, not main | Branch exists |
| 1 | Create `umrs-home/` skeleton with `antora.yml` + placeholder `index.adoc` | `make docs` passes |
| 2 | Create `umrs-project/` — move ROOT, architecture, security-concepts | `make docs` passes |
| 3 | Create `umrs-ai/` — move ai-transparency (simplest, no splits) | `make docs` passes |
| 4 | Create `umrs-operations/` — move deployment, operations, umrs-tools + ops logging pages | `make docs` passes |
| 5 | Create `umrs-development/` — move devel, patterns + dev reference/logging/crypto pages | `make docs` passes |
| 6 | Create `umrs-cui/` — move SELinux/CUI reference + admin crypto pages | `make docs` passes |
| 7 | **Cross-reference sweep** — update all xrefs to use new component names | `make docs` passes, zero broken links |
| 8 | Move glossary to `umrs-project/`, add cross-references from other components | `make docs` passes |
| 9 | Update `antora-playbook.yml` to multi-source format | `make docs` passes |
| 10 | Remove old monolithic `docs/antora.yml` and empty module dirs | `make docs` passes |
| 11 | Build collection home tiles (depends on doc-theme plan Phase 11) | `make docs` passes |
| 12 | **Final review** — all components, all links, all navs | Full site verified |

**Step 3 (umrs-ai) goes early** because it's the simplest — single module, no splits, minimal cross-references. It validates the multi-component pattern before tackling the complex splits.

---

## Rollback Strategy

- All work on a feature branch — `main` is untouched until migration is verified
- The old monolithic structure remains buildable until Step 10
- If migration fails partway, the branch can be abandoned with zero impact to main
- Git history preserves the complete state of the old structure

---

## Navigation Design

Each component gets a focused `nav.adoc`. Drafts to be produced by senior-tech-writer after reading all current navs and assigning pages. The nav is the most important deliverable for review — it defines the reader experience.

**Constraint:** Each nav should fit on one screen (no scrolling past ~20 items). If a component has more pages than that, use nested sections.

---

## Constraints

- **Do not touch `_scratch/` or `new-stuff/`** — staging areas, not production content
- **Do not modify `supplemental-ui/`** — theme carries over as-is (separate plan: `antora-doc-theme.md`)
- **Do not modify `images/`** — inventory which images are used by which components; copy as needed
- **Current site must remain buildable** throughout migration (feature branch ensures this)
- **One component at a time** — verify build after each step
- **Nothing gets deleted** until new structure builds cleanly and all cross-references resolve

---

## Risks and Open Questions

1. **Cross-reference volume** — how many xrefs exist? The pre-migration audit will quantify this. If hundreds, consider a script to automate updates.
2. **Shared images** — some images may be referenced by multiple components. Antora requires images to be in the component that uses them. Identify shared images during migration.
3. **Glossary growth** — if the glossary outgrows a single page, reconsider the strategy (Option A partials may become necessary).
4. **Search** — Antora's built-in search works across components. Verify after migration.
5. **Supplemental UI** — the theme applies globally to all components. Verify it renders correctly after split.
6. **`compliance-frameworks.adoc`** — assigned to `umrs-project` but could also fit in `umrs-development`. Senior-tech-writer to decide.

---

## Agent Assignment

**The Imprimatur (senior-tech-writer)** owns this migration. Responsibilities:
- Read every `nav.adoc` and `index.adoc` before starting
- Verify all split decisions by reading each page
- Draft new `nav.adoc` for each component
- Execute migration sequence on a feature branch
- Run `make docs` after every step
- Update all cross-references

**Pre-migration dependency:** Senior-tech-writer must produce the nav drafts for all 5 components before any files are moved. The navs are the plan's most important deliverable.

---

## Relationship to Other Plans

- **`antora-doc-theme.md`** — the wizard theme applies globally; can be implemented before or after this split. Phase 11 (collection home tiles) depends on both plans.
- **Completed doc restructure (2026-03-12)** — the module-level cleanup that preceded this plan. That work is done and stable.
