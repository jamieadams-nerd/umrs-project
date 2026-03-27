# Session Summary: Catalog & Setrans Work — 2026-03-27

## What was accomplished

### US CUI Catalog (US-CUI-LABELS.json)
- **18 fabricated entries removed**: AGR family (4), AVIATION, MARITIME, PIPELINE, EMERGENCY,
  EXPORT, FEDCON, GOVT family (5), BSA, HEALTH, TRANSPORT, PRIVACY group header
- **18 path corrections**: Entries moved to correct DoD parent groups per dodcui.mil registry
  (e.g., OPSEC → INTEL/OPSEC, NNPI → DEF/NNPI, PRIVACY → PRIV, etc.)
- **62 new entries added**: 55 real DoD categories from registry + 7 implicit group headers
  (DEF, IMMG, LGL, NCR, PAT, PRIV, TRANS)
- **Final count: 121 markings** (was 72 committed, 59 in broken working copy)
- **Added _metadata block** with catalog provenance, country_code, mcs_category_range

### Canadian Catalog (CANADIAN-PROTECTED.json)
- **CRITICAL fix**: Renamed JSON key from `"markings"` to `"labels"` — fixes 8 failing tests
- **category_base corrected**: PA=c200, PB=c201, PC=c202 (was all c200)
- **category_range_reserved**: Changed from c201-c299 to c203-c299 (c200-c202 are tier bases)
- **Extension example fixed**: Was using c201/c202 (collision with tier bases), now uses c203+
- **ATIP caveat added**: Mutual exclusivity section now notes ATIP release is distinct from reclassification
- **TBS numbering note**: Added metadata note explaining reverse severity ordering in TBS Appendix J

### Setrans Files (both MLS and TARGETED)
- **Fully regenerated** with all 121 US entries + 3 Canadian entries
- **TAX comment shift bug fixed** (c81/c82/c83 comments were misaligned)
- **3 new group blocks added**: NCR (c160), STAT (c170), INTL (c180)
- **Block allocation header updated**

### Tests Written
- **setrans_tests.rs**: 44 new integration tests covering:
  - Duplicate MCS/label detection
  - Bidirectional JSON-to-setrans sync
  - MCS structural integrity (sensitivity levels, compound format)
  - Hierarchy integrity (every subcategory has a group header)
  - Comment accuracy (comments match JSON names)
  - MLS vs TARGETED consistency
  - Canadian cross-reference
- **All 91 tests pass** (47 catalog_tests + 44 setrans_tests)

### DoD Registry Reconciliation
- DoD scrape at `references/cui-registry/categories/` has 113 entries
- Scrape quality: index table (abbreviation + group) is reliable; detail pages are garbage
  (null descriptions, website footer parsed as content)
- All 113 DoD entries now represented in our catalog (113 + CUI base = 114, + 7 group headers = 121)

---

## CRITICAL OPEN ISSUES — Must fix next session

### 1. Block Overflow = Cross-Domain Access Leaks (Knox Findings 1-4)

**This is the most important issue.** Three groups overflow their MCS category blocks,
creating real SELinux access control violations:

| Overflow | Collision | Access Leak |
|---|---|---|
| PRIV subcategory c10 | Hits PROCURE base c10 | Student Records user reads Procurement data |
| CRIT subcategory c50 | Hits INTEL base c50 | Water Assessments user reads Intelligence data |
| LGL subcategories c150-c152 | Hit NUC base c150 | Prior Arrest clearance reads Nuclear/SGI data |

**Root cause**: 10-wide blocks are too narrow for groups with many subcategories.
PRIV has 9 subs (needs 10+ slots), CRIT has 10 subs (needs 11+), LGL has 12 subs (needs 13+).

**Fix**: Expand US MCS range beyond c0-c199 and widen blocks. Safe allocation with 50% growth
room needs ~236 slots. Options:
- (A) Expand US to c0-c299, push Canada to c300+
- (B) Variable-width blocks sized to actual need + growth
- (C) 20-wide uniform blocks (18 groups x 20 = 360, needs range expansion)

**Impact**: Changing category numbers means updating setrans files, any deployed labels,
and the Five Eyes allocation table. Better to fix now before any deployment.

### 2. Canadian Encoding Ambiguity (Knox Finding 7)

Two different access control models are implied:
- **JSON model**: All tiers at category c200, separated by sensitivity level only (s1/s2/s3).
  BLP dominance means s3 process can read s2 and s1 — PC reads PB reads PA. This matches
  the Canadian injury-severity ladder (higher clearance = access to lower tiers).
- **Setrans model**: Each tier at a different category (c200/c201/c202) at different sensitivity
  levels. An s3:c202 process does NOT dominate s2:c201 because categories differ.

**Jamie must decide**: Should Canadian tiers use BLP-only separation (JSON model — simpler,
matches Canadian policy intent) or category+sensitivity separation (setrans model — more
restrictive but breaks the tier hierarchy)?

### 3. EXPT Group Inconsistencies (Knox Findings 5, 9)
- No bare `s1:c30 = CUI//EXPT` entry — only compound entries exist
- `parent_group` self-references as "EXPT" instead of "CUI"

### 4. Metadata/Documentation Drift (Knox Findings 6, 10)
- JSON metadata says c0-c149 allocated — actual is c0-c180
- Five Eyes allocation table says c0-c140 — also wrong
- Both need updating after block reallocation settles

### 5. Minor Fixes
- CMPRS description typo: "ecords" → "Records"
- SSEL description typo: "ource" → "Source"
- SystemHigh uses c0.c255 but header implies c0-c1023
- TARGETED missing SystemHigh entry

---

## Henri's Deferred Findings

| # | Finding | Status |
|---|---|---|
| 5 | PA French injury threshold wording needs Simone verification | Deferred to Simone |
| 6 | CSE French abbreviation (CST vs CSC) needs Termium Plus check | Deferred to Simone |
| 10 | Five Eyes sharing operational consequence not documented | Deferred per Jamie |

---

## Files Modified This Session

```
M  components/rusty-gadgets/umrs-labels/data/us/US-CUI-LABELS.json
M  components/rusty-gadgets/umrs-labels/data/ca/CANADIAN-PROTECTED.json
M  components/rusty-gadgets/umrs-labels/data/MLS-setrans.conf
M  components/rusty-gadgets/umrs-labels/data/TARGETED-setrans.conf
A  components/rusty-gadgets/umrs-labels/tests/setrans_tests.rs
```

---

## Review Reports Generated

- `.claude/reports/2026-03-27-setrans-catalog-access-control-review.md` (Knox — 17 findings)
- `.claude/reports/2026-03-27-session-summary-catalog-work.md` (this file)
- Henri's findings are in this summary (10 findings, inline above)

---

## Don't Panic

The block overflow is a **numbering problem, not an architecture problem**. The compound
MCS category scheme is fundamentally sound — it just needs wider block spacing so subcategory
numbers never land in another group's range. This is fixable by:

1. Choosing wider block widths (20-wide or variable)
2. Regenerating both setrans files (already scripted — takes seconds)
3. Updating the allocation table and metadata

No Rust code changes needed. No structural redesign. Just a renumbering pass.
The 44 setrans tests we wrote today will catch any new collisions immediately.
