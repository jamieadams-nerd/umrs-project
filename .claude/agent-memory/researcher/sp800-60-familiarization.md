---
name: NIST SP 800-60 Rev 1 Familiarization
description: Actionable distillation of SP 800-60 Volumes 1 and 2 for UMRS CUI/MCS work — impact level mappings, key information type families, CIA baselines, and 800-53 connection
type: reference
---

# NIST SP 800-60 Rev 1 — Familiarization Summary

**Volumes**: Vol 1 (framework) + Vol 2 (appendices/catalog), both Rev 1, August 2008.
**Status**: FISMA-mandatory guidance; still current (no superseding document as of 2026-03).
**Consumer note**: familiarize-only — NOT in RAG. Use Read tool on the PDFs for specific lookups.

---

## 1. How FIPS 199 Impact Levels Map to Information Types

**Core mechanism**: 800-60 provides *provisional* security categories in `{C, I, A}` form where
each dimension is Low, Moderate, High, or Not Applicable (N/A). "Provisional" means starting
point — organizations must review and adjust based on mission context.

**FIPS 199 format**: `SC = {(confidentiality, x), (integrity, y), (availability, z)}`
- Low: limited adverse effect
- Moderate: serious adverse effect
- High: severe or catastrophic adverse effect
- N/A: valid for confidentiality only (public information where disclosure causes no harm)

**High-water mark rule**: the system-level SC is the *maximum* across all information types
processed by the system. One High information type makes the system High in that dimension.

**Provisional escalation factors** (critical for UMRS label validation):
- Law enforcement data: provisional baselines often understate confidentiality when informant
  identity, ongoing investigations, or violent crime are involved — escalation to High C is common
- National security / intelligence: ENTIRELY outside FISMA/800-60 scope; governed by
  separate DoD/IC authorities (EO 12333, etc.)
- Privacy Act data: escalation triggers when breach causes harm beyond embarrassment (financial
  loss, physical safety risk)

---

## 2. Information Type Families Most Relevant to UMRS CUI Work

**In scope for UMRS** (federal civilian systems under FISMA):

| Family | 800-60 Section | CUI Connection |
|---|---|---|
| Law Enforcement | D.16 (8 sub-types) | LEI, INV, SP-CTI categories |
| Litigation / Legal | D.17 (5 sub-types) | PRVCY, Legal Privilege |
| Privacy (general) | C.2.8.x, D.14, D.15 | PRVCY, HLTH, SSI |
| Financial Management | C.3.2.x (7 sub-types) | FNC, TAX, PROCURE |
| Intelligence Operations | D.3 | SP-CTI (domestic/unclassified) |
| Homeland Security | D.2 (4 sub-types) | CEII, Critical Infrastructure |
| Health | D.14 | HLTH, PRVCY |
| Information Security | C.3.5.5 | All CUI system-level baseline |

**Out of scope for UMRS** (do not use 800-60 baselines):
- Defense / military operations (D.1): national security — different governance
- Classified intelligence (D.3 classified portions): IC governance, not FISMA
- Nuclear weapons systems: non-FISMA authority

---

## 3. CIA Baselines for Key Information Type Families

### D.16 Law Enforcement (all 8 sub-types)

| Sub-type | Abbrev | C | I | A | Notes |
|---|---|---|---|---|---|
| D.16.1 Criminal Apprehension | (LEI) | Low | Low | Mod | Escalate C to H: violent/organized crime, informant exposure |
| D.16.2 Criminal Investigation & Surveillance | INV | Mod | Mod | Mod | Core INV baseline |
| D.16.3 Citizen Protection | (LEI) | Mod | Mod | Mod | |
| D.16.4 Leadership Protection | (LEI) | Mod | Low | Low | |
| D.16.5 Property Protection | (LEI) | Low | Low | Low | |
| D.16.6 Substance Control | (LEI) | Mod | Mod | Mod | |
| D.16.7 Crime Prevention | (LEI) | Low | Low | Low | |
| D.16.8 Trade Law Enforcement | (LEI/EXPT) | Mod | Mod | Mod | |

**UMRS impact**: MCS category c0 (CUI umbrella) and LEI/INV range categories should map to
at minimum Moderate/Moderate/Moderate for the investigation-class sub-types.

### D.17 Litigation

| Sub-type | C | I | A |
|---|---|---|---|
| D.17.1 Judicial Hearings | Mod | Low | Low |
| D.17.2 Legal Defense | Mod | **High** | Low |
| D.17.3 Legal Investigation | Mod | Mod | Mod |
| D.17.4 Legal Prosecution/Litigation | Low | Mod | Low |
| D.17.5 Resolution Facilitation | Mod | Low | Low |

**Flag**: Legal Defense has High Integrity — integrity failures (tampering) could affect
constitutional rights. This is relevant to any UMRS system holding legal case files.

