# MCA / RAS / ECC — Machine Check Architecture, Reliability, and ECC Memory

**Category:** 13 — Reliability / Availability / Resilience
**Feature #:** 49–51
**Phase:** 1G
**Date:** 2026-03-18
**Classification:** Important (Informational/Important per sub-feature)

---

## Overview

This document covers three related reliability features documented together because they form a coherent error-detection and reporting stack:

- **MCA (Machine Check Architecture)** — the CPU hardware mechanism for detecting and reporting hardware errors
- **RAS (Reliability, Availability, Serviceability)** — the software and hardware ecosystem that consumes MCA data
- **ECC (Error-Correcting Code memory)** — the memory subsystem component most frequently reported via MCA/EDAC

---

## 23-Column Matrix Profile

### Feature 49: MCA (Machine Check Architecture)

| # | Column | Value |
|---|--------|-------|
| 1 | Feature name | MCA — Machine Check Architecture |
| 2 | Vendor | Both (Intel and AMD); ARM has a comparable mechanism (SError / RAS extensions) |
| 3 | Category | Reliability / Availability / Resilience (Category 13) |
| 4 | Purpose | CPU hardware mechanism for detecting and reporting hardware errors: memory errors, bus errors, cache errors, TLB errors, parity errors. When a machine check occurs, the CPU raises a Machine Check Exception (MCE, #MC) or a Corrected Machine Check Interrupt (CMCI) depending on severity. |
| 5 | Example instructions | RDMSR/WRMSR to MCi_STATUS/MCi_ADDR/MCi_MISC registers. `MCG_CAP` MSR (MSR 179H) reports the number of MCA banks. No user-visible instructions. |
| 6 | CPUID leaf/subleaf/bit | CPUID leaf 01H, EDX bit 7 (`MCE` — Machine Check Exception support), bit 14 (`MCA` — Machine Check Architecture). On modern x86_64 CPUs both bits are always set. |
| 7 | Linux `/proc/cpuinfo` flag | `mce` (Machine Check Exception), `mca` (Machine Check Architecture). Both present on all modern x86_64 systems. |
| 8 | Linux detection — authoritative path | `/dev/mcelog` — legacy interface (deprecated in kernel 5.x+; removed on RHEL 9+). Modern interface: `rasdaemon` reading from kernel tracepoints at `/sys/kernel/debug/tracing/` (requires non-confidentiality lockdown mode). EDAC sysfs: `/sys/devices/system/edac/mc/` — per-memory-controller error counts. Kernel ring buffer: `dmesg | grep -E 'mce|MCE|machine check'`. |
| 9 | Minimum CPU generations | Intel Pentium Pro and later (1995). AMD K7 Athlon and later. All modern x86_64 CPUs implement MCA. The number of MCA banks and their capabilities vary by generation. |
| 10 | Security benefit | Indirect: uncorrected hardware errors (especially memory errors) can cause data corruption that might affect security-relevant data structures, security labels, or audit records. MCA provides early detection before corruption propagates. On CUI systems, a hardware-induced bit flip in a security label or cryptographic key is a silent integrity violation. MCA detection converts silent corruption into a detectable event. |
| 11 | Performance benefit | None directly. MCA error handling (CMCI processing) adds minor interrupt overhead on systems with high corrected error rates. |
| 12 | Assurance caveats | (1) MCA does not prevent errors — it reports them after they occur. Corrected errors (single-bit ECC) are recovered; uncorrected errors typically panic the system. (2) Kernel lockdown (Secure Boot confidentiality mode) prevents `rasdaemon` from accessing debugfs tracing — the error logging daemon cannot read MCA data. Integrity mode lockdown is required for `rasdaemon` to function with Secure Boot. (3) Legacy `/dev/mcelog` is deprecated and absent on RHEL 9/10. Systems relying on `mcelog` for monitoring have a monitoring gap. (4) MCE during early boot or from a VM exit context can cause non-recoverable system failure. |
| 13 | Virtualization behavior | In VMs, MCA injection is a hypervisor function. The guest sees MCE via virtual MCFG registers controlled by the hypervisor. KVM can inject MCEs into guests for testing. Real hardware MCEs on the host may or may not be visible to guests depending on hypervisor policy. |
| 14 | Firmware / BIOS / microcode dependency | `{ bios_enable_required: false, microcode_required: false }` — MCA is always present on x86_64. Microcode updates can add new MCA bank definitions or change error reporting behavior. |
| 15 | Audit-card relevance | **Important** — Active MCA monitoring is a system health prerequisite. A system accumulating corrected memory errors without monitoring is at increasing risk of uncorrected errors that can silently corrupt data before triggering a crash. For CUI assurance, memory integrity matters. |
| 16 | Recommended disposition when unused | Enable `rasdaemon` service. Verify it is running: `systemctl status rasdaemon`. Check EDAC sysfs for corrected error counts. Monitoring is the disposition — no disable/enable choice exists for MCA itself. |
| 17 | Software utilization detection method | `systemctl is-active rasdaemon` — active means MCA events are being logged. `ls /sys/devices/system/edac/mc/` — populated means EDAC memory controller driver is loaded. `journalctl -u rasdaemon` for error history. |
| 18 | FIPS utilization requirement | N/A |
| 19 | Active mitigation status path | N/A — MCA is not a vulnerability mitigation. |
| 20 | Feature accessible vs advertised | Always accessible on x86_64. |
| 21 | Guest-vs-host discrepancy risk | **Moderate** — Guest MCE is hypervisor-injected; does not reflect real hardware error rates. |
| 22 | Notes | RHEL 9 and RHEL 10 ship with `rasdaemon` as the replacement for `mcelog`. `mcelog` is absent from RHEL 9+. On RHEL 10 with Secure Boot in confidentiality lockdown mode, `rasdaemon` requires `lockdown=integrity` kernel parameter to function. |
| 23 | Sources | Intel SDM Vol 3B Ch 15 (MCA); AMD APM Vol 2 Ch 9 (MCA); Linux EDAC docs; rasdaemon GitHub; Red Hat solution for mcelog vs rasdaemon |

---

### Feature 50: RAS (Reliability, Availability, Serviceability)

| # | Column | Value |
|---|--------|-------|
| 1 | Feature name | RAS — Reliability, Availability, Serviceability |
| 2 | Vendor | Both |
| 3 | Category | Reliability / Availability / Resilience (Category 13) |
| 4 | Purpose | The umbrella term for hardware and software features that detect errors, maintain uptime, and enable diagnosis without system downtime. On Linux, the RAS stack includes: MCA (hardware detection), EDAC (kernel drivers for memory controller ECC reporting), `rasdaemon` (userspace logging), and IPMI/BMC integration. |
| 5 | Example instructions | N/A — RAS is an architecture property, not an instruction set. |
| 6 | CPUID leaf/subleaf/bit | N/A at the CPU instruction level. RAS capabilities are enumerated via ACPI SRAT, SMBIOS type 17, and MCA bank capabilities. |
| 7 | Linux `/proc/cpuinfo` flag | No specific flag. RAS presence inferred from `mce` + `mca` flags. |
| 8 | Linux detection — authoritative path | `rasdaemon`: `systemctl status rasdaemon`. EDAC: `/sys/devices/system/edac/mc/<n>/` per memory controller. `/sys/devices/system/edac/mc/<n>/csrow/<n>/` per DIMM rank. Error counters: `/sys/devices/system/edac/mc/mc0/ce_count` (corrected errors), `ue_count` (uncorrected errors). |
| 9 | Minimum CPU generations | Server-class CPUs universally support RAS; consumer CPUs have limited RAS (often no ECC memory support at BIOS level). |
| 10 | Security benefit | Same as MCA: prevents silent data corruption. On CUI systems, unchecked memory errors can corrupt security labels or audit records. RAS monitoring converts silent corruption into actionable log events. |
| 11 | Performance benefit | None — RAS is monitoring overhead. |
| 12 | Assurance caveats | (1) EDAC must have a driver for the specific memory controller chipset to report errors — not all chipsets have upstream EDAC drivers. (2) IPMI/BMC integration requires a separate out-of-band management channel. (3) `rasdaemon` database is at `/var/lib/rasdaemon/ras-mc_event.db` (SQLite); this file should be in the audit trail. |
| 13 | Virtualization behavior | In VMs, EDAC typically reports zero events because the guest does not see real hardware memory controller registers. Hardware RAS events are visible only on bare metal or with hypervisor injection. |
| 14 | Firmware / BIOS / microcode dependency | `{ bios_enable_required: false, microcode_required: false }` for software stack. ECC memory itself requires BIOS/platform support. |
| 15 | Audit-card relevance | **Important** — Is hardware error monitoring active? |
| 16 | Recommended disposition | Enable `rasdaemon`. Review error logs regularly. Establish a corrected error threshold for escalation (e.g., >10 CE/hour = degraded DIMM, begin replacement planning). |
| 17 | Software utilization detection method | `systemctl is-active rasdaemon`. `edac-util -s` (if installed). `sqlite3 /var/lib/rasdaemon/ras-mc_event.db 'select * from mc_event order by timestamp desc limit 10'`. |
| 18 | FIPS utilization requirement | N/A |
| 19 | Active mitigation status path | N/A |
| 20 | Feature accessible vs advertised | Always accessible. |
| 21 | Guest-vs-host discrepancy risk | **HIGH** — Guests do not see real hardware RAS events. |
| 22 | Notes | Red Hat recommends `rasdaemon` over `mcelog` for RHEL 8+. Twitter (now X) published a case study on using `rasdaemon` at scale for hardware reliability monitoring. |
| 23 | Sources | rasdaemon GitHub; Red Hat solution 1412953 (mcelog vs rasdaemon); Twitter/X engineering blog on rasdaemon |

---

### Feature 51: ECC Memory (Platform Interaction)

| # | Column | Value |
|---|--------|-------|
| 1 | Feature name | ECC — Error-Correcting Code Memory (platform interaction) |
| 2 | Vendor | Both (CPU and memory controller interaction) |
| 3 | Category | Reliability / Availability / Resilience (Category 13) |
| 4 | Purpose | ECC memory uses additional data bits to detect and correct single-bit errors and detect double-bit errors in DRAM. The CPU's memory controller reads the ECC syndrome bits on every access, corrects single-bit errors transparently, and escalates double-bit or multi-bit errors via MCA. |
| 5 | Example instructions | None — transparent at the hardware level. |
| 6 | CPUID leaf/subleaf/bit | Not CPUID-detectable. ECC support depends on: (1) CPU memory controller ECC capability, (2) BIOS ECC enablement, (3) ECC DIMMs installed. SMBIOS type 17 (Memory Device) entry indicates ECC type per DIMM. |
| 7 | Linux `/proc/cpuinfo` flag | None. |
| 8 | Linux detection — authoritative path | EDAC sysfs: `/sys/devices/system/edac/mc/mc0/` — presence indicates EDAC driver is tracking this controller. `/sys/bus/edac/devices/` on newer kernels. `edac-util -s` if edac-utils is installed. `dmidecode -t 17 | grep -i 'error correction'` for DIMM ECC type from SMBIOS. `rasdaemon` logs corrected ECC events. |
| 9 | Minimum CPU generations | AMD Zen CPUs support ECC with compatible motherboards and BIOS. Intel Xeon supports ECC universally; Intel Core CPUs officially do not support ECC (though some do in practice with the right motherboard). Server platforms universally support ECC. |
| 10 | Security benefit | ECC prevents bit-flip attacks and naturally occurring DRAM bit errors. Without ECC, a rowhammer attack (CVE-related) can induce DRAM bit flips by repeatedly accessing adjacent rows — potentially flipping bits in security-relevant memory. ECC (specifically DRAM with on-die ECC or LPDDR5 ECC) can detect and correct single-bit rowhammer-induced flips before they are read. |
| 11 | Performance benefit | Slight latency increase (~1–3%) from ECC syndrome check on every memory access. Negligible in practice on modern memory controllers. |
| 12 | Assurance caveats | (1) ECC corrects single-bit errors silently — the correction is not visible without EDAC/MCA monitoring. High corrected error rates are a leading indicator of DIMM failure and must be monitored. (2) ECC does not protect against multi-bit errors (double-bit detect, no correct). DRAM errors from cosmic ray strikes or physical degradation can cause multi-bit errors that ECC cannot correct. (3) ECC does not protect against rowhammer if the rowhammer induces multi-bit flips — the relationship is probabilistic. (4) Consumer Intel CPUs (Core i3/i5/i7/i9) officially do not support ECC; whether to use non-official ECC support is a risk decision. (5) ECC presence cannot be reliably determined from within a VM. |
| 13 | Virtualization behavior | VMs do not see ECC events directly. EDAC events from real hardware appear only on the host. A VM guest cannot determine whether the host uses ECC memory. |
| 14 | Firmware / BIOS / microcode dependency | `{ bios_enable_required: true, microcode_required: false }` — ECC must be enabled in BIOS/UEFI. Some consumer BIOS firmware disables ECC even when CPU and DIMMs support it. Verify SMBIOS type 18 (Memory Array) for ECC type. |
| 15 | Audit-card relevance | **Important** — For CUI systems, ECC memory is a baseline integrity expectation. A DoD assessor reviewing a CUI processing workstation or server expects ECC memory. Consumer-grade non-ECC memory on a CUI system is a notable gap. |
| 16 | Recommended disposition | Use ECC DIMMs and enable ECC in BIOS for all CUI-processing systems. Monitor via `rasdaemon` and establish CE escalation thresholds. |
| 17 | Software utilization detection method | `dmidecode -t 17 | grep 'Error Correction Type'` — should show `Single-bit ECC` or `Multi-bit ECC`. EDAC driver loaded: `ls /sys/devices/system/edac/`. CE count: `cat /sys/devices/system/edac/mc/mc0/ce_count`. |
| 18 | FIPS utilization requirement | N/A directly. However, FIPS environments protecting cryptographic keys depend on memory integrity. ECC is a best practice for hardware protecting FIPS-validated modules. |
| 19 | Active mitigation status path | N/A |
| 20 | Feature accessible vs advertised | BIOS-gated. CPU may support ECC but BIOS may disable it. SMBIOS reports what is installed and configured. |
| 21 | Guest-vs-host discrepancy risk | **EXTREME** — Guests have no visibility into host ECC state. |
| 22 | Notes | Rowhammer and ECC: In-DRAM ECC (on-die, LPDDR5) is a different technology from DIMM ECC and can provide more granular protection. Conventional DIMM ECC corrects errors only when the row is read — rowhammer exploits the window between when the flip occurs and when ECC can observe it. AMD's Transparent Secure Memory Encryption (TSME) in Zen CPUs provides an additional layer complementary to ECC. |
| 23 | Sources | Linux EDAC kernel docs; `edac-util` man page; dmidecode SMBIOS type 17; Red Hat EDAC support article; rasdaemon GitHub; rowhammer research (Google Project Zero) |

---

## Description

### MCA — Error Detection at the CPU Layer

Machine Check Architecture is the x86 hardware mechanism for reporting hardware errors to the operating system. The CPU contains multiple MCA banks — each monitoring a different hardware component (L1 cache, L2 cache, memory controller, bus interface, etc.). When a component detects an error, it writes error information to the bank's MSR registers (MCi_STATUS, MCi_ADDR, MCi_MISC) and either:

- Raises a Corrected Machine Check Interrupt (CMCI) for recoverable errors, or
- Raises a Machine Check Exception (#MC, interrupt vector 18) for uncorrectable errors

CMCI events are handled by the OS without system disruption. MCE events may cause a kernel panic if the error is unrecoverable.

### RAS — The Software Layer

The Linux RAS stack processes MCA events:

```
CPU MCA Bank
     ↓
Kernel MCE handler (arch/x86/kernel/cpu/mcheck/)
     ↓
EDAC subsystem (drivers/edac/)        ← memory controller specific
     ↓
/sys/devices/system/edac/mc/           ← sysfs error counters
     ↓
rasdaemon                              ← userspace logging (modern)
     ↓
/var/lib/rasdaemon/ras-mc_event.db     ← persistent error history
```

On RHEL 9 and RHEL 10, `mcelog` (the legacy tool) has been removed. `rasdaemon` is the supported tool.

### ECC — Memory Integrity

ECC memory adds extra bits per 64-byte cache line to store a Hamming code. The memory controller reads these bits on every memory access and can:

- Silently correct single-bit errors (SECDED — Single Error Correct, Double Error Detect)
- Detect (but not correct) double-bit errors and report them via MCA

On CUI systems, ECC serves as a hardware integrity check for security-critical memory: cryptographic keys, SELinux labels, audit record buffers.

---

## Why Security Engineers Care

### Silent data corruption is a silent integrity violation

Without MCA monitoring, a system can accumulate corrected memory errors (CEs) while appearing fully functional. A DIMM degrading from 0 CEs/day to 1000 CEs/day will eventually produce an uncorrected error — which corrupts data or crashes the system. On a CUI system, if the corrupted memory contained a security label or cryptographic material, the corruption may not be detected before it affects a security decision.

MCA monitoring converts this silent degradation into observable events. `rasdaemon` logging is the operational prerequisite for acting on this data.

### Rowhammer and the ECC defense

The rowhammer technique repeatedly reads a DRAM row to induce charge leakage into adjacent rows, causing bit flips in arbitrary memory — including security-critical structures. ECC provides a probabilistic defense: single-bit rowhammer flips are corrected transparently. Multi-bit rowhammer flips are detected (not corrected) and generate an MCA event.

On CUI systems, ECC is a baseline expectation rather than a defense against a sophisticated targeted attack. Rowhammer mitigations also exist at the DRAM level (TRR — Target Row Refresh) and via kernel page allocator protections.

### Security labels in RAM — integrity under hardware failure

SELinux security labels are stored as xattrs in kernel memory. An MCA-detected uncorrected error affecting the kernel's VFS cache could corrupt a security label. The kernel's MCE handler attempts to offline the affected page before the corruption is read. ECC gives the hardware a chance to correct single-bit errors before they reach the kernel.

This is not a primary attack path but is relevant to the UMRS project's CUI threat model: hardware failures on a CUI system are not purely operational concerns.

---

## RHEL 10 Specifics

- `mcelog` is NOT present on RHEL 10 (removed in RHEL 9)
- `rasdaemon` is the supported RAS logging tool
- `rasdaemon` requires kernel lockdown mode `integrity` (not `confidentiality`) to access debugfs tracing when Secure Boot is active
- EDAC drivers are compiled into the RHEL kernel for major Intel and AMD chipsets
- `edac-utils` package provides `edac-util` for EDAC sysfs reporting

---

## Detection Reference

```
# Check if rasdaemon is running (RAS monitoring active)
systemctl is-active rasdaemon

# Check EDAC memory controller presence
ls /sys/devices/system/edac/mc/
# mc0, mc1, ... = EDAC driver covering those controllers

# Check corrected error counts
cat /sys/devices/system/edac/mc/mc0/ce_count
cat /sys/devices/system/edac/mc/mc0/ue_count

# Check MCA CPU flags
grep -m1 -E '\b(mce|mca)\b' /proc/cpuinfo

# Check ECC type via SMBIOS
dmidecode -t 17 | grep -A2 'Memory Device' | grep 'Error Correction Type'

# View rasdaemon error history
journalctl -u rasdaemon --since "7 days ago"

# Rasdaemon SQLite database (if installed)
# sqlite3 /var/lib/rasdaemon/ras-mc_event.db 'select count(*) from mc_event'
```

---

## Audit-Card Findings

| Finding | Severity | Condition |
|---------|----------|-----------|
| rasdaemon not active | MEDIUM | `systemctl is-active rasdaemon` returns inactive |
| EDAC driver absent | MEDIUM | `/sys/devices/system/edac/mc/` empty |
| Non-ECC memory on CUI server | HIGH | `dmidecode -t 17` shows no ECC on system handling CUI |
| High CE rate | MEDIUM | `ce_count` increasing rapidly (threshold: policy-dependent) |
| mcelog in use on RHEL 10 | LOW | Not possible — mcelog absent; finding: no RAS monitoring if rasdaemon also absent |

---

## Sources

- [Linux Machine Check Architecture Documentation — SUSE Communities](https://www.suse.com/c/understanding-hardware-error-handling-in-linux-mca-explained/)
- [Machine Check Architecture — Wikipedia](https://en.wikipedia.org/wiki/Machine_Check_Architecture)
- [Linux Kernel EDAC Documentation](https://www.kernel.org/doc/html/v4.14/driver-api/edac.html)
- [rasdaemon GitHub Repository](https://github.com/mchehab/rasdaemon)
- [Red Hat: Which of mcelog and rasdaemon should I use?](https://access.redhat.com/solutions/1412953)
- [Red Hat: EDAC Support in RHEL](https://access.redhat.com/solutions/3896)
- [Machine-check exception — ArchWiki](https://wiki.archlinux.org/title/Machine-check_exception)
- [MDS — Microarchitectural Data Sampling — Linux Kernel docs](https://docs.kernel.org/admin-guide/hw-vuln/mds.html)
- [Twitter/X Engineering: How Twitter uses rasdaemon](https://blog.x.com/engineering/en_us/topics/infrastructure/2023/how-twitter-uses-rasdaemon-for-hardware-reliability)
