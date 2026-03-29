# RMF Methodology-Grounded Plan Review
## Three Active UMRS Plans

```
Audit date: 2026-03-15
Depth: in-depth (plans reviewed exhaustively against RMF lifecycle, SP 800-53A procedures, SP 800-30 risk framing, and SP 800-39 tier model)
Scope:
  - .claude/plans/kernel-security-posture-probe.md
  - .claude/plans/cpu-security-corpus-plan.md
  - .claude/plans/umrs-platform-expansion.md
Methodology: SP 800-37 Rev. 2, SP 800-53A Rev. 5, SP 800-30 Rev. 1, SP 800-39
```

---

## Executive Summary

The three active UMRS plans collectively span the Implement and Monitor steps of the SP 800-37 RMF lifecycle, with the kernel posture probe directly producing continuous-monitoring evidence (CA-7) and the CPU corpus plan building the knowledge base that will eventually support SC-13 and SI-7 assessments. Each plan is technically rigorous and internally consistent with established UMRS high-assurance patterns. However, all three plans share a common evidence gap: they produce data-collection capabilities and typed findings, but they do not specify how those outputs become assessment-reusable artifacts (Security Assessment Reports, POA&M entries, or SSP narrative) per SP 800-37 task A-6 (assessment findings documentation). A real assessor arriving at these plans would find strong technical intent but would be unable to definitively determine whether assessed controls are "satisfied" or "other than satisfied" under SP 800-53A criteria, because the plans do not specify assessment objects, depth, or coverage. Recommendations are prioritized below by SP 800-30 risk level.

---

## Plan 1: Kernel Security Posture Probe

### 1.1 My Review Activities — SP 800-53A Classification

Per SP 800-53A Section 2.1, the three assessment methods are Examine, Interview, and Test. For this plan review, I applied:

- **Examine**: Reviewed the plan document as a specification artifact against SP 800-53 control requirements (the plan itself is an Examine object for SA-11, system design documentation).
- **Examine**: Reviewed the control citations within the plan (CM-6, CA-7, AU-3, SC-13, SI-7, SI-10, SI-11, SA-11) against SP 800-53A assessment procedures to determine whether the plan produces evidence that satisfies those controls' determination statements.
- **Test (simulated)**: Reasoned about the runtime behavior of `PostureSnapshot::collect()` and contradiction detection under adversarial conditions to assess whether the mechanism's claims can be validated.

I did not Interview (no personnel contact) and did not execute Test mechanisms (read-only review).

### 1.2 SP 800-53A Assessment Procedure Mappings

#### CM-6 — Configuration Settings

SP 800-53A assessment for CM-6 requires:

- **Examine objects**: Configuration management policy; system security plan; configuration management plan; configuration settings; documented deviations from established configuration settings.
- **Test objects**: Automated mechanisms implementing CM-6.

The plan's `PostureSnapshot::collect()` pipeline with sysctl.d merge and live-vs-configured contradiction detection maps precisely to the CM-6 Test object: it is an automated mechanism that compares live kernel settings against documented configuration. Phase 2a adds modprobe.d cross-check and FIPS persistence layer verification — both extend CM-6 Test coverage.

**Gap**: The plan specifies the mechanism but not the CM-6 assessment objects it would produce as output. There is no artifact defined (no structured report format, no machine-readable output schema) that an assessor could use as the "documented deviations from established configuration settings" Examine object for CM-6. `PostureSnapshot::findings()` returns `SignalReport` structs at runtime, but no plan phase produces a persisted deviation record that could be consumed in an SSP or SAR.

**SP 800-53A determination at risk**: CM-6 determination statement "the organization: (i) establishes and documents configuration settings... (ii) implements the configuration settings... (iii) identifies, documents, and approves any deviations" — the plan supports (i) and (ii) but leaves (iii) (documented deviations) as "other than satisfied" because no persistent deviation artifact is produced.

#### CA-7 — Continuous Monitoring

The plan explicitly cites CA-7 on `PostureSnapshot`. SP 800-53A CA-7 assessment requires:

- **Examine**: Continuous monitoring strategy; security status reports; results of security control assessments.
- **Test**: Automated mechanisms supporting ongoing security assessment.

