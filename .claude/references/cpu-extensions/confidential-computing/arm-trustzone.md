# ARM TrustZone

## 23-Column Matrix Profile

| # | Column | Value |
|---|--------|-------|
| 1 | Feature name | ARM TrustZone (TEE -- Trusted Execution Environment) |
| 2 | Vendor | ARM |
| 3 | CPUID detection | Not detected via `/proc/cpuinfo` flags. TrustZone is an architectural feature present in all ARMv7-A and ARMv8-A processors (Cortex-A series). Detection: check for `/dev/tee0` or `/dev/teepriv0` device nodes (OP-TEE), or probe the SCR_EL3 (Secure Configuration Register) from firmware. ARM TrustZone-M (Cortex-M series) is a separate, simpler implementation for microcontrollers. |
| 4 | Linux `/proc/cpuinfo` flag | No dedicated flag. TrustZone is assumed present on all Cortex-A processors. `Features:` line in `/proc/cpuinfo` does not enumerate TrustZone. |
| 5 | Key instructions | SMC (Secure Monitor Call -- EL1 to EL3 transition), HVC (Hypervisor Call -- EL1 to EL2). TrustZone is not an instruction set extension but a hardware architecture that splits the CPU into two worlds. |
| 6 | Introduced | ARMv6 (ARM1176JZF-S, 2004). Ubiquitous in ARMv7-A (Cortex-A7/A9/A15/A17) and all ARMv8-A processors. |
| 7 | Security relevance | TrustZone divides the CPU into two execution worlds: **Secure World** (runs trusted firmware and TEE OS) and **Normal World** (runs Linux and applications). The Secure World has exclusive access to protected hardware peripherals and memory regions. The Normal World cannot access Secure World memory or registers. Used for: secure boot chain, key storage, DRM, biometrics, secure element emulation. This is fundamentally different from SGX (per-enclave) or SEV (per-VM) -- TrustZone provides world-level isolation with a single trust boundary. |
| 8 | Performance benefit | No performance benefit. World switching via SMC adds latency (comparable to a syscall + context switch). TrustZone-aware peripherals add no overhead when accessed from the correct world. |
| 9 | Known vulnerabilities | **Extensive attack surface due to vendor TEE implementations.** TrustZone hardware itself is relatively simple and robust, but the Trusted OS and Trusted Applications running in the Secure World are vendor-specific and have significant CVE history. Qualcomm QSEE, Trustonic Kinibi, Huawei iTrustee, Samsung TEEGRIS all have documented vulnerabilities. DMA attacks (2018): DMA-capable devices can bypass TrustZone memory protection if TrustZone Address Space Controller (TZASC) is misconfigured. CVE-2024-0151 (TrustZone-M): zero/sign extension bug in guard functions allows non-secure world to compromise secure application. CVE-2021-35465 (ARMv8-M): VLLDM instruction does not properly handle exceptions, leaking secure context to non-secure handler. See CVE table below. |
| 10 | Compliance mapping | NIST SP 800-53 SC-39 (Process Isolation), SC-28 (Protection of Information at Rest -- for secure storage), SC-12 (Cryptographic Key Establishment -- secure key storage). ARM PSA Certified provides a compliance framework specific to TrustZone-based systems. |
| 11 | Classification | **Important** |
| 12 | Classification rationale | TrustZone is architecturally present on all Cortex-A processors and provides the foundation for platform security (secure boot, key storage). Classification is Important because: (1) UMRS on AArch64 benefits from TrustZone-backed secure boot and key storage, (2) TrustZone itself is not optional (always present), (3) the security value depends entirely on the TEE OS implementation (OP-TEE, QSEE, etc.), (4) attack history is in vendor implementations, not the TrustZone hardware architecture. |
| 13 | Linux kernel support | Linux runs in the Normal World. TEE interface: `CONFIG_TEE` (generic TEE subsystem), `CONFIG_OPTEE` (OP-TEE driver). Device nodes: `/dev/tee0` (Normal World client), `/dev/teepriv0` (supplicant for OP-TEE). Linux communicates with the Secure World via SMC calls, which are routed through the TEE driver. The kernel does not directly access Secure World memory -- all interaction is via the TEE API. |
| 14 | Detection method (safe Rust) | TrustZone presence: assumed on all ARMv7-A/ARMv8-A processors. TEE availability: check for `/dev/tee0` or `/dev/teepriv0` device nodes. OP-TEE version: `cat /sys/bus/tee/devices/optee-ta-mgmt/` or OP-TEE supplicant status. Secure boot state: platform-specific (e.g., UEFI secure boot variables on SBSA-compliant platforms). |
| 15 | Virtualization confidence | **DIFFERENT MODEL** -- TrustZone is orthogonal to virtualization. Under ARMv8-A with Virtualization Extension, the hypervisor runs at EL2 in the Normal World. TrustZone (Secure World at EL3 + S-EL1/S-EL0) is separate from the virtualization hierarchy. VMs in Normal World all share the same TrustZone. A VM cannot get exclusive TrustZone access (unlike SGX enclaves or SEV per-VM keys). ARM CCA (Confidential Compute Architecture) with Realms addresses per-VM isolation -- TrustZone does not. |
| 16 | x86 equivalent comparison | See comparison table below. TrustZone is most often compared to Intel SGX, but the models are fundamentally different. |
| 17 | References | ARM TrustZone documentation; OP-TEE documentation; ARM PSA Certified; Quarkslab TrustZone analysis; Azeria Labs TEE guide |
| 18 | Disposition when unused | **N/A** -- TrustZone cannot be disabled. It is an architectural feature of the CPU. The Secure World always exists; the question is what software runs there (minimal firmware vs full TEE OS). If no TEE OS is loaded, the Secure World runs only ARM Trusted Firmware (TF-A) for secure boot and runtime services. |
| 19 | Software utilization detection | Check for TEE device nodes (`/dev/tee0`, `/dev/teepriv0`). Check for OP-TEE or vendor TEE services (`systemctl status tee-supplicant`). Check loaded Trusted Applications (TA) via TEE API. `/proc/crypto` may show `optee-*` drivers if OP-TEE provides crypto acceleration. |
| 20 | FIPS utilization requirement | N/A for TrustZone hardware itself. If TrustZone-hosted crypto services are used (e.g., OP-TEE crypto TA), those services must be FIPS-validated independently. ARM Crypto Extension (AES, SHA) instructions are available in both worlds. |
| 21 | Active mitigation status | No entry in x86-style `/sys/devices/system/cpu/vulnerabilities/`. ARM-specific vulnerability handling varies by vendor and SoC. |
| 22 | Feature accessible vs advertised | **ALWAYS PRESENT** on Cortex-A processors. No BIOS/firmware gate for TrustZone hardware. However, the TEE OS (OP-TEE, vendor TEE) must be loaded by the secure boot chain. A system with TrustZone hardware but no TEE OS has minimal Secure World functionality (only TF-A runtime services). |
| 23 | Guest-vs-host discrepancy risk | **MODERATE** -- All VMs share the same TrustZone. A guest VM accesses TrustZone services through the TEE driver, which makes SMC calls through the hypervisor. The hypervisor can intercept or block SMC calls, preventing guest access to TrustZone services. Cloud VMs typically do not have direct TrustZone access. |

