# VMX / SVM — Intel VT-x and AMD-V Virtualization Extensions

**Category:** 12 — Virtualization Security
**Phase:** 1G
**Date:** 2026-03-18

---

## 23-Column Matrix Profile

### VMX (Intel VT-x)

| # | Column | Value |
|---|--------|-------|
| 1 | Feature name | VMX — Virtual Machine Extensions (Intel VT-x) |
| 2 | Vendor | Intel |
| 3 | Category | Virtualization Security (Category 12) |
| 4 | Purpose | Hardware extension enabling hardware-assisted virtualization; adds VMXON/VMXOFF/VMLAUNCH/VMRESUME/VMREAD/VMWRITE instructions and the Virtual Machine Control Structure (VMCS). |
| 5 | Example instructions | VMXON, VMXOFF, VMLAUNCH, VMRESUME, VMPTRLD, VMREAD, VMWRITE, VMCALL |
| 6 | CPUID leaf/subleaf/bit | Leaf 01H, ECX bit 5 (`CPUID.01H:ECX[5]`) |
| 7 | Linux `/proc/cpuinfo` flag | `vmx` |
| 8 | Linux detection — authoritative path | `/proc/cpuinfo` `flags` line for `vmx`; kernel module `kvm_intel` loaded confirms active use. `/sys/module/kvm_intel/` presence indicates KVM use of VMX. |
| 9 | Minimum CPU generations | Intel Pentium 4 Model 662/672 (Nov 2005 Prescott). Effectively all post-2006 Intel processors. |
| 10 | Security benefit | Guest OS operates in VMX non-root mode — hardware prevents guest from escaping to host state without hypervisor mediation. Provides hardware enforcement of VM isolation. |
| 11 | Performance benefit | Hardware-accelerated VM entry/exit reduces hypervisor overhead vs. software-only trap-and-emulate approaches. |
| 12 | Assurance caveats | VMX itself does not guarantee VM isolation security — isolation depends entirely on the hypervisor implementation. A buggy hypervisor using VMX is no safer than software virtualization from a correctness standpoint. VMX CPUID bit exposed to guests can be used by malware for hypervisor detection and anti-analysis evasion. Nested VMX (VMX-in-VMX) adds substantial attack surface if enabled unnecessarily. |
| 13 | Virtualization behavior | By default KVM/VMware expose `vmx` to guests only if nested virtualization is explicitly enabled. KVM: `nested=1` parameter on kvm_intel module. Without nested, `vmx` bit is masked from guest CPUID. Guest cannot distinguish hardware VMX from emulated VMX without attestation. |
| 14 | Firmware / BIOS / microcode dependency | `{ bios_enable_required: true, microcode_required: false }` — Many OEM BIOSes disable VT-x by default; BIOS must enable it. Once enabled, no special microcode rev is required for the base VMX feature (specific sub-features like VMCS shadowing may require updates). |
| 15 | Audit-card relevance | **Informational** — VMX presence indicates the platform can host VMs. Relevant to isolation posture only when VMs are actually in use. The security of VM isolation is a hypervisor question, not a CPU question. |
| 16 | Recommended disposition when unused | If the system does not run VMs and will never serve as a hypervisor host: **Disable in BIOS** — eliminates VMX as an attack vector for privilege escalation exploits that abuse VMM entry points. If KVM is used: leave enabled; verify `kvm_intel` module loaded correctly. |
| 17 | Software utilization detection method | `lsmod | grep kvm_intel` — if loaded, VMX is in active use. `/sys/module/kvm_intel/` directory exists when module is active. |
| 18 | FIPS utilization requirement | N/A — VMX is not a cryptographic feature. |
| 19 | Active mitigation status path | N/A — VMX itself is not a vulnerability mitigation. Related vulnerability mitigations that depend on VMX: `/sys/devices/system/cpu/vulnerabilities/itlb_multihit` (KVM-specific Intel VMX vulnerability). |
| 20 | Feature accessible vs advertised | BIOS can disable VMX entirely even when CPUID bit would otherwise be set. Kernel can also mask VMX from userspace `cpuinfo` if the feature is disabled via boot parameter. |
| 21 | Guest-vs-host discrepancy risk | **HIGH for nested VMX** — Host with nested=1 exposes `vmx` to guest; host without nested masks it. Guest sees a different CPUID picture than the host silicon. |
| 22 | Notes | VMX/SVM presence is often checked by malware to detect sandbox/analysis environments. In security-sensitive deployments, masking VMX from guest CPUID (default KVM behavior) is the correct posture. |
| 23 | Sources | Intel SDM Vol 3C Ch 23 (VMX architecture); Linux kernel `arch/x86/kvm/vmx/`; KVM CPUID docs `docs.kernel.org/virt/kvm/x86/cpuid.html` |

---

### SVM (AMD-V)

