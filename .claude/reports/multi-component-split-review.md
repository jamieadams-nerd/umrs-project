# Review: Plan — Antora Multi-Component Documentation Split

**Reviewer**: The Imprimatur (senior-tech-writer)
**Date**: 2026-03-17
**Plan reviewed**: `.claude/plans/antora-multi-component-split.md`
**Status**: Review complete — report only, no changes made to plan or content

---

## Session Context

Before reviewing the plan, I read the following sources to understand the current state:

- `docs/antora.yml` — current single-component descriptor
- All 13 `nav.adoc` files under `docs/modules/*/`
- `antora-playbook.yml` at the repo root
- Actual page content in `cryptography/`, `logging-audit/`, and `reference/` to verify split decisions
- `docs/modules/glossary/pages/index.adoc` — 889 lines, to assess Option C feasibility
- The doc-team feedback log — for open items that affect migration planning
- My own persistent memory — for current structural state

---

## Finding 1 — Cross-Reference Count

The plan says to run the pre-migration audit and report the count. Results:

**Total xref/include occurrences across all modules**: 743 (107 files)
**Cross-module xrefs specifically** (of the form `xref:module-name:page.adoc`): **272 occurrences across 72 files**

The 272 figure is the working number for migration planning. These are the xrefs that will require a component-name prefix change after migration (e.g., `xref:devel:...` becomes `xref:umrs-development:...`).

Breakdown by affected area:

| Source module | Cross-module xref count |
|---|---|
| `glossary` | 26 |
| `patterns` | 45 |
| `devel` | 28 |
| `security-concepts` | 26 |
| `reference` | 19 |
| `architecture` | 27 |
| `deployment` | 11 |
| `operations` | 11 |
| `ai-transparency` | 5 |
| `logging-audit` | 3 |
| `ROOT` | 16 |
| `cryptography` | 8 |

**Assessment**: 272 cross-module xrefs is a moderate workload — not trivial, but not prohibitive. This is manageable with a scripted find-and-replace per component migration step. A manual sweep without scripting is risky (easy to miss one). Recommendation: prepare a sed/rg substitution command for each step. Do not rely on manual search.

**Risk flag**: The glossary has 26 cross-module xrefs pointing outward. After moving glossary to `umrs-project`, every one of those xrefs from other modules pointing _into_ the glossary must be updated to use `xref:umrs-project:glossary:index.adoc[...]`. This is the highest cross-module xref density relative to a single page.

---

## Finding 2 — Current Module Count vs. Plan Description

The plan states the current site has 13 modules. The current `antora.yml` registers 13 nav files and I confirmed all 13 exist: ROOT, architecture, security-concepts, deployment, devel, patterns, umrs-tools, operations, logging-audit, cryptography, reference, glossary, ai-transparency.

The plan's "Current State" section is accurate.

---

## Finding 3 — Split Decisions: Cryptography Module

I read all 7 cryptography pages. Verdict on each:

| Page | Plan assignment | My assessment | Notes |
|---|---|---|---|
| `crypto-policy-tiers.adoc` | umrs-cui | **Correct** | Policy tiers framework — directly governs algorithm choice in deployed systems. Admin audience. |
| `key-recommendation-list.adoc` | umrs-cui | **Borderline** | Content describes key types for signing audit logs and system artifacts. This reads as operator/admin reference, not developer programming guide. Assignment is defensible. |
| `crypto-usage-map.adoc` | umrs-cui | **Correct but dual-audience** | Self-describes as "cross-reference for auditors and engineers." An auditor reads it to verify algorithm choices; an engineer reads it to identify alignment patterns. This page genuinely serves both. Consider whether it should be in both components (duplicated) or whether one audience is primary. I lean toward keeping it in umrs-cui as the plan proposes — auditors need it for compliance verification, and `xref:` can link to it from umrs-development. |
| `fips-cryptography-cheat-sheet.adoc` | umrs-development | **Correct** | Developer quick-reference. Algorithm tables, key sizes, "new design" guidance. |
| `crypto-cpu-extensions.adoc` | umrs-development | **Correct** | Currently a stub. Content scope: AES-NI, SHA extensions, ELF binary auditing. Developer-facing. |
| `openssl-no-vendoring.adoc` | umrs-development | **Correct** | Build decision for Rust developers. Direct Cargo.toml guidance. |
| `crypto-post-quantum.adoc` | umrs-development | **Partially correct — see below** | |

