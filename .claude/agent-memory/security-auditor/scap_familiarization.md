---
name: SCAP/STIG Corpus Familiarization
description: Structured findings from Phase 2 corpus familiarization — CCE annotation debt, Tier-1 indicator candidates, and auditd gap classification for RHEL 10 STIG / SCAP Security Guide corpus
type: reference
---

# SCAP / STIG Corpus Familiarization Notes

**Date:** 2026-03-17
**Source documents:**
- `.claude/references/scap-security-guide/stig-signal-index.md` (451 STIG rules)
- `.claude/references/scap-security-guide/cce-nist-crossref.md` (451 CCE entries)
- `.claude/references/reports/stig-signal-coverage.md` (researcher coverage report)
- `components/rusty-gadgets/libs/umrs-platform/src/posture/catalog.rs` (36 indicators)

---

## 1. CCE Annotation Debt — catalog.rs

**Confirmed finding:** `catalog.rs` has **zero CCE identifiers** in any `nist_controls` string.
All 36 entries use NIST control names only. CCE cross-references must be added.

Additionally, all 36 `nist_controls` strings use the abbreviated form `NIST 800-53` instead of
the canonical `NIST SP 800-53` required by the Citation Format Rule.

### Complete CCE Map — UMRS Indicators with Direct STIG Match

These 13 indicators have confirmed direct STIG matches. CCE identifiers are now known and
must be added to their `nist_controls` fields in `catalog.rs`.

| IndicatorId | CCE | STIG Signal | Authoritative NIST Controls (from STIG) |
|---|---|---|---|
| `KptrRestrict` | CCE-88686-1 | `sysctl_kernel_kptr_restrict` | CM-6(a), SC-30, SC-30(2), SC-30(5) |
| `RandomizeVaSpace` | CCE-87876-9 | `sysctl_kernel_randomize_va_space` | CM-6(a), SC-30, SC-30(2) |
| `UnprivBpfDisabled` | CCE-89405-5 | `sysctl_kernel_unprivileged_bpf_disabled` | AC-6, SC-7(10) |
| `PerfEventParanoid` | CCE-90142-1 | `sysctl_kernel_perf_event_paranoid` | AC-6 |
| `YamaPtraceScope` | CCE-88785-1 | `sysctl_kernel_yama_ptrace_scope` | SC-7(10) |
| `DmesgRestrict` | CCE-89000-4 | `sysctl_kernel_dmesg_restrict` | SI-11(a), SI-11(b) |
| `KexecLoadDisabled` | CCE-89232-3 | `sysctl_kernel_kexec_load_disabled` | CM-6 |
| `ProtectedSymlinks` | CCE-88796-8 | `sysctl_fs_protected_symlinks` | AC-6(1), CM-6(a) |
| `ProtectedHardlinks` | CCE-86689-7 | `sysctl_fs_protected_hardlinks` | AC-6(1), CM-6(a) |
| `CorePattern` | CCE-86714-3 | `sysctl_kernel_core_pattern` | SC-7(10) |
| `BluetoothBlacklisted` | CCE-87455-2 | `kernel_module_bluetooth_disabled` | AC-18(3), AC-18(a), CM-6(a), CM-7(a), CM-7(b), MP-7 |
| `UsbStorageBlacklisted` | CCE-89301-6 | `kernel_module_usb-storage_disabled` | CM-6(a), CM-7(a), CM-7(b), MP-7 |

**Note on NIST control discrepancies:** Several indicators currently cite different control
families than the STIG. For example:
- `KptrRestrict` catalog cites `SI-7, SC-39`; STIG uses `CM-6(a), SC-30, SC-30(2), SC-30(5)`.
  SC-30 (concealment/misdirection) is the correct control for kptr restriction. SI-7 is less
  precise. The STIG citation should take precedence per the auditor's inconsistency rule.
- `DmesgRestrict` catalog cites `SI-7, SC-28`; STIG correctly uses `SI-11(a), SI-11(b)`
  (error handling / information disclosure). SI-11 is the correct control here.
- `YamaPtraceScope` catalog cites `SC-39, AC-6`; STIG uses `SC-7(10)` (network boundary).
  SC-7(10) is more precisely mapped for ptrace scope. Both are defensible; SC-7(10) is STIG-aligned.
- `UnprivBpfDisabled` catalog cites `CM-7, SC-39`; STIG uses `AC-6, SC-7(10)`. The STIG mapping
  to AC-6 (least privilege) is more precise for access control on the BPF syscall.

