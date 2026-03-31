---
name: NIST SP 800-161r1-upd1 Familiarization
description: C-SCRM controls familiarization for security-auditor; supply chain dependency verification controls and SBOM requirements
type: project
---

# NIST SP 800-161r1-upd1 Familiarization
## Cybersecurity Supply Chain Risk Management Practices

**Base document:** May 2022 (r1) | **Update:** November 1, 2024 (upd1, approved 2024-09-25)
**Supersedes:** SP 800-161r1 (May 2022, withdrawn)
**File:** `.claude/references/nist/sp800-161r1-upd1.pdf` (309 pages)

---

## 1. Document Structure

| Section | Pages | Content |
|---|---|---|
| Chapters 1–3 | 1–56 | Background, C-SCRM framework, three-tier model |
| References | 57–62 | Normative and informative references |
| Appendix A | 63–171 | C-SCRM Controls — the actionable core |
| Appendix B | 164–171 | Complete control summary table (all families) |
| Appendix C | 172–195 | Risk Exposure Framework — 6 threat scenarios |
| Appendix D | 196–229 | Templates (policies, SLAs, C-SCRM plans) |
| Appendix E | 230–246 | FASCSA compliance |
| Appendix F | 247 | EO 14028 compliance mapping |
| Appendix G | 248–291 | RMF Activities integration |
| Appendix H–K | 292–309 | Glossary, acronyms, resources, revision history |

**Core mechanism:** SP 800-161r1-upd1 is an *overlay* on SP 800-53 Rev 5. It does not
define new controls — it provides supplemental C-SCRM guidance for 20 existing control
families. The guidance is organized by applicability level (L1/L2/L3) and flow-down
indicator (whether a control applies to Nth-tier suppliers).

**Three-level framework:**
- Level 1 — Enterprise (governance, C-SCRM policy, executive oversight)
- Level 2 — Mission/Business Process (program/acquisition)
- Level 3 — Operational/System (system development, integration, O&M)

---

## 2. What Changed in upd1 (vs. withdrawn r1)

Four new controls added — all are C-SCRM additions not present in r1:

| Control | Name | Levels | Notes |
|---|---|---|---|
| AT-3(6) | Supply Chain Training | L1, L2, L3 | Require suppliers to train their supply chain personnel |
| CM-8(10) | SBOM for Open Source Projects | L3 | If OSS lacks SBOM: fund, generate, or contribute generation on first version use |
| MA-8 | Software Maintenance | L2, L3 | Maintenance plans, patch scheduling, EOL tracking |
| SR-13 | Supplier Inventory | L2, L3 | Maintain tier-one supplier inventory (contract IDs, products, programs, criticality) |

**Why this matters for UMRS:** CM-8(10) and SR-13 directly address the Rust crate
dependency landscape. UMRS consumes open-source crates; both controls apply.

---

## 3. Controls That Map to UMRS Work

### High-Priority Controls for security-auditor

**SR-3 — Supply Chain Controls and Processes**
- Require contractual C-SCRM controls flowing down to sub-tier suppliers
- Mandate SBOM delivery from suppliers; define SBOM format requirements
- Related: RA-9 (criticality analysis as prerequisite)

**SR-4 — Provenance**
- Obtain SBOM in NTIA-supported format (SPDX, CycloneDX, SWID)
- SBOMs must be digitally signed with a verifiable, trusted key
- SBOMs complement — do not replace — vulnerability management and vendor risk
- **Direct UMRS mapping:** Rust cargo dependency graph = component provenance chain

**SR-11 — Component Authenticity**
- SR-11(1): Anti-counterfeit training
- SR-11(2): Configuration control for component service/repair
- SR-11(3): Anti-counterfeit scanning — scan received components; *requires RA-9 first*

**CM-7(6) / CM-7(7) — Least Functionality: Code Authentication**
- CM-7(6): Obtain code from verified source; verify digital signatures before execution
- CM-7(7): Prevent downloading without verification

**CM-8(10) — SBOM for Open Source Projects** (NEW in upd1)
- If an OSS component lacks an SBOM, the enterprise must:
  1. Contribute SBOM generation to the project, OR
  2. Fund SBOM generation, OR
  3. Generate one on first consumption of each version used
