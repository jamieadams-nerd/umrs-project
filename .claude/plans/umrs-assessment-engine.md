---
name: umrs-assessment-engine
agent: rust-developer
status: blocked — waiting on posture probe Phase 2b completion
depends-on:
  - kernel-security-posture-probe.md
  - security-auditor-corpus.md
---

# UMRS Assessment Engine

## Status

**BLOCKED** — Do not begin implementation until posture probe Phase 2b is complete and the
`security-auditor-corpus.md` plan has been executed. This plan is a vision and architecture
document only. No code changes are authorized at this time.

---

## Purpose

Build a typed Rust assessment engine that produces structured, provenance-tagged security
artifacts — not a scanner. The engine mirrors the RMF assessment lifecycle:

```
evidence → assertion → finding → risk → export
```

The goal is to make UMRS capable of automatically generating auditor-ready evidence packages
compatible with existing compliance ecosystems (OSCAL, FedRAMP, RMF). The engine should
behave like a real accreditation assessor: gathering evidence using examine and test methods,
mapping evidence to controls, identifying contradictions between runtime and persisted state,
and producing exportable artifacts.

The canonical mental model for the engine:

1. What is the claim?
2. What evidence supports it?
3. What is the provenance of that evidence?
4. What contradicts it?
5. What control or risk does it affect?
6. What remediation is appropriate?
7. Can this be exported in a structured form?

---

## Model Assignments

| Phase | Agent | Model | Rationale |
|---|---|---|---|
| Phase 1 — Core Types & Scaffold | rust-developer | **opus** | Architectural type design, security-critical invariants, compliance annotations |
| Phase 2 — Evidence Collection | rust-developer | **opus** | Provenance-verified I/O, multiple HA patterns, TOCTOU safety |
| Phase 3 — Assertions & Contradiction | rust-developer | **opus** | Reasoning layer, control mapping, dual-truth model complexity |
| Phase 4 — Findings & POA&M | rust-developer | **sonnet** | Derives from established types, more mechanical transformation |
| Phase 5 — Export | rust-developer | **sonnet** | Serialization, JSON output, template rendering |
| Phase 6 — Evidence Receipts | rust-developer | **opus** | Cryptographic operations, FIPS gate, security-critical |
| Phase 7 — Documentation | tech-writer | **sonnet** | Standard documentation from established API |
| Security review (after Phase 3) | security-auditor | **opus** | Full assessment of evidence/assertion/finding model |
| Security review (after Phase 5) | security-engineer | **opus** | OSCAL schema validation, export integrity |

---

## DO NOT CHANGE ANY CODE

This is a vision and planning document. No implementation work is authorized until:

- The `kernel-security-posture-probe.md` plan reaches Phase 2b completion.
- The `security-auditor-corpus.md` plan has been executed (corpus ingested into RAG).
- Jamie has reviewed and approved this plan for implementation.

The rust-developer agent must not create crates, modules, or files based on this plan
without explicit authorization.

---

## Proposed Crate: `umrs-assess`

New crate name: `umrs-assess` (alternatively `umrs-audit` — name TBD at implementation time).

The crate sits above `umrs-platform`, `umrs-selinux`, and `umrs-core` in the dependency graph.
It must not be depended upon by any existing crate — it is a consumer, not a provider.

```
umrs-platform   ← no workspace deps
umrs-selinux    ← depends on umrs-platform
umrs-core       ← depends on umrs-platform, umrs-selinux
umrs-assess     ← depends on umrs-platform, umrs-selinux, umrs-core
```

This placement is consistent with the fixed architectural constraints in CLAUDE.md. Adding
`umrs-assess` does not violate any existing dependency direction.

---

## Five-Layer Operating Model

### Layer A — Asset and Boundary Discovery

Determines what is in scope before assessment begins. Aligns with RMF system categorization
and control selection phases.

