# Microcode Version Tracking — Mitigation Correctness Prerequisite

**Category:** 14 — Platform Topology & Metadata
**Feature #:** 53
**Phase:** 1G
**Date:** 2026-03-18
**Classification:** Important

---

## 23-Column Matrix Profile

| # | Column | Value |
|---|--------|-------|
| 1 | Feature name | Microcode version — CPU microcode revision tracking |
| 2 | Vendor | Both (Intel and AMD; ARM equivalent: firmware/DTB firmware version) |
| 3 | Category | Platform Topology & Metadata (Category 14) |
| 4 | Purpose | CPU microcode is updateable firmware embedded in the CPU that implements or patches the behavior of CPU instructions and hardware features. Microcode updates are the primary delivery mechanism for hardware-level security mitigations for speculative execution vulnerabilities (Spectre, Meltdown, MDS, L1TF, etc.). Tracking the installed microcode version determines whether those mitigations are actually implemented at the hardware layer, independent of what the OS kernel believes it has activated. |
| 5 | Example instructions | No user-visible instructions. Microcode is loaded by the CPU's built-in microcode loader or by the OS during early boot. Interaction via WRMSR (write to model-specific register) is used by the kernel microcode loader — not available to userspace. |
| 6 | CPUID leaf/subleaf/bit | CPUID leaf 01H, EAX — processor signature (Family/Model/Stepping). Used to determine which microcode package applies to this CPU. Microcode version itself: read from IA32_BIOS_SIGN_ID MSR (MSR 8BH) after executing CPUID leaf 01H. This MSR is not directly readable by userspace on modern kernels (privilege required). |
| 7 | Linux `/proc/cpuinfo` flag | `microcode` field — hex value of the currently loaded microcode revision. Example: `microcode : 0xb4`. This is per-core; all cores should show the same value. On AMD: `microcode : 0x8301034`. |
| 8 | Linux detection — authoritative path | Primary: `/sys/devices/system/cpu/cpu0/microcode/version` — hex string of current microcode revision (same value as `/proc/cpuinfo` `microcode` field, per physical CPU). Updated dynamically if late microcode loading occurs. Secondary: `/proc/cpuinfo` `microcode` field — verify all cores show the same revision. Kernel log: `dmesg | grep -E 'microcode|Microcode'` — shows loading messages and whether an update was applied at early boot. |
| 9 | Minimum CPU generations | All x86_64 CPUs support microcode updates. The mechanism and format differ between Intel (µcode) and AMD (amd-ucode). Both are loaded via `intel-microcode` or `amd-ucode` packages on Linux. |
| 10 | Security benefit | Correct microcode is the hardware prerequisite for IBRS, eIBRS, IBPB, STIBP, SSBD, MD_CLEAR (VERW), and L1D flush to function as documented. A CPU with a pre-fix microcode version may CPUID-advertise a mitigation feature flag (e.g., `spec_ctrl` for IBRS) but implement it incorrectly or incompletely. Microcode staleness is an invisible vulnerability: the kernel believes mitigations are active, but the hardware is not implementing them correctly. |
| 11 | Performance benefit | Microcode updates occasionally improve performance (bug fixes to instruction execution timing). More commonly they reduce performance by adding overhead to mitigations. Early Spectre v2 microcode updates had significant performance impact on Skylake CPUs; eIBRS microcode for newer CPUs recovered most of this overhead. |
| 12 | Assurance caveats | (1) Microcode version staleness is not directly visible from `/sys/devices/system/cpu/vulnerabilities/`. A stale microcode with kernel-side mitigation code present will show the mitigation as "active" even if the hardware is not executing it correctly. (2) Determining whether a given revision is the "latest" requires consulting vendor-specific microcode revision databases (Intel: `intel-microcode` package; AMD: `amd-ucode` package). There is no in-kernel "stale microcode" indicator. (3) Early-load vs late-load: microcode loaded after CPU bring-up (late loading) may not apply to all CPUs on the physical package and is less reliable than early loading. RHEL/RHEL 10 uses early loading via initrd. (4) Hypervisor-assigned microcode in VMs is entirely under hypervisor control. Guest `/proc/cpuinfo` `microcode` field reflects the virtual microcode version exposed by the hypervisor, which may not correspond to actual hardware microcode. (5) CPU steppings matter: the same CPU model at different steppings requires different microcode packages and has different vulnerability profiles. |
| 13 | Virtualization behavior | In VMs: the `microcode` field in `/proc/cpuinfo` and `/sys/devices/system/cpu/cpu0/microcode/version` reflect whatever the hypervisor exposes. KVM by default exposes the host's actual microcode revision. Some hypervisors synthesize a fixed microcode revision for VM stability. A guest cannot verify that the exposed microcode revision corresponds to real hardware behavior. |
| 14 | Firmware / BIOS / microcode dependency | `{ bios_enable_required: false, microcode_required: true }` — by definition, this feature IS the microcode. BIOS firmware typically applies its own microcode during POST. The Linux kernel's early microcode loader (via initrd) then applies the latest available version, which may be newer than the BIOS-bundled version. Both paths must be functional. |
| 15 | Audit-card relevance | **Important** — Microcode version is a prerequisite check for all Critical/Defensive speculative execution mitigation features. Without knowing the microcode revision, the audit cannot confirm that advertised mitigations are actually implemented at the hardware layer. |
| 16 | Recommended disposition | Keep microcode up to date via package management: `dnf update microcode_ctl` (RHEL/RHEL 10). Verify early loading is configured in the initrd. Establish a monitoring process to detect when new microcode packages are released by the vendor (Intel/AMD security advisories). Compare installed revision against published latest revision for this CPU's stepping. |
| 17 | Software utilization detection method | Read `/proc/cpuinfo` `microcode` field for all cores. Read `/sys/devices/system/cpu/cpu0/microcode/version`. Compare against published `intel-microcode` or `amd-ucode` package release for this Family/Model/Stepping. Run `dmesg | grep microcode` to confirm early loading occurred and whether an update was applied at this boot. |
| 18 | FIPS utilization requirement | Indirect: FIPS-validated cryptographic modules on RHEL 10 depend on correct hardware behavior. If a cryptographic primitive relies on AES-NI, a microcode bug affecting AES-NI instruction behavior (hypothetical) would silently undermine FIPS validation. Microcode correctness is a silent prerequisite for FIPS module guarantees. |
| 19 | Active mitigation status path | Microcode version affects all these paths: `/sys/devices/system/cpu/vulnerabilities/spectre_v2` — requires IBRS or eIBRS microcode. `/sys/devices/system/cpu/vulnerabilities/mds` — requires MD_CLEAR microcode. `/sys/devices/system/cpu/vulnerabilities/l1tf` — PTE inversion mitigates kernel variant; hypervisor L1D flush requires correct microcode. `/sys/devices/system/cpu/vulnerabilities/srbds` — SRBDS_MSR or VERW depends on microcode. `/sys/devices/system/cpu/vulnerabilities/mmio_stale_data` — MMIO vulnerability mitigations are microcode-dependent. |
| 20 | Feature accessible vs advertised | Microcode is always present (CPU cannot boot without it). The version in `/proc/cpuinfo` is the post-load version. If the kernel's initrd early loader finds a newer microcode package, it applies it before the CPU begins normal execution. The version after early load is authoritative. |
| 21 | Guest-vs-host discrepancy risk | **HIGH** — Guest microcode version is entirely under hypervisor control. A guest cannot verify that its `microcode` field reflects real hardware microcode behavior. On bare metal, the value is authoritative. In a VM, it is informational at best. |
| 22 | Notes | Intel publishes microcode revision guidance for each CPU via the `intel-microcode` package and GitHub repository (`intel/Intel-Linux-Processor-Microcode-Data-Files`). AMD publishes via the `linux-firmware` package's `amd-ucode/` directory. The `microcode_ctl` package on RHEL bundles both. Microcode revision numbers are vendor-specific hex values — they are not monotonically increasing across CPU families. Comparing two revision numbers is only meaningful for the same Family/Model/Stepping. |
| 23 | Sources | Intel: `github.com/intel/Intel-Linux-Processor-Microcode-Data-Files`; `/proc/cpuinfo` microcode field docs; ArchWiki Microcode; Baeldung Linux microcode guide; RHEL microcode_ctl package; Linux kernel `arch/x86/kernel/cpu/microcode/` |