- Applies at Level 3 only
- **UMRS implication:** `cargo auditable` or `cargo sbom` output satisfies this for UMRS's own build

**CM-14 — Signed Components**
- Verify component genuineness via digital signatures from trustworthy CAs
- Reject components that cannot be verified

**SI-7 — Software, Firmware, and Information Integrity**
- Verify integrity via digital signatures and checksums
- Sandbox execution to detect unauthorized changes
- Related Control: SR-3(3)

**RA-9 — Criticality Analysis** (foundational)
- Identify critical components/services requiring elevated C-SCRM controls
- Required before: SR-3, SR-6, SR-8, SR-9, SR-10, SR-11(3), SR-13
- **UMRS mapping:** Crate criticality tiers (crypto, parsing, IPC) should drive which
  controls apply at higher rigor — RA-9 is the gate control

**SR-13 — Supplier Inventory** (NEW in upd1)
- Maintain tier-one supplier inventory: contract/task order IDs, product descriptions,
  program/system using each supplier, assigned criticality level
- Levels 2 and 3
- **UMRS mapping:** Rust crate manifest + audit trail = supplier inventory analog

**SR-6 — Supplier Assessments and Reviews**
- Conduct supplier risk assessments at defined frequency
- Include sub-tier suppliers for critical components
- **UMRS mapping:** `cargo audit` + dependency review cadence

**SA-9 — External System Services**
- Require C-SCRM practices in external service provider contracts
- Monitor compliance

### Secondary Controls Worth Noting

| Control | Relevance |
|---|---|
| AT-3(6) | Supply chain personnel training (new in upd1) |
| AU-6 | Audit record review — C-SCRM events in audit trail |
| CA-2(3) | Independent assessors for supply chain |
| IR-6 | Incident reporting from suppliers |
| PL-8 | Information security architecture includes supply chain |
| PM-30 | Supply chain risk management strategy (L1, L2 only) |
| SA-15 | Development process, standards, tools — applies to build toolchain |
| SR-8 | Notification agreements — supplier incident notification SLAs |

---

## 4. Appendix C — Risk Exposure Scenario for UMRS

**Scenario 6: Vulnerable Reused Components Within Systems**

This scenario directly maps to Rust's crate dependency model:
- Reused OSS components are integrated without complete vulnerability visibility
- Vulnerable component version is not identified until exploitation
- Mitigations: SBOM generation, component registry monitoring, `cargo audit` cadence,
  criticality-based monitoring frequency (RA-9-derived)

The scenario's recommended controls: CM-8(10), RA-9, SR-3, SR-4, SR-6, SI-7

---

## 5. UMRS Crate / Agent Mapping

| UMRS Component | Relevant Controls | Reason |
|---|---|---|
| `umrs-selinux` | CM-7(6/7), CM-14, SI-7 | Code authentication, integrity verification |
| `umrs-labels` | SR-4, CM-8 | Provenance tracking for label data |
| Build pipeline | CM-8(10), SR-4 | SBOM generation for OSS crates |
| Dependency audit | SR-6, SR-13, RA-9 | Crate supplier inventory + criticality |
| security-auditor agent | All SR family, RA-9 | Primary consumer — C-SCRM evaluation |
| rust-developer agent | AT-3(6), CM-7(6) | Awareness during dependency selection |

---

## 6. Practical Takeaways

1. **SBOM is now required, not optional.** CM-8(10) mandates SBOM for every OSS
   component that lacks one. Add `cargo auditable` to the build pipeline.

2. **SR-4 requires signed SBOMs.** Generated SBOMs must be digitally signed —
   unsigned SBOM output does not satisfy the control.

3. **RA-9 before SR controls.** Criticality analysis must precede anti-counterfeit
   and provenance controls. Document crate criticality tiers.

4. **SR-13 supplier inventory.** Maintain a versioned record of all Rust crates used,
   the UMRS component consuming them, and their assigned criticality.

5. **Scenario 6 is the threat model.** When reviewing dependency updates, this is
   the named threat scenario — use it to frame risk decisions.

**Why:** sp800-161r1-upd1 is a principles + controls document. Security-auditor
applies it as active knowledge when evaluating supply chain posture. No RAG needed.
