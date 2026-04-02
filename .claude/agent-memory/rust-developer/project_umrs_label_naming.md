---
name: umrs-label crate naming — lib is umrs_labels (plural)
description: The umrs-label package's library target is named umrs_labels (plural) — tests and examples use this name
type: project
---

# umrs-label crate naming

**Package name:** `umrs-label` (in `Cargo.toml`, singular)
**Binary name:** `umrs-label` (singular)
**Library name:** `umrs_labels` (plural) — declared via `[lib] name = "umrs_labels"` in `Cargo.toml`

Tests and examples use `use umrs_labels::cui::catalog;` — plural form.
`src/main.rs` also uses `use umrs_labels::cui::catalog;` after the fix.

**Why:** The codebase was written expecting `umrs_labels` (plural) in all consumers but the `Cargo.toml` had no `[lib]` section, so the auto-detected name was `umrs_label` (singular). Fixed by adding `[lib] name = "umrs_labels"` to `Cargo.toml`.

Also fixed: Rust 2024 edition requires explicit type annotations in closures passed to `filter` and `sort_by` when the iterator element type can't be inferred — annotate `|(_, m): &(_, &_)|` or `|a: &(&String, &catalog::Marking), b: &(...)|`.