### Partial Coverage — CCEs for Related STIG Rules

| IndicatorId | Related CCE | Notes |
|---|---|---|
| `FipsEnabled` | CCE-89085-5 (adjacent: `configure_crypto_policy`) | UMRS reads kernel truth; STIG checks policy name. CCE-89085-5 is the nearest STIG anchor. |
| `ModuleSigEnforce` | CCE-89982-3, CCE-88638-2, CCE-90172-8 (module loading audit rules) | STIG audits reactively; UMRS enforces proactively. No direct CCE for cmdline enforcement. |
| `SuidDumpable` | CCE-88330-6 (`disable_users_coredumps`), CCE-88732-3 (`coredump_disable_storage`) | Closest STIG analogs. |

### No CCE Available (UMRS-only indicators)

These have no STIG CCE and the `nist_controls` field cannot be enriched with a CCE. Document
as UMRS-exceeds-STIG-baseline in compliance evidence:

| IndicatorId | Reason |
|---|---|
| `Lockdown` | No STIG rule for lockdown LSM state |
| `ModulesDisabled` | No STIG rule for `kernel.modules_disabled` one-way latch |
| `RandomTrustCpu` | No STIG rule for entropy trust flags |
| `RandomTrustBootloader` | No STIG rule for entropy trust flags |
| `FirewireCoreBlacklisted` | No STIG rule (hardware-rare) |
| `ThunderboltBlacklisted` | No STIG rule |
| `UnprivUsernsClone` | No STIG rule for RHEL 10 |
| `NfConntrackAcct` | No STIG rule (module parameter) |
| All 10 CPU mitigation indicators | No STIG per-CVE cmdline checks |
| `ProtectedFifos` | No direct STIG rule |
| `ProtectedRegular` | No direct STIG rule |

---

## 2. Tier-1 Candidate Refinement

The coverage report identified 7 Tier-1 candidates. After reading the full corpus, the refined
list is as follows. All confirmed, with one addition.

### Confirmed Tier-1 (extend existing patterns, sysctl or module blacklist)

| Proposed IndicatorId | CCE | STIG Signal | NIST Controls | Priority Note |
|---|---|---|---|---|
| `BpfJitHarden` | CCE-89631-6 | `sysctl_net_core_bpf_jit_harden` | CM-6, SC-7(10) | Natural companion to `UnprivBpfDisabled`; same pattern |
| `NetIpv4Forwarding` | CCE-87377-8 | `sysctl_net_ipv4_ip_forward` | CM-6(a), CM-7(a), CM-7(b), SC-5, SC-7(a) | First network sysctl; high operational value |
| `NetIpv4AcceptRedirects` | CCE-90409-4 | `sysctl_net_ipv4_conf_all_accept_redirects` | CM-6(a), CM-7(a), CM-7(b), SC-7(a) | ICMP redirect routing bypass |
| `NetIpv4TcpSyncookies` | CCE-88084-9 | `sysctl_net_ipv4_tcp_syncookies` | CM-6(a), SC-5(1), SC-5(2), SC-5(3)(a) | SYN flood; SC-5 DoS protection |
| `CanModuleBlacklisted` | CCE-89282-8 | `kernel_module_can_disabled` | AC-18, CM-7 | Extends existing blacklist pattern |
| `SctpModuleBlacklisted` | CCE-90489-6 | `kernel_module_sctp_disabled` | CM-6(a), CM-7(a), CM-7(b) | Extends existing blacklist pattern |
| `TipcModuleBlacklisted` | CCE-86569-1 | `kernel_module_tipc_disabled` | CM-6(a), CM-7(a), CM-7(b) | TIPC is low STIG severity but pattern is trivial |

### New Additions (identified during full corpus review)

