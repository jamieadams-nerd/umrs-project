# Phase 1G Summary — Virtualization, Reliability & Platform Topology

**Phase:** 1G
**Date:** 2026-03-18
**Status:** COMPLETE
**Files in this directory:**

| File | Feature(s) | Classification |
|------|------------|---------------|
| `vmx-svm.md` | VMX (Intel VT-x) + SVM (AMD-V) | Informational |
| `iommu.md` | VT-d / AMD-Vi IOMMU | Critical/Defensive |
| `nested-paging.md` | EPT (Intel) + NPT (AMD) | Informational |
| `mca-ras-ecc.md` | MCA + RAS features + ECC memory | Important |
| `smt-topology.md` | SMT / Hyperthreading state | Important (security cross-cut) |
| `microcode-tracking.md` | Microcode version | Important |

---

## Phase 1G Scope

Phase 1G covers features in the plan's Categories 12, 13, and 14:

- **Category 12 (Virtualization Security):** VMX, SVM, nested paging (EPT/NPT), IOMMU
- **Category 13 (Reliability / Availability / Resilience):** MCA, RAS, ECC
- **Category 14 (Platform Topology & Metadata):** SMT state, microcode version

Features 45–53 in the complete feature list.

---

## Cross-Cutting Properties

Phase 1G reveals several properties that cut across other phases.

### IOMMU is the hardware prerequisite for Thunderbolt/FireWire blacklisting

The UMRS posture signals for Thunderbolt and FireWire blacklisting (module blacklist entries for `thunderbolt`, `firewire_core`, `firewire_ohci`) are necessary but not sufficient without IOMMU:

- Without IOMMU: a Thunderbolt hardware device can DMA physical memory without loading any kernel module. The kernel blacklist is bypassed.
- With IOMMU active: the device's DMA range is IOMMU-restricted to whatever the OS maps, which is nothing if no driver is loaded.

**Proposed signal:** IOMMU active + strict mode (not passthrough). This is a new signal proposal to pair with existing Thunderbolt/FireWire signals. Classification: Critical/Defensive.

### SMT state determines mitigation completeness for Phase 1E signals

SMT state is not a vulnerability but determines whether Phase 1E mitigations (MDS, L1TF, STIBP) provide full isolation:

- MDS with SMT active: VERW flush required on every context switch and VM entry
- L1TF with SMT active and untrusted VMs: L1D flush required on every VM entry (expensive) or SMT disabled
- STIBP: only relevant when SMT is active (prevents cross-sibling branch predictor contamination)

**The audit-card logic for mitigations must factor in SMT state.** A system showing "Mitigation: IBRS" in `/sys/devices/system/cpu/vulnerabilities/spectre_v2` but with STIBP not active and SMT enabled has incomplete sibling-thread isolation.

### Microcode version determines whether Phase 1E mitigations work at the hardware level

The most critical relationship in Phase 1G: microcode staleness is invisible from `/sys/devices/system/cpu/vulnerabilities/`. The sysfs mitigation status reflects what the kernel thinks is active; microcode version determines whether the hardware actually implements it.

MD_CLEAR is the canonical example: the kernel's VERW-based MDS mitigation only works if the CPU's microcode implements the MD_CLEAR behavior for VERW. Pre-MD_CLEAR microcode + kernel MDS mitigation code = kernel executes VERW instructions that do nothing useful.

---

## Detection Reference Table

Quick-reference for UMRS posture signal development.

