---
name: Security Auditor Methodology Corpus
agent: researcher
status: ready to begin
source: .claude/jamies_brain/enhance-sa.txt (Initiative A)
depends-on: none (independent track)
---

# Security Auditor Methodology Corpus — Acquisition Plan

## Purpose

Equip the security-auditor agent with the same source material and mental model
used by professional RMF assessors and accreditation bodies. Once ingested, the
security-auditor can provide sharper, methodology-grounded feedback on all project
plans — posture probe, CPU corpus, assessment engine, and expansion work.

This is a **force multiplier**: better auditor feedback improves every other plan.

## Current State

The security-auditor agent currently has:
- NIST SP 800-53 Rev. 5 control knowledge (from existing RAG corpus)
- NSA RTB and NIST SP 800-218 SSDF familiarity (from CLAUDE.md rules)
- Pattern-matching for annotation debt, pub-field invariant violations, etc.

What it lacks:
- Formal assessment methodology (how auditors actually test and verify)
- Evidence requirements and sufficiency criteria
- Finding/risk severity models used in real accreditations
- SSP/SAP/SAR/POA&M document structure knowledge
- OSCAL awareness for machine-readable compliance artifacts

---

## Model Assignments

| Phase | Agent | Model | Rationale |
|---|---|---|---|
| Phase 1 — RMF Core | researcher | **sonnet** | Document acquisition, structured extraction, RAG ingestion |
| Phase 2 — Accreditation Process | researcher | **sonnet** | Template analysis, structured extraction |
| Phase 3 — Technical Compliance | researcher | **sonnet** | STIG/CIS mapping, signal cross-reference |
| Phase 4 — Systems Engineering | researcher | **sonnet** | Document acquisition and summarization |
| Phase 5 — Threat Modeling | researcher | **haiku** | Low priority, simple acquisition tasks |
| Corpus familiarization (after each phase) | researcher | **sonnet** | Knowledge synthesis across ingested material |
| Security-auditor feedback pass (after Phase 1) | security-auditor | **opus** | Methodology-grounded review of all active plans |

---

## Acquisition Phases

### Phase 1 — RMF Core (highest priority)

These documents define the assessment lifecycle the security-auditor should follow.
Prioritize these because they directly improve plan feedback quality.

| Document | What It Teaches | Priority |
|---|---|---|
| NIST SP 800-37 Rev. 2 | Full RMF lifecycle (categorize → select → implement → assess → authorize → monitor) | Critical |
| NIST SP 800-53A Rev. 5 | Assessment methods: Examine / Interview / Test — how auditors verify controls | Critical |
| NIST SP 800-30 Rev. 1 | Risk assessment methodology — threat/likelihood/impact model | High |
| NIST SP 800-39 | Enterprise risk governance — how risk decisions roll up | Medium |

**Why 800-53A is the single most important document:** It defines the three assessment
methods (Examine, Interview, Test) and maps each control to specific assessment
procedures. This is the difference between "does AC-6 apply?" (current auditor)
and "here is how to verify AC-6 is implemented correctly" (target auditor).

**Checkpoint:** After Phase 1, the security-auditor should be able to:
- Classify its own review activities as Examine / Interview / Test
- Cite specific 800-53A assessment procedures when reviewing code
- Identify evidence gaps (not just annotation gaps)

---

### Phase 2 — Accreditation Process Documents

Real audits revolve around structured artifacts. These documents teach the
security-auditor what a complete accreditation package looks like.

| Document | What It Teaches | Priority |
|---|---|---|
| NIST SP 800-18 Rev. 1 | System Security Plan (SSP) structure — what a system description must contain | High |
| FedRAMP Security Assessment Framework | Real accreditation workflow with concrete steps | High |
| FedRAMP SSP Template | How systems document control implementations | Medium |
| FedRAMP SAP Template | How auditors write test plans | Medium |
| FedRAMP SAR Template | How auditors write findings reports | Medium |

**Why FedRAMP matters:** FedRAMP provides the best publicly available real audit
artifacts. The templates show what auditors actually deliver — not theory, but
concrete document structures with real examples.

**Checkpoint:** After Phase 2, the security-auditor should be able to:
- Evaluate whether UMRS documentation would satisfy an SSP review
- Structure its own findings in SAR-compatible format
- Identify what evidence an assessor would request for each control

---

### Phase 3 — Technical Compliance Checks

These resources map directly to observable system evidence — the kind of checks
that `umrs-platform` posture probes already collect.

| Document | What It Teaches | Priority |
|---|---|---|
| DISA STIG methodology (RHEL 9/10 STIG) | Technical configuration checks, severity categories (CAT I/II/III) | High |
| CIS Benchmark methodology (RHEL) | Hardening checks mapped to observable system state | Medium |
| CMMC 2.0 Assessment Guide | How CMMC auditors verify controls, evidence expectations, scoring | Medium |