The plan supports the Test object cleanly. However, CA-7 also requires a documented continuous monitoring strategy (Examine object) specifying frequencies, metrics, and reporting. The plan does not specify how snapshot results feed into a monitoring strategy document or security status report.

**Gap**: No monitoring frequency is specified. The plan describes how to collect a snapshot but not at what interval or under what trigger condition `PostureSnapshot::collect()` would be invoked in a monitoring context. SP 800-53A CA-7 requires organization-defined monitoring frequencies as an ODP [SDR-005 PENDING]. Without a defined frequency, the CA-7 determination "the organization monitors the information system and environment of operation on an ongoing basis" cannot be assessed as "satisfied."

#### AU-3 — Audit Record Content

The plan cites AU-3 on `SignalId` and `SignalReport`. SP 800-53A AU-3 assessment requires:

- **Examine**: Audit and accountability policy; audit record content; list of audit-relevant events.
- **Test**: Mechanisms generating audit records.

`SignalReport` structures contain `descriptor`, `live_value`, `configured_value`, `meets_desired`, and `contradiction`. This satisfies the AU-3 requirement for type of event, time of event, source, and outcome. The `boot_id` in `PostureSnapshot` provides a system identifier anchor.

**Partial gap**: AU-3 requires records to identify "the outcome (success or failure) of the event." `SignalReport::meets_desired: Option<bool>` captures this. However, `None` (unreadable signal) is distinguishable from `Some(false)` (read but not meeting desired) but neither carries a timestamp at the signal level — only `PostureSnapshot::collected_at` provides a timestamp for the whole snapshot. For fine-grained audit trail purposes, individual signal-level timestamps are missing. This is a LOW gap: the snapshot-level timestamp is sufficient for most monitoring scenarios.

### 1.3 Evidence Gaps — What an Assessor Needs

A real SP 800-53A assessor conducting a CA-7 / CM-6 assessment against this system would require:

| Evidence Required by Assessor | Plan Produces This? | Gap Severity |
|-------------------------------|---------------------|--------------|
| Documented continuous monitoring strategy (CA-7 ODP: frequency) | No | MEDIUM |
| Persistent deviation records from configuration baseline (CM-6 (iii)) | No | MEDIUM |
| Machine-readable output format for SAR consumption | No | LOW |
| Evidence that `SecureReader` provenance verification is non-bypassable (SI-7) | Yes (pattern docs) | None |
| Evidence that contradiction detection covers all contradiction classes | Yes (exhaustive tests) | None |
| Evidence that fail-closed parsing is enforced (SI-10) | Yes (tests + plan text) | None |
| FIPS state evidence chain for SC-13 assessment | Yes (Phase 2a FipsCrossCheck) | None |

### 1.4 SP 800-30 Risk Assessment — Identified Gaps

**Gap: No persistent deviation artifact for CM-6(iii)**

- **Threat source**: Adversarial (insider threat, privilege escalation)
- **Threat event**: Administrator modifies a sysctl setting to a non-hardened value; the change is not recorded persistently; the snapshot shows a `BootDrift` contradiction but no downstream artifact captures the deviation for audit review
- **Likelihood of initiation**: High (sysctl writes are a common post-exploitation technique)
- **Likelihood of success**: Moderate (the live contradiction is detected; the gap is that no persistent record is produced for the assessor)
- **Impact**: Moderate — the system detects the deviation but the assessor cannot verify that deviations are being reviewed and resolved (CM-6(iii) determination cannot be made)
- **SP 800-30 risk level**: Moderate → **UMRS MEDIUM**

**Gap: No monitoring frequency ODP for CA-7**

- **Threat source**: Non-adversarial (operational error, misconfiguration drift)
- **Threat event**: Sysctl settings drift over time; `PostureSnapshot::collect()` is never invoked in production because no monitoring schedule is defined
- **Likelihood**: Moderate (tooling without defined invocation schedule is commonly unused in practice)
- **Impact**: High — if monitoring is never invoked, CA-7 provides no actual assurance
- **SP 800-30 risk level**: High → **UMRS HIGH**

### 1.5 RMF Lifecycle Placement (SP 800-37)

