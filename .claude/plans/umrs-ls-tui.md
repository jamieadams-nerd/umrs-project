# Plan: umrs-ls TUI — Directory Security Browser

**Status:** in-progress — Phases 1-3 complete + custom renderer delivered. Canadian group-header lookup bug fixed 2026-04-15 (MCS-level fallback). Phase 4 (umrs-stat) and Phase 5 (palette colors) remain. Dash-separator bug still open.
**Date:** 2026-04-04
**ROADMAP Goals:** G4 (Tool Ecosystem), G5 (Security Tools), G8 (Human-Centered Design)

**Sources:**
- `.claude/jamies_brain/archive/tui-feedback.md` — audit card template requirements
- `.claude/jamies_brain/archive/file-cuddling-umrs-ls.md` — cuddling requirements (completed)
- `.claude/jamies_brain/archive/next-big-steps.md` — release sequencing
- `.claude/jamies_brain/archive/notes-catalog-schema-2026-03-25.txt` — catalog + ls integration vision

---

## Problem

`umrs-ls` is currently a CLI-only tool that outputs a flat text listing to stdout.
It has rich security data (SELinux context, MCS marking, IOV flags, security observations,
file cuddling) but no interactive way to explore it. Operators scanning a directory for
security-relevant files must read linearly — they cannot drill into entries, expand
cuddled groups, or see file detail.

## Goal

Convert `umrs-ls` into a TUI application using the existing `umrs-ui` **viewer pattern**
(`ViewerApp` + `ViewerState` + `TreeModel`). The viewer pattern was explicitly designed
for this use case (the module doc names `umrs-ls` TUI mode as an intended consumer).

The CLI output mode (`--json`, flat text) must remain available. The TUI is the default
interactive mode; pipe detection determines the mode (TUI when terminal, text when piped).

---

## Decisions (Jamie, 2026-04-04)

1. **Tabs:** One tab only (directory listing). No summary tab for now.

2. **Navigation:** Up/down arrows highlight rows. Selecting a directory (including `.`
   and `..`) opens that directory and replaces the current listing. A static display
   above the tab bar shows the current directory path.

3. **File selection:** Pressing Enter on a file launches `umrs-stat` on that file.
   **Open design question:** How to display umrs-stat — spawn as separate process,
   or link to its library and render in-process? (See Phase 4 below.)

4. **Recursive scanning:** Yes — selecting a directory re-scans and navigates into it.
   The tree shows one directory level at a time (like a file manager), not a deep tree.

5. **Cuddled siblings:** Tree approach — siblings appear as expandable child nodes
   under the base file.

6. **CLI fallback:** Pipe detection only. No `--cli` flag. TUI when stdout is a TTY,
   text output when piped or `--json` is passed.

7. **Color source:** Do NOT use `secolor.conf`. Color comes from `umrs-labels` JSON
   catalog palette data. The existing CLI group header with reverse/color/unicode must
   be preserved — do not break that rendering.

8. **Sequencing:** Build `umrs-labels` first — it provides the public API for label
   lookup and palette colors that umrs-ls TUI depends on. This plan is blocked until
   umrs-labels has a usable library API.

9. **Access-denied entries:** Shown as a separate group in the tree.

---

## Implementation Order

1. **Phase 1** — Tree adapter (DirGroup → TreeModel)
2. **Phase 2** — ViewerApp implementation
3. **Phase 3** — TUI event loop + directory navigation
4. **umrs-stat compact mode** — Add `--compact` flag to umrs-stat (small centered card, not fullscreen)
5. **Phase 4** — File selection spawns umrs-stat
6. *(blocked on umrs-labels)* **Phase 5** — Color from umrs-labels palette

Phases 1-3 have no external dependencies and can start now.

---

## What Already Exists

| Component | Location | Status |
|---|---|---|
| CLI output with cuddling | `umrs-ls/src/main.rs` | Complete (30 tests) |
| Grouping engine | `umrs-ls/src/grouping.rs` | Complete |
| `SecureDirent` (fd-anchored dir reads) | `umrs-selinux` | Complete |
| `list_directory()` → `DirGroup` | `umrs-selinux::utils::dirlist` | Complete |
| `ViewerApp` trait + `ViewerState` | `umrs-ui::viewer` | Complete |
| `TreeModel` + `TreeNode` (filter, expand/collapse) | `umrs-ui::viewer::tree` | Complete |
| Detail panel renderer | `umrs-ui::viewer::detail` | Complete |
| Viewer layout renderer | `umrs-ui::viewer::layout` | Complete |
| Theme (group_title, indicators, etc.) | `umrs-ui::theme` | Complete |
| Keymap with search, expand/collapse, tabs | `umrs-ui::keymap` | Complete |

---

## What Needs to Be Built

### Phase 1 — Data Adapter: DirGroup → TreeModel

**Goal:** Convert `list_directory()` output into a `TreeModel` for a single directory level.

Tree structure (flat per-directory, not deep):

```
. (current dir)                               ← always present
.. (parent dir)                               ← always present
SELinux Group: admin_home_t :: s0             ← branch node (SELinux group header)
  .bashrc                                     ← leaf node (file)
  .bash_profile                               ← leaf node (file)
  .ssh/                                       ← leaf node (directory — selectable to navigate)
  known_hosts                                 ← branch node (has cuddled siblings)
    known_hosts.old                           ← child node (sibling: backup)
    known_hosts.bak                           ← child node (sibling: backup)
SELinux Group: httpd_sys_content_t :: CUI//LEI
  index.html                                  ← leaf node
Access Denied                                 ← branch node (separate group)
  .gnupg/                                     ← leaf node (permission denied)
```

