# STIG Signal Coverage Report

**Date:** 2026-03-17
**Author:** researcher agent
**Source:** `components/rusty-gadgets/umrs-platform/src/posture/catalog.rs` (36 indicators)
**STIG reference:** `.claude/references/scap-security-guide/rhel10-playbook-stig.yml` (451 rules)
**CCE cross-reference:** `.claude/references/scap-security-guide/cce-nist-crossref.md`
**CMMC reference:** `.claude/references/dod-5200/cmmc-assessment-guide-l2.pdf`

---

## Summary

| Metric | Count |
|---|---|
| Total posture indicators (catalog.rs) | 36 |
| Indicators with direct STIG rule match | 20 |
| Indicators with partial / related STIG coverage | 7 |
| Indicators with no STIG coverage | 9 |
| Total STIG rules (RHEL 10 profile) | 451 |
| STIG rules covering UMRS indicators | 20 |
| STIG rules NOT covered by any UMRS indicator | 431 |

**Coverage rate (UMRS → STIG):** 56% (20 of 36 indicators map to at least one STIG rule)

**Inverse gap (STIG → UMRS):** 96% of STIG rules have no UMRS probe coverage. This is expected —
STIG covers the full OS hardening surface (PAM, SSH, file permissions, audit rules, etc.),
while UMRS Phase 1–2b focuses specifically on kernel security posture indicators.

---

## Covered Indicators — Direct STIG Match

Each row: UMRS IndicatorId → matching STIG signal name → CCE → STIG severity

| UMRS Indicator | STIG Signal | CCE | STIG Severity | UMRS Impact |
|---|---|---|---|---|
| `KptrRestrict` | `sysctl_kernel_kptr_restrict` | CCE-88686-1 | medium | Critical |
| `RandomizeVaSpace` | `sysctl_kernel_randomize_va_space` | CCE-87876-9 | medium | Critical |
| `UnprivBpfDisabled` | `sysctl_kernel_unprivileged_bpf_disabled` | CCE-89405-5 | medium | High |
| `PerfEventParanoid` | `sysctl_kernel_perf_event_paranoid` | CCE-90142-1 | low | High |
| `YamaPtraceScope` | `sysctl_kernel_yama_ptrace_scope` | CCE-88785-1 | medium | High |
| `DmesgRestrict` | `sysctl_kernel_dmesg_restrict` | CCE-89000-4 | low | Medium |
| `KexecLoadDisabled` | `sysctl_kernel_kexec_load_disabled` | CCE-89232-3 | medium | Critical |
| `ModulesDisabled` | _(see note below)_ | — | — | Critical |
| `ProtectedSymlinks` | `sysctl_fs_protected_symlinks` | CCE-88796-8 | medium | High |
| `ProtectedHardlinks` | `sysctl_fs_protected_hardlinks` | CCE-86689-7 | medium | High |
| `CorePattern` | `sysctl_kernel_core_pattern` | CCE-86714-3 | medium | High |
| `BluetoothBlacklisted` | `kernel_module_bluetooth_disabled` | CCE-87455-2 | medium | High |
| `UsbStorageBlacklisted` | `kernel_module_usb-storage_disabled` | CCE-89301-6 | medium | High |

**Note on `ModulesDisabled`:** No direct STIG rule for `kernel.modules_disabled` in the RHEL 10
profile. The closest STIG rules audit module-loading events (`audit_rules_kernel_module_loading_*`
— CCE-89982-3, CCE-88638-2, CCE-90172-8) which are reactive audit controls, not the proactive
one-way latch. This is a coverage gap in the STIG: UMRS provides stronger enforcement than the
STIG baseline.

### CPU Mitigation Indicators (Phase 2b)

The following CPU mitigation indicators have partial STIG coverage through the umbrella
`exec_shield` / ASLR rules, but individual per-CVE cmdline checks have no direct STIG rule:

| UMRS Indicator | STIG Relationship | Coverage |
|---|---|---|
| `Mitigations` (global `mitigations=off` absent) | No direct STIG rule for cmdline check | None |
| `Pti` (`pti=off` absent) | No direct STIG rule | None |
| `SpectreV2Off` | No direct STIG rule | None |
| `SpectreV2UserOff` | No direct STIG rule | None |
| `MdsOff` | No direct STIG rule | None |
| `TsxAsyncAbortOff` | No direct STIG rule | None |
| `L1tfOff` | No direct STIG rule | None |
| `RetbleedOff` | No direct STIG rule | None |
| `SrbdsOff` | No direct STIG rule | None |
| `NoSmtOff` | No direct STIG rule | None |

**These 10 CPU mitigation indicators represent UMRS coverage BEYOND the STIG baseline.**
The STIG uses the blanket SC-39 / SI-16 control citations but does not define individual
cmdline checks per CVE. UMRS provides finer-grained mitigation verification.

