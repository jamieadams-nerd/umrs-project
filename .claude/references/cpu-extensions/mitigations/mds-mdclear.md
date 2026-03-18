# MDS Mitigations / MD_CLEAR (VERW-Based Buffer Clearing)

## 23-Column Matrix Profile

| # | Column | Value |
|---|--------|-------|
| 1 | Feature name | MDS Mitigations / MD_CLEAR (VERW-based CPU buffer clearing). Covers: MDS (MSBDS, MFBDS, MLPDS, MDSUM), TAA (TSX Asynchronous Abort), SRBDS (Special Register Buffer Data Sampling), MMIO Stale Data, RFDS (Register File Data Sampling), GDS (Gather Data Sampling) |
| 2 | Vendor | Intel (AMD processors not affected by MDS/TAA/SRBDS/MMIO/GDS; RFDS affects Intel Atom only) |
| 3 | CPUID detection | **MD_CLEAR:** CPUID EAX=07H, ECX=0, EDX bit 10 (MD_CLEAR). When set, VERW instruction clears affected CPU buffers. **MDS_NO:** IA32_ARCH_CAPABILITIES MSR bit 5 -- when set, processor not affected by MDS. **TAA_NO:** IA32_ARCH_CAPABILITIES MSR bit 8 -- when set, not affected by TAA. **FBSDP_NO/PSDP_NO/SBDR_SSDP_NO:** IA32_ARCH_CAPABILITIES bits 13-15 -- MMIO stale data immunity. **FB_CLEAR:** IA32_ARCH_CAPABILITIES bit 17 -- VERW clears fill buffers (for MMIO). **GDS_NO/GDS_CTRL:** IA32_ARCH_CAPABILITIES bits for GDS. **RFDS_NO/RFDS_CLEAR:** IA32_ARCH_CAPABILITIES bits 27-28. |
| 4 | Linux `/proc/cpuinfo` flag | `md_clear` (enumerated when VERW buffer clearing is available). Also check for absence of bug flags: `mds`, `taa`, `srbds`, `mmio_stale_data`, `gds`, `rfds` |
| 5 | Key instructions | **VERW** -- the obsolete segment descriptor verification instruction is repurposed by microcode to clear CPU internal buffers (store buffer, fill buffer, load port, etc.). The instruction is executed by the kernel at privilege transitions. No new instructions were added. |
| 6 | Introduced | **MD_CLEAR microcode:** Intel, May 2019 (initial MDS fix). Extended for TAA (November 2019), SRBDS (June 2020), MMIO stale data (June 2022), GDS (August 2023), RFDS (March 2024). Each vulnerability required additional microcode updates to ensure VERW clears the newly-identified buffer. |
| 7 | Security relevance | MDS-class vulnerabilities allow unprivileged code to sample data from CPU internal buffers (store buffer, fill buffer, load ports) used by other security domains. These buffers contain recently-processed data from kernel, other processes, or other VMs. VERW-based clearing before privilege transitions (kernel-to-user, host-to-guest) ensures stale data is scrubbed. On SMT systems, cross-thread attacks via shared buffers are possible -- full mitigation requires either SMT disabled or VERW + SMT-aware scheduling. |
| 8 | Performance benefit | None -- VERW execution at every kernel-to-user and host-to-guest transition adds overhead. Typical impact: 1-5% for CPU-bound workloads, higher for syscall-heavy workloads. With SMT disabled for full protection, impact can reach 30-50%. |
| 9 | Known vulnerabilities | MDS (CVE-2018-12126/12127/12130, CVE-2019-11091), TAA (CVE-2019-11135), SRBDS (CVE-2020-0543), MMIO Stale Data (CVE-2022-21123/21125/21166), GDS (CVE-2022-40982), RFDS (CVE-2023-28746). The recurring pattern: each new vulnerability reveals a previously-unknown buffer or data path that VERW did not clear, requiring a new microcode update. |
| 10 | Compliance mapping | NIST SP 800-53 SC-39 (Process Isolation), SI-16 (Memory Protection), CM-6 (Configuration Settings -- microcode currency); CMMC SC.L2-3.13.10; NSA RTB (Defense in Depth) |
| 11 | Classification | **Critical/Defensive** |
| 12 | Classification rationale | MDS-class attacks allow sampling of arbitrary data from CPU internal buffers across all privilege boundaries (user-to-kernel, guest-to-host, process-to-process via SMT). Without mitigation, any co-resident unprivileged code can potentially read kernel secrets, crypto keys, or other processes' data. The attack does not require the attacker to control what data is leaked -- it is a sampling attack against whatever the CPU recently processed. |
| 13 | Linux kernel support | Kernel detects affected CPUs and enables VERW at kernel-to-user returns and before VM entry. For CPUs affected by MDS but not L1TF, VERW is also executed at idle transitions (C-state entry) on SMT systems. Boot parameters: `mds=`, `tsx_async_abort=`, `mmio_stale_data=`, `gather_data_sampling=`, `reg_file_data_sampling=`, `srbds=`. Global toggle: `mitigations=off`. |
| 14 | Detection method (safe Rust) | Parse `/proc/cpuinfo` for `md_clear` flag. Read multiple sysfs files under `/sys/devices/system/cpu/vulnerabilities/`: `mds`, `tsx_async_abort`, `srbds`, `mmio_stale_data`, `gather_data_sampling`, `reg_file_data_sampling`. Each reports independent mitigation status. |
| 15 | Virtualization confidence | **HIGH RISK** -- Guest vulnerability sysfs reflects the guest kernel's view, which depends on the hypervisor exposing MD_CLEAR. If the host microcode is updated but hypervisor does not expose MD_CLEAR CPUID bit to guest, the guest sees "Vulnerable: Clear CPU buffers attempted, no microcode" -- a best-effort mode that works if host microcode is present but provides no guarantee. SRBDS sysfs in a VM shows "Unknown: Dependent on hypervisor status" because the guest cannot determine host microcode status. |
| 16 | ARM/AArch64 equivalent | ARM processors are not affected by Intel MDS-class vulnerabilities. No equivalent mitigation needed. |
| 17 | References | Intel MDS advisory (INTEL-SA-00233); Intel TAA advisory (INTEL-SA-00270); Intel SRBDS advisory (INTEL-SA-00320); Intel MMIO advisory (INTEL-SA-00615); Intel GDS advisory (INTEL-SA-00828); Intel RFDS advisory (INTEL-SA-00898); Linux kernel `mds.rst`, `tsx_async_abort.rst`, `special-register-buffer-data-sampling.rst`, `processor_mmio_stale_data.rst`, `gather_data_sampling.rst`, `reg-file-data-sampling.rst` |
| 18 | Disposition when unused | **CRITICAL FINDING** -- If any MDS-class sysfs file shows `Vulnerable` on an affected processor, CPU buffer data is exposed across all privilege boundaries. Immediate microcode update and kernel configuration required. `Vulnerable: Clear CPU buffers attempted, no microcode` is acceptable only temporarily in virtualized environments where host has microcode. |
| 19 | Software utilization detection | Read all sysfs files under `/sys/devices/system/cpu/vulnerabilities/` for MDS-class status. Look for `Mitigation:` prefix in each. Any `Vulnerable` value is a finding. See per-vulnerability sysfs values below. |
| 20 | FIPS utilization requirement | **Indirectly relevant:** SRBDS specifically affects RDRAND, RDSEED, and SGX EGETKEY instructions. On FIPS systems using hardware RNG, unmitigated SRBDS allows cross-core sampling of random number generator output, potentially compromising key material. |
| 21 | Active mitigation status | Multiple sysfs files -- see Sysfs Reference below |
| 22 | Feature accessible vs advertised | MD_CLEAR requires microcode update. The CPUID bit is added by microcode, not present in original silicon. Each successive MDS-class vulnerability requires a NEW microcode update to extend VERW's clearing scope. A system with old microcode may show MD_CLEAR for the original MDS fix but still be vulnerable to later-discovered variants (TAA, SRBDS, MMIO, GDS, RFDS). |
| 23 | Guest-vs-host discrepancy risk | **HIGH** -- Guest sees MD_CLEAR only if hypervisor exposes it. Host microcode version determines actual protection scope. Guest cannot verify which MDS variants the host's VERW actually clears. SRBDS explicitly returns "Unknown: Dependent on hypervisor status" in VMs. |

