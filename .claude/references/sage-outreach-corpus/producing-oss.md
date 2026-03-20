# Producing Open Source Software — Karl Fogel

**Source:** https://producingoss.com/ (free online, Creative Commons)
**Date compiled:** 2026-03-20
**Phase:** 2B — Developer ecosystem behavior

---

## Overview

Fogel's comprehensive guide to running open source projects, covering governance, communication, contributor management, release processes, and project trust. Based on his experience with Subversion and extensive observation of successful/failed projects.

---

## Key Concepts Relevant to UMRS

### 1. Technical Trust Comes First

Contributors evaluate a project's technical foundation before considering contribution. They look at:

- **Code quality**: Is the code well-structured, tested, documented?
- **Architecture clarity**: Can I understand the design without talking to the author?
- **Build reproducibility**: Can I clone, build, and run tests within minutes?
- **Test coverage**: Do tests exist? Do they pass? Are they meaningful?

**UMRS position:** Strong on all four. Strict clippy, comprehensive tests, high-assurance patterns, clear architecture docs. This IS the trust signal.

### 2. Communication Infrastructure

Projects need clear communication channels with low barriers:

- **Issue tracker**: Where bugs and features are discussed (GitHub Issues)
- **Mailing list / forum**: For design discussions (GitHub Discussions)
- **Developer documentation**: How to build, test, contribute
- **Decision records**: Why things are the way they are

**UMRS gap:** Currently no public CONTRIBUTING.md, no GitHub Discussions enabled, no architectural decision records (ADRs) visible to outsiders.

### 3. Governance Clarity

Contributors need to know:

- Who makes decisions?
- How are disagreements resolved?
- What's the path from contributor to maintainer?
- Is there a code of conduct?

For a one-person project transitioning to public, "benevolent dictator" (BDFL) is the natural starting model. Be explicit about it.

### 4. Release Management

- **Predictable release cadence** builds confidence
- **Semantic versioning** communicates stability expectations
- **Changelogs** show that the project is actively maintained
- **Migration guides** for breaking changes show respect for users' time

### 5. The "Bus Factor" Problem

Single-maintainer projects have bus factor = 1. This is a legitimate concern for potential adopters of security-critical software. Mitigation strategies:

- Comprehensive documentation (someone else could maintain it)
- CI/CD automation (reduces maintainer-specific knowledge)
- Clear architecture docs (reduces "only Jamie knows how this works")
- Eventually: attract a co-maintainer

---

## Contributor Experience Design

Fogel emphasizes that the contributor's first experience determines whether they stay:

1. **First clone to first test pass**: Should take < 5 minutes
2. **First issue to first response**: Should take < 48 hours
3. **First PR to first review**: Should take < 1 week
4. **Documentation of "good first issues"**: Labeled issues that newcomers can tackle

**UMRS application:** Before going public, ensure:
- README has clear build instructions
- `cargo test` works out of the box on RHEL 10 (and ideally Ubuntu)
- A few issues labeled "good first issue" exist
- CONTRIBUTING.md explains the development workflow

---

## Actionable Insights for Sage

1. **The project's code quality IS its marketing** — link to source, tests, and CI in every blog post
2. **Architecture transparency builds trust** — publish design rationale, not just features
3. **Contributor experience is content** — a great CONTRIBUTING.md is also a great blog post
4. **Governance clarity prevents drama** — state the BDFL model early and explicitly
5. **Release cadence signals health** — even alpha releases with changelogs show progress

## Sources

- [Producing Open Source Software | producingoss.com](https://producingoss.com/)
- [Producing OSS — Getting Started | Chapter 2](https://producingoss.com/en/getting-started.html)
