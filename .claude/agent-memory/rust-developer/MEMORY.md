# Rust Developer Agent Memory

## Permanent Crate Dependency Rules (ARCHITECTURAL CONSTRAINT — NEVER VIOLATE)

| Crate          | Allowed workspace dependencies        |
|----------------|---------------------------------------|
| `umrs-platform`| None — no deps on selinux or core     |
| `umrs-selinux` | `umrs-platform` only                  |
| `umrs-core`    | `umrs-platform` and `umrs-selinux`    |

- Directions are **fixed**. Never reverse or add to them.
- Datatypes from `umrs_selinux::` must NOT be used in `umrs_platform`.
- Datatypes from `umrs_platform::` may be used by `umrs_selinux` and `umrs_core`.
- `umrs-platform` must never use `console::*` items for displaying.
- Before adding any `path = "../..."` dep to a `Cargo.toml`, verify it does not violate the table.
- If a proposed design requires a direction not listed here, STOP and raise it with Jamie.

## How Jamie Signals Permanent Rules

When Jamie says **"Add the following permanent constraint to CLAUDE.md: [rule]"**, that rule
must be written into `CLAUDE.md` as a standing, permanent rule — not just guidance for the
current session. The word "permanent" is the signal to treat it this way.

## Session Start Checklist
- Check `.claude/reports/` for outstanding security-auditor and security-engineer findings tagged "coder"
- Run `cargo xtask clippy && cargo xtask test` to verify workspace state
- Review git status for unverified changes from prior sessions

## Role Boundaries
- NEVER edit files under `docs/` — only note what needs updating for the tech-writer
- Dependency approval: security-engineer agent can approve new crates (must notify Jamie, create report)
- When in doubt on pattern measurement granularity, consult security-engineer agent

## High-Assurance Pattern Decisions
- **TPI**: Does NOT apply to kernel attributes (booleans, dual-booleans). ASK Jamie if a kernel attribute has complex parsed structure beyond bool/dual-bool.
- **Constant-time comparison**: Use judgment. Not every label comparison needs `subtle::ConstantTimeEq` — apply where timing side-channels are a realistic threat.
- **Zeroize**: Reserved for key material, passwords, and cleartext secrets only. `SecurityContext` and MLS labels do NOT require zeroize.
- **Pattern timing**: Use judgment on granularity. Consult security-engineer when unsure.
- **Mandatory patterns** (e.g., ProcfsText for /proc/ reads, TOCTOU safety): Apply without pausing. Report what was applied after the fact.

## Clippy and Code Style
- AVOID `#[allow(...)]` whenever possible. ASK Jamie before adding any allow attribute.
- `expect()` policy: Deferred — Jamie will provide secure coding guides as reference material. Until then, avoid `expect()` in new code.

## Cargo.lock
- Not a protected file. Let it change naturally when Cargo.toml is modified.

## Examples
- Every new public module gets an example in the standard Cargo `examples/` directory.
- guest-coder agent uses these examples as its starting point — make them clear and complete.

## xtask Clippy and Cargo.toml Lints Interaction (IMPORTANT)
- `cargo xtask clippy` passes `-D warnings` on the command line to rustc/clippy.
- Command-line `-D warnings` OVERRIDES `[lints.clippy]` allow entries in `Cargo.toml`.
- Consequence: any lint suppressed only in `Cargo.toml` `[lints]` will still fire under xtask.
- Fix: add `#![allow(clippy::lint_name)]` at the crate root in the source file, AND document
  the rationale. The `Cargo.toml` entry can stay for non-xtask builds.
- Always ask Jamie before adding new `#[allow]` attributes.

## gettextrs / gettext-rs (as of 2026-03-10)
- The crate is named `gettext-rs` on crates.io (latest stable: 0.7.7).
- Library name when imported in Rust: `gettextrs` (use `use gettextrs::gettext;`).
- It provides a plain `gettext(str) -> String` function — NOT a `gettext!()` macro.
- For translated strings with substitution: use `format!()` with the translated template.
  Pattern: `format!("{} ({n}) ", gettext("label"))` — keep the static string separate.
- The `gettext-system` feature uses the OS-provided libintl (preferred on RHEL10).
- Has FFI dep (`gettext-sys`) — supply chain review required before adding.

## Known Pre-existing Issues Fixed (2026-03-10)
- `umrs-selinux/src/observations.rs:160` — `missing_const_for_fn` on `SecurityObservation::kind()`.
  Fixed: added `const` to the fn signature.
- `umrs-ls/src/main.rs:66` — `BOX_SLIM_CONN` declaration missing semicolon (syntax error).
  Fixed: added semicolon and extra space cleanup.
- `umrs-ls/src/main.rs` — `format_push_string` and dead-code (`BOX_BOLD`, `BOX_SLIM_CONN`)
  lints were suppressed in Cargo.toml but fired under xtask -D warnings.
  Fixed: added crate-level `#![allow(clippy::format_push_string)]` and item-level
  `#[allow(dead_code)]` on the WIP box-drawing constants.
