---
name: Hard gate error messages lack operator action guidance
description: Detection pipeline failure messages use internal terminology without telling the operator what to do
type: project
---

Observed in umrs-os-detect-tui from_error() (2026-03-20): errors displayed as:
  "Hard gate: procfs is not real procfs"
  "Hard gate: PID coherence broken"
  "Hard gate: I/O error during kernel anchor"

"Hard gate" is internal UMRS design terminology. None of these messages say whether
the condition is a security event, a container/VM artifact, a permission problem, or
a system failure — and none say what the operator should do next.

**Why:** Error strings were written by developers who know the design context.

**How to apply:** When reviewing any error state display, check whether the message
(1) uses internal terminology, and (2) tells the operator what to do. If both are
absent, flag HIGH. If only one is absent, flag MEDIUM.
