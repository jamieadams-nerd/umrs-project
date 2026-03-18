# AMD SEV-SNP (Secure Encrypted Virtualization -- Secure Nested Paging)

## 23-Column Matrix Profile

| # | Column | Value |
|---|--------|-------|
| 1 | Feature name | AMD SEV-SNP (Secure Encrypted Virtualization -- Secure Nested Paging) |
| 2 | Vendor | AMD |
| 3 | CPUID detection | CPUID function 0x8000001f, EAX bit 4 (SEV-SNP supported). Requires SEV (bit 1) and SEV-ES (bit 3) as prerequisites. Segmented RMP: EAX bit 23. Additional leaf 0x80000025 for segmented RMP parameters. |
| 4 | Linux `/proc/cpuinfo` flag | `sev_snp` (in addition to `sev`, `sev_es`) |
| 5 | Key instructions | PVALIDATE (guest validates page state in RMP), RMPUPDATE (hypervisor updates RMP entry), RMPQUERY (query RMP entry), RMPADJUST (guest adjusts RMP permissions at VMPL level). PVALIDATE must be executed at VMPL0. |
| 6 | Introduced | AMD EPYC Milan (3rd Gen EPYC 7003, 2021). Zen 3 architecture. Enhanced in Genoa (4th Gen, Zen 4, 2022) and Turin (5th Gen, Zen 5, 2024). |
| 7 | Security relevance | SEV-SNP adds memory integrity protection via the Reverse Map Table (RMP). The RMP enforces a one-to-one mapping between system physical addresses and guest physical addresses, preventing page remapping attacks (SEVered), replay attacks, and memory aliasing. Provides hardware-enforced attestation via VCEK/VLEK-signed reports. Introduces VMPL (VM Privilege Levels) for in-guest privilege separation (e.g., SVSM at VMPL0). This is the minimum acceptable level for production confidential computing on AMD platforms. |
| 8 | Performance benefit | Additional overhead over SEV-ES due to RMP checks on every memory access. RMP lookup adds ~1-3% overhead. NUMA-aware segmented RMP (Zen 4+) reduces RMP access latency by placing RMP segments on the same NUMA node as the memory they protect. Overall overhead: 5-15% depending on workload. |
| 9 | Known vulnerabilities | CacheWarp (CVE-2023-20592): INVD instruction abuse reverts memory modifications, breaking authentication. WeSee (2024): #VC interrupt injection bypasses SNP GHCB handler. CVE-2024-56161 (2025): insecure hash function in microcode signature verification allows malicious microcode loading, compromising SNP confidentiality. See CVE table below. |
| 10 | Compliance mapping | NIST SP 800-53 SC-28 (Protection of Information at Rest), SC-39 (Process Isolation), AC-4 (Information Flow Enforcement), SC-12 (Cryptographic Key Establishment and Management), SI-7 (Software, Firmware, and Information Integrity via attestation). SEV-SNP provides the integrity protection needed for full SC-28 compliance in virtualized environments. |
| 11 | Classification | **Critical/Operational** |
| 12 | Classification rationale | SEV-SNP is Critical/Operational for confidential computing deployments on AMD platforms. It provides the complete security stack: memory encryption (SEV), register state encryption (SEV-ES), and memory integrity (SNP RMP). When an organization commits to AMD-based confidential VMs, SEV-SNP is the hardware root of trust -- its absence means the deployment model is not viable. The CVE history is concerning but each issue has received fixes; the overall architecture is sound. |
| 13 | Linux kernel support | **Host:** KVM SEV-SNP support (`CONFIG_KVM_AMD_SEV` with SNP support). RMP management: kernel allocates/initializes RMP (BIOS reserves memory). PVALIDATE/RMPUPDATE in KVM. Guest attestation support via firmware commands. **Guest:** `CONFIG_AMD_MEM_ENCRYPT` with SNP. `/dev/sev-guest` device for attestation reports. Guest calls PVALIDATE to accept memory pages. #VC handler for VMGEXIT/GHCB. SVSM support for VMPL0 services (e.g., vTPM). |
| 14 | Detection method (safe Rust) | **Host:** Check `/sys/module/kvm_amd/parameters/sev_snp` (value: `Y`/`N`). Check `dmesg` for RMP initialization messages. Check `/dev/sev` for SNP firmware commands. **Guest:** Check `/proc/cpuinfo` for `sev_snp` flag. Check `/dev/sev-guest` device (attestation interface). Check `dmesg` for "SEV-SNP" in AMD Memory Encryption Features. MSR 0xc0010131 (SEV_STATUS) bit 2 for SNP active. |
| 15 | Virtualization confidence | **STRONG** -- SEV-SNP provides hardware-enforced attestation. Guest can request an attestation report signed by the CPU's VCEK (Versioned Chip Endorsement Key) or VLEK (Versioned Loaded Endorsement Key). The report includes: measurement of initial guest state, SNP firmware version, platform security version, guest policy. Remote verifier can validate the signature against AMD's Key Distribution Service (KDS). This is the strongest guest verification mechanism in the AMD SEV family. |
| 16 | ARM/AArch64 equivalent | ARM CCA (Confidential Compute Architecture) with Realms and Granule Protection Tables (GPT). The GPT serves a similar role to the RMP -- enforcing memory ownership at the hardware level. ARM CCA is not yet widely deployed. |
| 17 | References | AMD SEV-SNP white paper; AMD APM Vol 2 sections 15.34-15.36; Linux kernel `Documentation/virt/coco/sev-guest.html`; kernel `Documentation/arch/x86/amd-memory-encryption.rst`; CacheWarp paper; WeSee paper |
| 18 | Disposition when unused | **LEAVE ENABLED** -- SNP hardware is dormant unless KVM initializes the RMP. The RMP memory reservation is done by BIOS but is inert without KVM SNP activation. No attack surface from unused SNP. |
| 19 | Software utilization detection | **Host:** `/sys/module/kvm_amd/parameters/sev_snp` = `Y`. RMP initialized (dmesg). Active SNP guests via KVM API. **Guest:** `/dev/sev-guest` present. Attestation report generation possible (ioctl on `/dev/sev-guest`). `/proc/crypto` NOT directly relevant. |
| 20 | FIPS utilization requirement | Same as SEV -- AES-128 in memory controller. Attestation report signing uses ECDSA (P-384), which is FIPS-approved. VCEK/VLEK are derived from chip-unique secrets in the AMD-SP. AMD's FIPS 140-3 validation for the PSP and attestation chain varies by platform generation. |
| 21 | Active mitigation status | No entry in `/sys/devices/system/cpu/vulnerabilities/`. |
| 22 | Feature accessible vs advertised | **BIOS/FIRMWARE-GATED.** CPUID indicates SNP support, but BIOS must: (1) enable SEV in AMD-SP firmware, (2) reserve RMP memory (contiguous or segmented), (3) set RMP_BASE/RMP_END MSRs. AMD-SP firmware version must support SNP commands. Minimum firmware version varies by EPYC generation. BIOS options: "SNP" or "Secure Nested Paging" must be enabled separately from SEV base. |
| 23 | Guest-vs-host discrepancy risk | **LOW-MODERATE** -- SNP guests have strong attestation. A guest in an SNP TD can verify its state via attestation report. However, a guest running without SNP on an SNP-capable host has no indication of the missed opportunity (same as SEV/SEV-ES). The discrepancy risk is whether the cloud provider actually enables SNP when they claim to. |

