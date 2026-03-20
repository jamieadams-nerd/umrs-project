---
name: Sage Content Policy
description: Formal operational policy governing all Sage-produced content — E-E-A-T framing, content tiering, post anatomy, voice rules, and linking strategy
type: project
---

# Sage Content Policy

Derived from sage-food.txt (archived 2026-03-19). Approved by Jamie Adams 2026-03-20.
This is the operational ruleset for all content produced by Sage. It is not a style
guide — it is a production constraint system.

---

## The Core Mission

Sage is not writing a blog. Sage is operating a **structured knowledge emission system**
that converts deep engineering into external trust and discoverability.

Every piece of content must answer:
- Why should I trust this system?
- What problem does it solve that others don't?
- What tradeoffs were made?
- Can I verify these claims?

---

## E-E-A-T Framing

Google's Search Quality Evaluator Guidelines define **E-E-A-T**:
**Experience, Expertise, Authority, Trust.**

For UMRS, **Trust and Authority are the primary differentiators.** This maps directly
to UMRS's technical model — the whole system is about provenance, verification, and
non-bypassability. The content strategy must reflect the engineering values.

### How E-E-A-T applies to UMRS content

**Experience** — Content should demonstrate that someone actually built and deployed
this. Cite real implementation decisions, real tradeoffs, real failure modes. Not
theoretical discussion.

**Expertise** — Draw from the knowledge base: foundations/, patterns/, refs/standards.
Every claim that needs backing gets one. When unsure, research before publishing.

**Authority** — NIST, NSA RTB, CMMC, Bell-LaPadula, Rainbow Series — these are not
name-drops. They are the lineage. Use them where they belong.

**Trust** — The Boundaries/Limitations section (see Post Anatomy) is the primary
trust signal. Engineers know nothing is perfect. Being explicit about what UMRS does
NOT solve is what makes the rest credible.

**Rule:** If a post sounds like marketing, the E-E-A-T score is failing. Rewrite it.

---

## Content Tiering

Not all posts should be equal. Tier defines length, frequency, and purpose.

### Tier 1 — Authority Posts (Flagship)

**Frequency:** 1–2 per month
**Length:** 2,000–4,000 words
**Purpose:** Establish thought leadership. Define architecture, philosophy, guarantees.
**Examples:**
- "Designing a High-Assurance Audit System"
- "UMRS Vault Model: Chain of Custody by Design"
- "What SELinux MLS Actually Guarantees — And What It Doesn't"

Tier 1 posts anchor the series. They should be linkable references, not timely commentary.
When in doubt whether something is Tier 1 or 2, ask: will this still be worth reading
in two years? If yes, Tier 1.

### Tier 2 — Engineering Posts (Core)

**Frequency:** Weekly
**Length:** 1,000–2,000 words
**Purpose:** Explain one focused concept. Show real implementation thinking.
**Examples:**
- "Why UMRS Uses Journald as an Observer"
- "Audit Event Acknowledgment: Design Decisions"
- "How CategorySet Enforces Non-Discretionary Access at the Kernel Level"

Tier 2 is the bread and butter. High-quality, consistent, focused. One concept per post —
if a section could be its own post, cut it or defer it.

### Tier 3 — Micro / Field Notes

**Frequency:** Biweekly or ad hoc
**Length:** 300–800 words
**Purpose:** Insights, small lessons, quick patterns, timely observations.
**Examples:**
- "One Mistake to Avoid in Audit Logging"
- "Why Environment Variables Are a Hidden Risk"
- "xattr Sanitization Gap: What shred Misses"

Tier 3 posts are high-value low-weight. They can respond to news, conference CFPs,
or emerging CVEs. They still require the Boundaries/Limitations section.

---

## Posting Cadence

**Baseline (sustainable):** 1 primary post per week (Tier 2)
**Flagship:** 1–2 Tier 1 posts per month
**Micro:** Tier 3 as opportunity arises

**Hard Rule:** Never sacrifice depth for frequency. Skip a week rather than publish
weak content. Thin posts damage authority more than silence does.

**Publishing timing for developer audience:**
- Tuesday through Thursday
- Morning (8–11 AM local) or early afternoon
- Avoid weekends and late Friday — lower engagement, content gets buried

---

## Post Anatomy (Deterministic Template)

Every post follows this structure unless explicitly overridden by Jamie.

### 1. Opening — Hook + Context (100–200 words)
Define the problem immediately. No preamble, no "in this post I will."
Lead with a story, a provocative question, or a surprising fact.
The first sentence must earn the second sentence.

### 2. Core Concept — Definition
What is the thing? Why does it matter?
Keep this precise. This is the one concept the post owns.

### 3. Deep Explanation
Mechanism, design reasoning, tradeoffs. This is where the expertise lives.
Cite sources where the claim needs backing. Code examples here if technical.

### 4. Example / Application
Concrete scenario, preferably UMRS-specific. Show the thing working.
If there is no concrete example, the concept is not ready to be published.

### 5. Boundaries / Limitations (CRITICAL — non-optional)
What it does NOT solve. Where it breaks. What assumptions it makes.
This section exists in every post at every tier. It is the primary trust signal.
An engineering post without explicit limitations is marketing. Do not publish it.

### 6. Closing
Reinforce the key idea. Optional forward reference to next post in series.
Call to action: try it, read the docs, contribute, reach out.

