---
name: STIG signal coverage mapping — Phase 3
description: Results of signal-to-STIG cross-reference for umrs-platform posture probe catalog
type: project
---

Signal coverage mapping completed 2026-03-17. Report at `.claude/references/reports/stig-signal-coverage.md`.

**Key findings:**
- 36 indicators in catalog; 20 direct STIG matches; 10 CPU mitigation indicators exceed the STIG baseline (UMRS-only value-add)
- Audit gap: 51 STIG auditd rules, zero UMRS coverage — highest-priority expansion area (AU-5, AU-9, AU-12)
- Network gap: 19 STIG net sysctl rules, zero UMRS coverage — second priority (SC-7)
- 7 Tier-1 candidate new indicators: BpfJitHarden, NetIpv4Forwarding, NetIpv4AcceptRedirects, NetIpv4TcpSyncookies, CanModuleBlacklisted, SctpModuleBlacklisted, TipcModuleBlacklisted
- ModulesDisabled is a UMRS strength over STIG (proactive one-way latch vs reactive audit rules)
- DISA RHEL 10 STIG not yet published as of 2026-03-17 — monitor public.cyber.mil

**Why:** Phase 3 of security-auditor-corpus.md — equip security-auditor to map posture signals to STIG rules and apply severity categories.

**How to apply:** When reviewing posture probe additions, check stig-signal-coverage.md for CCE and severity. Use STIG severity as the AssuranceImpact floor (CAT I=Critical, CAT II=High, CAT III=Medium), override upward for DoD/CUI context.

**Pending downloads (as of 2026-03-17):**
- DISA RHEL 9 STIG v2r5 — needs Bash write permission; URL confirmed: `https://dl.dod.cyber.mil/wp-content/uploads/stigs/zip/U_RHEL_9_V2R5_STIG.zip`
- CIS RHEL 9 Benchmark — free registration wall at cisecurity.org; manual download needed
