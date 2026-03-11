# Plan: Pattern Library Taxonomy — Concepts, Patterns, Techniques, Process

**Date**: 2026-03-11
**Status**: Proposed — awaiting Jamie review
**Author**: tech-writer
**Affects**: `docs/modules/patterns/`, potential new `docs/modules/concepts/`

---

## Problem Statement

The current `patterns/` module mixes four distinct kinds of content under one label:

| Current page | What it actually is |
|---|---|
| pattern-tpi.adoc | Coding technique — a specific implementation recipe |
| pattern-toctou.adoc | Coding technique |
| pattern-provenance.adoc | Coding technique (implements a broader concept) |
| pattern-fail-closed.adoc | Design pattern — an architectural decision |
| pattern-loud-failure.adoc | Design pattern |
| pattern-non-bypassability.adoc | Design pattern (with coding technique aspects) |
| pattern-error-discipline.adoc | Design pattern |
| pattern-sec.adoc | Design pattern |
| pattern-secure-arithmetic.adoc | Coding technique |
| pattern-bounds-safe.adoc | Coding technique |
| pattern-zeroize.adoc | Coding technique |
| pattern-constant-time.adoc | Coding technique |
| pattern-supply-chain.adoc | Process discipline |

All of these answer "how do I implement X?" They do not answer "what is X and why does it
exist?" That second question — explaining the underlying security concept — is missing from
the library entirely and will need to be added as the project matures.

As the library grows (privilege separation, provenance as a broader concept, reference monitor
theory, information flow control, Bell-LaPadula, least privilege), the absence of a
clear taxonomy will make the library harder to navigate, harder to assign to writers, and
harder to convert into blog posts targeting different audiences.

---

## Proposed Taxonomy

Four kinds of content. Every page belongs to exactly one.

### 1. Security Concept

A foundational security principle. Technology-independent. Answers "what is this, why does
it matter, and what class of threats does it address?"

Audience: developers new to high-assurance design; security auditors validating design
rationale; anyone asking "why does UMRS do things this way?"

Blog voice: explainer post. Accessible to a broad technical audience.

Examples of concepts we will eventually need:
- Reference Monitor (completeness, non-bypassability, tamper-resistance)
- Privilege Separation
- Provenance (broader: chain of custody for trusted data; not just the fstatfs check)
- Information Flow Control / Lattice Model (the theory behind MLS)
- Least Privilege
- Compartmentalization
- Non-interference (formal property; advanced)

### 2. Design Pattern

An architectural decision that applies a concept to a specific component boundary. Says
"structure your component this way so the concept is enforced." Language-agnostic, but
illustrated with Rust.

Audience: developers designing a new module or crate; architects reviewing a proposal.

Blog voice: technical deep-dive. Explains the pattern and the threat it prevents.

Current examples:
- Fail-Closed (applies: Reference Monitor completeness)
- Loud Failure (applies: Audit completeness)
- Non-Bypassability / RAIN (applies: Reference Monitor; bridges concept and technique)
- Error Information Discipline (applies: Least Privilege of information)
- Sealed Evidence Cache / SEC (applies: Integrity + Provenance)

### 3. Coding Technique

A language-specific implementation recipe that realizes a pattern. Says "write exactly this
code." Rust-specific. Includes the crate, the trait, the call pattern.

Audience: developers implementing a feature; reviewers checking correctness.

Blog voice: code walkthrough. Rust-specific. Shows the before and after.

Current examples:
- Two-Path Independence (TPI) — nom + FromStr, fail-closed on disagreement
- TOCTOU Safety — fd-anchored I/O via rustix
- Provenance Verification — ProcfsText + fstatfs magic check
- Secure Arithmetic — checked_*, saturating_*
- Bounds-Safe Indexing — .get(i) over [i]
- Zeroize Sensitive Data — zeroize crate, ZeroizeOnDrop
- Constant-Time Comparison — subtle crate, ConstantTimeEq

### 4. Process Discipline

A development workflow practice that is not a code artifact. Applied at the workspace or
release level, not within individual modules.

Current examples:
- Supply Chain Hygiene — cargo-audit before each new dependency

---

## Overlap Is Expected

Some pages span two kinds. Non-Bypassability is both a concept (the RAIN property) and a
design pattern (private constructors, newtype wrappers). Provenance is both a concept
(chain of custody) and a coding technique (the fstatfs check).

The rule: each page belongs to its primary kind. The See Also section and inline links
connect the two levels. A concept page for "Provenance" would link to the existing
`pattern-provenance.adoc` coding technique as its primary UMRS implementation. They are
complementary, not duplicates.

---

## Proposed Structure

### Phase 1 — Label Without Moving (do now)

