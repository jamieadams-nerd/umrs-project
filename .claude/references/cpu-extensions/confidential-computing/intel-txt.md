# Intel TXT (Trusted Execution Technology)

## 23-Column Matrix Profile

| # | Column | Value |
|---|--------|-------|
| 1 | Feature name | Intel TXT (Trusted Execution Technology) / SMX (Safer Mode Extensions) |
| 2 | Vendor | Intel |
| 3 | CPUID detection | EAX=01H, ECX bit 6 (SMX -- Safer Mode Extensions). SMX is the instruction set underlying TXT. Additional capability detection via GETSEC[CAPABILITIES] after enabling CR4.SMXE (bit 14). Bit 0 of GETSEC[CAPABILITIES] output indicates TXT-capable chipset present. |
| 4 | Linux `/proc/cpuinfo` flag | `smx` |
| 5 | Key instructions | GETSEC (ring-0: SENTER, SEXIT, PARAMETERS, SMCTRL, WAKEUP, CAPABILITIES), ENTERACCS, EXITAC |
| 6 | Introduced | Intel Core 2 Duo (Conroe, 2006); requires chipset support (vPro-class chipsets) |
| 7 | Security relevance | Provides Dynamic Root of Trust for Measurement (DRTM). The GETSEC[SENTER] instruction establishes a measured launch environment (MLE) that is measured into TPM PCRs, creating a trust chain independent of the BIOS/bootloader static boot path. Complements Secure Boot (SRTM) with a dynamic attestation point. |
| 8 | Performance benefit | No general performance benefit. TXT measured launch adds boot-time overhead (SINIT ACM execution, PCR extensions). Normal runtime is unaffected after the MLE is established. |
| 9 | Known vulnerabilities | TXT bypass via SMM (pre-STM era). TOCTOU attacks between SINIT measurement and OS handoff. Intel SA-00391 (2020): TXT-related issues in certain BIOS implementations. Woolen attack (2009, Invisible Things Lab): demonstrated SINIT bypass through chipset configuration manipulation. |
| 10 | Compliance mapping | NIST SP 800-53 SI-7 (Software, Firmware, and Information Integrity), SA-10 (Developer Configuration Management), CM-14 (Signed Components); NIST SP 800-155 (BIOS Integrity Measurement Guidelines); CMMC SI.L2-3.14.1 (Flaw Remediation). |
| 11 | Classification | **Important** |
| 12 | Classification rationale | TXT provides measured boot attestation that strengthens platform integrity verification. It complements Secure Boot but is not required for basic security posture. Classification is Important rather than Critical because: (1) Secure Boot provides a baseline integrity chain without TXT, (2) TXT requires specific chipset support and tboot integration, (3) most UMRS deployments will rely on Secure Boot + IMA rather than TXT-based DRTM. |
| 13 | Linux kernel support | `CONFIG_INTEL_TXT` -- in-tree. Kernel TXT support works with tboot (an open-source pre-kernel loader). tboot is loaded by GRUB as the "kernel" and performs GETSEC[SENTER] to establish DRTM. After MLE measurement, tboot launches the actual Linux kernel. Kernel provides TXT-related APIs for measured launch and late launch. Intel SINIT ACM (Authenticated Code Module) is Intel-signed firmware required for TXT -- processor-specific, must match CPU family. |
| 14 | Detection method (safe Rust) | Check `/proc/cpuinfo` for `smx` flag (Layer 1 -- CPU support). Check `dmesg` for `tboot` messages (Layer 2 -- active measured launch). Check `/sys/kernel/security/tpm0/pcrs` or equivalent for PCR[17-19] non-zero values (attestation evidence). `/sys/kernel/security/txt/` may be present on TXT-enabled systems with appropriate kernel config. |
| 15 | Virtualization confidence | **LIMITED** -- TXT is a host-only technology. GETSEC instructions are ring-0 privileged and not virtualizable. Guests cannot perform TXT measured launch. A guest can verify that the host performed a measured launch only through remote attestation (TPM quote verification). KVM does not expose TXT to guests. VMware vSphere supports TXT for host attestation (vSphere Trust Authority). |
| 16 | ARM/AArch64 equivalent | ARM Trusted Firmware (TF-A) measured boot, ARM Platform Security Architecture (PSA). Different model -- ARM uses ROM-based root of trust rather than dynamic launch. |
| 17 | References | Intel SDM Vol 2D (GETSEC instruction); Linux kernel `Documentation/arch/x86/intel_txt.html`; tboot project (SourceForge); Intel TXT Enabling Guide; NIST SP 800-155 |
| 18 | Disposition when unused | **LEAVE ENABLED** -- TXT/SMX capability has minimal attack surface when not actively used. Unlike SGX, there is no persistent enclave state to exploit. If tboot is not in the boot chain, TXT hardware is dormant. Disabling in BIOS is acceptable but not a priority recommendation. |
| 19 | Software utilization detection | Check boot log (`dmesg`) for tboot/SINIT messages. Check TPM PCR[17] (DRTM and Launch Control Policy), PCR[18] (Trusted OS startup), PCR[19] (OS configuration) for non-zero values. If PCR[17] = 0, no measured launch occurred. `/proc/crypto` is NOT relevant. |
| 20 | FIPS utilization requirement | N/A -- TXT is not a FIPS cryptographic primitive. TPM operations used during measured launch may fall under FIPS if the TPM module is FIPS-validated, but this is a TPM concern, not a TXT concern. |
| 21 | Active mitigation status | No entry in `/sys/devices/system/cpu/vulnerabilities/`. TXT is not a mitigation for a specific vulnerability -- it is an attestation mechanism. |
| 22 | Feature accessible vs advertised | **HEAVILY PLATFORM-GATED.** CPUID `smx` flag indicates CPU support, but TXT requires: (1) TXT-capable chipset (vPro-class), (2) BIOS enablement of TXT, (3) TPM 2.0 present and enabled, (4) matching Intel SINIT ACM for the CPU family, (5) tboot installed and configured in bootloader. BIOS settings: "Intel TXT" or "Trusted Execution" must be enabled. Many BIOS implementations default to Disabled. |
| 23 | Guest-vs-host discrepancy risk | **HIGH** -- Guests have no direct access to TXT. A guest cannot determine whether the host performed a TXT measured launch without remote attestation. Host TXT status is invisible to guest `/proc/cpuinfo` (the `smx` flag shows CPU capability, not active TXT state). |

