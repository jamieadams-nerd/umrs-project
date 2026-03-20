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
**Status:** open

Rusty completed kernel tab enhancement Phases 1-4 in `umrs-tui/src/main.rs`.

Changes affecting documentation:
- Value translations: integer indicators now show parenthetical meanings (e.g., `2 (full ASLR)`)
- Group descriptions: each of the 6 indicator groups shows a 1-2 sentence explanation below its title
- Trust / Evidence tab: "No downgrade — full trust retained" replaces bare "none"; contradictions get an explanation row
- New key bindings: `?` and `F1` open a contextual help overlay for the current tab

Operations reference guide (docs/modules/operations/ or docs/modules/umrs-tools/) needs updating:
- Key bindings table: add `?` / `F1` → ShowHelp
- Trust Evidence tab section: update downgrade reasons description to reflect positive framing
- Kernel Security tab section: note integer values now include plain-language annotations
- Source of truth for all indicator descriptions: `.claude/agent-memory/security-auditor/indicator-definitions-plain-language.md`

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

See `.claude/team-collaboration.md` for the full team structure and agent responsibilities.

---

<!-- Entries below, newest first -->

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

Please log in `.claude/CHANGELOG.md` under docs / architecture module.

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