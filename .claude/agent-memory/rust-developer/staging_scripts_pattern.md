---
name: Staging scripts recursion pattern
description: stage_scripts() recurses one level; strips .sh on copy; duplicate stem = hard error
type: project
---

`xtask/src/stage.rs` `stage_scripts()` (2026-04-16):

- Collects `scripts/*.sh` (flat) and `scripts/*/*.sh` (one level). Depth ≥ 2 silently skipped.
- Strips `.sh` suffix: `umrs-sign-mgr.sh` → `staging/bin/umrs-sign-mgr`.
- Duplicate stem across flat + nested is a `bail!` hard error (SA-12 provenance concern).
- Execute-bit guard preserved; non-executable scripts warn and skip.
- Helper `collect_scripts()` does per-directory scanning; `stage_scripts()` orchestrates two passes.

**Why:** Two real scripts (`umrs-sign-mgr.sh`, `umrs-shred.sh`) live in nested subdirs and were
silently skipped by the old flat-only scan. Suffix strip matches compiled binary naming convention.

**How to apply:** When adding new scripts under `scripts/`, commit with `git add --chmod=+x`.
Nested placement is fine (one level); duplicate stems with existing flat scripts are caught at stage time.
