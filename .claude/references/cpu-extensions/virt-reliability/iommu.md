# VT-d / AMD-Vi — IOMMU (DMA Isolation)

**Category:** 12 — Virtualization Security
**Feature #:** 48
**Phase:** 1G
**Date:** 2026-03-18
**Classification:** Critical/Defensive

---

## 23-Column Matrix Profile

| # | Column | Value |
|---|--------|-------|
| 1 | Feature name | VT-d / AMD-Vi — IOMMU (Input-Output Memory Management Unit) |
| 2 | Vendor | Intel (VT-d), AMD (AMD-Vi) — functionally equivalent; both |
| 3 | Category | Virtualization Security (Category 12); also Platform Security |
| 4 | Purpose | Hardware IOMMU applies virtual memory concepts to the I/O bus. Translates I/O Virtual Addresses (IOVA) to physical addresses, restricting each DMA-capable device to only the memory pages explicitly permitted by the OS. Without IOMMU active, DMA-capable peripherals have unrestricted access to all physical memory. |
| 5 | Example instructions | None — IOMMU is hardware infrastructure, not CPU instructions. Configured via ACPI tables (DMAR on Intel, IVRS on AMD) and kernel drivers. |
| 6 | CPUID leaf/subleaf/bit | Not CPUID-detectable. Presence is discovered via ACPI DMAR/IVRS tables read by the firmware. Intel VT-d capability is reported in ACPI DMAR (DMA Remapping) table. |
| 7 | Linux `/proc/cpuinfo` flag | None — IOMMU is not represented in `/proc/cpuinfo`. |
| 8 | Linux detection — authoritative path | Primary: `/sys/kernel/iommu_groups/` — if this directory exists and contains numbered subdirectories, IOMMU is active and devices are grouped. Secondary: `/sys/class/iommu/` — lists active IOMMU devices by name (e.g., `dmar0`, `amd_iommu`). Kernel log: `dmesg | grep -E 'DMAR|AMD-Vi|IOMMU'` shows initialization messages. `/proc/cmdline` for `intel_iommu=on` or `amd_iommu=on`. |
| 9 | Minimum CPU generations | Intel: Sandy Bridge (2011) and later with VT-d enabled in BIOS/chipset. AMD: Phenom II (2009) and later; Zen architecture has reliable AMD-Vi support. Requires both CPU and chipset support — some laptop chipsets omit IOMMU hardware even when CPU supports it. |
| 10 | Security benefit | Prevents DMA attacks: a malicious or compromised PCIe/Thunderbolt/FireWire device cannot read arbitrary physical memory or overwrite kernel memory. Limits blast radius of a compromised peripheral to its assigned IOVA range only. Enforces that kernel-level blacklisting of Thunderbolt and FireWire devices has physical backing — without IOMMU, kernel blacklisting can be bypassed by forcing DMA directly. |
| 11 | Performance benefit | Slight overhead on DMA operations from address translation. Modern IOMMU hardware includes TLBs to minimize this. On high-throughput NIC/storage workloads, IOMMU can reduce throughput 2–10%; passthrough mode (`iommu=pt`) eliminates overhead for trusted devices while keeping the infrastructure active for untrusted ones. |
| 12 | Assurance caveats | (1) BIOS-disabled by default on many OEM systems — CPU/chipset may support IOMMU but it is inert until a kernel parameter activates it. (2) Thunderclap (2019 NDSS) demonstrated that even with IOMMU active, complex device interactions (DMA + MMIO) can be exploited by malicious peripherals — IOMMU is necessary but not sufficient. (3) UEFI firmware bugs can prevent IOMMU initialization even when enabled (documented in CVE-2023-20588 adjacent issues). (4) `iommu=pt` (passthrough) mode disables isolation for most devices to recover performance — audit whether pt mode is in use. (5) IOMMU does not protect against CPU-side DMA attacks (e.g., Thunderbolt 4 USB4 merge means USB4 devices inherit Thunderbolt DMA capability). (6) FireWire (1394), PCMCIA, CardBus, and ExpressCard are NOT covered by IOMMU DMA protection on Windows; Linux IOMMU coverage for FireWire requires explicit configuration. |
| 13 | Virtualization behavior | VMs cannot directly observe the host IOMMU state. IOMMU groups are a host-level construct. For VFIO passthrough (GPU/NIC passthrough to VM), IOMMU groups determine which devices can be passed through together — IOMMU must be active on the host. Guest cannot verify IOMMU protection from inside a VM without platform attestation. |
| 14 | Firmware / BIOS / microcode dependency | `{ bios_enable_required: true, microcode_required: false }` — IOMMU is typically disabled in BIOS/UEFI by default, especially on consumer and workstation systems. Must be explicitly enabled. On servers, it is more often enabled by default. Intel VT-d requires chipset support in addition to CPU support. A BIOS bug can prevent IOMMU initialization even when BIOS says "enabled." Verify via `dmesg`. |
| 15 | Audit-card relevance | **Critical/Defensive** — Without active IOMMU, kernel-level blacklisting of Thunderbolt and FireWire (existing UMRS posture signals) is ineffective against a physical attacker with a malicious PCIe or Thunderbolt device. DMA attacks bypass all OS memory access controls. This is a direct physical attack path on CUI systems. |
| 16 | Recommended disposition when unused | **Enable in BIOS and activate in kernel.** Intel: add `intel_iommu=on` to kernel cmdline. AMD: IOMMU is often auto-detected; add `amd_iommu=on` if not. Use `iommu=strict` for maximum isolation (disable passthrough mode). If system has no Thunderbolt/FireWire/PCIe hot-plug ports, the risk is reduced but IOMMU should still be enabled for defense-in-depth against firmware-level attacks. **Finding if absent on a system with Thunderbolt ports.** |
| 17 | Software utilization detection method | Kernel: `dmesg | grep -E 'DMAR.*enabled|AMD-Vi.*enabled|iommu.*init'`. Groups populated: `ls /sys/kernel/iommu_groups/ | wc -l` — non-zero means active. Strict mode: parse `/proc/cmdline` for `iommu=strict`. Passthrough mode: parse `/proc/cmdline` for `iommu=pt`. KVM VFIO passthrough in use: `/dev/vfio/` directory populated. |
| 18 | FIPS utilization requirement | N/A — IOMMU is not a cryptographic feature. However, FIPS systems processing CUI with Thunderbolt-capable ports should treat IOMMU as a mandatory physical security control. |
| 19 | Active mitigation status path | No dedicated `/sys/devices/system/cpu/vulnerabilities/` path. IOMMU status is via `/sys/kernel/iommu_groups/` population and `dmesg` messages at boot. |
| 20 | Feature accessible vs advertised | IOMMU is not CPUID-advertised. BIOS must enable it; kernel must activate it via cmdline parameter (Intel) or auto-detect (AMD). Three independent gates: (1) chipset supports IOMMU hardware, (2) BIOS has it enabled, (3) kernel cmdline activates it. All three must pass. |
| 21 | Guest-vs-host discrepancy risk | **Extreme** — A VM guest has no visibility into host IOMMU state at all. IOMMU is entirely a host platform property. |
| 22 | Notes | The connection to UMRS posture signals for Thunderbolt and FireWire blacklisting is critical: those signals detect that the kernel has blacklisted the modules (`thunderbolt`, `firewire_core`, `firewire_ohci`) — but without IOMMU, physical DMA access via Thunderbolt hardware does not require loading those modules. IOMMU is the hardware layer that makes software blacklisting meaningful against a determined physical attacker. Intel brands its IOMMU as "VT-d" (Virtualization Technology for Directed I/O), distinct from "VT-x" (CPU virtualization). |
| 23 | Sources | Intel VT-d Architecture Specification; AMD IOMMU Architecture Spec; Linux kernel `Documentation/x86/iommu.rst`; NDSS 2019 Thunderclap paper; Linux ABI docs `sysfs-kernel-iommu_groups`; Red Hat RHEL 7 Virtualization Deployment Guide (IOMMU Groups) |

