# tui-cli/cli-ux Collection Index

## Purpose

This collection provides CLI and TUI design guidelines for the UMRS project's terminal
interface work. These documents govern CLI output verbosity, error messages, flag naming,
state feedback, subcommand structure, composability, color handling, and TUI patterns.

## Documents

### clig-guidelines.md
- **Source**: https://raw.githubusercontent.com/cli-guidelines/cli-guidelines/main/content/_index.md
- **Web version**: https://clig.dev/
- **Fetched**: 2026-03-15
- **Description**: Command Line Interface Guidelines — an open-source guide to writing better
  command-line programs, taking traditional UNIX principles and updating them for the modern day.
  Covers philosophy, basics, help, documentation, output, errors, arguments/flags, interactivity,
  subcommands, robustness, future-proofing, signals, configuration, environment variables, naming,
  distribution, and analytics. Primary reference for UMRS CLI/TUI design decisions.
- **UMRS relevance**: Governs umrs-ls, umrs-tui, and any future UMRS CLI tools. Key rules:
  - Disable color when NO_COLOR is set (aligns with operator environment requirements)
  - --json flag for all structured data output (UMRS TUI design principle)
  - Send output to stdout, messaging to stderr
  - State changes must be communicated explicitly (aligns with UMRS TUI design constraint)
  - Do not read secrets from flags or environment variables

### no-color.md
- **Source**: https://raw.githubusercontent.com/jcs/no_color/master/index.md
- **Web version**: https://no-color.org/
- **Fetched**: 2026-03-15
- **Description**: The NO_COLOR informal standard (proposed 2017). Defines that command-line
  software which adds ANSI color to its output by default should check for the NO_COLOR
  environment variable and, when present and not empty, suppress color output. Includes
  example C implementation, FAQ, and note on companion FORCE_COLOR standard.
- **UMRS relevance**: UMRS explicitly requires honoring NO_COLOR unconditionally (TUI/CLI
  design principle, CLAUDE.md). This document is the authoritative reference for that requirement.

### awesome-tuis.md
- **Source**: https://raw.githubusercontent.com/rothgar/awesome-tuis/master/README.md
- **Repository**: https://github.com/rothgar/awesome-tuis
- **Fetched**: 2026-03-15
- **Description**: Community-maintained list of TUI applications organized by category:
  Dashboards, Development, Docker/K8s, Editors, File Managers, Games, Libraries, Messaging,
  Miscellaneous, Multimedia, Productivity, Screensavers, Web.
- **UMRS relevance**: Reference list for TUI design patterns and Rust TUI libraries. Highlights:
  - **Rust TUI libraries**: Ratatui (primary choice), tui-rs (deprecated), tui-input, iocraft, Zaz
  - **Security/monitoring tools** (Rust): bandwhich, binsider, flawz (CVE browser), gpg-tui,
    kmon (Linux Kernel Manager), oryx (eBPF network sniffer), trippy, zenith, netscanner,
    rustnet, WireGuard Monitor, vortix
  - **System monitoring** (Rust): bottom, btop++ (C++), macmon, ttop, s-tui
  - **Ops/infra tools**: k9s, lazydocker, ctop, lazyjournal (journalctl TUI)
  - **Dev tools** (Rust): gitui, rainfrog, logradar, ATAC, cargo-seek
  - **Notable patterns**: k9s-style navigation used as UX reference by multiple tools

## Update Notes

- clig-guidelines.md: Check https://github.com/cli-guidelines/cli-guidelines/commits/main for updates
- no-color.md: Check https://github.com/jcs/no_color/commits/master for updates
- awesome-tuis.md: Active community list — check for new Rust security/monitoring entries quarterly
