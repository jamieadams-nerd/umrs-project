# Reference Document Manifest

This directory contains third-party standards and guidance documents that inform the
security architecture and compliance posture of the UMRS project.

## Integrity Verification

Before using any document, verify its SHA-256 checksum:

```bash
sha256sum .claude/references/nist/*.pdf .claude/references/dod-5200/*.pdf
```

Compare against the values in this manifest. A mismatch means the file was corrupted
or tampered with and must not be used as a reference.

## Update Checks

Periodically ask Claude Code to check for newer versions of these documents. It will
fetch the issuing authority's publication page, compare version/date against this
manifest, and summarize any changes between revisions.

---

## Documents

### NIST SP 800-218 — Secure Software Development Framework (SSDF)

| Field | Value |
|---|---|
| Full title | Secure Software Development Framework (SSDF) Version 1.1 |
| Issuing authority | NIST |
| Version | 1.1 |
| Published | February 2022 |
| Downloaded | 2026-03-01 |
| Local path | `nist/sp800-218-ssdf.pdf` |
| Source URL | https://nvlpubs.nist.gov/nistpubs/specialpublications/nist.sp.800-218.pdf |
| SHA-256 | `617746e553a9e2da49bfbd4eef0dfc3094758a39b869314e4173ac36605cde22` |
| Status | ✓ Downloaded |

**Key relevance**: PW.4 safe code guarantees (`#![forbid(unsafe_code)]`, secure arithmetic,
zeroize, bounds-safe indexing), supply chain hygiene (PO.1), and vulnerability testing (PW.7).

---

### CMMC 2.0 Final Rule — 32 CFR Part 170

| Field | Value |
|---|---|
| Full title | Cybersecurity Maturity Model Certification (CMMC) Program — Final Rule |
| Issuing authority | Department of Defense, Office of the Secretary |
| FR citation | 89 FR 83092 |
| Published | Federal Register, October 15, 2024 (Vol. 89, No. 199) |
| Effective | December 16, 2024 |
| Docket | DoD-2023-OS-0063 / RIN 0790-AL49 |
| Downloaded | 2026-03-12 |
| Local path | `dod/cmmc-32cfr170-final-rule.pdf` |
| Source URL | https://www.govinfo.gov/content/pkg/FR-2024-10-15/pdf/2024-22905.pdf |
| SHA-256 | `34105216b8735f14d42261d2a6794d6f12dacc4fa60bfc30cb2589d1a7dc5fc3` |
| Status | ✓ Downloaded |

**Note**: The previously listed URL (`FR-2023-12-26/pdf/2023-27756.pdf`) was an unrelated
 OMB submission. The proposed rule was 88 FR 89058 (Dec 26, 2023, document 2023-27280);
this is the **final rule** published October 2024.

**Key relevance**: Legal authority for CMMC Level 2 requirements; defines the regulatory
basis for CUI protection practices in this codebase. Establishes 32 CFR Part 170 with
a 4-phase implementation plan over three years.

---

### CMMC Assessment Guide — Level 2

| Field | Value |
|---|---|
| Full title | CMMC Assessment Guide Level 2 |
| Issuing authority | Department of Defense (DoD CIO) |
| Version | 2.13 |
| Published | September 2024 |
| Document ID | DoD-CIO-00003 (ZRIN 0790-ZA19), 24-T-0461 |
| Downloaded | 2026-03-12 |
| Local path | `dod/cmmc-assessment-guide-l2.pdf` |
| Source URL | https://dodcio.defense.gov/Portals/0/Documents/CMMC/AssessmentGuideL2v2.pdf |
| SHA-256 | `0dcaba1626a0d23893981d74dd3f0f2338fff54cece67d81bc811ce76392d867` |
| Status | ✓ Downloaded |

**Note**: The previous URL (`AssessmentGuide_L2.pdf`) returned 404 — the document was
renamed to `AssessmentGuideL2v2.pdf` as part of the v2.x series. The `dodcio.defense.gov`
domain now redirects to `dowcio.war.gov` (Akamai CDN); the content path is unchanged.

**Key relevance**: Defines the specific assessment objectives and evidence requirements
for each CMMC Level 2 practice; maps directly to NIST 800-171 controls.

---

---

### NIST SP 800-171 Rev 2 — Protecting CUI in Nonfederal Systems

| Field | Value |
|---|---|
| Full title | Protecting Controlled Unclassified Information in Nonfederal Systems and Organizations |
| Issuing authority | NIST |
| Version | Rev 2 |
| Published | February 2020 |
| Downloaded | 2026-03-01 |
| Local path | `nist/sp800-171r2.pdf` |
| Source URL | https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-171r2.pdf |
| SHA-256 | `298bdbfcf6a4890a564b225c893230a0b32b2e69e3b98dd898aaeb1d544c5e12` |
| Status | ✓ Downloaded |

**Key relevance**: Defines the 110 security requirements for CUI protection that map directly
to CMMC Level 2 practices. The authoritative source for understanding what controls this
codebase must support.

---

---

### NIST SP 800-53 Rev 5 — Security and Privacy Controls

| Field | Value |
|---|---|
| Full title | Security and Privacy Controls for Information Systems and Organizations |
| Issuing authority | NIST |
| Version | Rev 5 |
| Published | September 2020 |
| Downloaded | 2026-03-01 |
| Local path | `nist/sp800-53r5.pdf` |
| Source URL | https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-53r5.pdf |
| SHA-256 | `fc63bcd61715d0181dd8e85998b1e6201ae3515fc6626102101cab1841e11ec6` |
| Status | ✓ Downloaded |

**Key relevance**: The authoritative control catalog. Every compliance annotation in this
codebase (AC-4, SI-7, MP-4, AU-3, etc.) references controls defined here.

---

---

### FIPS 140-2 — Security Requirements for Cryptographic Modules

| Field | Value |
|---|---|
| Full title | Security Requirements for Cryptographic Modules |
| Issuing authority | NIST |
| Version | FIPS 140-2 |
| Published | May 2001 (with change notices through 2002) |
| Downloaded | 2026-03-01 |
| Local path | `nist/fips/fips140-2.pdf` |
| Source URL | https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.140-2.pdf |
| SHA-256 | `0df0fdd676df643874adfbbf767b8508af3cc6520aff5cfef7d39428e1c63cd4` |
| Status | ✓ Downloaded |

**Key relevance**: Still the operative validation standard in most DoD and federal contracts.
Any cryptographic primitive used in this codebase must be from a FIPS 140-2 validated module.
RHEL 10 ships OpenSSL with a validated FIPS provider — Rust code must route crypto through it.

---

### FIPS 140-3 — Security Requirements for Cryptographic Modules

| Field | Value |
|---|---|
| Full title | Security Requirements for Cryptographic Modules |
| Issuing authority | NIST |
| Version | FIPS 140-3 |
| Published | March 2019 |
| Downloaded | 2026-03-01 |
| Local path | `nist/fips/fips140-3.pdf` |
| Source URL | https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.140-3.pdf |
| SHA-256 | `942a4f929dfbd2b4af2e4e03df7f6e6377054346afd9bee346ed0ebac5db384b` |
| Status | ✓ Downloaded |

**Key relevance**: The current standard — supersedes 140-2. New validations are issued under
140-3. Understand both: contracts often still cite 140-2 validated modules while new modules
are validated under 140-3.

---

### FIPS 180-4 — Secure Hash Standard (SHS)

| Field | Value |
|---|---|
| Full title | Secure Hash Standard (SHS) |
| Issuing authority | NIST |
| Version | FIPS 180-4 |
| Published | August 2015 |
| Downloaded | 2026-04-02 |
| Local path | `nist/fips180-4.pdf` |
| Source URL | https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.180-4.pdf |
| SHA-256 | `0455b406d89648d20cbde375561e19c245b9815e894164c2670772e3d54deb82` |
| Status | ✓ Downloaded |

**Key relevance**: Defines SHA-1, SHA-224, SHA-256, SHA-384, SHA-512, and the SHA-512/t
variants. The `umrs-c2pa` crate uses SHA-256 for file integrity digests and SHA-384 for
C2PA content binding — both are defined here. Compliance annotations in `umrs-c2pa` source
code cite FIPS 180-4 directly. Required reference whenever NIST SP 800-53 SC-13 or CMMC
cryptographic controls are cited alongside hash algorithm usage.

---

### FIPS 186-5 — Digital Signature Standard (DSS)

| Field | Value |
|---|---|
| Full title | Digital Signature Standard (DSS) |
| Issuing authority | NIST |
| Version | FIPS 186-5 |
| Published | February 2023 |
| Downloaded | 2026-04-02 |
| Local path | `nist/fips186-5.pdf` |
| Source URL | https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.186-5.pdf |
| SHA-256 | `fbb9c7c2ba442f03c57b63b43c888311903c9d0f29f89b06efdebd9b619140c5` |
| Status | ✓ Downloaded |

**Key relevance**: Defines DSA, RSA, and ECDSA (including the P-256 and P-384 curves).
The `umrs-c2pa` crate uses ECDSA for C2PA manifest signing — the algorithm specification
and parameter requirements are defined here. FIPS 186-5 supersedes FIPS 186-4; notably,
it removes DSA and retains RSA and ECDSA. Compliance annotations in `umrs-c2pa` source
code cite FIPS 186-5 directly. Required reference whenever NIST SP 800-53 SC-13 or
CMMC cryptographic controls are cited alongside digital signature usage.

---

### NIST SP 800-90B — Recommendation for the Entropy Sources Used for Random Bit Generation

