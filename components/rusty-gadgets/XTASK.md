Got it. Here’s a **step-by-step** “xtask” setup that fits your `rusty-gadgets` workspace and mirrors the pattern you used in the unicode project (a dedicated helper crate that runs workspace chores).

## Goal

Add a new workspace member `xtask/` that you run as:

* `cargo xtask <cmd> …` (via cargo alias), or
* `cargo run -p xtask -- <cmd> …` (always works)

---

## Step 0 — Decide what xtask will do (initial set)

Start small and useful for this workspace:

* `xtask fmt` → `cargo fmt --all`
* `xtask clippy` → `cargo clippy --workspace --all-targets -- -D warnings`
* `xtask test` → `cargo test --workspace`
* `xtask robots` → run `robotgen` (or regenerate `umrs-core/src/robots/data.rs` if you wire that in)
* `xtask selinux` (later) → build/install policy bits from `umrs-core/selinux/`

You can add more later without changing the structure.

---

## Step 1 — Create the crate

From workspace root:

```bash
cargo new xtask --bin
```

You now have:

```
xtask/
  Cargo.toml
  src/main.rs
```

---

## Step 2 — Add `xtask` to the workspace

Edit **workspace root** `Cargo.toml` and add `xtask` to members.

Example:

```toml
[workspace]
members = [
  "cui-labels",
  "robotgen",
  "umrs-core",
  "umrs-logspace",
  "umrs-state",
  "vaultmgr",
  "xtask",
]
resolver = "2"
```

(Keep your existing list—just add `"xtask"`.)

---

## Step 3 — Add a cargo alias so you can run `cargo xtask`

Create or edit **workspace root** `.cargo/config.toml`:

```toml
[alias]
xtask = "run -p xtask --"
```

Now you can do:

```bash
cargo xtask fmt
```

If you don’t want another file, you can skip this and run the explicit command in Step 6.

---

## Step 4 — Minimal `xtask` Cargo.toml

Edit `xtask/Cargo.toml` to be lean:

```toml
[package]
name = "xtask"
version = "0.1.0"
edition = "2021"
publish = false

[dependencies]
anyhow = "1"
```

(You can add `clap` later if you want structured subcommands; start with simple parsing.)

---

## Step 5 — Implement the runner

Edit **`xtask/src/main.rs`**:

```rust
use anyhow::{bail, Context, Result};
use std::env;
use std::process::{Command, Stdio};

fn run(cmd: &mut Command) -> Result<()> {
    eprintln!("[xtask] {:?}", cmd);
    let status = cmd
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .context("failed to spawn command")?;

    if !status.success() {
        bail!("command failed: {:?}", cmd);
    }
    Ok(())
}

fn main() -> Result<()> {
    let mut args = env::args().skip(1);
    let sub = args.next().unwrap_or_else(|| "help".to_string());

    match sub.as_str() {
        "fmt" => {
            run(Command::new("cargo").arg("fmt").arg("--all"))?;
        }
        "clippy" => {
            run(
                Command::new("cargo")
                    .arg("clippy")
                    .arg("--workspace")
                    .arg("--all-targets")
                    .arg("--")
                    .arg("-D")
                    .arg("warnings"),
            )?;
        }
        "test" => {
            run(Command::new("cargo").arg("test").arg("--workspace"))?;
        }
        "robots" => {
            // Example: run your robot generator crate (adjust args as needed)
            run(Command::new("cargo").args(["run", "-p", "robotgen", "--"]))?;
        }
        "help" | "-h" | "--help" => {
            eprintln!(
                "Usage: cargo xtask <cmd>\n\
                 \n\
                 Commands:\n\
                 \tfmt\n\
                 \tclippy\n\
                 \ttest\n\
                 \trobots\n"
            );
        }
        other => bail!("unknown xtask command: {other}"),
    }

    Ok(())
}
```

Notes:

* Uses `Command::new("cargo")` (no `env` shebang issue here; this is Rust).
* Inherits stdio so output looks normal.
* Explicit failure handling.

---

## Step 6 — Run it

From workspace root:

```bash
cargo run -p xtask -- help
cargo run -p xtask -- fmt
cargo run -p xtask -- clippy
```

If you created the alias in Step 3:

```bash
cargo xtask fmt
cargo xtask clippy
cargo xtask test
```

---

## Step 7 — Optional: wire into your workflow

A common pattern is `cargo xtask ci` that runs fmt+clippy+test in order. Add a match arm:

```rust
"ci" => {
    run(Command::new("cargo").args(["fmt", "--all"]))?;
    run(Command::new("cargo").args(["clippy", "--workspace", "--all-targets", "--", "-D", "warnings"]))?;
    run(Command::new("cargo").args(["test", "--workspace"]))?;
}
```

---

If you want, tell me which **2–3** real chores you want first (robotgen regeneration into `umrs-core/src/robots/data.rs`, vaultmgr directory bootstrap, setrans build/install, etc.) and I’ll map them into `xtask` as concrete subcommands with the exact files/paths.