| # | Column | Value |
|---|--------|-------|
| 1 | Feature name | SVM — Secure Virtual Machine (AMD-V, AMD Virtualization) |
| 2 | Vendor | AMD |
| 3 | Category | Virtualization Security (Category 12) |
| 4 | Purpose | AMD's hardware virtualization extension. Introduces VMRUN/VMLOAD/VMSAVE instructions and the Virtual Machine Control Block (VMCB). Functionally analogous to Intel VMX. |
| 5 | Example instructions | VMRUN, VMLOAD, VMSAVE, VMMCALL, CLGI, STGI, SKINIT, INVLPGA |
| 6 | CPUID leaf/subleaf/bit | Extended leaf 80000001H, ECX bit 2 (`CPUID.80000001H:ECX[2]`). Also: CPUID 8000000AH returns SVM feature flags (VMCB clean bits, nested paging, AVIC, etc.). |
| 7 | Linux `/proc/cpuinfo` flag | `svm` |
| 8 | Linux detection — authoritative path | `/proc/cpuinfo` `flags` line for `svm`; `/sys/module/kvm_amd/` presence confirms active use by KVM. |
| 9 | Minimum CPU generations | AMD Athlon 64 X2 (Pacifica, 2006). All modern AMD processors since Zen architecture have SVM. |
| 10 | Security benefit | Same guest-isolation model as VMX — hardware prevents guest escape without hypervisor mediation. SKINIT instruction provides a measured launch capability similar in concept to Intel TXT. |
| 11 | Performance benefit | Eliminates software overhead of trap-and-emulate for privilege-sensitive instructions. NPT (Nested Page Tables) further reduces VM exit frequency. |
| 12 | Assurance caveats | Same hypervisor-correctness caveat as VMX. AMD's SKINIT feature is less widely deployed and audited than Intel TXT. VMCB (control block) integrity is a security-critical data structure — vulnerabilities in hypervisor VMCB handling are a known attack surface. |
| 13 | Virtualization behavior | KVM requires `nested=1` on `kvm_amd` module to expose `svm` flag to guests. Default: masked. |
| 14 | Firmware / BIOS / microcode dependency | `{ bios_enable_required: true, microcode_required: false }` — AMD-V may be disabled in BIOS by default on consumer systems. |
| 15 | Audit-card relevance | **Informational** — same reasoning as VMX. |
| 16 | Recommended disposition when unused | Disable in BIOS if no VM workloads. If KVM/QEMU in use: leave enabled; verify `kvm_amd` module loaded. |
| 17 | Software utilization detection method | `lsmod | grep kvm_amd` — loaded when SVM is in active use. |
| 18 | FIPS utilization requirement | N/A |
| 19 | Active mitigation status path | N/A — but AMD SVM-related vulnerabilities (e.g., CacheWarp CVE-2023-20592 affecting SEV-ES) are tracked separately. |
| 20 | Feature accessible vs advertised | BIOS-gated. CPUID may show `svm` but VMRUN will #UD if BIOS disabled SVM. |
| 21 | Guest-vs-host discrepancy risk | **HIGH for nested SVM** — same as VMX nested concern. |
| 22 | Notes | SKINIT provides the basis for a measured launch flow, but this is distinct from AMD SEV/SEV-SNP which provides guest memory encryption. Verify BIOS has SVM enabled before deploying KVM. |
| 23 | Sources | AMD APM Vol 2 Ch 15 (SVM architecture); Linux `arch/x86/kvm/svm/`; AMD CPUID leaf 8000000AH SVM Feature Identifiers |

---

## Description

VMX (Intel) and SVM (AMD) are hardware virtualization extensions that allow the CPU to run multiple OS instances simultaneously with hardware-enforced isolation. They introduce a two-mode model: VMX root mode (where the hypervisor runs) and VMX non-root mode (where guests run). The hardware enforces that transitions between these modes go through the hypervisor, enabling VM isolation without software trap-and-emulate overhead.

### Key architectural difference from software virtualization

Without VMX/SVM, hypervisors must trap every privileged instruction from guests using software techniques (binary translation or paravirtualization). With VMX/SVM, the hardware handles this at the CPU level — guests run at near-native speed, and the hypervisor only intervenes when a VM exit occurs.

---

## Why Security Engineers Care

**1. VM isolation depends on VMX/SVM correctness, not just presence.**
VMX/SVM presence does not guarantee isolation. Hypervisor bugs in VMCS/VMCB handling can allow guest escape. The CPU provides the mechanism; the hypervisor must use it correctly.

**2. VMX/SVM presence exposes new kernel attack surfaces.**
`/dev/kvm` and the KVM kernel module extend the kernel's attack surface. Hypervisor privilege escalation CVEs (e.g., CVE-2021-3653 AMD SVM, CVE-2021-26932 KVM) exploit bugs in VMX/SVM handling code, not the CPU instructions themselves.

**3. Nested virtualization dramatically increases attack surface.**
Nested VMX/SVM (VM-in-VM) requires the hypervisor to emulate VMX/SVM for the inner guest. This emulation layer is complex and has a poor CVE history. On non-hypervisor systems, nested should be disabled.