| Proposed IndicatorId | CCE | STIG Signal | NIST Controls | Priority Note |
|---|---|---|---|---|
| `NetIpv4SendRedirects` | CCE-88360-3 | `sysctl_net_ipv4_conf_all_send_redirects` | CM-6(a), CM-7(a), SC-5, SC-7(a) | Companion to AcceptRedirects; routers should not send redirects |
| `NetIpv4SourceRoute` | CCE-90165-2 | `sysctl_net_ipv4_conf_all_accept_source_route` | CM-6(a), CM-7(a), SC-5, SC-7(a) | Source routing is a spoofing vector |
| `NetIpv4RpFilter` | CCE-88689-5 | `sysctl_net_ipv4_conf_all_rp_filter` | CM-6(a), CM-7(a), SC-7(a) | Reverse path filtering; IP spoofing mitigation |
| `NetIpv6AcceptRa` | CCE-88665-5 | `sysctl_net_ipv6_conf_all_accept_ra` | CM-6(a), CM-7(a), CM-7(b) | IPv6 RA acceptance should be off on non-routers |
| `NetIpv4IcmpBroadcast` | CCE-86918-0 | `sysctl_net_ipv4_icmp_echo_ignore_broadcasts` | CM-7(a), SC-5 | Smurf attack mitigation; trivial sysctl |
| `NetIpv4TcpRatelimit` | CCE-86242-5 | `sysctl_net_ipv4_tcp_invalid_ratelimit` | SC-5 | Rate-limits duplicate ACKs; DoS hardening |

**Revised network sysctl total:** 13 candidate indicators (vs. 4 in the initial report).
The full STIG has 19 `sysctl_net_*` rules; 13 are directly actionable sysctl indicators.
The remaining 6 (IPv6 send/source-route variants) are lower priority but follow the same pattern.

---

## 3. Auditd Gap Classification (51 STIG Rules)

The 51 auditd STIG rules fall into 5 functional sub-categories. Classification below uses
three tiers: **Critical-for-CUI** (must have before claiming AU compliance), **Important**
(strong CUI hardening), **Nice-to-Have / Out-of-Scope** (UMRS platform is not the right layer).

### Sub-Category A: auditd daemon configuration (8 rules)
`auditd_data_retention_*`, `auditd_freq`, `auditd_local_events`, `auditd_log_format`,
`auditd_name_format`, `auditd_overflow_action`, `auditd_write_logs`

**Classification: Critical-for-CUI**
These configure the audit daemon itself (disk space handling, overflow policy, log format).
AU-5 disk space exhaustion on CUI systems is a direct compliance gap. NIST 800-53 AU-5,
AU-4(1). A `AuditdConfig` posture probe class would cover these.

CCE examples: CCE-89040-0 (admin_space_left_action), CCE-87003-0 (overflow_action),
CCE-88921-2 (log_format), CCE-88724-0 (write_logs).

**Proposed UMRS indicator:** `AuditdSpacePolicy` — verify `admin_space_left_action` is not
`ignore`, and `overflow_action` is `SYSLOG` or `HALT`.

### Sub-Category B: audit rules — kernel module loading (3 rules)
`audit_rules_kernel_module_loading_delete`, `audit_rules_kernel_module_loading_finit`,
`audit_rules_kernel_module_loading_init`

**Classification: Critical-for-CUI**
Reactive complement to UMRS's proactive `ModulesDisabled` / `ModuleSigEnforce` indicators.
AC-6(9), AU-12(c). CCEs: CCE-89982-3, CCE-88638-2, CCE-90172-8.

**Proposed UMRS indicator:** `AuditModuleLoadRules` — verify these three audit rules are
loaded and immutable. Requires audit rule file inspection, not sysctl.

### Sub-Category C: audit rules — SELinux enforcement commands (5 rules)
`audit_rules_execution_chcon`, `audit_rules_execution_semanage`,
`audit_rules_execution_setfiles`, `audit_rules_execution_setsebool`,
`audit_rules_execution_chacl` (related)

**Classification: Critical-for-CUI**
SELinux label manipulation must be audited on MLS/targeted systems. AC-6(9), AU-12(c).
CCEs: CCE-87762-1, CCE-89541-7, CCE-88818-0, CCE-87741-5.

### Sub-Category D: audit rules — file/permission modification (28 rules)
`audit_rules_dac_modification_*` (chmod, chown, xattr variants),
`audit_rules_file_deletion_events_*`, `audit_rules_unsuccessful_file_modification_*`,
`audit_rules_usergroup_modification_*`

**Classification: Important**
These are standard DAC and file-event audit rules. Important for AU-12(c) completeness on
CUI systems, but verifying individual audit rules at the posture probe level is very granular.
More appropriate for a dedicated `AuditRuleSet` probe that checks for the full set.

### Sub-Category E: audit rules — privileged commands (18 rules)
`audit_rules_privileged_commands_*` (su, sudo, passwd, mount, kmod, modprobe, etc.),
`audit_rules_suid_privilege_function`, `audit_rules_immutable`,
`audit_rules_system_shutdown`, `audit_privileged_commands_*`