| Feature | Primary Detection Path | Expected Value | Notes |
|---------|----------------------|----------------|-------|
| VMX active | `/sys/module/kvm_intel/` exists | directory present | KVM loaded |
| SVM active | `/sys/module/kvm_amd/` exists | directory present | KVM loaded |
| EPT active | `/sys/module/kvm_intel/parameters/ept` | `Y` | N = misconfiguration |
| NPT active | `/sys/module/kvm_amd/parameters/npt` | `1` | 0 = misconfiguration |
| IOMMU present | `/sys/kernel/iommu_groups/` | non-empty directory | populated = active |
| IOMMU class | `/sys/class/iommu/` | `dmar0` etc. | shows IOMMU devices |
| IOMMU strict | `/proc/cmdline` | `iommu=strict` | passthrough absent |
| Intel IOMMU active | `/proc/cmdline` | `intel_iommu=on` | required explicitly |
| SMT state | `/sys/devices/system/cpu/smt/active` | `0` or `1` | 1 = SMT active |
| SMT control | `/sys/devices/system/cpu/smt/control` | `on/off/forceoff` | |
| SMT siblings | `/sys/devices/system/cpu/cpu0/topology/thread_siblings_list` | `0,4` (if HT) | sibling pair |
| MCA flags | `/proc/cpuinfo` | `mce mca` in flags | both on x86_64 |
| RAS monitoring | `systemctl is-active rasdaemon` | `active` | |
| EDAC loaded | `/sys/devices/system/edac/mc/` | non-empty | |
| ECC type | `dmidecode -t 17` | `Single-bit ECC` | |
| ECC corrected errors | `/sys/devices/system/edac/mc/mc0/ce_count` | numeric (0 = no errors) | |
| Microcode revision | `/proc/cpuinfo` `microcode` field | hex (e.g., `0xb4`) | per-core, all must match |
| Microcode version (sysfs) | `/sys/devices/system/cpu/cpu0/microcode/version` | same hex | |
| Microcode early load | `dmesg \| grep microcode` | update message | initrd early load |
| Nested VMX | `/sys/module/kvm_intel/parameters/nested` | `Y` if enabled | HIGH risk if unnecessary |
| Nested SVM | `/sys/module/kvm_amd/parameters/nested` | `1` if enabled | HIGH risk if unnecessary |

---

## Key Audit Findings from Phase 1G

### IOMMU

| Finding | Severity | Detection |
|---------|----------|-----------|
| IOMMU not active, system has Thunderbolt | HIGH | `/sys/kernel/iommu_groups/` empty + Thunderbolt hardware present |
| IOMMU in passthrough mode (`iommu=pt`) | MEDIUM | `/proc/cmdline` |
| Intel IOMMU not explicitly activated | MEDIUM | `intel_iommu=on` absent from `/proc/cmdline` |
| BIOS has IOMMU but kernel did not initialize it | HIGH | `dmesg` ACPI DMAR table present, no "IOMMU enabled" message |

### Virtualization

| Finding | Severity | Detection |
|---------|----------|-----------|
| EPT disabled in KVM | MEDIUM | `/sys/module/kvm_intel/parameters/ept` = `N` |
| NPT disabled in KVM | MEDIUM | `/sys/module/kvm_amd/parameters/npt` = `0` |
| Nested VMX/SVM enabled unnecessarily | HIGH | Nested enabled on non-hypervisor host |

### Reliability / RAS

| Finding | Severity | Detection |
|---------|----------|-----------|
| rasdaemon not active | MEDIUM | `systemctl is-active rasdaemon` returns inactive |
| Non-ECC memory on CUI server | HIGH | `dmidecode -t 17` shows no ECC |
| High corrected error rate | MEDIUM | `ce_count` increasing rapidly — DIMM degrading |

### SMT / Topology

| Finding | Severity | Detection |
|---------|----------|-----------|
| SMT enabled + MDS unmitigated | HIGH | SMT active + `/vulnerabilities/mds` shows "Vulnerable" |
| SMT enabled + untrusted VMs + no L1D flush | HIGH | SMT active + VMs present + `/vulnerabilities/l1tf` shows conditional flushes only |
| SMT enabled + STIBP not active | MEDIUM | SMT active + `/vulnerabilities/spectre_v2` lacks STIBP |

### Microcode

| Finding | Severity | Detection |
|---------|----------|-----------|
| microcode_ctl package not current | MEDIUM | `dnf check-update microcode_ctl` |
| No early microcode loading | MEDIUM | `dmesg` shows no microcode update message; initrd lacks microcode |
| Core microcode mismatch | HIGH | Different microcode values across cores in `/proc/cpuinfo` |

