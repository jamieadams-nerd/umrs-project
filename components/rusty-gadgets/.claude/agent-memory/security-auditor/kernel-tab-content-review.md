# Kernel Security Tab — Content Advisory
**Date:** 2026-03-15
**Author:** security-auditor agent
**Purpose:** Advisory input to rust-developer for Phase 7 of the TUI Enhancement Plan.
**Scope:** Kernel Security tab content, grouping, CA-7 prioritization, CPU information.

This document is advisory only. It does not modify any source file.

---

## 1. What the Codebase Already Has (Do Not Duplicate)

Before recommending new content, this is what is already wired up and available
from `umrs-platform` without new kernel reads:

| Item | Source | Already in Phase 1 Header |
|---|---|---|
| Kernel lockdown mode | `KernelLockdown` (securityfs, TPI-parsed) | Yes — `lockdown_mode` indicator |
| FIPS mode | `ProcFips` (procfs, PROC_SUPER_MAGIC) | Yes — `fips_mode` indicator |
| Module load latch | `ModuleLoadLatch` (procfs) | No |
| SELinux enforce state | `SelinuxEnforce` (selinuxfs) | Yes — `selinux` indicator |
| Active LSM list | `/sys/kernel/security/lsm` (sysfs) | Yes — `active_lsm` indicator |
| Secure boot state | stub / `IndicatorValue::Unavailable` | Yes — in header indicators |

The full posture snapshot (`PostureSnapshot::collect()`) yields all 25 signals from the
`SIGNALS` catalog across sysctl, cmdline, securityfs, and modprobe.d domains.

The Kernel Security tab MUST draw from the posture snapshot — it must not re-read the
same kernel nodes a second time using raw `File::open`. Every read must flow through
the established `SecureReader` engine per NSA RTB RAIN.

---

## 2. Recommended Tab Content — Full Item List

### Items that should appear, with their backing source and control citation

