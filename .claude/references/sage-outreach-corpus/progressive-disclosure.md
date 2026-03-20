# Progressive Disclosure & Cognitive Load in Technical Documentation

**Source:** Write the Docs community, Nielsen Norman Group
**Date compiled:** 2026-03-20
**Phase:** 2C — Technical writing & documentation science

---

## Progressive Disclosure

Progressive disclosure is an interaction design pattern: present the most important information first, reveal complexity on demand. Applied to documentation:

- **Level 1**: What does this do? (1 sentence)
- **Level 2**: How do I use it? (quick start, 5 minutes)
- **Level 3**: How does it work? (architecture, design rationale)
- **Level 4**: How do I extend/modify it? (API reference, internals)

Each level should satisfy a reader who stops there. Don't force someone who just needs a quick start to read the architecture overview first.

---

## Information Scent

"Information scent" (Pirolli & Card, 1999) describes how users assess whether following a link will lead to useful information. Strong scent = descriptive headings, clear navigation labels, preview text.

Weak scent examples:
- "Getting Started" (with what?)
- "Advanced Topics" (which ones?)
- "Miscellaneous" (catch-all = lost content)

Strong scent examples:
- "Install UMRS on RHEL 10"
- "Configure SELinux MCS Categories for CUI"
- "Interpret the Kernel Security Tab Output"

---

## Cognitive Load Theory

Three types of cognitive load (Sweller, 1988):

1. **Intrinsic** — inherent complexity of the material. SELinux MLS is inherently complex. You can't eliminate this.
2. **Extraneous** — load from poor presentation. Confusing layout, inconsistent terminology, unnecessary jargon. This is what documentation can fix.
3. **Germane** — load from building mental models. This is productive learning. Good documentation maximizes germane load by connecting new concepts to familiar ones.

### Reducing extraneous load in UMRS docs:

- One concept per page (Diataxis principle)
- Consistent terminology (same term for same concept — see STE rules)
- Visual hierarchy (headings, code blocks, admonitions)
- Examples before theory (show the output, then explain why)

### Maximizing germane load:

- Analogies to familiar concepts ("CategorySet is like Unix groups, but mandatory and non-discretionary")
- Building from simple to complex ("First understand targeted policy, then MLS")
- Cross-references between related concepts

---

## The 7±2 Rule

Miller's Law: working memory holds 7±2 chunks. Applied to documentation:

- Navigation menus: 5-9 top-level items max
- Procedure steps: if > 7 steps, break into sub-procedures
- Parameter lists: group into categories if > 7
- Concept introductions: define ≤ 5 new terms per page

---

## Write the Docs Principles

Key principles from the Write the Docs community:

1. **Docs are a product** — they need design, testing, and iteration
2. **Write for the reader, not the writer** — organize by task, not by architecture
3. **Test your docs** — have someone follow procedures without help
4. **Keep docs close to code** — docs that live near code get updated with code
5. **Automate what you can** — API docs from code, spell checking, link checking

---

## Actionable Insights for Sage

- Blog posts should follow progressive disclosure: hook → concept → deep explanation → code → limitations
- Navigation labels need strong information scent — tell the reader what they'll find
- Limit new concepts introduced per post — if you need > 5, split into a series
- Always connect new concepts to familiar ones — readers need anchors
- Reduce extraneous load: consistent terminology, clear visuals, one topic per post

## Sources

- [Write the Docs — Documentation Guide](https://www.writethedocs.org/guide/)
- [Progressive Disclosure | Nielsen Norman Group](https://www.nngroup.com/articles/progressive-disclosure/)
- [Information Foraging | Pirolli & Card (1999)](https://dl.acm.org/doi/10.1145/302979.303030)