---

## New Signal Proposals

The following signals are proposed for UMRS based on Phase 1G research. These do not exist yet in the posture catalog.

### Proposed: `IOMMU_ACTIVE` (Critical/Defensive)

- **Layer 1:** `/sys/kernel/iommu_groups/` is non-empty
- **Layer 2:** `/proc/cmdline` does not contain `iommu=pt`
- **Finding condition:** Thunderbolt posture signals are active (blacklisting) but IOMMU_ACTIVE is false → contradiction finding
- **NIST SP 800-53:** SC-51, AC-4
- **Classification:** Critical/Defensive — without IOMMU, DMA attacks bypass kernel blacklisting

### Proposed: `MICROCODE_CURRENT` (Important)

- **Detection:** Compare `microcode_ctl` RPM version against latest available in RHEL repos
- **Finding:** Stale microcode = mitigation correctness unverifiable
- **NIST SP 800-53:** SI-2 (Flaw Remediation), CM-6 (Configuration Settings)
- **Classification:** Important — not immediately exploitable but creates invisible mitigation gap

### Proposed: `ECC_ACTIVE` (Important)

- **Layer 1:** `dmidecode -t 17` shows ECC type = `Single-bit ECC` or `Multi-bit ECC`
- **Layer 2:** `rasdaemon` active + EDAC loaded
- **Finding:** Non-ECC memory on CUI processing system
- **NIST SP 800-53:** SC-28 (Protection of Information at Rest — hardware integrity component)
- **Classification:** Important — ECC is a baseline CUI system expectation

### Proposed: `SMT_MITIGATIONS_COMPLETE` (Important, compound)

- **Logic:** If SMT active AND (`mds` mitigation = "Not affected" OR "Mitigation: Clear CPU buffers") AND (`l1tf` fully mitigated) AND (`spectre_v2` includes STIBP) → PASS
- **Finding:** Any of the above conditions not met while SMT is active
- **NIST SP 800-53:** SC-39 (Process Isolation)
- **Classification:** Important — depends on SMT state

---

## Relationship to Existing UMRS Signals

| Existing Signal | Phase 1G Connection |
|-----------------|---------------------|
| `Thunderbolt` module blacklisted | IOMMU must be active for this blacklisting to have physical effect |
| `FirewireOhci` module blacklisted | Same — IOMMU prerequisite |
| `Mitigations` (Critical) | SMT state determines which mitigations are needed; microcode version determines if they work |
| `Pti` | PTI performance is improved by PCID (Phase 1E); SMT state affects PTI necessity analysis |

---

## ARM/AArch64 Notes

| x86 Feature | ARM Equivalent | Detection |
|---|---|---|
| VMX / SVM | EL2 hypervisor mode (always present on server AArch64) | KVM built into RHEL 10 kernel for AArch64 |
| EPT / NPT | Stage 2 translation | Always active when KVM hypervisor is running |
| IOMMU (VT-d / AMD-Vi) | ARM SMMU v3 | `/sys/bus/platform/drivers/arm-smmu-v3/`; `dmesg \| grep smmu` |
| MCA | ARM RAS Extension (ARMv8.2+) | `/sys/devices/system/cpu/vulnerabilities/` same interface |
| SMT | Not present on current Neoverse N1/N2 | No SMT — each core is single-threaded; all SMT-related signals are N/A |
| Microcode | TF-A / UEFI firmware | `dmidecode -t 0` for firmware version; no `/proc/cpuinfo` microcode field |

**Important for UMRS on AArch64:** SMT-related findings (STIBP, MDS, L1TF cross-sibling) are not applicable on Neoverse N1/N2, which does not implement SMT. This must be accounted for in platform-specific signal logic.

---

## Phase Status

All 6 files written. Phase 1G is complete.

**Next phases:**
- Phase 1H: `/proc/crypto` & Software Utilization Reference
- Phase 1I: Matrix Synthesis (requires 1A-1H complete)