**Classification: Important (with key exceptions)**

- `audit_rules_immutable` (CCE-89816-3, AC-6(9), CM-6(a)): **Critical-for-CUI**.
  Immutable audit rules prevent runtime tampering. This is the highest-value single rule.
- `audit_rules_privileged_commands_sudo` / `su` / `passwd` (CCEs: CCE-89698-5, CCE-89587-0,
  CCE-89215-8): **Important** — privileged command auditing is required for AC-6(9).
- `audit_rules_privileged_commands_kmod` / `modprobe` / `rmmod` (CCEs: CCE-86727-5,
  CCE-89893-2, CCE-88804-0): **Critical-for-CUI** — directly related to kernel module
  integrity, complements `ModulesDisabled`.

### Out-of-Scope Auditd Rules (6 rules)
`audit_rules_execution_setfacl`, `audit_rules_execution_chacl`,
`audit_rules_dac_modification_umount*`, `audit_rules_media_export`

**Classification: Nice-to-Have / Out-of-Scope**
These cover ACL manipulation and media export events — important for full compliance but
not kernel security posture. Appropriate for a future `umrs-logspace` audit ruleset checker,
not the posture probe catalog.

### Summary — Auditd Rules by UMRS Priority

| Priority | Count | Examples | NIST Controls |
|---|---|---|---|
| Critical-for-CUI | ~12 | daemon config, module-load rules, SELinux execution rules, immutable flag | AU-4(1), AU-5(1), AU-5(2), AU-12(c), AC-6(9) |
| Important | ~28 | DAC modifications, privileged commands (sudo/su/passwd) | AU-12(c), AU-2(d) |
| Nice-to-Have / Out-of-Scope | ~11 | ACL rules, media export, specific mount/umount variants | AU-12(c) |

**Recommended Phase 3 approach:** Rather than 51 individual indicators, implement 3 composite
indicators:
1. `AuditdDaemonConfig` — checks daemon configuration settings (8 rules → 1 indicator)
2. `AuditRulesIntegrity` — checks immutable flag + kernel module load rules (4 rules → 1 indicator)
3. `AuditRulesSelinux` — checks SELinux command execution rules (5 rules → 1 indicator)

This brings critical AU coverage with 3 new indicators rather than 51.

---

## 4. Citation Format Issues (Action Item for Coder)

The `nist_controls` runtime strings in `catalog.rs` intentionally use abbreviated form per the
Citation Format Rule note: "Runtime output strings may use abbreviated forms for display
compactness." However, the `IndicatorDescriptor` doc comment at line 48 uses `NIST SP 800-53`
correctly, while all 36 `nist_controls` field values use `NIST 800-53` (abbreviated).

This is **not a violation** — the Citation Format Rule explicitly permits abbreviated forms in
runtime output strings. No action required on citation format in catalog.rs `nist_controls` fields.

---

## 5. Recommended Remediation Actions

### For `coder` (rust-developer agent):
1. Add CCE cross-reference annotations to all 12 indicators with direct STIG matches
   in `catalog.rs`. Format: append `; CCE-XXXXX-X` to the `nist_controls` string.
2. Review 4 indicators where UMRS NIST citations differ from STIG authoritative mappings:
   - `KptrRestrict`: add SC-30, SC-30(2) references
   - `DmesgRestrict`: SI-11(a), SI-11(b) is more precise than SI-7, SC-28
   - `YamaPtraceScope`: SC-7(10) is STIG-aligned; current SC-39 is acceptable but less precise
   - `UnprivBpfDisabled`: AC-6 should be added alongside CM-7

### For `security-engineer`:
1. Document the 3 composite auditd indicators as a Phase 3 design proposal.
2. Add 6 additional network sysctl indicators to the Tier-1 candidate list.
3. Update the STIG coverage report with the revised Tier-1 candidate list (now 13 items).

---

## 6. Key Cross-References

- Coverage report: `.claude/references/reports/stig-signal-coverage.md`
- STIG signal index: `.claude/references/scap-security-guide/stig-signal-index.md`
- CCE cross-ref: `.claude/references/scap-security-guide/cce-nist-crossref.md`
- Source catalog: `components/rusty-gadgets/libs/umrs-platform/src/posture/catalog.rs`
- SCAP/STIG corpus plan: `.claude/plans/scap-stig-corpus-plan.md`
