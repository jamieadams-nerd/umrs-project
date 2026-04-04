## Critical Coding Rules

- [RULE] Every crate root must have `#![forbid(unsafe_code)]`. When creating a new crate, include it. Exception: `umrs-hw` (hardware access requires unsafe). `#![forbid]` cannot be overridden by inner `#[allow]` — this is a compile-time proof (NIST SP 800-218 SSDF PW.4, NSA RTB).
- [RULE] Avoid FFI — always prefer pure Rust.
- TPI parsing — `SecurityContext` uses two independent parsers (`nom` + `FromStr`) and fails closed on disagreement.
- Rustfmt — 100-char max width, 4-space indent, Unix newlines.

## Clippy Policy

`lib.rs` enables `#![warn(clippy::pedantic)]` and `#![warn(clippy::nursery)]`.

**Principle:** Correctness and safety lints are law. Aesthetic lints are suppressed when they trade readability for style.

| Lint | Reason |
|---|---|
| `unwrap_used` | **Denied** — never allowed |
| `option_if_let_else` | `.map_or_else()` is less clear than `if let` |
| `redundant_closure` | Explicit closures are sometimes clearer at the call site |
| `module_name_repetitions` | `SelinuxUser` in module `selinux` is intentional |
| `missing_errors_doc` | `# Errors` on every `Result` fn is excessive noise |
| `missing_panics_doc` | `# Panics` for unreachable panics adds no value |
| `unreadable_literal` | Underscore grouping in hex/binary bitmasks obscures meaning |
| `doc_markdown` | Backtick-wrapping every code-looking term is disruptive |

When a lint fires and the rewrite would reduce clarity, suppress it on the function.
Prefer `#[expect(lint_name)]` over `#[allow(lint_name)]` — `#[expect]` errors if the lint
stops firing, preventing dead suppressions from accumulating. Migrate existing `#[allow]`
to `#[expect]` opportunistically when touching the code.

### Preferred Idioms (when clippy offers a choice)

- Explicit `match` over `.map().unwrap_or_else()`
- `let x && let y` chains for if-let guards
- `is_multiple_of(2)` over `% 2 == 0`
- `let-else` for early-return: `let Some(x) = expr else { return None; };`
- `missing_const_for_fn`: make helpers const, never suppress
- `unnecessary_literal_bound`: use `&'static str` for literal returns
- `cast_possible_truncation`: `#[allow]` with range-safety comment
- `bool_to_int_with_if`: use `usize::from(x.is_some())`
- `elidable_lifetime_names`: elide lifetimes when clippy suggests

## Cargo Metadata Rule

- Use `cargo metadata --format-version 1` for crate/dependency information.
- Use `--no-deps` by default; drop it only when the external dependency graph is needed.
- Never traverse `target/` for metadata.
- Before asking Jamie about crate structure, run `cargo metadata` first.

## Tool Security Posture Rule

- Design tool functionality from a security posture perspective.
- Design tool output from a security posture perspective.
- Implement features that evaluate trust, integrity, labeling, enforcement, or provenance.
- Do not implement functionality that is purely administrative unless instructed.

## Layered Separation Rule

- When building high-assurance tools, enforce layered separation.
- Separate data collection, storage, reading, and presentation.
- Do not combine write-path logic with read-path or display logic.
- Display layers (CLI, TUI, GUI) must depend only on read interfaces.
- Design layers to support different access control policies.
- Example: umrs-state writes JSON; a separate reader parses it; CLI/TUI/GUI display parsed state only.

## Presentation Tone Rule

- Maintain high-assurance functionality.
- Maintain integrity.
- Make tools and documentation approachable.
- Encourage engagement and exploration.
- Informal elements are permitted (e.g., ASCII art, visual effects).
- Informal elements must not reduce correctness.
- Informal elements must not reduce clarity.
- Informal elements must not reduce security posture.

## Source Comment Discipline Rule

- Preserve code readability.
- Do not place security control citations on every enum, field, or line item.
- Do not use excessive /// comments at fine granularity.
- Place security control mappings at module, struct, or major component level.
- Use parent documentation blocks to reference related items.
- Keep inline comments focused on behavior and intent.

## Control Flow Readability Rule