**4. Anti-VM evasion by malware.**
Malware routinely probes `vmx`/`svm` in CPUID to detect sandbox analysis environments and changes behavior accordingly. On analysis systems, masking these flags aids malware analysis. On production systems, this is irrelevant.

---

## CVE Summary

| CVE | Year | CVSS | Impact | Fix |
|-----|------|------|--------|-----|
| CVE-2021-3653 | 2021 | 8.8 | AMD SVM: missing check in nested SVM MSR handling allowed guest-to-host privilege escalation via crafted VMCB | Kernel patch to kvm_amd |
| CVE-2021-26932 | 2021 | 5.5 | Linux KVM: incorrect nested SVM/VMX handling causing host crash | Kernel patch |
| CVE-2020-12351 | 2020 | 8.3 | BleedingTooth — not VMX/SVM directly; shows adjacent attack surface in KVM infrastructure | Kernel patch |
| CVE-2019-3016 | 2020 | 6.5 | KVM: information leak via speculative execution in VMX exit path | Kernel patch + microcode |
| CVE-2018-12207 | 2019 | 6.5 | Intel: Machine Check Error on page size change (iTLB Multihit) — KVM-specific, affects VMX hosts | Kernel patch (KVM page size restriction) |

**Key observation:** CVEs accumulate in the KVM hypervisor code that uses VMX/SVM, not in the CPU instructions themselves. The attack surface is the software layer, not the hardware extension.

---

## Compliance-Specific Requirements

- **NIST SP 800-53 SC-39 (Process Isolation):** Hypervisors using VMX/SVM must maintain isolation between VMs. An audit finding would note if nested virtualization is enabled unnecessarily (expanded attack surface).
- **NIST SP 800-53 SI-7 (Software, Firmware, Information Integrity):** Hypervisor integrity is a prerequisite for trusting VM isolation claims.
- **CMMC SC.L2-3.13.1 (Boundary Protection):** VM isolation using VMX/SVM is a boundary protection mechanism for CUI-bearing systems.
- **DoD STIG relevance:** RHEL 10 STIGs do not directly control VMX/SVM flags, but KVM configuration (e.g., nested enablement, `/dev/kvm` permissions) is subject to STIG-level review.

---

## Virtualization Confidence

**Can a guest verify hardware-backed VMX/SVM?**

No — a guest cannot reliably distinguish hardware VMX from a hypervisor emulating VMX behavior. The `vmx`/`svm` CPUID bit in the guest is entirely under hypervisor control. The hypervisor can:
- Expose the real hardware bit (passthrough)
- Mask it (most KVM defaults)
- Emulate a fake `vmx` bit in pure software (QEMU TCG mode)

There is no guest-accessible mechanism to prove hardware backing without a TEE attestation chain (TDX, SEV-SNP).

---

## UMRS Posture Signal Connection

VMX/SVM presence is **Informational** — detecting them is useful for platform inventory, not for deriving a security finding. Relevant signals for UMRS would be:

- **Layer 1:** Is `vmx` or `svm` present? (inventory)
- **Layer 2:** Is `kvm_intel` or `kvm_amd` kernel module loaded? (utilization)
- **Finding:** Nested virtualization enabled on a non-hypervisor host (configuration finding)
- **Finding:** `/dev/kvm` world-accessible (permission finding — outside CPU scope)

---

## ARM/AArch64 Equivalent

ARM does not use the VMX/SVM model. ARM virtualization uses:
- **EL2 (Exception Level 2):** The hypervisor privilege level. VMs run at EL0/EL1; the hypervisor intercepts at EL2.
- **Detection:** No ARM equivalent of `vmx`/`svm` in `/proc/cpuinfo`. Virtualization capability is always present on server-class AArch64 CPUs (Cortex-A72+, Neoverse N1+). Kernel Hypervisor mode: `CONFIG_KVM` with ARM architecture.
- **ARM equivalent of nested VMX:** Not commonly available on current AArch64 hardware.

---

## Sources

- [Intel SDM Volume 3C — Virtual Machine Extensions (VMX Architecture)](https://www.intel.com/content/dam/www/public/us/en/documents/manuals/64-ia-32-architectures-software-developer-vol-3c-part-3-manual.pdf)
- [AMD64 Architecture Programmer's Manual Volume 2 — Chapter 15 (SVM)](https://www.amd.com/content/dam/amd/en/documents/processor-tech-docs/programmer-references/24593.pdf)
- [KVM CPUID bits — Linux Kernel documentation](https://docs.kernel.org/virt/kvm/x86/cpuid.html)
- [Linux Find Out If CPU Support Intel VT/AMD-V — nixCraft](https://www.cyberciti.biz/faq/linux-xen-vmware-kvm-intel-vt-amd-v-support/)
- [x86 virtualization — Wikipedia](https://en.wikipedia.org/wiki/X86_virtualization)
- [CVE-2021-3653 — NVD](https://nvd.nist.gov/vuln/detail/CVE-2021-3653)
- [CVE-2018-12207 (iTLB Multihit) — NVD](https://nvd.nist.gov/vuln/detail/CVE-2018-12207)
