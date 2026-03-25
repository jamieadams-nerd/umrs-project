---
name: feedback
description: Workflow and style preferences confirmed in this project
type: feedback
---

# Workflow Feedback

## Tool Selection
Use `rg` (ripgrep) for all searches, never built-in Search or grep.
Use `Bash(cat:*)` or `Bash(rg:*)` for reading; never the Read tool via the tool directly when project rules say Bash.

**Why:** Project CLAUDE.md RULE — hard enforcement.

**How to apply:** Always prefix searches with `cd components/rusty-gadgets && rg ...`.

## Path Rules
Never use `/media/psf/` paths. Use `/DEVELOPMENT/umrs-project/...`.

**Why:** `/media/psf/` mount is not traversable by subprocesses.

## No Git Commit or Push
Never run `git commit` or `git push`. Modify working files only.

**Why:** Project RULE — commits are Jamie's responsibility.

## Clippy Must Be Clean
Zero warnings. Fix the underlying issue; avoid `#[allow(...)]` suppressions.
Ask Jamie before adding any allow attribute — do not add unilaterally.

**Why:** `-D warnings` is enforced; any warning is a build failure.

## Tests in tests/ Only
No `#[cfg(test)]` or `mod tests` in src files. All tests in `tests/` directory.

**Why:** Project RULE — test placement.

## No Deletion of Documentation
If duplicate/redundant, flag it and ask. Never delete.

**Why:** Project RULE — documentation preservation.