| Field | Value |
|---|---|
| Full title | Recommendation for the Entropy Sources Used for Random Bit Generation |
| Issuing authority | NIST |
| Version | Initial Public Release |
| Published | January 2018 |
| Downloaded | 2026-03-18 |
| Local path | `nist/sp800-90B.pdf` |
| Source URL | https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-90B.pdf |
| SHA-256 | `9b0dd77131ade3617a91cd8457fa09e0dc354c273bb2220a6afeaca16e5defe7` |
| Status | ✓ Downloaded |

**Key relevance**: Defines requirements for entropy sources used in random bit generators
(RBGs). Directly relevant to `umrs-hw` and any code path that acquires hardware entropy
(RDSEED, RDRAND, `/dev/random`). SC-13 requires FIPS 140-validated cryptography; 800-90B
defines how the entropy feeding those algorithms must be validated. Blocking document for
the CPU security corpus Phase 1B work: governs hardware entropy quality guarantees that
underpin FIPS-mode key generation on RHEL 10.

---

---

### NIST SP 800-171 Rev 3 — Protecting CUI in Nonfederal Systems

| Field | Value |
|---|---|
| Full title | Protecting Controlled Unclassified Information in Nonfederal Systems and Organizations |
| Issuing authority | NIST |
| Version | Rev 3 |
| Published | May 2024 |
| Downloaded | 2026-03-01 |
| Local path | `nist/sp800-171r3.pdf` |
| Source URL | https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-171r3.pdf |
| SHA-256 | `3e4631df8b5d61f40a6e542b52779ef30ddbbfff31e09214fa94ad6e6f5e6d08` |
| Status | ✓ Downloaded |

**Key relevance**: Supersedes Rev 2. Restructured to align more closely with NIST SP 800-53
Rev 5 control families. New requirements added in areas including supply chain risk, software
integrity, and configuration management. CMMC assessments are beginning to reference Rev 3.

---

---

### NIST SP 800-171A Rev 3 — Assessing CUI Security Requirements

| Field | Value |
|---|---|
| Full title | Assessing Enhanced Security Requirements for Controlled Unclassified Information |
| Issuing authority | NIST |
| Version | Rev 3 |
| Published | May 2024 |
| Downloaded | 2026-03-12 |
| Local path | `nist/sp800-171Ar3.pdf` |
| Source URL | https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-171Ar3.pdf |
| SHA-256 | `946d963707cdaba19901c49d5c89517adb00844fe5d101e9dac7febc68e34cfa` |
| Status | ✓ Downloaded |

**Key relevance**: Companion to SP 800-171 Rev 3. Defines the assessment procedures (objectives,
methods, objects) for each of the 110 CUI security requirements. Essential for CMMC Level 2
assessment preparation — maps each control to specific evidence artifacts and test methods.
Directly informs what UMRS components must demonstrate and log to satisfy each requirement.

---

---

### NIST SP 800-172 — Enhanced Security Requirements for Protecting CUI

| Field | Value |
|---|---|
| Full title | Enhanced Security Requirements for Protecting Controlled Unclassified Information: A Supplement to NIST Special Publication 800-171 |
| Issuing authority | NIST |
| Version | Feb 2021 (Final) |
| Published | February 2021 |
| Downloaded | 2026-03-23 |
| Local path | `nist/sp800-172.pdf` |
| Source URL | https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-172.pdf |
| SHA-256 | `5d8a04c46d6972505ddd52da6e84725161358fb334751e9a12d94fe0b64c8302` |
| Status | ✓ Downloaded |
| Relevance | umrs-selinux, CMMC Level 3, AC-4, SC-3, SC-7, SI-3; enhanced CUI requirements for APT threat context |

**Key relevance**: Supplement to SP 800-171. Addresses protection of CUI from Advanced Persistent
Threats (APTs). Defines 35 enhanced requirements across 14 families beyond the 110 in 800-171.
Critical reference for CMMC Level 3 (formerly Level 4/5). Relevant to UMRS's MLS/MCS labeling
framework when operating at high-assurance protection levels against sophisticated adversaries.

---

### NIST SP 800-161 Rev 1 — Cybersecurity Supply Chain Risk Management

| Field | Value |
|---|---|
| Full title | Cybersecurity Supply Chain Risk Management Practices for Systems and Organizations |
| Issuing authority | NIST |
| Version | Rev 1 |
| Published | May 2022 |
| Downloaded | 2026-03-23 |
| Local path | `nist/sp800-161r1.pdf` |
| Source URL | https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-161r1.pdf |
| SHA-256 | `c8250b1ea542640a7cffc145be08c6b17ff76bd695f8c0ac1b40c49d45355d89` |
| Status | ⚠ Withdrawn — superseded by SP 800-161r1-upd1 (November 1, 2024). Do not cite for compliance. Acquire upd1. |
| Relevance | umrs-selinux, SA-12, SR family, CMMC SR.L2; supply chain risk management for Rust crates and OS packages |

**Key relevance**: Comprehensive C-SCRM guidance aligned to NIST SP 800-53 Rev 5 SR (Supply Chain
Risk Management) family. Directly applicable to UMRS's scrutiny of Rust crate dependencies and
RHEL package provenance. Informs the crate vetting checklist and component integrity verification
requirements. CMMC Level 2 includes SR.L2 requirements that reference this publication.

**WARNING**: This document was formally withdrawn by NIST on November 1, 2024 and replaced by
SP 800-161r1-upd1. The local copy is retained for reference only. Do not cite this version in
compliance documentation, control mappings, or design rationale. See the entry below for the
current version: SP 800-161r1-upd1.

---

### NIST SP 800-161 Rev 1 Upd 1 — Cybersecurity Supply Chain Risk Management (Current)

| Field | Value |
|---|---|
| Full title | Cybersecurity Supply Chain Risk Management Practices for Systems and Organizations |
| Issuing authority | NIST |
| Version | Rev 1, Update 1 |
| Published | November 1, 2024 |
| Downloaded | 2026-03-31 |
| Local path | `nist/sp800-161r1-upd1.pdf` |
| Source URL | https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-161r1-upd1.pdf |
| SHA-256 | `d2bacbf4053adbbe11628f74f071077d8fd59ba99754a3aab34d7813c1cb3d40` |
| Status | ✓ Downloaded |
| Relevance | umrs-selinux, SA-12, SR family, CMMC SR.L2; supply chain risk management for Rust crates and OS packages |

**Key relevance**: Current (Update 1) version of the C-SCRM guidance, superseding the withdrawn
Rev 1 (May 2022). Fully aligned to NIST SP 800-53 Rev 5 SR (Supply Chain Risk Management) family.
Directly applicable to UMRS scrutiny of Rust crate dependencies and RHEL package provenance.
Informs the crate vetting checklist and component integrity verification requirements. CMMC Level 2
SR.L2 requirements reference this publication. Cite this version — not `sp800-161r1.pdf` — in all
compliance documentation, control mappings, and design rationale.

---

### NIST SP 800-60 Vol 1 Rev 1 — Guide for Mapping Information Types to Security Categories

| Field | Value |
|---|---|
| Full title | Guide for Mapping Types of Information and Information Systems to Security Categories |
| Issuing authority | NIST |
| Version | Volume 1, Rev 1 |
| Published | August 2008 |
| Downloaded | 2026-03-23 |
| Local path | `nist/sp800-60v1r1.pdf` |
| Source URL | https://nvlpubs.nist.gov/nistpubs/Legacy/SP/nistspecialpublication800-60v1r1.pdf |
| SHA-256 | `6f13f57f11697efcba694d0c9935c99168512f737cc76f2c432f248550128460` |
| Status | ✓ Downloaded |
| Relevance | umrs-selinux MLS/MCS, RA-2, FIPS 199, information categorization, CUI classification mapping |

**Key relevance**: Establishes the FIPS 199 information categorization methodology. Defines how
information types map to LOW/MODERATE/HIGH security categories (confidentiality, integrity,
availability). Foundational for MLS/MCS label design — UMRS's Protected A/B/C mapping and CUI
categorization align to the categorization framework defined here. Companion Volume 2 defines
specific government information type mappings.

---

### NIST SP 800-60 Vol 2 Rev 1 — Appendices to Guide for Mapping Information Types to Security Categories

| Field | Value |
|---|---|
| Full title | Guide for Mapping Types of Information and Information Systems to Security Categories: Appendices |
| Issuing authority | NIST |
| Version | Volume 2, Rev 1 |
| Published | August 2008 |
| Downloaded | 2026-03-31 |
| Local path | `nist/sp800-60v2r1.pdf` |
| Source URL | https://nvlpubs.nist.gov/nistpubs/Legacy/SP/nistspecialpublication800-60v2r1.pdf |
| SHA-256 | `0b4c5128b39a90f1bb1c1004f22bfe1fa34110222da740011f86c050983dc8de` |
| Status | ✓ Downloaded |
| Relevance | umrs-selinux MLS/MCS, RA-2, FIPS 199, information categorization, CUI classification mapping, government information types |

**Key relevance**: The concrete mapping companion to Volume 1. Contains the authoritative
LOW/MODERATE/HIGH security category assignments for specific government information types
across all federal mission areas (e.g., Law Enforcement, Intelligence, Financial Management).
Directly informs UMRS MCS category range design: the US CUI category numbers assigned to
categories like Law Enforcement (LEI, INV) and Privacy (PRVCY) should be cross-checked against
the CIA-impact baselines defined here. Essential reference when validating that UMRS label
assignments reflect the federal information type hierarchy.