**Key decisions:**
- `.` and `..` are always the first two entries (navigation affordance)
- SELinux groups become branch nodes with the group header as label
- Files are leaf nodes; directories are leaf nodes that trigger re-scan on Enter
- Cuddled siblings are child nodes under the base file (tree approach per Jamie)
- Access-denied entries are a separate group at the bottom
- Node metadata: mode, owner:group, SELinux context, size, mtime, IOV flags,
  security observations, sibling kind (for cuddled children)

**Files:**
- New: `umrs-ls/src/tree_adapter.rs` — `fn build_tree(groups: &[DirGroup], access_denied: &[String]) -> TreeModel`

### Phase 2 — Implement ViewerApp for umrs-ls

**Goal:** Wire up the `ViewerApp` trait.

```rust
struct DirViewerApp {
    current_path: PathBuf,
    groups: Vec<DirGroup>,
    access_denied: Vec<String>,
    entry_count: usize,
    elapsed_us: u64,
}
```

Trait methods:
- `card_title()` → `"UMRS Directory Security"`
- `tabs()` → single tab: `"Directory"`
- `status()` → entry count, SELinux group count, scan time
- `viewer_header()` → tool name, current path, entry count
- `initial_tree()` → call `build_tree()` from Phase 1

Must support re-scanning: when the user navigates into a directory, the app re-runs
`list_directory()` on the new path and rebuilds the tree.

**Files:**
- New: `umrs-ls/src/viewer_app.rs`

### Phase 3 — TUI Event Loop + Navigation

**Goal:** TUI event loop with directory navigation.

**Mode detection:**
- `--json` → JSON output, exit
- stdout is not a TTY → CLI text output, exit
- Otherwise → TUI mode

**Event loop:**
1. Enter alternate screen
2. Scan initial directory → create `DirViewerApp` + `ViewerState`
3. Render: current path bar + viewer
4. Key events → `state.handle_action(...)`
5. **Enter on a directory node** → re-scan, rebuild tree, reset state
6. **Enter on a file node** → launch umrs-stat (see Phase 4)
7. Loop until quit

**Current path display:** A static line between the tab bar and the tree panel
showing the absolute path of the directory being viewed. Not scrollable, not
selectable — just context.

**Files:**
- Modified: `umrs-ls/src/main.rs` — add TUI path, keep CLI path
- New dependency: `umrs-ui` in `umrs-ls/Cargo.toml`

### Phase 4 — File Selection → umrs-stat (Compact Subprocess)

**Goal:** Pressing Enter on a file spawns `umrs-stat` as a compact overlay — not fullscreen.
Escape closes umrs-stat and returns immediately to umrs-ls where the user left off.

**Approach:** Subprocess spawn.

1. umrs-ls suspends its alternate screen (leave alternate screen mode)
2. Spawn `umrs-stat <path>` as a child process — umrs-stat must support a compact
   display mode (not fullscreen). This likely means a new `--compact` or `--embed` flag
   on umrs-stat that renders a smaller card (e.g., 60x20 centered) instead of taking
   the full terminal.
3. umrs-stat runs its own event loop. Escape exits it (exit code 0).
4. umrs-ls waits for the child to exit, re-enters alternate screen, and redraws.
   The tree selection state is preserved — the user is right back where they were.

**umrs-stat changes needed:**
- A compact display mode flag (`--compact` or similar) that renders a smaller,
  centered card instead of fullscreen
- Escape must cleanly exit in this mode
- Must be installed and on PATH (or located via a known install path)

**Fallback:** If umrs-stat is not found on PATH, show an error in the status bar
("umrs-stat not found — install it to inspect files"). Do not crash.

### Phase 5 — Color from umrs-labels Palette

**Goal:** Color SELinux group headers and MCS markings using `umrs-labels` palette data.

**Dependency:** `umrs-labels` must expose a public API for:
- Looking up a marking string → palette color (RGB or terminal color)
- The existing `US-CUI-PALETTE.json` data

**Constraint:** Do NOT use `secolor.conf`. Do NOT break the existing CLI group header
rendering (reverse/color/unicode character combination in `group_separator()`).

In TUI mode, the tree node labels for SELinux group headers use the palette color.
File nodes use the standard theme colors.

**Files:**
- Modified: `umrs-ls/src/tree_adapter.rs` — apply palette colors to group header nodes
- Requires: `umrs-labels` public color API (not yet built)

---

## Blocked On

- **umrs-labels library API** — must be built first. Provides label lookup and palette
  colors. Without this, Phase 5 cannot be completed and the TUI lacks proper coloring.

---

## Non-Goals (This Plan)

- Directory mutation (rename, delete, relabel) — viewer is read-only
- Multi-directory comparison
- File content preview
- IMA/EVM verification display (future)
- Multiple tabs or summary views

---

## Dependencies

| Dependency | Status | Blocking? |
|---|---|---|
| `umrs-ui` (viewer pattern) | Complete | No |
| `umrs-selinux` (`SecureDirent`, `list_directory()`) | Complete | No |
| `umrs-ls` grouping engine | Complete | No |
| `umrs-core` (i18n, size formatting) | Complete | No |
| `umrs-labels` (label lookup, palette API) | **Not started** | **Yes — Phase 5** |
| `umrs-stat` (library API or installed binary) | Exists as binary | Phase 4 |

---

## Test Strategy

- `tree_adapter_tests.rs` — verify `DirGroup` → `TreeModel` conversion: node counts,
  `.`/`..` always present, access-denied group, cuddled siblings as children,
  metadata population
- `viewer_app_tests.rs` — verify `ViewerApp` trait: header context, tab definition,
  re-scan after directory change
- Integration: manual TUI testing on goldeneye against `/var/log/`, `/etc/selinux/`,
  home directories, and a CUI test vault (when available)
