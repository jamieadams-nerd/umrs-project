---
name: SCAP/STIG Corpus Familiarization — RHEL 10
description: CCE mappings for existing IndicatorId variants, new indicator candidates from STIG corpus, and methodology comparison notes. Produced during Phase 2 of scap-stig-corpus-plan.
type: reference
---

# SCAP/STIG Corpus Familiarization

**Source files:**
- `.claude/references/scap-security-guide/stig-signal-index.md` — 451 STIG signals with CCE, NIST, severity, check method
- `.claude/references/scap-security-guide/cce-nist-crossref.md` — 451 CCEs sorted by CCE number

**Profile:** `xccdf_org.ssgproject.content_profile_stig` from `rhel10-playbook-stig.yml`

---

## 1. Existing IndicatorId → CCE Mapping

All catalog indicators that have a direct STIG equivalent. STIG signal names follow
the `stig-signal-index.md` column "Signal Name".

| IndicatorId | sysctl key / cmdline param | STIG Signal Name | CCE | STIG NIST Controls | Severity |
|---|---|---|---|---|---|
| `KptrRestrict` | `kernel.kptr_restrict` | `sysctl_kernel_kptr_restrict` | **CCE-88686-1** | CM-6(a), SC-30, SC-30(2), SC-30(5) | medium |
| `RandomizeVaSpace` | `kernel.randomize_va_space` | `sysctl_kernel_randomize_va_space` | **CCE-87876-9** | CM-6(a), SC-30, SC-30(2) | medium |
| `UnprivBpfDisabled` | `kernel.unprivileged_bpf_disabled` | `sysctl_kernel_unprivileged_bpf_disabled` | **CCE-89405-5** | AC-6, SC-7(10) | medium |
| `PerfEventParanoid` | `kernel.perf_event_paranoid` | `sysctl_kernel_perf_event_paranoid` | **CCE-90142-1** | AC-6 | low |
| `YamaPtraceScope` | `kernel.yama.ptrace_scope` | `sysctl_kernel_yama_ptrace_scope` | **CCE-88785-1** | SC-7(10) | medium |
| `DmesgRestrict` | `kernel.dmesg_restrict` | `sysctl_kernel_dmesg_restrict` | **CCE-89000-4** | SI-11(a), SI-11(b) | low |
| `KexecLoadDisabled` | `kernel.kexec_load_disabled` | `sysctl_kernel_kexec_load_disabled` | **CCE-89232-3** | CM-6 | medium |
| `ProtectedSymlinks` | `fs.protected_symlinks` | `sysctl_fs_protected_symlinks` | **CCE-88796-8** | AC-6(1), CM-6(a) | medium |
| `ProtectedHardlinks` | `fs.protected_hardlinks` | `sysctl_fs_protected_hardlinks` | **CCE-86689-7** | AC-6(1), CM-6(a) | medium |
| `CorePattern` | `kernel.core_pattern` | `sysctl_kernel_core_pattern` | **CCE-86714-3** | SC-7(10) | medium |
| `BluetoothBlacklisted` | `bluetooth` (modprobe.d) | `kernel_module_bluetooth_disabled` | **CCE-87455-2** | AC-18(3), AC-18(a), CM-6(a), CM-7(a), CM-7(b), MP-7 | medium |
| `UsbStorageBlacklisted` | `usb_storage` (modprobe.d) | `kernel_module_usb-storage_disabled` | **CCE-89301-6** | CM-6(a), CM-7(a), CM-7(b), MP-7 | medium |
| `Pti` | `pti=off` (cmdline absent) | `grub2_pti_argument` | **CCE-88971-7** | SI-16 | low |

### No-match catalog indicators

These IndicatorId variants have no direct STIG equivalent in the corpus:

