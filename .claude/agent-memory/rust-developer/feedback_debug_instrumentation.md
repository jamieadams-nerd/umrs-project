---
name: Debug instrumentation patterns and naming conventions
description: Naming, gating, and Error Information Discipline requirements for #[cfg(debug_assertions)] timing blocks; pre-existing setrans_tests failures
type: feedback
---

## Instant variable naming in debug blocks

Name the Instant variable `start`, never `_start`.

**Why:** `_start` tells clippy the variable is intentionally unused. When the same variable is referenced inside the `#[cfg(debug_assertions)]` block, `clippy::used_underscore_binding` fires and causes a build failure under `-D warnings`. This occurred in three umrs-selinux files simultaneously (context.rs, mls/level.rs, secure_dirent.rs).

**How to apply:** Always write `let start = Instant::now();` inside `#[cfg(debug_assertions)]` blocks. No underscore prefix.

## const fn requirement

`missing_const_for_fn` fires on simple match-over-self methods — always make them `const fn`. The project rule is: make helpers const, never suppress.

**Why:** Clippy pedantic+nursery treats this as a warning promoted to error via `-D warnings`.
**How to apply:** Any new `pub fn` returning `&'static str` via a match on `self` must be `const fn`.

## log crate gating

The `log` crate workspace-standard config is `{ version = "0.4", features = ["release_max_level_info"] }`. This compiles `log::debug!()` to nothing in release builds. The `#[cfg(debug_assertions)]` guard on the Instant capture additionally removes the timing machinery itself in release. Both guards are complementary — use both.

## Error Information Discipline in debug logs (SI-11)

Log only: function names, enum variant names (e.g., `{kind:?}` for `UmrsPattern`), computed sizes (char count, not string content), bool results, elapsed µs. Never log: input string values, configuration values, MLS level strings, or any security-relevant data.

## Pre-existing setrans_tests failures

The 7 failing `umrs-label/tests/setrans_tests.rs` tests (Canadian Protected c300/c301/c302 vs c200/c201/c202, US entry count 143 vs 121) are pre-existing in the codebase as of 2026-04-03. They are in protected catalog/conf files. Do not attempt to fix without Jamie's direction.
