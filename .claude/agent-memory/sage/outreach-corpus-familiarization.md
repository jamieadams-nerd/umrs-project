---
name: Sage Outreach Corpus — Familiarization Summary
description: Active knowledge summary of all 15 files in the sage-outreach-corpus RAG collection. Key frameworks, UMRS content strategy constraints, and priority insights. Date: 2026-03-20.
type: project
---

Source collection: `sage-outreach-corpus` (RAG, 113 chunks, 15 files)
Source files: `.claude/references/sage-outreach-corpus/`
Familiarization completed: 2026-03-20

---

## Active Knowledge: Key Frameworks

### 1. SEO Fundamentals (seo-fundamentals.md)

**Crawl → Index → Rank pipeline.** Google discovers pages via links and sitemaps; content
quality is the dominant ranking factor. Key technical requirements for UMRS's GitHub Pages
site: descriptive URLs, XML sitemap, canonical URLs on every page, image alt text, descriptive
anchor text on internal links.

**Core Web Vitals** (LCP < 2.5s, INP < 200ms, CLS < 0.1): Static GitHub Pages performs well
by default. Monitor via Search Console post-launch. No server-side rendering means no
JavaScript-related CWV risk.

**Topical depth matters.** Consistent publication on SELinux, RHEL 10 hardening, CMMC,
and Rust security signals domain authority across all related queries — not just individual
post rankings. Long-tail keywords ("SELinux security context Rust parsing") are more achievable
than head terms.

**Internal links are load-bearing.** Every blog post must link 2-5 times to UMRS docs.
Blog = discovery layer; docs = authority signal. Pages not linked from anywhere are invisible.

**Freshness signal:** `dateModified` in structured data. When RHEL 10 STIG or NIST
publications update, update the affected posts. Update > new thin post for SEO.

### 2. E-E-A-T Framework (eeat-framework.md)

Google's four-dimension content quality framework:
- **Experience** — firsthand, "here's what happened when we ran this on RHEL 10"
- **Expertise** — demonstrated through implementation and compliance cross-referencing
- **Authoritativeness** — currently building; requires external citations, backlinks, community presence
- **Trust** — most important dimension; open source + compliance annotations + named author = strong foundation

**YMYL status:** All UMRS content is implicitly YMYL (Your Money or Your Life) because
bad security advice causes real harm. Google holds YMYL to a higher standard. UMRS's
rigor is a competitive advantage here — shallow content cannot satisfy YMYL requirements.

**Critical insight:** Admitting Phase 1 limitations (no enforcement in targeted policy)
INCREASES E-E-A-T trust score. Overclaiming kills it.

**UMRS E-E-A-T status:**
- Experience: STRONG (Jamie is building it; real output in every post)
- Expertise: VERY STRONG (implementation + compliance annotations + NIST citations)
- Authoritativeness: BUILDING (new project; focus on conference talks, community citations)
- Trust: STRONG FOUNDATION (open source, named author, version-controlled docs)

**Immediate gap:** Build an author page for Jamie at jamieadams-nerd.github.io/about.

### 3. Structured Data / Schema.org (structured-data.md)

Every blog post needs `BlogPosting` JSON-LD with `datePublished`, `author`, and `description`.
Add to `_layouts/post.html` template — automated for every post. Key schemas for UMRS:
- `BlogPosting` — blog posts
- `SoftwareSourceCode` — project main page
- `TechArticle` with `proficiencyLevel: "Expert"` — documentation pages
- `BreadcrumbList` — documentation hierarchy

Validate with Google's Rich Results Test. This is a one-time implementation task
with ongoing per-post automation.

### 4. Developer Trust Patterns (developer-trust-patterns.md)

Engineers evaluate in four stages (< 30 seconds / 5-15 min / hours-days / adoption decision).
They trust: source code, benchmarks, design transparency, failure analysis.
They distrust: marketing claims, hidden limitations, corporate voice, vague benchmarks.

**UMRS natural strengths:** Open source, strict clippy/forbid-unsafe, compliance annotations
are verifiable, real SELinux on real RHEL 10 (not theoretical).

**Content types engineers trust most:** Architecture deep-dives, postmortems, "why we
built X" / "tradeoffs we made", benchmarks with methodology.

**Anti-patterns to avoid:** "enterprise-grade", "military-grade", any superlative without
evidence, hiding Phase 1 limitations, corporate "we're excited to announce" voice.

**Priority content opportunities:**
- "Why we chose `#![forbid(unsafe_code)]`" — architecture decision with reasoning
- "How TPI parsing prevents supply chain attacks on security contexts" — deep dive
- "What we got wrong in our first posture probe implementation" — postmortem
- "CategorySet: from kernel bitmask to typed Rust" — design transparency

### 5. Cathedral and the Bazaar (cathedral-bazaar.md)

ESR's open source model insight applied to UMRS: security architecture requires
cathedral-mode design (expert core, deliberate, proven correct). Documentation,
tool UX, posture signals, and testing are suitable for bazaar-mode (broader contribution).