## Reverse Map Table (RMP)

The RMP is the architectural innovation that distinguishes SNP from SEV/SEV-ES:

### Purpose
- Enforces one-to-one mapping between system physical addresses and guest physical addresses
- Prevents hypervisor from remapping guest pages (blocks SEVered attack)
- Prevents page aliasing (one physical page mapped to multiple guest addresses)
- Prevents replay attacks (page contents bound to a specific address)

### Structure (Contiguous RMP)
- System-wide table in physical memory, reserved by BIOS
- One 16-byte entry per 4KB page of assignable DRAM
- Location: RMP_BASE MSR (0xc0010132) to RMP_END MSR (0xc0010133)
- 16KB bookkeeping header followed by RMP entries
- Coverage: `((RMP_END + 1 - RMP_BASE - 16KB) / 16B) * 4KB` of physical memory
- Must cover ALL system memory for Linux to enable SNP

### Segmented RMP (Zen 4+)
- CPUID 0x8000001f EAX bit 23 indicates segmented RMP support
- CPUID 0x80000025 reports segment size parameters
- RMP_CFG MSR (0xc0010136) controls segmented RMP
- Segments placed on same NUMA node as the memory they cover
- Reduces cross-NUMA RMP access latency
- RST (RMP Segment Table): 4KB table with 512 8-byte entries pointing to segments