---

## Description

IOMMU (Input-Output Memory Management Unit) is a hardware component in the CPU/chipset that interposes on all DMA (Direct Memory Access) operations from I/O devices. It functions as a second-level memory management unit for the I/O bus: just as the CPU's MMU translates virtual addresses for processes, the IOMMU translates I/O Virtual Addresses (IOVA) for device DMA requests.

When active, every DMA request from a PCIe device passes through the IOMMU, which checks the device's IOVA against a set of page tables maintained by the OS kernel. The device is permitted access only to the physical pages explicitly mapped into its IOVA space. Unmapped physical addresses are blocked — the device cannot access them regardless of what address it targets.

Intel calls its IOMMU technology VT-d (Virtualization Technology for Directed I/O). AMD calls its implementation AMD-Vi (formerly known as IOMMU or AMD IOMMU). They are functionally equivalent from a kernel driver perspective — Linux uses a unified IOMMU subsystem with Intel and AMD backends.

---

## Why Security Engineers Care

### DMA attacks bypass all software memory protections

Without IOMMU, any DMA-capable device has unrestricted physical memory access. This includes:

- **Thunderbolt devices** (PCIe over Thunderbolt cable): A malicious Thunderbolt device plugged in for seconds can read RAM including encryption keys, authentication tokens, and process memory — without any OS interaction. The attack works even on a locked screen. The device loads no driver and triggers no OS event.
- **FireWire (IEEE 1394) devices**: FireWire was designed with DMA capability built in for peer-to-peer performance. A rogue FireWire device can read/write arbitrary physical memory.
- **Malicious PCIe cards**: A rogue NIC, GPU, or storage card with a crafted firmware can DMA arbitrary memory.
- **Compromised firmware on legitimate devices**: A NIC or storage controller with compromised firmware can use its DMA capability maliciously.

