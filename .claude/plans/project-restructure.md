---
name: Project Restructure — Multi-Repo / Publishing Strategy
agent: none — Jamie decision
status: not started — decision pending
depends-on: none (but blocks M4 Public Release)
---

# Project Restructure

## Problem

UMRS started as a single MLS labeling library but has grown into four distinct
concerns serving four audiences:

| Concern | Audience | Current Location |
|---|---|---|
| Libraries | Rust developers | `components/rusty-gadgets/` (single workspace) |
| Patterns & Education | Developers learning HA | `docs/modules/patterns/` + inline in crate code |
| Tools | Security operators | `components/rusty-gadgets/` (mixed with libs) |
| Compliance & Assessment | Auditors | planned (`umrs-assess`), `refs/`, `.claude/` |

Everything lives in one monorepo. That worked early on but creates friction for:
- crates.io publishing (versioning, independent releases)
- Public documentation (GitHub Pages, API docs)
- Contributor onboarding (too much to navigate)
- CI/CD (one repo = one pipeline for everything)
- Team workflow (agents, skills, CLAUDE.md all assume monorepo)

## Decision Required

How should the project be structured going forward?

### Option A — Multi-repo ecosystem
```
umrs-platform/        ← foundational crate (crates.io)
umrs-selinux/         ← SELinux/MLS modeling (crates.io)
umrs-core/            ← shared utilities (crates.io)
umrs-tools/           ← CLI tools (ls, state, logspace, assess)
umrs-patterns/        ← pattern library & education
umrs-docs/            ← Antora site, deployment guides, compliance refs
```

### Option B — Monorepo with clear internal boundaries
```
umrs-project/
  crates/             ← publishable libraries
  tools/              ← CLI binaries
  docs/               ← Antora
  patterns/           ← HA pattern library
  refs/               ← standards
```

### Option C — Hybrid (2-3 repos)
```
umrs-libs/            ← library crates + patterns (crates.io workspace)
umrs-tools/           ← CLI tools + assessment engine
umrs-docs/            ← documentation site + compliance refs
```

### Option D — Something else
Jamie may have a different structure in mind.

## Impact Assessment (must be completed before deciding)

- [ ] crates.io publishing workflow — how does each option affect versioning and releases?
- [ ] GitHub Pages / API docs — where do built docs live? One site or per-repo?
- [ ] CI/CD pipeline design — one pipeline or per-repo? Matrix builds for RHEL + Ubuntu?
- [ ] CLAUDE.md and agent configuration — currently assumes single repo root
- [ ] Skills and hooks — currently assume monorepo paths
- [ ] Cross-crate development workflow — how do devs work across crates during development?
- [ ] `refs/` and `.claude/references/` — where do compliance materials live?
- [ ] git history — how to preserve history if splitting repos?
- [ ] Dependency management — workspace deps become crates.io deps in multi-repo

## Not Blocking Current Work

This decision does NOT block M1 (Solid Foundation) or current development. All
active plans (posture probe, CPU corpus, security-auditor corpus, assessment engine)
can proceed in the current monorepo structure.

This decision DOES block M4 (Public Release) — specifically crates.io publishing,
GitHub Pages, and contribution guidelines.

## Phased Approach (Jamie's direction)

Don't try to restructure everything at once. Move in phases:

1. **Documentation structure** — separate docs concerns, set up GitHub Pages, Antora publishing
2. **Code workspace location** — move crates out of `components/rusty-gadgets/` if needed
3. **Repository splitting** — if multi-repo, split one crate at a time (start with `umrs-platform`?)
4. **crates.io publishing** — publish after repos are stable and CI is running
5. **Agent/skill migration** — update CLAUDE.md, agent configs, paths, skills last

Each phase is independently valuable. Can stop at any phase and still have improved things.

---

## DO NOT RESTRUCTURE ANYTHING

This plan is a decision document. No repo splitting, no directory moves, no
workflow changes until Jamie has decided and a migration plan exists.