---

---

## Research Reports

Researcher-authored synthesis reports live at `.claude/references/reports/`. Each report
combines findings from the RAG collections, codebase analysis, and official
reference documents. Reports are produced on request; the directory is the
permanent, stable location — always look here first.

### Kernel Module Loading Controls & SELinux Context Change Security

| Field | Value |
|---|---|
| File | `.claude/references/reports/kernel-selinux-module-context-security.md` |
| Date | 2026-03-10 |
| SHA-256 | `15aa1f21d577eac0dd4eca84081f02c73cc6a8feee07784ddeb7e4dfddde453e` |
| Topics | `modules_disabled` sysctl, `MODULE_SIG_FORCE`, Lockdown LSM, LoadPin LSM, IPE LSM, SELinux `system:module_request`, `process:dyntransition`/`setcurrent`, MLS `mlsvalidatetrans`, Bell-LaPadula tranquility |
| Relevance | `umrs-platform` (`ModuleLoadLatch`, `ProcFips`), `umrs-selinux` (`MlsRange`, `MlsLevel::dominates`), `umrs-logspace`, `AC-3`, `AC-4`, `CM-6`, `CM-7`, `SC-13`, `SC-28`, `SI-4`, `SI-7`, `AU-12` |

---

### STIG Signal Coverage — Phase 3 Posture Probe Cross-Reference

| Field | Value |
|---|---|
| File | `.claude/references/reports/stig-signal-coverage.md` |
| Date | 2026-03-17 |
| Topics | UMRS posture indicator → STIG rule mapping, coverage gaps, CMMC alignment, severity cross-reference, candidate new indicators |
| Relevance | `umrs-platform` (posture catalog), `umrs-selinux`, CMMC, `CM-6`, `CM-7`, `SC-7`, `SC-13`, `AU-5`, `AU-12`, `SI-16` |

---

### UMRS Capabilities — NIST SP 800-171 Rev 3 and CMMC Level 2 Mandate Mapping

| Field | Value |
|---|---|
| File | `.claude/references/reports/umrs-capabilities-800-171r3-mapping.md` |
| Date | 2026-03-19 |
| SHA-256 | `2868468e48e6a49350389014c5e834c2ca0e3fc373084927bfd491983da2165a` |
| Topics | 800-171r3 mandate mapping, CMMC L2 practice mapping, CUI labeling, handling restrictions, SELinux posture, typed findings, security engineering, memory safety, orthogonal category composition |
| Relevance | `umrs-selinux`, `umrs-platform`, `cui-labels`, blog post compliance claims, `AC-3`, `AC-4`, `AU-3`, `CM-6`, `CA-7`, `MP-3`, `PL-4`, `SA-8`, `SI-2`, SSDF PW.4 |

---

### NARA CUI Registry Cross-Reference

| Field | Value |
|---|---|
| File | `.claude/references/reports/nara-cui-registry-crossref.md` |
| Date | 2026-03-21 |
| SHA-256 | `75f7d3eb4204fbc6f0565db2588b70ce4e105cced323da3418cec3105d3a1f29` |
| Topics | NARA CUI registry canonical abbreviations vs `cui-labels.json`; standalone-vs-grouped placement for CTI/NNPI/OPSEC/PROT/PSEC/RAIL; 18 OURS_ONLY entries audit; fabricated entries (CHEM, PCI, RECS, LEGL, TRANSPORT, AVIATION, MARITIME, PIPELINE); wrong abbreviations (AMNT→CRITAN, BSA→FSEC, PROC→PROCURE, PRIVACY→PRVCY, HEALTH→HLTH, EXPORT→EXPT, EMERGENCY→EMGT); FEDCON misclassified as category |
| Relevance | `umrs-cui`, `cui-labels.json`, CUI marking compliance, `AC-4`, CMMC |

---

### Agent Knowledge Acquisition Plan — HCI, IA, KO, and Technical Communication

| Field | Value |
|---|---|
| File | `.claude/references/reports/agent-knowledge-acquisition-plan.md` |
| Date | 2026-03-21 |
| SHA-256 | `0ac16788b256f4c26dac13fb254741065387a275a8e527d1f4806b5a690dd09b` |
| Topics | HCI foundational texts, information architecture discipline, knowledge organization theory, technical communication academic theory, YouTube transcript RAG pipeline logistics, open-access source inventory, prioritized acquisition plan with Tier 1/2/3/4 sequencing |
| Relevance | Elena (senior-tech-writer), Sage, Simone educational background materialization; `.claude/references/` RAG collection design; UMRS documentation quality improvement; CUI taxonomy design; posture catalog classification design |

---

### Priority 4 NIST Familiarization — SP 800-172, SP 800-161r1, SP 800-60 Vol 1

| Field | Value |
|---|---|
| File | `.claude/references/reports/2026-03-31-priority4-nist-familiarization.md` |
| Date | 2026-03-31 |
| SHA-256 | `70bd815797b414b55289a8a61c2d954e0ed12688be06f43fd5e70703d845b49e` |
| Topics | SP 800-172 enhanced CUI requirements, CMMC Level 3 delta from Level 2, APT threat framing, C-SCRM framework (SP 800-161r1 — withdrawn), FIPS 199 information categorization methodology (SP 800-60), RAG ingestion decisions, pending acquisitions (SP 800-161r1-upd1 and SP 800-60 Vol 2) |
| Relevance | `umrs-selinux`, `umrs-labels`, `umrs-hw`, Knox (dependency scrutiny), future assessment engine (G4); SC.3.177, SR family, AC-4, RA-2, FIPS 199 |

---

## Access Control Reference Collection

Documents retrieved per research-plan-access-control.md (2026-03-11).
Location: `.claude/references/access-control/`
Status: Awaiting review before RAG ingestion.

### foundational-papers/

#### Bell-LaPadula (Unified Exposition, 1976)

| Field | Value |
|---|---|
| Full title | Secure Computer System: Unified Exposition and Multics Interpretation |
| Authors | D. Elliott Bell, Leonard J. LaPadula |
| Version | ESD-TR-75-306, MTR-2997 Rev. 1 |
| Published | 1976 |
| Downloaded | 2026-03-11 |
| Local path | `.claude/references/access-control/foundational-papers/bell-lapadula-1976-unified.pdf` |
| Source URL | https://csrc.nist.gov/files/pubs/conference/1998/10/08/proceedings-of-the-21st-nissc-1998/final/docs/early-cs-papers/bell76.pdf |
| SHA-256 | `1ab845a7114bcc3c21eac08dd26cfb70a9003836015ddccf8bc5be808bb24fbb` |
| Status | ✓ Downloaded |

Note: The original 1973 three-volume ESD-TR-73-278 was requested but blocked by DTIC automated-access controls. This 1976 NIST CSRC-hosted unified exposition is the canonical single-document form of the model. Filename saved as `bell-lapadula-1976-unified.pdf` (not `bell-lapadula-1973.pdf` as originally planned).

**Key relevance**: Foundational confidentiality model (no-read-up, no-write-down, star property). Directly underpins `MlsLevel::dominates`, `MlsRange`, lattice enforcement in `umrs-selinux`. `AC-4`, `SC-16`.

---

#### Biba Integrity Model (1977)

| Field | Value |
|---|---|
| Full title | Integrity Considerations for Secure Computer Systems |
| Authors | K. J. Biba |
| Version | ESD-TR-76-372, MTR-3153 Rev. 1 |
| Published | April 1977 |
| Downloaded | 2026-03-11 |
| Local path | `.claude/references/access-control/foundational-papers/biba-1977.pdf` |
| Source URL | https://apps.dtic.mil/sti/tr/pdf/ADA039324.pdf |
| SHA-256 | `6c6a1698ff5ebe1b7929a619bb9c6ccdc436b1e21d4d0ebaa31de5cfe080178a` |
| Status | ✓ Downloaded |

**Key relevance**: Integrity dual to Bell-LaPadula. Relevant to MLS integrity constraints and write-up prohibition in `umrs-selinux`. `AC-4`, `SC-16`.

---

#### Brewer-Nash (Chinese Wall) (1989)

| Field | Value |
|---|---|
| Full title | The Chinese Wall Security Policy |
| Authors | David F.C. Brewer, Michael J. Nash |
| Published | IEEE Symposium on Security and Privacy, 1989 |
| Downloaded | 2026-03-11 |
| Local path | `.claude/references/access-control/foundational-papers/brewer-nash-1989.pdf` |
| Source URL | https://www.cs.purdue.edu/homes/ninghui/readings/AccessControl/brewer_nash_89.pdf |
| SHA-256 | `2bcc9dfe18eb1c1d0647de8fccfd5e863c9cee7f1465dee266fabba3a17fb283` |
| Status | ✓ Downloaded |

Note: Purdue academic mirror of IEEE paper (DOI: 10.1109/SECPRI.1989.36295). Provenance: Purdue CERIAS course reading. IEEE is an approved source for academic papers; this is a personal-use mirror.

**Key relevance**: Conflict-of-interest policies, dynamic access control. Relevant to multi-tenant CUI data isolation in `cui-labels` and future MLS compartment design.

---

#### Saltzer and Schroeder (1975)

| Field | Value |
|---|---|
| Full title | The Protection of Information in Computer Systems |
| Authors | Jerome H. Saltzer, Michael D. Schroeder |
| Published | Proceedings of the IEEE, 1975 |
| Downloaded | 2026-03-11 |
| Local path | `.claude/references/access-control/foundational-papers/saltzer-schroeder.html` |
| Source URL | https://www.cs.virginia.edu/~evans/cs551/saltzer/ |
| SHA-256 | `a05a95aa3951626594df2c92f468483ba00ca9d5c2f34cd5a0bd146e3913d30b` |
| Status | ✓ Downloaded (HTML) |

