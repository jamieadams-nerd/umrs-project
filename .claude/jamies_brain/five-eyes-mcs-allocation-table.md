# Five Eyes MCS Category Allocation Map

This table defines the MCS sensitivity level and category range allocation
for each Five Eyes nation's unclassified-but-controlled marking program.

| Country | Program | Sensitivity Levels | Category Range | Tiers / Markings | Status |
|---|---|---|---|---|---|
| US | CUI (NARA Registry) | s1 only (all CUI at s1 in MLS) | c0-c199 | 72 categories in taxonomy | Catalog exists |
| Canada | Protected (TBS) | s1, s2, s3 (one per tier) | c200-c299 | PA (s1), PB (s2), PC (s3) | Catalog exists |
| UK | GSC OFFICIAL (Cabinet Office) | s1, s2 (OFFICIAL + OFFICIAL-SENSITIVE) | c300-c399 | 2 tiers + 3 descriptors | Pending Jamie decision |
| Australia | PSPF (Attorney-General) | s1, s2, s3 (OFFICIAL, OFFICIAL:Sensitive, PROTECTED) | c400-c499 | 3 tiers | Pending Jamie decision |
| New Zealand | PSR (GCSB/DPMC) | s1, s2 (OFFICIAL + SENSITIVE) | c500-c599 | 2-3 tiers (transitional) | Pending Jamie decision |

## Notes

- **US** uses a single sensitivity level (s1) with a deep category taxonomy (72+ named categories).
  In targeted policy, all US CUI falls to s0 (no MLS enforcement).
- **Canada** uses three sensitivity levels (s1/s2/s3) with no subcategories — the tier IS the
  classification. Categories c201-c299 reserved per tier for future departmental subcategories.
- **UK** OFFICIAL-SENSITIVE is explicitly NOT a classification level per Cabinet Office policy.
  UMRS models it as s2 for enforcement convenience. This is documented as an enforcement mapping,
  not a policy claim.
- **Australia** retained PROTECTED as a distinct classification level (unlike UK which collapsed it).
  PROTECTED sits above OFFICIAL in the Australian hierarchy.
- **New Zealand** is transitional — older IN CONFIDENCE / SENSITIVE terms coexist with the newer
  OFFICIAL framework. MCS mapping may need revision as NZ policy stabilizes.

## Gap Allocation

| Range | Owner | Notes |
|---|---|---|
| c0-c149 | US CUI | Active (c0-c140 used, c141-c149 reserved for US growth) |
| c150-c199 | Reserved | Buffer between US and Canadian allocations |
| c200-c299 | Canada | Active (c200-c202 used, c203-c299 reserved for departmental use) |
| c300-c399 | UK | Reserved — pending catalog construction |
| c400-c499 | Australia | Reserved — pending catalog construction |
| c500-c599 | New Zealand | Reserved — pending catalog construction |
| c600-c1023 | Unallocated | Available for future partner nations or extensions |

## Source

- Henri's Five Eyes research: `.claude/reports/five-eyes-marking-programs.md` (2026-03-25)
- Jamie's catalog schema notes: `jamies_brain/archive/notes-catalog-schema-2026-03-25.txt`
