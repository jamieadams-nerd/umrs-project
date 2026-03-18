# Intel TDX (Trust Domain Extensions)

## 23-Column Matrix Profile

| # | Column | Value |
|---|--------|-------|
| 1 | Feature name | Intel TDX (Trust Domain Extensions) |
| 2 | Vendor | Intel |
| 3 | CPUID detection | Not directly enumerable via standard CPUID from userspace. TDX is enabled via BIOS/firmware and the TDX module (loaded by UEFI). Guest detection: CPUID leaf 0x21 returns TDX identification when running inside a TD. Host detection: TDX module loaded and initialized, visible via KVM capabilities. |
| 4 | Linux `/proc/cpuinfo` flag | No dedicated `/proc/cpuinfo` flag on the host. Inside a TD guest, the kernel detects TDX via CPUID leaf 0x21. |
| 5 | Key instructions | SEAMCALL (host ring-0 to TDX module), TDCALL (guest to TDX module), TDVMCALL (guest to VMM via TDX module). TDX module runs in a new CPU mode called SEAM (Secure Arbitration Mode). |
| 6 | Introduced | Intel Sapphire Rapids (4th Gen Xeon Scalable, 2023). TDX 1.0 shipped with Sapphire Rapids; TDX 1.5 with Emerald Rapids/Granite Rapids (5th Gen, 2024). |
| 7 | Security relevance | Provides hardware-isolated confidential VMs (Trust Domains / TDs). TD memory is encrypted with per-TD keys managed by hardware. CPU register state is protected. VMM/host is excluded from the TD's TCB. Supports remote attestation via Intel's DCAP infrastructure. Addresses the "malicious hypervisor" threat model for multi-tenant cloud. |
| 8 | Performance benefit | No general performance benefit -- adds overhead. TD entry/exit transitions are more expensive than standard VM entry/exit. Memory encryption/decryption adds latency (mitigated by hardware AES engine). Typical overhead: 2-10% depending on workload I/O intensity. |
| 9 | Known vulnerabilities | **Significant pre-release and early audit findings.** Google Project Zero 9-month audit (2023, TDX 1.0): 10 vulnerabilities including CVSS 9.3 ACM privilege escalation. Google Cloud 5-month audit (2025, TDX 1.5): 5 CVEs including CVE-2025-30513 (full TDX compromise). All findings fixed before or shortly after disclosure. See CVE table below. |
| 10 | Compliance mapping | NIST SP 800-53 SC-39 (Process Isolation), SC-28 (Protection of Information at Rest), AC-4 (Information Flow Enforcement), SC-12 (Cryptographic Key Establishment and Management); CMMC SC.L2-3.13.16. |
| 11 | Classification | **Critical/Operational** |
| 12 | Classification rationale | TDX is Critical/Operational for confidential computing deployments. When an organization commits to confidential VMs, TDX (or SEV-SNP) is the hardware root of trust -- its absence means the deployment model is not viable. The early audit findings are concerning but were addressed pre-release. Classification reflects the technology's importance to the deployment model, not unconditional trust in the implementation. |
| 13 | Linux kernel support | **Host:** `CONFIG_INTEL_TDX_HOST` -- merged in Linux 6.16 for KVM host-side support. The TDX module is loaded by UEFI firmware and initialized early in boot. KVM uses SEAMCALL to manage TDs. **Guest:** `CONFIG_INTEL_TDX_GUEST` -- merged in Linux 5.19. Guest kernel handles #VE (Virtualization Exception) for CPUID, MSR, and I/O port accesses. Guest attestation via `/dev/tdx_guest`. |
| 14 | Detection method (safe Rust) | **Host:** Check `/sys/module/kvm_intel/parameters/tdx` (value: `Y` or `1` when enabled). Module parameter `kvm_intel.tdx=1` must be set at boot. Check KVM capabilities via ioctl for TDX support. **Guest (inside TD):** CPUID leaf 0x21 returns TDX signature. Check `/dev/tdx_guest` device node. The kernel sets `X86_FEATURE_TDX_GUEST` internally. `/sys/firmware/tdx/` may be present. |
| 15 | Virtualization confidence | **ASYMMETRIC** -- TDX is specifically designed for virtualization. The guest (TD) has strong guarantees: memory encryption, register protection, attestation. However, the guest must use remote attestation (Intel DCAP) to verify it is running in a genuine TD on genuine Intel hardware. Without attestation, a guest cannot distinguish a real TD from an emulated environment. Host detection is straightforward (KVM parameter + TDX module presence). |
| 16 | ARM/AArch64 equivalent | ARM CCA (Confidential Compute Architecture) with Realms. Different architecture: ARM CCA uses a Realm Management Monitor (RMM) at EL2 rather than a CPU-internal module. Not yet widely deployed. |
| 17 | References | Intel TDX Module specification; Linux kernel `Documentation/arch/x86/tdx.html`; `Documentation/virt/kvm/x86/intel-tdx.html`; Google Project Zero TDX audit report (2023); Google Cloud TDX 1.5 audit (2025); kernel `Documentation/security/snp-tdx-threat-model.rst` |
| 18 | Disposition when unused | **LEAVE ENABLED** -- TDX capability is dormant unless the KVM module is loaded with `tdx=1`. The TDX module in firmware is inert without explicit host-side activation. No attack surface from unused TDX. Disabling in BIOS is acceptable for systems that will never host confidential VMs. |
| 19 | Software utilization detection | **Host:** Check if TDX-capable VMs are running (`virsh list` or KVM fd inspection). `/sys/module/kvm_intel/parameters/tdx` shows enablement. **Guest:** `/dev/tdx_guest` device node present. Attestation report generation possible. `/proc/crypto` is NOT directly relevant (TDX uses hardware crypto engine, not kernel crypto subsystem). |
| 20 | FIPS utilization requirement | TDX memory encryption uses AES-128-XTS with hardware-managed keys. The TDX module's cryptographic operations are part of the Intel CPU's validated boundary. Specific FIPS 140-3 validation status of the TDX module's crypto engine depends on the CPU stepping and Intel's CMVP submissions. Organizations requiring FIPS should verify Intel's validation certificate covers TDX. |
| 21 | Active mitigation status | No entry in `/sys/devices/system/cpu/vulnerabilities/`. TDX is not a mitigation -- it is a confidential computing technology. TDX guests are subject to the same speculative execution vulnerability sysfs entries as normal VMs. |
| 22 | Feature accessible vs advertised | **HEAVILY GATED.** TDX requires: (1) Sapphire Rapids or later Xeon with TDX fuse enabled, (2) BIOS/firmware enablement (TDX module must be loaded into SEAM range by UEFI), (3) sufficient TDX-reserved memory (CMR -- Convertible Memory Ranges), (4) kernel 6.16+ for host support (or backport), (5) `kvm_intel.tdx=1` boot parameter. Many Sapphire Rapids SKUs shipped with TDX fuse disabled. |
| 23 | Guest-vs-host discrepancy risk | **MODERATE** -- A host may have TDX hardware but not enable it for KVM. Guests running as normal VMs on TDX-capable hardware see no TDX indicators. A TD guest will always know it is in a TD (CPUID 0x21). The risk is that a cloud provider claims TDX but does not actually enable it -- remote attestation is the verification mechanism. |