| Plan Component | RMF Step | Task |
|----------------|----------|------|
| Signal catalog (CM-6 baseline definition) | Select / Implement | S-3 (control allocation), I-2 (implementation) |
| `PostureSnapshot::collect()` at runtime | Monitor | M-2 (ongoing assessments) |
| Contradiction detection output | Monitor | M-3 (ongoing remediation actions) |
| Phase 2a FIPS cross-check | Implement | I-2 (SC-13 control implementation) |
| Phase 2b CPU mitigation sub-signals | Implement | I-2 (SI-7 control implementation) |

The plan's output is a continuous monitoring artifact (M-2). It does not yet reach M-4 (ongoing risk determination) because there is no integration with a risk reporting system.

### 1.6 "Other Than Satisfied" Flags Under Current Plan

Under a formal SP 800-53A assessment using this plan's current state:

1. **CA-7**: "other than satisfied" — no monitoring strategy document specifying frequency; ODP undefined. (SP 800-53A CA-7 Examine object: continuous monitoring strategy — not produced by the plan.)
2. **CM-6(iii)**: "other than satisfied" — no persistent deviation record artifact; runtime `findings()` iterator is not a persistent document.
3. **AU-3 (partial)**: "satisfied" with notation — signal-level timestamps absent, but snapshot-level timestamp anchors the audit record sufficiently for most assessments.

---

## Plan 2: CPU Security Extensions Corpus

### 2.1 My Review Activities — SP 800-53A Classification

- **Examine**: Reviewed the corpus plan as a system design and acquisition document (SA-12, SA-15 objects).
- **Examine**: Reviewed the control citations implied by each feature category against SP 800-53 control families (SC-13, SI-7, SP 800-90B requirements).
- **Examine**: Reviewed source acquisition requirements against SP 800-53A SA-12 supply chain assessment procedures.

### 2.2 SP 800-53A Assessment Procedure Mappings

#### SC-13 — Cryptographic Protection

SP 800-53A SC-13 assessment requires:

- **Examine**: System security plan; cryptographic module validation certificates; list of FIPS-validated cryptographic modules.
- **Test**: Cryptographic modules or implementations performing cryptographic operations.

The corpus plan's Phase 1H (`/proc/crypto` and software utilization reference) maps directly to the SC-13 Test object: it will document how to verify whether hardware-backed cryptographic implementations are active, carry `selftest: passed`, and carry `fips_allowed: yes`. This is precisely the verification procedure an assessor would apply to SC-13.

**Gap**: The corpus plan produces research artifacts, not running test procedures. For SC-13 to be "satisfied", an assessor needs to execute a test against a live system. The corpus produces the knowledge to write such a test; it does not define the test procedure itself. The post-research Phase 1J (rust-developer and security-engineer review) is where test procedure specifications should be defined, but the current plan does not specify this as a deliverable from 1J.

**Recommended addition**: Phase 1J deliverables should explicitly include: "Draft SC-13 verification procedure specifying which `/proc/crypto` driver names, priorities, and flags must be present to satisfy SC-13 for a given cryptographic algorithm on a FIPS-active system."

#### SI-7 — Software, Firmware, and Information Integrity

Phase 1F (CET binary verification) maps to SI-7. SP 800-53A SI-7 Examine objects include: integrity verification tools and mechanisms. The plan's CET deep dive — specifying how to verify the `.note.gnu.property` section in ELF binaries — defines the Examine method and object for a software integrity check on compiled binaries.

**This is a genuine SP 800-53A methodology contribution**: the corpus research will produce an Examine procedure specification for SI-7 that UMRS does not currently have.

#### SA-12 — Supply Chain Protection / SA-15 — Development Process (SP 800-218 SSDF)

The corpus plan itself is an SA-15 artifact: it documents the research process for acquiring knowledge about CPU security features before implementing detection. Requiring Phase 0 reviews before implementation, and gating Phase 1B on SP 800-90B acquisition, demonstrates supply chain hygiene for the knowledge base.