| IndicatorId | Reason for no match |
|---|---|
| `Sysrq` | STIG does not include `kernel.sysrq`; it is a site-policy item in RHEL 10 STIG |
| `ModulesDisabled` | STIG does not check `kernel.modules_disabled` (one-way latch post-boot) |
| `UnprivUsernsClone` | STIG does not include `kernel.unprivileged_userns_clone` |
| `ProtectedFifos` | No STIG equivalent for `fs.protected_fifos` |
| `ProtectedRegular` | No STIG equivalent for `fs.protected_regular` |
| `SuidDumpable` | No STIG equivalent for `fs.suid_dumpable` |
| `Lockdown` | Not in STIG corpus; lockdown LSM is not a standard STIG check item |
| `ModuleSigEnforce` | Not in STIG corpus as a direct check |
| `Mitigations` | STIG checks individual flags, not the umbrella `mitigations=off` |
| `RandomTrustCpu` | Not in STIG corpus |
| `RandomTrustBootloader` | Not in STIG corpus |
| `FipsEnabled` | STIG checks FIPS indirectly via `aide_use_fips_hashes` and crypto policy; no direct `crypto.fips_enabled` sysctl check |
| `NfConntrackAcct` | Not in STIG corpus |
| `FirewireCoreBlacklisted` | Not in STIG corpus (only bluetooth and usb-storage are checked) |
| `ThunderboltBlacklisted` | Not in STIG corpus |
| `SpectreV2Off` | Not individually checked — STIG uses `grub2_pti_argument` only |
| `SpectreV2UserOff` | Not individually checked |
| `MdsOff` | Not individually checked |
| `TsxAsyncAbortOff` | Not individually checked |
| `L1tfOff` | Not individually checked |
| `RetbleedOff` | Not individually checked |
| `SrbdsOff` | Not individually checked |
| `NoSmtOff` | Not individually checked |

---

## 2. New Indicator Candidates (Top Priority)

These STIG signals are not covered by any current IndicatorId but are directly relevant to
our kernel/sysctl monitoring scope. Ordered by priority based on security impact and
check-method alignment with our live-read + configured architecture.

### Tier 1 — Sysctl, high impact, direct parallel to existing indicators

| Candidate IndicatorId | STIG Signal | CCE | NIST Controls | Severity | Rationale |
|---|---|---|---|---|---|
| `BpfJitHarden` | `sysctl_net_core_bpf_jit_harden` | **CCE-89631-6** | CM-6, SC-7(10) | medium | BPF JIT hardening (value=2) makes JIT-compiled BPF programs harder to exploit; pairs with `UnprivBpfDisabled` |
| `Ipv4TcpSyncookies` | `sysctl_net_ipv4_tcp_syncookies` | **CCE-88084-9** | CM-6(a), CM-7(a), SC-5(1), SC-5(2) | medium | SYN flood protection; direct network hardening check via sysctl |
| `Ipv4IpForward` | `sysctl_net_ipv4_ip_forward` | **CCE-87377-8** | CM-6(a), CM-7(a), SC-5, SC-7(a) | medium | IP forwarding must be disabled on non-router systems; directly readable from `/proc/sys/net/ipv4/ip_forward` |
| `Ipv4ConfAllSendRedirects` | `sysctl_net_ipv4_conf_all_send_redirects` | **CCE-88360-3** | CM-6(a), CM-7(a), SC-5, SC-7(a) | medium | Disable sending ICMP redirects on all IPv4 interfaces |
| `Ipv4ConfAllAcceptRedirects` | `sysctl_net_ipv4_conf_all_accept_redirects` | **CCE-90409-4** | CM-6(a), CM-7(a), SC-7(a) | medium | Disable accepting ICMP redirects on all IPv4 interfaces |
| `Ipv4ConfAllAcceptSourceRoute` | `sysctl_net_ipv4_conf_all_accept_source_route` | **CCE-90165-2** | CM-6(a), CM-7(a), SC-5, SC-7(a) | medium | Disable source-routed packet acceptance |
| `Ipv4ConfAllRpFilter` | `sysctl_net_ipv4_conf_all_rp_filter` | **CCE-88689-5** | CM-6(a), CM-7(a), SC-7(a) | medium | Reverse path filtering; prevents IP spoofing |
| `Ipv4IcmpIgnoreBroadcasts` | `sysctl_net_ipv4_icmp_echo_ignore_broadcasts` | **CCE-86918-0** | CM-7(a), SC-5 | medium | Ignore broadcast echo requests (Smurf amplification prevention) |
| `Ipv6ConfAllAcceptRa` | `sysctl_net_ipv6_conf_all_accept_ra` | **CCE-88665-5** | CM-6(a), CM-7(a) | medium | Disable accepting IPv6 router advertisements |
| `Ipv6ConfAllAcceptRedirects` | `sysctl_net_ipv6_conf_all_accept_redirects` | **CCE-90083-7** | CM-6(a), CM-7(a) | medium | Disable accepting IPv6 ICMP redirects |
| `Ipv4TcpInvalidRatelimit` | `sysctl_net_ipv4_tcp_invalid_ratelimit` | **CCE-86242-5** | SC-5 | medium | Rate limit duplicate TCP ACKs; DoS mitigation |

