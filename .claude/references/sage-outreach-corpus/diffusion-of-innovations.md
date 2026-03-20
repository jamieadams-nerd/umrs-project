# Diffusion of Innovations — Everett Rogers (1962, 5th ed. 2003)

**Source:** Rogers, E.M. "Diffusion of Innovations" + Moore, G. "Crossing the Chasm"
**Date compiled:** 2026-03-20
**Phase:** 2E — Information diffusion & adoption

---

## Rogers' Core Theory

Innovation adoption follows a predictable S-curve distributed across five adopter categories:

| Category | % of Population | Characteristics |
|---|---|---|
| **Innovators** | 2.5% | Risk-tolerant, technically adventurous, need minimal external validation |
| **Early Adopters** | 13.5% | Opinion leaders, visionary, evaluate carefully but adopt quickly |
| **Early Majority** | 34% | Pragmatic, adopt when proven by early adopters, need references |
| **Late Majority** | 34% | Skeptical, adopt when it becomes standard/required |
| **Laggards** | 16% | Tradition-oriented, adopt only when forced |

---

## The Five Attributes of Innovation Adoption

Rogers identified five attributes that predict adoption speed:

1. **Relative advantage** — How much better is this than current practice?
2. **Compatibility** — Does it fit existing workflows, values, and needs?
3. **Complexity** — How hard is it to understand and use?
4. **Trialability** — Can I try it without full commitment?
5. **Observability** — Can I see others using it successfully?

### UMRS Adoption Profile

| Attribute | UMRS Status | Strategy |
|---|---|---|
| Relative advantage | Strong — nothing else does typed CUI labeling on SELinux | Demonstrate through comparison with current practice |
| Compatibility | Mixed — requires SELinux (some orgs resist), Rust ecosystem | Emphasize RHEL 10 native support, standard SELinux toolchain |
| Complexity | High — SELinux + MLS + Rust + compliance is inherently complex | Progressive disclosure; easy entry points; wizard TUI |
| Trialability | Medium — can run in permissive/targeted without commitment | Publish "try it in 10 minutes" tutorial |
| Observability | Low — new project, no public deployments | Blog posts showing real output, screenshots, demos |

---

## Crossing the Chasm (Moore, 1991)

Moore extended Rogers by identifying the critical gap between early adopters and early majority — "the chasm." The chasm exists because:

- Early adopters buy **vision** (they see the future)
- Early majority buys **pragmatism** (they want a complete solution with references)

### How to Cross the Chasm

1. **Pick a beachhead segment** — one narrow use case for one narrow audience
2. **Dominate that segment** — become the obvious choice
3. **Use the beachhead as a reference** — expand from there

### UMRS Beachhead Strategy

**Target segment:** Security engineers at organizations handling CUI on RHEL/SELinux, who:
- Already use SELinux (not fighting adoption resistance)
- Need to demonstrate NIST/CMMC compliance
- Are frustrated by the gap between policy requirements and actual enforcement

**Beachhead value proposition:** "UMRS gives you typed, verifiable CUI labeling on your existing SELinux infrastructure — with compliance mapping you can show an auditor."

**Expansion path:**
1. Beachhead: SELinux security engineers → CUI labeling + posture
2. Adjacent: Linux security teams → kernel security posture tools
3. Broader: Compliance teams → assessment and reporting
4. Future: MLS environments → full enforcement

---

## Opinion Leaders and Network Effects

Rogers found that **opinion leaders** (people others look to for technical guidance) are the critical adoption accelerator. They are:

- Not necessarily innovators (they're often early adopters)
- Respected for their judgment, not their risk-taking
- More integrated into social/professional networks than innovators

### For UMRS:

Opinion leaders in the target space:
- Senior SELinux engineers at Red Hat, NSA, DISA
- Maintainers of SELinux userspace tools
- Security-focused Linux kernel contributors
- Authors of NIST/CMMC guidance
- Conference speakers at SELinux/security events

Content strategy for opinion leaders:
- Technical depth that earns respect (not marketing)
- Invitation to review/critique (not just consume)
- Acknowledgment of prior work (SELinux heritage, NIST framework)
- Contributions back to the ecosystem (tools, documentation, patches)

---

## Communication Channels

Rogers identified two types of communication channels:

1. **Mass media** — creates awareness (blog posts, conference talks, social media)
2. **Interpersonal** — persuades adoption (code review, direct conversation, mentoring)

Both are necessary. Mass media alone creates awareness without adoption. Interpersonal alone reaches too few people.

**UMRS application:** Blog posts and conference talks create awareness. GitHub interactions, code reviews, and direct engagement with early adopters drive actual adoption.

---

## Actionable Insights for Sage

1. **UMRS is currently targeting Innovators and Early Adopters** — content should speak to vision and technical depth, not pragmatic completeness
2. **The beachhead is SELinux + CUI on RHEL** — don't dilute by targeting general Linux or general security
3. **Opinion leaders matter more than audience size** — one respected SELinux engineer who endorses UMRS is worth 10,000 random visitors
4. **Trialability is UMRS's weakest attribute** — create a "try it in 10 minutes" experience as early as possible
5. **Observability is critical** — every blog post with real output, real screenshots, real data improves observability

## Sources

- Rogers, E.M. (2003). *Diffusion of Innovations*, 5th Edition. Free Press.
- Moore, G.A. (2014). *Crossing the Chasm*, 3rd Edition. Harper Business.
