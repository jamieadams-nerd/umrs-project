# Diataxis Framework — A Systematic Approach to Technical Documentation

**Source:** https://diataxis.fr/ (Daniele Procida)
**Date compiled:** 2026-03-20
**Phase:** 2C — Technical writing & documentation science

---

## The Four Content Types

Diataxis organizes all documentation into four types based on two axes:
- **Orientation**: Learning vs. Working
- **Approach**: Practical vs. Theoretical

|  | Practical | Theoretical |
|---|---|---|
| **Learning** | Tutorial | Explanation |
| **Working** | How-to Guide | Reference |

### Tutorial — Learning by doing

- Goal: Help the learner acquire skills through guided steps
- Structure: "Do this, then this, then this — now you've built something"
- The author decides every step. The reader follows.
- Never explain "why" in detail — save that for Explanation
- Must work end-to-end if followed exactly
- **UMRS example:** "Build your first UMRS posture report in 10 minutes"

### How-to Guide — Solving a specific problem

- Goal: Help the user accomplish a specific task
- Structure: "To do X, follow these steps..."
- Assumes the reader already knows the basics
- Can branch based on conditions ("If RHEL 10, do A; if Ubuntu, do B")
- **UMRS example:** "Configure MCS categories for CUI directories"

### Explanation — Understanding the design

- Goal: Help the reader understand why things are the way they are
- Structure: Discussion, context, rationale, alternatives considered
- No steps, no commands — pure understanding
- Can reference history, design decisions, tradeoffs
- **UMRS example:** "Why UMRS uses TPI parsing for security contexts"

### Reference — Looking up specifics

- Goal: Provide accurate, complete technical descriptions
- Structure: API docs, configuration options, parameter tables
- Organized by the system's structure, not by user tasks
- Must be complete and consistent — austere, not chatty
- **UMRS example:** rustdoc API documentation, CLI `--help` output

---

## The Key Insight

Most documentation problems come from mixing types. A tutorial that stops to explain theory loses momentum. A reference that includes tutorials becomes bloated. A how-to guide that teaches fundamentals wastes expert time.

**Keep them separate.** Cross-reference between them, but don't merge them.

---

## How UMRS Maps to Diataxis

| Antora Module | Primary Diataxis Type | Notes |
|---|---|---|
| devel/ | How-to + Explanation | Developer guide mixes practical and conceptual |
| deployment/ | How-to | Procedure-heavy, STE rules apply |
| operations/ | How-to + Reference | Operational procedures + indicator reference |
| patterns/ | Explanation | Design rationale, pattern descriptions |
| reference/ | Reference | Configuration references, glossary |
| ROOT/ | Explanation | Project overview, introduction |
| Blog posts | Explanation + Tutorial | Vary by post type |

---

## Blog Content and Diataxis

Blog posts don't fit neatly into Diataxis types — they serve a different purpose (discovery, engagement, trust-building). But the framework still helps:

- **Tier 1 authority posts** → primarily Explanation (with code examples)
- **Tier 2 engineering posts** → How-to or Explanation
- **Tier 3 field notes** → How-to (quick, practical)

The blog's job is to bring people to the docs. The docs' job is to help them succeed.

---

## Actionable Insights for Sage

- Label blog posts by their Diataxis type in metadata — helps readers find what they need
- Authority posts should be predominantly Explanation — "why we built it this way"
- Engineering posts can be How-to — "how to set up MCS categories"
- Never mix tutorial content with reference content in the same post
- Cross-link blog → docs aggressively: blog post explains "why", doc explains "how"

## Sources

- [Diataxis Framework](https://diataxis.fr/)
- [Diataxis — Tutorials](https://diataxis.fr/tutorials/)
- [Diataxis — How-to Guides](https://diataxis.fr/how-to-guides/)
