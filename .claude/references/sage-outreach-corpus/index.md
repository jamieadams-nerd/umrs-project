# Sage Outreach Corpus — Index

**Collection:** sage-outreach-corpus
**Date compiled:** 2026-03-20
**RAG collection name:** `sage-outreach-corpus`
**Location:** `.claude/reference/sage-outreach-corpus/`

---

## Phase 2A — SEO & Web Discovery

| File | Description |
|---|---|
| `seo-fundamentals.md` | How Google Search works: crawling, indexing, ranking, Core Web Vitals, internal linking |
| `structured-data.md` | JSON-LD Schema.org markup for blog posts, software projects, and documentation |
| `eeat-framework.md` | Experience, Expertise, Authoritativeness, Trust — Google's quality framework and how UMRS maps to it |

## Phase 2B — Developer Ecosystem Behavior

| File | Description |
|---|---|
| `cathedral-bazaar.md` | ESR's open source development models; cathedral-core/bazaar-periphery hybrid for security projects |
| `producing-oss.md` | Fogel's guide to project trust, governance, contributor experience, and release management |
| `developer-trust-patterns.md` | How engineers evaluate tools: source code, benchmarks, transparency, failure analysis |

## Phase 2C — Technical Writing Science

| File | Description |
|---|---|
| `progressive-disclosure.md` | Information scent, cognitive load theory, Write the Docs principles |
| `google-dev-style.md` | Key style principles from Google's developer documentation guide |
| `diataxis.md` | The four content types (tutorial, how-to, explanation, reference) and how UMRS maps to them |

## Phase 2D — Technical Branding Models

| File | Description |
|---|---|
| `cloudflare-model.md` | Architecture deep-dives, postmortems, technology explainers — Cloudflare's content patterns |
| `stripe-model.md` | Explaining complex systems simply, infrastructure trust, calm authority voice |
| `hashicorp-model.md` | "Inside the system" articles, security-as-architecture framing, open source → enterprise bridge |

## Phase 2E — Information Diffusion & Adoption

| File | Description |
|---|---|
| `diffusion-of-innovations.md` | Rogers' adoption curve, Moore's chasm, beachhead strategy, opinion leaders |

## Phase 2F — Synthesis

| File | Description |
|---|---|
| `content-strategy-operational-spec.md` | **Sage's operational playbook**: content tiering, post anatomy, paragraph engineering, voice calibration, Phase 1/2 guardrails, quality checklist |

---

## Concept Map

### By UMRS Application

- **Blog strategy:** content-strategy-operational-spec, developer-trust-patterns, cloudflare-model
- **SEO/discovery:** seo-fundamentals, structured-data, eeat-framework
- **Audience targeting:** diffusion-of-innovations, developer-trust-patterns
- **Documentation design:** diataxis, progressive-disclosure, google-dev-style
- **Voice and tone:** stripe-model, cloudflare-model, content-strategy-operational-spec
- **Community building:** cathedral-bazaar, producing-oss, diffusion-of-innovations
- **Trust building:** eeat-framework, developer-trust-patterns, hashicorp-model

---

## Ingest Instructions

```bash
cd /media/psf/repos/umrs-project/.claude/rag
RAG_CHROMA_PATH=/media/psf/repos/ai-rag-vdb/chroma python3 ingest.py --collection sage-outreach-corpus
```
