# Priority 4 NIST Familiarization Report

**Date:** 2026-03-31
**Author:** The Librarian
**Pipeline reference:** `.claude/plans/research-pipeline-priorities.md`, Priority 4
**ROADMAP:** G3 (CUI & Five Eyes), G4 (Assessment Engine)
**Documents reviewed:**
- NIST SP 800-172 — Enhanced Security Requirements for Protecting CUI
- NIST SP 800-161 Rev 1 — Cybersecurity Supply Chain Risk Management
- NIST SP 800-60 Vol 1 Rev 1 — Guide for Mapping Information Types to Security Categories

---

## Executive Recommendations

| Document | RAG Recommendation | Rationale |
|---|---|---|
| SP 800-172 | **Familiarize only** | 84-page supplement; principles-based; Librarian owns it; requirements are enumerable and can be distilled into agent memory |
| SP 800-161r1 | **Do not ingest — acquire upd1 first** | WITHDRAWN November 2024; superseded by SP 800-161r1-upd1; existing copy is reference-only pending replacement |
| SP 800-60 Vol 1 Rev 1 | **Do not ingest — acquire Vol 2 first** | Vol I is methodology only; the mapping catalog (Vol II) is not downloaded; without it Vol I provides insufficient retrieval value |

**Critical actions required before closing Priority 4:**

1. Download SP 800-161r1-upd1 from `https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-161r1-upd1.pdf` and replace the withdrawn document. Update the manifest entry.
2. Download SP 800-60 Vol 2 Rev 1 from `https://nvlpubs.nist.gov/nistpubs/Legacy/SP/nistspecialpublication800-60v2r1.pdf`. Add a manifest entry. Only after both volumes are present does ingestion make sense.
3. After acquiring the two missing/updated documents, evaluate ingestion again using the decision framework below.

---

## Document 1: NIST SP 800-172

### Scope

SP 800-172 is a 84-page supplement to SP 800-171. It defines 35 enhanced security requirements for protecting CUI in nonfederal systems and organizations against Advanced Persistent Threat (APT) actors. It does not stand alone — every requirement assumes SP 800-171 Rev 2 (the 110-requirement baseline) is already implemented. SP 800-172 is the reference document for CMMC Level 3.

**Publication:** February 2021. No revision or update has been issued. The document is current as of this report.

### Structure

The document is compact and tightly organized:

- **Chapter 1:** Purpose, scope, and relationship to SP 800-171. Establishes that these requirements apply only to CUI designated as "critical programs and high value assets" facing APT-level threats.
- **Chapter 2:** APT threat background and why enhanced requirements are needed.
- **Chapter 3:** The 35 enhanced requirements, organized into 14 control families using the same family codes as SP 800-171 (AC, AT, AU, CM, IA, IR, MA, MP, PS, RA, CA, SC, SI, SR implied). Requirements use dot-notation identifiers (e.g., AC.3.022, SC.3.177).
- **Appendix A:** References.
- **Appendix B:** Glossary.
- **Appendix C:** Acronyms.

### UMRS Relevance

SP 800-172 matters to UMRS in two ways.

**Direct relevance — cryptographic controls.** SC.3.177 requires FIPS-validated cryptography for all CUI. This is already a UMRS axiom (FIPS 140-2/140-3 gating in `umrs-selinux` and `umrs-hw`), but 800-172 gives the enhanced-protection citation for that requirement in high-value CUI contexts. The SC family requirements also mandate network segmentation and physical separation of CUI processing environments — directly applicable to the MLS/MCS architecture.

**Indirect relevance — roadmap awareness.** UMRS currently targets CMMC Level 2 (the 110 requirements from 800-171). Level 3 is built on 800-172. If the project ever moves toward Level 3, this document becomes the primary reference. Having it now means Knox and Herb can reason about the delta without a separate acquisition sprint.

**Crates affected:** `umrs-selinux` (information flow, MCS label enforcement), `umrs-hw` (FIPS entropy, SC-13), `umrs-core` (audit log integrity).

### Key Controls and Requirements