Collects:
- OS identity, kernel version, installed packages
- Running services and listening sockets
- Storage and mount layout
- Trust boundaries and management interfaces
- Identities, roles, and security subsystem inventory

Output: asset inventory, boundary summary, subsystem map, interface map.

### Layer B — Evidence Collection

Gathers raw, attributable, provenance-tagged facts from the system. This is the "examine"
and "test" substrate. Consumes existing UMRS outputs as primary evidence sources.

Primary evidence sources:
- `PostureSnapshot` + `SignalReport` — kernel security posture domain
- `DetectionResult` + `EvidenceBundle` (from `umrs-platform::detect`) — OS detection domain
- `SecurityContext` + `SecureDirent` — SELinux domain

**Naming note**: The `EvidenceBundle` type in `umrs-platform::detect` is specific to the OS
detection pipeline and has different semantics from the assessment engine's `EvidenceRecord`.
Do not reuse the name. The assessment engine uses `EvidenceRecord` exclusively.

Collects:
- `/proc` and `/sys` kernel state (via existing `SecureReader` engine)
- SELinux mode, loaded policy, context facts
- Package provenance
- Crypto policy and OpenSSL configuration
- journald and audit daemon configuration
- systemd unit data
- File permissions, labels, and xattr state
- Boot state, immutable flags, mount flags, lockdown state

### Layer C — Assertions and Control Mapping

Converts raw evidence into security-relevant typed statements. Maps assertions to NIST SP
800-53 control families (AU, AC, CM, SC, SI, IA).

Examples:
- "Kernel lockdown is active in integrity mode."
- "SELinux is enforcing targeted policy."
- "System-wide crypto policy prohibits legacy algorithms."
- "journald is configured for persistent storage."
- "PermitRootLogin is disabled in the effective sshd configuration."

One assertion may rely on multiple evidence records. One evidence record may support several
controls. Assertions must be re-evaluable as policies evolve.

### Layer D — Findings and Risk Logic

Produces typed findings from assertion evaluation. Includes contradiction detection between
runtime state and persisted configuration — the dual-truth model.

Finding conditions:
- Assertion failed
- Assertion partially satisfied
- Assertion contradicted by another source
- Runtime state differs from configured/persisted state
- Control is present but weak
- Control claim exists in documentation but no supporting evidence exists

Outputs: typed `Finding` values with severity, control references, evidence references,
and remediation recommendation.

### Layer E — Reporting and Export

Produces both human-readable and machine-ingestible output.

Primary export target: OSCAL JSON (SSP, SAP, assessment results, POA&M).
Secondary export target: SAR-style Markdown / AsciiDoc.
Future: optional InSpec profile / OpenSCAP bridge data.

---

## Core Types

These types represent the typed internal model. Field names and shapes are provisional —
final design happens at implementation time.

### `EvidenceRecord`

A provenance-tagged, hashable, typed fact about the system.

```
EvidenceRecord {
    evidence_id:      String         // unique, stable identifier
    collector:        String         // collector identity and version
    collected_at:     DateTime<Utc>  // collection timestamp
    source_type:      SourceKind     // KernelPseudofile, PackageDb, ConfigFile, etc.
    source_path:      PathBuf        // path or API used
    trust_level:      TrustTier      // T1 through T4
    value:            EvidenceValue  // parsed, normalized value
    raw_sha256:       [u8; 32]       // hash of raw bytes before parsing
    parse_status:     ParseStatus    // Ok / Partial / Failed
    host_id:          String         // system boundary identifier
    notes:            Vec<String>    // collector annotations
}
```

### `Assertion`

Derived from one or more evidence records. Maps to NIST controls. Carries a confidence level.

```
Assertion {
    assertion_id:  String
    statement:     String
    derived_from:  Vec<EvidenceId>
    result:        AssertionResult    // see severity/confidence model below
    confidence:    Confidence
    scope:         AssertionScope     // Host / Service / Policy / etc.
    control_refs:  Vec<ControlRef>   // e.g. ["CM-6", "SI-7"]
    rationale:     String
}
```