### D.2 Homeland Security

| Sub-type | C | I | A |
|---|---|---|---|
| D.2.1 Border/Transportation Security | Mod | Mod | Mod |
| D.2.2 Critical Infrastructure Protection | **High** | **High** | **High** |
| D.2.3 Catastrophic Defense | **High** | **High** | **High** |
| D.2.4 Emergency Preparedness (EOP) | **High** | Mod | **High** |

**UMRS impact**: CEII (Critical Energy Infrastructure Information) is rightly at `s1`/High
tier. This confirms the CUI//CEII assignment at the higher MCS range.

### D.3 Intelligence Operations (domestic, unclassified)

| Sub-type | C | I | A |
|---|---|---|---|
| All domestic intelligence sub-types | **High** | **High** | **High** |

Note: This covers unclassified domestic intelligence functions under FISMA. Classified
intelligence activities fall outside this framework entirely.

### D.14 Health

| Sub-type | C | I | A |
|---|---|---|---|
| Access to Care | Low | Mod | Low |
| Health Care Delivery Services | Low | **High** | Low |
| Health Care Administration | Low | Mod | Low |

**Note**: Health has High Integrity (care delivery errors = patient safety risk).
Relevant to HLTH and PRVCY categories.

### D.15 Income Security

| Sub-type | C | I | A |
|---|---|---|---|
| Federal/Tribal Social Services | Low | Mod | Low |
| Unemployment Compensation | Low | Mod | Low |
| Workers' Compensation | Low | Mod | Low |

### C.3.5 Information Technology Management

| Sub-type | C | I | A |
|---|---|---|---|
| C.3.5.5 Information Security | Low | Mod | Low |
| C.3.5.8 System and Network Monitoring | Mod | Mod | Low |

**Note**: UMRS itself falls under C.3.5.5 as an information security management system.
Baseline: {Low, Moderate, Low} — this sets the floor for UMRS system categorization.

---

## 4. How SP 800-60 Connects to SP 800-53 Control Selection

**The chain**:
1. Identify information types processed by the system (using 800-60 Vol 2 tables)
2. Assign provisional security categories {C, I, A} per information type
3. Apply high-water mark → system-level SC (e.g., {Moderate, High, Moderate})
4. The *overall* SC = max across all three dimensions → system impact level (Low/Mod/High)
5. FIPS 200 mandates minimum security requirements based on impact level
6. SP 800-53 Rev 5 + SP 800-53B define control baselines: Low/Moderate/High baseline
7. The system impact level selects the control baseline

**Practical example for UMRS**:
- UMRS processes Law Enforcement (INV: Mod/Mod/Mod) + Information Security (Low/Mod/Low)
- System SC high-water: {Moderate, Moderate, Moderate}
- System impact level: Moderate
- Control baseline: SP 800-53B Moderate baseline applies
- Justification for any High control: requires tailoring documentation

**Control selection shortcuts from baselines**:
- Low C → no SC-28 (protection at rest) typically required, but Moderate+ → SC-28 applies
- High I → AU-10 (non-repudiation) typically enters; High A → CP-2 (contingency plan) required
- UMRS MCS label integrity (preventing label downgrade) is an SI-7 / AU-10 concern at Moderate

**800-60 → 800-53 cross-reference not automatic**: 800-60 defines *what* the system processes;
800-53 defines *what controls* result. The RMF step 2 (Categorize) → step 3 (Select) boundary
is where 800-60 output feeds into 800-53 selection. SP 800-37r2 governs the step transitions.

---

## Quick-Reference: UMRS MCS Assignment Validation Checks

When reviewing `US-CUI-LABELS.json` entries against these baselines:

1. **CUI//CEII and Critical Infrastructure categories** → should map to c-range reflecting High C/I/A
2. **CUI//INV (Investigation)** → Moderate across all three; c-range should distinguish from
   Low-C-only categories like CUI//LEI basic crime prevention
3. **CUI//HLTH** → flag if integrity not at Moderate+ (D.14 Health Care Delivery = High I)
4. **CUI//SP-CTI** → domestic intelligence unclassified = High/High/High; confirm c-range
   assignment reflects this
5. **SP-EXPT export control** → verify c-range places this above Low-tier basic categories;
   800-60 does not cover export control directly but the CEII and Defense baselines provide a floor
6. **Legal Privilege / D.17.2** → High Integrity requirement; any PRIVILEGE category should
   reflect I:High in the level assignment

---

*Source*: NIST SP 800-60 Vol 1 Rev 1 (`.claude/references/nist/sp800-60v1r1.pdf`) and
Vol 2 Rev 1 (`.claude/references/nist/sp800-60v2r1.pdf`).
*Familiarized*: 2026-03-31 by The Librarian.
*Primary consumers*: researcher, security-auditor agents.