## Sysfs Reference (Per-Vulnerability)

### /sys/devices/system/cpu/vulnerabilities/mds

| Value | Meaning |
|-------|---------|
| `Not affected` | Processor not vulnerable (IA32_ARCH_CAPABILITIES.MDS_NO=1 or non-Intel) |
| `Vulnerable` | Vulnerable, no mitigation |
| `Vulnerable: Clear CPU buffers attempted, no microcode` | Best-effort mitigation without confirmed microcode |
| `Mitigation: Clear CPU buffers` | Microcode present, VERW clearing active |

Appended SMT status: `SMT vulnerable`, `SMT mitigated`, `SMT disabled`, `SMT Host state unknown`

### /sys/devices/system/cpu/vulnerabilities/tsx_async_abort

| Value | Meaning |
|-------|---------|
| `Not affected` | Processor not vulnerable (TAA_NO=1 or no TSX) |
| `Vulnerable` | Vulnerable, no mitigation |
| `Vulnerable: Clear CPU buffers attempted, no microcode` | Best-effort mode |
| `Mitigation: Clear CPU buffers` | VERW clearing active, TSX still enabled |
| `Mitigation: TSX disabled` | TSX disabled entirely |

### /sys/devices/system/cpu/vulnerabilities/srbds

| Value | Meaning |
|-------|---------|
| `Not affected` | Processor not vulnerable |
| `Vulnerable` | Vulnerable, mitigation disabled |
| `Vulnerable: No microcode` | Vulnerable, no microcode fix |
| `Mitigation: Microcode` | Microcode serializes RDRAND/RDSEED access |
| `Mitigation: TSX disabled` | Vulnerable only with TSX; TSX is disabled |
| `Unknown: Dependent on hypervisor status` | Running in VM, cannot determine host status |