**Key relevance**: Eight foundational security design principles (least privilege, complete mediation, fail-safe defaults, economy of mechanism, open design, separation of privilege, least common mechanism, psychological acceptability). Directly cited by NSA RTB and NIST 800-53. Foundational to every UMRS design decision.

---

#### Clark-Wilson (1987)

| Field | Value |
|---|---|
| Full title | A Comparison of Commercial and Military Computer Security Policies |
| Authors | David D. Clark, David R. Wilson |
| Published | IEEE Symposium on Security and Privacy, 1987 (DOI: 10.1109/SP.1987.10001) |
| Downloaded | — |
| Local path | `.claude/references/access-control/foundational-papers/clark-wilson-1987.pdf` |
| Source URL | — |
| SHA-256 | — |
| Status | ⚠ Requires manual download — IEEE Xplore paywall; original Cornell URL (404); no verified free mirror found |

**Manual download instructions**: Access via IEEE Xplore (DOI: 10.1109/SP.1987.10001) through institutional access or IEEE membership. Save PDF to the path above, run `sha256sum`, and record the value here.

**Key relevance**: Well-formed transactions, separation of duties in commercial integrity policy. Underpins `vaultmgr` transaction design and Clark-Wilson-style constrained data item / transformation procedure modeling.

---

#### Graham-Denning Model

| Field | Value |
|---|---|
| Full title | Protection in Operating Systems |
| Authors | G. S. Graham, Peter J. Denning |
| Published | Communications of the ACM, 1972 |
| Downloaded | — |
| Local path | `.claude/references/access-control/foundational-papers/graham-denning.pdf` |
| Source URL | https://dl.acm.org/doi/pdf/10.1145/361011.361067 |
| SHA-256 | — |
| Status | ⚠ Requires manual download — ACM Digital Library requires subscription or institutional access |

**Manual download instructions**: Access via ACM DL (DOI: 10.1145/361011.361067) with institutional or ACM member access.

---

#### HRU Model

| Field | Value |
|---|---|
| Full title | Protection in Operating Systems (HRU) |
| Authors | Michael Harrison, Walter Ruzzo, Jeffrey Ullman |
| Published | Communications of the ACM, 1976 |
| Downloaded | — |
| Local path | `.claude/references/access-control/foundational-papers/hru-model.pdf` |
| Source URL | https://dl.acm.org/doi/pdf/10.1145/360051.360056 |
| SHA-256 | — |
| Status | ⚠ Requires manual download — ACM Digital Library requires subscription or institutional access |

**Manual download instructions**: Access via ACM DL (DOI: 10.1145/360051.360056) with institutional or ACM member access.

---

### standards/

#### NIST SP 800-53 Rev 5 (copy from .claude/references/nist/)

| Field | Value |
|---|---|
| Full title | Security and Privacy Controls for Information Systems and Organizations |
| Issuing authority | NIST |
| Version | Rev 5 |
| Downloaded | 2026-03-01 (original); copied to collection 2026-03-11 |
| Local path | `.claude/references/access-control/standards/nist-sp-800-53r5.pdf` |
| Source URL | https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-53r5.pdf |
| SHA-256 | `fc63bcd61715d0181dd8e85998b1e6201ae3515fc6626102101cab1841e11ec6` |
| Status | ✓ Downloaded (copied from .claude/references/nist/sp800-53r5.pdf) |

**Key relevance**: Authoritative control catalog — AC, SC, SI, AU, CM family controls.

---

#### NIST SP 800-162 — ABAC Guide

| Field | Value |
|---|---|
| Full title | Guide to Attribute Based Access Control (ABAC) Definition and Considerations |
| Issuing authority | NIST |
| Published | January 2014 |
| Downloaded | 2026-03-11 |
| Local path | `.claude/references/access-control/standards/nist-sp-800-162.pdf` |
| Source URL | https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-162.pdf |
| SHA-256 | `ea90ee967a7011c8de5cfaffe0820d157167edefdea5fc2a84af730af4295cd7` |
| Status | ✓ Downloaded |

**Key relevance**: ABAC concepts directly applicable to CUI label-based access decisions in `umrs-selinux`, `cui-labels`. `AC-2`, `AC-3`, `AC-4`, `AC-16`.

---

#### NIST SP 800-207 — Zero Trust Architecture

| Field | Value |
|---|---|
| Full title | Zero Trust Architecture |
| Issuing authority | NIST |
| Published | August 2020 |
| Downloaded | 2026-03-11 |
| Local path | `.claude/references/access-control/standards/nist-sp-800-207.pdf` |
| Source URL | https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-207.pdf |
| SHA-256 | `0290d6ece24874287316f4bf430fef770aa4ec08a2227c8f2c1e5b2ff975e03d` |
| Status | ✓ Downloaded |

**Key relevance**: Zero-trust principles (never trust, always verify) directly applicable to provenance verification patterns in `umrs-platform`, network posture of deployed binaries. `AC-4`, `SC-7`, `IA-3`.

---

### os-security/

#### SELinux Notebook (2024)

| Field | Value |
|---|---|
| Full title | The SELinux Notebook |
| Issuing authority | SELinux Project (github.com/SELinuxProject) |
| Version | Release 20240430 |
| Downloaded | 2026-03-11 |
| Local path | `.claude/references/access-control/os-security/selinux-notebook.pdf` |
| Source URL | https://github.com/SELinuxProject/selinux-notebook/releases/download/20240430/SELinux_Notebook.pdf |
| SHA-256 | `ac37709fdb64b16ffc74814247d551b71d4302015c4474ddb8ee73e023541317` |
| Status | ✓ Downloaded |

Note: The `.claude/references/selinux-notebook/20240430/` directory contains markdown source but no PDF. This PDF is a separate compiled artifact from the official release, not a duplicate.

**Key relevance**: Comprehensive SELinux reference for all `umrs-selinux` work: policy language, type enforcement, MLS/MCS, labeling, xattrs, access vectors. `AC-3`, `AC-4`.

---

#### Linux Capabilities Manual Page

| Field | Value |
|---|---|
| Full title | capabilities(7) — Linux Manual Page |
| Issuing authority | kernel.org / man-pages project |
| Version | man-pages 6.16 (2025-10-05) |
| Downloaded | 2026-03-11 |
| Local path | `.claude/references/access-control/os-security/linux-capabilities.html` |
| Source URL | https://man7.org/linux/man-pages/man7/capabilities.7.html |
| SHA-256 | `998883a9b8f461e490888ab39217b7743e17d0e48aed0d21155d20de4d0a2b9a` |
| Status | ✓ Downloaded |

**Key relevance**: Linux capability model interacts with SELinux enforcement; relevant to privilege minimization design and `CAP_MAC_ADMIN`/`CAP_MAC_OVERRIDE` controls. `AC-6`, `CM-7`.

---

#### POSIX ACL Manual Page

| Field | Value |
|---|---|
| Full title | acl(5) — Linux Manual Page |
| Issuing authority | kernel.org / man-pages project |
| Downloaded | 2026-03-11 |
| Local path | `.claude/references/access-control/os-security/posix-acl.html` |
| Source URL | https://man7.org/linux/man-pages/man5/acl.5.html |
| SHA-256 | `1e63b0e71360b4807f8a6e6e3968b764e10d4a7ccde4b87a7d08f5c7d99c67de` |
| Status | ✓ Downloaded |

**Key relevance**: POSIX ACL interaction with SELinux DAC+MAC layering; relevant to `SecureDirent` mode/permission reporting. `AC-3`, `AC-6`.

---

### rule-catalog/

Seven YAML rule files created locally (no web retrieval). All stored at `.claude/references/access-control/rule-catalog/`.

| File | Rule ID | Title |
|---|---|---|
| `AC-001.yaml` | AC-001 | Principle of Least Privilege |
| `AC-002.yaml` | AC-002 | Complete Mediation |
| `AC-003.yaml` | AC-003 | Separation of Duties |
| `MLS-001.yaml` | MLS-001 | Bell-LaPadula Simple Security Property (no read up) |
| `MLS-002.yaml` | MLS-002 | Bell-LaPadula Star Property (no write down) |
| `INTEGRITY-001.yaml` | INTEGRITY-001 | Biba Simple Integrity Property (no read down) |
| `INTEGRITY-002.yaml` | INTEGRITY-002 | Biba Star Integrity Property (no write up) |

---

---

## DoD 5200.01 — Information Security Program

These documents implement the DoD Information Security Program for classified and CUI.
Source: esd.whs.mil (official DoD Issuances portal, Washington Headquarters Services).

### DoDI 5200.01 — DoD Information Security Program and Protection of SCI

```
name: DoDI 5200.01 — DoD Information Security Program and Protection of SCI
path: .claude/references/dod-5200/dodi-5200.01.pdf
version: April 21, 2016 (incorporating Change 3)
source_url: https://www.esd.whs.mil/Portals/54/Documents/DD/issuances/dodi/520001p.pdf
date_retrieved: 2026-03-12
sha256: 206eaef84331885b2f9700ed86f5579da80c3119ab4a5c6707e8ed349b2f35e6
relevance: umrs-selinux, cui-labels, deployment, AC-1, AC-2, AC-3, AC-4, SC-16, MP-1
status: downloaded
```