- Avoid long combinator or method chains when they reduce readability.
- Break complex chains into intermediate variables when intent is unclear.
- Prefer explicit control flow for multi-step logic.
- Do not refactor solely for compactness.
- Performance or security requirements may justify chaining.
- If chaining is required, keep transformations understandable and scoped.

## Citation Format Rule

- All NIST citations in Rust doc comments (`///` and `//!`) must use the canonical form: `NIST SP 800-53` (not `NIST 800-53`).
- NSA RTB citations use: `NSA RTB` followed by the specific principle (e.g., `NSA RTB RAIN`).
- NIST SSDF citations use: `NIST SP 800-218 SSDF` followed by the practice (e.g., `NIST SP 800-218 SSDF PW.4`).
- FIPS citations use: `FIPS 140-2` or `FIPS 140-3` (with the dash).
- CMMC citations use: `CMMC` followed by domain and level (e.g., `CMMC SC.L2-3.13.10`).
- Runtime output strings (e.g., `nist_controls` fields in catalog entries) may use abbreviated forms: `SP 800-53 CM-6` (drop "NIST"), `RTB RAIN` (drop "NSA"). Never abbreviate below the document number (e.g., never `CM-6` alone without the SP reference).
- CCE citations use the canonical form: `CCE-NNNNN-N` (uppercase, hyphen separator) followed by provenance `(RHEL 10 STIG)`.
- CCE always follows a NIST control citation — never cite CCE alone.
- When adding a CCE identifier to source code, include the STIG version and date in a comment on the same line or block. Example: `// CCE-89232-3 (RHEL 10 STIG, scap-security-guide 2026-03-17)`
- If a CCE's authoritative NIST control mapping conflicts with an existing UMRS citation, flag the conflict for review — do not silently overwrite. Both mappings may be defensible; the decision requires human judgment.

## Control Mapping Conventions

Standard control citations for common high-assurance patterns:

| Pattern | Controls |
|---|---|
| TPI (dual-path validation) | NIST SP 800-53 SI-7, NSA RTB |
| Fail-closed | NIST SP 800-53 SI-10, NSA RTB |
| Bounded reads / checked arithmetic | NIST SP 800-218 SSDF PW.4.1 |
| Error information discipline | NIST SP 800-53 SI-12 |
| Audit record integrity / append-only | NIST SP 800-53 AU-10 |
| Non-bypassable security checks | NSA RTB RAIN |
| TOCTOU fd-anchored I/O | NSA RTB, NIST SP 800-53 SI-7 |
| Component inventory (RPM/dpkg) | NIST SP 800-53 CM-8, SA-12 |
| FIPS mode gating | NIST SP 800-53 SC-13, CMMC L2 SC.3.177 |

## Tiered Annotation Expectations Rule

Public items need NIST SP 800-53, CMMC, or NSA RTB annotations in their doc comments, but the
requirement is tiered:

- **Modules** — always include relevant control references in the module-level doc comment
- **Security-critical types and functions** — require explicit control citations (e.g., `NIST SP 800-53 AC-4`, `NSA RTB RAIN`)
- **Simple accessors and display impls** — no annotation required if the parent type is already annotated.
  A "simple accessor" is a method that returns a field value or computed derivative without
  performing validation, I/O, or security decisions (e.g., `.user()`, `.as_u64()`, `Display`/`Debug` impls).
  If the method makes a trust decision, validates input, or crosses a boundary, it is security-critical
  regardless of its size.

Do not flag missing citations on trivial items. Flag only where a citation is actually required
by the above tiers.

## Module Documentation Checklist Rule

Every `.rs` source file under `src/` MUST have a `//!` module-level doc block. No exceptions.

The `//!` block must contain at minimum:

1. **Purpose** — one or two sentences explaining what the module does.
2. **Key exported types** — name the primary public types, traits, or functions.
3. **`## Compliance` section** — list applicable NIST SP 800-53, CMMC, or NSA RTB controls using canonical citation form.

If a module has no security-relevant controls (rare), the `## Compliance` section must still appear with a note explaining why (e.g., "This module provides internal formatting utilities with no direct security surface.").

### Exemplary templates

Use these existing modules as templates for new `//!` blocks:

- `umrs-selinux/src/secure_dirent.rs` — full design principles, pattern cross-references, structured compliance block with Rev 5 citations
- `umrs-selinux/src/posix/primitives.rs` — concise "Why typed primitives?" rationale, validation summary, complete compliance block