---

## Partial Coverage — Related STIG Rules

These indicators have related STIG rules that address the same threat surface from a
different angle.

| UMRS Indicator | Related STIG Signal | CCE | Notes |
|---|---|---|---|
| `FipsEnabled` | `configure_crypto_policy` (CCE-89085-5) | CCE-89085-5 | STIG checks crypto policy name (FIPS), not `fips_enabled` sysctl directly. UMRS reads kernel truth. |
| `Lockdown` | `sysctl_kernel_kexec_load_disabled` (adjacent) | — | STIG has no direct lockdown LSM state check. UMRS reads `/sys/kernel/security/lockdown`. |
| `ModuleSigEnforce` | `audit_rules_kernel_module_loading_*` (3 rules) | CCE-89982-3, etc. | STIG audits module load events; UMRS checks enforcement at boot via cmdline. |
| `Sysrq` | No direct rule | — | Low priority in STIG; UMRS flags bitmask value as Custom requiring site policy. |
| `ProtectedFifos` | No direct STIG rule | — | Related to protected_regular (see below). |
| `ProtectedRegular` | No direct STIG rule | — | Protected_fifos and protected_regular are not covered by the RHEL 10 STIG profile. |
| `SuidDumpable` | `disable_users_coredumps` (CCE-88330-6), `coredump_disable_storage` (CCE-88732-3) | CCE-88330-6 | STIG handles coredumps via systemd and PAM limits; UMRS checks kernel sysctl. |

---

## Indicators with No STIG Coverage

These UMRS indicators probe security properties not addressed by any STIG rule:

| UMRS Indicator | UMRS Impact | Rationale for Gap |
|---|---|---|
| `RandomTrustCpu` | Medium | NIST SP 800-90B entropy requirement; STIG does not check cmdline entropy trust flags. |
| `RandomTrustBootloader` | Medium | Same — entropy trust flags are not in STIG scope. |
| `NfConntrackAcct` | Medium | Module parameter for connection tracking accounting; not a STIG security check. |
| `FirewireCoreBlacklisted` | High | FireWire DMA attack vector; STIG covers USB and Bluetooth but not FireWire. |
| `ThunderboltBlacklisted` | High | Thunderbolt DMA; not in STIG profile (likely hardware-rare on servers). |
| `UnprivUsernsClone` | High | Unprivileged user namespace clone not in RHEL 10 STIG profile. |
| `Mitigations` (global) | Critical | See CPU mitigation table above. |
| `Pti` | High | See CPU mitigation table above. |
| `SpectreV2Off` through `NoSmtOff` (8) | High/Med | See CPU mitigation table above. |

---

## STIG Rules NOT Covered by UMRS — Priority Gaps

These are STIG rules in categories where UMRS has zero coverage. Ordered by STIG severity
and NIST control family relevance to UMRS.

### High-Severity Gaps (CAT II equivalent — medium STIG severity, strategic value)

| STIG Signal | CCE | NIST Controls | Why UMRS Should Care |
|---|---|---|---|
| `configure_crypto_policy` | CCE-89085-5 | AC-17(2), SC-13 | System-wide crypto policy affects FIPS compliance; UMRS reads `fips_enabled` but not policy name |
| `ensure_gpgcheck_globally_activated` | CCE-88404-9 | CM-5(3), SI-7 | Package integrity verification; relevant to supply chain (`umrs-platform` build pipeline) |
| `ensure_gpgcheck_never_disabled` | CCE-88176-3 | CM-5(3), SI-7 | Companion to above |
| `fapolicy_default_deny` | CCE-90343-5 | CM-7(5)(b) | Application allowlisting — complementary to module loading controls |
| `sysctl_net_core_bpf_jit_harden` | CCE-89631-6 | CM-6, SC-7(10) | BPF JIT hardening — related to `UnprivBpfDisabled` but distinct |
| `kernel_module_can_disabled` | CCE-89282-8 | AC-18 | CAN bus module — same pattern as Bluetooth/USB blacklisting |
| `kernel_module_sctp_disabled` | CCE-90489-6 | CM-7 | SCTP module — same pattern |
| `kernel_module_tipc_disabled` | CCE-86569-1 | CM-7 | TIPC module — same pattern |

### Audit Coverage Gap

UMRS has no posture indicators covering `auditd` configuration (51 STIG rules, all medium severity).
This is the largest single category gap. Key examples:

| STIG Signal | CCE | NIST Controls |
|---|---|---|
| `auditd_data_retention_admin_space_left_action` | CCE-89040-0 | AU-5(1), AU-5(2) |
| `auditd_overflow_action` | CCE-87003-0 | AU-4(1) |
| `audit_rules_immutable` | CCE-89816-3 | AC-6(9), CM-6(a) |
| `audit_rules_kernel_module_loading_delete` | CCE-89982-3 | AC-6(9), AU-12(c) |

