---
name: # Errors section placement and linter race condition
description: Rules for # Errors doc section placement and how to handle a background linter injecting duplicate/misplaced doc comments
type: feedback
---

## `# Errors` section placement rule

`# Errors` doc sections MUST appear BEFORE `#[must_use]` and other attributes,
never between an attribute and `pub fn`. The correct order is:

```rust
/// Brief description.
///
/// # Errors
///
/// Returns `FooError::Bar` if ...
#[must_use = "..."]
pub fn my_fn() -> Result<T, FooError> {
```

**Why:** Rustdoc parses doc comments as attached to the item immediately following
them. If a `#[must_use]` or other attribute appears between the doc comment and
`pub fn`, rustdoc emits E0585 "found a documentation comment that doesn't
document anything" in some positions, and clippy emits `empty_line_after_doc_comments`
in others.

**How to apply:** Always scan the lines immediately below your `# Errors` addition.
If an attribute (`#[must_use]`, `#[allow(...)]`, etc.) sits between the doc section
and `pub fn`, the `# Errors` section must precede ALL attributes.

---

## Linter injection race condition

A background code-action tool (language server / IDE) fires on every file save
and injects generic `# Errors` sections. This creates several failure modes:

1. **Duplicate `# Errors` sections** — your specific text + the linter's generic text
2. **Doc comments injected inside code blocks** (E0585) — the linter fires after the
   closing `}` of a for loop or inside an array literal, injecting `///` lines that
   are not adjacent to any item
3. **Linter re-adds `#![allow(clippy::missing_errors_doc)]`** — after removal

**Fix:** Use the `Write` tool (full file rewrite) instead of the `Edit` tool for
any file actively being processed by the linter. A full write is atomic from the
linter's perspective and prevents injection during the edit window.

**Diagnosis:** E0585 errors like "found a documentation comment that doesn't
document anything" at a line inside a function body or array literal are a
reliable signal that the linter has injected a doc comment in the wrong position.

**How to apply:** When multiple `# Errors` additions are needed in the same file,
prefer a single Write rewrite over sequential Edit calls.