---

## Description

CPU microcode is a layer of firmware that sits between the CPU's hardware circuits and the instruction set architecture (ISA). It translates complex x86 instructions into sequences of simpler internal micro-operations. Crucially, microcode can be updated after a CPU is manufactured — BIOS/UEFI applies a microcode patch during POST, and the Linux kernel can apply a newer patch during early boot.

Since the discovery of Spectre and Meltdown in 2018, microcode updates have become the primary delivery vehicle for hardware-level security mitigations. New speculative execution vulnerabilities often require microcode to add new MSR-based controls (e.g., IA32_SPEC_CTRL for IBRS/STIBP/SSBD) or to change the behavior of the VERW instruction (for MD_CLEAR).

---

## Why Security Engineers Care

### Microcode is the invisible prerequisite for mitigation correctness

When the Linux kernel reports a mitigation as active in `/sys/devices/system/cpu/vulnerabilities/`, it is reporting:

1. Whether the kernel-side code is present and enabled, and
2. Whether the CPU advertised the relevant CPUID feature bits

It is **not** reporting whether the microcode implementing those feature bits is the version that correctly implements the mitigation.

A CPU with an early Skylake-era microcode may have the `spec_ctrl` CPUID bit set (IBRS support advertised) but implement IBRS with a bug that was fixed in a subsequent microcode revision. The kernel would report "IBRS" as active. The hardware is not correctly implementing it.