| Display Label | Kernel Source | `SignalId` or kattr | NIST Control | Priority |
|---|---|---|---|---|
| Lockdown mode | `/sys/kernel/security/lockdown` | `SignalId::Lockdown` / `KernelLockdown` | CM-7, SI-7 | CA-7 P1 |
| FIPS mode | `/proc/sys/crypto/fips_enabled` | `SignalId::FipsEnabled` / `ProcFips` | SC-13, SC-28 | CA-7 P1 |
| FIPS persistence | `/etc/system-fips`, `/etc/crypto-policies/state/current` | `FipsCrossCheck` | SC-13, CM-6 | CA-7 P1 |
| Module load latch | `/proc/sys/kernel/modules_disabled` | `SignalId::ModulesDisabled` | CM-7, SI-7 | CA-7 P1 |
| Module sig enforce | `/proc/cmdline` token `module.sig_enforce=1` | `SignalId::ModuleSigEnforce` | SI-7, CM-7 | CA-7 P1 |
| ASLR level | `/proc/sys/kernel/randomize_va_space` | `SignalId::RandomizeVaSpace` | SI-16, SC-39 | CA-7 P1 |
| Kernel ptr restrict | `/proc/sys/kernel/kptr_restrict` | `SignalId::KptrRestrict` | SI-7, SC-39 | CA-7 P1 |
| CPU mitigations | `/proc/cmdline` token `mitigations=off` absent | `SignalId::Mitigations` | SI-16, SC-39 | CA-7 P1 |
| PTI (Meltdown) | `/proc/cmdline` token `pti=off` absent | `SignalId::Pti` | SI-16 | CA-7 P2 |
| Secure boot | EFI vars / kattr stub | header indicator | SI-7, CM-7 | CA-7 P1 |
| Kernel version | `/proc/sys/kernel/osrelease` or `uname` | already in header | CM-8 | CA-7 P2 |
| YAMA ptrace scope | `/proc/sys/kernel/yama/ptrace_scope` | `SignalId::YamaPtraceScope` | SC-39, AC-6 | CA-7 P2 |
| Unprivileged BPF | `/proc/sys/kernel/unprivileged_bpf_disabled` | `SignalId::UnprivBpfDisabled` | CM-7, SC-39 | CA-7 P2 |
| Perf event paranoid | `/proc/sys/kernel/perf_event_paranoid` | `SignalId::PerfEventParanoid` | SC-39, SI-7 | CA-7 P2 |
| dmesg restrict | `/proc/sys/kernel/dmesg_restrict` | `SignalId::DmesgRestrict` | SI-7, SC-28 | CA-7 P2 |
| kexec disabled | `/proc/sys/kernel/kexec_load_disabled` | `SignalId::KexecLoadDisabled` | SI-7, CM-7 | CA-7 P1 |
| SysRq | `/proc/sys/kernel/sysrq` | `SignalId::Sysrq` | AC-3, CM-7 | CA-7 P3 |
| Unprivileged userns | `/proc/sys/kernel/unprivileged_userns_clone` | `SignalId::UnprivUsernsClone` | SC-39, CM-7 | CA-7 P2 |
| Protected symlinks | `/proc/sys/fs/protected_symlinks` | `SignalId::ProtectedSymlinks` | SI-10, SC-28 | CA-7 P3 |
| Protected hardlinks | `/proc/sys/fs/protected_hardlinks` | `SignalId::ProtectedHardlinks` | AC-6, SI-10 | CA-7 P3 |
| Protected FIFOs | `/proc/sys/fs/protected_fifos` | `SignalId::ProtectedFifos` | SI-10, CM-7 | CA-7 P3 |
| Protected regular | `/proc/sys/fs/protected_regular` | `SignalId::ProtectedRegular` | SI-10, CM-7 | CA-7 P3 |
| SUID dumpable | `/proc/sys/fs/suid_dumpable` | `SignalId::SuidDumpable` | SC-28, SI-12 | CA-7 P2 |
| Bluetooth blacklisted | modprobe.d + `/sys/module/bluetooth` | `SignalId::BluetoothBlacklisted` | CM-7, SC-39 | CA-7 P2 |
| USB storage blacklisted | modprobe.d + `/sys/module/usb_storage` | `SignalId::UsbStorageBlacklisted` | MP-7, CM-7 | CA-7 P1 |
| FireWire blacklisted | modprobe.d + `/sys/module/firewire_core` | `SignalId::FirewireCoreBlacklisted` | SI-7, CM-7 | CA-7 P2 |
| Thunderbolt blacklisted | modprobe.d + `/sys/module/thunderbolt` | `SignalId::ThunderboltBlacklisted` | SI-7, CM-7 | CA-7 P2 |
| CPU vulnerability mitigations detail | `/sys/devices/system/cpu/vulnerabilities/*` | new sysfs reads (SysfsText) | SI-16, SC-39 | CA-7 P2 |
| IMA policy active | `/sys/kernel/security/ima/policy` (securityfs) | new kattr needed | SI-7, CM-7 | CA-7 P2 |
| SELinux policy version | `/sys/fs/selinux/policyvers` | `SelinuxPolicyVers` kattr | AC-3, CM-7 | CA-7 P3 |
| SELinux MLS active | `/sys/fs/selinux/mls` | `SelinuxMls` kattr | AC-16, AC-4 | CA-7 P1 |
| Active crypto policy | `/etc/crypto-policies/state/current` | via `FipsCrossCheck` | SC-13, CM-6 | CA-7 P2 |
| Boot cmdline | `/proc/cmdline` | via `CmdlineReader` | CM-6, CM-7 | CA-7 P2 |

### Items That Are Missing from the Current Signal Catalog

The following items are NOT currently in the `SIGNALS` array or any existing kattr.
They are high-value for an assessor and should be added in a future phase:

1. **IMA (Integrity Measurement Architecture) policy presence** —
   `/sys/kernel/security/ima/policy` (securityfs). An empty policy means IMA is
   not measuring file integrity. NIST SP 800-53 SI-7.

