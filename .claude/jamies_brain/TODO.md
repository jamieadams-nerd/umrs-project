# Jamie's TODO List

Add items here anytime. Format: `- [ ] item` to track, `- [x] item` when done.

---

## Active — CRITICAL (Next Session, First Priority)

- [ ] **BLOCK OVERFLOW — CROSS-DOMAIN ACCESS LEAKS** (Knox Findings 1-4)
      - PRIV/STUD (c1,c10) collides with PROCURE base (c10) — Student Records reads Procurement
      - CRIT/WATER (c40,c50) collides with INTEL base (c50) — Water reads Intelligence
      - LGL/PRIOR (c140,c150) collides with NUC base (c150) — Prior Arrest reads Nuclear/SGI
      - Fix: widen blocks so no subcategory overflows. Safe allocation needs ~236 slots.
      - Decision needed: expand US range from c0-c199 to c0-c299? Push Canada to c300+?
      - Or variable-width blocks? Or uniform 20-wide?
      - See: `.claude/reports/2026-03-27-setrans-catalog-access-control-review.md`
      - See: `.claude/reports/2026-03-27-session-summary-catalog-work.md`
- [ ] **CANADIAN ENCODING AMBIGUITY** (Knox Finding 7)
      - JSON says all tiers at c200 (BLP-only separation via s1/s2/s3)
      - Setrans uses c200/c201/c202 (category + sensitivity separation)
      - These are DIFFERENT access control models. Must pick one.
      - BLP-only: PC reads PB reads PA (matches Canadian injury ladder)
      - Category+sensitivity: tiers are mutually isolated (breaks tier hierarchy)

## Active — Next Session Priority

- [ ] EXPT group inconsistencies (Knox Findings 5, 9)
      - No bare `s1:c30 = CUI//EXPT` entry — only compound entries
      - `parent_group` self-references as "EXPT" should be "CUI"
- [ ] Metadata/doc drift (Knox Findings 6, 10)
      - JSON metadata says c0-c149 allocated — actual is c0-c180
      - Five Eyes allocation table says c0-c140 — also wrong
      - Fix after block reallocation settles
- [ ] Schema merge — US and CA marking fields still divergent
      - US handling = string, CA handling = structured object with k/v pairs
      - Need unified field set: null out fields that don't apply to a nation
      - Drop handling_group_id and US-CUI-HANDLING.json (inline handling like CA)
      - Drop empty `other: {}` field from US markings
- [ ] US JSON needs category_base/subcategory_number fields (Knox Finding 11)
      - Enables programmatic cross-validation against setrans
- [ ] Palette colors for PA/PB/PC — Jamie asked for this, don't drop it
      - Sage + Elena to review
      - Must survive dark/light themes
      - Rename US-CUI-PALETTE.json → shared UMRS-PALETTE.json
- [ ] SP- prefix in marking keys for specified categories
      - CUI//SP-CTI, CUI//SP-NNPI, etc. — not yet in catalog keys or setrans
- [ ] Description typos: CMPRS "ecords"→"Records", SSEL "ource"→"Source" (Knox Findings 14-15)
- [ ] SystemHigh: c0.c255 vs c0.c1023 (Knox Finding 12), TARGETED missing SystemHigh (Finding 16)
- [ ] French terminology verification (Henri Findings 5, 6) — route to Simone
      - PA injury threshold wording: "limité ou modéré" vs TBS verbatim
      - CSE abbreviation: CST vs CSC

## Active — Ongoing

- [ ] Discuss targeted vs MLS enforcement differences for PA/PB/PC demonstration
      - Targeted: labeling + category control only, NO BLP dominance
      - MLS: full Bell-LaPadula dominance enforcement
- [ ] umrs-label crate rename (from umrs-labels) — Jamie will provide details
- [ ] US-CUI-RISK-DOMAINS.json — low priority, Henri + Knox to review usefulness
- [ ] Five Eyes sharing operational consequence documentation (Henri Finding 10) — deferred

## Backlog


## Done

- [x] Merge US CUI + Canadian Protected catalog schemas (2026-03-27)
- [x] Five Eyes MCS allocation table saved to jamies_brain/
- [x] DoD registry alignment — 18 path fixes, 18 invented entries dropped (2026-03-27)
- [x] CUI Basic/Specified — 9 specified identified, 5 missing added to catalog (2026-03-27)
- [x] setrans.conf restructure — groups on multiples of 10, proper group,sub pattern (2026-03-27)
- [x] Block allocation maps updated in both setrans headers (2026-03-27)
- [x] All 55 missing DoD categories added to US catalog (121 total) (2026-03-27)
- [x] Canadian JSON: "markings"→"labels", category_base fixed, extension example fixed (2026-03-27)
- [x] Both setrans files regenerated with all 121 US + 3 CA entries (2026-03-27)
- [x] 44 setrans integration tests written (setrans_tests.rs) (2026-03-27)
- [x] All 91 tests passing (47 catalog + 44 setrans) (2026-03-27)
- [x] Knox security review: 17 findings (4 critical, 4 high, 5 medium, 4 low) (2026-03-27)
- [x] Henri Canadian review: 10 findings (1 critical, 3 medium, 6 low/info) (2026-03-27)