### `Finding`

A failed or partially satisfied assertion with severity, control references, evidence
references, and a remediation recommendation.

```
Finding {
    finding_id:      String
    title:           String
    status:          FindingStatus     // Open / Closed / Mitigated / Accepted
    severity:        Severity
    likelihood:      Likelihood
    impact:          Impact
    affected_scope:  String
    control_refs:    Vec<ControlRef>
    evidence_refs:   Vec<EvidenceId>
    assertion_refs:  Vec<AssertionId>
    summary:         String
    recommendation:  String
}
```

### `PoamItem`

A remediation candidate derived from findings. Represents a Plan of Action and Milestones
entry compatible with RMF/FedRAMP POA&M format.

```
PoamItem {
    poam_id:       String
    finding_ref:   FindingId
    title:         String
    severity:      Severity
    remediation:   String
    owner:         String
    target_date:   NaiveDate
    status:        PoamStatus         // Open / In-Progress / Closed / Deferred
}
```

### `AssessmentBundle`

The full artifact package. The manifest covers all included artifacts with integrity hashes,
making the bundle itself a provenance object.

```
AssessmentBundle {
    manifest:        BundleManifest
    system_profile:  SystemProfile
    evidence:        Vec<EvidenceRecord>
    assertions:      Vec<Assertion>
    findings:        Vec<Finding>
    poam:            Vec<PoamItem>
}

BundleManifest {
    assessment_id:   String
    tool_versions:   HashMap<String, String>
    hosts:           Vec<String>
    generated_at:    DateTime<Utc>
    boundary:        String
    profile_used:    String         // baseline/mission profile name
    artifact_hashes: HashMap<String, [u8; 32]>
}
```

---

## Severity and Confidence Model

### Severity

| Value | Meaning |
|---|---|
| Critical | Immediate compromise risk |
| High | Major security weakness |
| Moderate | Increased attack surface |
| Low | Minor security improvement |
| Informational | Observation only |

### Result (for assertions)

| Value | Meaning |
|---|---|
| Satisfied | Control is fully implemented and evidenced |
| PartiallySatisfied | Control is implemented but with gaps or weaknesses |
| NotSatisfied | Control is absent or its implementation is ineffective |
| NotApplicable | Control does not apply to this system or context |
| UnableToDetermine | Insufficient evidence to make a determination |

### Confidence

| Value | Meaning |
|---|---|
| High | Multiple corroborating evidence sources, no contradictions |
| Medium | Single source or minor inconsistency between sources |
| Low | Weak or indirect evidence, significant uncertainty |

### Evidence Sufficiency

| Value | Meaning |
|---|---|
| Adequate | Evidence is sufficient and unambiguous |
| Weak | Evidence is present but indirect or incomplete |
| Missing | No relevant evidence was collected |
| Contradictory | Evidence sources disagree — dual-truth mismatch detected |

---

## Dual-Truth Model

For any configurable security feature, the engine collects two distinct views:

**Runtime truth** — what is actually active on the system right now:
- Actual kernel lockdown mode
- Actual SELinux enforcement mode
- Actual mount options in effect
- Actual running service state
- Actual crypto provider state

**Intended truth** — what the system is configured to become:
- Kernel command line
- sysctl configuration files
- /etc/ configuration files
- SELinux policy modules
- systemd unit files
- Package policy and bootloader configuration

The engine then compares them. Mismatches between runtime and intended truth are the most
valuable findings because they represent exactly the kind of gap real assessors look for:

- Configured but not active
- Active but not persisted (will revert on reboot)
- Persisted but overridden at runtime
- Documented but not implemented

Contradiction detection is already partially implemented in the posture probe via
`ContradictionKind`. The assessment engine extends this model across all domains.

---

## Mission-Profile Overlays

The same host can be evaluated against different baselines:

