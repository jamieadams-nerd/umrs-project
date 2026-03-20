# Sage Outreach & Release Strategy

**Date:** 2026-03-20
**Author:** Jamie Adams (with Claude Code)
**Status:** draft
**ROADMAP Goals:** G7 (Public Project), G10 (AI Transparency & Public Trust), G12 (Documentation Theme & Visual Identity)
**Milestones:** M4 (Public Release), M5 (AI Transparency) — cross-cutting

---

## Executive Summary

This plan tracks the public-facing infrastructure, content strategy, release planning, and
branding work needed to take UMRS from a private development project to a credible public
presence. It consolidates four workstreams that are tightly coupled: you cannot publish
compelling content without infrastructure, you cannot attract contributors without a release
strategy, and none of it lands without a coherent visual identity.

Sage (the senior-tech-writer agent) owns the content strategy workstream. Jamie owns
release and branding decisions. Infrastructure is shared.

---

## Dependencies on Other Plans

| Dependency | Why |
|---|---|
| `antora-multi-component-split` | Hosting strategy depends on final Antora site structure |
| `project-restructure` (long-term) | Repo layout affects CI/CD, crates.io publishing, and contributor experience |
| `doc-quality-plan` | Content must pass quality gates before public promotion |
| `tui-enhancement-plan` | Blog content tied to active development — need features to write about |
| `performance-baseline` | Release readiness requires performance evidence |

**Execution order context:** Per the plan execution order, project-restructure is the
furthest-horizon plan. This outreach plan can proceed on Workstreams 1-2 independently,
but Workstream 3 (Release Strategy) is partially blocked by project-restructure decisions.

---

## Workstream 1: Public Infrastructure

**Owner:** Jamie + infrastructure support
**Depends on:** `antora-multi-component-split` (for doc hosting decisions)

### Phase 1.1 — Analytics for GitHub Pages

- Research analytics options compatible with Jekyll on GitHub Pages
  - Evaluate: Plausible, GoatCounter, Umami (privacy-respecting, no cookie banners)
  - Exclude: Google Analytics (privacy concerns for a security-focused project)
- Select and integrate analytics provider
- Verify analytics work with the existing Jekyll site at jamieadams-nerd.github.io
- Define key metrics: page views, referral sources, top content, geographic distribution

### Phase 1.2 — Antora Documentation Hosting

- Evaluate GitHub Pages for Antora output
  - Constraint: GitHub Pages serves static content; Antora generates static HTML — should work
  - Evaluate: custom domain support, build pipeline integration, deployment frequency limits
- Evaluate alternatives if GitHub Pages has limitations:
  - Netlify: build hooks, preview deploys, custom headers
  - Cloudflare Pages: edge caching, Web Analytics built-in, generous free tier
  - Vercel: similar to Netlify, strong preview deploy story
- Decision criteria: cost (free tier), build integration, custom domain, SSL, indexability
- Document decision and rationale

### Phase 1.3 — SEO & Indexing

- Generate XML sitemap for Jekyll content (jekyll-sitemap plugin)
- Add structured data (JSON-LD) for blog posts: Article schema, author, datePublished
- Register site with Google Search Console
- Submit sitemap to Google and Bing
- Verify robots.txt allows indexing of all public content
- Add canonical URLs to prevent duplicate content issues between blog and docs
- If Antora is hosted separately, ensure cross-site linking does not fragment SEO

### Phase 1.4 — Unified Build Pipeline

- GitHub Actions workflow: build Jekyll site + build Antora docs + deploy both
- Antora playbook configured to pull from the correct branch(es)
- Preview builds on PRs (if hosting platform supports it)
- Build status badges in README

---

## Workstream 2: Content Strategy (Sage-Owned)

**Owner:** Sage (senior-tech-writer agent)
**Depends on:** Active development work (content follows features, not the reverse)

### Phase 2.1 — Blog Series Plan

- Define blog series arc aligned to ROADMAP milestones and active development
- Posts must be tied to what is being built — no orphan topics
- Suggested series alignment:
  - M1 work: high-assurance patterns, SELinux modeling, typed security
  - M2 work: assessment engine, evidence pipelines, CPU probes
  - M3 work: CUI labeling, Five Eyes, MCS translation
  - M4 work: the release itself, contributor onboarding, CI/CD
  - M5 work: AI transparency, how UMRS uses AI agents