**Gap**: The plan requires NIST SP 800-90B, Intel SDM, AMD APM, and NSA RHEL hardening guidance. Acquiring these via curl to `.claude/references/` is documented in the plan, but the plan does not specify verification of document integrity (hash checks) for newly acquired documents. The `.claude/references/refs-manifest.md` tracks SHA-256 checksums — the plan should explicitly require updating `.claude/references/refs-manifest.md` with hashes for each new document acquired.

**SP 800-53A SA-12 Examine object**: supply chain risk assessment; procurement policies. The plan's source acquisition table constitutes a procurement-equivalent process; checksum verification is the integrity control on that process.

### 2.3 Evidence Gaps — What an Assessor Needs

| Evidence Required by Assessor | Plan Produces This? | Gap Severity |
|-------------------------------|---------------------|--------------|
| SC-13 verification procedure (hardware crypto active on FIPS system) | No (deferred to 1J, not specified as deliverable) | HIGH |
| SI-7 procedure for CET binary ELF verification | Yes (Phase 1F deep dive deliverable) | None |
| SP 800-90B compliance determination for RDRAND/RDSEED on FIPS systems | Deferred (BLOCKING on 800-90B acquisition) | HIGH |
| Integrity verification of acquired source documents (ref checksums) | Not explicitly required in plan | MEDIUM |
| ARM/AArch64 equivalents coverage for SC-13 on AArch64 systems | Yes (included in scope) | None |
| FIPS-allowed driver mapping for `/proc/crypto` | Yes (Phase 1H deliverable) | None |

### 2.4 SP 800-30 Risk Assessment — Identified Gaps

**Gap: SC-13 verification procedure not a named deliverable from Phase 1J**

- **Threat source**: Adversarial (supply chain, cryptographic weakening)
- **Threat event**: AES-NI present in CPU but OpenSSL compiled without hardware acceleration; software AES fallback with known cache-timing side channels is active; system is assessed as FIPS-compliant because the hardware capability is present (Layer 1), but Layer 2/3 utilization is never verified
- **Likelihood of initiation**: Moderate (builds without `-march=native` are common in container images)
- **Likelihood of success**: High (without a Layer 2/3 verification procedure, this gap is invisible to an assessor)
- **Impact**: High — FIPS cryptographic protection claim on SC-13 may be "satisfied" at Layer 1 only; actual encryption uses non-validated software path
- **SP 800-30 risk level**: High → **UMRS HIGH**

**Gap: SP 800-90B analysis blocked; RDRAND/RDSEED classification unresolved**

- **Threat source**: Adversarial (cryptographic weakness, predictable entropy)
- **Threat event**: System uses RDRAND as primary entropy source without verifying compliance with SP 800-90B health test requirements; specific CPU steppings with known RDRAND bugs (CVE-2019-11090 AMD) are not distinguished
- **Likelihood of success**: Moderate (CPU stepping verification is rarely performed)
- **Impact**: High — entropy failure on a FIPS system affects all cryptographic operations
- **SP 800-30 risk level**: High → **UMRS HIGH**

**Gap: Acquired document integrity not verified**

- **Threat source**: Adversarial (supply chain, substituted reference document)
- **Threat event**: A reference document acquired from a vendor portal is subtly altered; incorrect specification guidance flows into the corpus and subsequently into UMRS detection logic
- **Likelihood**: Low (Intel/NIST portals are well-defended)
- **Impact**: Low (detection logic built on wrong specs would fail during testing)
- **SP 800-30 risk level**: Low → **UMRS LOW**

### 2.5 RMF Lifecycle Placement (SP 800-37)

| Plan Component | RMF Step | Task |
|----------------|----------|------|
| Feature inventory and 23-column matrix | Select | S-2 (control tailoring; establishes what hardware must be present for selected controls) |
| Phase 1A-1H research deliverables | Implement (pre-requisite) | I-1 (baseline configuration documentation) |
| Phase 1H `/proc/crypto` reference | Assess (preparation) | A-1 (assessment plan development) |
| Phase 1J SC-13 verification procedure (if added) | Assess | A-3 (control assessment execution) |
| Phase 1K corpus refinement | Implement | I-2 (control implementation) |

The corpus plan currently ends at Implement step preparation. It does not yet reach the Assess step because no assessment procedures are formally defined as deliverables.

### 2.6 "Other Than Satisfied" Flags Under Current Plan