| Requirement ID | Family | Topic | UMRS relevance |
|---|---|---|---|
| AC.3.022 | Access Control | Dynamic access control based on security attributes | MCS label enforcement in SELinux targeted/MLS mode |
| IA.3.083 | Identification and Authentication | Multi-factor authentication for all non-privileged CUI access | Deployment gating |
| SC.3.177 | System and Comms Protection | FIPS-validated crypto for all CUI | `umrs-hw`, FIPS mode gate |
| SC.3.179 | System and Comms Protection | Prevent unauthorized exfiltration of CUI | MCS category enforcement at network boundary |
| SC.3.187 | System and Comms Protection | Physical or logical separation of CUI processing | SELinux domain separation, MLS enforcement |
| SC.3.192 | System and Comms Protection | Implement domain isolation via virtualization | Namespace/container boundary |
| SI.3.218 | System and Information Integrity | Multi-point malware protection (not just endpoint) | IMA/EVM posture checks |
| SI.3.219 | System and Information Integrity | Threat hunting on CUI systems | Posture monitoring scope |
| CA.3.161 | Security Assessment | Conduct ongoing security assessments | Continuous assessment engine (G4) |

### Most Important Sections for UMRS

1. **Chapter 3, SC family (pages 40–50):** The cryptography and network separation requirements are directly actionable. SC.3.177 is already implemented in principle; SC.3.187 and SC.3.192 are architectural targets for Phase 2.
2. **Chapter 3, SI family (pages 51–58):** Multi-point malware protection and integrity checking align with the IMA/EVM posture indicators already in the STIG signal coverage work.
3. **Chapter 1 (pages 1–10):** The threat framing is important context — it explains why these requirements exist and at what point an organization would be required to implement them.

### RAG Recommendation: Familiarize Only

SP 800-172 is 84 pages. It contains 35 enumerable requirements organized into familiar families. It is not a document agents will search for specific passages — it is a document they will apply as principles ("what does Level 3 require in the SC family?"). The Librarian owns this document. The distilled requirement list (see Key Controls table above) captures the actionable content. No RAG ingestion is needed.

**Action:** Record distilled requirement list in Librarian memory. Update `umrs-capabilities-800-171r3-mapping.md` report to note which of UMRS's existing capabilities address 800-172 SC requirements.

---

## Document 2: NIST SP 800-161 Rev 1

### CRITICAL: Document Is Withdrawn

NIST SP 800-161 Rev 1 (May 2022) was formally withdrawn on November 1, 2024. It has been superseded by SP 800-161r1-upd1 (Update 1). The withdrawn document remains on the NIST server for reference but is no longer the authoritative version. Any compliance work or control citations must reference the updated version.

**Action required:** Acquire SP 800-161r1-upd1 before treating this document as a design reference. The manifest entry for `nist/sp800-161r1.pdf` should be annotated as withdrawn pending replacement.

### Scope (from withdrawn Rev 1)

SP 800-161 Rev 1 is the primary NIST guidance for Cybersecurity Supply Chain Risk Management (C-SCRM). At 326 pages it is the most comprehensive of the three Priority 4 documents. It defines a three-tiered C-SCRM framework applicable to all federal agencies and nonfederal organizations handling federal information.

**The three tiers:**
- **Level 1 — Organization:** Enterprise C-SCRM policy, governance, and strategy
- **Level 2 — Mission/Business Process:** Program and acquisition-level supply chain risk decisions
- **Level 3 — System:** System-level component vetting, procurement security, and operational monitoring

### Structure

- **Chapters 1–2:** Background, scope, definitions, relationship to NIST CSF and RMF
- **Chapter 3:** C-SCRM roles and responsibilities; organizational governance
- **Chapter 4 (core):** Three-tiered C-SCRM practices — supplier risk assessment, acquisition strategies, contractual controls, ongoing monitoring
- **Appendix A:** C-SCRM Security Controls — an enhanced overlay of SP 800-53 Rev 5 organized into all 20 control families, each with supplemental C-SCRM guidance and applicability levels (1, 2, 3)
- **Appendices B–F:** Templates, case studies, C-SCRM maturity indicators, EO 14028 implementation, acronyms

### UMRS Relevance

**Direct relevance — dependency scrutiny model.** UMRS's scrutiny of Rust crate dependencies is a form of C-SCRM at Level 3. The document's guidance on supplier vetting, SBOM concepts (referenced via NTIA SBOM minimum elements), and component integrity verification maps directly to the crate vetting practices the team applies. The SR (Supply Chain Risk Management) family controls in Appendix A are the formal citation set for this work.

**CMMC alignment.** CMMC Level 2 includes SR domain requirements (SR.L2-3.14.1 through SR.L2-3.14.7) that derive from this publication. Understanding C-SCRM at the source document level strengthens the UMRS argument for its dependency controls.