## TrustZone vs x86 TEE Comparison

| Aspect | ARM TrustZone | Intel SGX | AMD SEV | Intel TDX |
|--------|--------------|-----------|---------|-----------|
| Isolation unit | Entire world (2 worlds) | Per-enclave | Per-VM | Per-VM (Trust Domain) |
| Memory model | Separate address spaces | Enclave Page Cache (EPC) | Per-VM AES key | Per-TD AES key |
| Trust boundary | Secure World vs Normal World | Enclave vs everything else | VM vs hypervisor | TD vs hypervisor |
| Concurrent secure instances | 1 Secure World (shared) | Multiple enclaves | Multiple VMs (ASID) | Multiple TDs |
| Attestation | Platform-specific (PSA) | DCAP/EPID | VCEK/VLEK (SNP) | DCAP (Intel) |
| OS role | Normal World only | Untrusted (enclave bypasses OS) | Untrusted hypervisor (for SEV-SNP) | Untrusted hypervisor |
| Deprecation risk | None (architectural) | Deprecated on consumer CPUs | Active development | Active development |
| Attack surface | Vendor TEE OS + TAs | SGX microarchitecture | GHCB protocol | SEAMCALL/TDCALL |

### Key Architectural Difference

TrustZone provides a single, system-wide trust domain. Every process in the Secure World can access all secure resources. This is simpler but less granular than SGX (which provides per-enclave isolation) or SEV-SNP (per-VM isolation). The implication for multi-tenant systems:
- **TrustZone:** All tenants share one Secure World -- not suitable for multi-tenant isolation
- **SGX:** Each tenant can have independent enclaves
- **SEV-SNP/TDX:** Each tenant gets a hardware-isolated VM

## OP-TEE (Open Portable Trusted Execution Environment)

OP-TEE is the most widely deployed open-source TEE OS for TrustZone:

- **Developed by:** Linaro (originally STMicroelectronics)
- **License:** BSD 2-Clause
- **Architecture:** Runs at S-EL1 (Secure EL1) under TF-A (EL3)
- **Trusted Applications (TAs):** Run at S-EL0, loaded from Normal World filesystem
- **Crypto:** Uses ARM Crypto Extension when available; software fallback otherwise
- **GlobalPlatform TEE API:** OP-TEE implements GP TEE Internal Core API and Client API
- **Linux integration:** `tee-supplicant` daemon in Normal World handles filesystem and RPMB access for the Secure World

### OP-TEE Security Considerations
- TAs are loaded from Normal World filesystem -- integrity must be verified (signed TAs)
- OP-TEE crypto library (LibTomCrypt) had a timing side-channel (CVE-2017-1000413)
- Buffer overflow vulnerabilities in TA interface handling have been found in vendor TEE implementations
- DMA attacks can bypass TZASC if Address Space Controller is misconfigured

