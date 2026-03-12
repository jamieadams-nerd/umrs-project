---
name: project-cleanup
description: Housekeeping sweep. Merges local.settings.json, consolidates rogue .claude/ directories, and retires completed plans.
---

# Project Cleanup

This skill performs three housekeeping passes over the repository. Report
findings as you go so the user can see what changed.

## Pass 1 — Merge stray `local.settings.json` files

1. Search the entire repo for any file named `local.settings.json`.
2. If found, read its contents and read `.claude/settings.json`.
3. Merge the keys from `local.settings.json` into `.claude/settings.json`
   (`.claude/settings.json` values win on conflict).
4. Delete the stray `local.settings.json` file after merging.
5. Report what was merged or report "No stray local.settings.json found."

## Pass 2 — Consolidate rogue `.claude/` directories

1. Search for any `.claude/` directory that is NOT the top-level
   `/media/psf/repos/umrs-project/.claude/`.
   - Use: `find /media/psf/repos/umrs-project -name '.claude' -type d`
   - Exclude the top-level one from results.
2. For each rogue `.claude/` found:
   a. List its contents.
   b. Determine the appropriate merge target under the top-level `.claude/`.
   c. Copy files into the top-level `.claude/`, preserving subdirectory
      structure where sensible.
   d. Remove the rogue `.claude/` directory.
3. Report what was moved or report "No rogue .claude/ directories found."

## Pass 3 — Review and retire completed plans

1. Read each `.md` file in `.claude/plans/` (skip the `completed/` subdirectory).
2. For each plan, check its status by reading the frontmatter or header.
   A plan is considered complete if its status field contains any of:
   `completed`, `done`, `implemented`, `shipped`, `closed`.
3. For plans that are complete:
   a. Ensure `.claude/plans/completed/` exists.
   b. If the plan does not already have a `status: completed` line, update
      the status field to `completed` (preserve existing frontmatter format).
   c. Move the file to `.claude/plans/completed/`.
   d. Report each plan moved.
4. For plans that are NOT complete, report their name and current status
   so the user has visibility.

## Output

Provide a summary at the end:

```
## Project Cleanup Summary

### Settings
- <what happened>

### Rogue .claude/ directories
- <what happened>

### Plans
- Retired: <list of completed plans moved>
- Active: <list of plans still in .claude/plans/ with their status>
```