### Tier 2 — Cmdline (grub2), direct parallel to existing KernelCmdline indicators

| Candidate IndicatorId | STIG Signal | CCE | NIST Controls | Severity | Rationale |
|---|---|---|---|---|---|
| `PagePoison` | `grub2_page_poison_argument` | **CCE-89086-3** | CM-6(a) | medium | `page_poison=1` on cmdline; use-after-free mitigation for freed pages |
| `InitOnFree` | `grub2_init_on_free` | **CCE-90140-5** | SC-3 | medium | `init_on_free=1`; zero-fills freed memory to prevent data leakage |
| `VsyscallNone` | `grub2_vsyscall_argument` | **CCE-87153-3** | CM-7(a) | medium | `vsyscall=none`; eliminates legacy vsyscall page as an exploit gadget |
| `AuditEnabled` | `grub2_audit_argument` | **CCE-88376-9** | AC-17(1), AU-10, AU-14(1), CM-6(a), IR-5(1) | low | `audit=1` on cmdline; enables audit subsystem from kernel boot |
| `AuditBacklogLimit` | `grub2_audit_backlog_limit_argument` | **CCE-88192-0** | CM-6(a) | low | `audit_backlog_limit=N`; prevents audit event loss under load |
| `NoexecShield` | `sysctl_kernel_exec_shield` | **CCE-89079-8** | CM-6(a), SC-39 | medium | NX/exec-shield; STIG checks cmdline `noexec` argument presence |

### Tier 3 — Kernel module blacklists (parallel to existing Phase 2a)

| Candidate IndicatorId | STIG Signal | CCE | NIST Controls | Severity | Rationale |
|---|---|---|---|---|---|
| `CanBlacklisted` | `kernel_module_can_disabled` | **CCE-89282-8** | AC-18 | medium | CAN bus protocol module; not needed on servers |
| `SctpBlacklisted` | `kernel_module_sctp_disabled` | **CCE-90489-6** | CM-6(a), CM-7(a) | medium | SCTP protocol module; not needed on most government systems |
| `TipcBlacklisted` | `kernel_module_tipc_disabled` | **CCE-86569-1** | CM-6(a), CM-7(a) | low | TIPC (Transparent Inter-Process Communication) protocol module |

---

## 3. Methodology Comparison: STIG vs UMRS Configured-vs-Live

### STIG Check Approach (check_method column in signal index)

The STIG corpus uses several check methods:

| STIG check_method | How it works | Coverage in STIG |
|---|---|---|
| `sysctl` | Reads live value from `/proc/sys/...` via `sysctl` command or direct read | ~33 indicators |
| `cmdline` | Reads `/proc/cmdline` or parses `/etc/default/grub` | ~9 indicators |
| `other` | Ansible/OVAL tasks that may check files, packages, services, or multi-step logic | ~380 indicators (the majority) |
| `audit-rule` | Checks `/etc/audit/rules.d/` for specific `auditctl` rules | ~30 indicators |
| `file-check` | Stat/permission checks on specific file paths | ~30 indicators |

**Key insight:** STIG `sysctl` checks are purely live-value reads — they do not cross-reference
configured values. The STIG approach has no equivalent of our configured-vs-live contradiction
detection. A STIG check passes if the live value is correct, regardless of whether a sysctl.d
configuration exists to enforce it across reboots.

