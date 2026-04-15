---
name: CLI clap migration and verbose pattern (2026-04-15)
description: umrs-ls migrated to clap; all 5 tools now have --verbose/-v; /opt/umrs defaults wired
type: project
---

## Coverage

All 5 staged binaries now pass `--help`, `--version`, `--verbose`/`-v`.

## umrs-ls

- Migrated from hand-rolled `std::env::args()` to clap derive `struct Args`.
- `run_json(args, target)` → `run_json(target)` — no longer takes `&[String]`
- `run_cli(args, target, flat)` → `run_cli(target, flat, no_iov, no_mtime, with_size, with_inode, color)`
  Needs `#[expect(clippy::fn_params_excessive_bools)]` — 5 bool CLI flags.
- `struct Args` needs `#[expect(clippy::struct_excessive_bools)]` — 6 bool fields.
- `load_catalogs()` now honors: `UMRS_CONFIG_DIR` → `/opt/umrs/etc/umrs/` → CWD → `../umrs-label/`
- Tests: `tests/cli_args_tests.rs` (4 tests: help, version, unknown_flag, verbose_flag_accepted)

## umrs-uname

- Added clap derive `struct Args` with `--json`, `--cli`, `--verbose`/`-v`.
- `--json` and `--cli` exit 0 with "not yet implemented" on stderr.
- `Args::parse()` called BEFORE logging init so `--help`/`--version` exit cleanly.
- Tests: `tests/cli_args_tests.rs` (5 tests: help, version, unknown_flag, json_exits_cleanly, cli_exits_cleanly)

## umrs-label

- Added `--verbose`/`-v` to existing clap struct.
- Catalog defaults changed: `config/us/…` → `/opt/umrs/etc/umrs/us/…`
- Path resolution: explicit flag → `UMRS_CONFIG_DIR` env → `/opt/umrs` → CWD.
- Added `i18n::init("umrs-label")` and `use umrs_core::i18n` — was missing entirely.
- Wrapped 11 TUI help overlay strings with `i18n::tr()`.

## umrs-stat

- Added `--verbose`/`-v` to existing clap struct.

## umrs-c2pa

- `--config` default changed from `umrs-c2pa.toml` to `/opt/umrs/etc/umrs/umrs-c2pa.toml`.
- Path resolution: explicit `--config` (if not equal to install default) → `UMRS_CONFIG_DIR` → `/opt/umrs` → CWD.
- Comparison against default uses `std::path::Path::new(...)` not `PathBuf::from(...)` (avoids `cmp_owned` lint).

## Verbose macro pattern

All tools use a function-local macro — do NOT use `concat!` inside it (breaks format_args implicit capture).
Correct pattern:
```rust
macro_rules! verbose {
    ($($arg:tt)*) => {
        if verbose {
            eprintln!("  [tool-name] {}", format_args!($($arg)*));
        }
    };
}
```

## Doc sync needed

Man pages and Antora OPTIONS sections for all 5 tools need updating.
Tech-writer owns EN man + Antora. Simone owns fr_CA man.