Add a kind label to each existing page. Minimal change. No files move. No new module.

**Implementation:**
1. Add an AsciiDoc role badge below the title on each page:
   ```asciidoc
   [.pattern-kind]#Coding Technique#
   ```
   Or use a simple NOTE admonition on pages where the distinction is non-obvious.

2. Update the Pattern Reference Table in `index.adoc` to add a "Kind" column:
   ```asciidoc
   [cols="1,2,1,1,1", options="header"]
   | Pattern | What it does | Kind | Primary control | Status
   ```

3. Rename the "Patterns by Group" section to "By Kind" with sub-groups:
   - Design Patterns
   - Coding Techniques
   - Process Disciplines
   - Security Concepts (initially empty; placeholder for Phase 2)

**Estimated effort**: 1–2 hours. No reader-visible URL changes.

---

### Phase 2 — Add Concept Pages (when first concept is ready)

Add `concept-*.adoc` pages to the existing `patterns/` module using a naming convention.

```
docs/modules/patterns/pages/
  index.adoc
  concept-reference-monitor.adoc   ← new
  concept-privilege-separation.adoc ← new
  concept-provenance.adoc          ← new
  pattern-*.adoc                   ← unchanged
```

Update `nav.adoc` to add a "Security Concepts" group above the pattern groups.

Update `index.adoc` to add a "Security Concepts" table above the pattern table.

**When to start Phase 2**: When we have at least two concept pages ready to write. Writing
one concept page in isolation is not worth the structural overhead.

**Suggested first two concept pages** (because they are the theoretical foundation for the
most existing patterns):
1. `concept-reference-monitor.adoc` — basis for RAIN, Non-Bypassability, Fail-Closed
2. `concept-provenance.adoc` — basis for Provenance Verification, SEC, evidence chains

---

### Phase 3 — Separate Module (when concept library reaches ~6 pages)

If the concept library grows large enough that it warrants its own navigation tree and
cross-promotion (blog series, standalone section in the site header), migrate to a separate
Antora module.

```
docs/modules/patterns/   ← design patterns + coding techniques + process
docs/modules/concepts/   ← security concepts only
```

Register both in `antora.yml`. Update all cross-module xrefs.

**Trigger for Phase 3**: Six or more concept pages, OR a use case where the concepts section
needs to appear independently in site navigation (separate nav bar entry, separate sitemap
category, etc.).

**This is deferred work.** Do not migrate until the trigger condition is met. Premature
module splitting adds xref maintenance overhead with no reader benefit.

---

## Blog Post Mapping

The taxonomy maps cleanly to two editorial categories:

| Kind | Blog category | Target audience | Tone |
|---|---|---|---|
| Security Concept | "Understanding X" | Broad technical; security-curious developers | Accessible; no code required |
| Design Pattern | "Building X the high-assurance way" | Senior developers; architects | Technical; architecture diagrams |
| Coding Technique | "Implementing X safely in Rust" | Rust developers | Code-first; before/after |
| Process Discipline | "Our X workflow" | DevSecOps; maintainers | Process-focused |

The SEC page (`pattern-sec.adoc`) is already blog-ready as a Design Pattern post. The
`Summary` section in each file is the post's lede paragraph.

---

## Immediate Next Steps (Phase 1 only)

In priority order:

1. **Add Kind column to the Pattern Reference Table** in `index.adoc`
2. **Rename "Patterns by Group" to "By Kind"** and reorganize the group list
3. **Add kind badge to each pattern page** (role or NOTE admonition)
4. **Write two concept page stubs**: `concept-reference-monitor.adoc` and
   `concept-provenance.adoc` — title, description, one-paragraph overview, See Also only.
   Full content deferred until Jamie provides the source material or approves expansion.

Phase 2 and Phase 3 are triggered by content volume, not by calendar.

---

## Open Questions for Jamie

1. **Kind label format**: Role badge (`[.pattern-kind]#Coding Technique#`) requires CSS
   in the Antora UI theme. A NOTE admonition is theme-independent and works today.
   Which is preferred?

2. **First concept pages**: Should the tech-writer draft `concept-reference-monitor.adoc`
   and `concept-provenance.adoc` now as stubs, or wait until there is source material
   (a design doc, a whitepaper, or a "here's what I want to say" note from Jamie)?

3. **Blog conversion**: Is this a concrete near-term goal, or a "nice to have someday"?
   If concrete, we should align the taxonomy work with whatever publishing platform will
   be used, so the AsciiDoc → blog conversion is low-friction.

4. **Non-Bypassability**: This page spans Design Pattern and Security Concept. Leave it
   as a design pattern and let a future `concept-reference-monitor.adoc` absorb the
   conceptual explanation? Or split the page now?
