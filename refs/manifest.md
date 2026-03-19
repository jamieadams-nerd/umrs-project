# Reference Document Manifest

This directory contains third-party standards and guidance documents that inform the
security architecture and compliance posture of the UMRS project.

## Integrity Verification

Before using any document, verify its SHA-256 checksum:

```bash
sha256sum refs/nist/*.pdf refs/dod/*.pdf
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

## Research Reports

Researcher-authored synthesis reports live at `refs/reports/`. Each report
combines findings from the RAG collections, codebase analysis, and official
reference documents. Reports are produced on request; the directory is the
permanent, stable location — always look here first.

### Kernel Module Loading Controls & SELinux Context Change Security

| Field | Value |
|---|---|
| File | `refs/reports/kernel-selinux-module-context-security.md` |
| Date | 2026-03-10 |
| SHA-256 | `15aa1f21d577eac0dd4eca84081f02c73cc6a8feee07784ddeb7e4dfddde453e` |
| Topics | `modules_disabled` sysctl, `MODULE_SIG_FORCE`, Lockdown LSM, LoadPin LSM, IPE LSM, SELinux `system:module_request`, `process:dyntransition`/`setcurrent`, MLS `mlsvalidatetrans`, Bell-LaPadula tranquility |
| Relevance | `umrs-platform` (`ModuleLoadLatch`, `ProcFips`), `umrs-selinux` (`MlsRange`, `MlsLevel::dominates`), `umrs-logspace`, `AC-3`, `AC-4`, `CM-6`, `CM-7`, `SC-13`, `SC-28`, `SI-4`, `SI-7`, `AU-12` |

---

### STIG Signal Coverage — Phase 3 Posture Probe Cross-Reference

| Field | Value |
|---|---|
| File | `refs/reports/stig-signal-coverage.md` |
| Date | 2026-03-17 |
| Topics | UMRS posture indicator → STIG rule mapping, coverage gaps, CMMC alignment, severity cross-reference, candidate new indicators |
| Relevance | `umrs-platform` (posture catalog), `umrs-selinux`, CMMC, `CM-6`, `CM-7`, `SC-7`, `SC-13`, `AU-5`, `AU-12`, `SI-16` |

---

### UMRS Capabilities — NIST SP 800-171 Rev 3 and CMMC Level 2 Mandate Mapping

| Field | Value |
|---|---|
| File | `refs/reports/umrs-capabilities-800-171r3-mapping.md` |
| Date | 2026-03-19 |
| SHA-256 | `2868468e48e6a49350389014c5e834c2ca0e3fc373084927bfd491983da2165a` |
| Topics | 800-171r3 mandate mapping, CMMC L2 practice mapping, CUI labeling, handling restrictions, SELinux posture, typed findings, security engineering, memory safety, orthogonal category composition |
| Relevance | `umrs-selinux`, `umrs-platform`, `cui-labels`, blog post compliance claims, `AC-3`, `AC-4`, `AU-3`, `CM-6`, `CA-7`, `MP-3`, `PL-4`, `SA-8`, `SI-2`, SSDF PW.4 |

---

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

#### NIST SP 800-53 Rev 5 (copy from refs/nist/)

| Field | Value |
|---|---|
| Full title | Security and Privacy Controls for Information Systems and Organizations |
| Issuing authority | NIST |
| Version | Rev 5 |
| Downloaded | 2026-03-01 (original); copied to collection 2026-03-11 |
| Local path | `.claude/references/access-control/standards/nist-sp-800-53r5.pdf` |
| Source URL | https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-53r5.pdf |
| SHA-256 | `fc63bcd61715d0181dd8e85998b1e6201ae3515fc6626102101cab1841e11ec6` |
| Status | ✓ Downloaded (copied from refs/nist/sp800-53r5.pdf) |

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
path: refs/dod/dodi-5200.01.pdf
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
path: refs/dod/dodm-5200.01-vol1.pdf
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
path: refs/dod/dodm-5200.01-vol2.pdf
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
path: refs/dod/dodm-5200.01-vol3.pdf
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
path: refs/dod/dodi-5200.48-cui.pdf
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
path: refs/nist/sp800-18r1.pdf
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
path: refs/fedramp/fedramp-csp-authorization-playbook.pdf
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
path: refs/fedramp/fedramp-agency-authorization-playbook.pdf
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
path: refs/fedramp/fedramp-sap-training.pdf
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
path: refs/fedramp/fedramp-sar-training.pdf
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
path: refs/fedramp/fedramp-ssp-template.docx
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
path: refs/fedramp/fedramp-sap-template.docx
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
path: refs/fedramp/fedramp-sar-template.docx
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
path: refs/dod/stig/U_RHEL_9_V2R5_STIG.zip
version: V2R5 (September 2025)
source_url: https://dl.dod.cyber.mil/wp-content/uploads/stigs/zip/U_RHEL_9_V2R5_STIG.zip
date_retrieved: —
sha256: —
relevance: security-auditor, umrs-platform posture probe, CM-6, CM-7, SC-39, SI-16
status: requires_manual_download
```

**Status: Awaiting download — needs Bash write permission to `refs/dod/stig/`.**
HTTP 200 confirmed from `dl.dod.cyber.mil` (2.1 MB ZIP). The file is publicly accessible
without authentication. Once Bash write permission is granted, run:

```bash
mkdir -p /media/psf/repos/umrs-project/refs/dod/stig
curl -L -o /media/psf/repos/umrs-project/refs/dod/stig/U_RHEL_9_V2R5_STIG.zip \
  https://dl.dod.cyber.mil/wp-content/uploads/stigs/zip/U_RHEL_9_V2R5_STIG.zip
sha256sum /media/psf/repos/umrs-project/refs/dod/stig/U_RHEL_9_V2R5_STIG.zip
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
path: refs/dod/stig/cis-rhel9-benchmark.pdf
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
3. Save to `refs/dod/stig/cis-rhel9-benchmark.pdf`
4. Run `sha256sum` and record the checksum here

**Priority:** Medium. The SCAP Security Guide playbooks already provide the CIS
Level 1/2 content in machine-readable form (`rhel10-playbook-cis.yml`). The PDF
benchmark adds rationale text and Level 1 / Level 2 profile distinctions that
improve the security-auditor's reasoning depth.

---

## Pending Documents

The following documents are referenced in CLAUDE.md but not yet downloaded.
Add them here as they are acquired.

| Document | Authority | Notes |
|---|---|---|
| NSA RTB VNSSA | NSA | Raise the Bar — Vendor Network Security Spec |
| NSA RAIN | NSA | May be distribution-restricted; verify availability |
