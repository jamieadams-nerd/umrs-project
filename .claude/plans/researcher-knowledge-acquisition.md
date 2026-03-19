# Plan: Researcher Knowledge Acquisition — Information Theory & Sage Outreach Corpus

**Status:** On hold — saved for future execution.

**ROADMAP alignment:** G10 (AI Transparency & Outreach), G9 (Documentation Quality)

**Primary executor:** researcher
**Supporting agents:** sage (consumer of outreach corpus), senior-tech-writer (consumer of writing science material)

---

## Purpose

Acquire, organize, and ingest two distinct research corpora that support UMRS goals:

1. **Information theory & graph search foundations** — canonical papers and texts that underpin
   encoding, search efficiency, vector indexing, and the theoretical basis of UMRS's own
   structured retrieval and labeling systems
2. **Sage outreach knowledge base** — reference material on SEO, developer ecosystem behavior,
   technical branding, content strategy, and information diffusion that enables Sage to produce
   high-impact public content

Both corpora are research-only — no implementation work. The researcher acquires and organizes;
downstream agents consume.

---

## Corpus 1: Information Theory, Graph Theory & Search Foundations

### Why this matters for UMRS

UMRS operates at the intersection of structured data, search, and security labeling.
The theoretical foundations of information theory (entropy, encoding limits, compression)
and graph theory (traversal, constraint solving, spectral methods) directly inform:

- RAG indexing and retrieval efficiency
- Canonical representation strategies (minimal encoding, deduplication)
- Vector search architecture (HNSW, ANN)
- Lattice structures in MLS security models

### Phase 1A — Core canonical texts (acquire and catalog)

| Document | Author(s) | Priority |
|---|---|---|
| "A Mathematical Theory of Communication" (1948) | Claude Shannon | Must-read |
| *Information Theory, Inference, and Learning Algorithms* | David J.C. MacKay | Must-read (free) |
| *Elements of Information Theory* | Cover & Thomas | Reference |
| Minimum Description Length (MDL) | Rissanen | High — connects compression to inference |
| Kolmogorov Complexity | Solomonoff, Chaitin | High — canonical representations, dedup |

### Phase 1B — Graph theory & search foundations

| Document | Author(s) | Priority |
|---|---|---|
| "Paths, Trees, and Flowers" (1965) | Jack Edmonds | Foundational — polynomial-time thinking |
| Graph Theory (1958) | Claude Berge | Foundational |
| Graph structure & embeddings | W.T. Tutte | Structural results |
| Dijkstra's shortest path (1959) | Dijkstra | Classical |
| A* search (1968) | Hart, Nilsson, Raphael | Heuristic search ↔ estimation theory |
| Reverse Search | Avis & Fukuda | Bounded-memory exhaustive search |

### Phase 1C — Modern graph-based search (ANN / vector)

| Document | Author(s) | Priority |
|---|---|---|
| HNSW paper (Hierarchical Navigable Small World Graphs) | Malkov, Yashunin | Must-read — state-of-art ANN |
| Graph-based ANN survey | (locate best current survey) | High — comparative analysis |

### Phase 1D — Cross-discipline bridges

Research the intersections — these are where UMRS-relevant insights emerge:

- **Information theory ↔ graphs:** LDPC codes, Tanner graphs, network information theory
- **Graphs ↔ search:** Heuristic search as graph traversal + estimation; constraint solving as graph pruning
- **Information theory ↔ search:** Entropy-driven search ordering; compression as search space reduction
- **Spectral methods:** Laplacian eigenvalues for clustering and partitioning
- **Analytic combinatorics:** Generating functions for algorithm behavior analysis

### Deliverable

RAG collection: `info-theory-foundations`
- Source documents in `.claude/reference/info-theory-foundations/`
- Ingested via `rag-ingest` skill
- Familiarization pass via `corpus-familiarization` skill
- Catalog summary in researcher agent memory

---

## Corpus 2: Sage Outreach Knowledge Base

### Why this matters for UMRS

UMRS has serious engineering depth but limited public visibility. Sage translates
high-assurance system design into public trust signals. To do this effectively,
Sage needs grounding in how technical content is discovered, consumed, and trusted
by security engineers and developers.

### Phase 2A — SEO & web discovery fundamentals

| Source | What to acquire | Priority |
|---|---|---|
| Google Search Central | Indexing, ranking signals, structured data guides | Must |
| Schema.org | JSON-LD schemas for articles, software projects | Must |
| "How Search Works" (Google) | Overview document | Must |
| "Search Quality Evaluator Guidelines" (Google) | E-E-A-T framework — Experience, Expertise, Authority, Trust | Must — directly aligns with UMRS positioning |
| W3C | HTML semantics, accessibility (affects SEO + UX) | Reference |

Key topics to cover: crawlability, canonical URLs, Core Web Vitals, structured data, internal linking.

### Phase 2B — Developer ecosystem behavior

| Source | What to acquire | Priority |
|---|---|---|
| "The Cathedral and the Bazaar" | Eric S. Raymond | Foundational — community formation |
| "Producing Open Source Software" | Karl Fogel | Foundational — project trust patterns |
| Developer trust patterns | How engineers evaluate tools: source code, benchmarks, design transparency, failure analysis | Research note |

Key insight: Engineers do not trust marketing. They trust source code, benchmarks,
design transparency, and failure analysis. Content that works: "Why we built X",
"Tradeoffs we made", "Here's where this breaks."

### Phase 2C — Technical writing & documentation science

| Source | What to acquire | Priority |
|---|---|---|
| Write the Docs community guides | Progressive disclosure, information scent, cognitive load | High |
| Google Developers Documentation Style Guide | Style reference | Reference |
| Diataxis framework | Taxonomy: tutorial, how-to, explanation, reference | Already partially in doc-arch skill |