### Phase 2.2 — Tier 1 Authority Posts

- Minimum cadence: 1 authority post per month
- Authority post criteria: original research, deep technical content, positions UMRS as a reference
- Candidate topics:
  - "What UMRS Guarantees" — the trust pipeline from hardware to tools
  - xattr research — security observations, provenance verification
  - CategorySet and MCS enforcement — kernel set-theory, non-discretionary access
  - High-assurance Rust patterns — TPI, fail-closed, validate-at-construction
  - CPU security extensions — what they are, how to verify, why they matter
  - AI transparency in security tooling — how we trace claims to sources

### Phase 2.3 — Internal Linking Strategy

- Blog posts serve as entry points for broader audiences
- Every authority post must link to 2+ Antora documentation pages for depth
- Antora docs reference blog posts where they provide accessible introductions
- Create a "Further Reading" pattern: blog post footer links to Antora modules
- Navigation: blog sidebar or footer widget linking to documentation site

### Phase 2.4 — Post Scoring Rubric

- Sage to develop scoring rubric (separate deliverable)
- Rubric evaluates: technical depth, ROADMAP alignment, linking quality, audience reach
- Used to prioritize post backlog and evaluate published post effectiveness

### Phase 2.5 — Cross-Posting Workflow

- Define cross-posting targets: dev.to, r/netsec, r/rust, r/linux, Hacker News
- Establish canonical URL practice (blog is canonical, cross-posts link back)
- Cross-posting checklist:
  - Adapt formatting for target platform
  - Include canonical link
  - Monitor comments and respond (Sage or Jamie)
  - Track referral traffic from each platform (feeds back to Phase 1.1 analytics)

---

## Workstream 3: Release Strategy

**Owner:** Jamie
**Partially blocked by:** `project-restructure` (long-term) for crates.io decisions

### Phase 3.1 — Define Phase 1 and Phase 2 Feature Lists

- **Phase 1 Release** (M1 + early M2):
  - `umrs-selinux` crate: SecurityContext, MLS, CategorySet, SecureDirent
  - `umrs-platform` crate: OS detection, kernel posture, trust tiers
  - `umrs-ls` tool: security-enriched directory listing (CLI + TUI)
  - `umrs-state` tool: system state introspection (prototype → functional)
  - Documentation: getting-started guide, pattern library, API docs
  - AI transparency module published
- **Phase 2 Release** (M2 + M3):
  - Assessment engine v1
  - CPU extension detection
  - CUI labeling tools
  - OSCAL export
  - `umrs-logspace` tool
  - Event viewer
- Each phase must have concrete acceptance criteria, not aspirational feature lists

### Phase 3.2 — Versioning Strategy

- Crate versioning: semver, starting at 0.1.0 for initial public release
- When crates get version numbers: at Phase 1 release, not before
- Documentation versioning: Antora supports versioned docs — version docs when crate APIs stabilize
- Pre-1.0 contract: public API may change; document stability expectations per crate
- Version bump policy: patch for fixes, minor for features, major for breaking changes

### Phase 3.3 — Release Schedule

- Map features to versions with target dates (after Phase 1 feature list is locked)
- Release cadence: quarterly for minor versions, patch releases as needed
- Release checklist: tests pass, clippy clean, docs built, changelog written, crates published

### Phase 3.4 — Contribution Guide

- `CONTRIBUTING.md` at repo root
- Sections: how to report issues, how to submit PRs, coding standards, test requirements
- Reference the clippy policy, test placement rule, and compliance annotation requirements
- Contributor license: decide on CLA or DCO (Developer Certificate of Origin)
- Good first issues: label strategy for onboarding new contributors

### Phase 3.5 — CI/CD Pipeline

- GitHub Actions workflows:
  - `cargo xtask fmt` — formatting check
  - `cargo xtask clippy` — lint check (must be clean, -D warnings)
  - `cargo xtask test` — full test suite
  - Documentation build (Antora + cargo doc)
  - Release automation: tag → build → publish to crates.io