**Content opportunity:** Jamie's motivation story IS the marketing for engineers. "Scratching
a personal itch" is the highest-trust origin story for open source tools. Publish about
work in progress — show the journey, not just finished features.

**The hybrid model:** Cathedral the core (type system, enforcement logic, high-assurance
patterns). Bazaar the periphery (docs, platform examples, integration guides).

### 6. Producing Open Source Software (producing-oss.md)

**Technical trust comes first.** Contributors check: code quality, architecture clarity,
build reproducibility, test coverage. UMRS scores well on all four.

**Current gaps (pre-public):**
- No public CONTRIBUTING.md
- No GitHub Discussions enabled
- No architectural decision records visible to outsiders
- No "good first issue" labels

**First-clone-to-first-test-pass** should take < 5 minutes. Verify `cargo test` works
out of the box before any public announcement.

**BDFL model:** For a one-person project transitioning to public, state the benevolent
dictator model explicitly. Prevents governance confusion.

**Bus factor:** Documented architecture, CI/CD automation, and clear design rationale
mitigate the single-maintainer risk for potential adopters.

### 7. Progressive Disclosure (progressive-disclosure.md)

Four-level disclosure: What does this do? → How do I use it? → How does it work? → How do I extend it?
Each level satisfies a reader who stops there.

**Information scent:** Navigation and heading labels must tell readers what they'll find.
"Install UMRS on RHEL 10" (strong scent) vs "Getting Started" (weak scent).

**Cognitive load:** Reduce extraneous load (poor presentation, inconsistent terminology).
Maximize germane load (analogies: "CategorySet is like Unix groups, but mandatory").
New concepts per post: ≤ 5. If more, split into a series.

### 8. Diataxis Framework (diataxis.md)

Four content types, kept separate:
- **Tutorial** — learning by doing (guided steps, end-to-end)
- **How-to** — solving a specific problem (task-oriented)
- **Explanation** — understanding why (rationale, design decisions)
- **Reference** — looking up specifics (API docs, CLI help)

**UMRS blog mapping:**
- Tier 1 authority posts → primarily Explanation
- Tier 2 engineering posts → How-to or Explanation
- Tier 3 field notes → How-to (short)

**Cross-link rule:** Blog explains "why", docs explain "how". Blog is the entry point;
docs are the deep reference. Mixing types is the most common documentation failure.

### 9. Google Dev Style Guide (google-dev-style.md)

Key rules already aligned with UMRS standards: active voice, present tense, specific numbers,
short sentences (20-word target), consistent terminology, code font for technical terms.

**UMRS deviation from Google style (intentional):** Google is deliberately neutral;
UMRS blog posts use first person ("I built") for authenticity. Google avoids first person
in docs; UMRS energetic tone is a feature, not a bug.

### 10. Cloudflare Model (cloudflare-model.md)

Signature pattern: architecture deep-dives (problem → constraints → design → results →
limitations → future). "How we built Pingora" is the canonical example.

**UMRS parallel:** "Why we built typed security primitives in Rust" — same structure.
Postmortems build trust faster than success stories.

**Consider "launch week":** Coordinated blog series introducing UMRS from multiple angles
when going public.

Key voice characteristics: confident but not arrogant, technical depth without gatekeeping,
diagrams first-class, honest about limitations. Closely matches UMRS target voice.

### 11. Stripe Model (stripe-model.md)

Signature pattern: "Explain complex systems simply" — accessible entry, expert depth,
progressive disclosure within a single post.

**UMRS parallel:** "How CategorySet encodes CUI boundaries in 1024 bits" — start with
"what's a CUI category?", build to bitmask encoding, end at kernel enforcement semantics.

**Voice:** Calm authority. Confident without bragging. "Understated confidence — demonstrates
competence through content, not claims." This is the target register.

Infrastructure trust articles map directly to UMRS high-assurance patterns.
"How we ensure security context parsing can never silently succeed with wrong data" is
the Stripe narrative translated to UMRS engineering.

### 12. HashiCorp Model (hashicorp-model.md)

Signature pattern: "Inside the system" articles — transparency IS the trust signal.
Architecture of internal mechanisms (Vault seal/unseal, Terraform state) builds trust
through openness.

**UMRS parallel:** "Inside UMRS's trust evidence chain" — provenance verification
end-to-end. "Security is not a feature — it's an architecture" maps directly to
"Labeling is not a feature — it's a custody guarantee."

**Open source → enterprise bridge:** Content that serves both audiences simultaneously
(architecture deep-dives = community trust + enterprise rigor). Start building this
bridge now through content even before enterprise features exist.

**Role-targeted guides:** "Security operator's guide to the UMRS kernel security tab."
Different roles need different entry points.

### 13. Diffusion of Innovations (diffusion-of-innovations.md)

Rogers' five adopter categories: Innovators (2.5%) → Early Adopters (13.5%) → Early
Majority (34%) → Late Majority (34%) → Laggards (16%).

**UMRS current position:** Targeting Innovators and Early Adopters. Content should
speak to vision and technical depth, not pragmatic completeness (that's Early Majority
language).

