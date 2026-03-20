# E-E-A-T Framework — Experience, Expertise, Authoritativeness, Trust

**Source:** Google Search Quality Evaluator Guidelines (2024)
**Date compiled:** 2026-03-20
**Phase:** 2A — SEO & web discovery fundamentals

---

## What E-E-A-T Is

E-E-A-T is Google's framework for evaluating content quality. It is NOT a direct ranking factor — it is a set of criteria used by human quality raters to calibrate Google's algorithms. The signals that correlate with high E-E-A-T ratings do influence ranking.

The four dimensions:

| Dimension | What it means | How Google assesses it |
|---|---|---|
| **Experience** | Author has firsthand experience with the topic | Personal anecdotes, original data, screenshots, demos |
| **Expertise** | Author has knowledge or skill in the subject area | Credentials, depth of coverage, technical accuracy |
| **Authoritativeness** | Author/site is recognized as a go-to source | External citations, backlinks from authoritative domains |
| **Trust** | Content is accurate, honest, safe, and reliable | Transparency, source citations, site reputation |

**Trust is the most important dimension.** Google's guidelines explicitly state that trust is the foundation — a page can have high expertise but low trust if it's deceptive.

---

## YMYL Classification

"Your Money or Your Life" — topics where bad information can cause real harm. Google applies stricter E-E-A-T standards to YMYL topics. Security software and system administration guidance falls into YMYL territory because:

- Bad security advice can lead to data breaches
- Incorrect SELinux configuration can render systems unbootable
- CUI mishandling has legal/regulatory consequences

**UMRS implication:** All UMRS content is implicitly YMYL. Google holds it to a higher standard. This is actually an advantage — UMRS's depth and rigor naturally satisfy YMYL requirements that shallow content cannot.

---

## How UMRS Scores on E-E-A-T

### Experience — STRONG

- Jamie Adams is building the system — firsthand experience is inherent
- Blog posts showing actual output, real configurations, real problems = high experience signal
- "Here's what happened when we ran this on RHEL 10" > "Here's what the docs say"

### Expertise — VERY STRONG

- Deep SELinux knowledge demonstrated through implementation, not just description
- Rust code with security annotations, compliance mappings, high-assurance patterns
- Cross-referencing NIST, CMMC, NSA RTB — demonstrates domain expertise
- The codebase itself is evidence of expertise

### Authoritativeness — BUILDING

- Currently low — new project with limited external citations
- Strategy to build: publish consistently, get cited by SELinux/security communities
- GitHub stars, forks, and contributors are authority signals
- Conference talks, podcasts, guest posts on established security blogs

### Trust — STRONG FOUNDATION

- Open source = transparent
- Compliance annotations = accountable
- Version-controlled documentation = traceable
- Named author with public profile = identifiable

---

## Content Strategies That Build E-E-A-T

### Do:

1. **Show the work** — include code, output, configuration examples
2. **Cite authoritative sources** — link to NIST publications, kernel documentation
3. **Name the author** — "Jamie Adams" with a consistent bio and profile page
4. **Admit limitations** — "Phase 1 does not enforce; it ensures awareness" (this builds trust more than overclaiming)
5. **Update content** — add `dateModified` when content is refreshed
6. **Demonstrate depth** — one thorough post outweighs five shallow ones
7. **Cross-link** — blog → docs → code → references creates a trust network

### Don't:

1. Overclaim capabilities (kills trust)
2. Write anonymous content (kills expertise signal)
3. Publish without sources (kills authoritativeness)
4. Use marketing language ("revolutionary", "best-in-class") — engineers distrust it

---

## Actionable Insights for Sage

- UMRS is naturally high E-E-A-T if content is honest about Phase 1 vs Phase 2 capabilities
- Every post needs: named author, date, sources, real examples
- Admitting what UMRS does NOT do (enforcement in targeted policy) actually builds trust
- The compliance annotation practice in the codebase is itself an E-E-A-T signal — surface it in content
- Build an author page for Jamie at jamieadams-nerd.github.io/about with credentials and project role

## Sources

- [Google Search Quality Evaluator Guidelines (PDF)](https://services.google.com/fh/files/misc/hsw-sqrg.pdf)
- [E-E-A-T and Google Search | Google Search Central](https://developers.google.com/search/docs/fundamentals/creating-helpful-content)
