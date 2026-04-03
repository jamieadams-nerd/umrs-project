Perfect. Here are the revised instructions:

---

## Instructions for Rusty

```markdown
# Task: Implement xtask staging pipeline

## Workspace Layout (for context)

The workspace root is `components/rusty-gadgets/`. Structure:

    libs/
      umrs-core/
      umrs-hw/
      umrs-platform/
      umrs-selinux/    ← has config/
      umrs-ui/
    umrs-label/        ← binary crate, has config/ca/ and config/us/
    umrs-ls/           ← binary crate
    umrs-stat/         ← binary crate
    umrs-uname/        ← binary crate
    scripts/           ← end-user operational scripts (key creation, etc.)
    xtask/             ← this crate
    staging/           ← OUTPUT, ephemeral, in .gitignore

## Commands to Implement

    cargo xtask stage            # stages debug build output
    cargo xtask stage --release  # stages release build output
    cargo xtask clean            # removes staging/

## workspace_root()

xtask lives at `<workspace_root>/xtask/`. Use CARGO_MANIFEST_DIR
and go up exactly one level:

    fn workspace_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("xtask has no parent directory")
            .to_owned()
    }

Never hardcode paths. Never use relative paths from cwd.

## Staging Layout

    staging/
      bin/        ← compiled binaries + end-user scripts, merged
      config/     ← all config/ contents from any crate, merged

## Staging Rules

### 1. Compiled Binaries → staging/bin/

Scan `target/debug/` or `target/release/` (based on --release flag).

Include a file if ALL of the following are true:
- It is a regular file (not a symlink, not a directory)
- It has no file extension
- It is executable (Unix mode & 0o111 != 0)

Exclude files matching these extensions regardless:
  .d  .so  .rlib  .rmeta  .dylib  .a

If the target directory does not exist, print:
  "target/debug/ not found — run 'cargo build --workspace' first"
and exit with code 1.

### 2. End-User Scripts → staging/bin/

If `scripts/` exists at workspace root:
- Copy every file from `scripts/` (non-recursive, flat) into
  `staging/bin/`
- Verify each file has execute permission after copy
- If a script is not executable in the source, print a warning:
  "WARNING: scripts/<name> is not executable — skipping"
  and skip it rather than copying a non-executable file
- If `scripts/` does not exist, skip silently — do not error

Scripts and binaries share staging/bin/ without prefixing.
The installer will handle final placement.

### 3. Config Files → staging/config/

Scan these directories for a `config/` subdirectory:
  libs/umrs-core/
  libs/umrs-hw/
  libs/umrs-platform/
  libs/umrs-selinux/
  libs/umrs-ui/
  umrs-label/
  umrs-ls/
  umrs-stat/
  umrs-uname/

For each crate that HAS a `config/` directory:
- Copy recursively into `staging/config/`
- Preserve subdirectory structure
  (e.g. umrs-label/config/ca/ → staging/config/ca/)
- If no `config/` exists for a crate, skip silently

There will be no filename collisions across crates.
Do not add any crate-name prefixing.

## Execute Bit Note

`fs::copy` preserves permissions from source on Linux.
Scripts in `scripts/` MUST be committed to git with execute
bit set. When adding a new script to the repo:

    git add --chmod=+x scripts/<name>.sh

Rusty: add a note to the team doc reminding contributors of this.

## Error Handling

- No unwrap() anywhere — use proper Result propagation
- Main entry point: `fn main() -> anyhow::Result<()>`
- Add `anyhow` to xtask/Cargo.toml dependencies if not present
- All directory creation via `fs::create_dir_all` (idempotent)
- Print clear human-readable message before returning any error

## NIST / CMMC Annotations Required

    // AC-3:  Access Enforcement — binary and script execute
    //        permission check before staging
    // CM-2:  Baseline Configuration — config file staging
    // CM-7:  Least Functionality — only stage known artifact types
    // SA-12: Supply Chain Protection — staging is pre-installation
    //        checkpoint before IMA signing

## Code Structure

    xtask/src/
      main.rs     ← arg parsing and dispatch only
      stage.rs    ← all staging logic
      clean.rs    ← clean logic

## Tests

No inline tests. External file only: `xtask/tests/stage_test.rs`

Tests must cover:
- workspace_root() returns a path containing Cargo.toml
- is_stageable_binary() correctly filters .d .so .rlib etc.
- Non-executable script is skipped with warning, not copied
- Staging with missing target/ exits nonzero with correct message
- Config copy preserves subdirectory structure (ca/, us/)
- Missing scripts/ does not cause an error

## .gitignore

Verify `staging/` is in the workspace root `.gitignore`.
Add it if missing. Do not add it to any crate-level .gitignore.
```

---

## Makefile

```makefile
# Makefile — components/rusty-gadgets/

.PHONY: build release clean-staging

build:
	cargo build --workspace
	cargo xtask stage

release:
	cargo build --release --workspace
	cargo xtask stage --release

clean-staging:
	cargo xtask clean
```

This Makefile should be callable fromt he top-level project make. 

That's the complete picture. When you write the installer later, everything it needs is in two flat-ish directories: `staging/bin/` and `staging/config/`. Clean handoff.


