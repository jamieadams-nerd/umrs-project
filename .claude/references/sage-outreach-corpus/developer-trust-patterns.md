# Developer Trust Patterns — How Engineers Evaluate Tools

**Source:** Synthesized from Fogel, ESR, Dan Luu, and observed patterns
**Date compiled:** 2026-03-20
**Phase:** 2B — Developer ecosystem behavior

---

## Core Insight

Engineers do not trust marketing. They trust:

1. **Source code** — "Show me the code"
2. **Benchmarks** — "Show me the numbers"
3. **Design transparency** — "Show me why you made these choices"
4. **Failure analysis** — "Show me what went wrong and how you fixed it"

Any content strategy for a technical audience must lead with these, not features or claims.

---

## The Evaluation Funnel

When an engineer encounters a new tool, they typically evaluate in this order:

### Stage 1 — First Impression (< 30 seconds)

- README: Does it explain what this does in one sentence?
- Stars/forks: Social proof (weak signal but noticed)
- Last commit date: Is this actively maintained?
- Language/stack: Is it in my ecosystem?

### Stage 2 — Quick Assessment (5-15 minutes)

- Build from source: Does `cargo build` / `npm install` work?
- Run tests: Do they pass?
- Read the API: Is it well-designed?
- Check dependencies: Are they reasonable or bloated?

### Stage 3 — Deep Evaluation (hours-days)

- Read architecture docs: Do I understand the design?
- Try a real use case: Does it solve my actual problem?
- Read issues/PRs: How does the maintainer respond?
- Check security posture: Has anyone audited this?

### Stage 4 — Adoption Decision

- Can I contribute if I need to?
- Will this be maintained in 2 years?
- What's the migration cost if I need to leave?

---

## Content Types Engineers Trust

| Content Type | Trust Level | Why |
|---|---|---|
| Architecture deep-dive | Very high | Shows thinking, not just results |
| Postmortem / failure analysis | Very high | Honesty about failures = trustworthy |
| "Why we built X" | High | Shows motivation and constraints |
| "Tradeoffs we made" | High | Acknowledges complexity |
| Benchmark with methodology | High | Reproducible evidence |
| Tutorial with real output | Medium-high | Practical, verifiable |
| Feature announcement | Medium | Expected, less informative |
| Comparison with competitors | Low-medium | Often biased, engineers know this |
| Marketing page | Very low | "Best-in-class" = immediate distrust |

---

## Anti-Patterns That Destroy Trust

1. **Overclaiming** — "Enterprise-grade security" with no evidence
2. **Hiding limitations** — engineers will find them and distrust everything else
3. **Vague benchmarks** — "10x faster" without methodology
4. **Corporate voice** — "We're excited to announce" feels inauthentic
5. **No source access** — security tool without source = untestable claims
6. **Ignoring issues** — unanswered GitHub issues signal abandonment
7. **Breaking changes without migration guides** — disrespects users' time

---

## UMRS Application

### Natural strengths:

- Open source with strict quality controls (clippy pedantic, forbid unsafe)
- Compliance annotations are verifiable — readers can check NIST citations
- High-assurance patterns are documented with rationale
- Real SELinux, real kernel, real CUI — not theoretical

### Content opportunities:

- "Why we chose `#![forbid(unsafe_code)]`" — architecture decision with reasoning
- "How TPI parsing prevents supply chain attacks on security contexts" — deep dive
- "What we got wrong in our first posture probe implementation" — postmortem
- "CategorySet: from kernel bitmask to typed Rust" — design transparency
- Benchmark data from real RHEL 10 deployments

### What to avoid:

- Never say "enterprise-grade" or "military-grade"
- Never hide Phase 1 limitations (targeted ≠ enforcement)
- Never compare to commercial tools without methodology
- Never write in corporate voice — write as an engineer to engineers

---

## Actionable Insights for Sage

1. Lead every post with "why" before "what"
2. Include real code and real output in every technical post
3. Acknowledge limitations proactively — it builds more trust than hiding them
4. Link to source code, not just describe it
5. Use first person ("I built this because...") not third person ("UMRS provides...")

## Sources

- [How good corporate engineering blogs are written | Dan Luu](https://danluu.com/corp-eng-blogs/)
- [Producing Open Source Software | producingoss.com](https://producingoss.com/)
- [The Cathedral and the Bazaar | catb.org](http://www.catb.org/~esr/writings/cathedral-bazaar/)
