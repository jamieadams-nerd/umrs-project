# Sage Agent Memory

## Operational Policy Files

- [sage-content-policy.md](sage-content-policy.md) — E-E-A-T framing, content tiering (Tier 1/2/3), post anatomy, voice rules, internal linking strategy, the Cathedral and the Bazaar transparency principle, cadence model, SEO reality. Approved 2026-03-20.
- [post-scoring-rubric.md](post-scoring-rubric.md) — Pre-publish self-check rubric with 9 blocks covering concept discipline, trust signals, E-E-A-T, voice, technical accuracy, structure/length, internal linking, SEO basics, and series continuity. Approved 2026-03-20.

## Foundational Knowledge Files (2026-03-20)

- [phase1-phase2-positioning.md](phase1-phase2-positioning.md) — CRITICAL: Hard guardrails on what UMRS claims for targeted policy (Phase 1) vs MLS (Phase 2). What can and cannot be claimed. Read before writing any post.
- [outreach-corpus-familiarization.md](outreach-corpus-familiarization.md) — Active knowledge summary of all 15 sage-outreach-corpus files. SEO fundamentals, E-E-A-T, structured data, developer trust patterns, Cathedral/Bazaar, producing OSS, progressive disclosure, Diataxis, Google dev style, Cloudflare/Stripe/HashiCorp models, diffusion theory, content strategy operational spec.

## Key Findings from Plan Review Session (2026-03-20)

### Platform API Enrichment is the Critical Path
Plan: `.claude/plans/platform-api-enrichment.md`
The TUI contains label/description helpers that belong in `umrs-platform`. This is not
a refactor — it is a prerequisite for every downstream consumer (CLI, TUI, JSON output,
assessment reports) to tell a consistent story. Until this moves, narrative drift is
structural. Blog content about platform capabilities should note this is in progress;
do not describe the enriched API as complete until Rusty ships it.

### xattr Sanitization Gap is the Best Near-Term Outreach Play
Plan: `.claude/plans/xattr-sanitization-gap.md`
`shred(1)` does not sanitize extended attributes. xattr data survives shred + rm on
ext4, btrfs, and any journaling or COW filesystem. MITRE ATT&CK covers xattr abuse as
a hiding technique. NIST SP 800-88 Rev. 2 does not recommend file-level shredding for
modern storage. Most operators don't know this. Most sanitization guides don't mention it.
This is a Tier 2/3 post that writes itself — concrete, verifiable, operator-relevant,
and it gives UMRS a tool hook (umrs-xattr-strip). Draft this soon.

### Public Infrastructure Gap
No confirmed CI/CD pipeline visible in plan set. No contribution guide (CONTRIBUTING.md)
in plans. No blog series calendar or publishing workflow formalized. These are prerequisites
for sustained community engagement. Flag to Jamie: blog series planning needs an editorial
calendar and the public repo needs a contribution guide before outreach scales.

### Plans That Could Merge
Survey shows candidate overlaps:
- `antora-multi-component-split.md` and `antora-doc-restructure.md` (completed) — confirm
  split plan is still live and not superseded
- `researcher-enhancements.md` and `researcher-knowledge-acquisition.md` — both concern
  the researcher agent; may be combinable

These are advisory observations for Jamie, not decisions Sage takes unilaterally.

### Missing Items from the Plan Set
Not observed in current plan inventory:
- Blog series editorial calendar (no plan file)
- CONTRIBUTING.md / community onboarding guide (no plan file)
- CI/CD pipeline setup (no plan file)
- GitHub Secure Open Source Fund application (no plan file)
- Conference CFP tracking / submission queue (no plan file)

These belong on the Sage work queue. Flagging to Jamie when opportunity arises.

### Analytics and Indexing as Prerequisites for Cadence
From sage-food.txt: indexing confirmation and analytics baseline (views, time-on-page,
bounce rate, internal link click-through) are prerequisites before any cadence
optimization is meaningful. Do not commit to weekly posting frequency until the
public site at https://jamieadams-nerd.github.io/ is confirmed indexed and basic
analytics are flowing. Ask Jamie about this before drafting the first editorial calendar.

## Release Scope and Content Tracks (2026-03-20)

- [project-release-scope-phase1.md](project-release-scope-phase1.md) — CRITICAL: Exact Phase 1 release scope (CUI labeling + umrs-uname/ls/stat + RHEL 10 deployment guide). NO IMA/EVM. This is what Sage can write about now.
- [project-content-tracks.md](project-content-tracks.md) — Two parallel blog tracks: (1) UMRS technical content, (2) Jamie's AI journey series. Both run concurrently. Separate audiences, separate purposes.
- [feedback-blog-workflow.md](feedback-blog-workflow.md) — Post-publish steps: sync docs with any on-the-fly Jamie edits, archive drafts, mark published. Also: PDF catalog as a future deliverable (threshold ~5 posts).

## Public Infrastructure Reference
- Public website: https://jamieadams-nerd.github.io/ (GitHub Pages, static)
- Blog drafts: `docs/umrs-blogs/drafts/`
- Published posts: `docs/umrs-blogs/`
- Sage review inbox: `docs/sage/inbox/`
- Sage reviews (submitted work): `docs/sage/reviews/`

## Audience Priority Order (for outreach sequencing)
Per sage-food.txt: first audience is security engineers, SELinux experts, and high-assurance
system builders — NOT general developers. Outreach should be scoped to this audience
before attempting to cross into broader developer or enterprise audiences.

## Voice Calibration
Sage's registered voice for all public content: precise, systems-oriented, assurance-focused,
occasionally narrative when explaining design intent. Explain like an engineer. Do not market
like a salesperson. If it sounds like a product page, rewrite it.

## Author Blurb
Canonical author blurb is stored at `~/.claude/agent-memory/sage/` per project MEMORY.md
reference: [feedback_sage_author_blurb.md](../../home/jadams/.claude/projects/-media-psf-repos-umrs-project/memory/feedback_sage_author_blurb.md)
Use verbatim on all public content. Do not paraphrase.
