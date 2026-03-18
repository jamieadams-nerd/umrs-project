# SMT / Hyperthreading — Platform Topology as a Security Property

**Category:** 14 — Platform Topology & Metadata
**Feature #:** 52
**Phase:** 1G
**Date:** 2026-03-18
**Classification:** Important (security cross-cutting property)

---

## 23-Column Matrix Profile

| # | Column | Value |
|---|--------|-------|
| 1 | Feature name | SMT — Simultaneous Multithreading (Intel: Hyperthreading / HT; AMD: SMT) |
| 2 | Vendor | Both (Intel as "Hyperthreading," AMD as "SMT") |
| 3 | Category | Platform Topology & Metadata (Category 14) — cross-cutting security property |
| 4 | Purpose | SMT allows two (or more) independent hardware threads to share a single physical CPU core's execution resources simultaneously. On Intel, this is called Hyperthreading (HT). On AMD, it is called SMT. SMT is not a traditional CPU "feature" in the capability sense — it is a platform topology property that affects security posture by determining whether SMT sibling threads share microarchitectural state. |
| 5 | Example instructions | None — SMT is topology, not an instruction set. Relevant instructions are those whose behavior changes under SMT: STIBP-governed indirect branches, VERW (used by MDS mitigation), L1D cache operations. |
| 6 | CPUID leaf/subleaf/bit | CPUID leaf 01H, EDX bit 28 (`HTT` — Hyper-Threading Technology). Reports whether the processor supports more than one logical processor per physical package. CPUID leaf 0BH (Extended Topology) enumerates core and thread topology. Note: `HTT` bit being set does not mean HT is enabled — it indicates the capability. |
| 7 | Linux `/proc/cpuinfo` flag | No dedicated flag for SMT state. `ht` flag in `/proc/cpuinfo` indicates HT capability (not current state). `siblings` field reports logical CPUs per physical package. `cpu cores` field reports physical cores. If `siblings > cpu cores`, HT/SMT is active. |
| 8 | Linux detection — authoritative path | **Primary:** `/sys/devices/system/cpu/smt/active` — `1` if SMT is active (threads online), `0` if SMT is inactive. `/sys/devices/system/cpu/smt/control` — current control state: `on`, `off`, `forceoff`, `notsupported`, `notimplemented`. **Secondary:** `/sys/devices/system/cpu/cpu<n>/topology/thread_siblings_list` — lists sibling threads for core N. If N has siblings, SMT is present and those threads are siblings on the same core. |
| 9 | Minimum CPU generations | Intel: Pentium 4 Northwood (2002) introduced HT. Disabled in Intel Core 2 era. Re-introduced in Nehalem (2008) and present in all subsequent Intel CPUs with the exception of some low-power SKUs. AMD: Zen (2017) introduced modern AMD SMT. |
| 10 | Security benefit | **Disabling SMT improves isolation.** SMT siblings share L1D cache, L1I cache, TLB, branch predictor, and store buffers. These shared resources are the attack surface for cross-thread side-channel attacks (MDS, L1TF, STIBP bypass). Disabling SMT eliminates the co-scheduling attack surface but is not required if the relevant mitigations are active. |
| 11 | Performance benefit | **Enabling SMT improves throughput.** SMT typically provides 15–30% additional throughput on workloads that do not saturate all execution units. Disabling SMT effectively halves the logical CPU count, which can significantly reduce performance on compute-intensive workloads. |
| 12 | Assurance caveats | (1) SMT enabled does not mean the system is insecure — MDS, L1TF, and STIBP mitigations address the specific cross-thread attacks when SMT is active. Disabling SMT is not required if mitigations are confirmed active. (2) SMT disabled reduces performance. On high-assurance systems where the threat model includes cross-tenant isolation (multi-VM, multi-user co-hosting), SMT disable may be warranted. (3) KVM+SMT: on hypervisor hosts, `kvm.nosmt_on_offline` kernel parameter can online only one sibling per core for VMs. (4) `nosmt` kernel parameter disables SMT but can be overridden by `echo on > /sys/devices/system/cpu/smt/control` at runtime — use `nosmt=force` to make it irrevocable without reboot. (5) The `mitigations=auto,nosmt` parameter activates SMT disable only for vulnerability classes where SMT significantly worsens the threat (L1TF with untrusted VMs); it is not a blanket SMT disable. |
| 13 | Virtualization behavior | In VMs, the guest sees logical CPUs. KVM does not expose SMT topology to guests by default — the guest cannot tell whether it is co-scheduled with a sibling thread on the same physical core. On hypervisor hosts running untrusted VMs with SMT enabled, L1TF and MDS mitigations must account for cross-SMT-sibling access to L1 cache. |
| 14 | Firmware / BIOS / microcode dependency | `{ bios_enable_required: false, microcode_required: false }` for detection. BIOS can enable/disable SMT at boot. Microcode is not required for SMT itself; microcode IS required for STIBP, MDS mitigation (VERW), and L1D flush to function correctly on SMT systems. |
| 15 | Audit-card relevance | **Important** — SMT state is a prerequisite input for evaluating several Critical/Defensive mitigations. The audit finding is not "SMT enabled = finding" but rather "SMT enabled + MDS/L1TF/STIBP mitigations not confirmed = finding." On systems with untrusted co-tenants (multi-VM hypervisors, multi-user environments), SMT+missing-mitigations is a HIGH finding. |
| 16 | Recommended disposition when unused | For single-tenant dedicated-use systems: leave SMT enabled; confirm MDS, L1TF, STIBP mitigations active. For hypervisors hosting untrusted VMs (e.g., cloud provider, multi-tenant): evaluate `nosmt` or `mitigations=auto,nosmt` based on performance vs isolation trade-off. For MLS systems with cross-domain co-scheduling risk: consult the specific threat model; `nosmt=force` is the highest-assurance option. |
| 17 | Software utilization detection method | `/sys/devices/system/cpu/smt/active` — current SMT active state. `/proc/cmdline` for `nosmt`, `nosmt=force`, `mitigations=auto,nosmt`. `/sys/devices/system/cpu/smt/control` for current control value. |
| 18 | FIPS utilization requirement | N/A — SMT is not a cryptographic feature. |
| 19 | Active mitigation status path | SMT interacts with multiple vulnerability mitigations: `/sys/devices/system/cpu/vulnerabilities/mds` — MDS mitigation status (SMT-dependent). `/sys/devices/system/cpu/vulnerabilities/l1tf` — L1TF mitigation status (SMT-dependent for full mitigation). `/sys/devices/system/cpu/vulnerabilities/spectre_v2` — STIBP status (SMT-relevant). `/sys/devices/system/cpu/vulnerabilities/srbds` — SRBDS status (SMT-relevant). |
| 20 | Feature accessible vs advertised | BIOS can disable SMT, in which case `HTT` CPUID bit may still be set (capability) but SMT is inactive. The sysfs control interface is the authoritative current state. |
| 21 | Guest-vs-host discrepancy risk | **HIGH** — Guests cannot observe host SMT topology or co-scheduling decisions. A guest cannot determine whether it is sharing a physical core with another guest's vCPU. |
| 22 | Notes | SMT is not a vulnerability — it is a design trade-off. The vulnerabilities (MDS, L1TF, Spectre v2/STIBP) exploit the shared microarchitectural resources that SMT creates. Whether to disable SMT or rely on mitigations is a deployment-specific decision. On RHEL 10 with current microcode and kernel, the mitigations are active by default for SMT-enabled systems. The choice of `nosmt` is a performance vs isolation policy decision, not a correctness issue. |
| 23 | Sources | Linux kernel admin-guide/hw-vuln/mds.rst; admin-guide/hw-vuln/l1tf.rst; Red Hat SMT solution article; Flatcar Container Linux SMT disabling docs; Linux /sys/devices/system/cpu/smt ABI |