## Kernel Threat Model (from snp-tdx-threat-model.rst)

The Linux kernel's confidential computing threat model for TDX and SEV-SNP defines:

### Security Objectives
1. Preserve confidentiality and integrity of guest private memory and registers
2. Prevent privilege escalation from host into guest kernel

### Primary Assets
1. Guest kernel execution context
2. Guest kernel private memory

### Attack Surface
Any interface exposed from guest kernel to untrusted host:
- Port I/O, MMIO, DMA interfaces
- PCI configuration space access
- VMM-specific hypercalls (TDVMCALL)
- Shared memory pages
- Interrupt injection
- CPUID emulation (via #VE handler)
- Guest firmware, bootloader, kernel image, command line (all untrusted until attested)

### Out of Scope
- Host Denial of Service against guest (host controls resources)
- Physical attacks on the platform
- Attacks on the Intel TDX module itself (treated as TCB)

## CVE / Vulnerability Table

| ID | Name | Year | CVSS | Impact | Fix Mechanism | TDX-Specific? |
|----|------|------|------|--------|---------------|--------------|
| N/A (Google PZ #1) | ACM privilege escalation | 2023 | 9.3 | Incorrect interrupt handling in ACM transition allows arbitrary code execution in privileged ACM mode. Full TDX compromise. | TDX module code fix (pre-release) | Yes |
| N/A (Google PZ #2-10) | Various (9 findings) | 2023 | Various | Input validation, state management issues in TDX module 1.0 | TDX module code fixes (pre-release) | Yes |
| CVE-2025-30513 | TDX 1.5 operator compromise | 2025 | High | Untrusted operator can fully compromise TDX security guarantees | TDX module update | Yes |
| CVE-2025-32007 | TDX 1.5 issue | 2025 | TBD | Privilege escalation | TDX module update | Yes |
| CVE-2025-27940 | TDX 1.5 issue | 2025 | TBD | Information disclosure | TDX module update | Yes |
| CVE-2025-27572 | TDX 1.5 issue | 2025 | TBD | Information disclosure | TDX module update | Yes |
| CVE-2025-32467 | TDX 1.5 issue | 2025 | TBD | Information disclosure | TDX module update | Yes |

### Audit History Analysis

TDX has received more pre-deployment security scrutiny than almost any other confidential computing technology:

1. **2023 (TDX 1.0):** Google Project Zero conducted a 9-month audit. 81 attack vectors analyzed, 10 confirmed vulnerabilities. The CVSS 9.3 ACM finding was severe but caught before production deployment. All findings fixed.

2. **2025 (TDX 1.5):** Google Cloud Security + Intel INT31 conducted a 5-month audit using manual review, custom tools, and AI-assisted analysis. 5 CVEs + 35 additional bugs/weaknesses found. CVE-2025-30513 allowed full compromise.

The pattern is mixed: Intel's willingness to submit TDX to external audit is positive, but the severity of findings (CVSS 9.3 in 1.0, full compromise in 1.5) indicates that the TDX module's code quality requires ongoing scrutiny. The TDX module is not open-source, so community review is limited to Intel-sponsored audits.

## Trust Model

**What must be trusted:**
- Intel CPU hardware (TDX module execution environment)
- Intel TDX module firmware (loaded in SEAM range, Intel-signed)
- Intel attestation infrastructure (DCAP/PCCS for remote attestation)
- UEFI firmware (loads TDX module -- but TDX module verifies its own integrity)
- Guest kernel and software stack (once attested)

**What is NOT trusted:**
- Host OS / hypervisor / VMM (explicitly excluded from TCB)
- Host administrator
- Other VMs / TDs on the same host
- DMA-capable devices (unless using TDX-IO with IOMMU)
- Network infrastructure

**Trust model weaknesses:**
- TDX module is Intel-proprietary (not auditable by end users)
- Intel is both hardware vendor and root of attestation trust (same concern as SGX)
- TDX module update requires firmware update (slow deployment cycle)
- Early audit findings suggest non-trivial implementation complexity
- Shared hardware resources (cache, memory bandwidth) may leak timing information

## Hypervisor Behavior

| Hypervisor | TDX Support | Guest Detection | Notes |
|------------|------------|-----------------|-------|
| KVM | Host support in Linux 6.16+ | CPUID leaf 0x21 in TD guest | `kvm_intel.tdx=1` required |
| QEMU | TD launch via `-machine confidential-guest-support=tdx0` | Standard TD detection | Requires KVM TDX support |
| VMware vSphere | Not yet (as of early 2026) | N/A | Expected in future release |
| Hyper-V | TDX support announced | Guest detection via standard TDX CPUID | Azure confidential VMs |

## Recommended Audit Card Display

```
TDX Status: [Available/Enabled/Not Available]
  CPU: [Sapphire Rapids+ with TDX fuse/TDX not present]
  Firmware: TDX module [loaded/not loaded]
  KVM: tdx parameter [Y/N/module not loaded]
  Active TDs: [count or N/A]

  Attestation: [DCAP configured/not configured]
  TDX Module Version: [version string]

  Finding: [NONE / INFO: available-not-enabled / INFO: enabled-no-active-TDs]
  Recommendation: [Enable TDX for confidential VM workloads; verify attestation infrastructure]
```

## Sources

- [Intel TDX Module Specification](https://www.intel.com/content/www/us/en/developer/tools/trust-domain-extensions/overview.html)
- [Linux kernel: Documentation/arch/x86/tdx.html](https://docs.kernel.org/arch/x86/tdx.html)
- [Linux kernel: Documentation/virt/kvm/x86/intel-tdx.html](https://docs.kernel.org/virt/kvm/x86/intel-tdx.html)
- [Linux kernel: Documentation/security/snp-tdx-threat-model.rst](https://docs.kernel.org/security/snp-tdx-threat-model.html)
- [Google Project Zero TDX Audit Report (April 2023)](https://services.google.com/fh/files/misc/intel_tdx_-_full_report_041423.pdf)
- [Google Project Zero Blog: Technical Report into Intel TDX](https://googleprojectzero.blogspot.com/2023/04/technical-report-into-intel-tdx.html)
- [Intel TDX Security Research and Assurance](https://www.intel.com/content/www/us/en/developer/articles/technical/software-security-guidance/technical-documentation/tdx-security-research-and-assurance.html)
- [Intel TDX Guest Hardening Strategy](https://intel.github.io/ccc-linux-guest-hardening-docs/tdx-guest-hardening.html)
- [Phoronix: Intel TDX Host Support Merged for Linux 6.16](https://www.phoronix.com/news/Intel-TDX-Host-KVM-Linux-6.16)
