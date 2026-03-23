---
name: Henri first task assignments
description: Three initial tasks — French QA, Protected mapping with Herb, and CANADIAN-PROTECTED.json catalog
type: project
---

## Task 1: French QA on umrs-uname

Review Simone's French Canadian translation of the umrs-uname tool output.

- Focus: regulatory weight, not linguistic correctness (that's Simone's lane)
- Check: do translated security terms carry the correct policy meaning per TBS/CCCS usage?
- Check: would a Canadian federal operator recognize these terms from their own documentation?
- Deliver: findings in standard format, routed to Simone for linguistic action and Jamie for policy decisions
- Translation files: `resources/i18n/umrs-uname/fr_CA.po`

## Task 2: Protected A/B/C Mapping (with Herb)

Work with Herb (security-auditor) and Jamie to map out the Canadian Protected designation system.

- Map Protected A/B/C injury thresholds from TBS Standard on Security Categorization §J.2.4.2
- Identify which TBS sections govern each tier
- Document the structural difference: impact-based ladder (Canada) vs category-based taxonomy (US)
- Document where the CUI-to-Protected correspondence breaks down — with caveats at every level
- Confirm BLP dominance applies correctly to the Protected A < B < C hierarchy
- Note: Canada has NO standardized LDCs (no NOFORN equivalent) — two-axis system only

**MCS Architecture (confirmed by Jamie):**
- s0 = OS baseline
- s1:c0–c199 = US CUI categories
- s1:c200+ = Canadian Protected A
- s2:c200+ = Canadian Protected B
- s3:c200+ = Canadian Protected C

**Collaborators:** Henri (regulatory accuracy) + Herb (security controls) + Jamie (decisions)

## Task 3: Build CANADIAN-PROTECTED.json

Create the enrichment catalog — same structure as the CUI catalog — so that when
an operator runs `umrs-stat` on a file carrying e.g. s2:c200, the lookup returns:

- Designation: Protected B
- French: Protégé B
- Injury threshold: serious injury to non-national interests
- Authority: TBS Directive on Security Management, Appendix J, §J.2.4.2
- Handling: need-to-know (procedural, not marking-based)
- No LDCs — Canadian system does not use standardized dissemination caveats

The catalog provides the same depth of enrichment as the US CUI catalog.
When a file is `stat`'d, the operator gets useful, authoritative information
about what the designation means and what governs it.

**Not in scope for this catalog:** display labeling decisions (how umrs-stat
renders the label on screen). That's a downstream presentation question.

**Collaborators:** Henri (content authority) + Herb (control mappings) + Rusty (JSON structure) + Jamie (approval)