2. **CPU vulnerability mitigation detail** — `/sys/devices/system/cpu/vulnerabilities/`
   contains per-CVE mitigation status files (e.g., `spectre_v1`, `spectre_v2`,
   `meltdown`, `mds`, `tsx_async_abort`, `itlb_multihit`, `srbds`, `mmio_stale_data`,
   `retbleed`, `spec_store_bypass`, `l1tf`). The current catalog only checks the
   umbrella `mitigations=off` cmdline flag — it does not read per-vulnerability status.
   These sysfs files require `SYSFS_MAGIC` verification. NIST SP 800-53 SI-16, SC-39.

3. **Secure boot verification** — `/sys/firmware/efi/efivars/SecureBoot-*` or
   `/proc/sys/kernel/secure_boot` (where it exists). The current plan stubs this as
   `IndicatorValue::Unavailable`. A RHEL-specific path exists:
   `/sys/firmware/efi/efivars/SecureBoot-8be4df61-93ca-11d2-aa0d-00e098032b8c`.
   EFI vars are on efivarfs (`EFIVARS_MAGIC = 0xde5e81e4`), which is not yet a
   verified magic in the kattrs engine. This needs a new kattr with efivarfs magic.
   NIST SP 800-53 SI-7, CM-7.

4. **LoadPin LSM status** — `/sys/kernel/security/loadpin/enabled`.
   LoadPin restricts kernel module loading to a trusted filesystem. On RHEL 10 this
   LSM is not enabled by default, but its absence is assessable. NIST SP 800-53 CM-7.

5. **Kernel taint flags** — `/proc/sys/kernel/tainted`. A non-zero taint value means
   the kernel has been modified in a way that changes its security guarantees
   (e.g., out-of-tree module loaded, unsigned module, GPL-incompatible module).
   This is a CA-7 signal that the kernel's integrity posture has changed since boot.
   NIST SP 800-53 SI-7.

6. **NX/SMEP/SMAP CPU feature flags** — `/proc/cpuinfo` or MSR reads.
   The presence of NX (No-Execute), SMEP (Supervisor Mode Execution Prevention),
   and SMAP (Supervisor Mode Access Prevention) determines what hardware exploit
   mitigations are available. These are CPU-level, not OS-level — they may be
   disabled by a hypervisor in a VM environment.
   NIST SP 800-53 SI-16, SC-39.

---

## 3. Proposed Groupings

This grouping structure maps to `DataRow::group_title()` calls in `build_kernel_security_rows()`.
The groups are ordered from "most operator-critical at a glance" to "detail for deep assessment".

### Group 1: BOOT INTEGRITY
Controls that govern what the kernel itself is and whether it has been tampered with.
These are the hardest to change post-boot and the highest impact if wrong.

```
BOOT INTEGRITY
 Secure Boot      : Active / Inactive / Unavailable
 Lockdown Mode    : integrity / confidentiality / none
 Module Sig Enf   : enabled / not set
 kexec disabled   : yes / no
 Kernel Taint     : 0 (clean) / <hex flags>
```

Controls: NIST SP 800-53 SI-7, CM-7; NSA RTB boot integrity.

### Group 2: CRYPTOGRAPHIC POSTURE
State of cryptographic enforcement. Assessors check this first on any DoD/government system.

```
CRYPTOGRAPHIC POSTURE
 FIPS Mode        : Active / Inactive
 FIPS Persistence : marker=present cmdline=fips=1 policy=FIPS
 Crypto Policy    : FIPS / DEFAULT / LEGACY
 Module Load Latch: locked / unlocked
```

Controls: NIST SP 800-53 SC-13, SC-28; FIPS 140-2/3; CMMC SC.L2-3.13.10.

### Group 3: KERNEL SELF-PROTECTION
Runtime kernel hardening parameters. This maps directly to the sysctl group in the
signal catalog.

