# Developer Guide Review — Sage (Outreach Lens)
## Date: 2026-03-20
## Reviewer: Sage (outreach-corpus-trained, Diataxis + developer trust + progressive disclosure lenses)

---

## Executive Summary

The UMRS developer guide contains genuinely excellent technical writing in places —
the posture probe developer guide and compliance annotations guide in particular read
like they were written by someone who has done this work and wants others to succeed
at it. The pattern documentation philosophy ("what you are about to do, not what the
compliance framework says") is exactly the right instinct. However, the guide as a
whole has a structural inversion problem: it reads like a reference corpus organized
by module shape, not a guide organized by what a developer wants to accomplish. There
is no onboarding path. A new engineer landing on the index page cannot find "where do
I start?" The first-hour experience is absent. The guide is excellent for developers
already inside the project; it does not yet serve the developer deciding whether to
enter it.

---

## 1. Structure & Flow Assessment

### Current Navigation Order (from nav.adoc)

```
Design Rationale (3 pages cross-linked to architecture module)
High-Assurance Patterns (5 pages)
Platform Internals (4 pages)
Internationalization (1 page)
Build & Workspace (3 pages)
```

### The Core Structural Problem

The navigation puts Design Rationale first. That is an explanation-type document.
It is valuable — but it is not where a developer should start.

Following the progressive disclosure model (Cloudflare, Stripe, HashiCorp all
do this), the correct first experience is: "run this, see that, understand why."
The current guide skips the first two steps entirely and lands on "understand why."

By Diataxis framing, the guide is almost entirely **explanations** and **references**
with zero **tutorials** and very few **how-tos**. The procedure sections that exist
(e.g., "How to Add a New Indicator") are buried inside reference-style documents
rather than surfaced as first-class navigable pages.

### Recommended Structural Reorder

A developer-experience-first order would be:

1. **Getting Started** (missing — needs to be written)
   - "What UMRS gives you in 5 minutes"
   - Clone, build, run the example, understand what you saw
2. **Your First Contribution** (missing how-to)
   - Add a new posture indicator (the existing how-to is excellent; make it a
     first-class nav entry, not buried in a 2000-line reference page)
3. **Why We Build This Way** (current Design Rationale — good content, right place
   as step 3 not step 1)
4. **High-Assurance Patterns** (current content, right place)
5. **Component Deep Dives** (posture probe, OS detection — existing content is strong)
6. **Reference: Build Tooling, i18n, Cargo Notes, Git Signing** (current content,
   right place as reference material at the end)

---

## 2. Diataxis Analysis

| Page | Current Type | Should Be | Notes |
|---|---|---|---|
| `index.adoc` | Explanation + reference | Portal + tutorial entry point | Missing "start here" path; the crate table is good but orphaned |
| `umrs-prog-lang.adoc` | Explanation | Explanation | Correct type, wrong position in nav (should be further in) |
| `high-assurance-patterns.adoc` | Explanation + reference | Explanation | Strong content; philosophy section excellent |
| `compliance-annotations.adoc` | Reference + how-to | Reference | Well-written; CCE section is authoritative |
| `rust-must-use-contract.adoc` | Reference + explanation | Reference | Good; auditor checklist at end is a nice how-to sub-section |
| `nom-parser.adoc` | Explanation | Explanation | Good; the TPI example is the best part |
| `os-detection-deep-dive.adoc` | Reference (deep) | Reference | Excellent technical content; well-cited |
| `posture-probe.adoc` | How-to + reference | How-to + reference | "How to Add a New Indicator" should be extracted to its own nav entry |
| `posture-probe-internals.adoc` | Reference | Reference | Correct type; good companion to posture-probe |
| `update-checklists.adoc` | How-to | How-to | Best procedural writing in the guide; STE-compliant; well-structured |
| `i18n.adoc` | How-to | How-to | Correct type; step-by-step clear |
| `build-tooling.adoc` | Reference (impure) | How-to + reference | Currently contains conversational xtask setup discussion mixed with reference content (see notes below) |
| `cargo-notes.adoc` | Reference (raw) | Reference | Content is raw Cargo documentation copy-paste, not UMRS-specific guidance |
| `git-commit-signing.adoc` | Explanation | Explanation | Correct type; the NIST mappings table is thorough |

### Critical Diataxis Violation: build-tooling.adoc

The build-tooling page currently contains what appears to be an LLM-generated
xtask setup walkthrough in conversational tone, with phrases like "Got it. Here's
a step-by-step..." and "If you want, tell me which 2-3 real chores you want first."
This is session context that was committed by mistake, not finished documentation.
It is the most urgent correctness issue in the guide.

### Critical Diataxis Violation: cargo-notes.adoc

This page contains raw Cargo documentation extracted from the Cargo Book, not
UMRS-specific guidance. It does not explain UMRS workspace decisions, dependency
hygiene policy, or supply chain approach. It reads like a Cargo primer pasted in
as a placeholder. Either replace it with UMRS-specific content or remove it.

---

## 3. Getting Started Gap Analysis

### What's Missing for the "First 10 Minutes" Experience

There is **no tutorial**. Zero. A developer who finds this guide has no path from
"I cloned the repo" to "I understand what I built and why."

The minimum viable Getting Started section needs:

1. **Prerequisites** — what the developer needs installed (Rust toolchain version,
   RHEL 10 or compatible, SELinux enabled)
2. **First build** — `cargo xtask test` from `components/rusty-gadgets/`; expected
   output; what to do if it fails
3. **First run** — `cargo run -p umrs-platform --example posture_demo`; what you
   should see; what it means
4. **First read** — link to one short piece of code (the posture catalog would be
   ideal) with annotations explaining what you're looking at
5. **Where to go next** — three branching paths: "I want to add a posture
   indicator," "I want to understand the trust model," "I want to write a new tool"

The posture probe developer guide has excellent "Usage Examples" sections with
real code. Those could anchor a tutorial if restructured as one.

### What the index.adoc Does Right

The crate architecture table and Mermaid diagram are genuinely good. A developer
can understand the layering in about 90 seconds. The "High-Assurance System
Interrogation" section explains the philosophical bet the project makes in a way
that earns buy-in. These are the strongest parts of the index page.

What's missing: after explaining all of this, the index page gives the developer
nowhere to go that feels like a next step. "What's in This Guide" is a table that
links to the same pages they can already see in the nav.

---

## 4. Excitement Factor

### What Works

- The nom-parser page is delightful. The "chainsaw vs. scalpel" analogy for
  `split(':')` vs. nom is the kind of writing that makes engineers want to use
  the tool. The null-byte injection example makes TPI feel viscerally real.
- The posture probe's "Dual-Check Model" framing — live value vs. configured value —
  immediately communicates the operational value. An operator reading this
  understands what they get that they don't have today.
- The core_pattern TPI classification section is exactly the kind of "why would
  you do it this way?" content that builds trust. The example value table showing
  why `|usr/bin/handler` is Invalid when `|` passes the byte check is superb.
- The philosophy quote in high-assurance-patterns.adoc ("Make the right thing easy
  to do and the wrong thing hard to do") is the right opening. It frames everything
  that follows.

### What Falls Flat

- `umrs-prog-lang.adoc` reads like a committee document defending a technology
  choice to skeptics. The language is hedged and comparative ("Rust does not
  pretend unsafe code does not exist"). The choice was made; a developer in this
  guide has already found the Rust codebase. Lead with what Rust makes possible
  here, not a defense of the choice.
- The git-commit-signing page is 90% NIST cross-reference table and 10%
  "how to actually sign a commit." A developer who lands here probably wants to
  know how to configure GPG or SSH signing in Git, not the complete provenance
  of NIST 800-161. The control mapping is correct and useful — but it is
  proportionally overwhelming relative to the procedural content that should be
  its purpose.
- The index page "Core Assumption" section uses passive framing ("system state
  must be validated") when it could use active, concrete framing ("when you read
  a file with UMRS, you get back a proof, not a string"). The insight is right;
  the framing undersells it.
- The CCE citation section in compliance-annotations.adoc is extremely thorough
  but reads like a legal standard. It would benefit from a one-paragraph
  orientation at the top explaining what CCE is and why any developer would care,
  before the format rules.

---

## 5. Example Coverage

### What Exists

- posture-probe.adoc: five complete, runnable code examples — excellent
- compliance-annotations.adoc: annotated code blocks at module, type, and function
  levels — thorough and correct
- nom-parser.adoc: one focused TPI example — effective
- rust-must-use-contract.adoc: correct/non-compliant examples — the right pattern
- i18n.adoc: step-by-step code blocks — clear
- high-assurance-patterns.adoc: extensive examples for each pattern — strong

### What's Missing

1. **An end-to-end worked example.** The OS detection deep-dive is the closest
   thing, but it is organized as a reference, not a walkthrough. There is no page
   that starts with: "here is a real problem, here is how UMRS solves it, here is
   the code you would write."
2. **A "wrong way, right way" example for trust boundaries.** The compliance
   annotations guide shows how to annotate correctly; it does not show the common
   mistake pattern that leads developers to under-annotate or annotate the wrong tier.
3. **A complete minimal consumer crate example.** A developer building a tool on
   top of UMRS has no worked example of "this is what a crate that uses
   `umrs-platform` looks like." What goes in `Cargo.toml`? What does `main.rs`
   look like? What is the first call you make?
4. **Error handling examples.** `SecurityContext::parse()` is mentioned as the
   canonical input validation example, but there is no example showing what to do
   with the `Err` case — how to log it with discipline, how to surface it to the
   operator.

---

## 6. Use Case Scenarios Needed

The guide is organized by module/component, not by what a developer wants to
accomplish. The following "I want to do X" paths should exist as navigable routes:

1. **"I want to add a new posture indicator"** — the procedure exists and is good
   (posture-probe.adoc Steps 1-6); it needs to be a first-class nav entry
2. **"I want to write a new CLI tool that reads posture state"** — missing entirely
3. **"I want to understand how the trust model works before I write code"** —
   currently scattered across index, patterns, and OS detection; no single path
4. **"I want to add a new high-assurance pattern"** — not addressed
5. **"I want to write a security annotation for a new type"** — compliance-annotations
   covers the format; does not cover how to identify which control applies when
   you are unsure
6. **"I want to understand why my PR was flagged for a missing #[must_use]"** —
   rust-must-use-contract.adoc exists but is not surfaced as a "why did I get this
   feedback?" entry point
7. **"I want to add i18n to a new CLI tool"** — i18n.adoc covers this well; just
   needs a link from the "writing a new tool" path that does not exist yet

---

## 7. Cognitive Load Issues

### Concepts Introduced Without Sufficient Runway

1. **TPI (Two-Path Independence)** appears in the index page's "High-Assurance
   System Interrogation" section without a definition. A developer reads
   "TPI parsing" and has no idea what this means until they reach nom-parser.adoc
   or find the pattern library. The index should define TPI in one sentence, or
   not use the acronym until it is introduced.
2. **"Trust ladder" / "T0 through Tx"** is referenced in the index as if the
   developer knows what trust levels mean. No definition. No link to where this
   is explained.
3. **`SecureReader`**, **`StaticSource`**, **`fstatfs` provenance check** appear
   in posture-probe.adoc without prior context. The CAUTION box ("A direct
   `File::open` bypasses the `fstatfs` provenance check") assumes the developer
   already understands what provenance verification means and why it matters.
4. **"Fail closed"** is used throughout before the pattern is formally defined.
   The posture probe developer guide uses it in CAUTION boxes before the pattern
   library is introduced.
5. **MLS vs. targeted policy** is used without differentiation in the index.
   A developer who does not know SELinux will not understand which mode the
   current code operates in or how that affects what they can implement.

### Missing Prerequisites

The guide assumes the developer:
- Understands SELinux basics (contexts, labels, enforcement modes)
- Knows what MLS means
- Has worked with procfs/sysfs before
- Understands the concept of a reference monitor

None of these are obscure — but they are not universal. A mid-level Rust developer
with strong memory safety instincts but no SELinux background will feel lost within
the first three pages. A brief "What you should already know" section on the index
page, with links to the security-concepts module for the unfamiliar, would solve this.

---

## 8. Missing How-Tos

Procedural content that does not exist but should:

1. **How to set up a development environment** — what to install, how to verify
   SELinux is active, how to confirm the build works
2. **How to write a new CLI tool using UMRS platform** — from `cargo new` to
   first `PostureSnapshot::collect()` call
3. **How to identify which NIST control applies to a new type** — compliance-
   annotations.adoc gives the format; it does not give a decision procedure
4. **How to run and interpret the posture demo** — the demo exists as an example;
   there is no page that says "here is what you should see, here is what each
   line means"
5. **How to add a new high-assurance pattern to the library** — the pattern
   library is referenced throughout but there is no guide to contributing a
   new pattern
6. **How to debug a failed provenance check** — what to look for when
   `StaticSource::read()` returns an error; what the error message means; what
   to check next
7. **How to write a security-annotated test** — the test structure rules prohibit
   inline tests and require `tests/` placement; there is no guide to writing tests
   that satisfy the compliance annotation expectations

---

## 9. Deeper Explanation Needs

Concepts that need more context before a developer encounters them in code:

1. **The reference monitor concept** — mentioned in index but not explained.
   What is it? Why does UMRS implement one? What does it mean for the code a
   developer writes? The security-concepts module has this; the devel guide needs
   a link and a one-paragraph orientation.
2. **MLS lattice and dominance** — CategorySet and MlsLevel appear in examples
   without any explanation of what an MLS lattice is or why dominance math matters
   for CUI protection. Even a three-sentence summary would make the code
   examples comprehensible.
3. **The evidence chain concept** — the index explains evidence chains well, but
   the connection to developer practice is missing. When a developer calls
   `PostureSnapshot::collect()`, what happens to the evidence? Where does it go?
   Who can read it?
4. **Why `const` in the catalog matters for auditors** — the guide explains that
   catalog constants are compiler-verified, but does not explain what an auditor
   does with that fact. The auditor can look at one file and see the complete
   monitored signal set. That is a big deal. Say it.
5. **The BLS bootloader entry model** — `bootcmdline.rs` reads the BLS entry to
   find the configured kernel command line. This is not obvious. Why not just
   read `/proc/cmdline`? What does it mean that configured and live cmdlines can
   disagree? This needs a two-paragraph explanation before the developer
   encounters `KernelCmdline`-class indicators.

---

## 10. Trust & Voice Assessment

### Does It Sound Like an Engineer or a Compliance Officer?

Mostly engineer, with some compliance officer leakage.

**Sounds like an engineer:**
- The nom-parser page throughout. Concrete, specific, honest about tradeoffs.
- The posture probe "Two Audiences" section. "Novice callers need a summary in
  three lines" is exactly the kind of observation that earns trust.
- The core_pattern TPI classification with example values. No hedging, just the
  explanation of why the edge cases exist and how the gate catches them.
- The update-checklists page. Clear, numbered, STE-compliant. This is how
  procedural writing should read.

**Sounds like a compliance officer:**
- The git-commit-signing page. It is a NIST control mapping document, not a
  developer guide page. An engineer reading this learns that signing is good
  and complies with many standards. They do not learn how to sign a commit.
- The CCE section of compliance-annotations.adoc is thorough to the point of
  inaccessibility. The format tables are correct. The introductory motivation
  ("why would I care about a CCE identifier?") is missing.
- The umrs-prog-lang.adoc language defense. It reads like it was written to
  convince a skeptical procurement officer, not to motivate a developer who
  has already found the Rust codebase.

**Voice inconsistency between pages:**
The high-assurance-patterns page and posture-probe page are both strong and consistent.
Build-tooling has a completely different voice (session transcript). Cargo-notes has
yet another voice (raw documentation paste). Git-commit-signing is formal compliance
writing. The guide does not yet have a unified register.

---

## 11. Priority Recommendations

Ranked by impact on developer experience:

### Priority 1 — Fix build-tooling.adoc (Correctness)

The current page contains what appears to be a chat session transcript with
"Got it. Here's a step-by-step..." text. This is the most urgent fix because
it represents incorrect content, not missing content. Replace with:
- What `cargo xtask` is and why it exists
- The command table from CLAUDE.md (fmt, clippy, test, the single-crate and
  integration test invocations)
- The Clippy policy explanation
- Expected output for a clean build

### Priority 2 — Replace or substantially rewrite cargo-notes.adoc (Correctness)

Current content is raw Cargo Book documentation, not UMRS guidance.
Replace with UMRS-specific content:
- Why the workspace is structured as it is
- Dependency hygiene and supply chain policy (`cargo audit`, `cargo-geiger`)
- Which crates are safe to add without review, which require explicit approval
- How to check for upstream vulnerability disclosures

### Priority 3 — Write a Getting Started page (Impact)

This is the highest-impact new content the guide is missing. Even 600 words with
four code blocks and a "where to go next" section would transform the first-hour
experience. Anchor it on running `posture_demo` and understanding what the output
means. Make it the first item in the nav, above Design Rationale.

### Priority 4 — Surface "How to Add a New Indicator" as a first-class nav entry (Flow)

The existing procedure in posture-probe.adoc (Steps 1-6) is genuinely good.
Extract it to a dedicated page or add a direct nav entry that links directly to
that section. A developer looking to contribute should not have to find it buried
inside a 2000-line reference page. This is the "good first contribution" path and
it should be visible.

### Priority 5 — Add a "What You Should Already Know" section to index.adoc (Cognitive Load)

Three bullets, each with a link:
- Basic SELinux familiarity (contexts, enforcement modes) — link to security-concepts
- Understanding of Linux kernel pseudo-filesystems (procfs, sysfs) — can be brief
- Rust Result/Option patterns — link to the language rationale page

This tells developers where they are before they start reading, and gives
unfamiliar developers a path rather than a wall.

---

## Appendix: Pages Reviewed

All 14 pages under `docs/modules/devel/pages/` and `docs/modules/devel/nav.adoc`.

- `index.adoc`
- `umrs-prog-lang.adoc`
- `high-assurance-patterns.adoc` (full content via persisted output)
- `compliance-annotations.adoc`
- `rust-must-use-contract.adoc`
- `nom-parser.adoc`
- `os-detection-deep-dive.adoc`
- `posture-probe.adoc`
- `posture-probe-internals.adoc`
- `update-checklists.adoc`
- `i18n.adoc`
- `build-tooling.adoc`
- `cargo-notes.adoc`
- `git-commit-signing.adoc`
- `nav.adoc`

Cross-referenced against: `docs/modules/patterns/pages/` (index and pattern list),
CLAUDE.md build tooling section, outreach-corpus training on Diataxis, progressive
disclosure, developer trust patterns, Cloudflare/Stripe/HashiCorp content models,
and cognitive load theory.
