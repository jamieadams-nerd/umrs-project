# Risk Domain Concept Assessment — CUI File Labeling

**Date:** 2026-03-21
**Author:** security-auditor (The IRS)

---

## Executive Summary

| Question | Verdict |
|---|---|
| Is "risk domain" a recognized concept in any authoritative framework? | No — not as a named construct |
| Is it operationally useful? | Conditionally yes — intent is valid, original implementation was fabricated and must not be restored |
| If worth keeping, what taxonomy source? | NIST SP 800-30 Rev 1, Appendix H, Table H-2 (Adverse Impact Types) |
| Could it feed umrs-stat? | Yes — but only a clearly separated `handling_requirements` field feeds machine checks; impact profile is human-awareness only |
| Recommendation | **Rework** under the name "impact profile" with SP 800-30 anchoring, splitting human-awareness from machine-verifiable handling requirements |

---

## 1. Framework Survey: Does "Risk Domain" Exist Anywhere?

**NIST SP 800-30 Rev 1** does not use the term "risk domain." It defines a precise taxonomy of adverse impact types in Appendix H, Table H-2. This is the closest authoritative analogue:

| Impact Type | When it applies |
|---|---|
| Harm to Operations | Mission/function degradation, noncompliance costs, reputational damage |
| Harm to Assets | Loss or damage to physical or information assets, intellectual property |
| Harm to Individuals | Physical harm, PII exposure, identity theft |
| Harm to Other Organizations | Partner/contractor noncompliance, relational harms |
| Harm to the Nation | Critical infrastructure, national security, government continuity |

SP 800-30 also defines "impact level" as the magnitude of harm from unauthorized disclosure, modification, or destruction of information. This is exactly the concern Jamie wants to communicate.

**FIPS 199** assigns Low/Moderate/High impact values to CIA objectives per information type. It is a scalar severity model, not a consequence taxonomy.

**NARA CUI Registry and 32 CFR Part 2002** define what information is CUI and how to handle it. They do not define consequence categories.

**DoDI 5200.48** focuses on identification, marking, safeguarding, and destruction. No consequence taxonomy.

**DHS Critical Infrastructure Sectors** (16 sectors under the NIIP) are the closest analogue to the domain tags in the original ChatGPT example. However, DHS sector designations have no official connection to the CUI registry. Using them would require custom UMRS mapping documented as a local extension.

**Verdict:** "Risk domain" as a construct does not exist in any authoritative framework. The functional intent is served by the SP 800-30 impact type taxonomy.

---

## 2. The ChatGPT Implementation Must Not Be Restored

The original examples (`AGRO-INFRA`, `FOOD-SUPPLY`, `AGRO-BIOSEC`, `AG-CHEM-SENS`, `ENV-CONTAIN`, `SUPPLY-CHAIN`) have compound integrity problems:

1. No authoritative source defines these identifiers
2. They use inconsistent naming conventions (some sector-based, some consequence-based, some asset-based)
3. They were attached to non-canonical CUI abbreviations (`AGR`, `CHEM`) that the NARA cross-reference report already flagged as fabricated
4. An operator or auditor looking these up finds nothing — they carry the appearance of authority without its substance

This makes them worse than having no tags at all. False authority is more dangerous than acknowledged absence.

---

## 3. Recommendation: Rework as "Impact Profile"

**Rename** from "risk domain" to **impact profile** — aligns with SP 800-30 terminology without claiming a specific standard identifier.

**Taxonomy anchor:** SP 800-30 Rev 1, Appendix H, Table H-2.

**Proposed JSON structure:**

```json
"CUI//CRIT/CEII": {
  "name": "Critical Energy Infrastructure Information",
  "abbrv_name": "CEII",
  "impact_profile": {
    "sp_800_30_impact_types": ["OPS", "ASSET", "NATION"],
    "impact_rationale": "Disclosure could enable physical attacks on energy infrastructure, causing service disruption, asset destruction, and harm to national continuity.",
    "dhs_sector": "Energy"
  },
  "handling_requirements": [],
  "handling": "...",
  "other": {}
}
```

`sp_800_30_impact_types` is structured and queryable (enum over `{OPS, ASSET, IND, ORG, NATION}`). `impact_rationale` is human-awareness prose. `dhs_sector` is optional, applied only where a clear connection exists, and documented as a UMRS extension.

---

## 4. Critical Separation: Human Awareness vs. Machine-Verifiable

The original concept mixed two concerns that must be kept separate:

**Impact profile** (`impact_profile` field) — human awareness, not machine-enforced. Tells the operator what class of harm disclosure causes. Source: SP 800-30 Table H-2.

**Handling requirements** (`handling_requirements` field) — machine-verifiable by `umrs-stat`. Tells the system what additional marks or controls are required. Source: NARA/DoD CUI registry, controlling authority citations.

The 9 CUI Specified categories already documented in `cui-basic-vs-specified.adoc` are the correct first set for `handling_requirements`:

| Category | Checkable requirement |
|---|---|
| CTI | Distribution Statement B–F present |
| NNPI | NOFORN dissemination control present |
| EXPT | ITAR/EAR warning statement present |
| SSI | 49 CFR Part 1520 warning present |
| CVI | 6 CFR Part 27 warning present |
| DCNI, UCNI | UCN marking present |
| PRIVILEGE | ATTORNEY-CLIENT or ATTORNEY-WP control |
| PROPIN | 10 CFR 600.15 notice present |

Note: "encrypted at rest" is cross-cutting (applies to all CUI) and must NOT be encoded per-category — that would create a misleading implication that categories without the flag are exempt.

---

## 5. Sequencing

The NARA cross-reference report (P1–P5 items) must be addressed before adding impact profiles. Building profiles on top of non-canonical abbreviations compounds the integrity problem.

Phase 1: impact profiles (awareness). Phase 2: `handling_requirements` enforcement by `umrs-stat`. This maps directly to the Phase 1/Phase 2 architecture already established.

---

## Citation Anchors

| Framework | Section | Role |
|---|---|---|
| NIST SP 800-30 Rev 1 | Appendix H, Table H-2 | Authoritative adverse impact taxonomy |
| NIST SP 800-30 Rev 1 | Chapter 2, §2.3 | Definition of impact level and impact value |
| FIPS 199 | Security categorization | CIA impact values per information type |
| 32 CFR Part 2002 | CUI Basic handling | Cross-cutting, not category-specific |
| DoDI 5200.48 | §1.2, §3.3 | DoD CUI policy — no consequence taxonomy |