---

## Description

Simultaneous Multithreading (SMT) allows a single physical CPU core to appear as two or more logical CPUs to the operating system. The two "hardware threads" or "sibling threads" share the core's physical execution units — execution pipelines, ALUs, L1 data cache, L1 instruction cache, TLBs, branch predictors, and store buffers — but each has its own architectural state (register file, program counter, flags).

Intel's implementation is called Hyperthreading (HT). AMD's is called SMT. Functionally they are equivalent for security analysis purposes, though implementation details differ.

The OS scheduler treats each logical CPU as an independent scheduling target. On a 4-core CPU with HT enabled, the OS sees 8 logical CPUs. Two processes scheduled to "CPU 0" and "CPU 1" that are HT siblings on the same physical core are co-executing on the same physical hardware — sharing microarchitectural resources.

---

## Why Security Engineers Care

### Shared microarchitectural resources create cross-thread attack surfaces

The following resources are shared between SMT siblings on the same physical core:

| Shared Resource | Attack Exploiting It |
|----------------|---------------------|
| L1 data cache | L1TF (Foreshadow-OS), MDS MLPDS |
| Store buffer | MDS (MSBDS — Microarchitectural Store Buffer Data Sampling) |
| Load buffer | MDS (MFBDS — Microarchitectural Fill Buffer Data Sampling) |
| Branch predictor | Spectre v2 (STIBP mitigates on SMT siblings) |
| Line fill buffer | MDS (MDSUM) |
| Execution port contention | Portsmash timing side-channel |

