# The Cathedral and the Bazaar — Eric S. Raymond (1997)

**Source:** http://www.catb.org/~esr/writings/cathedral-bazaar/
**Date compiled:** 2026-03-20
**Phase:** 2B — Developer ecosystem behavior

---

## Overview

ESR's seminal essay on open source development models, written from his experience with fetchmail and observations of Linux kernel development. Contrasts two development models:

- **Cathedral**: Small group designs in isolation, releases polished product. (GNU Emacs, GCC pre-EGCS)
- **Bazaar**: Open development with many contributors, frequent releases, rapid iteration. (Linux kernel)

The essay's core argument: given enough eyeballs, all bugs are shallow ("Linus's Law").

---

## The 19 Lessons (Selected Key Ones)

1. Every good work of software starts by scratching a developer's personal itch
2. Good programmers know what to write. Great ones know what to rewrite (and reuse)
3. Plan to throw one away; you will, anyhow (from Brooks)
5. When you lose interest in a program, your last duty is to hand it off to a competent successor
6. Treating your users as co-developers is your least-hassle route to rapid code improvement
7. Release early. Release often. And listen to your customers
8. Given a large enough beta-tester and co-developer base, almost every problem will be characterized quickly and the fix obvious to someone ("Linus's Law")
9. Smart data structures and dumb code works a lot better than the other way around
10. If you treat your beta-testers as if they're your most valuable resource, they will respond by becoming your most valuable resource
11. The next best thing to having good ideas is recognizing good ideas from your users
12. Often, the most striking and innovative solutions come from realizing that your concept of the problem was wrong
18. To solve an interesting problem, start by finding a problem that is interesting to you

---

## UMRS Application

### Where UMRS is Cathedral-like (by design):

- **Security architecture requires careful design** — you cannot bazaar your way to a correct MLS implementation
- **Compliance annotations require expertise** — NIST/CMMC mapping is not crowdsourceable
- **High-assurance patterns need deliberate engineering** — TPI parsing, fail-closed defaults

### Where UMRS should be Bazaar-like:

- **Documentation** — contributors can improve docs, add examples, fix typos
- **Tool UX** — user feedback on CLI output, error messages, TUI layout
- **Posture signals** — community members can propose new indicators for different platforms
- **Testing** — more platforms, more configurations = more coverage

### The hybrid model for security projects:

Cathedral the core (security architecture, type system, enforcement logic).
Bazaar the periphery (docs, tools, platform support, examples).

This is actually how Linux kernel security modules work — the LSM framework is cathedral, individual policy modules accept broader contribution.

---

## Trust-Building Lessons for Sage

1. **"Scratching a personal itch"** — Jamie's motivation story IS the marketing. Engineers trust projects born from real needs.
2. **"Release early, release often"** — publish blog posts about work in progress, not just finished features. Show the journey.
3. **"Treat beta-testers as co-developers"** — early adopters who file issues and contribute fixes become advocates.
4. **"Smart data structures and dumb code"** — UMRS's typed security primitives (SecurityContext, CategorySet, MlsLevel) are the story. The types encode the security model.

---

## Key Insight

The essay assumes that more contributors = better software. For security-critical systems, this is only true at the periphery. The core security architecture must be cathedral — designed by experts, reviewed rigorously, proven correct. UMRS's approach (expert core + community periphery) is the right hybrid for a security project.

## Sources

- [The Cathedral and the Bazaar | catb.org](http://www.catb.org/~esr/writings/cathedral-bazaar/)
- [Homesteading the Noosphere | catb.org](http://www.catb.org/~esr/writings/homesteading/)
