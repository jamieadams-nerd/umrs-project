# UMRS Content Strategy — Operational Specification

**Date compiled:** 2026-03-20
**Phase:** 2F — Synthesis document
**Status:** Sage's operational playbook — derived from all Phase 2A-2E research

---

## Purpose

This document is Sage's actionable content policy. It distills research on SEO, developer
trust, technical branding, documentation science, and diffusion theory into concrete
operational rules for UMRS content production.

---

## Content Tiering

### Tier 1 — Authority Posts

- **Length:** 2,000–4,000 words
- **Cadence:** 1–2 per month
- **Purpose:** Establish UMRS as a serious, trustworthy project. Build E-E-A-T. Attract opinion leaders.
- **Diataxis type:** Primarily Explanation
- **Examples:**
  - "Why CUI needs kernel enforcement" (published 2026-03-19)
  - "Inside the UMRS trust evidence chain"
  - "How TPI parsing prevents silent security context corruption"
  - "The three-layer CPU security model: hardware, OS, application"
- **Structure:** Hook → problem statement → design exploration → implementation → results → limitations → references
- **Quality gate:** Must pass Jamie review. Must cite at least 2 authoritative sources (NIST, kernel docs, etc.)

### Tier 2 — Engineering Posts

- **Length:** 1,000–2,000 words
- **Cadence:** Weekly (aspirational; actual cadence depends on development velocity)
- **Purpose:** Show active development, demonstrate technical depth, build content volume for SEO
- **Diataxis type:** How-to or Explanation
- **Examples:**
  - "Adding a new posture signal to umrs-platform"
  - "CategorySet: from kernel bitmask to typed Rust"
  - "What the kernel security tab tells you about your RHEL 10 system"
- **Structure:** Problem → approach → code → output → takeaway
- **Quality gate:** Must include real code and/or real output

### Tier 3 — Field Notes

- **Length:** 300–800 words
- **Cadence:** Biweekly
- **Purpose:** Quick practical content, SEO long-tail, community engagement
- **Diataxis type:** How-to (short) or reference note
- **Examples:**
  - "Quick tip: reading SELinux contexts with umrs-ls"
  - "TIL: SELinux category bits are little-endian in the kernel"
  - "What changed in RHEL 10's default SELinux policy"
- **Structure:** Context (1-2 sentences) → content → takeaway
- **Quality gate:** Must be technically accurate. Does not require Jamie review.

---

## Post Anatomy (Tier 1 Template)

```
1. Hook (1-2 sentences)
   - State the problem or question the reader has
   - Make it concrete and relatable

2. Context (1-2 paragraphs)
   - Why this matters
   - What's at stake (for CUI handlers, for compliance, for security)

3. Deep Explanation (bulk of the post)
   - Technical content with code, diagrams, examples
   - Progressive disclosure: start accessible, build to expert depth
   - One primary concept per post

4. Real-World Application
   - Show actual output from UMRS tools
   - Include screenshots, terminal output, or code examples

5. Limitations and Honest Assessment
   - What this doesn't solve
   - What Phase 2 will add (if applicable)
   - Known gaps or future work

6. Closing (2-3 sentences)
   - Restate the key insight
   - Point to next steps or related content

7. References
   - All cited sources with links
   - Links to UMRS documentation
   - Links to related UMRS blog posts
```

---

## Paragraph Engineering

- **Sentences per paragraph:** 3–5
- **Words per paragraph:** 60–120
- **Words per sentence:** Target 12–20, max 30
- **One idea per paragraph** — if you need to say "however" or "on the other hand", start a new paragraph
- **Lead with the point** — first sentence states the claim, rest supports it
- **No orphan paragraphs** — single-sentence paragraphs are for emphasis only, use sparingly

---

## Voice Calibration

### Do:

- Write as an engineer explaining to a peer — technically precise, conversationally warm
- Use first person for blog posts ("I built", "we designed") — authenticity signal
- Use confidence grounded in evidence ("this approach ensures X because Y")
- Admit limitations and unknowns — "Phase 1 does not enforce; it ensures awareness"
- Show excitement about the engineering, not the marketing

### Don't:

- Use corporate voice ("We're excited to announce...")
- Use marketing superlatives ("revolutionary", "best-in-class", "enterprise-grade")
- Use vague hedging ("might", "could potentially", "in some cases")
- Write in passive voice for procedures
- Hide limitations behind euphemisms

### Voice examples:

**Good:** "CategorySet is a 1024-bit bitmask that the kernel evaluates on every access check. It's not optional, it's not advisory — if your process doesn't have the right categories, the kernel says no."