**Exception: `crypto-post-quantum.adoc`**

This page has a dual-audience problem more acute than `crypto-usage-map.adoc`. It currently contains:
- The quantum threat model and CRQC timeline — conceptual, fits umrs-project or umrs-cui
- FIPS 203/204/205 algorithm parameter tables — developer reference
- RHEL 10 PQC availability matrix — admin/deployment reference
- Migration guidance — both admin and developer

Assigning it entirely to umrs-development is defensible (the algorithm tables are the primary content), but an admin reading umrs-operations or umrs-cui who needs to understand the FIPS/PQC mutual exclusion will have no local reference. **Recommendation**: Accept the plan's umrs-development assignment, but ensure umrs-cui's `crypto-policy-tiers.adoc` explicitly cross-references `umrs-development:crypto-post-quantum.adoc` for the FIPS/PQC constraint. A single cross-component xref solves this without duplication.

---

## Finding 4 — Split Decisions: Logging-Audit Module

I read the actual page content. Verdict:

| Page | Plan assignment | My assessment | Notes |
|---|---|---|---|
| `auditing-noise.adoc` | umrs-operations | **Correct** | Operational tuning — reduce auditd noise in production. Admin task. |
| `logging-capacity.adoc` | umrs-operations | **Correct** | Capacity planning. Storage sizing, retention planning. Admin task. |
| `log-tuning.adoc` | umrs-operations | **Correct** | journald filesystem tuning. Admin task. |
| `structured-logging.adoc` | umrs-development | **Correct** | Explains the syslog key=value emission pattern. Developer architecture. |
| `how-to-structure-log.adoc` | umrs-development | **Correct** | How to write log events as a developer. Developer guide. |
| `log-lifecycle-model.adoc` | umrs-development | **Borderline — see below** | |
| `boot-id.adoc` | umrs-development | **Correct with caveat** | |

**Exception: `log-lifecycle-model.adoc`**

This page describes the four log lifecycle states (Active, Inactive/Rotated, Archived, Destroyed). It opens with: "The terms are heritage vocabulary, not obsolete architecture." This is conceptual/architectural content that an administrator needs to understand custody chain and retention policy, and that a developer needs to understand when designing log-handling code. The plan assigns it to umrs-development. That is defensible, but an operator managing log retention may encounter it without context.

**Recommendation**: Accept the umrs-development assignment. Add a cross-reference from umrs-operations' logging section to `umrs-development:log-lifecycle-model.adoc`. The lifecycle model is more architectural than operational.

**Exception: `boot-id.adoc`**

The plan notes "used by both, but primarily dev reference." Having read it, I agree. The page explains the boot ID UUID, its kernel origin, its journald use, and how UMRS uses it as a runtime event epoch. This is developer architecture — the concept governs how UMRS correlates events. An operator does not need to configure it. Assignment to umrs-development is correct.

---

## Finding 5 — Split Decisions: Reference Module

I read the nav to verify the assignments. Verdict:

| Item | Plan assignment | My assessment |
|---|---|---|
| `selinux/` subdirectory (11 pages) | umrs-cui | **Correct** — SELinux context model reference for admins configuring MLS |
| `cui/` subdirectory (2 pages) | umrs-cui | **Correct** — CUI category reference |
| `rust-style-guide.adoc` | umrs-development | **Correct** |
| `secure-bash.adoc` | umrs-development | **Correct** |
| `secure-python.adoc` | umrs-development | **Correct** |
| `compliance-frameworks.adoc` | umrs-project | **Correct — see discussion** |
| `cpu-extensions.adoc` | umrs-development | **Correct** |
| `kernel-probe-signals.adoc` | umrs-development | **Correct** |

