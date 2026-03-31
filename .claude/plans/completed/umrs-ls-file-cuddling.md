# Plan: umrs-ls File Cuddling (Compact View)

**Status:** Completed — all 3 phases implemented 2026-03-31. 30 tests, clippy clean.

**ROADMAP alignment:** G4 (Tool Ecosystem), G8 (Human-Centered Design)

**Source:** `jamies_brain/file-cuddling-umrs-ls.md` (Jamie Adams, archived 2026-03-19)

---

## Problem

Directories like `/var/log/` contain dozens of related files — base file plus rotations,
archives, and signatures. In the current `umrs-ls` output, these all appear as individual
rows, burying the meaningful files in noise. An operator scanning for security-relevant
files has to mentally group `boot.log`, `boot.log-20251210`, `boot.log-20251211`, etc.

## Design

### Compact View ("Cuddling")

Related files are grouped under the base file with a summary line:

```
boot.log                          4.2 KiB
  └ 7 rotations                  18.6 KiB total

messages                         12.1 KiB
  └ 3 rotations                   9.8 KiB total

report.pdf                        1.2 MiB
  └ 1 signature                   256 B
```

### Grouping Rules

1. Files are sorted lexically (already the case)
2. A file is the **base** if it is the shortest name in a prefix group
3. A **sibling** is any file that starts with the base name AND has a separator
   character immediately after (`.`, `-`, `_`)
4. Grouping is O(n), single-pass, no allocations beyond the output

**Separator check prevents false positives:**
- `file.log` groups with `file.log.1`, `file.log-20260301`
- `file.log` does NOT group with `file.log_backup_copy` unless `_` is accepted
- Accept separators: `.`, `-`, `_`

### Sibling Classification

| Suffix pattern | Label | Example |
|---|---|---|
| Numeric after separator (`.1`, `-20260301`) | rotation | `boot.log.1`, `boot.log-20260301` |
| `.gz`, `.bz2`, `.xz`, `.zst` | compressed rotation | `syslog.1.gz` |
| `.sig`, `.asc`, `.p7s` | signature | `report.pdf.sig` |
| `.sha256`, `.sha512`, `.md5` | checksum | `package.tar.sha256` |
| `.bak`, `.orig`, `.old` | backup | `config.bak` |
| Everything else matching prefix | related | `access.log.tmp` |

### Display

Each group shows:
- Base file row (full detail — permissions, security context, size, etc.)
- Summary line: `└ N <label>  <aggregate size> total`
- If mixed types: `└ 3 rotations, 1 signature  19.2 KiB total`

### TUI Compatibility

The grouping data must be structured, not just display formatting:

```rust
struct FileGroup {
    base: SecureDirent,
    siblings: Vec<Sibling>,
}

struct Sibling {
    entry: SecureDirent,
    kind: SiblingKind,
}

enum SiblingKind {
    Rotation,
    CompressedRotation,
    Signature,
    Checksum,
    Backup,
    Related,
}
```

When `umrs-ls` becomes a TUI:
- Collapsed group shows the summary line (default)
- Expanding a group reveals individual siblings
- Siblings are selectable
- Group row carries aggregate metadata (total size, count by kind)

### Context Detection

In log directories (`/var/log/`, or directories where >50% of files match rotation
patterns), prefer "rotations" label. Otherwise use generic "related" or the specific
type label.

---

## Implementation Phases

### Phase 1: Grouping Logic (library)

**Scope:** `umrs-ls/src/` — new module `grouping.rs`

1. Implement `group_entries(entries: &[SecureDirent]) -> Vec<FileGroup>`
2. `SiblingKind` classification based on suffix patterns
3. Aggregate size computation per group
4. Tests: rotation patterns, signature detection, mixed groups, no false positives

**Deliverable:** `group_entries()` produces correct `FileGroup` vectors from sorted entry lists.

### Phase 2: CLI Display

**Scope:** `umrs-ls/src/` — update display/formatting module

1. Default: cuddled view (grouped)
2. `--no-cuddle` or `--flat` flag: traditional flat listing
3. Summary line formatting with aggregate sizes
4. Mixed-type groups show all labels

**Deliverable:** `umrs-ls` output shows cuddled groups by default.

### Phase 3: JSON Output

**Scope:** `--json` mode

1. `FileGroup` serializes to JSON with base + siblings array
2. Each sibling carries its `SiblingKind` tag
3. Aggregate size in group metadata

**Deliverable:** `umrs-ls --json` produces grouped output.

---

## Constraints

- O(n) grouping — no quadratic prefix matching
- No heap allocations in the inner loop beyond the output vectors
- Separator check is mandatory — `starts_with` alone causes false positives
- The grouping module must be independent of display — TUI will consume the same `FileGroup` type
- `#![forbid(unsafe_code)]` — no exceptions

---

## Source Material

Derived from Jamie's research notes:
- `jamies_brain/file-cuddling-umrs-ls.md` (archived 2026-03-19)
