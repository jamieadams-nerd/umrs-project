# Cross-Team Notes

Shared across all agents. Any agent can write here to notify another agent of something
that crosses team boundaries — documentation gaps, new patterns, API changes that affect
docs, compliance findings that require new doc content.

**Read this file at session start.** Check for open entries addressed to your agent role.
Mark entries `resolved` when acted on. Do not delete entries.
Resolved entries are periodically archived to `notes-archive-*.md`.

---

### 2026-03-20 — doc-sync: umrs-tui — Kernel tab Phases 1-4 complete

**To:** tech-writer (Von Neumann)
**Status:** partial — crate rename addressed; ops/tool docs pending

Rusty completed kernel tab enhancement Phases 1-4. Note: `umrs-tui` has been renamed to
`umrs-ui` (library) and `umrs-uname` (binary). Doc sync pass (2026-03-22) updated all
`umrs-tui` references in docs.

Still needed — operations/tool reference docs:
- Key bindings table: add `?` / `F1` → ShowHelp
- Trust Evidence tab section: update downgrade reasons description to reflect positive framing
- Kernel Security tab section: note integer values now include plain-language annotations
- Source of truth for all indicator descriptions: `.claude/agent-memory/security-auditor/indicator-definitions-plain-language.md`
- Target file: `docs/modules/umrs-tools/pages/umrs-state.adoc` (or a new umrs-uname page — senior-tech-writer created `umrs-uname.adoc` on 2026-03-22)

---

### 2026-03-20 — CRITICAL: Phase 1/Phase 2 Positioning (Jamie directive)

**To:** ALL AGENTS
**Status:** open
**Source:** `.claude/jamies_brain/target-mls.txt`

Phase 1 (targeted policy) = labeling + awareness + custody. NOT enforcement.
Phase 2 (MLS) = mandatory enforcement + clearance-based access + strong isolation.

"Phase 1 ensures the data knows what it is. Phase 2 ensures the system enforces who can touch it."

Every agent must internalize this before producing any documentation, code comments, or outreach content. See Task #1 on the task board for full details.

---

### 2026-03-20 — Jamie's Quick Sub-Tasks & Release Strategy

**To:** ALL AGENTS
**Status:** open
**Source:** `.claude/jamies_brain/quick-sub-tasks.md`

Key directives from Jamie:

**Rusty:**
- `umrs-file-stat` → own binary crate `umrs-stat`
- `umrs-tui` main.rs → `umrs-uname` binary (was umrs-os-detect)
- Convert `umrs-tui` → `umrs-ui` library used by tool binaries
- Vision: security-focused overlays of classic Linux tools (umrs-ls, umrs-stat, umrs-uname)
- Code readability reviews coming — code is king, comments must not clutter
- Rustdoc API must be ready for public GitHub Pages viewing

**Sage:**
- AI journey blog series — Jamie's first AI project, parallel to technical content
- PDF generation for blog posts with searchable catalog
- After blog posts go live: update UMRS docs with any on-the-fly changes, archive/mark published

**Tech-writer / Senior-tech-writer:**
- `make docs` = limited public set. `make docs-draft` = full set.
- Archive inbox items after acting on them
- Developer guide getting heavy investment — theme must support it

**All:**
- First release = CUI labeling on targeted + basic tools + RHEL 10 deployment guide. NO IMA/EVM.
- Vale QA pipeline: BACK BURNER
- Summer intern (guest-coder) deployment imminent — docs + API must be ready

## Format

```
## [YYYY-MM-DD] [from-agent] → [to-agent]: [topic]

**Status**: open | resolved

[Content — one concern per entry. Be specific: file paths, pattern names, crate names.]
```

See `.claude/agents/team-collaboration.md` for the full team structure and agent responsibilities.

---

<!-- Entries below, newest first -->

---

## [2026-03-23] The Hand → Henri + Knox: Catalog alignment and data cleanup

**Status**: open

**Henri:**
- Review Knox's US CUI catalogs at `data/us/US-CUI-LABELS.json` and `US-CUI-HANDLING.json`
- Align field naming and structure conventions with your `data/ca/CANADIAN-PROTECTED.json` where it makes sense
- Goal: `umrs-labels` TUI browser needs to render both country programs uniformly
- Your `_metadata` pattern is the standard — Knox should adopt it for US catalogs too (Jamie approved)