This is the "microcode staleness" problem: the vulnerability sysfs interface can show "Mitigation: IBRS" even when the microcode is too old to provide the full protection.

### Minimum microcode revisions for key mitigations

The relationship between microcode version and mitigation correctness is CPU-generation specific. The general framework:

| Mitigation | Microcode Requirement | Notes |
|---|---|---|
| IBRS | IA32_SPEC_CTRL MSR (spec_ctrl CPUID) | Introduced via microcode update post-2018 for affected CPUs |
| eIBRS | Same MSR; enhanced behavior | Available on Cascade Lake+ / Tiger Lake+ natively; older CPUs need microcode |
| IBPB | spec_ctrl / stibp CPUID bits | Microcode adds the mechanism; kernel issues IBPB on context switch |
| STIBP | Same IA32_SPEC_CTRL bit 1 | Per-sibling prediction isolation; microcode-dependent |
| SSBD | arch_capabilities MSR bit 2 OR via IA32_SPEC_CTRL | Some CPUs require microcode; some have hardware SSBD |
| MD_CLEAR (VERW) | Microcode changes VERW behavior to flush MDS buffers | Critical: without MD_CLEAR microcode, the kernel's VERW instruction does nothing useful for MDS |
| SRBDS (SRBDS_MSR) | Microcode adds IA32_MCU_OPT_CTRL MSR | Specific to Intel Ice Lake and earlier |

**MD_CLEAR is the clearest example of microcode dependency:** On pre-patch systems, VERW was a valid instruction but only verified access rights to a memory segment. After the MD_CLEAR microcode update, VERW also flushes the CPU's internal buffers. A system running the kernel MDS mitigation code on pre-MD_CLEAR microcode is executing VERW instructions that do nothing for MDS protection — the kernel thinks it is mitigating MDS, but the hardware is not.

### Intel and AMD microcode sources

**Intel:**
- Package: `microcode_ctl` (RHEL) or `intel-microcode` (Debian/Ubuntu)
- Source: `github.com/intel/Intel-Linux-Processor-Microcode-Data-Files`
- Microcode file format: `microcode.dat` or `intel-ucode/<CPUID>` binary files
- Release cadence: irregularly, following Intel Product Security Center advisories

**AMD:**
- Included in `linux-firmware` package: `amd-ucode/` directory
- Source: AMD publishes through distribution channels, not a public GitHub-like repository
- RHEL 10 bundles AMD microcode in `linux-firmware` package

---

## Linux Microcode Loading Architecture

```
BIOS/UEFI POST
    → applies BIOS-bundled microcode (may be outdated)
    ↓
Linux early boot (initrd stage)
    → intel-microcode or amd-ucode package files in initrd
    → applied before the CPU initializes AP (Application Processor) cores
    → this is the "early load" — preferred
    ↓
Running kernel
    → /proc/cpuinfo microcode field = post-early-load revision
    → /sys/devices/system/cpu/cpu0/microcode/version = same
    ↓
Optional: late microcode load
    → echo 1 > /sys/devices/system/cpu/microcode/reload
    → less reliable; may not apply uniformly to all cores
    → not recommended for security-critical systems
```

Early loading is critical: many mitigations require microcode to be applied before all CPU cores initialize. Late loading is not guaranteed to apply to all cores and is not recommended for production.

---

## Detection Reference

