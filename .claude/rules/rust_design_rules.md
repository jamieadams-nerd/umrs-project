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
- Runtime output strings (e.g., `nist_controls` fields in catalog entries) may use abbreviated forms for display compactness.
- CCE citations use the canonical form: `CCE-NNNNN-N` (uppercase, hyphen separator) followed by provenance `(RHEL 10 STIG)`.
- CCE always follows a NIST control citation — never cite CCE alone.
- When adding a CCE identifier to source code, include the STIG version and date in a comment on the same line or block. Example: `// CCE-89232-3 (RHEL 10 STIG, scap-security-guide 2026-03-17)`
- If a CCE's authoritative NIST control mapping conflicts with an existing UMRS citation, flag the conflict for review — do not silently overwrite. Both mappings may be defensible; the decision requires human judgment.

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