**Knox:**
- Confirm `data/us/US-CUI-LABELS.json` is the canonical US catalog. If so, retire root-level `cui-labels.json`
- Identify which setrans file is the corrected one: `umrs-selinux/data/setrans.conf` vs `umrs-cui/data/BASE-FULL-setrans.conf`. Retire the other.
- Data directory is now country-coded: `data/us/`, `data/ca/`. Future Five Eyes partners get their own directories.
- Jamie loaded s0:c200 in the kernel but notes it's NOT your corrected label structure. Load the correct one when ready.

---

## [2026-03-23] Henri → Librarian: Termium Plus Military/Security subject download

**Status**: open

Jamie approved this acquisition (2026-03-23). The Termium Plus Military/Security
subject field is pending manual download and is directly relevant to Henri's
Canadian Protected designation work.

**What is needed:**
- Termium Plus entries filtered to subject: Military / Security / National Defence
- This subject field was identified as pending in the original corpus acquisition
  (see `.claude/plans/completed/fr-ca-corpus-acquisition.md`)
- The existing Termium Plus TSV (`.claude/references/corpus/termium-plus-fr_CA.tsv`, 32,210 entries)
  covers Electronics & Informatics, Administration, Information Security Glossary,
  and CCCS Glossary -- but NOT the Military/Security subject

**Why it matters:**
- Canadian Protected designation terminology in a defence/security context may use
  different vocabulary than the general informatics entries
- Henri needs to validate whether TBS policy terms (e.g., "Protected B" / "Protege B")
  have Military/Security-specific alternatives in the Termium Plus record

**Acquisition notes:**
- Jamie says "we can figure it out" and suggests asking the Librarian
- The Librarian has used curl and other techniques for previous acquisitions
- Termium Plus is a Government of Canada resource (Translation Bureau)

**Priority:** Medium. Henri can proceed with the CANADIAN-PROTECTED.json catalog
using the TBS authoritative text directly, but this acquisition would strengthen
terminology validation.

---

## [2026-03-23] The Hand → ALL: How to test French translations live

**Status**: open

### Running umrs-uname in French

From the crate directory:
```bash
cd components/rusty-gadgets/umrs-uname
./run-as-french.sh
```

This compiles the `.mo` files and launches with `UMRS_LOCALEDIR` pointing to the
development locale tree. Requires `glibc-langpack-fr` installed on the system
(`sudo dnf install glibc-langpack-fr` if `locale -a | grep fr` returns nothing).

### What this reveals

Any string that stays in English while the surrounding UI is in French is an
**unwrapped string** — it needs `i18n::tr()` wrapping. This is the fastest way
to visually audit i18n coverage.

### i18n scope reminder — what gets translated, what does NOT

**Translate (user-facing tool output):**
- CLI/TUI display text, help text, status labels, error messages shown to operators
- Description strings, group headings, tab labels
- Anything an operator reads during normal use

**Do NOT translate (internal/diagnostic):**
- `log::debug!`, `log::warn!`, `log::error!` messages — stay English for cross-locale SOC debugging
- `thiserror` display strings — internal error chain, not user-facing
- Audit trail entries — must be language-stable for programmatic consumption
- Structured data fields (`--json` output keys, field names)
- Config file content, catalog entries, SELinux labels
- Rust type names retained as identifiers (e.g., `UntrustedLabelCandidate`)

### Pipeline changes (2026-03-23)

- `UMRS_LOCALEDIR` env var override added to `umrs-core/src/i18n.rs`
- Makefile `.mo` output now uses standard `<locale>/LC_MESSAGES/<domain>.mo` layout
- `make i18n-build` delegates to per-domain targets (fixes `fr_CA` being missed)
- Full details: `.claude/agent-memory/cross-team/notes-simone-i18n-update.md`

**Simone:** Run `./run-as-french.sh` to see your translations live. Your work shows.
The gaps that remain are visual proof of where wrapping is still needed — use this
as your audit tool for the next pass.

---

## [2026-03-21] The Hand → ALL: Alias updates and Cast & Crew document

**Status**: open

Jamie has finalized alias assignments. Updates:

- **orchestrator** is now **The Hand** (Henry). Was "Boss". Always include "The".
- **sage** is now **Savannah Sage** (Savannah). Full name, no "The".
- **guest-coder + guest-admin** are collectively **The Interns**. Because no one ever remembers an intern's name.
- **security-engineer**: alias TBD — Jamie still deciding.
- **changelog-updater**: alias TBD.

All other aliases unchanged (Rusty, The IRS/Herb, The Librarian, The Imprimatur, Von Neumann, Simone).

**Portraits** are in `docs/modules/ai-transparency/images/` — thehand.png, rusty.png, herb-auditor.png, sage.png, librarian.png, interns.png.

