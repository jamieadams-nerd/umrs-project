---
name: Operator documentation gaps
description: UMRS tool crates have developer guides but no operator-facing pages in docs/modules/umrs-tools/
type: project
---

As of 2026-03-20, umrs-tui has a how-to.md aimed at developers adding new binaries,
and an audit-card.md that is a design request document. Neither serves an operator.
No page exists in docs/modules/umrs-tools/ for the umrs-os-detect-tui binary.

**Why:** Tool documentation was not written in parallel with the tool implementation.

**How to apply:** When reviewing any UMRS tool binary, check for a corresponding
operator page in docs/modules/umrs-tools/pages/. If absent, flag HIGH. Also check
that the page covers: what the tool does, how to launch it, minimum privilege,
tab/output interpretation, and what to do on a red finding.