1. **SC-13**: "other than satisfied" for software utilization claim — Layer 1 (hardware) can be verified from corpus; Layer 2/3 (kernel driver and library utilization) assessment procedure is not a named deliverable from any phase. An assessor cannot determine that hardware acceleration is in use without a Test procedure against `/proc/crypto`.
2. **SA-12**: "other than satisfied" for document integrity — acquired reference documents are not required to be checksum-verified against `.claude/references/refs-manifest.md` entries.

---

## Plan 3: UMRS Platform Expansion (Umbrella)

### 3.1 My Review Activities — SP 800-53A Classification

- **Examine**: Reviewed the umbrella plan as a system design document — an SSP narrative component (SA-11 object).
- **Examine**: Reviewed architectural decisions and cross-platform readiness issues against CM-6, SI-7 assessment objects.
- **Examine**: Reviewed compliance citations table against SP 800-53 control family requirements.

### 3.2 SP 800-53A Assessment Procedure Mappings

#### CM-8 — Component Inventory (OS Detection Pillar)

The plan cites CM-8 for OS Detection. SP 800-53A CM-8 assessment requires:

- **Examine**: Component inventory; system security plan; procedures addressing component inventory.
- **Test**: Automated mechanisms supporting maintenance of information system component inventory.

`OsDetector::detect()` produces a `DetectionResult` with `SubstrateIdentity`, `OsRelease`, and `EvidenceBundle`. This maps to the CM-8 Test object: it is an automated mechanism producing component inventory evidence (OS identity, package substrate, confidence tier).

**Gap**: CM-8 requires the inventory to be reviewed and updated at an organization-defined frequency (ODP). The umbrella plan does not specify how `OsDetector::detect()` results are persisted or reviewed. Like the posture probe, the mechanism is correct but the persistence and review loop is absent [SDR-005 PENDING for ODP frequency].

#### SI-7 — Software, Firmware, and Information Integrity (Cross-Platform)

Issue 3 in the cross-platform section (no integration tests for Ubuntu/dpkg path) is a direct SI-7 concern. SP 800-53A SI-7 requires testing of integrity mechanisms. If the dpkg detection path has no test coverage, an assessor cannot verify that the T2 → T3 trust tier logic works on Ubuntu without a manual test.

**This is a clear SP 800-53A Test gap**: the absence of Ubuntu integration tests means the SI-7 Test object (integrity verification mechanism testing) is "other than satisfied" for the dpkg code path.

#### SC-28 — Protection of Information at Rest (DetectionResult Serialization)

The umbrella plan identifies DetectionResult serialization as future work and notes that "validated newtypes must re-validate on deserialization, or the deserialization path must be treated as a trusted boundary with MAC enforcement." This is a precise SC-28 concern: data written to and read from a cache must be protected from unauthorized modification.

The plan correctly identifies the design question (format, sequencing) but defers both the decision and the implementation. From an SP 800-37 perspective, this leaves the SC-28 Implement step incomplete until serialization is decided and built.

**SP 800-53A SC-28 Examine object**: system design documentation showing protection of data at rest. The current plan states the concern but provides no implementation decision — the Examine object would be "other than satisfied" for the cache-at-rest scenario.

### 3.3 Evidence Gaps — What an Assessor Needs

| Evidence Required by Assessor | Plan Produces This? | Gap Severity |
|-------------------------------|---------------------|--------------|
| Ubuntu integration tests for dpkg detection path (SI-7) | No (deferred, marked Medium priority) | HIGH |
| CM-8 inventory persistence / review mechanism | No (not addressed) | MEDIUM |
| SC-28 implementation decision for DetectionResult cache | No (explicitly deferred) | MEDIUM |
| MAC abstraction decision for T3 on Ubuntu (Issue 1) | No (open architectural decision) | MEDIUM |
| Cross-platform Definition of Done test coverage | Partially (checklist items defined but not implemented) | LOW |

### 3.4 SP 800-30 Risk Assessment — Identified Gaps

**Gap: Ubuntu dpkg path untested — SI-7 unverifiable**

