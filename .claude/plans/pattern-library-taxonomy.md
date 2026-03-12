# Plan: Pattern Library Taxonomy — Implementation Patterns with Provenance Tags

**Date**: 2026-03-11
**Status**: Approved — Phase 1 ready for implementation
**Reviewed by**: senior-tech-writer (2026-03-12), Jamie (2026-03-12)
**Author**: tech-writer
**Affects**: `docs/modules/patterns/`

---

## Decisions (locked 2026-03-12)

1. **Single kind: "Implementation Pattern"** — collapse "Design Pattern" and "Coding Technique"
   into one kind. Use a sub-label for scope: *architectural* or *technique*.

2. **High-assurance provenance tag** — patterns derived from RAIN/RTB, NIST, or other formal
   security frameworks get a visible badge so developers know they are picking from a
   security-mandated pool, not just a general engineering best-practice pool.
   Format: NOTE admonition at top of page, e.g.:
   ```asciidoc
   NOTE: *High-Assurance Pattern* — derived from NSA RTB RAIN (Non-Bypassability).
   ```

3. **Kind label format**: NOTE admonitions (theme-independent, works today). No CSS role badges.

4. **Two-zone page template**: each pattern page uses two clearly marked zones:
   - Zone 1: "Why This Pattern Exists" — concept/explanation (can be read independently)
   - Zone 2: "Implementation" — reference/procedure (consulted during work)

5. **Concept basis column**: add a "Concept basis" column to the index table linking each
   pattern to its `security-concepts/` page where applicable. Empty cells = visible gap list.

6. **Phase 2 becomes cross-linking audit** — before writing any new concept pages, audit
   existing `security-concepts/` pages (5 exist) against patterns for coverage.

7. **`security-concepts/` module**: deferred. Jamie will revisit when a more complete list of
   topics is ready. Not to be used as a catch-all.

8. **Phase 3 (module split)**: deferred indefinitely until `security-concepts/` scope is clear.

9. **"Process Discipline"**: kept as a kind but acknowledged as having only one member
   (supply chain hygiene). Will grow naturally or be folded later.

---

## Problem Statement

The current `patterns/` module mixes content types under one label without indicating which
patterns are general engineering practices and which are mandated by formal security frameworks
(RAIN, RTB, NIST). As the library grows, developers need to distinguish between "good idea"
and "security requirement."

---

## Taxonomy

Three kinds. Every page belongs to exactly one.

### 1. Implementation Pattern

A pattern applied in the UMRS codebase — either an architectural decision or a language-specific
coding technique. Sub-labeled as *architectural* or *technique* for navigation grouping.

Patterns with formal security provenance (RAIN, RTB, NIST) carry a **High-Assurance Pattern**
badge. This tells developers: "this is not optional engineering taste — it traces to a specific
security control or principle."

**Sub-label: Architectural** — structural decisions at module/crate boundaries:
- Fail-Closed
- Loud Failure
- Non-Bypassability / RAIN
- Error Information Discipline
- Sealed Evidence Cache / SEC

**Sub-label: Technique** — language-specific implementation recipes:
- Two-Path Independence (TPI)
- TOCTOU Safety
- Provenance Verification
- Secure Arithmetic
- Bounds-Safe Indexing
- Zeroize Sensitive Data
- Constant-Time Comparison

### 2. Process Discipline

A development workflow practice at the workspace or release level, not within individual modules.

Current members:
- Supply Chain Hygiene

### 3. Security Concept *(deferred — not in patterns/ module)*

Foundational security principles live in `docs/modules/security-concepts/`. The patterns module
links to them via the "Concept basis" column in the index table. New concept pages are not
created in `patterns/` — they go to `security-concepts/` when Jamie is ready.

---

## Overlap Is Expected

Some pages span architectural and technique. Non-Bypassability is both an architectural
principle (RAIN) and a set of coding techniques (private constructors, newtype wrappers).

The rule: each page belongs to its primary sub-label. The See Also section connects levels.

---

## Phase 1 — Label, Tag, and Cross-Link (do now)

No files move. No new module. No URL changes.

### 1a. Update the Pattern Reference Table in `index.adoc`

Add "Sub-group" and "Concept basis" columns:

```asciidoc
[cols="2,3,1,1,1,1", options="header"]
| Pattern | What it does | Sub-group | Provenance | Concept basis | Status
```

"Provenance" values: `HA` (high-assurance — RAIN/RTB/NIST derived) or blank.
"Concept basis" values: xref link to `security-concepts/` page or blank.

### 1b. Add provenance badge to high-assurance pattern pages

At the top of each high-assurance pattern page, below the title:

```asciidoc
NOTE: *High-Assurance Pattern* — derived from <source> (<specific control/principle>).
```

Examples:
- TPI: `NIST 800-53 SI-10 (Input Validation)`
- TOCTOU Safety: `NSA RTB RAIN (Non-Bypassability)`
- Provenance Verification: `NIST 800-53 SI-7 (Software Integrity)`
- Non-Bypassability: `NSA RTB RAIN`
- Fail-Closed: `NSA RTB (Fail Secure)`
- Zeroize: `NIST 800-53 SC-28 (Protection of Information at Rest)`

Patterns without formal security provenance (e.g., Bounds-Safe Indexing) get no badge —
they are good engineering practice, not security-mandated.

