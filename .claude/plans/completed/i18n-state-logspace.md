# Plan: i18n Onboarding — umrs-state and umrs-logspace

**Date**: 2026-03-10
**Status**: COMPLETE — both steps done
**Prerequisite**: tr_core() cleanup complete (build clean, 237 tests passing)

---

## Context

Neither `umrs-state` nor `umrs-logspace` currently calls `i18n::init()` or `tr()`.
Both have user-facing terminal strings that should be translatable.
Both need to be added to the Makefile i18n pipeline.

---

## Step 1 — umrs-state

### 1a. Rust changes (`components/rusty-gadgets/umrs-state/src/main.rs`)

Add `i18n::init("umrs-state")` before any output in `main()`.
Add `use umrs_core::i18n;` import.

Wrap the following user-facing strings with `i18n::tr()`:

| Current string | Location | Notes |
|---|---|---|
| `"Starting umrs-state"` | `console_info!` macro | Wrap the literal |
| `"FIPS is Enabled"` | `console_status!` macro | Wrap the literal |
| `"System Purpose defined in state file"` | `console_status!` macro | Wrap the literal |
| `"State file '{}' does not exist. It will be created."` | `eprintln!` | Translate template only; path is runtime |
| `"Warning: state file '{}' does not exist. Using default state."` | `eprintln!` | Translate template only |
| `"(not set)"` | `println!` (in `handle_get`) | Wrap the literal |
| `"Unknown key: {key}"` | `eprintln!` (in `handle_get` and `handle_set`) | Translate template |
| `"Invalid boolean value '{value}', using false."` | `eprintln!` (in `handle_set`) | Translate template |

Do NOT wrap:
- `log::info!` strings — audit trail, must stay English
- `.expect()` strings (none currently, but keep the rule)

Pattern for strings with embedded runtime values:
```rust
// Translate the static template, format inline
eprintln!("{}", format!("{} '{}'.", i18n::tr("State file does not exist. It will be created"), path.display()));
```
Or simplest approach: use a translated prefix + untranslated variable:
```rust
eprintln!("{}: {}", i18n::tr("State file does not exist — will be created"), path.display());
```

### 1b. Makefile changes

Add to the i18n configuration block:
```makefile
I18N_ACTIVE_DOMAINS := umrs-ls umrs-logspace umrs-state   # add umrs-state

I18N_SRC_DIR_umrs_state := components/rusty-gadgets/umrs-state/src

I18N_ACTIVE_LOCALES_umrs_state := fr_CA
```

### 1c. Scaffold + extract

```bash
make i18n-setup
make i18n-extract-umrs-state
# verify resources/i18n/umrs-state/umrs-state.pot contains all wrapped strings
msginit --input=resources/i18n/umrs-state/umrs-state.pot \
        --locale=fr_CA --output=resources/i18n/umrs-state/fr_CA.po
make i18n-merge-umrs-state
make i18n-compile-umrs-state
```

### 1d. Build verification

```bash
cd components/rusty-gadgets && cargo xtask clippy && cargo xtask test
cargo run -p umrs-state -- --help
```

**STATUS: COMPLETE**
- 8 strings extracted into `resources/i18n/umrs-state/umrs-state.pot`
- `fr_CA.po` initialized and compiled to `fr_CA.mo`
- Orphan `console-test.rs` moved to `docs/_scratch/notes/`
- Clippy clean, binary runs, `--help` shows translated strings (English fallback)

**→ STOP. Ask Jamie to approve Step 2.**

---

## Step 2 — umrs-logspace

### 2a. Rust changes (`components/rusty-gadgets/umrs-logspace/src/`)

**`main.rs`** — add `use umrs_core::i18n;` and call `i18n::init("umrs-logspace")` before
`Cli::parse()`.

Do NOT wrap `.expect("configuration error")` or `.expect("measurement error")` — these are
panic messages, not user UI.

**`output.rs`** — wrap the display labels in `print_pools`:

| Current string | Notes |
|---|---|
| `"Resource Pool: {}"` | Translate label |
| `"  Mount point : {}"` | Translate label |
| `"  Total space : {} MiB"` | Translate label |
| `"  Free space  : {} MiB"` | Translate label |
| `"  Lifecycle: {:?}"` | Translate label; `{:?}` value stays as-is |

`output.rs` calls `println!` directly — it will need access to `umrs_core::i18n::tr`.
Add `use umrs_core::i18n;` to `output.rs`.

Format pattern for label + value:
```rust
println!("{} {}", i18n::tr("Resource Pool:"), pool.name);
```

### 2b. Makefile changes

Add to the i18n configuration block (after Step 1's changes are in):
```makefile
I18N_ACTIVE_DOMAINS := umrs-ls umrs-state umrs-logspace   # add umrs-logspace

I18N_SRC_DIR_umrs_logspace := components/rusty-gadgets/umrs-logspace/src

I18N_ACTIVE_LOCALES_umrs_logspace := fr_CA
```

### 2c. Scaffold + extract

```bash
make i18n-setup
make i18n-extract-umrs-logspace
# verify resources/i18n/umrs-logspace/umrs-logspace.pot contains wrapped strings
msginit --input=resources/i18n/umrs-logspace/umrs-logspace.pot \
        --locale=fr_CA --output=resources/i18n/umrs-logspace/fr_CA.po
make i18n-merge-umrs-logspace
make i18n-compile-umrs-logspace
```

### 2d. Build verification

```bash
cd components/rusty-gadgets && cargo xtask clippy && cargo xtask test
cargo run -p umrs-logspace -- --help
```

**→ DONE. Report final status.**

---

## Makefile: also remove umrs-core from I18N_ACTIVE_DOMAINS

After tr_core() cleanup, `umrs-core` source has zero `tr()` calls. Extraction produces
an empty .pot. Remove it from active domains to keep the pipeline clean:

```makefile
# Before
I18N_ACTIVE_DOMAINS := umrs-ls umrs-core umrs-platform

# After (also adds state + logspace via steps above)
I18N_ACTIVE_DOMAINS := umrs-ls umrs-state umrs-logspace
```

Also remove:
- `I18N_SRC_DIR_umrs_core`
- `I18N_ACTIVE_LOCALES_umrs_core`

Note: `umrs-platform` is left out for the same reason — no wrapped strings, empty pot.
If platform strings are added later, re-add it.

---

## Files Modified

| File | Change |
|---|---|
| `Makefile` | Update `I18N_ACTIVE_DOMAINS`, add `I18N_SRC_DIR_*` and `I18N_ACTIVE_LOCALES_*` for both crates; remove umrs-core entries |
| `umrs-state/src/main.rs` | Add `i18n::init`, wrap user-facing strings |
| `umrs-logspace/src/main.rs` | Add `i18n::init` |
| `umrs-logspace/src/output.rs` | Wrap display label strings |
| `resources/i18n/umrs-state/` | New: `.pot`, `fr_CA.po`, `fr_CA.mo` |
| `resources/i18n/umrs-logspace/` | Updated: `.pot`, add `fr_CA.po`, `fr_CA.mo` |