### RMP Entry Fields
- **Assigned:** page is assigned to a guest
- **VMSA:** page contains VM save area (register state)
- **GPA:** guest physical address this page is mapped to
- **ASID:** which VM owns this page
- **VMPL permissions:** per-VMPL read/write/execute permissions
- **Validated:** guest has accepted this page via PVALIDATE

## Attestation Architecture

### VCEK (Versioned Chip Endorsement Key)
- Per-chip ECDSA (P-384) key derived from chip-unique hardware secret
- Versioned: key derivation includes TCB (Trusted Computing Base) version
- Used to sign attestation reports
- Verifiable via AMD's Key Distribution Service (KDS): `https://kdsintf.amd.com`
- VCEK certificate chain: AMD Root → AMD SEV Signing → VCEK

### VLEK (Versioned Loaded Endorsement Key)
- Alternative to VCEK -- loaded by cloud provider via AMD's Attestation Signing Key (ASK)
- Allows provider to control key rotation without AMD per-chip certificates
- Same attestation report format
- Interchangeable with VCEK from verifier perspective

### Attestation Report Contents
- Platform TCB version (firmware, microcode, SNP versions)
- Guest measurement (hash of initial guest state)
- Guest policy (minimum firmware version, migration policy, debug policy)
- Report data (64 bytes of guest-provided nonce for freshness)
- Signature (VCEK or VLEK ECDSA-P384)

### VMPL (VM Privilege Levels)
- 4 levels: VMPL0 (most privileged) through VMPL3
- Guest OS typically runs at VMPL2 or VMPL3
- VMPL0 reserved for SVSM (Secure VM Service Module)
- SVSM provides: vTPM, PVALIDATE proxy, privilege separation within the guest
- PVALIDATE MUST execute at VMPL0

## CVE / Vulnerability Table

| ID | Name | Year | CVSS | Impact | Fix Mechanism | SNP-Specific? |
|----|------|------|------|--------|---------------|--------------|
| CVE-2023-20592 | CacheWarp | 2023 | 5.3 | INVD instruction abuse reverts memory modifications on single-store granularity. Demonstrated: sudo authentication bypass, ipp-crypto private key extraction. Affects EPYC 1st-3rd gen (Naples/Rome/Milan). Genoa (4th gen) NOT affected. | Microcode update | Yes (targets SEV-ES/SNP VMSA) |
| N/A | WeSee | 2024 | N/A | #VC interrupt injection into SNP guest. Guest GHCB handler processes injected #VC, leaking registers or accepting malicious input. Demonstrated: kTLS key extraction, firewall rule corruption, root shell injection. | Guest kernel hardening (restrict #VC handler). AMD bulletin AMD-SB-3008. | Yes (targets GHCB/#VC) |
| CVE-2024-56161 | Microcode sig bypass | 2025 | 7.2 | Insecure hash function (CRC32) in AMD CPU ROM microcode patch loader signature verification. Local admin with root can load malicious microcode, compromising SNP confidentiality/integrity for all guests. | AMD firmware update. Coordinated disclosure by Google Security Research. | Indirectly (compromises the platform SNP runs on) |
| N/A | SEV-SNP firmware vulns | Various | Various | AMD-SB-3007: multiple SNP firmware vulnerabilities in AMD-SP. Various information disclosure and integrity issues in SNP firmware. | AMD-SP firmware updates | Yes |
| N/A | SEV-SNP ABI issues | Various | Various | AMD-SB-3019: SEV Confidential Computing vulnerability. Platform-specific issues in attestation chain. | Firmware update | Yes |

### Attack Evolution (SNP-specific)

1. **2023 (CacheWarp):** The INVD instruction (flush cache without writeback) allows the hypervisor to revert guest memory modifications with per-store granularity. On EPYC Milan, this can bypass sudo authentication (revert `auth_ok` flag) or extract private keys. Microcode fix disables the vulnerable INVD path. **Genoa and later are NOT affected** (architectural fix).

2. **2024 (WeSee):** Even with SNP's integrity protection, the GHCB protocol remains an attack surface. The #VC handler in the guest kernel must be hardened against injected interrupts. This is a guest kernel software issue, not a hardware flaw, but it highlights the complexity of the SNP software stack.

3. **2025 (CVE-2024-56161):** The most severe finding: the CPU ROM microcode loader used CRC32 (not a cryptographic hash) for signature verification, allowing a local admin to load arbitrary microcode. This undermines ALL CPU security guarantees including SNP. The fix required AMD to issue new firmware and a coordinated disclosure with Google. This is a platform-level vulnerability that affects SNP transitively.