**Key Appendix A controls seen:**
- **AC-3, AC-4:** Access enforcement and information flow enforcement — with C-SCRM-specific guidance about supply chain metadata flows and controlled release of acquisition-sensitive data
- **AC-5:** Separation of duties applied to preventing developers from promoting their own code to production environments
- **AC-6 (5):** Least privilege for non-organizational users (suppliers, contractors, system integrators)
- **SR-13 (new):** Supplier Inventory — a C-SCRM-specific control requiring inventory of all suppliers, not just components
- **MA-8 (new):** Maintenance Monitoring and Information Sharing — supply chain specific

**Crates affected:** Cargo workspace practices (not a crate per se); the dependency vetting checklist; any future `umrs-sbom` or provenance verification component.

### Most Important Sections for UMRS

1. **Appendix A, SR family:** The Supply Chain Risk Management control family additions are the primary reference for UMRS's crate dependency justifications.
2. **Chapter 4, Level 3 practices:** System-level supplier vetting and acquisition controls — the level at which UMRS operates.
3. **Chapter 4, Section 4.2 (C-SCRM Respond):** Controls selection and tailoring guidance; explains how to derive a C-SCRM baseline from the 800-53 Rev 5 catalog.

### RAG Recommendation: Do Not Ingest — Acquire upd1 First

The withdrawn status is disqualifying for ingestion. Ingesting a withdrawn document risks having agents cite superseded requirements or withdrawn control guidance. The correct path is:

1. Download SP 800-161r1-upd1.
2. Read the update's change summary to identify what changed between Rev 1 and upd1.
3. Decide ingestion based on upd1: at 326+ pages with a 20-control-family appendix, upd1 is a strong candidate for RAG ingestion — it is exactly the kind of large, multi-agent, passage-searchable reference the framework calls for. Knox and the rust-developer will want to look up specific control families.

**Interim:** The withdrawn Rev 1 copy remains available for historical reference but must not be cited as current guidance.

---

## Document 3: NIST SP 800-60 Vol 1 Rev 1

### CRITICAL: Incomplete Reference — Volume 2 Not Downloaded

SP 800-60 is a two-volume publication. Volume I (downloaded) provides the methodology for mapping information types to FIPS 199 security categories. Volume II (not downloaded) contains the actual information type catalog — the tables mapping specific government information types (e.g., "Law Enforcement Operations," "Procurement and Acquisition") to Confidentiality, Integrity, and Availability impact levels (HIGH/MODERATE/LOW).

Without Volume II, this reference cannot be used for its primary purpose: looking up what security category applies to a given type of government information. Volume I alone tells you how the mapping works but not what the mappings are.

**Action required:** Download SP 800-60 Vol 2 Rev 1 before treating this reference as usable for information type categorization.

### Scope

SP 800-60 Vol 1 Rev 1 (53 pages, August 2008) defines the methodology for assigning FIPS 199 security categories to federal information and information systems. FIPS 199 requires agencies to categorize all information they handle as LOW, MODERATE, or HIGH for each of three security objectives: Confidentiality, Integrity, Availability.

The document establishes that security categories are expressed as:
`{(confidentiality, impact), (integrity, impact), (availability, impact)}`

The overall system security category is the "high-water mark" — the highest impact value across all three objectives for all information types the system processes.

### Structure

- **Chapter 1:** Purpose and scope; relationship to FIPS 199, FIPS 200, and the RMF
- **Chapter 2:** FIPS 199 security categorization background — how security objectives and impact levels are defined
- **Chapter 3:** Step-by-step categorization process
  - Step 1: Identify information types
  - Step 2: Select provisional impact levels from Volume II
  - Step 3: Review provisional impact levels against threat/vulnerability information
  - Step 4: Assign system security category
- **Appendix A:** Government mission-based information types (Legislative, Judicial, Executive branch functions — the organizing taxonomy)
- **Appendix B:** Supporting delivery information types (cross-cutting service types)

### UMRS Relevance

**Direct relevance — MLS/MCS label design.** The FIPS 199 categorization framework is the formal basis for assigning security categories to government information. When UMRS maps CUI categories (Protected A/B/C in Canadian context; CUI Basic/Specified in US context) to MCS categories and MLS sensitivity levels, the mapping should be defensible in terms of FIPS 199 impact levels. SP 800-60 provides the theoretical grounding for those design decisions.

**Specific connection:** The `umrs-labels` CUI catalog assigns sensitivity levels (`s0`–`s3`) to information types. The assignment of those levels should correspond to FIPS 199 Confidentiality impact assessments. Volume II of SP 800-60 is the source of those provisional impact levels for US federal information types.

