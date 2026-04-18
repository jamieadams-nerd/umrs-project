---
name: Rust coding conventions — source headers, use ordering, doc tests, test tags, i18n, doc-comment conventions
description: Project-specific coding conventions not captured in CLAUDE.md or rules files
type: feedback
---

## Source File Headers

Every `.rs` source file must begin with this header block:

```rust
// SPDX-License-Identifier: MIT
// Copyright (C) <year> <author or organization>
// Crate: <crate-name>
// Module: <module path relative to src/>
```

The SPDX line must appear before any other content, including `//!` doc comments.

**Why:** Audit and license compliance — allows automated SPDX tooling to scan the workspace.
**How to apply:** Add to every new `.rs` file. When touching existing files that lack the header, add it during the same edit.

---

## `use` Statement Ordering

Group `use` statements in this order, separated by blank lines:

1. `std::` imports
2. Third-party crate imports
3. Local crate imports (`crate::`, `super::`)

```rust
use std::collections::HashMap;
use std::path::PathBuf;

use thiserror::Error;

use crate::context::SecurityContext;
```

Rustfmt preserves blank lines between groups; it does not reorder groups. Maintain the order manually.

**Why:** Consistency and readability — auditors scanning imports can immediately locate the trust boundary between stdlib, external, and internal code.
**How to apply:** All new files and all `use` blocks added during edits.

---

## No Doc Tests

Documentation must be narrative. Do not write executable ```` ```rust ```` code blocks in doc comments that are intended to run as `cargo test` doc tests.

Use ```` ```rust,no_run ```` or ```` ```text ```` for illustrative code in doc comments. Examples that must compile and run belong in `examples/`.

**Why:** Doc tests in this project are a maintenance liability — they require the full crate to compile cleanly under doc test harness rules, and failures are silent in CI unless explicitly enabled. Narrative documentation is reviewed by auditors, not test runners.
**How to apply:** Never add ```` ```rust ```` fenced blocks to `///` or `//!` doc comments without `no_run` or `ignore`. Move runnable examples to `examples/`.

---

## Test Case Documentation Tags

Integration tests in `tests/` must include structured doc comment tags. Place these in the doc comment for each `#[test]` function:

```rust
/// # TEST-ID: SEL-AUTH-001
/// # REQUIREMENT: SecurityContext must reject empty user field
/// # COMPLIANCE: NIST SP 800-53 AC-3
#[test]
fn test_security_context_rejects_empty_user() {
    ...
}
```

Tags:
- `TEST-ID` — unique identifier within the test file (format: `<MODULE>-<CATEGORY>-<NNN>`)
- `REQUIREMENT` — plain-English statement of what is being verified
- `COMPLIANCE` — applicable NIST SP 800-53, CMMC, or NSA RTB control

All three tags are required for security-relevant tests. For utility/structural tests with no direct control mapping, `COMPLIANCE` may be omitted with a comment explaining why.

**Why:** Audit-readiness — security auditors trace test coverage to requirements and controls. Tags make this machine-scannable without reading test bodies.
**How to apply:** New tests in security-relevant modules. Retrofit when touching existing test files.

---

## i18n Coding Rule

i18n is opt-in. Do not wrap strings proactively. Only wrap strings that meet ALL of these criteria:

1. The string is **interactive and user-facing** — it appears in TUI output, CLI output, or error messages shown to an operator.
2. The string is in a **tool binary** — libraries return typed values, not translated strings.
3. The string is **not a security label, path, identifier, or log key** — those must never be translated.

Use the UMRS core i18n module. Do not use `gettext` or other crates directly.

```rust
// Correct — user-facing TUI label
label: i18n::tr("Trust Tier"),

// Incorrect — security label, must not be translated
label: i18n::tr("system_u:object_r:etc_t:s0"),

// Incorrect — library code must not call tr()
pub fn describe(&self) -> String { i18n::tr("...") }  // NO
```

i18n assets live at the repository top level, not inside crates.

**Why:** Translation of security labels or identifiers would corrupt semantics. Library-level translation breaks the separation between typed values and display strings.
**How to apply:** At every new string — ask "is this interactive, user-facing, and in a binary?" If not, do not wrap.

---

## Doc-Comment Conventions (Conventions A/B/C — applied project-wide 2026-04-18)

### Convention A — Enum doc-comments use `## Variants:` in the main block

Per-variant `///` comments are removed from variant declarations. Content is consolidated into a `## Variants:` section in the enum's own doc block. For struct variants with named fields, inner field docs are also moved into the variant description.

**Exception:** `#[derive(Parser)]` / `#[derive(Subcommand)]` enums (clap CLI types) — `///` on variants and enum fields are rendered by clap as `--help` text. These MUST remain as per-item doc comments and must NOT be migrated to a `## Variants:` block.

### Convention B — Struct doc-comments use `## Fields:` in the main block

Per-field `///` comments are removed from field declarations. Content is consolidated into a `## Fields:` section in the struct's own doc block. Private fields are listed with `(private)` notation.

**Exception:** `#[derive(Parser)]` structs (clap CLI types) — `///` on fields are rendered by clap as argument help text. These MUST remain as per-field doc comments.

### Convention C — `## Compliance` header for NIST/CMMC/RTB citations

Bare NIST SP 800-53, NIST SP 800-171, CMMC, FIPS, RTB, or RMF control citations in doc comments are placed under a `## Compliance` markdown header and formatted as dash-bullet list items.

**Applies to:** Module-level `//!` blocks, struct/enum-level `///` blocks, function-level `///` blocks.
