# Guest Admin Persistent Memory

## Recurring Documentation Patterns (as of 2026-03-20)

- [Operator doc gaps](pattern_operator_doc_gaps.md) — UMRS tools lack operator-facing pages in docs/modules/umrs-tools/. Developer guides exist; operator quickstarts do not.
- [Internal type names in display](pattern_internal_names.md) — Rust enum variant names (LabelClaim, UntrustedLabelCandidate, BootDrift) appear in operator-visible display strings. Needs mapping to plain-English labels.
- [Missing --help and --json](pattern_missing_cli_flags.md) — TUI binaries have no --help, --version, or --json flags. Project design principle requires --json for structured data tools.
- [Hard gate error messages](pattern_error_messages.md) — Detection pipeline failures use internal terminology ("hard gate") without explaining what the operator should do next.

## Reports Written

- 2026-03-20: `.claude/admin-reports/2026-03-20-tui-os-detect.md` — TUI OS Detection Audit Card review. 18 findings (4H/8M/6L). Overall usability: 6.5/10. No live binary available.
