---
name: new-crate
description: >
  Scaffolds a new Rust crate (library or binary) inside the
  components/rusty-gadgets/ workspace. Trigger this skill whenever the user
  asks to create a new crate, add a new package to the workspace, scaffold a
  new library or binary, or says things like "add a new crate called X",
  "create a new tool", "start a new workspace member", or "new binary for Y".
  Also trigger when discussion moves toward implementing a new tool or library
  that does not yet exist in the workspace.
---

# New Crate Scaffold

This skill walks through every step required to add a new workspace member to
`components/rusty-gadgets/`. It covers both `--lib` and `--bin` crates.

The goal is a crate that builds cleanly, follows all project conventions, and
is ready for real development — not just a bare `cargo new` skeleton.

## Pre-flight

Before creating anything, confirm with the user:

1. **Crate name** — should follow the `umrs-*` naming convention unless there
   is a reason not to.
2. **Crate kind** — library (`--lib`) or binary (`--bin`).
3. **Workspace dependencies** — check the dependency rules table in CLAUDE.md.
   At minimum, binary crates typically depend on `umrs-core`. Library crates
   may have no workspace dependencies (like `umrs-platform`) or depend only on
   allowed upstream crates.
4. **Purpose** — one sentence describing what this crate does, for the module
   doc comment.

## Step 1 — Create the crate

Run `cargo new <name> --lib` (or `--bin`) from inside `components/rusty-gadgets/`.

Then create the standard subdirectories:

```
components/rusty-gadgets/<name>/tests/
components/rusty-gadgets/<name>/examples/
```

## Step 2 — Register in workspace

Add the new crate name to the `members` array in
`components/rusty-gadgets/Cargo.toml`. Keep the list sorted alphabetically.

## Step 3 — Set up Cargo.toml dependencies

Add workspace dependencies as agreed in pre-flight. For binary crates, the
typical starting set is:

```toml
[dependencies]
umrs-core = { path = "../umrs-core" }
log = "0.4"
systemd-journal-logger = "2"
```

Do NOT add `env_logger` — this project uses `systemd-journal-logger` for
structured journald output.

## Step 4 — Set up the crate root (lib.rs or main.rs)

Every crate root must include these elements in order:

### 4a. Copyright header

```rust
// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams
```

### 4b. Forbid unsafe code

```rust
// NIST 800-218 SSDF PW.4 / NSA RTB: Provable safe-code guarantee.
// #![forbid] cannot be overridden by any inner #[allow] — this is a
// compile-time proof, not a policy.
#![forbid(unsafe_code)]
```

### 4c. Clippy lints

```rust
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![deny(clippy::unwrap_used)]
// Lint suppressions — rationale mirrors umrs-selinux policy:
#![allow(clippy::option_if_let_else)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::doc_markdown)]
```

### 4d. Module doc comment

```rust
//! # <Crate Name>
//!
//! <One-sentence purpose from pre-flight.>
//!
//! NIST SP 800-53 <relevant controls> / NSA RTB <relevant principles>.
```

### 4e. For binary crates — journald logging setup

Binary crates use `systemd-journal-logger` (not `env_logger`) for structured
logging to the systemd journal:

```rust
use log::LevelFilter;
use systemd_journal_logger::JournalLog;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    JournalLog::new()
        .expect("failed to connect to journald")
        .with_syslog_identifier("umrs".to_string())
        .install()
        .expect("failed to install journal logger");
    log::set_max_level(LevelFilter::Info);

    // Structured logging — fixed syntax: "key" => value; "message"
    // View with: sudo journalctl -t <crate-name> -o json-pretty
    log::info!(target: "<crate-name>"; "Started");

    Ok(())
}
```

### 4f. For binary crates — i18n initialization (optional)

If the crate will have user-facing strings that need translation, add i18n
initialization. This is optional and can be added later:

```rust
use umrs_core::i18n;

// Inside main(), before any user-facing output:
i18n::init("<crate-name>");
```

Mark this section with a comment indicating it is optional and can be removed
if the crate has no translatable strings.

## Step 5 — Verify the build

Run `cargo build -p <name>` to confirm the crate compiles cleanly.

Then run `cargo xtask clippy` from the workspace root to verify no lint
warnings are introduced.

## Step 6 — Summary

After all steps complete, report:
- Crate name and kind
- Files created
- Dependencies added
- Any follow-up items (e.g., "add to CLAUDE.md workspace layout table",
  "create initial test file", "add to CHANGELOG")