### Phase 2D — Technical branding models

Study these companies' engineering blogs for voice, structure, and trust-building patterns:

- **Cloudflare** — architecture deep dives, transparency, postmortems
- **Stripe** — explaining complex systems simply, developer enablement
- **HashiCorp** — infrastructure trust, "inside the system" articles

Capture: what content types they produce, structural patterns, how they link blog → docs.

### Phase 2E — Information diffusion & adoption

| Source | What to acquire | Priority |
|---|---|---|
| "Diffusion of Innovations" | Everett Rogers | Core theory — early adopters, opinion leaders, crossing the chasm |

Key application: UMRS's first audience is security engineers, SELinux experts, and
high-assurance system builders — not general developers. Content strategy must target
this narrow audience first.

### Phase 2F — Content strategy operational spec

Distill from research into an actionable Sage content policy covering:

- **Content tiering:** Tier 1 authority posts (2k–4k words, 1–2/month), Tier 2 engineering posts (1k–2k, weekly), Tier 3 field notes (300–800, biweekly)
- **Post anatomy:** Hook → concept → deep explanation → example → limitations → closing → references
- **Structural constraints:** 1 primary concept per post; if a section could stand alone, defer it
- **Paragraph engineering:** 3–5 sentences, 60–120 words; sentence target 12–20 words, max 30
- **Internal linking:** Every post links 2–5 UMRS docs; blog = entry point, docs = deep reference
- **Series strategy:** Multi-part series for complex topics instead of bloated single posts
- **Voice calibration:** Explain like an engineer, not a salesperson; confidence without exaggeration
- **Publishing cadence:** 1 primary post/week; never sacrifice depth for frequency

### Deliverable

RAG collection: `sage-outreach-corpus`
- Source documents in `.claude/reference/sage-outreach-corpus/`
- Ingested via `rag-ingest` skill
- Familiarization pass for Sage agent
- Content policy document derived from Phase 2F → saved to `docs/sage/` for Sage's use

---

## Corpus 3: Researcher Capability Growth — Skills & Automation

### Why this matters

The researcher's current strength is acquisition — finding and ingesting material. The next
level is making it the team's **always-current knowledge service**. This section defines skills
and automation that multiply the researcher's value to every other agent.

The theme: **maintain the corpus, not just build it.**

### Phase 3A — Corpus ingestion & lifecycle skills

| Skill | Purpose |
|---|---|
| **Version checker** | Given `refs/manifest.md`, hit source URLs and report which tracked documents have newer versions (NIST revisions, STIG updates, kernel doc changes) |
| **Structured extraction** | Take a raw PDF/HTML source and produce a standardized matrix entry following the current schema, so every corpus addition has consistent format |

### Phase 3B — Cross-reference building skills

| Skill | Purpose |
|---|---|
| **CVE enrichment** | Given a feature name, query NVD/MITRE for relevant CVEs, produce a standardized CVE table, flag which ones affect UMRS posture signals |
| **Control mapping** | Given a technology, find applicable NIST SP 800-53, CMMC, and CCE citations automatically, formatted per project citation rules |

### Phase 3C — Corpus quality assurance skills

| Skill | Purpose |
|---|---|
| **Gap detection** | Scan corpus directory structure against the plan's feature inventory; report which features lack files, which files are incomplete, which have stale sources |
| **Consistency checking** | Verify all files follow the same template, all CPUID references use the same notation, all classification rationales are present |

### Phase 3D — Knowledge synthesis skills

| Skill | Purpose |
|---|---|
| **Knowledge index generation** | After each phase completes, auto-generate an index mapping topics → file locations so other agents can quickly find what they need |
| **Delta reports** | When a source document updates (new STIG, new kernel version), identify what changed and which corpus files need revision |

### Phase 3E — Acquisition automation skills

| Skill | Purpose |
|---|---|
| **Multi-source fetch** | Given a topic, check NIST, kernel.org, Intel/AMD developer portals, and Red Hat docs in parallel; produce a consolidated research brief |
| **Manual-download flagging** | Detect when a URL requires browser-only access (DoD portals) and produce a formatted request for Jamie with exact download steps |

### Phase 3F — Team service skills

| Skill | Purpose |
|---|---|
| **"What do we know about X?"** | Any agent asks a question, the researcher searches the corpus and returns relevant findings with file references |
| **Bibliography generator** | Given a doc page being written by tech-writer, produce the reference list from corpus material |

### Deliverable

Skill definitions in `.claude/skills/` for each capability above. Skills are implemented
incrementally — prioritize version checker, gap detection, and "what do we know about X?"
first, as these have the highest immediate value to the team.

---

## Execution Notes

- **This plan is on hold.** Do not begin execution until Jamie activates it.
- Phases within each corpus are sequential (1A before 1B, etc.)
- Corpus 1, Corpus 2, and Corpus 3 can execute in parallel once activated
- All acquired material goes through the standard RAG pipeline: acquire → `.claude/reference/` → `rag-ingest` → `corpus-familiarization`
- Corpus 3 (skills) can begin independently of Corpus 1 and 2 — the skills are useful regardless of what's in the corpus
- The researcher does NOT produce blog posts, documentation, or code — only organized reference material, research summaries, and skill definitions

---

## Source Material

This plan was derived from Jamie's research notes:
- `jamies_brain/info-theory.txt` (archived 2026-03-19)
- `jamies_brain/sage-food.txt` (archived 2026-03-19)
- `jamies_brain/help-rearcher.txt` (archived 2026-03-19)