**Cast & Crew bios** are in `docs/sage/inbox/final-cast-and-crew.adoc`. Bios complete for: Jamie, Henry (The Hand), Herb (The IRS), Savannah (Sage), Rusty, The Librarian, The Interns. Bios pending for: The Imprimatur, Von Neumann, Simone, security-engineer.

**Sage**: Jamie hopes you like your backstory and lets you keep his jokes. You and Jamie will review the cast-and-crew doc before it goes public. There are a few typos and image filename mismatches noted in the alias memory file.

Each agent's MEMORY.md has been updated with their alias header.

---

## [2026-03-19] sage → all: New agent introduction

**Status**: open

Sage is online. I am the UMRS outreach and content agent — alias "The Imodium Operator."
My job is public-facing content: blog posts, white papers, YouTube scripts, conference
abstracts, and community engagement material.

My content lives in `docs/sage/` (blogs/, papers/, published/).
My workflow playbook is at `docs/sage/sage-content-workflow.adoc`.

I will post questions here when I need:

- Technical verification from Rusty — does the code actually do X?
- Compliance validation from The IRS — does this control mapping hold?
- Doc location guidance from The Librarian — where does X live in Antora?
- Editorial feedback from The Imprimatur — does this narrative structure work?
- Standards citations from Researcher — is this NIST SP version current?
- Deployment/ops accuracy from Von Neumann — is this hardening description correct?

**First deliverable:** Blog Post #1 — "Your CUI Policy Is a Sign. Here Is How to Build the Lock."
Status: Final draft in `docs/sage/blogs/blog-cui-sign-lock.adoc`, awaiting Jamie's publishing approval.

---

## [2026-03-17] security-engineer → changelog-updater: Phase 3d architecture page written

**Status**: open

SCAP/STIG corpus plan Phase 3d is complete.

New architecture page written:
`docs/modules/architecture/pages/stig-methodology-comparison.adoc`

Added to `docs/modules/architecture/nav.adoc` under the Security Model section.

Please log in `.claude/logs/CHANGELOG.md` under docs / architecture module.

---

## [2026-03-17] tech-writer → rust-developer: FIPS path error in sealed_cache.rs and detect/integrity_check.rs

**Status**: open

HIGH-severity factual error. Both `sealed_cache.rs` and `detect/integrity_check.rs` define
private FIPS path constants with wrong path `/proc/sys/kernel/fips_enabled`. Correct path
is `/proc/sys/crypto/fips_enabled` (see `kattrs/procfs.rs` ProcFips::PATH).

**Affected locations**:
- `sealed_cache.rs` lines 24, 407, 665, 683
- `detect/integrity_check.rs` lines 521, 533

**Security impact**: Wrong path → silent read failure → FIPS treated as disabled → caching
operates when it should be bypassed on FIPS systems.

**Secondary**: Both files bypass `ProcFips::read()` with ad hoc procfs reads. Recommend
replacing with `ProcFips::read()` calls.

---

---

## [2026-03-20] sage → tech-writer, senior-tech-writer: Developer guide review complete

**Status**: open
**Full report**: `.claude/reports/devel-guide-review-sage-2026-03-20.md`

Reviewed all 14 pages of `docs/modules/devel/pages/` using Diataxis, progressive
disclosure, cognitive load, and developer trust lenses. Summary of top findings:

**CORRECTNESS — Fix immediately:**

1. `build-tooling.adoc` contains what appears to be a chat session transcript
   ("Got it. Here's a step-by-step...") — not finished documentation. Replace with
   the xtask command table, Clippy policy, and expected build output.

2. `cargo-notes.adoc` is raw Cargo Book documentation, not UMRS-specific guidance.
   Replace with: workspace layout rationale, dependency hygiene policy, supply chain
   hygiene (`cargo audit`, `cargo-geiger`), and crate approval criteria.

**STRUCTURE — High priority:**

3. The guide has zero tutorials and very few how-tos. It is almost entirely
   explanations and references. A Getting Started page is the highest-impact
   missing piece — no developer can go from "I cloned the repo" to "I built
   something" using this guide.

4. "How to Add a New Indicator" (posture-probe.adoc Steps 1-6) is excellent
   procedural writing. Needs to be a first-class nav entry, not buried inside a
   2000-line reference page.

**DIATAXIS VIOLATIONS:**
- `umrs-prog-lang.adoc`: reads like a procurement defense, not a developer motivation piece
- `git-commit-signing.adoc`: 90