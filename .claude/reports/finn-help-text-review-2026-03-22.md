# Help Text & Operator Usability Review — Finn (guest-admin)

**Date:** 2026-03-22
**Reviewer:** Finn (guest-admin — operator perspective)
**Files reviewed:**
- `umrs-uname/src/main.rs` — `help_text_for_tab()`, lines 1479–1646
- `umrs-ui/src/dialog.rs` — scroll support
- `umrs-ui/src/status_bar.rs` — KEY_LEGEND constant

**Scope:** Source-only review; no binary available. Evaluated as written text and rendered layout.

---

## Overview

This report evaluates the `?`/`F1` contextual help overlay in `umrs-uname`, a TUI tool that reports kernel security posture. The overlay covers three tabs: OS Information (0), Kernel Security (1), and Trust / Evidence (2).

---

## Tab 0 — OS INFORMATION

**Finding H-01 — LOW**
Section: BOOT ID paragraph
Finding: "Useful for correlating journald log entries to a specific boot session." Accurate but leaves an implied action dangling. The operator learned journald correlation matters but not how to act on it.
Suggestion: Add: "To view all journald logs for this boot, run: `journalctl _BOOT_ID=<boot_id>`."

**Finding H-02 — MEDIUM**
Section: PLATFORM IDENTITY paragraph
Finding: "Cross-checked against /proc/sys/kernel/osrelease for corroboration — two independent sources must agree." An operator does not know what happens when they do NOT agree or where a disagreement appears.
Suggestion: Append: "A disagreement between these sources is reported as a contradiction on the Trust / Evidence tab."

---

## Tab 1 — KERNEL SECURITY

**Finding H-03 — HIGH**
Section: Contradiction types — DRIFT
Finding: The only action given is "Check the audit logs." For an operator encountering DRIFT on a production system, this is not enough. Which audit logs? What command? A system where intended hardening is silently not active is a meaningful security gap.
Suggestion: Expand DRIFT with: "Run `ausearch -m MAC_CONFIG_CHANGE -ts today` or `journalctl -k --since today` to identify what changed the running kernel setting. If no change event is found, treat this as an unexplained deviation and escalate."

**Finding H-04 — HIGH**
Section: Contradiction types — all three
Finding: A single shared paragraph covers all three contradiction types. The action for NOT PERSISTED (persist the setting) is entirely different from DRIFT (investigate a change event). The help text does not differentiate.
Suggestion: Give each contradiction type its own action line:
- DRIFT: "Investigate with `ausearch` or `journalctl -k`."
- NOT PERSISTED: "To persist, add the setting to `/etc/sysctl.d/` and run `sysctl --system`."
- UNVERIFIABLE: "Verify manually with `sysctl <key>` or `cat /proc/sys/<path>`."

**Finding H-05 — MEDIUM**
Section: Contradiction types — UNVERIFIABLE
Finding: "Config exists but kernel value could not be read to confirm." The consequence is not stated. An operator does not know whether UNVERIFIABLE is benign (kernel moved the sysctl path) or potentially adversarial (node removed to hide a value).
Suggestion: Add: "UNVERIFIABLE does not mean the system is compromised, but the tool cannot confirm this setting. Verify manually."

**Finding H-06 — MEDIUM**
Section: NOTE — "Review each indicator's description for risk context and remediation guidance."
Finding: This implies each indicator row shows description and remediation, but does not explain where or how this information appears. An operator scanning a red row does not know whether to scroll, press a key, or look at a side panel.
Suggestion: "Each red indicator row shows a brief description and a recommended value directly below the indicator name. Scroll down within the tab to see all indicators."

**Finding H-07 — LOW**
Section: EVIDENCE CHAIN — "? = Unavailable (dim)"
Finding: "Dim" is a terminal rendering attribute. On monochrome or NO_COLOR terminals, dim and normal text may be indistinguishable.
Suggestion: Replace "(dim)" with "(shown as `?` — kernel node could not be read)".

**Finding H-08 — MEDIUM**
Section: SUMMARY — kernel baseline caveat
Finding: "If the kernel is a major version newer, accuracy cannot be guaranteed." No action guidance.
Suggestion: Add: "When the kernel is ahead of the tool baseline, treat all indicator results as advisory and verify critical settings manually with `sysctl <key>`."

---

## Tab 2 — TRUST / EVIDENCE

**Finding H-09 — HIGH**
Section: TRUST TIERS table
Finding: The trust tier labels are defined but not connected to any operational decision. An operator seeing T0 or T1 needs to know whether CUI processing must stop. An operator seeing T3 needs to know whether T3 is acceptable or T4 is required.
Suggestion: Add: "T0 and T1 indicate insufficient platform verification — do not process CUI on this system without manual investigation. T3 and T4 are the expected production states."

