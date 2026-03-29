---
name: Post Scoring Rubric
description: Self-check rubric Sage runs before any post is marked ready to publish — covers concept discipline, trust signals, E-E-A-T, technical accuracy, and SEO basics
type: project
---

# Post Scoring Rubric

Run this rubric before marking any post ready to publish. Every item is a hard check —
not a soft suggestion. A post that fails any CRITICAL item does not publish.

Score each item: PASS / FAIL / N/A
A post is ready when: all CRITICAL items PASS, all IMPORTANT items PASS or have
documented exceptions, STANDARD items are mostly green.

---

## Block 1 — Concept Discipline (CRITICAL)

These failures invalidate the post regardless of other scores.

| # | Check | Notes |
|---|---|---|
| 1.1 | **Single primary concept** — I can state the one thing this post is about in one sentence | If you need "and" in that sentence, the post has two concepts |
| 1.2 | **No section that could stand alone** — every section reinforces the primary concept; none introduces an independent new concept | Cut deferrable sections to future posts |
| 1.3 | **Topic stated in opening** — the reader knows what the post is about within the first paragraph | |
| 1.4 | **Closing reinforces the primary concept** — the ending lands on the one thing, not a tangent | |

---

## Block 2 — Trust Signals (CRITICAL)

| # | Check | Notes |
|---|---|---|
| 2.1 | **Boundaries/Limitations section present** — explicit statement of what this does NOT solve, where it breaks, what it assumes | Non-optional at every tier. No exceptions. |
| 2.2 | **No unsupported superlatives** — every "most," "best," "only," or "uniquely" claim has evidence or is removed | |
| 2.3 | **Failure modes are honest** — when the system can fail, the post says so directly | |
| 2.4 | **Threat model assumptions stated** — if the post makes security claims, the assumed threat model is visible | |

---

## Block 3 — E-E-A-T Signals (IMPORTANT)

| # | Check | Notes |
|---|---|---|
| 3.1 | **Experience signal** — post reflects someone who actually built or deployed this (real decisions, real tradeoffs, not theoretical) | |
| 3.2 | **Expertise signal** — claims that need backing have citations (NIST, NSA RTB, CMMC, or UMRS docs) | |
| 3.3 | **Authority signal** — relevant lineage cited where it belongs (Bell-LaPadula, Rainbow Series, SELinux history, etc.) without name-dropping | |
| 3.4 | **Trust signal** — Boundaries/Limitations + honest failure modes present (covered in Block 2, but confirm E-E-A-T alignment) | |

---

## Block 4 — Voice Check (IMPORTANT)

| # | Check | Notes |
|---|---|---|
| 4.1 | **No marketing voice** — read the post aloud; nothing sounds like a product page or press release | Rewrite any sentence that fails this test |
| 4.2 | **No hedging on known facts** — where you know something cold, you say it with appropriate confidence | |
| 4.3 | **No false confidence** — where you're uncertain, you say so; cited claims are verified | |
| 4.4 | **Engineer-to-engineer register** — assumes technical competence; does not condescend | |
| 4.5 | **No passive construction hiding limitations** — "may not be suitable" → "does not work for" | |

---

## Block 5 — Technical Accuracy (CRITICAL for technical posts)

| # | Check | Notes |
|---|---|---|
| 5.1 | **Phase 1 vs Phase 2 distinction maintained** — capabilities described as complete are actually complete; planned features are labeled as planned | Never claim UMRS does something it does not yet do |
| 5.2 | **Code examples verified** — any command, code snippet, or config shown has been tested or is clearly labeled as illustrative | |
| 5.3 | **No implementation details that aid adversaries** — reviewed for sensitive details about undeployed security mechanisms | |
| 5.4 | **No false government endorsement implied** — post does not imply NSA, NIST, or any agency endorses UMRS | |
| 5.5 | **Citations checked against .claude/references/refs-manifest.md** — any cited standard is in the manifest and the version cited is the version we reference | |

---

## Block 6 — Structure and Length (STANDARD)

| # | Check | Notes |
|---|---|---|
| 6.1 | **Tier-appropriate length** — Tier 1: 2,000–4,000 words; Tier 2: 1,000–2,000 words; Tier 3: 300–800 words | |
| 6.2 | **Post anatomy followed** — Opening, Core Concept, Deep Explanation, Example/Application, Boundaries/Limitations, Closing, References | Override is allowed with documented reason |
| 6.3 | **Section headers every 150–300 words** — no wall-of-text blocks | |
| 6.4 | **Paragraph discipline** — target 3–5 sentences, max 6, 60–120 words | |
| 6.5 | **Sentence discipline** — target 12–20 words, hard max 30 | |
| 6.6 | **Concrete example present** — there is at least one specific, UMRS-grounded example | A post with no concrete example is not ready |

---

## Block 7 — Internal Linking (IMPORTANT)

| # | Check | Notes |
|---|---|---|
| 7.1 | **2–5 internal links present** — linking to UMRS Antora docs, other blog posts in series, or GitHub source | |
| 7.2 | **Links add value** — every link deepens understanding; none are SEO filler | |
| 7.3 | **Blog → Docs pattern followed** — this post is the entry point; the docs are the deep reference | |
| 7.4 | **No broken links** — verify all internal links resolve before publish | |

---

## Block 8 — SEO Basics (STANDARD)

| # | Check | Notes |
|---|---|---|
| 8.1 | **Title contains primary keyword** — title is specific, descriptive, under 60 characters where possible | |
| 8.2 | **Description/subtitle present** — 150–160 character summary for meta description | |
| 8.3 | **Tags applied** — at least 3 relevant tags from: SELinux, CUI, CMMC, Rust, memory-safety, high-assurance, mandatory-access-control, NIST, audit, MLS | |
| 8.4 | **Natural keyword integration** — primary keywords appear in opening, at least one header, and closing — not stuffed | |
| 8.5 | **No duplicate title** — title is not a repeat of an existing post | |

---

## Block 9 — Series and Forward Reference (STANDARD)

| # | Check | Notes |
|---|---|---|
| 9.1 | **Series placement clear** — if this is part of a series, the series name and part number are in the title or opening | |
| 9.2 | **Forward reference optional but present if applicable** — closing mentions the next post if one is planned | |
| 9.3 | **Back reference present** — if this is not part 1, the opening links to the prior post in series | |

---

## Publication Checklist (Final Gate)

Run this last, after all blocks score clean.

- [ ] All CRITICAL items: PASS
- [ ] All IMPORTANT items: PASS or documented exception on file
- [ ] STANDARD items: majority green, failures noted
- [ ] Post saved to `docs/umrs-blogs/drafts/` and reviewed at least once before promotion to `docs/umrs-blogs/`
- [ ] Jamie has reviewed or waived review for Tier 1 posts
- [ ] Publication timing: Tuesday–Thursday, 8–11 AM local (defer if not)

---

## Scoring by Tier

**Tier 1 — Authority Posts:** All blocks CRITICAL. Jamie review required.
No Tier 1 post publishes without full rubric pass.

**Tier 2 — Engineering Posts:** All CRITICAL + IMPORTANT blocks required.
STANDARD blocks should be green but exceptions are documentable.

**Tier 3 — Micro / Field Notes:** Blocks 1, 2, 4, 5 CRITICAL.
Blocks 3, 6, 7, 8 apply but length/structure constraints are relaxed.
Tier 3 still requires the Boundaries/Limitations section.