### /sys/devices/system/cpu/vulnerabilities/mmio_stale_data

| Value | Meaning |
|-------|---------|
| `Not affected` | Processor not vulnerable |
| `Vulnerable` | Vulnerable, no mitigation |
| `Vulnerable: Clear CPU buffers attempted, no microcode` | Best-effort mode |
| `Mitigation: Clear CPU buffers` | VERW clearing active |
| `Unknown: No mitigations` | Out of servicing period, status unknown |

Appended SMT status as with MDS.

### /sys/devices/system/cpu/vulnerabilities/gather_data_sampling

| Value | Meaning |
|-------|---------|
| `Not affected` | Processor not vulnerable (GDS_NO=1) |
| `Vulnerable` | Vulnerable, mitigation disabled |
| `Vulnerable: No microcode` | No GDS microcode fix |
| `Mitigation: AVX disabled, no microcode` | AVX disabled as software-only mitigation |
| `Mitigation: Microcode` | Microcode mitigation active |
| `Mitigation: Microcode (locked)` | Microcode mitigation active and locked (cannot be disabled by guest) |
| `Unknown: Dependent on hypervisor status` | Running in VM |

### /sys/devices/system/cpu/vulnerabilities/reg_file_data_sampling

| Value | Meaning |
|-------|---------|
| `Not affected` | Processor not vulnerable (RFDS_NO=1) |
| `Vulnerable` | Vulnerable, no mitigation |
| `Vulnerable: No microcode` | No RFDS microcode fix |
| `Mitigation: Clear Register File` | VERW clearing active |

## SMT Impact

Most MDS variants can be exploited cross-thread on SMT systems because CPU internal buffers are shared between sibling threads. The kernel does NOT disable SMT by default. Full MDS protection requires:

1. VERW-based buffer clearing at privilege transitions (default when microcode available)
2. SMT disabled (`nosmt` or `mds=full,nosmt`) for cross-thread protection

Without SMT disabled, a malicious thread on one sibling can continuously sample data from the other sibling's buffer operations. The kernel mitigates this for kernel-to-user and host-to-guest transitions, but cannot prevent cross-thread user-to-user sampling within the same scheduling quantum.

## Microcode Dependency Chain

Each MDS-class vulnerability requires its own microcode update. The updates are cumulative but a system's effective protection depends on having the LATEST microcode:

| Vulnerability | First Microcode Fix Date | What VERW Clears After Update |
|---------------|--------------------------|-------------------------------|
| MDS (original) | May 2019 | Store buffer, fill buffer, load ports |
| TAA | November 2019 | + TSX-related buffers |
| SRBDS | June 2020 | + Serialized RDRAND/RDSEED access |
| MMIO Stale Data | June 2022 | + Fill buffer stale data from MMIO |
| GDS | August 2023 | + Vector register stale data on gather faults |
| RFDS | March 2024 | + Register file data on Atom cores |

**Key insight for UMRS:** A single microcode version check is insufficient. The system must verify that microcode is recent enough to cover ALL applicable MDS variants. The sysfs per-vulnerability files provide this verification automatically.

## CVE / Vulnerability Table