```
KERNEL SELF-PROTECTION
 ASLR Level       : 2 (full) / 1 (partial) / 0 (disabled)    [HARDENED / WARNING / CRITICAL]
 KASLR / kptr     : restricted (2) / partial (1) / exposed (0)
 CPU Mitigations  : enabled / DISABLED (mitigations=off)
 PTI (Meltdown)   : enabled / disabled
 BPF Unprivileged : disabled / ENABLED
 Perf Paranoid    : 2 (restricted) / <n>
 YAMA Ptrace      : 1+ (restricted) / 0 (unrestricted)
 dmesg Restrict   : 1 (restricted) / 0 (open)
```

Controls: NIST SP 800-53 SC-39, SI-7, SI-16.

### Group 4: PROCESS ISOLATION
Namespace and inter-process attack surface controls.

```
PROCESS ISOLATION
 User Namespaces  : blocked / allowed
 SysRq Key        : disabled / restricted / enabled
 SUID Dumpable    : 0 (off) / 1 / 2
```

Controls: NIST SP 800-53 SC-39, AC-6.

### Group 5: FILESYSTEM HARDENING

```
FILESYSTEM HARDENING
 Protected Symlinks: 1 (on) / 0 (off)
 Protected Hardlinks: 1 (on) / 0 (off)
 Protected FIFOs  : 2 / 1 / 0
 Protected Regular: 2 / 1 / 0
```

Controls: NIST SP 800-53 SI-10, AC-6.

### Group 6: MODULE RESTRICTIONS
Which modules are blocked and whether the blacklist is being enforced.

```
MODULE RESTRICTIONS
 Load Latch       : locked / unlocked
 Sig Enforcement  : enforced / not enforced
 Bluetooth        : blacklisted / loaded
 USB Storage      : blacklisted / loaded
 FireWire         : blacklisted / loaded
 Thunderbolt      : blacklisted / loaded
```

Controls: NIST SP 800-53 CM-7, MP-7.

### Group 7: CPU SECURITY EXTENSIONS
Hardware-level security features available on this CPU. These are read-only from
the OS perspective — they reflect physical hardware capability.

```
CPU SECURITY EXTENSIONS
 Kernel Version   : 6.12.0-211.el10
 CPU Spectre V1   : Mitigation: usercopy/SWAPGS barriers
 CPU Spectre V2   : Mitigation: Retpoline; IBPB: conditional
 CPU Meltdown     : Not affected / Mitigation: PTI
 CPU MDS          : Not affected / Mitigation: Clear CPU buffers
 CPU TSX Async    : Not affected
 CPU Spec Store   : Mitigation: Speculative Store Bypass disabled via prctl
 NX (No-Execute)  : present / absent
 SMEP             : present / absent
 SMAP             : present / absent
```

Controls: NIST SP 800-53 SI-16, SC-39.

### Group 8: INTEGRITY MEASUREMENT
IMA/EVM status — only relevant if enabled. Show "not configured" if IMA policy is empty.

```
INTEGRITY MEASUREMENT
 IMA Policy       : active / not configured
 EVM Status       : enforcing / not configured
 Active LSM Stack : selinux / selinux,ima,evm
```

Controls: NIST SP 800-53 SI-7, CM-7.

---

## 4. CA-7 Ongoing Monitoring — Priority Ordering

CA-7 (Continuous Monitoring) requires the most security-relevant items to be checked
on the shortest cycle. For this tab, the following items are CA-7 P1 — they must be
correct on every boot and should be the first things an operator sees:

### P1 — Check Every Boot / Every Session (CRITICAL impact signals)

These map to `AssuranceImpact::Critical` in the signal catalog:

1. **FIPS Mode** — single-bit gate on all cryptographic operations.
   If this is off on a DoD system, the system is non-compliant immediately.
   `SignalId::FipsEnabled` / `ProcFips`.