**Discussion: `compliance-frameworks.adoc`**

The plan flags this as a decision point. The page is a registry of governing standards (NIST 800-53, 800-171, 800-218 SSDF, CMMC, NSA RTB, FIPS) with their versions and roles in UMRS. It explicitly states: "Control citations in the source code refer to the frameworks listed here." It contains a cross-reference to `devel:compliance-annotations.adoc`.

This page serves evaluators assessing UMRS compliance, developers understanding what they're annotating, and auditors verifying control coverage. Assigning it to umrs-project is the right choice. It belongs in the "about the project" layer, not inside the developer guide, and not inside CUI operations. It is a project-level reference that all other components can cross-reference.

**Untracked item: `reference/pages/apparmor/`**

The `reference/pages/` directory contains an `apparmor/` subdirectory. It is empty — no files. No nav entry references it. This is not a migration concern but should be noted: the empty directory should be cleaned up before migration begins to avoid confusion.

---

## Finding 6 — Component Boundaries: Are the 5 Components the Right Groupings?

**Overall assessment**: The 5-component structure is sound. Each maps to a distinct audience with distinct needs. I would not merge or reorder any of them.

Specific observations:

**umrs-project (Component 1)**: Correct scope. ROOT, architecture, security-concepts. This is the project's foundational layer. Its audience — evaluators, Five Eyes community, newcomers — needs history, concepts, and rationale, not procedures. The compliance-frameworks.adoc belongs here as an evaluator resource. One question: `architecture/pages/` has 20+ pages including extensive history deep-dives. At migration time, the umrs-project nav will need careful sectioning (Architecture Overview / History / Security Concepts) to avoid overwhelming the reader. This is a nav design task, not a structure problem.

**umrs-cui (Component 2)**: Narrowly scoped to SELinux/MLS and CUI configuration — appropriate. Risk: this component may appear thin to a CUI practitioner who wants more operational context. The cryptography admin pages and reference pages fill it out somewhat, but the nav might feel sparse. Evaluate after drafting the nav.

**umrs-operations (Component 3)**: Well-scoped. deployment + operations + umrs-tools + operational logging. This is the largest component by page count and the most immediately useful to system administrators. The nav will need good sectioning to avoid scrolling past 20 items.

**umrs-development (Component 4)**: Also well-scoped, and will be the largest component by raw page count (devel + patterns + developer reference/logging/crypto pages). The patterns module alone has 15 pages. The devel module has 15+ pages. Total will exceed 30 pages easily. **This is the component most at risk of a navigation size problem.** See Finding 7.

**umrs-ai (Component 5)**: Clean, minimal, correct. 13 pages, one module, no splits. The simplest migration step.

---

## Finding 7 — Navigation Size Analysis

The plan establishes a 20-item nav constraint. Let me assess each component against it.

**umrs-project**:
- ROOT module: 6 nav items
- architecture module: 22 nav items (currently — includes all history deep-dives)
- security-concepts module: 6 nav items
- **Total if flat**: ~34 items. Exceeds constraint.
- **Mitigation**: The architecture history section (13 pages under `history/`) can be grouped under a single collapsible section header. With proper grouping, this fits.

**umrs-cui**:
- selinux/ reference: 12 nav items
- cui/ reference: 2 nav items
- cryptography admin pages: 3 nav items
- **Total**: ~17 items. Fits comfortably.

**umrs-operations**:
- deployment: 13 nav items
- operations: 12 nav items
- umrs-tools: 7 nav items
- logging ops pages: 3 nav items
- **Total if flat**: ~35 items. Exceeds constraint.
- **Mitigation**: RHEL vs Ubuntu grouping already exists in deployment. Operations sections are naturally grouped. With collapsible sections for deployment/operations/tools, this fits in one screen as a collapsed-by-default nav.