```
# Current microcode revision (all cores — verify all match)
grep 'microcode' /proc/cpuinfo | sort -u
# Example Intel: microcode : 0xb4
# Example AMD:   microcode : 0x8301034

# Per-socket microcode version (sysfs)
cat /sys/devices/system/cpu/cpu0/microcode/version

# Verify early loading occurred
dmesg | grep -i 'microcode'
# Look for: "microcode: updated to revision 0xXX, date = YYYY-MM-DD"
# Or: "microcode: Current revision: 0xXX"
# Absence of update message = BIOS version is latest, or early loading not configured

# CPU Family/Model/Stepping (needed to look up latest microcode)
grep -m1 'cpu family\|model\|stepping' /proc/cpuinfo

# RHEL: check installed microcode package version
rpm -q microcode_ctl

# Verify initrd includes microcode (RHEL)
lsinitrd /boot/initramfs-$(uname -r).img | grep -E 'intel-ucode|amd-ucode'
```

---

## Staleness Assessment

Determining whether microcode is stale requires:

1. Identify CPU: `grep -m1 'cpu family\|model\|stepping\|vendor_id' /proc/cpuinfo`
2. Determine the microcode revision ID needed for the latest release:
   - Intel: check `github.com/intel/Intel-Linux-Processor-Microcode-Data-Files` releases
   - AMD: check the `linux-firmware` package version in RHEL 10 updates
3. Compare installed revision (from `/proc/cpuinfo`) against the latest available
4. Verify that `microcode_ctl` package is at the latest RHEL 10 version: `dnf check-update microcode_ctl`

There is no automated Linux tool that reports "your microcode is N revisions behind the latest security fix." This is a manual check. RHEL's `microcode_ctl` package maintainers bundle the latest approved microcode per CPU model; keeping the package updated is the operational control.

---

## Audit-Card Findings

| Finding | Severity | Condition |
|---------|----------|-----------|
| Microcode not loaded by early loader | MEDIUM | `dmesg` shows no microcode update message AND BIOS version is not current |
| microcode_ctl package not at latest RHEL version | MEDIUM | `dnf check-update microcode_ctl` shows available update |
| Microcode revision mismatch across cores | HIGH | Different `microcode` values across CPU cores — late load applied inconsistently |
| Guest microcode version reported as stale | LOW (info only) | VM guest — cannot verify; note for host assessment |
| MD_CLEAR-dependent mitigation active + microcode pre-MD_CLEAR | HIGH | Requires manual revision cross-reference; not automatically detectable |

---

## ARM/AArch64 Equivalent

ARM CPU firmware is not "microcode" in the x86 sense — ARM CPUs do not have an x86-style microcode update mechanism. ARM security patches are delivered via:

- **Trusted Firmware-A (TF-A)** — the ARM equivalent of BIOS+microcode for Cortex-A and Neoverse processors. TF-A is updated via firmware packages in the OS.
- **DTB/ACPI** — board firmware (UEFI on server platforms like Neoverse N1/N2)

On RHEL 10 AArch64 (Neoverse N1/N2 target):
- Spectre v2 mitigation status: `/sys/devices/system/cpu/vulnerabilities/spectre_v2` — same interface as x86
- Firmware version: `dmidecode -t 0` (BIOS information) shows UEFI firmware version
- No direct `microcode` field in `/proc/cpuinfo` on ARM; the analogous tracking is firmware package version

---

## Sources

- [Intel Linux Processor Microcode Data Files — GitHub](https://github.com/intel/Intel-Linux-Processor-Microcode-Data-Files)
- [Check CPU Microcode Version on Linux — Georg's Log](https://gms.tf/check-cpu-microcode-version-on-linux.html)
- [Intel and AMD Microcode and Properly Updating Microcode in Linux — Baeldung](https://www.baeldung.com/linux/microcode-intel-amd-update)
- [Microcode — ArchWiki](https://wiki.archlinux.org/title/Microcode)
- [Get CPU Microcode Version on Linux — Lindevs](https://lindevs.com/get-cpu-microcode-version-on-linux)
- [How to install/update Intel microcode firmware on Linux — nixCraft](https://www.cyberciti.biz/faq/install-update-intel-microcode-firmware-linux/)
- Linux kernel `arch/x86/kernel/cpu/microcode/` — kernel microcode loader implementation
- [MDS — Microarchitectural Data Sampling — Linux Kernel docs](https://docs.kernel.org/admin-guide/hw-vuln/mds.html) — MD_CLEAR microcode requirement