## CVE / Vulnerability Table

| ID | Name | Year | CVSS | Impact | Fix Mechanism | TrustZone-Specific? |
|----|------|------|------|--------|---------------|-------------------|
| CVE-2017-1000413 | LibTomCrypt timing leak | 2017 | 5.9 | Timing side-channel in modular exponentiation used by OP-TEE. Can leak private key bits. | OP-TEE update (constant-time implementation) | OP-TEE specific |
| CVE-2018-18068 | RPi TrustZone debug bypass | 2018 | 7.8 | ARM debugging features allow non-secure world to access Secure World memory on Raspberry Pi. | Platform-specific fix | Platform-specific |
| CVE-2021-35465 | VLLDM exception leak | 2021 | 5.5 | ARMv8-M VLLDM instruction does not properly restore FPU context, leaking Secure context to Non-secure handler. Affects Cortex-M33, M35P, M55. | GCC/compiler mitigation | TrustZone-M specific |
| CVE-2024-0151 | Guard function arg extension | 2024 | 6.5 | Zero/sign extension on arguments to secure gateway functions allows compromised non-secure side to crash or compromise secure application. | Compiler update | TrustZone-M specific |
| Various | Qualcomm QSEE vulns | 2014-2023 | Various | Multiple buffer overflows, integer overflows, and privilege escalation in Qualcomm's QSEE TEE implementation. Dozens of CVEs. | Qualcomm firmware updates | Vendor TEE |
| Various | Trustonic Kinibi vulns | 2016-2020 | Various | Multiple vulnerabilities in Trustonic's Kinibi TEE (used in Samsung, Huawei). | Vendor firmware updates | Vendor TEE |

### Vulnerability Pattern Analysis

TrustZone vulnerability history reveals a clear pattern:
1. **TrustZone hardware** itself has very few vulnerabilities (simple hardware mechanism)
2. **Vendor TEE OS implementations** (QSEE, Kinibi, iTrustee) are the primary attack surface
3. **Trusted Application interfaces** (buffer handling, argument validation) are the most common vulnerability class
4. **DMA attacks** bypass TrustZone when TZASC is misconfigured
5. **TrustZone-M** (microcontroller variant) has had compiler/instruction-level bugs

The lesson: TrustZone security depends almost entirely on the software running in the Secure World. The hardware architecture is sound; the implementations are where vulnerabilities concentrate.

## Trust Model

**What must be trusted:**
- ARM CPU hardware (world switching, TZASC enforcement)
- ARM Trusted Firmware (TF-A) at EL3
- TEE OS (OP-TEE, vendor TEE) at S-EL1
- Trusted Applications at S-EL0
- Secure boot chain (Chain of Trust from ROM to TF-A to TEE OS)

**What is NOT trusted:**
- Normal World OS (Linux)
- Normal World applications
- Normal World hypervisor (EL2 in Normal World)
- DMA devices (unless TZASC/SMMU configured)

**Trust model weaknesses:**
- Single Secure World shared by all Normal World processes/VMs
- TEE OS is typically vendor-proprietary (not auditable for most platforms)
- OP-TEE is auditable but depends on platform-specific TF-A configuration
- No hardware attestation mechanism (unlike SGX DCAP or SEV-SNP VCEK)
- Secure World compromise affects ALL Normal World users

## Recommended Audit Card Display

```
TrustZone Status: [Present (architectural)]
  CPU: [Cortex-A series -- TrustZone always present]
  TEE OS: [OP-TEE vX.Y / Vendor TEE / Minimal (TF-A only)]
  TEE device: /dev/tee0 [present/absent]
  Supplicant: tee-supplicant [running/stopped/not installed]
  Trusted Apps: [count loaded or N/A]
  Secure Boot: [verified/not verified/unknown]

  Finding: [NONE / INFO: TrustZone present but no TEE OS loaded]
  Recommendation: [Verify secure boot chain; ensure OP-TEE or vendor TEE is current]
```

## Sources

- [ARM TrustZone Technology](https://developer.arm.com/technologies/trustzone)
- [OP-TEE Documentation](https://optee.readthedocs.io/en/latest/)
- [Quarkslab: Introduction to ARM TrustZone](https://blog.quarkslab.com/introduction-to-trusted-execution-environment-arms-trustzone.html)
- [Azeria Labs: Trusted Execution Environments and TrustZone](https://azeria-labs.com/trusted-execution-environments-tee-and-trustzone/)
- [SoK: Understanding the Prevailing Security Vulnerabilities in TrustZone-assisted TEE Systems (IEEE S&P 2020)](https://ieeexplore.ieee.org/document/9152801/)
- [Quarkslab: Attacking ARM's TrustZone](https://blog.quarkslab.com/attacking-the-arms-trustzone.html)
- [ARM PSA Certified](https://www.psacertified.org/)
- [OP-TEE Security Advisories](https://github.com/OP-TEE/optee_os/security)