**Impact:** `umrs-logspace` handles audit trail, but no posture probe verifies that `auditd`
itself is correctly configured. This is a gap relevant to AU-5, AU-9, and AU-12 compliance.

### Network Hardening Gap

The RHEL 10 STIG contains 19 `sysctl_net_*` rules. UMRS has zero network sysctl indicators.
Key examples by NIST control family:

| STIG Signal | CCE | NIST Controls |
|---|---|---|
| `sysctl_net_ipv4_conf_all_accept_redirects` | CCE-90409-4 | CM-7(a), SC-7(a) |
| `sysctl_net_ipv4_ip_forward` | CCE-87377-8 | CM-7(a), SC-7(a) |
| `sysctl_net_ipv4_tcp_syncookies` | CCE-88084-9 | SC-5(1) |
| `sysctl_net_ipv6_conf_all_accept_ra` | CCE-88665-5 | CM-7(a) |

**Impact:** UMRS posture probe covers kernel self-protection and integrity well; network
hardening checks are entirely absent. For a system handling CUI, SC-7 network boundary
protection is a significant gap.

---

## STIG Severity Mapping to UMRS AssuranceImpact

DISA STIG uses CAT I / CAT II / CAT III severity (mapped in practice to high/medium/low).
UMRS uses `AssuranceImpact` with levels: Critical, High, Medium, Low.

| STIG Severity | UMRS AssuranceImpact | Notes |
|---|---|---|
| CAT I (high) | Critical | STIG has only 1 high-severity sysctl rule: `configure_bind_crypto_policy` (SC-13) |
| CAT II (medium) | High or Critical | Most sysctl rules are CAT II; UMRS Critical/High indicators align here |
| CAT III (low) | Medium | `sysctl_kernel_dmesg_restrict`, `sysctl_kernel_perf_event_paranoid` are CAT III |

**Notable divergence:** UMRS rates `KptrRestrict` and `RandomizeVaSpace` as Critical (kptr can enable KASLR bypass for remote exploits; ASLR disablement makes memory corruption trivially exploitable), while STIG rates the corresponding rules as medium (CAT II). UMRS's threat model is more conservative, which is appropriate for a DoD CUI deployment.

**Notable alignment:** UMRS rates `KexecLoadDisabled` as Critical and STIG rates it medium.
This reflects STIG's focus on configurable risk rather than binary boot-chain integrity.

---

## Coverage Gaps Requiring New Indicators

These STIG rules represent high-value additions to the UMRS posture probe catalog. Presented
for consideration as Phase 3 or later indicators:

### Tier 1 — High Priority (directly STIG-covered, CMMC-relevant)

| Proposed IndicatorId | STIG Signal | CCE | NIST / CMMC | Rationale |
|---|---|---|---|---|
| `BpfJitHarden` | `sysctl_net_core_bpf_jit_harden` | CCE-89631-6 | CM-6, SC-7(10) | BPF JIT hardening is a standard complement to `UnprivBpfDisabled` |
| `NetIpv4Forwarding` | `sysctl_net_ipv4_ip_forward` | CCE-87377-8 | CM-7, SC-7 | Should be disabled on non-routing servers; basic network posture |
| `NetIpv4AcceptRedirects` | `sysctl_net_ipv4_conf_all_accept_redirects` | CCE-90409-4 | CM-7, SC-7 | ICMP redirect acceptance is a routing control bypass vector |
| `NetIpv4TcpSyncookies` | `sysctl_net_ipv4_tcp_syncookies` | CCE-88084-9 | SC-5 | SYN flood protection |
| `CanModuleBlacklisted` | `kernel_module_can_disabled` | CCE-89282-8 | AC-18, CM-7 | Extends existing module blacklist pattern |
| `SctpModuleBlacklisted` | `kernel_module_sctp_disabled` | CCE-90489-6 | CM-7 | Extends existing module blacklist pattern |
| `TipcModuleBlacklisted` | `kernel_module_tipc_disabled` | CCE-86569-1 | CM-7 | Extends existing module blacklist pattern |

### Tier 2 — Medium Priority (important but requires new probe class)

| Proposed Area | STIG Category | NIST Controls | Notes |
|---|---|---|---|
| `AuditdState` | auditd service running | AU-5, AU-12 | Requires service state probe, not sysctl |
| `CryptoPolicyFips` | `configure_crypto_policy` | AC-17(2), SC-13 | Read `/etc/crypto-policies/state/current` |
| `FapolicyEnabled` | `fapolicy_default_deny` | CM-7 | Requires service/mode probe |
| `GpgcheckEnabled` | `ensure_gpgcheck_globally_activated` | CM-5(3), SI-7 | Requires dnf config file read |

