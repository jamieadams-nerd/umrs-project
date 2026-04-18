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

## Popup and Panel Right-Margin Discipline

Text must never sit flush against a popup, panel, or dialog border. Every row needs a
visible one-cell gap between its content (value, ellipsis, hash tail, table column) and
the right border character. This has been a recurring bug — it is now a rule.

[RULE] Every bordered renderer MUST reserve the right-margin gap at the rendering
boundary, not per row-variant.

- Bordered renderers in `libs/umrs-ui/` (popup, data panel, pinned pane, dialog)
  MUST compute their content rect by shrinking the block's inner rect by one cell on
  the right. The canonical helper is `popup::reserve_right_margin(rect)`.
- Render the `Block` and the `Paragraph` separately. Do NOT attach the block to the
  paragraph via `.block(block)` when you need the margin — ratatui will use
  `block.inner()` verbatim and you lose control of the content rect.
- Row-variant renderers (`data_row_to_line`, `build_key_value_line`, and friends) MUST
  NOT compute per-arm margin. The reservation is upstream; each arm just fills its
  budget.

[ANTI-PATTERN] Computing `val_width = scroll_area.width - col_width - 3` (or any
per-arm subtraction) and trusting each match arm to get the math right. Off-by-one
bugs keep returning this way — six match arms, six chances to miscount.

### Long content (hash digests, paths)

- Content wider than the narrowest supported popup (e.g., SHA-384 at 96 hex chars on a
  100-col terminal) MUST be split across multiple lines at build time — before it ever
  reaches the renderer. Do not rely on the renderer to truncate cryptographic values.
- Paths that may exceed the value column MUST use the shared left/right ellipsis
  truncation (`libs/umrs-ui/src/text_fit.rs`). The ellipsis glyph is `…` (U+2026);
  never three ASCII dots.

### Regression protection

Every bordered renderer that accepts arbitrary content SHOULD have a TestBackend-based
test that renders overwide content at several widths and asserts the rightmost content
column of every non-border row is blank. See
`libs/umrs-ui/tests/popup_right_margin_tests.rs` as the reference pattern.
