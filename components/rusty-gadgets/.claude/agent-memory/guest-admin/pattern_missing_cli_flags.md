---
name: Missing --help, --version, --json flags on TUI binaries
description: umrs-tui binaries have no argument parsing — no --help, --version, or --json output
type: project
---

Observed in umrs-os-detect-tui (2026-03-20): main() has no argument parsing. Running
the binary with --help produces no output. The UMRS project design principle
(CLAUDE.md) explicitly requires --json for all commands returning structured data.

**Why:** TUI binaries were built without argument parsing infrastructure.

**How to apply:** When reviewing any UMRS binary, check for --help, --version, and
--json. Absence of --json is a direct violation of the project's own stated design
principle (CLAUDE.md TUI/CLI Design Principles section) and should be flagged MEDIUM.
Absence of --help is a standard operator expectation, flag MEDIUM.