### UMRS Configured-vs-Live Approach

Our posture engine reads:
1. **Live value** — from `/proc/sys/...` or `/sys/module/.../parameters/...` via `SecureReader` (provenance-verified)
2. **Configured value** — merged from `/etc/sysctl.d/` files

We then detect **contradictions**: live=hardened but configured=missing (not persistent across reboots),
or configured=hardened but live=unhardened (setting not applied yet).

**This is a strict superset of the STIG check.** UMRS catches an extra failure class that
STIG misses: a system that passes STIG today (live value correct) but will fail after reboot
(no persistent sysctl.d entry). This distinction is audit-relevant under NIST SP 800-53 CM-6
(continuous enforcement, not just point-in-time compliance).

### STIG Desired Values vs Catalog

Most sysctl STIG checks want the same values as our catalog:
- `kptr_restrict=2`, `randomize_va_space=2`, `unprivileged_bpf_disabled=1`, etc.

**Notable divergence:**
- `sysctl_kernel_core_pattern`: STIG NIST citation is SC-7(10), our catalog cites SC-28 + CM-6 + AU-9.
  The STIG description focuses on "disable storing core dumps" (implying value=`|/dev/null` or similar),
  while our approach checks for a piped handler (`|...`) enabling audit and access control.
  The STIG intent may differ slightly — worth clarifying during CCE annotation work.

- `sysctl_kernel_dmesg_restrict`: STIG cites SI-11 (Error Handling) — treating dmesg leakage as
  an error-information-discipline concern. Our catalog cites SI-7 + SC-28 (information disclosure
  prevention). Both are valid; SI-11 may be worth adding to our `nist_controls` string.

- `sysctl_kernel_perf_event_paranoid`: STIG rates this as `low`; our catalog rates it `High`.
  Our assessment of KASLR leak potential via perf appears stricter than STIG guidance.

### STIG Coverage Gaps (things we check, STIG does not)

- `ModulesDisabled` (one-way latch) — not in STIG
- `UnprivUsernsClone` — not in STIG (RHEL 10 specific parameter)
- `ProtectedFifos` / `ProtectedRegular` — not in STIG
- `SuidDumpable` — not in STIG
- `Lockdown` LSM — not in STIG
- `ModuleSigEnforce` — not in STIG as a direct check
- `Mitigations` (umbrella) — not in STIG
- Per-CVE mitigation flags (SpectreV2, MDS, L1TF, RetBleed, etc.) — not in STIG
- `RandomTrustCpu` / `RandomTrustBootloader` — not in STIG
- `FipsEnabled` (direct sysctl) — not in STIG
- `NfConntrackAcct` — not in STIG
- `FirewireCoreBlacklisted` / `ThunderboltBlacklisted` — not in STIG

Our catalog is measurably stricter than STIG on kernel hardening. The gap items are defensible
under NSA RTB and NIST SP 800-53 CM-7 (attack surface reduction) even when STIG does not
mandate them.

---

## 4. CCE Annotation Priority

For immediate action in source: add CCE identifier to `nist_controls` strings in `catalog.rs`
for the 13 matched indicators. This closes the CCE cross-referencing requirement from the
`project_cce_crossref` plan.

Suggested format for `nist_controls` field addition:
```
"... ; CCE-88686-1"
```

or as a separate field if the catalog struct is extended to carry a `cce: Option<&'static str>`.
The struct-field approach is cleaner for programmatic SCAP/STIG report generation but requires
an API change. The inline string approach requires no struct change.

---

## 5. Network Sysctl Phase Assessment

The Tier 1 network sysctl candidates (11 indicators) represent a coherent "Phase 3 — Network
Hardening" block. They are pure sysctl indicators following the same pattern as existing Phase 1
entries. Each maps directly to a STIG CCE. This phase would require:
- 11 new `IndicatorId` variants
- 11 new `IndicatorDescriptor` entries in `catalog.rs`
- Integration test coverage for contradiction detection on each
- A `doc-sync` task for tech-writer (catalog reference page update)

No new dependencies or architectural patterns are required; this is additive to existing
Phase 1 infrastructure.