- Linux host hardening baseline
- FIPS posture profile
- SELinux enforced posture profile
- UMRS high-assurance posture profile
- Compartmented vault posture profile

Mission profiles allow layered accreditation views rather than a single flat scan result.
Profile definitions are external to the engine — the engine evaluates; the profile specifies
what to require.

---

## Evidence Receipts

Each evidence record or bundle can be signed, hashed, and chained. This makes later audit
review much stronger and supports chain-of-custody requirements for CUI.

**FIPS constraint**: Any cryptographic operations used for evidence receipts (signing, hashing
for integrity) must use FIPS 140-3 validated primitives only. The engine must read the FIPS
kernel attribute at construction time and fail closed if a required primitive is unavailable
under FIPS policy.

---

## Assessment Bundle Directory Layout

```
assessment-bundle/
  manifest.json
  system-profile/
    boundary.json
    architecture-summary.md
    inventory.json
  evidence/
    kernel/
    selinux/
    crypto/
    logging/
    identity/
    services/
    storage/
  assertions/
    assertions.json
  findings/
    findings.json
  poam/
    poam.json
  reports/
    summary.md
    technical-report.md
  oscal/
    ssp.json
    assessment-results.json
    poam.json
```

---

## Export Targets

### Primary: OSCAL JSON

OSCAL (Open Security Controls Assessment Language) is the machine-readable format for
security compliance artifacts. It supports SSPs, SAPs, assessment results, and POA&Ms in
JSON/XML/YAML. FedRAMP validates OSCAL-based package content.

Target OSCAL schema version: **open question — see architectural decisions below.**

### Secondary: SAR-style Markdown / AsciiDoc

Human-readable Security Assessment Report format. Matches the structure used in real
FedRAMP and RMF assessment deliverables.

### Future: InSpec / OpenSCAP Bridge

Export findings and results in formats consumable by Chef InSpec profiles or the OpenSCAP /
SCAP ecosystem. Treat these as downstream consumers, not the primary model.

---

## Relationship to Existing Code

| Existing artifact | Role in assessment engine |
|---|---|
| `PostureSnapshot` | Primary evidence source for kernel security domain (Layer B) |
| `SignalReport` | Contradiction detection input (Layer D) |
| `DetectionResult` | Evidence source for OS detection domain (Layer B) |
| `umrs-platform::detect::EvidenceBundle` | Evidence source for OS detection domain — note: different type from `EvidenceRecord`; do not conflate |
| `SecurityContext` | Evidence source for SELinux domain (Layer B) |
| `SecureDirent` | Evidence source for SELinux domain (Layer B) |
| `SealedCache` | Candidate for caching assessment bundles with HMAC+TTL |

The engine is a consumer of all of the above. It does not modify them.

---

## Open Architectural Decisions

These decisions are intentionally deferred. They must be resolved before implementation begins.

1. **Crate name**: `umrs-assess` vs `umrs-audit` — preference for `umrs-assess` (mirrors RMF
   "assess" phase language), but confirm with Jamie before creating the crate.

2. **OSCAL schema version**: Which version of the OSCAL JSON schema to target? NIST has
   released 1.0.x and is working toward 1.1.x. FedRAMP has specific version requirements.
   The researcher agent should confirm the current authoritative version.

3. **Evidence bundle serialization**: Which format for on-disk evidence bundles?
   - `postcard` — compact binary, no schema, good for embedded/performance-critical paths
   - CBOR — binary, schema-free, IETF standard
   - JSON — human-readable, widely tooled, larger
   Recommendation: JSON for the assessment bundle (auditor-readable); postcard or CBOR only
   if a sealed/cached internal format is needed. Confirm with Jamie.

4. **DetectionResult serialization**: Should `DetectionResult` gain a serialization
   implementation before the assessment engine is built, or after? Rust-developer recommends:
   design the `DetectionResult` serialization interface now (as part of this planning phase)
   so the assessment engine can rely on it. But do not implement until authorized.