**Crates affected:** `umrs-labels` (MCS category design, CUI catalog), `umrs-selinux` (MLS sensitivity level enforcement when Phase 2 MLS work begins).

### Key Concepts

| Concept | Definition | UMRS relevance |
|---|---|---|
| Security category | `{(C, impact), (I, impact), (A, impact)}` — three-dimensional impact rating | Basis for MLS sensitivity level assignment |
| High-water mark | System category = highest impact across all information types processed | Explains why Mixed-mode systems require the highest label present |
| Provisional impact level | Default mapping from Volume II; subject to adjustment by mission context | Starting point for CUI catalog sensitivity assignments |
| Information type | Distinct category of information defined by its mission context | Maps to CUI categories in UMRS catalog |

### Most Important Sections for UMRS

1. **Chapter 3 (pages 14–27):** The four-step categorization process is the actionable methodology. Step 2 (select provisional impact levels) is where Volume II becomes essential.
2. **Appendix A (pages 31–43):** The government mission-based information type taxonomy is the organizing framework for the Volume II mapping tables. Understanding this taxonomy is needed to navigate Volume II once it is acquired.
3. **Chapter 2 (pages 7–13):** The FIPS 199 definitions — especially the three security objectives and their impact level definitions — are foundational for understanding why certain CUI categories warrant higher sensitivity levels.

### RAG Recommendation: Do Not Ingest — Acquire Vol 2 First

Volume I alone is insufficiently useful as a RAG source. Agents searching for "what is the security category for law enforcement investigation data" will find only the methodology in Volume I, not the answer. The answer is in Volume II. Ingesting Volume I without Volume II creates a misleading reference — the RAG would return methodology pages when agents need impact tables.

Once Volume II is acquired:
- Both volumes together are a strong RAG candidate: large information type catalog, passage-searchable, multiple agents will reference it (Librarian, Knox, future assessment engine work)
- Recommended collection name: `nist-sp800-60` (both volumes together)

**Interim:** Volume I is valuable for understanding the categorization methodology and grounding UMRS label design decisions. The Librarian should familiarize with it as a principles document (already done via this report). No ingestion until Vol 2 is present.

---

## Manifest Status

All three documents were added to `.claude/references/refs-manifest.md` prior to this familiarization pass (downloaded 2026-03-23). Checksums verified:

| Document | Local path | SHA-256 |
|---|---|---|
| SP 800-172 | `nist/sp800-172.pdf` | `5d8a04c46d6972505ddd52da6e84725161358fb334751e9a12d94fe0b64c8302` |
| SP 800-161r1 | `nist/sp800-161r1.pdf` | `c8250b1ea542640a7cffc145be08c6b17ff76bd695f8c0ac1b40c49d45355d89` |
| SP 800-60v1r1 | `nist/sp800-60v1r1.pdf` | `6f13f57f11697efcba694d0c9935c99168512f737cc76f2c432f248550128460` |

The manifest entry for SP 800-161r1 should be annotated as withdrawn (status: `⚠ Withdrawn — superseded by SP 800-161r1-upd1`). This is a manifest update, not a file deletion.

---

## Pending Acquisitions

| Document | Source URL | Priority | Reason |
|---|---|---|---|
| SP 800-161r1-upd1 | `https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-161r1-upd1.pdf` | High | Current version of the C-SCRM guide; withdrawn copy must not be cited |
| SP 800-60 Vol 2 Rev 1 | `https://nvlpubs.nist.gov/nistpubs/Legacy/SP/nistspecialpublication800-60v2r1.pdf` | Medium | Information type mapping catalog; Vol 1 is incomplete without it |

---

## Summary for Research Pipeline

Priority 4 is functionally complete in terms of acquisition — all three documents are downloaded and checksummed. However, two documents have quality issues that block their operational use:

- SP 800-161r1 is withdrawn and must be replaced with upd1 before citations or ingestion
- SP 800-60 Vol 1 is half a reference and must be paired with Vol 2 before ingestion

SP 800-172 is ready for active use now. The distilled requirement table in this report covers the UMRS-relevant content. No further work is needed on 800-172 before Knox and the team can use it as a design reference.

**Recommended pipeline status:** Priority 4 transitions from "in progress" to "partial" until the two pending acquisitions are completed. Full closure requires: SP 800-161r1-upd1 downloaded + manifest updated, SP 800-60 Vol 2 downloaded + manifest entry added, and a follow-up ingestion decision made for both.
