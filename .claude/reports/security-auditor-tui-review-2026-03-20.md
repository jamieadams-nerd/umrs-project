# Security-Auditor TUI Review — Kernel Security & Trust/Evidence Tabs
## Date: 2026-03-20
## Score: 14 ACCURATE / 17 CONCERN / 3 ERROR

---

## ERRORS

**E-1 (MEDIUM):** `kptr_restrict = 2` annotated as `"restricted"` in translate_integer() but recommendation says `"2 (hidden from all users)"`. Auditor sees two different descriptions for same state. Fix: make them match.

**E-2 (MEDIUM):** `NfConntrackAcct` placed in MODULE RESTRICTIONS but it's a network accounting parameter. Group description talks about USB/FireWire exfiltration — nothing to do with nf_conntrack. Fix: move to NETWORK AUDIT group.

**E-3 (LOW):** `UnprivUsernsClone` is Debian/Ubuntu-only sysctl, absent on RHEL 10. Shows as "Unavailable" with no explanation — looks like a probe failure. Fix: note it's platform-specific, name RHEL equivalent.

---

## KEY CONCERNS

### Kernel Security Tab

**C-4 (MEDIUM):** `ModulesDisabled` grouped under CRYPTOGRAPHIC POSTURE but it's a boot integrity control. Move to BOOT INTEGRITY.

**C-7 (LOW):** Help text "Red rows require remediation before CUI processing" is too absolute. Some red items degrade logging but don't block CUI. Revise to "review each indicator's description to determine priority."

**O-2 (MEDIUM):** Kernel tab uses ONLY color for hardened vs not. No ✓/✗ symbols. In NO_COLOR mode, tab is unreadable for posture assessment. Trust/Evidence tab already uses ✓/✗. Add symbols to kernel tab too.

### Trust/Evidence Tab — Verification Column

**C-12 (HIGH priority, MEDIUM severity):** THE VERIFICATION COLUMN GAP.
Verification shows `"✓ ok (fd)"` but doesn't disclose WHAT was verified. An SP 800-53A assessor asking "what examination method was used?" cannot determine that PROC_SUPER_MAGIC or SYSFS_MAGIC was confirmed via fstatfs(2).

Recommended fix: `"✓ ok (fd, PROC_MAGIC)"` / `"✓ ok (fd, SYS_MAGIC)"` per source type.

**C-15 (MEDIUM):** Contradiction text truncated to 48 chars. Full text (naming conflicting fields and values) should appear in a description row below.

**C-13 (LOW):** T3 Platform Verified uses blue/Info instead of green/Ok. T3 is positive — should be green.

**C-16 (LOW):** Evidence groups in alphabetical order. Should follow pipeline execution order (kernel checks first → config → package → filesystem) so assessors follow the trust-elevation narrative.

**O-3 (MEDIUM):** T4 sha256 digest in EvidenceRecord not surfaced in display. Assessor recording a T4 finding needs the digest reference.

---

## WHAT WORKS WELL

- Indicator descriptions are technically accurate, threat-model-focused, and self-contained
- Every description follows: what the setting controls + what the attack is if it fails
- Recommended values correct for all 27 indicators
- `[ Recommended: ... ]` only on red indicators is correct
- Pinned summary pane = correct UX for SP 800-53A (trust finding can't scroll off screen)
- Sticky column headers work correctly
- Contradiction framing "may indicate tampering" is correctly hedged
- Evidence type labels are clear plain English

---

## RECOMMENDED PRIORITY FOR RUSTY

1. C-12 + O-2 — Verification column magic disclosure + ✓/✗ symbols on kernel tab + NO_COLOR
2. E-1 — kptr_restrict annotation fix (one-liner)
3. E-2 + C-4 — Group placement fixes (nf_conntrack, ModulesDisabled)
4. C-13, C-15, O-3 — Trust tab accuracy improvements
5. Help dialog bug fix (height/sizing)
6. All remaining LOWs — polish pass
