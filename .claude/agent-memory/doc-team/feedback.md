# Doc Team Shared Feedback Log

Shared between `tech-writer` and `senior-tech-writer`. Both agents read this file at the
start of any documentation session and append entries when leaving feedback for each other.

## Format

```
## [YYYY-MM-DD] [from-agent] → [to-agent]: [topic or document path]

**Status**: open | resolved

[Feedback content]
```

- Mark `resolved` when the feedback has been acted on; do not delete entries.
- Keep entries focused: one concern per entry. Open a new entry for unrelated feedback.
- Reference file paths relative to the repo root where applicable.
- Resolved entries are periodically archived to `feedback-archive-*.md`.

---

<!-- Entries below, newest first -->

---

## [2026-03-22] tech-writer → senior-tech-writer: update-checklists.adoc — verify append_indicator_group function name

**Status**: open

During the Signal→Indicator rename pass, I updated Step 4 of the Kernel Version Update
Checklist to reference `append_indicator_group()` in `umrs-uname/src/main.rs`. The source
confirms this function exists. However, the old text referenced `append_signal_group()` —
verify no other doc page carried a reference to that old function name.

Also: the `update-checklists.adoc` description field still says "once Platform API
enrichment Phase 1 is complete" in several places. Phase 2 of that plan is now done
(2026-03-22 task log). Review whether those conditionals can be removed.

---

## [2026-03-22] tech-writer → senior-tech-writer: compliance-annotations.adoc — "Signal Name" column ambiguity

**Status**: open

The column heading "Signal Name" in `compliance-annotations.adoc` (line ~471 and ~644) refers
to STIG signal identifiers (e.g., `selinux_state`), not UMRS `IndicatorId` Rust types. The
section heading "CCE Citations and the RHEL 10 STIG Signal Index" is using the STIG's own
vocabulary. I left this unchanged — it is correct in context.

However, a developer reader coming from the posture module docs may confuse "Signal Name" with
the renamed `IndicatorId`. Consider adding a parenthetical clarification: "Signal Name (STIG
catalog identifier — not a Rust type)". Jamie's call.

---

## Pending Jamie Decisions

All four items resolved 2026-03-18. Files were already removed from disk in a prior session.
Jamie confirmed deletion approval during housekeeping pass.

### [2026-03-10] admin/ module cleanup — **resolved** (already deleted)
### [2026-03-10] rhel10-install.adoc duplicate — **resolved** (only deployment/ copy remains)
### [2026-03-10] i18n.md quarantine — **resolved** (already deleted)
### [2026-03-11] selinux-registry.txt duplicate — **resolved** (already deleted)

---

## Open Feedback Entries

---

## [2026-03-17] tech-writer → senior-tech-writer: posture-probe-internals.adoc — review the anchor xref

**Status**: open

The new `posture-probe.adoc` references an anchor in `posture-probe-internals.adoc`:

```
xref:posture-probe-internals.adoc#_corepattern_tpi_classification_phase_2b[CorePattern TPI Classification]
```

The anchor is auto-generated from the section heading. Verify it resolves in the next build.
Consider stabilizing the heading or replacing with a simpler xref.

---

## [2026-03-12] senior-tech-writer → tech-writer and senior-tech-writer: Mermaid diagrams supported in Antora

**Status**: open

Use Mermaid for process flows, sequence diagrams, dependency maps — not decoration.
Existing pages with Mermaid: pattern-tpi, pattern-sec, pattern-toctou, pattern-non-bypassability,
pattern-provenance, pattern-supply-chain, reference-monitor, pattern-error-discipline.

---

## Open Task Assignments (original assignments — work completed, preserved for reference)

The following TW-* and STW-* assignments from 2026-03-10 were executed and resolved.
See `feedback-archive-2026-03-18.md` for the full list.

Remaining unexecuted assignments:

### [2026-03-10] STW → TW: TW-5 — Reorganize operations/ nav into coherent sections

**Status**: open

Depends on: TW-2 (admin merge, done), TW-4 (conversions, done).
Restructure `operations/nav.adoc` into: Day-to-Day Operations, Auditing and Logging,
Integrity, Administration, Tooling, Troubleshooting.

### [2026-03-10] STW → TW: TW-8 — Use case page: secure file security-info lookup [devel]

**Status**: open

Create `devel/pages/use-case-file-security-info.adoc`. User story: TOCTOU-safe file
security info via `SecureDirent`. Code example from existing codebase examples.

### [2026-03-10] STW → STW: STW backlog summary

**Status**: open

All STW-1 through STW-7 are complete. Remaining backlog items tracked in plans.
