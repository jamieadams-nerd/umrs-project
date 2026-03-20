# Guest-Admin TUI Review — Operator Perspective
## Date: 2026-03-20
## Reviewer: RHEL sysadmin persona, 10+ years
## Note: Review based on source code reading — could not execute binary due to permissions

---

## Summary: 15 findings — 7 HIGH, 6 MEDIUM, 2 LOW
## Usability Rating: 5/10

Solid foundation — layout, color coding, and inline indicator descriptions are genuinely useful. Blocked by: no entry point for a new user, actionable-but-incomplete remediation guidance, and help dialog bug.

---

## HIGH Findings

**H1: No --help flag**
Tool has no self-description at invocation. A sysadmin running `umrs-os-detect-tui --help` gets nothing. Every CLI tool needs --help as the entry point.

**H2: Recommendations say WHAT but not HOW**
`[ Recommended: 1 (locked) ]` tells me the value but not the command. Need: `sysctl -w kernel.kexec_load_disabled=1` or the sysctl.d file path for persistence.

**H3: Module blacklist naming is ambiguous**
"bluetooth (blacklisted)" can be misread as "bluetooth is currently blacklisted" vs "bluetooth SHOULD be blacklisted." Needs clearer phrasing.

**H4: Label Trust field displays Rust enum variant names**
Operators see internal type names instead of plain language.

**H5: No operator documentation page**
No man page, no --help, no docs page for this specific tool in the Antora docs.

**H6: Help dialog rendering bug**
Multi-line help text is clipped to 1 line — dialog height appears hardcoded to 6. The help content cannot actually be read. (Found via source analysis, not visually confirmed.)

**H7: Binary name reads as developer artifact**
`umrs-os-detect-tui` sounds like a test binary, not a production tool. (Note: already planned rename to `umrs-uname`.)

---

## MEDIUM Findings

**M1: `nf_conntrack acct` misplaced**
Currently in MODULE RESTRICTIONS group — it's a network accounting parameter, not a module restriction. Should be in a network or accounting group.

**M2: 'r' (Refresh) key bound but does nothing**
Never disclosed to operator. Either implement it or remove the binding.

**M3-M6:** (Additional medium findings in full report — permission blocked before writing details)

---

## LOW Findings

**L1-L2:** (Minor items — permission blocked)

---

## What Works Well

- Layout, color coding, and inline indicator descriptions are genuinely useful
- The curated approach is right — not trying to show everything
- Status bar key legend is helpful for orientation
- Group descriptions provide context that most tools lack

## What Would Make Me Use This Daily

1. `--help` that explains what this tool does in one sentence
2. Remediation commands in the recommendations (not just target values)
3. Fix the help dialog so I can actually read the help text
4. Rename to `umrs-uname` (already planned)