- Branch protection: require CI pass before merge to main
- Dependency audit: `cargo audit` in CI
- MSRV (Minimum Supported Rust Version) policy: document and test

---

## Workstream 4: Branding & Landing Page

**Owner:** Jamie
**Depends on:** G12 (Documentation Theme & Visual Identity)

### Phase 4.1 — Project Identity

- Wizard mascot: finalize design, create SVG assets in multiple sizes
- Color palette: dark theme, terminal green, muted blues — consistent with TUI posture indicators
- Typography: select fonts that communicate precision (monospace for code, clean sans for prose)
- Voice guide: how UMRS sounds in public — technical but approachable, confident but not arrogant
- The identity should make security professionals think "these people take this seriously"

### Phase 4.2 — GitHub Pages Landing Page

- Redesign landing page at jamieadams-nerd.github.io
- Sections:
  - Hero: what UMRS is, in one sentence, with the wizard
  - What it does: trust pipeline (hardware → platform → SELinux → tools → evidence)
  - Why it matters: CUI operators need evidence, not claims
  - Quick start: how to try it (link to getting-started guide)
  - Blog highlights: latest authority posts
  - About: Jamie's perspective, the project's origin story
- Mobile responsive
- Fast load time (static site, minimal JS)

### Phase 4.3 — Consistent Visual Language

- Apply the G12 Antora theme to documentation site
- Ensure blog theme and documentation theme share visual DNA
- Favicon: wizard mascot
- Social preview images (og:image) for blog posts and documentation
- 404 page: wizard-themed, helpful (link back to home and docs)

---

## Cross-Workstream Dependencies

```
Workstream 1 (Infrastructure)
  Phase 1.2 ──depends on──► antora-multi-component-split
  Phase 1.4 ──depends on──► Phase 1.2 decision (hosting platform)

Workstream 2 (Content)
  Phase 2.1 ──depends on──► Active development milestones (M1-M3)
  Phase 2.3 ──depends on──► Phase 1.2 (need to know where docs live)
  Phase 2.5 ──depends on──► Phase 1.1 (analytics to track referrals)

Workstream 3 (Release)
  Phase 3.1 ──depends on──► ROADMAP milestone definitions being stable
  Phase 3.5 ──depends on──► project-restructure (repo layout)
  Phase 3.5 ──depends on──► Phase 1.4 (shared build pipeline)

Workstream 4 (Branding)
  Phase 4.2 ──depends on──► Phase 4.1 (need identity before redesign)
  Phase 4.3 ──depends on──► G12 doc theme plan
  Phase 4.3 ──depends on──► Phase 1.2 (hosting platform for docs)
```

---

## What Can Start Now

Even though some phases are blocked, several can begin immediately:

1. **Phase 1.1** — Analytics research requires no dependencies
2. **Phase 1.2** — Hosting evaluation can proceed in parallel with the multi-component split
3. **Phase 2.1** — Blog series planning is always unblocked (Sage can draft the arc)
4. **Phase 2.2** — Authority posts can be written and published now
5. **Phase 2.4** — Scoring rubric is self-contained
6. **Phase 3.1** — Feature list definition is a planning exercise
7. **Phase 4.1** — Identity work is creative and has no technical blockers

---

## Success Criteria

- GitHub Pages site has working analytics showing traffic trends
- Antora documentation is publicly accessible and indexed by search engines
- At least 3 authority blog posts published before Phase 1 release
- Blog posts link to Antora docs; Antora docs link back
- Phase 1 release has a concrete, testable feature list
- CI/CD pipeline runs on every PR and blocks broken code
- Landing page communicates what UMRS is to a security professional in under 30 seconds
- Visual identity is consistent across blog, docs, and README

---

## Open Questions

- [ ] Analytics provider: Plausible vs GoatCounter vs Umami?
- [ ] Hosting: GitHub Pages for everything, or split blog (GitHub Pages) and docs (Netlify/Cloudflare)?
- [ ] Contributor license: CLA or DCO?
- [ ] Cross-posting: who monitors community responses — Sage or Jamie?
- [ ] Branding: commission custom wizard art, or iterate on generated assets?
- [ ] Release timing: is Phase 1 tied to a date, or "when ready"?