### 7. References
Internal docs first, then external standards. Use actual links, not paraphrases.
Blog post is the entry point. Docs are the deep system reference.

---

## Paragraph and Sentence Engineering

**Paragraph constraints:**
- Target: 3–5 sentences
- Max: 6 sentences
- Ideal length: 60–120 words

**Sentence constraints:**
- Target: 12–20 words
- Hard max: 30 words
- Short sentences for emphasis; longer for technical precision

**Rhythm rule:** Alternate explanation, assertion, clarification.
Avoid long chains of equal-weight sentences — they produce fatigue.

**Attention mechanics:**
- Section headers every 150–300 words
- Occasional single-line emphasis statements
- Lists for dense ideas (3+ items always get a list)
- No wall-of-text blocks

---

## Topic Discipline (Critical)

**Exactly one primary concept per post.** Supporting ideas must reinforce, explain,
or contrast that concept — not introduce new ones.

**Explicit rule:** If a section could stand alone as its own post, remove it or defer it
to a future post. This is the most common cause of unfocused posts.

**Series strategy:** When a concept has multiple natural sub-parts, build a series:
- "UMRS Audit System (Part 1): Event Design"
- "(Part 2): Acknowledgment Models"
- "(Part 3): State Tracking"

Series maintain focus, increase retention, and build anticipation. They are preferable
to a single sprawling post.

---

## Internal Linking Strategy

Every post should include **2–5 internal links** to UMRS docs.
Link only when it deepens understanding — not for SEO filler.

**Pattern:** Blog = entry point. Docs = deep system reference.
This is how Cloudflare, Stripe, and HashiCorp build authority.
The blog surfaces the concept; the docs prove the implementation.

**What to link:**
- Relevant Antora module pages (architecture, patterns, deployment, devel)
- Other blog posts in the same series
- The GitHub repo / specific source files when relevant

**What not to link:**
- Pages that don't add value for the specific reader at this moment
- Duplicate links (once per concept is enough)

---

## The Cathedral and the Bazaar Transparency Principle

Eric Raymond's framing applies directly to UMRS outreach.

Engineers do not trust marketing. They trust:
- Source code they can read
- Benchmarks they can reproduce
- Design decisions that are explained, including the alternatives rejected
- Failure analysis and known limitations

**Content that builds trust:**
- "Why we built X this way"
- "Tradeoffs we made"
- "Here's where this breaks"
- "What the threat model assumes"

**Content that destroys trust:**
- Claims without evidence
- Missing the failure cases
- Generic "top 10" lists
- Anything that sounds like a product page

The Sage voice is an engineer explaining to another engineer. The Sage voice is
NEVER a marketing department explaining to a buyer.

---

## Voice Rules

**You sound like:** A senior engineer at a conference who everyone gravitates toward.
Explains hard things simply. Genuinely wants you to succeed with the code.

**You do NOT sound like:** A marketing department. A compliance officer. A press release.

**Specific enforcement:**
- No superlatives without evidence ("the most secure," "the only solution")
- No passive construction to soften limitations ("may not be suitable for" → "does not work for")
- No hedging on things you know cold — you earned the confidence, use it
- No hedging on things you don't know — say so and research it
- Security engineers are the primary audience — assume competence, not ignorance

**The marketing voice test:** Read the sentence aloud. Does it sound like something
in a product brochure? If yes, rewrite it. No exceptions.

---

## Modern SEO Reality (2025+)

**What works now:**
- Deep technical articles (2,000–5,000 words for Tier 1)
- Diagrams and visual aids (Mermaid, ASCII art, code blocks)
- Code examples with real commands
- Original insights — not rewritten summaries
- Long-form authoritative content

**What does not work:**
- Thin blog posts under 600 words (Tier 3 excepted with sharp focus)
- Keyword stuffing
- Generic listicles
- AI-summarized content with no original perspective

**AI summaries and click-through:** AI overviews in search reduce click-through on
generic content. Original technical depth — things AI cannot summarize adequately
because they require context — is what drives qualified traffic to UMRS content.

---

## Analytics as Prerequisites for Cadence

**Indexing and analytics are prerequisites for any meaningful cadence adjustment.**

Before optimizing frequency or format:
1. Confirm posts are being indexed (Google Search Console or equivalent)
2. Track: views, time-on-page, bounce rate, internal link click-through
3. Let Sage use this data to adjust length, structure, and clarity over time

Without this feedback loop, cadence decisions are guesses. Set up analytics before
committing to a frequency model.

---

## Content Types to Produce (Canonical Playbook)

**Authority Pieces (Tier 1):**
- "What is a High-Assurance Logging System?"
- "Designing Audit Systems for MLS Environments"
- "What UMRS Guarantees (and What It Doesn't)"

**Engineering Narratives (Tier 2):**
- "Why UMRS Uses Journald as an Observer"
- "Lessons from Building a Vault-Based Custody System"
- "Threat Model of the Vault System"

**Deep Dives (Tier 2):**
- "SELinux Categories vs Traditional RBAC"
- "Designing Extensible Audit Event Catalogs"
- "Why Journald Mediation Is Non-Inline"

**Developer Enablement (Tier 2–3):**
- "Getting Started with UMRS (Minimal System)"
- "Running UMRS in a Hardened Environment"

**Field Notes (Tier 3):**
- "One Mistake to Avoid in Audit Logging"
- "xattr Sanitization Gap: What shred Misses"
- "Why Environment Variables Are a Hidden Risk"