If two processes at different trust levels (e.g., a CUI process and an untrusted process, or two VMs with different classification levels) are co-scheduled as SMT siblings, the sibling can observe microarchitectural state from the co-scheduled sibling.

### The mitigation landscape for SMT-enabled systems

The kernel's response to SMT-related vulnerabilities:

**MDS (CVE-2018-12126, CVE-2018-12127, CVE-2018-12130, CVE-2019-11091):**
- Mitigation: VERW instruction execution at every kernel-to-user transition and hypervisor VM entry
- Detection: `/sys/devices/system/cpu/vulnerabilities/mds`
- SMT impact: Full mitigation requires the VERW flush to clear shared buffers between sibling transitions. Without SMT, the timing window does not exist.

**L1TF (CVE-2018-3620, CVE-2018-3646):**
- Mitigation for VMs: L1D cache flush on every VM entry (expensive), or disable SMT on hypervisor hosts running untrusted VMs
- Detection: `/sys/devices/system/cpu/vulnerabilities/l1tf`
- SMT impact: The VM variant (CVE-2018-3646) is specifically a cross-SMT-sibling attack in hypervisor context

**Spectre v2 / STIBP (CVE-2017-5715):**
- Mitigation: eIBRS or IBRS + STIBP for cross-sibling branch prediction isolation
- STIBP is specifically a cross-SMT sibling mitigation: it prevents one SMT sibling from influencing the other's indirect branch predictor
- Detection: `/sys/devices/system/cpu/vulnerabilities/spectre_v2`

### SMT and MLS

On MLS systems where processes at different sensitivity levels may execute on the same host, SMT introduces cross-level microarchitectural exposure even if MLS policy isolates them at the OS level. STIBP prevents cross-sibling branch predictor contamination, but store buffer and load buffer attacks (MDS) depend on the hardware generation's microcode and the VERW mitigation.

For the highest-assurance MLS environments, disabling SMT eliminates the shared-hardware attack surface entirely. This is the `nosmt=force` posture.

---

## Linux sysfs SMT Interface

The kernel's SMT control interface:

```
/sys/devices/system/cpu/smt/active
    0 = SMT is offline (disabled, or hardware does not support it)
    1 = SMT is active (at least one core has sibling threads online)

/sys/devices/system/cpu/smt/control
    on           = SMT is enabled, siblings are online
    off          = SMT is disabled at runtime (siblings offline but can be re-enabled)
    forceoff     = SMT was disabled at boot with nosmt=force (cannot be re-enabled without reboot)
    notsupported = Hardware does not support SMT
    notimplemented = Kernel was built without SMT support
```

Runtime SMT disable:

```bash
# Disable SMT at runtime (reversible):
echo off > /sys/devices/system/cpu/smt/control

# Re-enable:
echo on > /sys/devices/system/cpu/smt/control
```

Boot-time SMT disable (permanent for session):

```
# Add to kernel cmdline:
nosmt          # disables SMT, can be re-enabled at runtime
nosmt=force    # disables SMT, cannot be re-enabled without reboot
mitigations=auto,nosmt  # disables SMT only when required by active mitigations (L1TF)
```