| ID | Name | Year | CVSS | Scope | Fix |
|----|------|------|------|-------|-----|
| CVE-2018-12126 | MSBDS (Fallout) | 2019 | 6.5 | Store buffer | VERW + microcode |
| CVE-2018-12127 | MLPDS | 2019 | 6.5 | Load ports | VERW + microcode |
| CVE-2018-12130 | MFBDS (RIDL/ZombieLoad) | 2019 | 6.5 | Fill buffers | VERW + microcode |
| CVE-2019-11091 | MDSUM | 2019 | 3.8 | Uncacheable memory | VERW + microcode |
| CVE-2019-11135 | TAA | 2019 | 6.5 | TSX async abort | VERW + microcode + TSX disable option |
| CVE-2020-0543 | SRBDS (CrossTalk) | 2020 | 6.5 | RDRAND/RDSEED/EGETKEY | Microcode serialization |
| CVE-2022-21123 | SBDR | 2022 | 6.1 | Shared buffers read | VERW + microcode |
| CVE-2022-21125 | SBDS | 2022 | 6.1 | Shared buffers sampling | VERW + microcode |
| CVE-2022-21166 | DRPW | 2022 | 6.1 | Device register partial write | VERW + microcode |
| CVE-2022-40982 | GDS (Downfall) | 2023 | 6.5 | Vector registers via gather | Microcode + optional AVX disable |
| CVE-2023-28746 | RFDS | 2024 | -- | Register file (Atom only) | VERW + microcode |

## Kernel Command Line Parameters

| Parameter | Values | Effect |
|-----------|--------|--------|
| `mds=` | `full`, `full,nosmt`, `off` | MDS mitigation control |
| `tsx_async_abort=` | `full`, `full,nosmt`, `off` | TAA mitigation control |
| `tsx=` | `on`, `off`, `auto` | TSX feature control (disabling TSX mitigates TAA) |
| `mmio_stale_data=` | `full`, `full,nosmt`, `off` | MMIO stale data mitigation |
| `srbds=` | `off` | Disable SRBDS mitigation for RDRAND/RDSEED performance |
| `gather_data_sampling=` | `off`, `force` | GDS mitigation; `force` disables AVX if no microcode |
| `reg_file_data_sampling=` | `on`, `off` | RFDS mitigation |
| `mitigations=off` | (global) | Disables all of the above |

## UMRS Posture Signal Connection

**IndicatorId::Mitigations (Critical):**
- Multiple sysfs files must be checked for comprehensive MDS-class coverage
- ANY `Vulnerable` value in any MDS-class sysfs file is a CRITICAL finding
- `Vulnerable: No microcode` is a HIGH finding requiring microcode update
- `Vulnerable: Clear CPU buffers attempted, no microcode` is an INFORMATIONAL note in VMs (host may have microcode)
- `Unknown: Dependent on hypervisor status` in VMs is an INFORMATIONAL note

**FIPS relevance:**
- SRBDS specifically compromises RDRAND/RDSEED output -- on FIPS systems, unmitigated SRBDS undermines hardware entropy source trustworthiness
- GDS could leak vector register contents from crypto operations

**Proposed new signal:** `IndicatorId::MicrocodeCurrency` -- a composite check that verifies all MDS-class sysfs files show `Mitigation:` or `Not affected`

## Sources

- [Intel MDS Advisory (INTEL-SA-00233)](https://www.intel.com/content/www/us/en/security-center/advisory/intel-sa-00233.html)
- [Intel TAA Advisory (INTEL-SA-00270)](https://www.intel.com/content/www/us/en/security-center/advisory/intel-sa-00270.html)
- [Intel SRBDS Advisory (INTEL-SA-00320)](https://www.intel.com/content/www/us/en/security-center/advisory/intel-sa-00320.html)
- [Intel MMIO Advisory (INTEL-SA-00615)](https://www.intel.com/content/www/us/en/security-center/advisory/intel-sa-00615.html)
- [Intel GDS Advisory (INTEL-SA-00828)](https://www.intel.com/content/www/us/en/security-center/advisory/intel-sa-00828.html)
- [Intel RFDS Advisory (INTEL-SA-00898)](https://www.intel.com/content/www/us/en/security-center/advisory/intel-sa-00898.html)
- [Linux kernel mds.rst](https://docs.kernel.org/admin-guide/hw-vuln/mds.html)
- [Linux kernel tsx_async_abort.rst](https://docs.kernel.org/admin-guide/hw-vuln/tsx_async_abort.html)
- [Linux kernel gather_data_sampling.rst](https://docs.kernel.org/admin-guide/hw-vuln/gather_data_sampling.html)