### IOMMU is the hardware prerequisite for software blacklisting effectiveness

UMRS already tracks kernel-level blacklisting of `thunderbolt`, `firewire_core`, and `firewire_ohci` modules as posture signals. Without IOMMU, these signals are necessary but insufficient:

- A Thunderbolt hardware device can still perform PCIe DMA even if the kernel module is blacklisted, because Thunderbolt DMA operates at the PCIe level, not the driver level.
- With IOMMU active, the Thunderbolt device's DMA range is restricted to what the IOMMU allows — which is nothing if no driver has mapped IOVA ranges for it.

**The security guarantee chain:** Thunderbolt blacklisting (software) + IOMMU (hardware) = DMA attack blocked. Thunderbolt blacklisting alone (no IOMMU) = attacker can still DMA without loading a driver.

### IOMMU and MLS boundary enforcement

On CUI systems, IOMMU enforcement means that no peripheral can bypass the OS's memory protection to read labeled data directly. This is a hardware enforcement point for NIST SP 800-53 AC-4 (Information Flow Enforcement) at the physical layer.

---

## CVE Summary

| CVE | Year | CVSS | Impact | Fix |
|-----|------|------|--------|-----|
| Thunderclap (no CVE) | 2019 | — | NDSS paper: even with IOMMU, complex device interactions (DMA + MMIO) allow memory access beyond IOVA mapping. Demonstrated on macOS, FreeBSD, Linux. Root cause: IOMMU protects DRAM access but not MMIO interactions. | OS patches to restrict MMIO access from untrusted devices |
| CVE-2023-20588 | 2023 | 5.5 | AMD IOMMU: division by zero in IOMMU exception handling; adjacent to a class of IOMMU initialization failures | AMD microcode + kernel patch |
| CVE-2022-40982 (Gather Data Sampling) | 2023 | 6.5 | Intel: IOMMU-protected memory still leaked via AVX gather instructions; IOMMU does not protect against speculative access patterns | Microcode + kernel mitigation |
| CVE-2019-0155 / CVE-2019-0154 | 2019 | 8.8 / 6.0 | Intel i915: GPU DMA operations not properly restricted; IOMMU groups could be bypassed via userspace GPU commands | Kernel patch (DRM/i915) |
| GIGABYTE UEFI (2023) | 2023 | — | Firmware vulnerability preventing IOMMU initialization on certain Gigabyte motherboards even when enabled in BIOS | UEFI firmware update |

---

## Compliance-Specific Requirements

- **NIST SP 800-53 SC-51 (Hardware-Based Protection):** Physical DMA protection via IOMMU directly satisfies the intent of hardware-enforced access separation. The control explicitly covers hardware features that protect information from physical access attacks.
- **NIST SP 800-53 AC-4 (Information Flow Enforcement):** IOMMU enforces that peripheral devices cannot access memory beyond their authorized range — an information flow control at the hardware layer.
- **NIST SP 800-53 SI-3 (Malicious Code Protection):** IOMMU limits the blast radius of compromised firmware on peripheral devices.
- **CMMC SC.L2-3.13.1 (Boundary Protection):** IOMMU is a hardware boundary control preventing unauthorized DMA across the I/O bus boundary.
- **DoD STIG relevance:** No specific RHEL 10 STIG rule mandates IOMMU, but physical security assessments for DoD systems expect DMA protection to be active on systems with hot-plug peripherals (Thunderbolt, FireWire). An assessor reviewing a CUI system with Thunderbolt ports and no IOMMU would flag it.

---

## Virtualization Confidence

IOMMU is entirely a host platform property. A VM guest:

- Cannot detect whether the host has IOMMU active
- Cannot verify that its own DMA operations are restricted vs unrestricted
- Cannot distinguish a host running VFIO passthrough (IOMMU-isolated) from one without IOMMU

For confidential VM workloads, IOMMU active on the host is a prerequisite for trusting that a passed-through device cannot DMA the guest's memory outside its assigned range. AMD SEV-SNP and Intel TDX provide additional protection for guest memory beyond what IOMMU alone offers.