- **Threat source**: Adversarial (adversary targeting Ubuntu deployment path)
- **Threat event**: A regression in the dpkg substrate probe causes T3 to be asserted on Ubuntu without valid MAC enforcement verification; the evidence bundle records incorrect confidence; a downstream consumer trusts an inflated trust tier
- **Likelihood of initiation**: Moderate (Ubuntu path is a supported target; regressions are common in untested paths)
- **Likelihood of success**: High (without tests, the regression is invisible until runtime)
- **Impact**: High — confidence tier inflation directly affects authorization decisions that consume `DetectionResult`
- **SP 800-30 risk level**: High → **UMRS HIGH**

**Gap: SC-28 implementation deferred without interim control**

- **Threat source**: Adversarial (cache poisoning)
- **Threat event**: Attacker modifies a cached `DetectionResult`; deserialization reads the tampered result; trust tier is inflated without re-running the detection pipeline
- **Likelihood**: Low currently (cache is in-memory via SEC pattern; on-disk persistence not yet implemented)
- **Impact**: High if on-disk serialization is added before MAC protection is implemented
- **SP 800-30 risk level**: Moderate for current state (in-memory only); would escalate to High if serialization is implemented before SC-28 control — **UMRS MEDIUM** now, with HIGH escalation trigger on serialization decision

**Gap: Open architectural decisions undocumented in SSP**

- **Threat source**: Non-adversarial (organizational process error)
- **Threat event**: An architectural decision (`CpuSignalId` enum, `MandatoryAccessControl` abstraction, serialization format) is made without documenting the rationale; the choice is later questioned during authorization; the AO cannot determine whether the decision was risk-informed
- **Impact**: Low technical impact; Moderate authorization impact (AO risk determination requires documented rationale per SP 800-37 Authorize task R-2)
- **SP 800-30 risk level**: Low → **UMRS LOW**

### 3.5 RMF Lifecycle Placement (SP 800-37)

| Plan Component | RMF Step | Task |
|----------------|----------|------|
| OS Detection (complete) | Implement | I-2 (CM-8 mechanism implemented) |
| Kernel Posture Probe (in progress) | Implement → Monitor | I-2 → M-2 |
| CPU Extension Detection (future) | Select (corpus) → Implement | S-2 → I-2 |
| Cross-platform readiness issues | Implement | I-2 (SI-7 coverage gap) |
| DetectionResult serialization (future) | Implement | I-2 (SC-28 implementation) |
| Open architectural decisions | Authorize | R-1 (risk response selection) |