## Trust Model

**What must be trusted:**
- AMD CPU hardware (RMP enforcement, memory encryption, attestation key derivation)
- AMD Secure Processor / PSP (firmware commands, key management, attestation signing)
- AMD-SP firmware (specific version, validated by attestation report TCB fields)
- CPU microcode integrity (CVE-2024-56161 showed this trust was misplaced for a time)
- AMD Key Distribution Service (certificate chain for VCEK verification)
- BIOS/firmware (RMP memory reservation -- but incorrect RMP is caught by hardware)

**What is NOT trusted:**
- Hypervisor / VMM
- Host OS and administrator
- Other VMs/guests
- Physical DRAM and memory bus
- DMA-capable devices (unless using SNP-aware IOMMU)
- Network infrastructure

**Trust model strengths (over SEV/SEV-ES):**
- Memory integrity (RMP prevents remapping, aliasing, replay)
- Hardware-signed attestation (VCEK/VLEK)
- VMPL privilege separation within guest
- Validated page acceptance (guest must PVALIDATE pages)

**Trust model weaknesses:**
- AMD is both hardware vendor and attestation root (same concern as Intel TDX)
- CVE-2024-56161 demonstrated that microcode integrity was weaker than assumed
- GHCB protocol is shared memory -- inherently an attack surface
- AMD-SP firmware is proprietary and not independently auditable
- RMP memory must cover all system RAM -- misconfigurations silently weaken protection

## Hypervisor Behavior

| Hypervisor | SEV-SNP Support | Guest Detection | Notes |
|------------|----------------|-----------------|-------|
| KVM | Full support since Linux 6.x | MSR + /dev/sev-guest + dmesg | Primary open-source platform |
| QEMU | Full support | Standard detection | `-object sev-snp-guest` |
| VMware vSphere | Since 9.0 (ESXi) | Guest OS detection | Lenovo/Dell server validation |
| Hyper-V | Azure confidential VMs | Standard detection | Production deployment at scale |
| AWS | EC2 SNP instances | Standard detection | Production since 2023 |

## Recommended Audit Card Display

```
SEV-SNP Status: [Available/Enabled/Active/Not Available]
  CPUID: sev_snp=[yes/no]
  RMP: [initialized/not initialized]
    Type: [contiguous/segmented]
    Coverage: [full/partial/unknown]
  KVM: sev_snp parameter [Y/N/module not loaded]
  /dev/sev-guest: [present/absent]
  Attestation: [VCEK/VLEK available/not configured]
  Active SNP VMs: [count or N/A]
  Platform TCB: [firmware version]

  Finding: [NONE / HIGH: SNP capable but not enabled for confidential workloads]
  Recommendation: [Enable SEV-SNP and configure attestation for confidential VM deployments]
```

## Sources

- [AMD SEV-SNP White Paper: Strengthening VM Isolation](https://www.amd.com/content/dam/amd/en/documents/epyc-business-docs/white-papers/SEV-SNP-strengthening-vm-isolation-with-integrity-protection-and-more.pdf)
- [AMD APM Vol 2: Sections 15.34-15.36 (SEV-SNP, RMP)](https://www.amd.com/content/dam/amd/en/documents/processor-tech-docs/programmer-references/24593.pdf)
- [Linux kernel: SEV-SNP Guest API](https://docs.kernel.org/virt/coco/sev-guest.html)
- [Linux kernel: AMD Memory Encryption](https://docs.kernel.org/arch/x86/amd-memory-encryption.html)
- [CacheWarp Attack](https://cachewarpattack.com/)
- [CISPA: CacheWarp Discovery](https://cispa.de/en/cachewarp)
- [WeSee: Malicious #VC Interrupts (IEEE S&P 2024)](https://arxiv.org/html/2404.03526v1)
- [CVE-2024-56161: AMD Microcode Signature Vulnerability](https://github.com/google/security-research/security/advisories/GHSA-4xq7-4mgh-gp6w)
- [AMD Security Bulletin AMD-SB-3007](https://www.amd.com/en/resources/product-security/bulletin/amd-sb-3007.html)
- [AMD Security Bulletin AMD-SB-3019](https://www.amd.com/en/resources/product-security/bulletin/amd-sb-3019.html)
- [AMD SNP Attestation (LSS presentation)](https://www.amd.com/content/dam/amd/en/documents/developer/lss-snp-attestation.pdf)
- [SNPGuard: Remote Attestation (arXiv)](https://arxiv.org/html/2406.01186v1)