**umrs-development**:
- devel: 17 nav items
- patterns: 15 nav items
- reference dev pages (rust-style-guide, secure-bash, secure-python, cpu-extensions, kernel-probe-signals, compliance-frameworks): 6 nav items
- logging dev pages: 4 nav items
- cryptography dev pages: 4 nav items
- **Total**: ~46 items. Significantly exceeds constraint.
- **Mitigation required**: This component needs aggressive grouping into collapsible sections. Proposed structure:
  - Design Guides (devel sections as-is)
  - High-Assurance Pattern Library (patterns module — collapsed by default)
  - Language & Style (rust-style, secure-bash, secure-python)
  - Platform Reference (cpu-extensions, kernel-probe-signals)
  - Cryptography for Developers (4 pages)
  - Logging for Developers (4 pages)
  - Compliance Reference (compliance-frameworks, compliance-annotations)

  With this grouping and all sections collapsible, the first-view nav fits on one screen.

**umrs-ai**:
- 13 nav items, already well-structured.
- Fits comfortably.

**umrs-home**:
- Single landing page with tiles. No nav size concern.

**Conclusion**: Navigation size is manageable but umrs-development requires deliberate nav architecture before migration begins. The plan correctly flags "Drafts to be produced by senior-tech-writer after reading all current navs" — this is the right gate.

---

## Finding 8 — Migration Sequence Assessment

The sequence is logically correct. Specific observations:

**Step 3 (umrs-ai first)** — Strongly agree. 13 pages, no splits, minimal outbound cross-references (5 cross-module xrefs). This is the right validation step for the multi-component pattern.

**Step 5 (umrs-development) vs Step 6 (umrs-cui)** — Order is correct. umrs-development gets the complex mixed-origin pages (logging dev, reference dev, crypto dev). These should be stabilized before umrs-cui takes the remaining reference pages.

**Step 7 (cross-reference sweep)** — This is the highest-risk step. 272 cross-module xrefs touching 72 files. A script to transform xref prefixes is not optional — it is mandatory. Manual sweep at this scale will miss references. Recommend preparing the substitution patterns before Step 2:

```bash
# Example substitution patterns (to be verified before use)
# devel: → umrs-development:
# patterns: → umrs-development:
# architecture: → umrs-project:
# security-concepts: → umrs-project:
# deployment: → umrs-operations:
# operations: → umrs-operations:
# umrs-tools: → umrs-operations:
# logging-audit: → umrs-operations: (ops pages) or umrs-development: (dev pages)
# reference: → umrs-cui: or umrs-development: (split required)
# cryptography: → umrs-cui: or umrs-development: (split required)
# glossary: → umrs-project:
# ROOT: → umrs-project: or umrs-home: (verify per xref)
```

Note: the logging-audit and reference and cryptography splits mean their xrefs cannot be handled by a single substitution rule — the target component depends on which page is being referenced. These require page-specific substitution or a lookup table.

**Step 10 (remove old antora.yml)** — The plan says "remove old monolithic `docs/antora.yml` and empty module dirs." This step should also include removing the old single `start_path: docs` from the playbook, which should already be done in Step 9 (playbook update). Verify the sequence is clear on this: Step 9 replaces the playbook config; Step 10 removes the old component descriptor. Do not leave both active simultaneously at any point.

**Missing step: feature branch creation verification** — Step 0 says "Feature branch — all work on a branch, not main." The verification criterion is just "Branch exists." This is adequate, but given the scope of this migration, add explicit confirmation that the old monolithic structure builds cleanly on main *before* branching. If there are pre-existing build errors at branch point, they will be harder to distinguish from migration-introduced errors. (The current known pre-existing errors in `ubuntu.adoc` are documented but should be noted in the branch commit message.)

---

## Finding 9 — Glossary Strategy: Option C Assessment

Option C is correct. The reasoning in the plan is sound.

Supporting observations:

1. The glossary is 889 lines — substantial but a single document. It is growing (expanded 2026-03-15 with 10 new terms; expanded again with crypto terms). A single-file glossary in umrs-project is the right long-term home.

2. The glossary has 26 outbound cross-module xrefs. After migration, these become 26 xrefs that cross component boundaries (e.g., `xref:umrs-project:reference-monitor.adoc` from a glossary term pointing to security-concepts). Antora handles cross-component xrefs correctly with the `component:module:page.adoc` or `component::page.adoc` syntax. This is expected to work.