### Post-implementation self-check

Before considering any new file or module complete, verify:

- Does the file have a `//!` block?
- Does the `//!` block name the key exported types?
- Does it have a `## Compliance` section with canonical citations?
- Are NIST citations in `NIST SP 800-53` form (not `NIST 800-53`)?

This check applies to every new file, every refactored module, and every file touched during a review pass.

## Internal Reference Prohibition Rule

- Doc comments must NEVER contain references to internal review documents, finding numbers, or session-specific context (e.g., "Finding 1", "Finding 3", "RAG Finding 5", "security review session").
- All rationale in doc comments must be self-contained — a reader must understand the WHY without access to any external document.
- If a design decision was driven by a review finding, embed the technical rationale directly in the comment.

## Performance-Aware Construction Patterns

These patterns are construction-time habits, not post-hoc optimizations. Apply them on first
touch whenever the opportunity arises. Each pattern has zero cost to correctness and zero
compromise to security posture. If a pattern conflicts with security or correctness in a
specific context, correctness and security win — document why the pattern was not applied.

[PATTERN] **Static singleton via `OnceLock<T>` per variant, not `Mutex<HashMap>`**

When caching compiled resources (regex, parsed configs, lookup tables) with a fixed set of
keys known at compile time, declare one `static OnceLock<T>` per key variant rather than a
shared `Mutex<HashMap<K, T>>`. After the first initialization, reads are a single atomic load
with no lock acquisition, no hash lookup, and no clone. This pattern is only safe when T is
`Send + Sync` and the set of keys is bounded and enumerable at compile time. Do not apply it
to dynamically keyed caches — a `Mutex<HashMap>` remains correct there.

[PATTERN] **Pre-allocate `Vec` when element count is bounded**

When the number of elements in a collection is bounded and predictable at construction time
(e.g., evidence records across a fixed number of phases, category sets with a known maximum
cardinality), use `Vec::with_capacity(n)` rather than `Vec::new()`. This eliminates
reallocation cascades on push. Choose `n` to cover the typical case with modest headroom —
over-allocating wastes memory; under-allocating triggers the reallocation the pattern exists
to avoid. Never guess a capacity that could silently truncate security-relevant data; the
pre-allocated size is a hint, not a hard limit.

[PATTERN] **Byte-slice indexing for ASCII-only strings**

When processing strings that are structurally required to be ASCII-only — hex digests, numeric
identifiers, SELinux user/role identifiers — use `str::as_bytes()` and index the resulting
`&[u8]` slice directly. This avoids the heap allocation and four-times memory expansion of
`chars().collect::<Vec<char>>()`. The validation contract is non-negotiable: any code that
takes the byte-slice fast path must have already validated (or must validate inline) that all
bytes are in the expected ASCII range, rejecting invalid bytes before they influence any
security decision. A fast-path that skips validation is a vulnerability, not a pattern.

[PATTERN] **`splitn` iterator instead of `split().collect::<Vec>`**

When parsing a delimited string into a known, bounded number of fields, use `splitn(n, delim)`
and consume the iterator directly with `next()` calls rather than collecting into a
`Vec<&str>`. This avoids the heap allocation for the intermediate vector. The last element
produced by `splitn` preserves any remaining delimiters in the tail, which is useful for
fields like MLS sensitivity levels that may themselves contain the delimiter character (e.g.,
an MLS range `s0-s3:c0,c5` split on `:` for four colon-separated fields). When the field
count is not known at compile time, `split().collect()` remains the correct choice.

[PATTERN] **Fail-closed `Default` impls for evidence and audit types**

When a struct carries a boolean field that names a success or verification state — `parse_ok`,
`verified`, `integrity_confirmed`, or similar — its `Default` impl must initialize that field
to `false`. Callers must explicitly set it to `true` on a confirmed success path. This ensures
that any code path that forgets to set the flag produces a safe, fail-closed result rather than
a silent false positive. Conversely, security-critical fields that have no safe default — such
as `source_kind`, `trust_level`, or `classification` — must not appear in `Default` at all.
Force callers to supply them explicitly at construction. If a type cannot be safely default-
constructed, do not implement `Default` for it; implement a named constructor that requires
all security-critical arguments.