**Bad:** "UMRS provides enterprise-grade category management that leverages kernel-level enforcement capabilities to ensure compliance with CUI handling requirements."

---

## Internal Linking Rules

- Every blog post links to 2–5 UMRS documentation pages
- Every blog post links to at least 1 previous UMRS blog post (when available)
- Blog = entry point, documentation = deep reference
- Use descriptive anchor text: "how MCS categories work" not "click here"
- Hub pages (project overview, getting started) should be linked from many posts

---

## Series Strategy

For topics requiring > 4,000 words:

- Split into a planned series (2–4 parts)
- Each part must stand alone (don't require reading previous parts to understand)
- Publish on a predictable schedule (e.g., consecutive weeks)
- First part includes a series overview / table of contents
- Each part cross-links to others in the series

---

## SEO Integration

- **Post titles match search intent:** "How SELinux MCS Categories Enforce CUI Boundaries" > "MCS Categories"
- **Include `datePublished` and `dateModified`** in structured data (JSON-LD BlogPosting)
- **Named author** with linked author page
- **Canonical URLs** on all pages
- **XML sitemap** updated with each new post
- **Meta description:** 150-160 characters summarizing the post's value proposition

---

## Publishing Cadence

- **Target:** 1 primary post per week
- **Never sacrifice depth for frequency** — a late, thorough post beats a shallow on-time post
- **Buffer:** Maintain 2 posts in draft at all times (prevents gaps during busy development periods)
- **Seasonal awareness:** Conference seasons (RSA, DEF CON, Black Hat) are high-visibility windows

---

## Phase 1 vs Phase 2 Content Guardrails

**CRITICAL:** All content must accurately reflect UMRS's current capabilities.

### Phase 1 (Targeted Policy) — What we CAN claim:

- Authoritative labeling — all data is explicitly labeled and traceable
- Operator awareness — continuous signaling of sensitivity and handling restrictions
- System-bound custody — integrity, visibility, and correctness of metadata within system boundary
- Minimal controls — type enforcement, controlled directories where feasible
- Boundary-based responsibility model

### Phase 1 — What we CANNOT claim:

- End-to-end enforcement of CUI handling
- Mandatory access control based on clearance
- Strong isolation between sensitivity levels
- Enforcement beyond the system boundary

### How to frame Phase 1:

- "UMRS ensures the data knows what it is" ✓
- "UMRS enforces who can touch it" ✗ (Phase 2)
- "Awareness is a control — and often the missing one" ✓
- "UMRS guarantees CUI protection" ✗ (overclaim)

---

## Quality Checklist (Pre-Publication)

Before any post goes live:

- [ ] Technical accuracy verified (code examples tested, output current)
- [ ] Phase 1/Phase 2 claims are honest and correctly scoped
- [ ] At least 2 internal links to UMRS docs
- [ ] Named author with date
- [ ] JSON-LD structured data included
- [ ] No marketing superlatives
- [ ] Limitations acknowledged
- [ ] Sources cited with links
- [ ] Paragraph engineering constraints met (3-5 sentences, 60-120 words)
- [ ] Mermaid diagram included if architecture is discussed
- [ ] CUI marking syntax correct (`CUI//LEI/INV`)

---

## Measurement (Future — When Analytics Are Available)

Track per post:
- Page views (volume)
- Time on page (engagement)
- Referral sources (discovery channel)
- Outbound clicks to docs (funnel effectiveness)
- GitHub stars/forks within 48h of publication (conversion)

Track over time:
- Search Console impressions and clicks for target keywords
- Backlink acquisition rate
- Returning visitor rate

---

## Sources

This operational spec synthesizes insights from:
- [SEO Starter Guide | Google Search Central](https://developers.google.com/search/docs/fundamentals/seo-starter-guide)
- [E-E-A-T Framework | Google Search Quality Guidelines](https://services.google.com/fh/files/misc/hsw-sqrg.pdf)
- [Diataxis Framework | diataxis.fr](https://diataxis.fr/)
- [The Cathedral and the Bazaar | catb.org](http://www.catb.org/~esr/writings/cathedral-bazaar/)
- [Producing Open Source Software | producingoss.com](https://producingoss.com/)
- [How good corporate engineering blogs are written | Dan Luu](https://danluu.com/corp-eng-blogs/)
- [Diffusion of Innovations | Everett Rogers](https://en.wikipedia.org/wiki/Diffusion_of_innovations)
- [Progressive Disclosure | Nielsen Norman Group](https://www.nngroup.com/articles/progressive-disclosure/)