5. **FIPS gate on evidence receipts**: Signed evidence receipts require a FIPS 140-3 validated
   cryptographic primitive. RustCrypto's HMAC-SHA256 is available under the `hmac` crate (used
   by `SealedCache`). Confirm whether this is sufficient for the evidence receipt use case or
   whether a different primitive is needed. Raise with security-engineer before implementation.

6. **POA&M ownership model**: POA&M items require an owner field. In the first prototype,
   this will be a free-form string. A future version should link to an identity model. Defer.

7. **Persistence model for bundles**: Is the assessment bundle written to the filesystem, held
   in memory only, or both? For the first vertical slice, write to a user-specified output
   directory. Streaming / daemon mode is a future concern.

---

## First Vertical Slice (v1 Scope)

When implementation is authorized, v1 targets a narrow but complete demonstration:

**Assessment profile**: Linux host hardening posture

**Evidence collected for**:
- OS identity and package substrate
- Kernel lockdown state
- FIPS status
- SELinux enabled / enforcing / policy facts
- journald persistence and forwarding posture
- Audit subsystem presence and configuration
- SSH posture (PermitRootLogin, PasswordAuthentication, key configuration)
- Mount hardening (noexec, nosuid, nodev on applicable mounts)
- Module loading restrictions
- Key crypto policy facts (system-wide crypto policy, OpenSSL FIPS mode)

**Output artifacts produced**:
- `evidence.json`
- `assertions.json`
- `findings.json`
- `summary.md`
- `poam.json`
- One OSCAL assessment-results export

This scope is already well-supported by existing UMRS infrastructure. It is deliberately
narrow to produce a credible, complete artifact rather than a broad but shallow scan.

---

## High-Assurance Patterns Required

When implementation begins, the following patterns apply:

- **TPI** — any new parser for evidence values read from configuration files requires two
  independent parse paths. Does not apply to kernel attribute parsers (boolean/dual-boolean).
- **TOCTOU safety** — all file I/O in evidence collection must be fd-anchored via rustix.
- **ProcfsText / SysfsText routing** — mandatory for all `/proc/` and `/sys/` reads.
- **Fail-closed** — on any parse error, provenance failure, or contradiction: record the
  failure as a `UnableToDetermine` result; do not silently succeed with a degraded default.
- **Zeroize** — not required for classification labels or evidence values, but required if
  any type holds key material used for evidence receipt signing.
- **Loud failure** — use `log::warn!` or `log::error!` for any security-relevant degradation.
- **FIPS gate** — read FIPS kernel attribute at engine construction; fail closed if required
  primitives are unavailable.
- **Pattern execution measurement** — record timing in debug mode for TPI, dual-truth
  comparison, and evidence receipt operations.
- **`#[must_use]`** — all public functions returning `Result`, `Option`, or a security-relevant
  type must carry `#[must_use]` with a descriptive message string.
- **`#![forbid(unsafe_code)]`** — must appear in the new crate root immediately.

---

## Compliance Control Mapping

| Engine component | Control references |
|---|---|
| Evidence collection with provenance | NIST SP 800-53 AU-3, AU-9; NSA RTB RAIN |
| Assertion mapping to controls | NIST SP 800-53A Rev. 5 (assessment methods) |
| Contradiction detection (dual-truth) | NIST SP 800-53 CM-6, CA-7 |
| Finding severity model | NIST SP 800-30 risk assessment methodology |
| POA&M generation | NIST SP 800-37 Rev. 2 RMF lifecycle |
| OSCAL export | NIST SP 800-53 SA-11; FIPS alignment |
| Evidence receipts (signed, hashed) | NIST SP 800-53 AU-10; CMMC AU.L2-3.3.1 |
| Mission-profile overlays | NIST SP 800-53 RA-2 (security categorization) |
| Fail-closed on parse error | NSA RTB Fail Secure; NIST SP 800-218 SSDF PW.4 |