### 1c. Add two-zone structure markers to pattern pages

Each pattern page should use this template structure:

```asciidoc
== Why This Pattern Exists
// Zone 1: concept content — threat, consequence, control basis
// Can be read independently by auditors or newcomers

== The Pattern
// Zone 2: implementation content — invariant, rule, code recipe
// Consulted during active development work

== In the UMRS Codebase
// Zone 2 continued: actual types, traits, file paths

== When to Apply
// Trigger conditions from Architectural Review Triggers table
```

### 1d. Reorganize nav.adoc grouping

Rename "Patterns by Group" to sub-groups:

```
* Architectural Patterns
** Fail-Closed
** Loud Failure
** ...
* Coding Techniques
** TPI
** TOCTOU Safety
** ...
* Process
** Supply Chain Hygiene
```

### 1e. Cross-linking pass

For each pattern, check `security-concepts/` for a matching concept page and add the link
to the "Concept basis" column. Current `security-concepts/` pages:
- `reference-monitor.adoc`
- `integrity-and-provenance.adoc`
- `rtb-vnssa.adoc`
- `security-model.adoc`
- `truth-concepts.adoc`

---

## Phase 2 — Cross-Linking Audit (when Phase 1 is complete)

Before writing any new content:

1. Audit the 5 existing `security-concepts/` pages against all pattern pages.
2. Identify which patterns have matching concept coverage and which do not.
3. Populate all "Concept basis" cells that have matches.
4. Report gaps — concept content genuinely missing from any existing page.

Only then decide whether new concept pages are needed and where they belong.

---

## Phase 3 — Cryptography Section (proposed — awaiting review)

**Date added**: 2026-03-12
**Status**: Draft — review later

A new major section in the pattern library dedicated to cryptography. This is not a single
pattern page — it is a multi-page section covering the full cryptographic posture of UMRS.

### Scope

| Area | Content |
|---|---|
| FIPS-compliant primitives | Authoritative list of FIPS 140-2/140-3 validated algorithms and their status (approved, deprecated, transitioning) |
| Purpose mapping | What each primitive is used for — HMAC for integrity sealing, AES-GCM for data-at-rest, key derivation, digital signatures, etc. |
| Code examples | Rust code showing correct usage of each primitive in a FIPS-compliant context — crate selection, initialization, error handling |
| UMRS cross-references | Where cryptography is used in UMRS source (e.g., SEC sealing key, future dm-crypt integration) and in the broader US system context |
| Post-quantum cryptography | PQC algorithms (ML-KEM, ML-DSA, SLH-DSA per NIST FIPS 203/204/205), migration timeline, hybrid approaches, impact on UMRS |

### Proposed page structure

```
docs/modules/patterns/pages/
  crypto-overview.adoc           ← Section landing page: why crypto matters in UMRS, FIPS mandate
  crypto-fips-approved.adoc      ← Reference table: validated algorithms, modes, key sizes, status
  crypto-primitives-usage.adoc   ← What each primitive does, when to use it, Rust code examples
  crypto-in-umrs.adoc            ← Cross-reference: where UMRS uses crypto, which controls apply
  crypto-post-quantum.adoc       ← PQC landscape, NIST standards, migration strategy, hybrid schemes
```

### Provenance

All pages in this section carry the High-Assurance Pattern badge:
- `NIST FIPS 140-2/140-3` (cryptographic module validation)
- `NIST SP 800-175B` (guideline for using crypto standards)
- `NIST SP 800-131A` (transitioning crypto algorithms)
- `NIST 800-53 SC-12, SC-13` (cryptographic key management, cryptographic protection)

### Dependencies

- RAG `nist` collection already contains FIPS 140-2/140-3 and SP 800-175B
- Researcher may need to acquire NIST SP 800-131A Rev 2 (Transitioning Crypto Algorithms)
  and NIST FIPS 203/204/205 (PQC standards) for the post-quantum section
- rust-developer input needed for code examples and crate recommendations
- security-engineer input needed for FIPS mode validation on RHEL10

### Open questions (for later review)

1. Should PQC get its own standalone page or be a subsection of the overview?
2. Which Rust crates are FIPS-validated or wrap FIPS-validated libraries (e.g., `aws-lc-rs`,
   `openssl` with FIPS provider)? Needs rust-developer + security-engineer assessment.
3. Should this section also cover key management lifecycle (generation, storage, rotation,
   destruction) or is that a separate operations concern?
4. Scope of "US system context" — how deep do we go into system-level crypto (dm-crypt,
   LUKS, IMA signatures) vs. application-level crypto?

---

## Phase 4 — Deferred

Module split and `security-concepts/` expansion deferred until Jamie has a complete
topic list and clear scope for that module. No action until then.

---

## Open Questions (resolved)

| # | Question | Decision |
|---|---|---|
| 1 | Kind label format? | NOTE admonitions — theme-independent |
| 2 | First concept pages? | Deferred — audit existing `security-concepts/` first |
| 3 | Blog conversion? | Not a near-term priority; taxonomy supports it when ready |
| 4 | Non-Bypassability split? | Leave as Implementation Pattern (architectural); future concept page links to it |
| 5 | `security-concepts/` coverage? | Deferred — Jamie will revisit with a fuller list |
