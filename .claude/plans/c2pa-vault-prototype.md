# C2PA Content Credentials — UMRS Vault Integration Prototype

**Status:** draft
**Type:** feasibility prototype (spike)
**ROADMAP alignment:** G4 (CUI vault integrity), G6 (chain-of-custody), G10 (transparency)
**Tech lead:** TBD — new Rust-capable agent or Rusty
**LOE estimate:** Small (1-2 sessions for spike)
**Source:** `.claude/jamies_brain/c2pa-in-vault-concept.txt`

---

## What Is C2PA?

C2PA (Coalition for Content Provenance and Authenticity) is an open standard for embedding
cryptographically signed provenance metadata into digital assets. It creates an append-only
chain of manifests inside a file — each manifest records what happened (assertions), bundles
them into a signed claim, and links to the previous manifest.

Key properties:
- **Tamper-evident**: hash-bound to file content; any modification breaks the chain
- **Identity-backed**: each manifest signed with X.509 certificates
- **Append-only**: capture → edit → publish, each step adds a signed record
- **Standardized**: ISO-based container (JUMBF), CBOR encoding, interoperable across vendors
- **Trust ≠ truth**: proves who said what happened, not whether the content is "real"

The spec explicitly states: *"It does not judge truth, only whether provenance is valid."*

---

## Why This Matters for UMRS

The structural parallel is immediate:

| C2PA Concept | UMRS Equivalent |
|---|---|
| Manifest | Evidence receipt / audit record |
| Assertion | Event payload |
| Claim | Signed audit envelope |
| Signature | Provenance attestation |
| Manifest chain | Chain-of-custody log |
| JUMBF container | Embedded vault sidecar |

### Potential UMRS Benefits

1. **CUI vault chain-of-custody** — when a file enters a CUI vault, UMRS could read/verify
   existing C2PA manifests and potentially append a "CUI ingestion" manifest recording the
   labeling event, operator identity, and timestamp.

2. **Provenance verification at ingestion** — before accepting a file into a vault, verify
   its C2PA chain. Broken chain = finding. No chain = advisory. Valid chain = evidence record.

3. **Export attestation** — when files leave a vault (declassification, sharing), append a
   manifest recording the export event, authorization, and destination.

4. **Audit trail enrichment** — C2PA manifests are structured data UMRS can read, index,
   and surface through existing tool infrastructure (umrs-stat, umrs-ls).

5. **AI-generated content detection** — C2PA includes AI generation assertions. UMRS could
   flag AI-generated content entering CUI vaults as an advisory finding.

---

## Feasibility Questions (What the Spike Must Answer)

1. **Can we read C2PA manifests from Rust?**
   - The `c2pa` crate exists on crates.io (official C2PA Rust SDK)
   - Does it use `unsafe`? What's the dependency tree? License compatibility?
   - Can we extract manifest data without pulling in the full media processing stack?

2. **What file formats carry C2PA today?**
   - JPEG, PNG, MP4, PDF — are any of these common in CUI workflows?
   - Can C2PA be attached to arbitrary files, or only supported media types?

3. **Can we write manifests?**
   - Appending a manifest requires signing infrastructure (X.509 certs)
   - Is there a lightweight path, or does this require a full PKI setup?
   - Could UMRS use the system's existing IMA/EVM keys?

4. **What's the minimum viable read path?**
   - Parse JUMBF container → extract manifest store → decode CBOR → structured data
   - Can this be done without the full `c2pa` crate if it's too heavy?

5. **Does this conflict with any UMRS constraints?**
   - `#![forbid(unsafe_code)]` — does the c2pa crate or its deps require unsafe?
   - No FFI rule — does c2pa shell out or link C libraries?
   - Supply chain risk — how many transitive deps?

---

## Spike Plan

### Phase 1: Crate Assessment (30 min)
- Audit `c2pa` crate: unsafe usage, dep tree, license, FFI
- If unsuitable: assess `jumbf` / `cbor` crates for manual parsing
- Decision gate: use c2pa crate, roll minimal parser, or stop

### Phase 2: Read Path Prototype (1 session)
- Create `umrs-c2pa-spike` crate in workspace (or `examples/` if lightweight)
- Read a C2PA-signed JPEG and extract manifest chain
- Print: assertions, claim hashes, signer identity, chain integrity
- Map output to UMRS finding/observation types

### Phase 3: Feasibility Report
- Document: what works, what doesn't, dependency cost, security posture
- Recommendation: integrate as feature, defer, or abandon
- If viable: draft integration plan for vault ingestion path

---

## Reference Material

- C2PA Technical Specification — Librarian downloading to `.claude/references/c2pa/`
- Source concept: `.claude/jamies_brain/c2pa-in-vault-concept.txt` (archived)
- Official Rust SDK: `c2pa` crate on crates.io

---

## Open Questions for Jamie

- Should the spike agent be a new specialized agent, or should Rusty handle this?
- Do we have sample C2PA-signed files to test with, or should we generate them?
- Is the read path (verification) sufficient for Phase 1, or do we also need write (attestation)?
- Priority relative to current CUI labeling workstream — after umrs-mcs, or parallel?
