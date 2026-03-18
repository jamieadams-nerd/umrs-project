# EPT / NPT — Nested Paging (Second Level Address Translation)

**Category:** 12 — Virtualization Security
**Feature #:** 47
**Phase:** 1G
**Date:** 2026-03-18
**Classification:** Informational

---

## 23-Column Matrix Profile

| # | Column | Value |
|---|--------|-------|
| 1 | Feature name | EPT / NPT — Extended Page Tables (Intel) / Nested Page Tables (AMD); also called Second Level Address Translation (SLAT) |
| 2 | Vendor | Intel (EPT), AMD (NPT) — both |
| 3 | Category | Virtualization Security (Category 12) |
| 4 | Purpose | Hardware second-level address translation for virtual machines. Adds a CPU-managed page table layer that translates Guest Physical Addresses (GPA) to Host Physical Addresses (HPA) without hypervisor intervention on every access. Reduces VM exits for memory translation and provides hardware-enforced memory isolation between VMs. |
| 5 | Example instructions | No new instructions. EPT/NPT are enabled via VMCS/VMCB control bits, not explicit instructions. EPT violation (fault type) triggers a VM exit handled by the hypervisor. |
| 6 | CPUID leaf/subleaf/bit | Intel EPT: CPUID leaf 01H does not directly advertise EPT. EPT capability is reported via IA32_VMX_EPT_VPID_CAP MSR (MSR 48CH) — accessible only from VMX root mode. From guest/userspace: infer from the `ept` flag in `/proc/cpuinfo` (kernel exposes this). AMD NPT: CPUID leaf 8000000AH (SVM Feature Identifiers), EAX bit 0 (`NP` — Nested Paging). |
| 7 | Linux `/proc/cpuinfo` flag | Intel: `ept` — present in the `flags` line when EPT is supported. Also `vpid` (Virtual Processor IDs, a companion feature). AMD: `npt` — present when Nested Page Tables are supported. |
| 8 | Linux detection — authoritative path | Intel: `/sys/module/kvm_intel/parameters/ept` — reads `Y` when EPT is active in KVM. Value of `N` means KVM is running without EPT (software page table shadow mode). AMD: `/sys/module/kvm_amd/parameters/npt` — reads `1` when NPT is active. |
| 9 | Minimum CPU generations | Intel: Nehalem (Core i7 first gen, 2008). Effectively all Intel CPUs post-2008 support EPT. AMD: Phenom II (2009) with SVM + NPT. All Zen-architecture AMD CPUs support NPT. |
| 10 | Security benefit | Hardware-enforced VM memory isolation: each VM's GPA-to-HPA mapping is managed by CPU hardware, not hypervisor software page tables. A guest OS cannot access another guest's physical pages without modifying the EPT/NPT, which is a privileged hypervisor operation. Also: EPT supports execute-only permissions (AMD NPT does not), enabling a hypervisor to implement page-level execute controls that AMD NPT cannot. |
| 11 | Performance benefit | Eliminates the hypervisor "shadow page table" maintenance overhead. Without EPT/NPT, the hypervisor must keep a synchronized shadow copy of guest page tables, taking a VM exit on every guest page table modification. With EPT/NPT, the CPU manages the two-level walk in hardware — typical VM workloads see 10–30% performance improvement. |
| 12 | Assurance caveats | (1) EPT/NPT is less secure than IOMMU — it protects CPU-driven memory accesses but does not restrict DMA from I/O devices (that is IOMMU's role). (2) EPT violations generate VM exits that the hypervisor handles; a buggy VM exit handler can be exploited (see VENOM CVE-2015-3456). (3) AMD NPT does not support execute-only pages (EPT does) — this is an asymmetric security capability between Intel and AMD hypervisor platforms. (4) L1TF/Foreshadow (CVE-2018-3646) exploited EPT to speculatively read L1 cache across EPT boundaries — EPT does not fully prevent speculative cross-VM access without L1D flushing on VM entry. (5) Nested EPT (EPT for a VM that is itself running a hypervisor) adds another page-table level and substantially complicates the hypervisor's security surface. |
| 13 | Virtualization behavior | EPT/NPT is transparent to the guest OS. The guest believes it is operating on physical memory; the CPU silently adds the second-level translation. A guest cannot detect whether EPT/NPT is active from inside the VM. KVM uses EPT/NPT by default when available; shadow paging is the fallback. The `ept` flag in guest `/proc/cpuinfo` reflects the host CPU's capability, not an assertion that EPT is being used for this guest — the KVM module parameter is authoritative. |
| 14 | Firmware / BIOS / microcode dependency | `{ bios_enable_required: false, microcode_required: false }` — EPT/NPT is part of the VMX/SVM feature set. If VMX/SVM is BIOS-enabled, EPT/NPT capability is available. No separate BIOS switch required. Minimum microcode: baseline for the CPU generation (no special fix required for the feature itself; L1TF mitigation requires later microcode). |
| 15 | Audit-card relevance | **Informational** — EPT/NPT presence is necessary infrastructure for secure virtualization but is not independently actionable. The security value comes from the hypervisor using it correctly (KVM module parameter) rather than from presence alone. Relevant to platform inventory for systems running VMs. |
| 16 | Recommended disposition when unused | If the system runs no VMs: N/A (EPT/NPT is inert without VMX/SVM active). If running KVM: verify `kvm_intel ept=Y` or `kvm_amd npt=1` — running KVM in shadow-paging mode (EPT/NPT disabled) is a misconfiguration that degrades isolation and performance simultaneously. |
| 17 | Software utilization detection method | Intel KVM: `cat /sys/module/kvm_intel/parameters/ept` — value `Y` means EPT in use. AMD KVM: `cat /sys/module/kvm_amd/parameters/npt` — value `1` means NPT in use. Also: `dmesg | grep -E 'EPT|NPT|ept|npt'` for KVM initialization messages. |
| 18 | FIPS utilization requirement | N/A — EPT/NPT is not a cryptographic feature. |
| 19 | Active mitigation status path | Related: `/sys/devices/system/cpu/vulnerabilities/l1tf` — L1TF mitigation effectiveness is affected by whether EPT is active and whether L1D flush is configured. Full L1TF mitigation requires EPT active + `l1d_flush=always` on VM entry or SMT disabled. |
| 20 | Feature accessible vs advertised | EPT/NPT is dependent on VMX/SVM being enabled (BIOS gate). Once VMX/SVM is available, EPT/NPT is accessible. KVM administrator can explicitly disable EPT via `kvm_intel.ept=0` module parameter — this is a misconfiguration finding. |
| 21 | Guest-vs-host discrepancy risk | **Moderate** — The `ept` flag in guest `/proc/cpuinfo` is passed through from the host CPU. A guest cannot determine whether EPT is actually being used for its own page table management vs shadow paging. The KVM parameter is a host-only observable. |
| 22 | Notes | Intel EPT supports execute-only page permissions (read=0, write=0, execute=1). AMD NPT does not — AMD NPT pages must be readable if executable. This asymmetry matters for hypervisors implementing fine-grained memory permission enforcement. Intel's advantage here is relevant when comparing hypervisor hardening depth across platforms. VPID (Virtual Processor Identifiers) is a companion feature to EPT that tags TLB entries per-VCPU, reducing TLB flush overhead on VM entry/exit. |
| 23 | Sources | Intel SDM Vol 3C Chs 28-29 (EPT architecture); AMD APM Vol 2 Ch 15.25 (Nested Paging); Linux kernel `arch/x86/kvm/` EPT and NPT implementations; L1TF kernel documentation; NVD CVE-2018-3646 |

---

## Description

Nested paging (EPT on Intel, NPT on AMD) implements Second Level Address Translation (SLAT): a hardware page table layer that the CPU uses to translate Guest Physical Addresses into Host Physical Addresses, in addition to the normal guest virtual-to-physical page walk.

Without nested paging, hypervisors must maintain "shadow page tables" — a synchronized copy of the guest's page tables in host-physical address space. Every guest page table write triggers a VM exit to the hypervisor, which updates the shadow copy. This is correct but slow.

With nested paging, the CPU hardware performs a two-dimensional page table walk on every guest memory access: first the guest's virtual-to-GPA translation, then the EPT/NPT GPA-to-HPA translation. No VM exit is required for ordinary memory accesses. The hypervisor only intervenes on EPT/NPT violations (guest accessing a GPA that has no mapping or violates permissions in the EPT).

---

## Why Security Engineers Care

### Hardware-enforced VM memory isolation

EPT/NPT is the mechanism that makes VM memory isolation hardware-enforced rather than software-enforced. The EPT/NPT page tables are under hypervisor control and not accessible to the guest. A guest OS that has been fully compromised cannot access another VM's physical pages without the hypervisor mapping those pages into its EPT — which it will not do.

This is distinct from software virtualization where the hypervisor's shadow page table implementation is the only thing preventing a compromised guest from manipulating its own page table to access host memory.

### L1TF and the limits of EPT isolation

CVE-2018-3646 (L1 Terminal Fault — VMM variant, "Foreshadow-VMM") exploited a CPU speculative execution bug to read L1 cache contents that the EPT had mapped as "not present" for the guest. The CPU speculatively fetched from the L1 cache using the physical address in the not-present PTE before checking EPT permissions — allowing a guest to potentially read another guest's L1-cached data.

The mitigation requires the hypervisor to flush the L1 data cache on every VM entry from an untrusted guest. This is expensive, and makes EPT's isolation guarantee conditional on the hypervisor correctly implementing L1D flushing.

**Practical consequence:** EPT page tables can be bypassed by speculative execution. The CPU instruction set architecture (ISA) guarantee of EPT is weaker than the marketing claims. Full mitigation requires EPT + L1D flush + (optionally) SMT disabled.

### Execute-only pages: Intel EPT advantage over AMD NPT

Intel EPT can mark pages execute-only (R=0, W=0, X=1). AMD NPT cannot — a page must be readable if it is executable. Execute-only pages allow:

- Hiding code from a guest attempting to read its own text segment (useful for hypervisor introspection hooks)
- Fine-grained isolation of code vs data without making code readable

Security researchers use execute-only EPT to implement transparent hypervisor monitoring hooks. This capability is absent from AMD NPT.

---

## CVE Summary

| CVE | Year | CVSS | Impact | Fix |
|-----|------|------|--------|-----|
| CVE-2018-3646 (L1TF / Foreshadow-VMM) | 2018 | 5.6 | Speculative EPT violation allows cross-VM L1 cache data read. VMM variant of L1 Terminal Fault. | Microcode + kernel L1D flush on VM entry; optional SMT disable |
| CVE-2015-3456 (VENOM) | 2015 | 9.3 | Floppy disk controller VM escape via unchecked VM exit handler — not EPT directly, but demonstrates hypervisor VM-exit handler attack surface that EPT violations also trigger | QEMU patch |
| CVE-2021-26932 | 2021 | 5.5 | Linux KVM nested SVM/VMX (nested paging for nested VMs) — incorrect handling in nested EPT/NPT code path | Kernel patch |

---

## Compliance-Specific Requirements

- **NIST SP 800-53 SC-39 (Process Isolation):** EPT/NPT provides the hardware substrate for VM-level isolation. An audit would verify that KVM is using EPT/NPT (not shadow paging) for security-sensitive VM workloads.
- **NIST SP 800-53 SI-7 (Software and Firmware Integrity):** EPT/NPT correctness depends on hypervisor integrity; the hypervisor owns the EPT page tables.
- **L1TF mitigation compliance:** On systems running untrusted VMs, DoD assessors would expect L1D flushing to be configured — confirming EPT is active with L1D flush is part of the mitigation assessment.

---

## Virtualization Confidence

A guest OS cannot reliably determine:

- Whether EPT/NPT is actively being used for its GPA-to-HPA translation (vs shadow paging)
- Whether the EPT page tables are properly configured to isolate it from other VMs
- Whether L1D flushing is occurring on VM entry

The only verification is from the hypervisor host side: check the KVM module parameters and L1TF mitigation status in `/sys/devices/system/cpu/vulnerabilities/l1tf`.

---

## Detection Reference

```
# Intel — is EPT active in KVM?
cat /sys/module/kvm_intel/parameters/ept
# Y = active, N = shadow paging (misconfiguration)

# AMD — is NPT active in KVM?
cat /sys/module/kvm_amd/parameters/npt
# 1 = active, 0 = shadow paging (misconfiguration)

# L1TF status (EPT-dependent mitigation)
cat /sys/devices/system/cpu/vulnerabilities/l1tf
# "Mitigation: PTE Inversion; VMX: cache flushes, SMT disabled" = fully mitigated
# "Mitigation: PTE Inversion; VMX: conditional cache flushes, SMT vulnerable" = partial

# Check /proc/cpuinfo for EPT flag (Intel host, host-level check)
grep -m1 ' ept ' /proc/cpuinfo
```

---

## ARM/AArch64 Equivalent

ARM virtualization uses Stage 2 translation — the AArch64 equivalent of EPT/NPT. The MMU performs a two-stage translation:

- **Stage 1:** Virtual address → Intermediate Physical Address (IPA), controlled by the guest OS at EL1.
- **Stage 2:** IPA → Physical Address, controlled by the hypervisor at EL2.

Stage 2 translation is mandatory for hardware-assisted virtualization on AArch64. It is always present when a hypervisor runs at EL2 — there is no AArch64 equivalent of the legacy x86 shadow-paging fallback. Detection: if KVM is running on AArch64, Stage 2 is active by definition.

---

## Sources

- [Intel SDM Volume 3C — Extended Page Tables (EPT Architecture)](https://www.intel.com/content/dam/www/public/us/en/documents/manuals/64-ia-32-architectures-software-developer-vol-3c-part-3-manual.pdf)
- [AMD64 Architecture Programmer's Manual Volume 2 — Ch 15.25 Nested Paging](https://www.amd.com/content/dam/amd/en/documents/processor-tech-docs/programmer-references/24593.pdf)
- [L1TF — L1 Terminal Fault — Linux Kernel documentation](https://docs.kernel.org/admin-guide/hw-vuln/l1tf.html)
- [NVD: CVE-2018-3646](https://nvd.nist.gov/vuln/detail/CVE-2018-3646)
- [NVD: CVE-2015-3456 (VENOM)](https://nvd.nist.gov/vuln/detail/CVE-2015-3456)
- [nixCraft — Linux Find Out If CPU Support Intel VT/AMD-V](https://www.cyberciti.biz/faq/linux-xen-vmware-kvm-intel-vt-amd-v-support/)
