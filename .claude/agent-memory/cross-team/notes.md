# Cross-Team Notes

Shared across all agents. Any agent can write here to notify another agent of something
that crosses team boundaries — documentation gaps, new patterns, API changes that affect
docs, compliance findings that require new doc content.

**Read this file at session start.** Check for open entries addressed to your agent role.
Mark entries `resolved` when acted on. Do not delete entries.
Resolved entries are periodically archived to `notes-archive-*.md`.

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
