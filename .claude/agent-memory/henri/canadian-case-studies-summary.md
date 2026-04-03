# Canadian Case Studies Research Summary

**Date:** 2026-04-03
**Author:** Henri
**Output:** `docs/modules/ROOT/pages/foundations/history/canadian-case-studies.adoc`

## Strongest Cases

These cases have the best documentation, clearest failure patterns, and strongest parallels to the US case study corpus:

1. **Phoenix Pay System** — The single strongest Canadian case. Well-documented by the Auditor General, IEEE Spectrum, and CBC. $5.1B+ cost, 80% of federal employees affected. Perfect "trusted but didn't verify" parallel to OPM. The scale and duration of failure are unmatched.

2. **CRA/ESDC GCKey Breach (2020)** — OPC Special Report to Parliament provides authoritative sourcing. 48,000+ accounts, 31,000+ confirmed breaches. Clean "over-access / no compartmentalization" parallel to OPM and Equifax.

3. **Walkerton Water Crisis (2000)** — 7 deaths, judicial inquiry report (O'Connor). Operators falsified chlorine records for years. Direct parallel to US falsified drinking water reports. The strongest "record falsification kills people" case in the Canadian corpus.

4. **Lac-Megantic (2013)** — 47 deaths, TSB investigation report. Transport Canada failed to follow up on its own safety orders. Parallel to WMATA/MBTA rail inspection failures and Deepwater Horizon oversight gaps.

5. **ArriveCAN** — Auditor General report is authoritative. 46% of contracts had no evidence work was performed. 21% had contractors working without required security clearance. No direct US parallel in current corpus — unique addition.

6. **LifeLabs (2019)** — 15M records, joint BC/Ontario Privacy Commissioner investigation. Four-year fight to suppress report. Clean Equifax parallel.

## Good Supporting Cases

7. **BGRS/SIRVA Military Breach (2023)** — 480K potentially affected, 24 years of data. Good "third-party trust" case.
8. **NEB/Enbridge Audit Suppression** — Regulator edited its own findings. Parallel to Boeing/FAA.
9. **VW Emissions (Canadian prosecution)** — $196.5M fine, 60 counts. Same incident as US case but demonstrates cross-jurisdictional enforcement.
10. **SNC-Lavalin** — $280M fine. Falsified accounting records. Good corporate fraud case.
11. **Nortel Espionage** — Decade-long breach attributed to Chinese state actors. Good corporate espionage case.
12. **Muzzled Scientists** — Information Commissioner confirmed suppression. Unique "inverse UMRS" case about restricting data that should be public.

## Weaker Cases (included but less detailed sourcing)

13. **IRCC Employee Access** — 12 employees, relatively small scale. Included for "insider misuse" pattern.
14. **Ontario Health atHome** — Recent (2025), limited public detail.
15. **eHealth Ontario** — Procurement scandal, not data breach per se.
16. **SSC Data Centre Failures** — Infrastructure failure, not data integrity.
17. **Statistics Canada Census Forms** — Physical custody failure on First Nations reserves.

## Gaps — Canadian Parallels Not Found

- **Pipeline safety record falsification** — Found NEB audit suppression but did not find a direct Canadian parallel to US pipeline inspector fabricating test results. Enbridge case is about regulator suppression, not operator falsification.
- **Rail inspection falsification** — Found oversight failures (Lac-Megantic, CN audit) but not direct falsification of inspection records by Canadian rail inspectors. US cases (WMATA, MBTA, LIRR) involve inspector-level fabrication.
- **Water testing fraud by labs** — Walkerton involved operator falsification. Did not find Canadian laboratory-level falsification parallel to US DOJ cases involving lab directors.
- **First Nations water testing data** — Searched specifically; found systemic under-resourcing but no documented falsification cases.
- **Digital evidence / legal proceedings** — No Canadian parallel found for US v. Vayner or Lorraine v. Markel.
- **Mars Climate Orbiter / engineering unit mismatch** — No Canadian parallel. This is a NASA-specific case.
- **Deepwater Horizon alarm bypass** — No direct Canadian parallel found for "safety configuration silently modified."

## Policy Notes

- All Protected tier mappings in the case studies follow TBS Standard on Security Categorization, Appendix J definitions.
- Provincial health data cases (LifeLabs, Ontario Health atHome) are mapped to "Protected B equivalent" since provincial data is not categorized under the TBS framework, but the injury threshold is comparable.
- The Nortel case is mapped to "Protected B equivalent" for the same reason — corporate data, not government data.
- The scientist muzzling case is included as an "inverse UMRS" example — it demonstrates that controls must work in both directions (restricting sensitive data AND ensuring public data remains accessible).

## Cross-Team Notes

- The ArriveCAN case has no US parallel in the current case study corpus. Jamie may want to add a US procurement fraud case to fill the gap.
- The muzzled scientists case is unique to the Canadian corpus and raises an important design question: UMRS labels should prevent both unauthorized access AND unauthorized restriction.