---

## Detection Reference

### Layer 1 — Is IOMMU present and active?

```
# Check for populated IOMMU groups (most reliable)
ls /sys/kernel/iommu_groups/ | wc -l
# Non-zero = IOMMU active

# Check for IOMMU class devices
ls /sys/class/iommu/
# Lists: dmar0, dmar1 (Intel) or amd_iommu_0 (AMD)

# Check kernel boot messages
dmesg | grep -E 'DMAR|AMD-Vi|IOMMU'
# Intel: "DMAR: IOMMU enabled"
# AMD: "AMD-Vi: AMD IOMMUv2 enabled"

# Check kernel cmdline
grep -E 'intel_iommu|amd_iommu|iommu' /proc/cmdline
```

### Layer 2 — Is IOMMU in strict (non-passthrough) mode?

```
# Passthrough mode (isolation disabled for most devices)
grep 'iommu=pt' /proc/cmdline

# Strict mode (full isolation, performance cost)
grep 'iommu=strict' /proc/cmdline

# Intel: explicit activation required
grep 'intel_iommu=on' /proc/cmdline
```

### Finding conditions

| Finding | Severity | Condition |
|---------|----------|-----------|
| IOMMU not active on system with Thunderbolt/FireWire | HIGH | `/sys/kernel/iommu_groups/` empty, system has Thunderbolt-capable hardware |
| IOMMU in passthrough mode | MEDIUM | `iommu=pt` in cmdline — isolation disabled for most devices |
| BIOS reports VT-d/AMD-Vi but IOMMU not initialized | HIGH | `dmesg` shows ACPI DMAR table but no "IOMMU enabled" message |
| Thunderbolt blacklisted but IOMMU absent | HIGH | Software blacklisting is insufficient without IOMMU hardware backing |

---

## UMRS Posture Signal Connection

The IOMMU posture signal is a **new signal proposal** for UMRS. It connects to existing Thunderbolt/FireWire blacklisting signals as a prerequisite:

- **Existing signals:** `thunderbolt` module blacklisted, `firewire_ohci` module blacklisted
- **New signal:** IOMMU active (hardware DMA protection)
- **Layer 2:** IOMMU in strict mode (not passthrough)
- **Compound finding:** Thunderbolt blacklisted + IOMMU inactive = incomplete protection (the two signals together should trigger a contradiction finding)

Classification: **Critical/Defensive** — without IOMMU, DMA attacks are a viable physical attack path against CUI data even with module blacklisting in place.

---

## ARM/AArch64 Equivalent

ARM server platforms use the **ARM SMMU (System Memory Management Unit)**, the ARM equivalent of x86 IOMMU. Detection:

- `/sys/bus/platform/drivers/arm-smmu/` or `/sys/bus/platform/drivers/arm-smmu-v3/`
- `dmesg | grep -i 'smmu'`
- SMMU v3 (ARM SMMU Architecture version 3) is the modern standard for AArch64 server platforms

On the UMRS project's AArch64 RHEL 10 target (kernel 6.12, Neoverse N1/N2 class), SMMU v3 is the expected IOMMU implementation.

---

## Sources

- [Intel Virtualization Technology for Directed I/O Architecture Specification](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) — Intel Architecture Spec
- [Linux Kernel x86 IOMMU Documentation](https://www.kernel.org/doc/html/v6.2/x86/iommu.html)
- [Linux sysfs-kernel-iommu_groups ABI](https://www.kernel.org/doc/Documentation/ABI/testing/sysfs-kernel-iommu_groups)
- [Red Hat RHEL 7 Virtualization — IOMMU Groups Deep-Dive](https://docs.redhat.com/en/documentation/red_hat_enterprise_linux/7/html/virtualization_deployment_and_administration_guide/sect-iommu-deep-dive)
- [NDSS 2019 — Thunderclap: Exploring Vulnerabilities in Operating System IOMMU Protection](https://www.ndss-symposium.org/ndss-paper/thunderclap-exploring-vulnerabilities-in-operating-system-iommu-protection-via-dma-from-untrustworthy-peripherals/)
- [Synacktiv — IOMMU and DMA attacks (2020)](https://www.synacktiv.com/sites/default/files/2020-05/IOMMU_and_DMA_attacks_presentation.pdf)
- [Red Hat — How to enable IOMMU for SR-IOV on RHEL](https://access.redhat.com/solutions/7128263)
- [GitHub: rasdaemon](https://github.com/mchehab/rasdaemon)
- [NVD: CVE-2023-20588](https://nvd.nist.gov/vuln/detail/CVE-2023-20588)