2. **Lockdown Mode** — determines whether root can modify the kernel at runtime.
   Must be `integrity` or `confidentiality` on a high-assurance system.
   `SignalId::Lockdown` / `KernelLockdown`.

3. **Module Signature Enforcement** — prevents unsigned kernel modules from loading.
   `SignalId::ModuleSigEnforce`.

4. **ASLR Level** — must be `2` (full). A value of `0` or `1` is an exploit amplifier.
   `SignalId::RandomizeVaSpace`.

5. **CPU Mitigations** — `mitigations=off` on the cmdline is a critical weakening.
   An assessor must see this immediately.
   `SignalId::Mitigations`.

6. **kexec Load Disabled** — prevents runtime kernel replacement, which would defeat
   Secure Boot and lockdown guarantees.
   `SignalId::KexecLoadDisabled`.

7. **Secure Boot** — hardware root of trust. If off, all firmware-level integrity
   guarantees are void.
   (currently stubbed — needs efivarfs kattr).

8. **USB Storage Blacklisted** — data exfiltration control, mandatory on CUI systems.
   `SignalId::UsbStorageBlacklisted`.

9. **Module Load Latch** — once locked, no new modules. Shows whether the kernel's
   attack surface is frozen for this boot.
   `SignalId::ModulesDisabled`.

### P2 — Check Weekly / On Config Change (HIGH impact signals)

10. YAMA ptrace scope
11. Unprivileged BPF disabled
12. Perf event paranoid >= 2
13. Unprivileged user namespace clone disabled
14. PTI not disabled
15. FireWire blacklisted
16. Thunderbolt blacklisted
17. SUID dumpable = 0
18. Per-CVE CPU vulnerability mitigation detail
19. Bluetooth blacklisted

### P3 — Baseline Verification Only (MEDIUM impact signals)

20. dmesg restrict = 1
21. Protected symlinks = 1
22. Protected hardlinks = 1
23. Protected FIFOs = 2
24. Protected regular = 2
25. SysRq = 0
26. kptr restrict = 2 (also verifies ASLR quality)

---

## 5. CPU Information Section — Specific Recommendations

The current placeholder includes "CPU information / extensions" as a section. Here
is the specific breakdown of what matters for an assessor and why:

### 5a. Per-CVE Mitigation Status (most important)

Read from `/sys/devices/system/cpu/vulnerabilities/` (sysfs, SYSFS_MAGIC required).
Each file contains a human-readable status string. Relevant files on RHEL 10:

| File | CVE Class | Why It Matters |
|---|---|---|
| `spectre_v1` | Branch prediction side-channel | Universal — affects all x86 |
| `spectre_v2` | Indirect branch predictor | Universal — affects all x86 |
| `meltdown` | Out-of-order execution + kernel memory | Older Intel hardware |
| `spec_store_bypass` | Speculative Store Bypass | Intel/AMD, significant |
| `mds` | Microarchitectural Data Sampling | Intel MDS attack class |
| `tsx_async_abort` | Intel TSX speculation | Intel-specific |
| `l1tf` | L1 Terminal Fault | Intel-specific, severe in VMs |
| `itlb_multihit` | iTLB multi-hit (KVM) | Intel, VM environments |
| `srbds` | SRBDS sampling | Intel-specific |
| `mmio_stale_data` | MMIO stale data | Intel-specific |
| `retbleed` | Return stack buffer | AMD primary, also Intel |
| `gather_data_sampling` | AVX gather instruction | Newer Intel |

**Display recommendation**: Show each as `"Not affected"` (green), `"Mitigation: ..."` (normal),
or `"Vulnerable"` (red). Use `StyleHint::TrustGreen`, `StyleHint::Normal`, `StyleHint::TrustRed`
respectively.

**Rendering note**: This is 12+ items. Use `DataRow::TwoColumn` to put two items per row,
keeping the display compact. Alternatively, show only the ones that are NOT "Not affected"
at rest, with a count: `"3 CVE mitigations active, 9 not affected"` with a drill-down (future Phase).