Establishes responsibilities for collateral, SAP, SCI, and CUI within the DoD
Information Security Program. Authorizing directive for the DoDM 5200.01 volumes.

---

### DoDM 5200.01 Volume 1 — Overview, Classification, and Declassification

```
name: DoD Manual 5200.01 Volume 1 — DoD Information Security Program: Overview, Classification, and Declassification
path: .claude/references/dod-5200/dodm-5200.01-vol1.pdf
version: February 24, 2012, incorporating Change 3 (effective January 17, 2025)
source_url: https://www.esd.whs.mil/Portals/54/Documents/DD/issuances/dodm/520001m_vol1.pdf
date_retrieved: 2026-03-12
sha256: d2db068ed16c7c610125ff9d2ca0bd8dbbc6a5497a4d69aa8b33887e4c52983c
relevance: umrs-selinux, cui-labels, deployment, AC-1, AC-3, SC-16, MP-1
status: downloaded
```

Implements policy for the designation, marking, protection, and dissemination of
classified and CUI. Change 3 (Jan 2025) removes OCA language in accordance with DoDM 5200.45.

---

### DoDM 5200.01 Volume 2 — Marking of Information

```
name: DoD Manual 5200.01 Volume 2 — DoD Information Security Program: Marking of Information
path: .claude/references/dod-5200/dodm-5200.01-vol2.pdf
version: February 24, 2012, incorporating Change 4 (effective July 28, 2020)
source_url: https://www.esd.whs.mil/portals/54/Documents/DD/issuances/dodm/520001m_vol2.pdf
date_retrieved: 2026-03-12
sha256: 05c4e7d5074704194259bc81f54bb9e78dbcd28f522617bea8ec7378cdb1bbc4
relevance: umrs-selinux, cui-labels, mcs-setrans, SC-16, AC-16
status: downloaded
```

Marking procedures for classified and CUI. Directly relevant to label rendering
and MCS human-readable category translation in `mcs-setrans`.

---

### DoDM 5200.01 Volume 3 — Protection of Classified Information

```
name: DoD Manual 5200.01 Volume 3 — DoD Information Security Program: Protection of Classified Information
path: .claude/references/dod-5200/dodm-5200.01-vol3.pdf
version: February 24, 2012, incorporating Change 4 (effective January 17, 2025)
source_url: https://www.esd.whs.mil/Portals/54/Documents/DD/issuances/dodm/520001m_vol3.pdf
date_retrieved: 2026-03-12
sha256: 816b5320f018e603a0d45134f0ad6863bc5d728de10598c94aa5fa6fad98f8dc
relevance: umrs-selinux, deployment, AC-3, AC-4, MP-4, SC-28
status: downloaded
```

Physical and procedural safeguards for classified information. Change 4 (Jan 2025)
aligns with updated OCA provisions. Relevant to storage, transmission, and access controls.

**Note**: Volume 3 also exists at the legacy URL `520001_p.PDF` — use `520001m_vol3.pdf`
(confirmed via search 2026-03-12).

---

### DoDI 5200.48 — Controlled Unclassified Information (CUI)

```
name: DoDI 5200.48 — Controlled Unclassified Information (CUI)
path: .claude/references/dod-5200/dodi-5200.48-cui.pdf
version: March 6, 2020
source_url: https://www.esd.whs.mil/Portals/54/Documents/DD/issuances/dodi/520048p.PDF
date_retrieved: 2026-03-12
sha256: bbbe4937419808636b21e060c7bc3912edb4bdb20dbe4737d26edb1b7c0a040d
relevance: umrs-selinux, cui-labels, deployment, CMMC, AC-3, AC-16, MP-4, SC-28
status: downloaded
```

DoD-wide CUI program policy: identification, marking, handling, dissemination,
and protection. Cancels DoDM 5200.01 Volume 4 (CUI marking). Requires NIST SP 800-171
compliance for contractors. Directly governs how UMRS must handle and label CUI.

**Note**: Volume 4 of DoDM 5200.01 was superseded by DoDI 5200.48 (March 2020).
DoDI 5200.48 is the current authoritative CUI policy. Volume 4 is not retrieved.

---

## Post-Quantum Cryptography Standards

### FIPS 203 — Module-Lattice-Based Key-Encapsulation Mechanism Standard (ML-KEM)

| Field | Value |
|---|---|
| Full title | Module-Lattice-Based Key-Encapsulation Mechanism Standard |
| Issuing authority | NIST |
| Version | FIPS 203 |
| Published | August 13, 2024 |
| Downloaded | 2026-03-13 |
| Local path | `nist/fips/fips203.pdf` |
| Source URL | https://nvlpubs.nist.gov/nistpubs/fips/nist.fips.203.pdf |
| SHA-256 | `fe1f12f32a7e44ec9fdebbf400cda843a40b506dee676725234dc6f7923b6cac` |
| Status | ✓ Downloaded |

**Key relevance**: First NIST post-quantum key encapsulation mechanism standard. Based on the
CRYSTALS-Kyber algorithm (Module Learning With Errors). Defines ML-KEM-512, ML-KEM-768, and
ML-KEM-1024 parameter sets. Critical for future-proofing key exchange in FIPS-validated
environments. Supersedes pre-quantum key agreement mechanisms for quantum-resistant deployments.
`SC-12`, `SC-13`.

---

### FIPS 204 — Module-Lattice-Based Digital Signature Standard (ML-DSA)

| Field | Value |
|---|---|
| Full title | Module-Lattice-Based Digital Signature Standard |
| Issuing authority | NIST |
| Version | FIPS 204 |
| Published | August 13, 2024 |
| Downloaded | 2026-03-13 |
| Local path | `nist/fips/fips204.pdf` |
| Source URL | https://nvlpubs.nist.gov/nistpubs/fips/nist.fips.204.pdf |
| SHA-256 | `57239b9f84c03227eda3ca0991204dc7764c79af9ce2e6824eda774918d46b6b` |
| Status | ✓ Downloaded |

**Key relevance**: Primary NIST post-quantum digital signature standard. Based on the
CRYSTALS-Dilithium algorithm (Module Learning With Errors). Defines ML-DSA-44, ML-DSA-65,
and ML-DSA-87 parameter sets. Intended as the general-purpose quantum-resistant replacement
for RSA and ECDSA signatures in FIPS-validated environments. `SC-12`, `SC-13`, `SI-7`.

---

### FIPS 205 — Stateless Hash-Based Digital Signature Standard (SLH-DSA)

| Field | Value |
|---|---|
| Full title | Stateless Hash-Based Digital Signature Standard |
| Issuing authority | NIST |
| Version | FIPS 205 |
| Published | August 13, 2024 |
| Downloaded | 2026-03-13 |
| Local path | `nist/fips/fips205.pdf` |
| Source URL | https://nvlpubs.nist.gov/nistpubs/fips/nist.fips.205.pdf |
| SHA-256 | `8ef34228276f3386d23cb0da8c14592b8cfb0db3358016bba64df7a004f8d13d` |
| Status | ✓ Downloaded |

**Key relevance**: Conservative post-quantum digital signature standard based on SPHINCS+
(hash-based cryptography). Security relies only on hash function properties — no lattice
assumptions. Provides a hedge against potential weaknesses in lattice-based schemes. Defines
twelve parameter sets across SHA-256 and SHAKE variants. Larger signatures but minimal
cryptographic assumptions. `SC-12`, `SC-13`, `SI-7`.

---

---

## RMF Core Documents (Security-Auditor Methodology Corpus — Phase 1)

Acquired 2026-03-15 per `.claude/plans/security-auditor-corpus.md` Phase 1.

### NIST SP 800-37 Rev. 2 — Risk Management Framework

| Field | Value |
|---|---|
| Full title | Risk Management Framework for Information Systems and Organizations |
| Issuing authority | NIST |
| Version | Rev 2 |
| Published | December 2018 |
| Downloaded | 2026-03-15 |
| Local path | `nist/sp800-37r2.pdf` |
| Source URL | https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-37r2.pdf |
| SHA-256 | `4f75e1136bb905a654f1bcd9eefefb4108f47597276ee5eec0688f6d96f6a01d` |
| Status | ✓ Downloaded |

**Key relevance**: Full RMF lifecycle (categorize, select, implement, assess, authorize,
monitor). Teaches the security-auditor agent the assessment workflow real assessors follow.

---

### NIST SP 800-53A Rev. 5 — Assessing Security and Privacy Controls

| Field | Value |
|---|---|
| Full title | Assessing Security and Privacy Controls in Information Systems and Organizations |
| Issuing authority | NIST |
| Version | Rev 5 |
| Published | January 2022 |
| Downloaded | 2026-03-15 |
| Local path | `nist/sp800-53Ar5.pdf` |
| Source URL | https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-53Ar5.pdf |
| SHA-256 | `75665570048b969ad465a4f4f1db425ce505c374951c2c64e462949c6b21be47` |
| Status | ✓ Downloaded |

**Key relevance**: Defines the three assessment methods (Examine, Interview, Test) and maps
each control to specific assessment procedures. The single most important document for making
the security-auditor agent behave like a real assessor.

---

### NIST SP 800-30 Rev. 1 — Guide for Conducting Risk Assessments

| Field | Value |
|---|---|
| Full title | Guide for Conducting Risk Assessments |
| Issuing authority | NIST |
| Version | Rev 1 |
| Published | September 2012 |
| Downloaded | 2026-03-15 |
| Local path | `nist/sp800-30r1.pdf` |
| Source URL | https://nvlpubs.nist.gov/nistpubs/Legacy/SP/nistspecialpublication800-30r1.pdf |
| SHA-256 | `f214087f0bdb35932a28c16eb93932a33c67a6edfb5e6f1638866bd201c98e04` |
| Status | ✓ Downloaded |