**Five adoption attributes for UMRS:**
- Relative advantage: STRONG (nothing else does typed CUI labeling on SELinux)
- Compatibility: MIXED (requires SELinux; emphasize RHEL 10 native support)
- Complexity: HIGH (SELinux + MLS + Rust + compliance is inherently complex)
- Trialability: MEDIUM (weakest attribute; "try it in 10 minutes" tutorial needed)
- Observability: LOW (new project; every post with real output improves this)

**Beachhead strategy (Moore's Crossing the Chasm):**
1. Beachhead: Security engineers handling CUI on RHEL/SELinux, needing NIST/CMMC compliance
2. Adjacent: Linux security teams → kernel security posture tools
3. Broader: Compliance teams → assessment and reporting
4. Future: MLS environments → full enforcement

**Opinion leaders matter more than audience size.** One respected SELinux engineer who
endorses UMRS (Red Hat, NSA, DISA) is worth 10,000 random visitors. Target them with
technical depth, invitation to critique, acknowledgment of prior work.

**Two channel types:** Mass media (blog, conference) creates awareness. Interpersonal
(code review, GitHub engagement, direct conversation) drives adoption. Both required.

### 14. Content Strategy Operational Spec (content-strategy-operational-spec.md)

This is the primary operational playbook. Key rules:

**Content tiers:**
- Tier 1 (Authority): 2,000-4,000 words, 1-2/month, primarily Explanation, Jamie review required
- Tier 2 (Engineering): 1,000-2,000 words, weekly aspirational, code required
- Tier 3 (Field Notes): 300-800 words, biweekly, no Jamie review needed

**Post anatomy (Tier 1):** Hook → Context → Deep Explanation → Real-World Application
→ Limitations → Closing → References

**Paragraph engineering:** 3-5 sentences, 60-120 words, lead with the point, one idea per
paragraph, no orphan paragraphs (single-sentence for emphasis only)

**Phase 1/2 guardrails:** The operational spec explicitly calls these CRITICAL. See
phase1-phase2-positioning.md for the full decision tree.

**Quality checklist (pre-publication):** 12 items including technical accuracy, Phase 1/2
claim review, 2 internal links, named author, JSON-LD structured data, no marketing
superlatives, limitations acknowledged, sources cited, paragraph engineering constraints,
Mermaid diagram if architecture discussed, CUI marking syntax.

**Measurement (future):** Page views, time on page, referral sources, outbound clicks to
docs, GitHub stars/forks within 48h. Track search impressions and clicks in Search Console.

---

## UMRS Content Strategy Constraints (Summary)

1. **Phase 1/2 claims are hard guardrails** — always verify enforcement language; Phase 1 is awareness and labeling, not enforcement
2. **Beachhead audience first** — SELinux security engineers on RHEL; do not dilute toward general Linux or general developer content
3. **Trialability is the weakest adoption attribute** — "try it in 10 minutes" tutorial is high priority
4. **Analytics prerequisite** — do not optimize cadence before site is confirmed indexed and analytics are flowing
5. **CONTRIBUTING.md and "good first issues" are prerequisites for community engagement** — not optional
6. **Author page at /about is a trust gap** — E-E-A-T requires named author with profile
7. **Structured data (JSON-LD) is a one-time implementation task** — then automated per post
8. **Never say "enterprise-grade" or "military-grade"** without specific, verifiable evidence

---

## Priority Insights That Will Shape Outreach Strategy

### Highest priority: Fix the trialability gap
UMRS's weakest Rogers adoption attribute is trialability. Before any content volume
push, there needs to be a "try it in 10 minutes on RHEL 10" path. Without it, engineers
who read the blog have nowhere to go.

### Content type to lead with: Architecture deep-dives
Cloudflare, Stripe, and HashiCorp models all converge on this. Engineering deep-dives
with code, design rationale, and honest limitations are the highest-trust content type
for UMRS's target audience. The existing blog post on CUI sign/lock is in this direction —
continue this pattern.

### Opinion leaders before audience size
The beachhead is narrow on purpose. Red Hat/NSA/DISA SELinux people, SELinux
userspace maintainers, NIST/CMMC authors are the target. One piece of content that
earns their respect is worth more than 50 general developer posts.

### Postmortems and failures are content gold
Engineers deeply trust organizations that publish what went wrong. UMRS should plan
at least one "what we got wrong and how we fixed it" post in the first quarter of
public content. This is counterintuitive but the E-E-A-T data supports it strongly.

### Launch week concept worth planning
When UMRS goes public, a coordinated "launch week" (Cloudflare innovation week pattern)
creates event-like buzz — 5-7 coordinated posts from different angles hitting in rapid
succession. Worth planning now so content is in draft when the repo goes public.

### The "explain complex systems simply" pattern is our highest-value move
Stripe's signature pattern maps perfectly to UMRS's content challenge. MLS, SELinux,
CMMC, CUI — all are genuinely complex. Content that makes these accessible without
dumbing them down will outperform on both E-E-A-T and SEO.
