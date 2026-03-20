# Guest Admin Blog Review — "Your CUI Policy Is a Sign. Here Is How to Build the Lock."

**Date:** 2026-03-20
**Reviewer:** RHEL sysadmin persona, 10+ years, SELinux daily user (mostly permissive-fixer), CUI-aware but not CUI-expert
**Post reviewed:** `docs/sage/blogs/blog-cui-sign-lock.md`

---

## First Impressions (first 30 seconds)

**Title:** Got me immediately. "Your CUI Policy Is a Sign" — I have been in that meeting. The sign/lock framing was clear before I read the first word of body text.

**First paragraph:** The FOUO/SBU/LES soup is accurate history, and the author places themselves as someone who was actually there. That matters. By the end of the first paragraph I had decided this was written by someone who knows the subject.

**Would I keep reading?** Yes. I read it straight through. That is not my default behavior with security blog posts.

---

## Clarity

**What worked:** The policy history, the sign/lock metaphor throughout, the "kernel is always right" section (personally felt that one — I have wasted hours on the config-says-enforcing-but-kernel-says-no problem), Bell-LaPadula summary, the CUI-is-horizontal argument. The Ada/Rust compiler analogy at the end landed hard.

**Where I got lost:**

- **Two-Path Independence (TPI):** Defined in a parenthetical crammed into a paragraph with four other concepts. I re-read that paragraph twice. Needs a sentence break or callout.
- **CategorySet:** Used as a known term before being explained. Minor.
- **The trust pipeline (umrs-hw, umrs-platform, umrs-selinux):** Component names with no indication of which ones an operator actually interacts with versus which are internal library layers. Are these CLI tools? Daemons? Background services? The post does not say.
- **`#![forbid(unsafe_code)]`:** The author explains it for non-Rust readers, good. But the explanation comes after a sentence that already assumes you know what `unsafe` means. Small ordering issue.

**Technical depth:** Right for a sysadmin audience. Deeper than marketing, lighter than a white paper.

---

## Relevance to My Work

The "kernel is always right" section is immediately applicable. I will check `/sys/fs/selinux/enforce` versus `/etc/selinux/config` tomorrow morning.

CUI specifically: not my daily reality yet, but the post convinces me it will be. "Your CUI Policy Is a Sign" describes exactly where my organization is right now.

**Would I forward to my team:** Yes. With the note: "Read this before someone asks us about CMMC compliance."

**Would I forward to my CISO:** Maybe. The policy history and CMMC 2.0 sections would land. The TPI/CategorySet details would not. A CISO version would be shorter and heavier on audit risk.

**Does it make me want to try UMRS:** It makes me want to look at the GitHub repo. Whether I run anything depends on setup experience, which the post does not show.

---

## Trust

**Does the author sound credible:** Yes. Specific signals that worked: the FOUO/SBU history is accurate, the footnoted references are real documents, and most importantly — admitting Phase 2 is not done is the single most credible sentence in the post. "I will not tell you Phase 2 is done when it is not." That anchors everything else.

**Claims backed up:** Historical and regulatory claims, yes. UMRS technical claims: described but not demonstrated. I cannot see the tool run. The claims are plausible but unverified.

**Overselling moments:** "An onsite security officer telling them exactly how to handle each resource" — evocative, but whether a parsed JSON catalog actually fills that role depends on the operator interface, which I have not seen. The "fifty years of engineering honesty starts here" line is the most self-congratulatory sentence in the post. Working sysadmins raise an eyebrow at that.

---

## Actionability

**What I got:** GitHub link (bottom of post), three source artifacts to look at, a path to an 800-171 mapping document.

**What I did not get:**
- A command to run
- A package name or `dnf` command
- RHEL version requirements (RHEL 10 mentioned once; RHEL 9 fleet coverage: unknown)
- What I see when it works
- How long setup takes
- Whether this needs root or a custom SELinux policy

The "What You Can Do Right Now" section points me at source files and a Rust crate. As a sysadmin, not a developer, "look at the crate" is not an actionable next step. I need a command or a demo.

**What would move me from reading to installing:** One screenshot of tool output. Three commands that produce visible output on RHEL 10. RHEL version requirements stated explicitly.

---

## What Worked

- The sign/lock metaphor — sticky, accurate, I will reuse it
- The kernel-is-always-right section — practical, immediately applicable
- Admitting Phase 2 is not done — the trust anchor for the whole post
- The lineage paragraph — earns the confidence being asked for
- CUI-is-horizontal framing — genuinely reframed something I had not thought about correctly
- The Five Eyes paragraph — shows coalition thinking, relevant for .mil-adjacent systems
- The author blurb with the AI transparency disclosure — transparent, slightly funny, I respect it

---

## What Did Not Work

- No screenshots or output examples — for a tool project, this is table stakes
- "What You Can Do Right Now" is for developers, not sysadmins
- Component names (umrs-hw, umrs-platform, umrs-selinux, umrs-ls) with no indication of which ones an operator touches
- RHEL 10 requirement unstated upfront — most of my fleet is RHEL 9
- The "assurance pendulum" section ran two paragraphs too long for this audience — the Morris Worm/Code Red namedrops are good but the conference-keynote framing around them is not

---

## The Big Question

I am bookmarking the project. I am going to the GitHub repo. But I am bookmarking it as "check back when Phase 2 ships" not "install this today." The gap is entirely the absence of a runnable demo. One command and one screenshot in "What You Can Do Right Now" and I am in the repo right now instead of writing this.

---

## Ratings

| Dimension | Score | Notes |
|---|---|---|
| Relevance to sysadmins | 7/10 | Real problem, RHEL-native, MCS is daily territory. Docked 3 for no runnable demo. |
| Clarity | 8/10 | Best technical blog I have read on this subject. Minor loses on TPI paragraph and component naming. |
| Trust/credibility | 9/10 | Phase 2 admission earns the rest. Minus 1 for "onsite security officer" claim not backed by a demo. |
| Actionability | 4/10 | Biggest weakness. Source files, not commands. No screenshot, no quick-start. |
| Would I share with my team | Yes | With the caveat: not installing today, but we need to understand this problem is coming. |

---

## Summary Recommendation

The post makes me care about UMRS. It does not make me able to do anything about it today. Fix "What You Can Do Right Now": replace the artifact list with three commands that produce visible output on RHEL 10, and add one screenshot. Everything else is strong enough to carry the post on its own.