**Key relevance**: Risk assessment methodology — threat identification, likelihood
determination, impact analysis. Underpins the severity model in the assessment engine.

---

### NIST SP 800-39 — Managing Information Security Risk

| Field | Value |
|---|---|
| Full title | Managing Information Security Risk: Organization, Mission, and Information System View |
| Issuing authority | NIST |
| Version | Initial |
| Published | March 2011 |
| Downloaded | 2026-03-15 |
| Local path | `nist/sp800-39.pdf` |
| Source URL | https://nvlpubs.nist.gov/nistpubs/Legacy/SP/nistspecialpublication800-39.pdf |
| SHA-256 | `cf680760d171fc590396c1d98eef184af96efa971ef9880830cd2912ea1a54a8` |
| Status | ✓ Downloaded |

**Key relevance**: Enterprise risk governance — how risk decisions roll up from system
level to organizational level. Provides the framing for how UMRS assessment findings
feed into broader accreditation decisions.

---

---

## Accreditation Process Documents (Security-Auditor Methodology Corpus — Phase 2)

Acquired 2026-03-15 per `.claude/plans/security-auditor-corpus.md` Phase 2.
RAG collection: `accreditation-artifacts` at `.claude/references/accreditation-artifacts/`.

Downloaded 2026-03-15. All items acquired via curl from official sources.
RAG ingestion pending. DOCX templates converted to .txt with pandoc for ingestion.

**URL corrections from original SOURCE.md**: The `/assets/resources/documents/` and
`/assets/resources/training/` URL paths contained S3 website redirect objects (redirect to
path + trailing slash), not actual files. Correct paths are `/resources/documents/` and
`/resources/templates/`. The 200-B and 200-C training PDFs were removed in the FedRAMP Rev5
reorganization and are no longer available at fedramp.gov.

### NIST SP 800-18 Rev. 1 — Guide for Developing Security Plans

```
name: NIST SP 800-18 Rev. 1 — Guide for Developing Security Plans for Federal Information Systems
path: .claude/references/nist/sp800-18r1.pdf
version: Rev 1
source_url: https://nvlpubs.nist.gov/nistpubs/Legacy/SP/nistspecialpublication800-18r1.pdf
date_retrieved: 2026-03-15
sha256: 1635870f7cac9c0606cddbf42a7e4d0d8d01c5ab8709d3667821a4cdeaddd506
relevance: security-auditor, umrs-selinux, deployment, PL-2, CA-2, CA-7
status: downloaded
```

Published February 2006. Defines SSP structure, required system description elements, control
implementation summaries, and SSP approval process. This is the foundational document
for understanding what a complete accreditation package must contain. Required reading for
the security-auditor to evaluate whether UMRS documentation would satisfy an SSP review.

Also copied to: `.claude/references/accreditation-artifacts/sp800-18r1.pdf`

---

### FedRAMP CSP Authorization Playbook v4.2

```
name: FedRAMP CSP Authorization Playbook
path: .claude/references/fedramp/fedramp-csp-authorization-playbook.pdf
version: 4.2 (November 17, 2025)
source_url: https://www.fedramp.gov/resources/documents/CSP_Authorization_Playbook.pdf
date_retrieved: 2026-03-15
sha256: 6ae7066b90afdc6bc3a54836b6aa1ad59181cda4efef5bb63ed3cb2d0cb5b8e4
relevance: security-auditor, deployment, CA-2, CA-7, PL-2, RA-3
status: downloaded
```

Current authoritative FedRAMP authorization process document. Replaces the legacy "Security
Assessment Framework" document in the Rev5 transition. Covers the end-to-end authorization
workflow: readiness assessment, full authorization package, agency sponsor engagement,
continuous monitoring. V4.2 aligns with OMB Memo M-24-15 (FedRAMP modernization, July 2024).

Also copied to: `.claude/references/accreditation-artifacts/fedramp-csp-authorization-playbook.pdf`

**URL correction**: Original SOURCE.md had `/assets/resources/documents/CSP_Authorization_Playbook.pdf`
which is an S3 redirect stub. Correct URL is `/resources/documents/CSP_Authorization_Playbook.pdf`.

---

### FedRAMP Agency Authorization Playbook v4.1

```
name: FedRAMP Agency Authorization Playbook
path: .claude/references/fedramp/fedramp-agency-authorization-playbook.pdf
version: 4.1 (November 17, 2025)
source_url: https://www.fedramp.gov/resources/documents/Agency_Authorization_Playbook.pdf
date_retrieved: 2026-03-15
sha256: 96fa3abc505e7b7aa89fa03d5595678824caaf99b8289a3d7e73c66e8a6cb87d
relevance: security-auditor, deployment, CA-2, CA-7, PL-2
status: downloaded
```

Agency-side view of the FedRAMP authorization process. Covers the AO/ISSO role, review
procedures, authorization package evaluation, and ongoing monitoring expectations. Paired
with the CSP Playbook to give the security-auditor both sides of the accreditation process.

Also copied to: `.claude/references/accreditation-artifacts/fedramp-agency-authorization-playbook.pdf`

---

### FedRAMP SAP Training

```
name: FedRAMP Security Assessment Plan (SAP) Training
path: .claude/references/fedramp/fedramp-sap-training.pdf
version: —
source_url: https://www.fedramp.gov/assets/resources/training/200-B-FedRAMP-Training-Security-Assessment-Plan-SAP.pdf
date_retrieved: —
sha256: —
relevance: security-auditor, CA-2, CA-7, SA-11
status: not_available
```

**Removed from fedramp.gov**: The 200-B SAP Training PDF was a Rev4-era document. As of
2026-03-15, the URL returns an S3 website redirect stub (83 bytes, not a valid PDF). The file
no longer exists at fedramp.gov. The SAP Template DOCX (Rev 5) covers the same structural
ground and is available below.

---

### FedRAMP SAR Training

```
name: FedRAMP Security Assessment Report (SAR) Training
path: .claude/references/fedramp/fedramp-sar-training.pdf
version: —
source_url: https://www.fedramp.gov/assets/resources/training/200-C-FedRAMP-Training-Security-Assessment-Report-SAR.pdf
date_retrieved: —
sha256: —
relevance: security-auditor, CA-2, CA-7, SA-11
status: not_available
```

**Removed from fedramp.gov**: The 200-C SAR Training PDF was a Rev4-era document. As of
2026-03-15, the URL returns an S3 website redirect stub (85 bytes, not a valid PDF). The file
no longer exists at fedramp.gov. The SAR Template DOCX (Rev 5) covers the same structural
ground and is available below.

---

### FedRAMP SSP Template (Rev 5)

```
name: FedRAMP High/Moderate/Low/LI-SaaS Baseline System Security Plan Template
path: .claude/references/fedramp/fedramp-ssp-template.docx
version: Rev 5 (last modified 2025-08-07)
source_url: https://www.fedramp.gov/resources/templates/FedRAMP-High-Moderate-Low-LI-SaaS-Baseline-System-Security-Plan-(SSP).docx
date_retrieved: 2026-03-15
sha256: e05d7fb0021cf42f7fe15eed5c21362e99a0182ccb6f428b1015babc59226c48
relevance: security-auditor, PL-2, CA-2
status: downloaded
```

Official FedRAMP SSP template. DOCX only — no PDF version published. Converted to .txt with
pandoc for RAG ingestion: `.claude/references/accreditation-artifacts/fedramp-ssp-template.txt`
(SHA-256: `e631914c8afbfad5603220c42ae13038c3b7e42ff9a542e1f7a2cdbb9e02166e`, 151K).

Also copied to: `.claude/references/accreditation-artifacts/fedramp-ssp-template.docx`

---

### FedRAMP SAP Template (Rev 5)

```
name: FedRAMP Security Assessment Plan Template
path: .claude/references/fedramp/fedramp-sap-template.docx
version: Rev 5 (last modified 2025-08-07)
source_url: https://www.fedramp.gov/resources/templates/FedRAMP-Security-Assessment-Plan-(SAP)-Template.docx
date_retrieved: 2026-03-15
sha256: 4d2f79e0577cb52a34e2eff1aad0290585c6e0af1ac1f6468d6ef0cba7fc6aae
relevance: security-auditor, CA-2, SA-11
status: downloaded
```

Official FedRAMP SAP template. DOCX only. Converted to .txt with pandoc for RAG ingestion:
`.claude/references/accreditation-artifacts/fedramp-sap-template.txt`
(SHA-256: `004cbf245201cf229e236090716e5761969f77c43f3a964e663be6ff4f326fa4`, 91K).

Also copied to: `.claude/references/accreditation-artifacts/fedramp-sap-template.docx`

---

### FedRAMP SAR Template (Rev 5)

```
name: FedRAMP Security Assessment Report Template
path: .claude/references/fedramp/fedramp-sar-template.docx
version: Rev 5 (last modified 2025-08-07)
source_url: https://www.fedramp.gov/resources/templates/FedRAMP-Security-Assessment-Report-(SAR)-Template.docx
date_retrieved: 2026-03-15
sha256: a779218bd12a5c52f26f8a5edd8896fcd8515801975e9a5f7c09f8228864b9a7
relevance: security-auditor, CA-2, SA-11
status: downloaded
```