**Finding H-10 — LOW**
Section: TRUST TIERS — T4 definition
Finding: "Verification codes confirm provenance of kernel filesystems" uses "provenance" without prior definition.
Suggestion: Replace with: "All sources agree and the kernel confirmed that /proc and /sys are genuine kernel filesystems, not substitutes."

**Finding H-11 — HIGH**
Section: Contradiction note — "may indicate tampering or a misconfigured system"
Finding: "Tampering" is named but no immediate next step is given. This is the highest-severity implication in the entire help text and it leaves the operator without direction.
Suggestion: Add: "If tampering is suspected, isolate the system and contact your ISSO. Do not continue CUI processing. Preserve the tool output and journald logs: `journalctl -b > /tmp/boot-journal.txt`."

**Finding H-12 — MEDIUM**
Section: VERIFICATION CODES — absence of a code
Finding: The four verification codes are explained. What is not explained: what does it mean when a source has NO verification code?
Suggestion: Add: "Sources without a verification code were read by conventional file path — no additional kernel verification was applied. This is normal for configuration files and package databases."

**Finding H-13 — LOW**
Section: EVIDENCE TYPES — Package database
Finding: A `✗` on a package database source is very different from a `✗` on a `/proc` source. The evidence types treat all sources as equivalent.
Suggestion: Add: "A failed package database read affects OS identity verification but not kernel-level indicators. A failed /proc or /sys read is more significant and reduces the trust tier."

---

## CLI / Navigation Usability

**Finding N-01 — LOW**
Tool: status bar (KEY_LEGEND in status_bar.rs)
Finding: PgDn/PgUp is bound in the keymap but not shown in the status bar legend.
Suggestion: Amend to: `Tab: tabs | ↑↓/jk/PgDn: scroll | ?: help | q: quit`

**Finding N-02 — LOW**
Tool: fallback navigation block (unknown tab index)
Finding: Omits PgDn/PgUp, omits Enter as close key, different format from tab-specific blocks.
Suggestion: Bring to parity with tab-specific blocks.

**Finding N-03 — MEDIUM**
Tool: Tab key behavior while help is open
Finding: The NAVIGATION says "Tab / Shift-Tab: switch between tabs." When help is open, Tab advances the tab AND re-opens help for the new tab. This behavior is undocumented.
Suggestion: Revise to: "While this help is open, Tab switches to the next tab's help page."

---

## Scroll Support

The scrollable dialog implementation is sound. `▲ more above` / `▼ more below` indicators follow terminal conventions.

**Finding S-01 — LOW**
Finding: Arrow key scroll in dialog should be confirmed on first live run.

**Finding S-02 — LOW**
Finding: The `▼ more below` hint gives no indication of remaining lines. Terminal operators expect a count or percentage.
Suggestion: Extend to: `▼ more below (12 lines)`. The information is available from `total_lines` and `scroll_offset`.

---

## High-Assurance Communication Assessment

The overlay communicates that `umrs-uname` is doing something more sophisticated than `uname`. The multi-source corroboration, trust tiers, and verification codes signal a system designed for environments where data integrity cannot be assumed.

However, the overlay does not connect findings to operational decisions. An operator seeing DRIFT on a production CUI system needs to know: can I continue processing, or must I stop? The trust tier table is defined but not mapped to a go/no-go threshold. The tampering warning is alarming but actionless.

**Core finding:** The help text diagnoses conditions accurately but does not give operators the named commands, decision criteria, or escalation path needed to respond. For a high-assurance system, that gap is a usability risk.

---

## Summary

```
Sections reviewed: 4 (Tab 0, Tab 1, Tab 2, fallback) + dialog.rs + status_bar.rs
Total findings: 18 (4 HIGH, 7 MEDIUM, 7 LOW)

HIGH:
  H-03  DRIFT has no named investigation procedure
  H-04  Three contradiction types share one action sentence
  H-09  Trust tier table has no go/no-go operational guidance
  H-11  "May indicate tampering" — no immediate next-step

MEDIUM:
  H-02  Tab 0 does not point to where disagreements are reported
  H-05  UNVERIFIABLE consequence not stated
  H-06  "Review each indicator's description" — doesn't say how
  H-08  Kernel baseline caveat has no action guidance
  H-12  Absence of verification code unexplained
  N-03  Tab-switches-help-page behavior undocumented

LOW:
  H-01  Boot ID doesn't complete journald use case
  H-07  "(dim)" not reliable in NO_COLOR mode
  H-10  "provenance" used without definition
  H-13  Failed package DB vs failed /proc not distinguished
  N-01  Status bar omits PgDn/PgUp
  N-02  Fallback navigation less complete
  S-01  Arrow key scroll needs live confirmation
  S-02  No line count in scroll indicator
```
