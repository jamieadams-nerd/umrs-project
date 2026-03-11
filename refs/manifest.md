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
| Issuing authority | Department of Defense |
| Published | Federal Register, December 26, 2023 (Vol. 88, No. 246) |
| Downloaded | — |
| Local path | `dod/cmmc-32cfr170-final-rule.pdf` |
| Source URL | https://www.govinfo.gov/content/pkg/FR-2023-12-26/pdf/2023-27756.pdf |
| SHA-256 | — |
| Status | ⚠ Requires manual download — govinfo.gov blocks automated requests |

**Manual download instructions**: Open the source URL in a browser and save the PDF to
`refs/dod/cmmc-32cfr170-final-rule.pdf`, then run `sha256sum` and record the value here.

**Key relevance**: Legal authority for CMMC Level 2 requirements; defines the regulatory
basis for CUI protection practices in this codebase.

---

### CMMC Assessment Guide — Level 2

| Field | Value |
|---|---|
| Full title | CMMC Assessment Guide Level 2 v2.0 |
| Issuing authority | Department of Defense (DoD CIO) |
| Version | 2.0 |
| Downloaded | — |
| Local path | `dod/cmmc-assessment-guide-l2.pdf` |
| Source URL | https://dodcio.defense.gov/Portals/0/Documents/CMMC/AssessmentGuide_L2.pdf |
| SHA-256 | — |
| Status | ⚠ Requires manual download — dodcio.defense.gov (Akamai) blocks automated requests |

**Manual download instructions**: Open the source URL in a browser and save the PDF to
`refs/dod/cmmc-assessment-guide-l2.pdf`, then run `sha256sum` and record the value here.

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

## Pending Documents

The following documents are referenced in CLAUDE.md but not yet downloaded.
Add them here as they are acquired.

| Document | Authority | Notes |
|---|---|---|
| NSA RTB VNSSA | NSA | Raise the Bar — Vendor Network Security Spec |
| NSA RAIN | NSA | May be distribution-restricted; verify availability |