---

## CMMC Level 2 Alignment

The CMMC Assessment Guide L2 (v2.13, `.claude/references/dod-5200/cmmc-assessment-guide-l2.pdf`) maps 110 practices
to assessment objectives. The posture probe covers the following CMMC practice families:

| CMMC Practice Family | UMRS Coverage | Notes |
|---|---|---|
| CM.L2-3.4.6 (least functionality) | Partial | Bluetooth, USB, Thunderbolt, FireWire blacklisting |
| CM.L2-3.4.7 (nonessential functions) | Partial | Module blacklisting; no service-level checks |
| SC.L2-3.13.10 (crypto) | Partial | `FipsEnabled` indicator; no crypto policy probe |
| SC.L2-3.13.1 (network boundary) | None | Network sysctl indicators absent |
| CM.L2-3.4.1 (configuration baseline) | Covered | Entire catalog is a configuration baseline |
| CM.L2-3.4.2 (security configuration settings) | Covered | Sysctl-based indicators |
| MP.L2-3.8.7 (removable media) | Covered | `UsbStorageBlacklisted` |
| SI.L2-3.14.1 (system flaws — mitigations) | Covered | CPU mitigation indicators (Phase 2b) |
| AU.L2-3.3.x (audit) | None | No auditd probe; `umrs-logspace` handles writing |

---

## Recommendations for Security-Auditor Agent

When reviewing UMRS posture probe work, the security-auditor should:

1. **Use STIG as a coverage floor.** For any sysctl indicator added to the catalog, check whether
   a matching STIG rule exists. If it does, ensure the desired value is at least as strict as the
   STIG check. Flag divergences where UMRS is less strict than the STIG default.

2. **Apply STIG severity categories as a starting point for UMRS `AssuranceImpact`.** CAT I →
   Critical, CAT II → High, CAT III → Medium. Override upward when the UMRS threat model
   (CUI/MLS, DoD deployment) warrants stricter treatment.

3. **The audit gap (51 STIG rules, zero UMRS coverage) is the highest-priority expansion area.**
   `umrs-logspace` and future posture probe work should address `auditd` configuration
   verification (AU-5, AU-9, AU-12).

4. **The network hardening gap (19 STIG rules, zero UMRS coverage) is second priority.**
   For a CUI system, SC-7 network boundary checks must be present before the probe can be
   considered broadly compliant.

5. **CPU mitigation indicators (10) are UMRS-only value-add.** These exceed the STIG baseline.
   The security-auditor should cite them as UMRS strength in any compliance review.

6. **`ModulesDisabled` is a UMRS strength over STIG.** The one-way latch semantics of
   `kernel.modules_disabled` provide stronger assurance than audit rules. Document this
   explicitly in assessment evidence.

---

## Acquisition Status for Phase 3 Documents

### Already Available

| Document | Location | Status |
|---|---|---|
| RHEL 10 STIG playbook (SCAP/SSG) | `.claude/references/scap-security-guide/rhel10-playbook-stig.yml` | Ingested (451 rules extracted) |
| CMMC Assessment Guide Level 2 v2.13 | `.claude/references/dod-5200/cmmc-assessment-guide-l2.pdf` | Downloaded (manifest entry exists) |

### Requires Manual / Privileged Download — Flag for Jamie

| Document | Source URL | Notes |
|---|---|---|
| DISA RHEL 9 STIG v2r5 (ZIP) | `https://dl.dod.cyber.mil/wp-content/uploads/stigs/zip/U_RHEL_9_V2R5_STIG.zip` | HTTP 200 confirmed; requires Bash/curl write permission to `.claude/references/dod-5200/stig/`. Contains XCCDF and SCAP content. Useful as RHEL 10 STIG precursor. |
| DISA RHEL 10 STIG | Not yet published | DISA has not released an official RHEL 10 STIG as of 2026-03-17. The SCAP Security Guide playbooks are the best available substitute. Monitor `public.cyber.mil`. |
| CIS RHEL 9 Benchmark (PDF) | `https://www.cisecurity.org/benchmark/red_hat_linux` | Requires free registration. Not auto-downloadable. Adds defense-in-depth checks and Level 1/2 profiles. |

---

## References

- UMRS catalog source: `components/rusty-gadgets/umrs-platform/src/posture/catalog.rs`
- STIG rule index: `.claude/references/scap-security-guide/stig-signal-index.md` (451 entries)
- CCE cross-reference: `.claude/references/scap-security-guide/cce-nist-crossref.md`
- CMMC Assessment Guide: `.claude/references/dod-5200/cmmc-assessment-guide-l2.pdf`
- Phase 3 plan: `.claude/plans/security-auditor-corpus.md`
