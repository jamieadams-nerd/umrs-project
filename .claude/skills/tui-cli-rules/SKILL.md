---
name: tui-cli-rules
description: >
  TUI and CLI design principles and I/O discipline for UMRS tools: NO_COLOR
  compliance, --json output, --verbose mode, error message standards, and the
  mandatory platform abstraction requirement for all system state reads in
  binary crates. Use this skill when developing or reviewing TUI or CLI binary
  crates (umrs-ls, umrs-stat, umrs-label, umrs-uname, umrs-c2pa). Trigger
  when the user or agent mentions TUI, CLI, ratatui, crossterm, NO_COLOR,
  --json, --verbose, terminal output, popup, or binary crate I/O.
---

## TUI/CLI Design Principles

UMRS targets security operators in high-stakes environments. The interface must communicate trust.

- Honor `NO_COLOR` environment variable unconditionally.
- State changes must be explicitly communicated to the user.
- Provide `--json` output mode for all commands that return structured data.
- Default output must be operator-readable without log-level labels or debug noise.
- Verbose mode (`--verbose` / `-v`) is for developer-facing output.
- Error messages must describe what happened and what the operator should do next.

## TUI/CLI I/O Discipline

[CONSTRAINT] TUI and CLI tools are NOT exempt from secure read obligations. "Display-only"
is not a waiver for bypassing `umrs-platform` or `umrs-selinux` abstractions.

Every binary crate that reads system state — for any reason, including populating a TUI
header, rendering a status bar, or driving autocomplete — MUST use the platform abstraction
layer. The full System State Read Prohibition Rule in `rust-design-rules` skill applies to all
binary crates without exception.

**Rationale:** A TUI tool that reads `/etc/os-release` with a hand-rolled parser and
displays the result has already performed an unverified, non-provenance-checked read of a
system file. The fact that the output goes to a screen instead of a policy engine does not
undo the read. If the file was tampered with, the tool displays tampered data to a security
operator — that IS a security-relevant outcome.

**Dependency requirement:** Every binary crate that displays system state MUST depend on
`umrs-platform`. If a TUI tool's `Cargo.toml` does not list `umrs-platform` in
`[dependencies]`, that is a review finding.

**Hardcoded path prohibition:** TUI/CLI tools MUST NOT hardcode policy-variant paths
(e.g., `/etc/selinux/targeted/secolor.conf`). Query the active policy name from the kernel
and construct paths dynamically. Today's targeted policy is tomorrow's MLS policy.