**Why STIGs matter for UMRS specifically:** STIG checks map directly to the
posture probe's signal catalog. Each STIG rule is essentially a `SignalDescriptor`
with a desired value. The security-auditor can cross-reference STIG rule IDs
against posture probe coverage to identify detection gaps.

**Checkpoint:** After Phase 3, the security-auditor should be able to:
- Map posture probe signals to specific STIG rule IDs
- Identify which STIG checks are covered vs uncovered by current probes
- Apply severity categories (CAT I/II/III) to posture findings

---

### Phase 4 — Systems Security Engineering

| Document | What It Teaches | Priority |
|---|---|---|
| NIST SP 800-160 Vol. 1 | How to design systems for security from the start — trust, assurance, architecture | High |
| NIST SP 800-160 Vol. 2 | Cyber resiliency engineering — survivability, recovery | Medium |

**Why 800-160 is important:** It aligns directly with what UMRS is doing —
building a system designed for security from the architecture level. The
security-auditor should be able to evaluate whether UMRS design decisions
satisfy systems security engineering principles, not just individual controls.

---

### Phase 5 — Threat Modeling and Secure Code (lower priority)

| Resource | What It Teaches | Priority |
|---|---|---|
| OWASP Code Review Guide | Application-level auditing techniques | Low |
| CERT Secure Coding Standards (C/Rust) | Language-specific secure coding rules | Low |
| STRIDE threat model methodology | Threat classification for design review | Low |

These are lower priority because the security-auditor already has strong
code-level review capabilities from CLAUDE.md rules and pattern enforcement.
The bigger gap is in methodology and process, not code inspection.

---

## Acquisition Notes

### Sources

- NIST publications: https://csrc.nist.gov/publications — all freely available as PDF
- FedRAMP templates: https://www.fedramp.gov/documents-templates/ — publicly available
- DISA STIGs: https://public.cyber.mil/stigs/ — public portal, some require manual download
- CIS Benchmarks: https://www.cisecurity.org/cis-benchmarks — free with registration
- CMMC: https://dodcio.defense.gov/CMMC/ — public documentation

### RAG Ingestion Strategy

Use the existing RAG infrastructure (`rag-ingest` skill). Recommended collection structure:

```
.claude/references/
  rmf-methodology/
    nist-800-37-rev2.pdf
    nist-800-53a-rev5.pdf
    nist-800-30-rev1.pdf
    nist-800-39.pdf
  accreditation-artifacts/
    nist-800-18-rev1.pdf
    fedramp-saf.pdf
    fedramp-ssp-template.pdf
    fedramp-sap-template.pdf
    fedramp-sar-template.pdf
  compliance-checks/
    rhel-stig.pdf
    cis-rhel-benchmark.pdf
    cmmc-assessment-guide.pdf
  systems-engineering/
    nist-800-160-vol1.pdf
    nist-800-160-vol2.pdf
```

### Manual Download Items

DISA STIGs may require manual browser download from public.cyber.mil (similar to
existing items in `refs/manifest.md`). Note these in `refs/manifest.md` when acquired.

---

## Downstream Impact

Once the corpus is ingested, the security-auditor agent gains capabilities that
directly improve feedback on other active plans:

| Plan | Improved Feedback |
|---|---|
| kernel-security-posture-probe | STIG rule mapping, evidence sufficiency checks, severity categorization |
| cpu-security-corpus-plan | Control mapping validation, evidence gap identification |
| umrs-platform-expansion | Compliance citation review, architectural risk assessment |
| umrs-assessment-engine (future) | SSP/SAP/SAR structure guidance, OSCAL schema validation, finding model review |

---

## Definition of Done

- [ ] Phase 1: RMF core documents acquired and ingested into RAG
- [ ] Phase 1: Security-auditor demonstrates Examine/Interview/Test classification
- [ ] Phase 2: FedRAMP templates and SSP guidance acquired and ingested
- [ ] Phase 3: RHEL STIG and CIS Benchmark ingested; signal-to-STIG mapping started
- [ ] Phase 4: NIST SP 800-160 acquired and ingested
- [ ] Phase 5: Lower-priority resources acquired as time permits
- [ ] All acquired documents tracked in `refs/manifest.md` with version, date, URL, SHA-256
- [ ] `corpus-familiarization` skill run after each phase to build active knowledge

---

## DO NOT CHANGE ANY CODE

This plan is corpus acquisition and RAG ingestion only. No Rust code changes.
The assessment engine architecture (Initiative B from enhance-sa.txt) is a
separate plan to be created after posture probe Phase 2b completes.