---

## Definition of Done

### Phase 0 — Prerequisites (BLOCKED until complete)

- [ ] Posture probe Phase 2b is complete and merged.
- [ ] `security-auditor-corpus.md` plan has been executed; NIST SP 800-53A, SP 800-37,
      FedRAMP SAR/SAP templates, and OSCAL schema docs are ingested into the RAG pipeline.
- [ ] Open architectural decisions (OSCAL version, serialization format, crate name) have
      been resolved and documented here.
- [ ] Jamie has reviewed and approved this plan for implementation.

### Phase 1 — Core Types and Crate Scaffold

- [ ] `umrs-assess` crate created in workspace with `#![forbid(unsafe_code)]` in crate root.
- [ ] `EvidenceRecord`, `Assertion`, `Finding`, `PoamItem`, `AssessmentBundle` types defined
      with private constructors and validated-at-construction invariants.
- [ ] Severity, confidence, result, and evidence sufficiency enums defined.
- [ ] All public types carry NIST SP 800-53 compliance annotations.
- [ ] `cargo xtask clippy` passes clean with zero warnings.
- [ ] At least one example in `examples/` demonstrating the type model.
- [ ] Tech-writer task created for API documentation sync.

### Phase 2 — Evidence Collection (Layer B)

- [ ] Evidence collectors implemented for all v1 scope items (OS identity, lockdown, FIPS,
      SELinux, journald, audit, SSH, mounts, module loading, crypto policy).
- [ ] All collectors use `SecureReader` / `ProcfsText` / `SysfsText` routing — no raw File::open
      on `/proc/` or `/sys/`.
- [ ] `EvidenceRecord` includes trust tier, parse status, and raw SHA-256.
- [ ] Integration tests in `tests/` for each collector.
- [ ] Pattern execution measurement (timing) in debug mode for TPI collectors.

### Phase 3 — Assertions and Contradiction Detection (Layers C and D)

- [ ] Assertion engine converts evidence records into typed `Assertion` values.
- [ ] Dual-truth comparison implemented: runtime vs persisted state per domain.
- [ ] Contradiction detection produces `Contradictory` evidence sufficiency and finding.
- [ ] Control mapping table implemented (at minimum: CM-6, SI-7, AC-6, IA-2, AU-2, AU-9,
      SC-13, AC-3, CM-3).
- [ ] Integration tests for contradiction scenarios (e.g., SELinux mode mismatch).

### Phase 4 — Findings and POA&M (Layer D)

- [ ] `Finding` values derived from failed / partial assertions.
- [ ] `PoamItem` values generated automatically from open findings.
- [ ] Severity model correctly applied across all finding types.
- [ ] Integration tests covering finding generation from known-bad evidence sets.

### Phase 5 — Export (Layer E)

- [ ] `summary.md` human-readable report generated.
- [ ] `evidence.json`, `assertions.json`, `findings.json`, `poam.json` written to output
      directory with correct structure.
- [ ] OSCAL assessment-results export producing valid OSCAL JSON for the target schema version.
- [ ] Assessment bundle manifest includes artifact integrity hashes.
- [ ] `cargo xtask test` passes clean.

### Phase 6 — Evidence Receipts (Optional — authorize separately)

- [ ] Evidence records can be hashed and optionally HMAC-signed.
- [ ] FIPS gate is enforced: signing path disabled if FIPS kernel attribute is active and
      the required primitive is not FIPS 140-3 validated.
- [ ] Chain-of-custody manifest format defined and documented.

### Phase 7 — Documentation

- [ ] Antora documentation page in `docs/modules/patterns/pages/` for the assessment engine
      architecture.
- [ ] Developer guide updated to reference `umrs-assess` crate and its position in the
      dependency graph.
- [ ] Tech-writer task created and completed.
- [ ] `make docs` passes cleanly.