The umbrella plan spans all active RMF Implement sub-tasks and explicitly names items requiring Authorize-step input (architectural decisions needing Jamie's approval = risk response selection per R-1).

### 3.6 "Other Than Satisfied" Flags Under Current Plan

1. **SI-7 (Ubuntu dpkg path)**: "other than satisfied" — no Test procedure exists for the dpkg detection path integrity; regressions are undetectable.
2. **SC-28 (DetectionResult cache)**: "other than satisfied" for in-transit/at-rest protection of cached results — the design decision is deferred; no interim control documented.
3. **CM-8 (inventory persistence)**: "other than satisfied" — `OsDetector::detect()` produces inventory evidence at runtime but no persistence mechanism ensures the inventory is maintained and reviewed.

---

## New Capabilities Demonstrated

These reviews demonstrate capabilities that were not available to this agent before the rmf-methodology corpus familiarization pass:

### 1. Assessment Method Attribution (SP 800-53A)
Previously, findings were described in terms of "what is missing." Now each gap is explicitly classified as an Examine, Interview, or Test gap with the specific assessment object identified. This allows a real assessor to reproduce the finding using SP 800-53A procedures directly.

### 2. "Satisfied / Other Than Satisfied" Determination
Previously, I used "HIGH/MEDIUM/LOW" with policy rationale. Now each finding maps to a specific SP 800-53A determination statement and concludes "satisfied" or "other than satisfied" with the specific Examine/Test object that is missing. This is the language AOs and assessors use — it eliminates translation loss between UMRS reports and formal authorization packages.

### 3. RMF Lifecycle Placement
Previously, plans were reviewed as standalone technical documents. Now each plan component is mapped to a specific SP 800-37 step and task (e.g., Implement I-2, Monitor M-2, Authorize R-1). This allows a program manager to see where each deliverable fits in the authorization timeline.

### 4. ODP Completeness Awareness [SDR-005 PENDING]
SP 800-53A states that undefined Organization-Defined Parameters produce "other than satisfied" findings. I can now flag ODP gaps in plans (monitoring frequency for CA-7, inventory review frequency for CM-8) as assessment-blocking gaps, not just good-practice recommendations.

### 5. SP 800-30 Risk Characterization in Plan Gaps
Previously, plan gaps were noted as findings with severity labels. Now each gap carries explicit threat source, threat event, likelihood, and impact analysis per SP 800-30 Appendix D/E/F tables. This grounds severity assignments in auditor-reproducible logic rather than judgment.

### 6. Three-Tier Impact Notation (SP 800-39 / SDR-003)
The monitoring frequency gap in the posture probe (CA-7 ODP undefined) has Tier 2 implications: if no monitoring schedule is defined, the mission-level assurance that kernel hardening is maintained over time is also unverifiable. I can now add Tier Impact notes to findings that cross system boundaries.

### 7. Authorization Package Terminology (SDR-004)
I can now map UMRS artifacts to RMF authorization package components:
- These plan reviews → SAR components (SP 800-37 task A-6)
- Identified gaps → POA&M entries (SP 800-37 Authorize/Monitor)
- Crate-level doc comments → SSP narrative sections

---

## Recommendations — Prioritized by SP 800-30 Risk Level

### UMRS HIGH — Address before next authorization decision

**R-1: Define CA-7 monitoring frequency ODP for the kernel posture probe**
- Owner: security-engineer (feeds into SSP and monitoring strategy)
- Action: Define the organization-defined frequency at which `PostureSnapshot::collect()` must be invoked in production (e.g., daily, at each boot, continuously via daemon)
- RMF anchor: SP 800-37 Monitor M-2; SP 800-53A CA-7 assessment ODP
- SP 800-30 risk: High likelihood non-adversarial (tooling unused without schedule) × High impact (CA-7 monitoring gap)

**R-2: Add SC-13 verification procedure as a Phase 1J named deliverable (CPU corpus plan)**
- Owner: security-engineer (Phase 1J review deliverable)
- Action: Add to Phase 1J: "Draft SC-13 Test procedure specifying `/proc/crypto` driver names, priorities, selftest status, and `fips_allowed` flags that must be present for each crypto algorithm on a FIPS-active RHEL 10 system"
- RMF anchor: SP 800-37 Assess A-3; SP 800-53A SC-13 Test object
- SP 800-30 risk: High (Layer 1-only FIPS claim; software fallback active and undetected)

**R-3: Resolve SP 800-90B blocking dependency for RDRAND/RDSEED (CPU corpus plan)**
- Owner: researcher (Phase 1B pre-requisite)
- Action: Acquire NIST SP 800-90B from `https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-90B.pdf`; add to `.claude/references/refs-manifest.md` with SHA-256 checksum; unblock Phase 1B
- RMF anchor: SP 800-37 Select S-2; SC-13 ODP for FIPS entropy source requirements
- SP 800-30 risk: High (entropy quality unverifiable on FIPS systems without 800-90B)

**R-4: Add Ubuntu dpkg integration tests (umbrella plan, Issue 3)**
- Owner: rust-developer
- Action: Implement test seam in `pkg_substrate::run_inner`; write integration tests asserting dpkg success, T3 non-assertion when SELinux check fails, and correct evidence bundle on Ubuntu
- RMF anchor: SP 800-37 Implement I-2; SP 800-53A SI-7 Test object
- SP 800-30 risk: High (trust tier inflation on Ubuntu undetectable without tests)

### UMRS MEDIUM — Address before next major phase

**R-5: Add persistent deviation artifact to posture probe (CM-6(iii))**
- Owner: rust-developer (Phase 2b or 3 scope addition)
- Action: Define a structured output format (machine-readable, e.g., JSON or CBOR) for `PostureSnapshot::findings()` that can be written to disk and consumed as the CM-6 "documented deviations" Examine artifact
- RMF anchor: SP 800-53A CM-6 Examine object: documented deviations from configuration settings

**R-6: Require SHA-256 checksum updates in `.claude/references/refs-manifest.md` for all newly acquired CPU corpus documents**
- Owner: researcher (policy addition to cpu-security-corpus-plan.md)
- Action: Add to each Phase 1x "Post-phase Hygiene" section: "Update `.claude/references/refs-manifest.md` with SHA-256 hash for each newly acquired document"
- RMF anchor: SP 800-53A SA-12 Examine object: supply chain risk assessment

**R-7: Document interim SC-28 control for DetectionResult cache while serialization is deferred**
- Owner: security-engineer / tech-writer
- Action: Add to umbrella plan: "Until DetectionResult serialization is implemented, SC-28 is satisfied by SEC pattern's in-memory HMAC seal (tamper detection only). When on-disk serialization is added, SC-28 implementation must be confirmed before the feature is merged."
- RMF anchor: SP 800-53A SC-28 Examine object: system design documentation

**R-8: Document CM-8 inventory persistence and review mechanism for OS Detection**
- Owner: security-engineer
- Action: Define how `OsDetector::detect()` results are persisted and at what frequency the inventory is reviewed (ODP for CM-8)
- RMF anchor: SP 800-53A CM-8 Examine object: component inventory; SA-12

### UMRS LOW — Address in next documentation cycle

**R-9: Add Tier Impact note to CA-7 ODP gap (SP 800-39 / SDR-003)**
- Owner: tech-writer
- Action: Note in the posture probe architecture documentation that undefined monitoring frequency is a Tier 2 concern: the mission-level assurance that kernel hardening is maintained over time is unverifiable without a defined schedule

**R-10: Document architectural decision rationale for AO visibility**
- Owner: security-engineer / tech-writer
- Action: When Jamie resolves the three open architectural decisions (CpuSignalId, MandatoryAccessControl, serialization format), record the rationale in a formal architectural decision record that can serve as an SP 800-37 Authorize R-1 input artifact

---

## Gap Analysis Summary

```
Plans reviewed: 3
Total findings: 14 (4 HIGH, 5 MEDIUM, 5 LOW)

HIGH findings:
  - CA-7 ODP undefined: no monitoring frequency for PostureSnapshot (posture probe)
  - SC-13 verification procedure missing as Phase 1J deliverable (CPU corpus)
  - SP 800-90B acquisition blocking RDRAND/RDSEED classification (CPU corpus)
  - Ubuntu dpkg integration tests absent: SI-7 Test gap (umbrella)

MEDIUM findings:
  - CM-6(iii) persistent deviation artifact not produced by posture probe
  - SA-12 document integrity: checksum verification not required in CPU corpus phases
  - SC-28 interim control undocumented while DetectionResult serialization is deferred
  - CM-8 inventory persistence and review mechanism undefined for OS Detection
  - SC-13 Layer 2/3 utilization test procedure absent from CPU corpus plan deliverables (partially overlaps R-2 and R-5)

LOW findings:
  - AU-3 signal-level timestamps absent from SignalReport (snapshot-level sufficient)
  - CA-7 gap has Tier 2 mission impact (tier notation missing from plan)
  - Open architectural decisions lack formal decision record for AO input
  - CM-8 review frequency ODP undefined [SDR-005 PENDING]
  - Cross-platform Definition of Done partially specified (checklist defined, implementation deferred)

"Other than satisfied" controls (under current plan state):
  - CA-7 (posture probe): monitoring frequency ODP undefined
  - CM-6(iii) (posture probe): no persistent deviation document
  - SC-13 (CPU corpus): Layer 2/3 utilization assessment procedure not produced
  - SI-7 (umbrella, Ubuntu path): Test object not covered
  - SC-28 (umbrella, DetectionResult): implementation decision deferred, interim control undocumented
  - CM-8 (umbrella, OS Detection): inventory persistence mechanism undefined

Inconsistencies (plan vs. RMF requirements):
  - All three plans correctly implement mechanisms but do not specify how mechanism outputs
    become persisted assessment artifacts. This is a consistent gap across the portfolio:
    strong Implement-step coverage, weak Assess/Monitor-step artifact production.
```