## TXT Architecture: SRTM vs DRTM

### Static Root of Trust for Measurement (SRTM)
- Trust chain starts at CPU reset in firmware (BIOS/UEFI)
- Each stage measures the next: firmware -> bootloader -> kernel
- Measurements stored in TPM PCRs 0-7
- Weakness: any compromise in the firmware chain breaks the entire trust chain
- UEFI Secure Boot provides integrity verification (signature checks) but not measurement

### Dynamic Root of Trust for Measurement (DRTM)
- TXT provides DRTM via GETSEC[SENTER]
- Trust chain can be re-established at any point during a power cycle
- CPU resets to a known good state, executes Intel-signed SINIT ACM
- SINIT ACM measures the MLE (tboot), which measures the OS
- Measurements stored in TPM PCRs 17-22 (DRTM-specific)
- Advantage: does not depend on firmware integrity -- measurement starts fresh from CPU microcode

### PCR Allocation (DRTM)

| PCR | Contents | Updated By |
|-----|----------|------------|
| 17 | DRTM and Launch Control Policy (LCP) | SINIT ACM |
| 18 | Trusted OS startup code (MLE/tboot) | SINIT ACM |
| 19 | OS configuration (kernel command line, initrd) | tboot |
| 20 | Trusted OS (kernel) | tboot |
| 21 | Reserved / implementation-specific | varies |
| 22 | Reserved / implementation-specific | varies |

## Trust Model

**What must be trusted:**
- Intel CPU microcode (SINIT ACM verification)
- Intel SINIT ACM (Intel-signed, processor-family-specific)
- TPM hardware and firmware (measurement storage and attestation)
- tboot integrity (measured by SINIT)
- CPU reset behavior (hardware guarantee)

**What is NOT trusted:**
- BIOS/UEFI firmware (DRTM bypasses firmware trust)
- Bootloader (GRUB is untrusted -- tboot is the measured entry point)
- DMA-capable devices during launch (DMA protected during SENTER)
- Other software not in the MLE measurement chain

**Trust model weaknesses:**
- SINIT ACM is a black box (Intel-signed, not auditable)
- SINIT ACM must match CPU family -- mismatched ACM causes silent TXT failure
- Pre-STM (SMI Transfer Monitor) systems: SMM code runs outside TXT measurement
- TOCTOU window between measurement and execution can be exploited in theory

## CVE / Vulnerability Table

| ID | Name | Year | CVSS | Impact | Fix Mechanism |
|----|------|------|------|--------|---------------|
| N/A | Woolen/ITL SMM Attack | 2009 | N/A | SMM code outside TXT measurement can subvert measured environment | STM (SMI Transfer Monitor) |
| INTEL-SA-00391 | TXT Advisory | 2020 | 7.2 | Certain BIOS configurations allow TXT bypass | BIOS update |
| N/A | TOCTOU attacks (academic) | Various | N/A | Theoretical gap between SINIT measurement and OS execution | Architectural (reduced by DMA protection during SENTER) |

## Hypervisor Behavior

| Hypervisor | TXT Support | Guest Detection | Notes |
|------------|------------|-----------------|-------|
| KVM | N/A -- host-only | Guest cannot detect host TXT state | `smx` flag in guest cpuinfo reflects CPU capability only |
| VMware vSphere | Host TXT attestation via Trust Authority | Guest relies on vSphere attestation API | vSphere 6.7+ Trust Authority |
| Hyper-V | Host attestation via HGS (Host Guardian Service) | Shielded VMs verify host attestation | Different architecture from TXT |

## Recommended Audit Card Display

```
TXT Status: [Capable/Active/Unavailable]
  CPUID: smx=[yes/no]
  Chipset: [TXT-capable/not detected]
  BIOS: [Enabled/Disabled/Unknown]
  SINIT ACM: [loaded/not found]
  tboot: [active/not in boot chain]
  TPM: [present/absent]
  PCR[17]: [non-zero: measured launch/zero: no DRTM]

  Finding: [NONE / INFO: capable-but-not-configured / INFO: Secure-Boot-only]
  Recommendation: [Consider enabling TXT for DRTM attestation on high-value servers]
```

## Sources

- [Intel SDM Vol 2D: GETSEC Instruction](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html)
- [Linux kernel: Documentation/arch/x86/intel_txt.html](https://docs.kernel.org/arch/x86/intel_txt.html)
- [tboot Project](https://sourceforge.net/projects/tboot/)
- [Intel TXT Enabling Guide](https://www.intel.com/content/dam/www/public/us/en/documents/guides/txt-enabling-guide.pdf)
- [NIST SP 800-155: BIOS Integrity Measurement Guidelines](https://csrc.nist.gov/publications/detail/sp/800-155/final)
- [Gentoo Wiki: Trusted Boot](https://wiki.gentoo.org/wiki/Trusted_Boot)
- [tboot DRTM guide (hawrylko.pl)](https://hawrylko.pl/2021/10/15/tboot_and_drtm.html)