### 5b. Hardware Security Feature Flags

Read from `/proc/cpuinfo` (procfs, PROC_SUPER_MAGIC). Grep the `flags:` line for:

| CPU Flag | What It Means | NIST Control |
|---|---|---|
| `nx` | NX bit (No-Execute, W^X enforcement) | SI-16, SC-39 |
| `smep` | Supervisor Mode Execution Prevention | SC-39 |
| `smap` | Supervisor Mode Access Prevention | SC-39 |
| `pti` | Page Table Isolation (kernel visible) | SI-16 |
| `ibpb` | Indirect Branch Prediction Barrier | SC-39 |
| `stibp` | Single Thread Indirect Branch Predictors | SC-39 |
| `md_clear` | MDS mitigation (VERW/MD_CLEAR) | SC-39 |
| `aes` | AES-NI (hardware AES acceleration) | SC-13 |
| `sha_ni` | SHA-NI (hardware SHA acceleration) | SC-13 |
| `rdrand` | RDRAND instruction (hardware RNG) | SC-12 |
| `rdseed` | RDSEED instruction (hardware entropy) | SC-12 |
| `avx`, `avx2` | AVX/AVX2 (relevant to gather_data_sampling) | SI-16 |

**Display recommendation**: Group as `CPU SELF-PROTECTION` (nx, smep, smap, pti) and
`CPU CRYPTOGRAPHIC ACCELERATION` (aes, sha_ni, rdrand, rdseed). Present/Absent rather
than raw flag strings.

### 5c. Virtualization / Hypervisor Detection

Read from `/proc/cpuinfo` `hypervisor` flag or `/sys/hypervisor/type`.

| Item | Significance |
|---|---|
| Running in VM | L1TF and similar side-channels are more exploitable; cross-VM timing attacks possible |
| Hypervisor type | Xen/KVM/VMware have different IOMMU and memory isolation guarantees |

Display: `Bare metal / KVM / Xen / VMware / unknown`. This directly informs the
risk profile for L1TF and MMIO stale data. NIST SP 800-53 SC-39.

---

## 6. Items NOT in Current Signal Catalog — Gaps for Developer Follow-Up

These are assessment-relevant kernel security properties with no current backing signal.
They require new kattr types or new signal descriptors before the tab can display
live data for them. Developer should decide which to address:

| Item | Kernel Path | Obstacle | Controls |
|---|---|---|---|
| Secure boot | efivarfs `SecureBoot-*` | No efivarfs magic in kattrs engine | SI-7, CM-7 |
| IMA policy | `/sys/kernel/security/ima/policy` | securityfs, but no kattr defined | SI-7 |
| Kernel taint | `/proc/sys/kernel/tainted` | New procfs signal needed in catalog | SI-7 |
| CPU vuln files | `/sys/devices/system/cpu/vulnerabilities/*` | Dynamic paths, 12+ files | SI-16, SC-39 |
| NX/SMEP/SMAP flags | `/proc/cpuinfo` flags field | ProcfsText + grep logic needed | SI-16, SC-39 |
| LoadPin LSM | `/sys/kernel/security/loadpin/enabled` | New securityfs kattr needed | CM-7 |
| Hypervisor type | `/proc/cpuinfo` or `/sys/hypervisor/type` | New read path needed | SC-39 |
| IOMMU enabled | `/sys/kernel/iommu_groups/` or cmdline `iommu=` | Needs new reader | SC-39 |
| TPM presence | `/sys/class/tpm/tpm0/` or `/dev/tpm0` | Needs new sysfs reader | SC-28, SI-7 |

---

## 7. What the Placeholder Sections Should Show (Phase 7 Now)

For the immediate Phase 7 implementation where a `"(not yet probed)"` placeholder is
acceptable, here is the recommended minimal set that uses already-available data:

```
BOOT INTEGRITY
 Secure Boot      : [from SecurityIndicators::secure_boot]
 Lockdown Mode    : [from SecurityIndicators::lockdown_mode]
 Module Sig Enf   : (not yet probed)
 kexec Disabled   : (not yet probed)

CRYPTOGRAPHIC POSTURE
 FIPS Mode        : [from SecurityIndicators::fips_mode]
 Crypto Policy    : (not yet probed)

KERNEL SELF-PROTECTION
 ASLR Level       : (not yet probed)
 CPU Mitigations  : (not yet probed)
 kptr Restrict    : (not yet probed)

MODULE RESTRICTIONS
 Load Latch       : (not yet probed)
 USB Storage      : (not yet probed)
 Bluetooth        : (not yet probed)

CPU SECURITY
 Kernel Version   : [from header — uname result]
 CPU Architecture : (not yet probed)
 CPU Mitigations  : (not yet probed)
```

Items marked `[from SecurityIndicators::*]` should reuse the already-populated header
indicator values rather than issuing a second read. This is the correct architectural
pattern: one read at startup, display in multiple places.

---

## 8. Two-Column Layout Recommendations

For Phase 7's placeholder layout, two-column pairs that read well together:

```
[Secure Boot : Active]        [Lockdown : integrity]
[FIPS Mode   : Active]        [Crypto Policy : FIPS]
[ASLR        : 2 (full)]      [kptr Restrict : 2]
[Module Latch: locked]        [Sig Enforce   : on]
[USB Storage : blacklisted]   [Bluetooth     : blacklisted]
```

Boot integrity and cryptographic posture pair well. ASLR pairs with kptr (both
address-space leak controls). Module controls pair together.

---

## 9. Display Value Policy

**Show compliance status, not raw kernel integers.** An assessor reading `"2"` for
`randomize_va_space` must know that means "full ASLR." The display should provide
interpretation:

- Integer sysctl values should have a label: `"2 (full)"`, `"1 (partial)"`, `"0 (disabled)"`
- Boolean values: `"enabled"` / `"disabled"` — never `"true"` / `"false"`
- Absent/locked values: `"locked (latch set)"` for `modules_disabled = 1`
- Cmdline absent checks: `"enabled"` when `mitigations=off` is absent, `"DISABLED"` when present

Style hints:
- Value meets hardened baseline: `StyleHint::TrustGreen`
- Value is partially compliant or unknown: `StyleHint::TrustYellow`
- Value is non-compliant: `StyleHint::TrustRed`
- Value is `"(not yet probed)"`: `StyleHint::Dim`

---

## 10. Information the Assessor Needs That Is NOT Kernel-Derived

These items require documentation or configuration review, not kernel reads.
They are out of scope for this tab but belong in a future "Policy Configuration" tab:

- SELinux policy name / version (policy file hash, not kernel version)
- SCAP/OpenSCAP baseline compliance scan results
- `aide` (Advanced Intrusion Detection Environment) database integrity status
- sudo policy audit (`/etc/sudoers` least-privilege verification)
- PAM configuration (auth, password quality)
- systemd unit hardening (`CapabilityBoundingSet`, `SecureBits`, `ProtectSystem`)

These cannot be reliably read through kernel pseudo-filesystems.

---

## Document Control

This advisory is referenced from:
- `.claude/agent-memory/security-auditor/MEMORY.md`
- Phase 7 of `.claude/plans/tui-enhancement-plan.md`

The rust-developer agent should use this as input when implementing
`build_kernel_security_rows()` and the subsequent Phase 8+ content population work.

Questions for Jamie before implementation:
1. Should Phase 7 reuse `PostureSnapshot::collect()` to populate live values, or remain
   fully placeholder until a dedicated Phase 7.5 / Phase 9?
2. The CPU vulnerability files (12+ items) — should they appear as a scrollable sub-list
   or be collapsed to a summary count?
3. Should `"(not yet probed)"` items be hidden entirely or shown with Dim style? Hidden
   reduces noise but obscures the fact that the tab is incomplete.
