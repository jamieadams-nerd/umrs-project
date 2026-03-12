# Rust Developer Agent Memory

## Project Structure
- Workspace root: `components/rusty-gadgets/`
- Primary crate: `umrs-selinux/` (SELinux MLS reference monitor)
- All commands via `cargo xtask fmt/clippy/test`

## Key Architectural Rules (quick ref)
- `#![forbid(unsafe_code)]` in every crate root — compile-time proof
- No `unwrap()` — `#![deny(clippy::unwrap_used)]`
- No inline tests — all in `tests/`
- Dependency direction: `umrs-platform` → `umrs-selinux` → `umrs-core`; tool binaries can depend on any of these
- Protected files: `**/*.json`, `**/setrans.conf`, `**/.gitignore`

## Crate Inventory
| Crate | Type | Purpose |
|---|---|---|
| `umrs-selinux` | lib | SELinux MLS reference monitor, primary crate |
| `umrs-platform` | lib | Low-level OS/kernel layer, no workspace deps |
| `umrs-core` | lib | Shared formatting, i18n, timing, robot art |
| `umrs-ls` | bin | Security-enriched directory listings |
| `umrs-logspace` | lib/bin | Audit trail and logging |
| `umrs-state` | bin | System state introspection |
| `umrs-tui` | bin | Terminal UI — wizard art viewer (added 2026-03-11) |
| `umrs-apparmor` | lib | AppArmor support |
| `umrs-crypto` | lib | Crypto primitives |

## umrs-core::robots Module
- `AsciiArtStatic` in `src/robots/data.rs`: `{name, width, height, lines: &'static [&'static str]}`
- Builtins in `src/robots/builtins.rs`: `ALL_ROBOTS` array, individual `pub static` entries
- Width = char count per line (braille chars are 1 char wide each)
- Added 2026-03-11: `WIZARD_MEDIUM` (29×13) and `WIZARD_SMALL` (15×7) from txt files in crate root

## umrs-tui Binary
- Created 2026-03-11 at `umrs-tui/src/main.rs`
- No external deps beyond `umrs-core`
- Args: `--justify/-j left|right` via `std::env::args()`
- Terminal width: `COLUMNS` env var → fallback 80
- Renders `WIZARD_MEDIUM` then blank line then `WIZARD_SMALL`
- Right-justify: `" ".repeat(term_width.saturating_sub(art_width))` prefix per line

## Patterns Applied
- `saturating_add`/`saturating_sub` on all index/width arithmetic (secure arithmetic rule)
- `#![forbid(unsafe_code)]` in new crate root (mandatory)
- No `unwrap()` anywhere — `.unwrap_or()` / `match` throughout

## Outstanding Audit Findings (from .claude/reports/)
- `2026-03-11-os-detection-umrs-platform.md` — multiple coder-assigned findings in `umrs-platform` (integrity_check.rs, pkg_substrate)
- `sec-audit-2026-03-11.md` — SEC-03, SEC-05, SEC-06 and others assigned to coder
- These are NOT blocking the wizard/tui work but should be addressed in a dedicated session

## Session Start Checklist
1. Check `.claude/reports/` for coder-assigned findings
2. `cargo xtask clippy && cargo xtask test` must be clean before proceeding
3. Check git status for incomplete prior-session work
4. Check `TaskList` for pending tasks