Official FedRAMP SAR template. DOCX only. Converted to .txt with pandoc for RAG ingestion:
`.claude/references/accreditation-artifacts/fedramp-sar-template.txt`
(SHA-256: `77f745835de5b2160bed708166d54d7da318794f6a46612dd60db1d3a80b1372`, 93K).

**URL correction**: Original SOURCE.md had `/assets/resources/templates/...` which is an S3
redirect stub. Correct URL is `/resources/templates/FedRAMP-Security-Assessment-Report-(SAR)-Template.docx`.

Also copied to: `.claude/references/accreditation-artifacts/fedramp-sar-template.docx`

---

---

## Technical Compliance Documents (Security-Auditor Methodology Corpus — Phase 3)

Acquisition status per `.claude/plans/security-auditor-corpus.md` Phase 3.
Initiated 2026-03-17.

### RHEL 10 STIG Playbook (SCAP Security Guide)

```
name: SCAP Security Guide — RHEL 10 STIG Ansible Playbook
path: .claude/references/scap-security-guide/rhel10-playbook-stig.yml
version: SCAP Security Guide (ComplianceAsCode project) — version at collection date
source_url: https://github.com/ComplianceAsCode/content (official RHEL SCAP content)
date_retrieved: (prior to 2026-03-17 — already present in collection)
sha256: (not computed — RAG augmentation source, no provenance chain required)
relevance: umrs-platform posture probe, CM-6, CM-7, AC-6, SC-39, SI-16, AU-12
status: ingested (451 STIG rules extracted to stig-signal-index.md)
```

**Note:** This is the ComplianceAsCode SCAP Security Guide content, not the official DISA
STIG. DISA has not published an official RHEL 10 STIG as of 2026-03-17. The SSG playbook
is the authoritative Red Hat-maintained STIG-aligned content until a DISA RHEL 10 STIG is
published. Monitor `public.cyber.mil` for the official STIG release.

---

### DISA RHEL 9 STIG v2r5

```
name: DISA Red Hat Enterprise Linux 9 Security Technical Implementation Guide
path: .claude/references/dod-5200/stig/U_RHEL_9_V2R5_STIG.zip
version: V2R5 (September 2025)
source_url: https://dl.dod.cyber.mil/wp-content/uploads/stigs/zip/U_RHEL_9_V2R5_STIG.zip
date_retrieved: —
sha256: —
relevance: security-auditor, umrs-platform posture probe, CM-6, CM-7, SC-39, SI-16
status: requires_manual_download
```

**Status: Awaiting download — needs Bash write permission to `.claude/references/dod-5200/stig/`.**
HTTP 200 confirmed from `dl.dod.cyber.mil` (2.1 MB ZIP). The file is publicly accessible
without authentication. Once Bash write permission is granted, run:

```bash
mkdir -p /media/psf/repos/umrs-project/.claude/references/dod-5200/stig
curl -L -o /media/psf/repos/umrs-project/.claude/references/dod-5200/stig/U_RHEL_9_V2R5_STIG.zip \
  https://dl.dod.cyber.mil/wp-content/uploads/stigs/zip/U_RHEL_9_V2R5_STIG.zip
sha256sum /media/psf/repos/umrs-project/.claude/references/dod-5200/stig/U_RHEL_9_V2R5_STIG.zip
```

Then update this entry with the computed SHA-256 and `status: downloaded`.

**Why RHEL 9 STIG is useful:** RHEL 9 STIG v2r5 is the closest official DISA STIG to the
RHEL 10 target. Many kernel-level rules (sysctl, module blacklisting) are identical between
RHEL 9 and RHEL 10. Until a DISA RHEL 10 STIG exists, the RHEL 9 STIG provides the
authoritative DISA rule IDs and Vulnerability IDs (VulnIDs) needed for formal evidence mapping.

---

### CMMC Assessment Guide Level 2 v2.13

Already tracked under the DoD section above. See entry `dod/cmmc-assessment-guide-l2.pdf`.
No duplicate entry needed.

---

### CIS Red Hat Enterprise Linux Benchmark

```
name: CIS Red Hat Enterprise Linux 9 Benchmark
path: .claude/references/dod-5200/stig/cis-rhel9-benchmark.pdf
version: v2.0.0 (June 2024)
source_url: https://www.cisecurity.org/benchmark/red_hat_linux
date_retrieved: —
sha256: —
relevance: umrs-platform posture probe, CM-6, CM-7, SC-39
status: requires_manual_download
```

**Status: Requires manual browser download.**
CIS Benchmarks are available free of charge but require account registration at
`cisecurity.org`. An unofficial mirror was located at `rayasec.com` but cannot be used
as it is not an approved source. Manual steps:

1. Create a free account at https://www.cisecurity.org/cis-benchmarks/
2. Download "CIS Red Hat Enterprise Linux 9 Benchmark" (PDF)
3. Save to `.claude/references/dod-5200/stig/cis-rhel9-benchmark.pdf`
4. Run `sha256sum` and record the checksum here

**Priority:** Medium. The SCAP Security Guide playbooks already provide the CIS
Level 1/2 content in machine-readable form (`rhel10-playbook-cis.yml`). The PDF
benchmark adds rationale text and Level 1 / Level 2 profile distinctions that
improve the security-auditor's reasoning depth.

---

### OSCAL v1.1.2 JSON Schemas (RAG Collection)

**Collection path:** `.claude/references/oscal-schemas/`
**Source:** NIST usnistgov/OSCAL GitHub release v1.1.2
**Date retrieved:** 2026-03-23

These are the official OSCAL JSON Schema files from NIST's OSCAL v1.1.2 release.
They define the machine-readable structure for OSCAL documents used in FedRAMP Rev5
automation and CMMC assessment tooling.

| File | SHA-256 |
|---|---|
| `oscal_assessment-plan_schema.json` | `43464ad048b711c735934b66015bcf8239782c6263d377a742c6b205ea796ecb` |
| `oscal_assessment-results_schema.json` | `d033da70154cf6625ae46a746199e88e58f2928b1387dfac051d381b92f41b0d` |
| `oscal_catalog_schema.json` | `5b069afa4f4ecc38d59914dab56098566d4247d3578a2123c030c80d36fc5104` |
| `oscal_complete_schema.json` | `f0b24aef59190cb2649b404976f1677b546e6bb1982597d8bccb9e6b1244e18c` |
| `oscal_component_schema.json` | `7b74710940ad39b6b63d4ddccbadf2c7d2e9bf11b07808d41d2aa27a4616e5ce` |
| `oscal_poam_schema.json` | `906725163d767036c6189aec51252109b203214e121fc1acaff494b4d2dfbc04` |
| `oscal_profile_schema.json` | `c910ea1a852e9d4ccfb7f6a8d0898b0cd4f137e48f88886412a083c8d87d540a` |
| `oscal_ssp_schema.json` | `08d3faeb12f0fab7705dec15fb648c72400c7ab6ac0056222d49d21507e02a69` |

**Relevance:** assessment-engine, CMMC CA-7, NIST RMF artifacts, FedRAMP SSP/SAP/SAR/POA&M structure

---

### FedRAMP OSCAL Rev5 Baseline Profiles (RAG Collection)

**Collection path:** `.claude/references/oscal-schemas/` (flat, alongside OSCAL schemas)
**Source:** GSA/fedramp-automation GitHub `master` branch — `dist/content/rev5/baselines/json/`
**Date retrieved:** 2026-03-23

These are the official FedRAMP Rev5 OSCAL baseline profiles, representing LOW, MODERATE, and
HIGH impact levels. The resolved-profile catalogs embed all 800-53 Rev5 control text.

| File | Size | SHA-256 |
|---|---|---|
| `fedramp_rev5_LOW-baseline_profile.json` | 795 KB | `b13459be3665380b559f10c500310668ce12c2ef71b3c8be997509659988265e` |
| `fedramp_rev5_LOW-baseline-resolved-profile_catalog.json` | 3.3 MB | `54517b456a19c937b4f8cecb58674de917c02af7deaf6be368cf8bbae3771071` |
| `fedramp_rev5_MODERATE-baseline_profile.json` | 1.2 MB | `5db32cd3c31c3450c03168edd3db26641324c15971edf52163d2310ccdaa925f` |
| `fedramp_rev5_MODERATE-baseline-resolved-profile_catalog.json` | 5.0 MB | `072d831452744341de1de1201e9494b0ed84e9f62b54b52ec8017369df4be780` |
| `fedramp_rev5_HIGH-baseline_profile.json` | 1.4 MB | `cce692728b57fae69b25860aba7116426a3637d5b01eb02cd23b13c4eff868f4` |
| `fedramp_rev5_HIGH-baseline-resolved-profile_catalog.json` | 5.9 MB | `0bc97487599ceeffe1adad218570b948157aa96987ff7308759caf8fc84236fb` |

**Note:** The resolved-profile catalogs are large JSON files (~3–6 MB each). They contain full
800-53 Rev5 control text with FedRAMP-specific parameters. RAG ingestion chunks these by
top-level control objects. The profile files contain only control selections and parameter
overrides, which are smaller and more targeted for automation use.

**Relevance:** FedRAMP authorization, CMMC alignment, assessment-engine, AC-2, AC-4, CA-7,
RA-2, SI-12; informs which controls are selected at each impact level

---

## Five Eyes Classification Policies

Acquired per `.claude/plans/research-pipeline-priorities.md` Priority 2 (Canada portion).
Collection path: `.claude/references/five-eyes-classification/`
Source record: `.claude/references/five-eyes-classification/SOURCE.md`
Familiarization-only — no RAG ingestion required (Henri and Simone use directly).
Initial download: 2026-03-24. SOURCE.md and manifest entries added: 2026-03-31.