---

## Interaction Matrix with Mitigation Signals

| Vulnerability | SMT Active | SMT Disabled | Mitigation Status Path |
|---|---|---|---|
| MDS | Mitigated via VERW (hardware generation dependent) | Full isolation — VERW overhead eliminated | `/sys/devices/system/cpu/vulnerabilities/mds` |
| L1TF (OS) | Mitigated via PTE inversion (CVE-2018-3620) | Same mitigation applies | `/sys/devices/system/cpu/vulnerabilities/l1tf` |
| L1TF (VMM) | Requires L1D flush on VM entry (expensive) or SMT disable | Fully mitigated without L1D flush | Same path |
| Spectre v2 | STIBP provides per-sibling isolation; eIBRS preferred | STIBP irrelevant (no siblings) | `/sys/devices/system/cpu/vulnerabilities/spectre_v2` |
| SRBDS | Mitigated via SRBDS_MSR or VERW on context switch | Fully mitigated | `/sys/devices/system/cpu/vulnerabilities/srbds` |
| TAA | Mitigated via VERW (TSX disabled or MD_CLEAR) | Timing window eliminated | `/sys/devices/system/cpu/vulnerabilities/tsx_async_abort` |

---

## Audit-Card Logic

The correct audit-card logic for SMT is not "SMT enabled = finding." It is:

```
if smt_active:
    verify(mds_mitigation == "Mitigation: Clear CPU buffers; SMT vulnerable" is NOT acceptable)
    verify(l1tf_mitigation includes "SMT disabled" OR "conditional cache flushes" (if VMs present))
    verify(spectre_v2 includes "STIBP" or "eIBRS")
    if untrusted_vms_present AND l1tf_mitigation != "SMT disabled":
        finding = HIGH ("SMT enabled with untrusted VMs requires L1D flush or SMT disable for full L1TF mitigation")
else:
    record("SMT disabled — cross-sibling attack surface eliminated")
    check_performance_impact_acknowledged = True
```

---

## CVE Summary

| CVE | Year | CVSS | Feature | Mitigation |
|-----|------|------|---------|------------|
| CVE-2018-3620 | 2018 | 5.6 | L1TF OS — speculative EPT L1 cache read | PTE inversion |
| CVE-2018-3646 | 2018 | 5.6 | L1TF VMM — cross-SMT-sibling speculative L1 read in hypervisor context | L1D flush on VM entry or nosmt |
| CVE-2018-12126 | 2019 | 5.6 | MSBDS — MDS store buffer sampling | VERW + SMT consideration |
| CVE-2018-12127 | 2019 | 5.6 | MLPDS — MDS load port sampling | VERW |
| CVE-2018-12130 | 2019 | 6.5 | MFBDS / ZombieLoad — line fill buffer | VERW |
| CVE-2019-11091 | 2019 | 3.8 | MDSUM — uncacheable memory | VERW |
| CVE-2019-11184 | 2019 | — | Portsmash — SMT execution port timing | SMT disable |

---

## Sources

- [MDS — Microarchitectural Data Sampling — Linux Kernel docs](https://docs.kernel.org/admin-guide/hw-vuln/mds.html)
- [L1TF — L1 Terminal Fault — Linux Kernel docs](https://docs.kernel.org/admin-guide/hw-vuln/l1tf.html)
- [Simultaneous Multithreading in Red Hat Enterprise Linux](https://access.redhat.com/solutions/rhel-smt)
- [Flatcar Container Linux — Disabling SMT](https://www.flatcar.org/docs/latest/setup/security/disabling-smt/)
- [Oracle Linux — Disable SMT to Prevent CPU Security Issues](https://docs.oracle.com/en/operating-systems/oracle-linux/cockpit/disable_smt.html)
- [Gentoo Project:Security — MDS/ZombieLoad](https://wiki.gentoo.org/wiki/Project:Security/Vulnerabilities/MDS_-_Microarchitectural_Data_Sampling_aka_ZombieLoad)
- [NVD: CVE-2018-3646](https://nvd.nist.gov/vuln/detail/CVE-2018-3646)
- [NVD: CVE-2018-12130](https://nvd.nist.gov/vuln/detail/CVE-2018-12130)