3. The glossary is referenced by every other component's content (developers, admins, evaluators all use it). A central location in umrs-project makes the cross-reference syntax consistent: every component that references a glossary term uses the same prefix.

4. The alternative — duplicating the glossary — is a maintenance trap. With 889 lines and active growth, four copies would diverge within weeks.

**Caveat on Option C**: When moving the glossary to umrs-project, the existing inbound xrefs from other modules pointing to glossary terms (e.g., `xref:glossary:index.adoc[...]`) must all be updated to `xref:umrs-project:glossary:index.adoc[...]` or `xref:umrs-project::glossary:index.adoc[...]` depending on Antora's cross-component xref syntax. Count these carefully before Step 8.

**Antora cross-component xref syntax reminder**: The format is `xref:component:module:page.adoc[]` where the component is the `name` field from the target's `antora.yml`. If glossary lives in umrs-project's ROOT module (no named module), the syntax is `xref:umrs-project::filename.adoc[]`. The plan must specify where in umrs-project the glossary lives — in ROOT (no module prefix needed) or in a named `glossary` module.

---

## Finding 10 — Feasibility and Effort

**Effort estimate**: This is a large but well-defined migration. Nothing about it is architecturally uncertain — the Antora mechanics are understood, the content boundaries are clear, and the rollback strategy is solid (feature branch). The primary risk is the xref update volume.

Rough time breakdown:
- Nav drafts for all 5 components: 4–6 hours (the most important deliverable; cannot be rushed)
- Steps 1–6 (file moves, component scaffolding): 3–5 hours
- Step 7 (xref sweep): 2–4 hours with scripting, 6–10 hours manual
- Steps 8–12 (glossary, playbook, cleanup, verification): 2–3 hours
- **Total**: 11–18 hours of focused work

**Biggest risk**: Split-module xref updates for `logging-audit`, `reference`, and `cryptography`. Pages from these modules go to different destination components depending on their content. No single substitution rule handles them. Errors here will produce broken links that the build will catch — but only if you run `make docs` after every step as specified. The build gate is the safety net; the sequence discipline is what makes the safety net effective.

**Second risk**: The `devel/nav.adoc` currently contains cross-module xrefs pointing to `architecture:rationale-strongly-typed.adoc` and `architecture:library-model.adoc`. These appear in the nav file itself, not a page body. Nav-file xrefs must be updated too — they are not page body content and a page-level grep will miss them unless the glob includes nav.adoc files explicitly.

---

## Finding 11 — Relationship to Doc-Theme Plan

The doc-theme plan (`antora-doc-theme.md`) is marked **Approved — ready for implementation** and operates entirely in `supplemental-ui/` CSS and partials. It explicitly does not touch content files.

**Sequencing**: The two plans are independent at the execution level. The theme can be applied before, after, or during the component split without conflict. One shared concern: both plans reference Phase 11 (collection home tiles). The doc-theme plan defers tile content markup to a follow-on task. The multi-component split plan marks tile build as the last step (Step 11) and notes it "depends on doc-theme plan Phase 11."

**Decision required**: Either:
1. Complete the component split without tiles (stop at Step 10), then implement the theme, then add tiles together.
2. Implement the theme first (it is approved and ready), then execute the component split, then add tiles.

Option 2 is cleaner — the theme is already approved, it does not block or complicate the split, and having the theme applied before the split means the split's build verification steps run against the final UI, catching any theme/nav interaction issues early. **Recommendation: execute doc-theme plan first, then the component split.**

The tile CSS (Phase 10 of the doc-theme plan) is already included in the theme — only the content markup (`[.tile-grid]`, `[.tile]`) is deferred. After the split creates `umrs-home`, the home page content markup is a single-day task.

---

## Finding 12 — Open Items From Feedback Log Relevant to Migration

Several open entries in the feedback log affect migration scope:

1. **`rhel10-install.adoc` potential duplicate** (2026-03-10) — Unresolved. Before migration, this duplicate must be resolved. The correct page must be established before deployment content is moved to umrs-operations. An unresolved duplicate creates two pages with conflicting content in the same component.

2. **`admin/` module originals still present** (2026-03-10) — Unresolved. The admin/ pages were copied to operations/ but the originals remain. The admin/ module is not registered in `antora.yml` so it does not affect the current build, but before migration it should be resolved. If admin/ is unregistered at migration time, the situation is that its files exist on disk but are invisible. This is confusing during a major restructure. Recommendation: resolve before Step 1.

3. **`security-model.adoc` redirect stub** (2026-03-10) — Unresolved. This file is at `ROOT/pages/security-model.adoc` and contains only a redirect notice. It is in the ROOT module, which becomes umrs-project at migration. If it is not resolved before migration, umrs-project will contain a known-stub page. Not a blocker, but clean up first.

4. **`selinux-registry.txt` duplicate** (2026-03-11) — Unresolved. A byte-for-byte duplicate of `umrs-mls-registry.txt` remains in `reference/pages/`. Not a blocker, but should be cleaned before the reference module is split into umrs-cui.

**Recommendation**: Resolve all four open items before the feature branch is created. The split should start from a clean state.

---

## Summary: Findings and Recommendations

| # | Finding | Severity | Recommendation |
|---|---|---|---|
| 1 | Cross-module xref count: 272 across 72 files | Planning input | Prepare substitution scripts; do not rely on manual search |
| 2 | Module count in plan is accurate | None | No action |
| 3 | Cryptography splits mostly correct; `crypto-usage-map.adoc` and `crypto-post-quantum.adoc` are dual-audience | Minor | Accept plan assignments; add cross-component xrefs for dual-audience pages |
| 4 | Logging splits are correct; `log-lifecycle-model.adoc` is borderline but dev assignment is defensible | Minor | Add cross-component xref from operations to lifecycle model |
| 5 | Reference splits are correct; `compliance-frameworks.adoc` → umrs-project is the right call; `apparmor/` dir is empty | Minor | Clean up empty `apparmor/` dir before migration |
| 6 | Component boundaries are correct; no merges or splits recommended | None | Proceed as planned |
| 7 | umrs-development nav will exceed 30+ items; requires deliberate nav sectioning before migration | Medium | Draft umrs-development nav first; its complexity will drive the most nav architecture work |
| 8 | Migration sequence is correct; Step 7 xref sweep requires scripting; Step 10 sequencing note added | Medium | Prepare xref substitution patterns before Step 2; note the nav-file xref risk |
| 9 | Option C (glossary in umrs-project) is correct; cross-component xref syntax must be verified | Minor | Confirm Antora cross-component xref syntax before Step 8 |
| 10 | Feasibility: medium-large effort, 11–18 hours; xref update is the primary risk | Planning input | Use build gate after every step; automate xref updates |
| 11 | Doc-theme plan is independent; recommend completing theme before the split | Sequencing | Run doc-theme first; both plans benefit from the ordering |
| 12 | Four open feedback items should be resolved before branching | Prerequisite | Resolve `rhel10-install.adoc` duplicate, `admin/` originals, `security-model.adoc` stub, `selinux-registry.txt` before Step 0 |

---

## Pre-Migration Gate: Minimum Conditions Before Step 0

Do not create the feature branch until all of the following are true:

1. `make docs` passes cleanly from the main branch (only the known `ubuntu.adoc` pre-existing errors are acceptable)
2. The `rhel10-install.adoc` duplicate is resolved with Jamie's decision
3. The `admin/` originals are cleaned up or Jamie has explicitly deferred this
4. The `security-model.adoc` redirect stub is resolved or explicitly retained by Jamie
5. The empty `apparmor/` directory is removed
6. Nav drafts for all 5 components are written and reviewed by Jamie
7. Xref substitution patterns are documented (which module names map to which component names)

Items 6 and 7 are the most important. The navs are the plan's primary reader-experience deliverable and must be designed before any files move.

---

*This report is provided for information and decision-making. No changes to any documentation file were made during this review.*