---

### TBS Policy on Government Security (PGS) — English

```
name: TBS Policy on Government Security — English
path: .claude/references/five-eyes-classification/tbs-policy-gov-security-en.md
version: July 1, 2019 (last amended January 6, 2025)
source_url: https://www.tbs-sct.canada.ca/pol/doc-eng.aspx?id=16578
date_retrieved: 2026-03-24
sha256: ⚠ pending — run sha256sum on file and record here
relevance: umrs-labels, CANADIAN-PROTECTED.json, Henri, Simone, Five Eyes mapping, AC-16
status: downloaded
```

High-level governance policy. Defines security management obligations for GoC departments.
Covers: security screening (A.1), IT security (A.2), physical security (A.3), BCM (A.4),
information management security (A.5), contract security (A.6), security event management (A.7),
security awareness (A.8). Full definitions glossary in Appendix B.

**Note:** The PGS does NOT define Protected A/B/C tiers. Those are in Appendix J of the DOSM
(id=32614). The PGS is the enabling governance instrument; Appendix J is the classification standard.

---

### TBS Policy on Government Security (PGS) — French

```
name: TBS Politique sur la sécurité du gouvernement — French (fr-CA)
path: .claude/references/five-eyes-classification/tbs-policy-gov-security-fr.md
version: 1er juillet 2019 (modifiée le 6 janvier 2025)
source_url: https://www.tbs-sct.canada.ca/pol/doc-fra.aspx?id=16578
date_retrieved: 2026-03-24
sha256: ⚠ pending — run sha256sum on file and record here
relevance: umrs-labels, CANADIAN-PROTECTED.json, Simone (French terminology), Five Eyes mapping
status: downloaded
```

French (fr-CA) version of the PGS. Primary use: Simone's bilingual terminology reference for
GoC security management vocabulary. Covers identical content to the English version.
Canonical French title: "Politique sur la sécurité du gouvernement".

---

### TBS Directive on Security Management (DOSM) — English

```
name: TBS Directive on Security Management — English
path: .claude/references/five-eyes-classification/tbs-dosm-en.md
version: July 1, 2019
source_url: https://www.tbs-sct.canada.ca/pol/doc-eng.aspx?id=32611
date_retrieved: —
sha256: —
relevance: umrs-labels, CANADIAN-PROTECTED.json, Henri, Simone, AC-16
status: requires_download
```

**Status: Not yet downloaded.** The DOSM is the mandatory directive implementing the PGS.
It defines mandatory procedures for all security management domains. Parent document for
Appendix J (the classification standard). Fetch with:

```bash
bash -c 'curl -L -s --max-time 60 \
  -A "Mozilla/5.0 (X11; Linux x86_64; rv:120.0) Gecko/20100101 Firefox/120.0" \
  "https://www.tbs-sct.canada.ca/pol/doc-eng.aspx?id=32611&section=html" \
  | pandoc -f html -t markdown --wrap=none \
  > /DEVELOPMENT/umrs-project/.claude/references/five-eyes-classification/tbs-dosm-en.md'
```

---

### TBS Directive on Security Management (DOSM) — French

```
name: TBS Directive sur la gestion de la sécurité — French (fr-CA)
path: .claude/references/five-eyes-classification/tbs-dosm-fr.md
version: 1er juillet 2019
source_url: https://www.tbs-sct.canada.ca/pol/doc-fra.aspx?id=32611
date_retrieved: —
sha256: —
relevance: umrs-labels, CANADIAN-PROTECTED.json, Simone (French terminology)
status: requires_download
```

**Status: Not yet downloaded.** French (fr-CA) version of the DOSM.
Canonical French title: "Directive sur la gestion de la sécurité". Fetch with:

```bash
bash -c 'curl -L -s --max-time 60 \
  -A "Mozilla/5.0 (X11; Linux x86_64; rv:120.0) Gecko/20100101 Firefox/120.0" \
  -H "Accept-Language: fr-CA,fr;q=0.9" \
  "https://www.tbs-sct.canada.ca/pol/doc-fra.aspx?id=32611&section=html" \
  | pandoc -f html -t markdown --wrap=none \
  > /DEVELOPMENT/umrs-project/.claude/references/five-eyes-classification/tbs-dosm-fr.md'
```

---

### TBS DOSM Appendix J — Standard on Security Categorization — English

```
name: TBS DOSM Appendix J: Standard on Security Categorization — English
path: .claude/references/five-eyes-classification/tbs-dosm-appendix-j-security-categorization-en.md
version: July 1, 2019
source_url: https://www.tbs-sct.canada.ca/pol/doc-eng.aspx?id=32614
date_retrieved: —
sha256: —
relevance: umrs-labels, CANADIAN-PROTECTED.json, setrans.conf, Henri, Simone, AC-16, MP-3
status: requires_download — HIGHEST PRIORITY in this collection
```

**Status: Not yet downloaded. HIGHEST PRIORITY.** This is the document that defines
Protected A, Protected B, Protected C, and Classified (Confidential/Secret/Top Secret) tiers
by injury type. This is the authoritative source for `CANADIAN-PROTECTED.json` label design
and `setrans.conf` Canadian entries.

Key content (confirmed via search):
- Protected = injury outside the national interest (to individuals or organizations)
- Classified = injury to the national interest
- Protected A: limited injury to an individual or organization
- Protected B: serious injury to an individual or organization
- Protected C: extremely grave injury (comparable to Classified; rarely used)

Fetch with:
```bash
bash -c 'curl -L -s --max-time 60 \
  -A "Mozilla/5.0 (X11; Linux x86_64; rv:120.0) Gecko/20100101 Firefox/120.0" \
  "https://www.tbs-sct.canada.ca/pol/doc-eng.aspx?id=32614&section=html" \
  | pandoc -f html -t markdown --wrap=none \
  > /DEVELOPMENT/umrs-project/.claude/references/five-eyes-classification/tbs-dosm-appendix-j-security-categorization-en.md'
```

---

### TBS DOSM Appendix J — Norme sur la catégorisation de la sécurité — French

```
name: TBS DOSM Annexe J : Norme sur la catégorisation de la sécurité — French (fr-CA)
path: .claude/references/five-eyes-classification/tbs-dosm-appendix-j-security-categorization-fr.md
version: 1er juillet 2019
source_url: https://www.tbs-sct.canada.ca/pol/doc-fra.aspx?id=32614
date_retrieved: —
sha256: —
relevance: umrs-labels, CANADIAN-PROTECTED.json, setrans.conf, Simone (canonical French terms)
status: requires_download — HIGHEST PRIORITY in this collection
```

**Status: Not yet downloaded. HIGHEST PRIORITY.** French (fr-CA) version of Appendix J.
Canonical French title: "Norme sur la catégorisation de la sécurité".
Primary use: Simone's canonical source for "Protégé A", "Protégé B", "Protégé C" — the
Treasury Board-authorized French-language designations for GoC classification tiers.
These terms must appear in any fr-CA localized security label output from UMRS.

Fetch with:
```bash
bash -c 'curl -L -s --max-time 60 \
  -A "Mozilla/5.0 (X11; Linux x86_64; rv:120.0) Gecko/20100101 Firefox/120.0" \
  -H "Accept-Language: fr-CA,fr;q=0.9" \
  "https://www.tbs-sct.canada.ca/pol/doc-fra.aspx?id=32614&section=html" \
  | pandoc -f html -t markdown --wrap=none \
  > /DEVELOPMENT/umrs-project/.claude/references/five-eyes-classification/tbs-dosm-appendix-j-security-categorization-fr.md'
```

---

### UK Government Security Classification Policy (GSCP)

```
name: UK Government Security Classification Policy (GSCP)
path: .claude/references/five-eyes-classification/uk-gscp-june2023.pdf
version: June 2023
source_url: https://www.gov.uk/government/publications/government-security-classifications
date_retrieved: 2026-03-24
sha256: ⚠ pending — run sha256sum on file and record here
relevance: umrs-labels, Five Eyes mapping, AC-16
status: downloaded
```

Cabinet Office policy defining the UK classification scheme: OFFICIAL, SECRET, TOP SECRET.
Includes guidance on handling, markings, and system requirements. The OFFICIAL tier (with
SENSITIVE subset) maps roughly to US CUI / Canadian Protected B.

---

### New Zealand Information Security Manual (NZISM) v3.7

```
name: New Zealand Information Security Manual (NZISM)
path: .claude/references/five-eyes-classification/nzism-3.7.pdf
version: 3.7
source_url: https://www.nzism.gcsb.govt.nz/
date_retrieved: 2026-03-24
sha256: ⚠ pending — run sha256sum on file and record here
relevance: umrs-labels, Five Eyes mapping, AC-16
status: downloaded
```

GCSB (Government Communications Security Bureau) security manual. Covers NZ classification
scheme: UNCLASSIFIED, IN-CONFIDENCE, SENSITIVE, RESTRICTED, CONFIDENTIAL, SECRET, TOP SECRET.
Relevant for Five Eyes label equivalency mapping.

---

## Pending Documents

The following documents are referenced in CLAUDE.md but not yet downloaded.
Add them here as they are acquired.

| Document | Authority | Notes |
|---|---|---|
| NSA RTB VNSSA | NSA | Raise the Bar — Vendor Network Security Spec |
| NSA RAIN | NSA | May be distribution-restricted; verify availability |
