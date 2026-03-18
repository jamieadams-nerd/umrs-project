# AMD SEV (Secure Encrypted Virtualization)

## 23-Column Matrix Profile

| # | Column | Value |
|---|--------|-------|
| 1 | Feature name | AMD SEV (Secure Encrypted Virtualization) |
| 2 | Vendor | AMD |
| 3 | CPUID detection | CPUID function 0x8000001f, EAX bit 1 (SEV supported). EBX bits [5:0]: C-bit position in page table entry. ECX bits [31:0]: number of encrypted guests supported simultaneously (ASID count). |
| 4 | Linux `/proc/cpuinfo` flag | `sev` |
| 5 | Key instructions | No new user-visible instructions. SEV is managed via the AMD Secure Processor (AMD-SP / PSP) through a firmware API. Hypervisor communicates with AMD-SP via SEV firmware commands (LAUNCH_START, LAUNCH_UPDATE_DATA, LAUNCH_MEASURE, LAUNCH_FINISH, etc.). |
| 6 | Introduced | AMD EPYC Naples (1st Gen EPYC 7001, 2017). Zen 1 architecture. |
| 7 | Security relevance | SEV encrypts each VM's memory with a unique AES-128 key managed by the AMD Secure Processor (PSP). The hypervisor cannot read guest memory in plaintext. Protects against: hypervisor snooping, physical DRAM attacks, cross-VM memory access. Does NOT encrypt CPU register state (that requires SEV-ES) or provide integrity protection (that requires SEV-SNP). |
| 8 | Performance benefit | Minimal overhead. AES encryption/decryption performed in the memory controller (inline). Typical overhead: <5% for compute workloads. I/O-heavy workloads may see higher overhead due to bounce buffer requirements for DMA (shared memory regions). |
| 9 | Known vulnerabilities | **Significant attack history due to lack of integrity protection.** SEVered (2018): hypervisor manipulates nested page tables to extract all VM plaintext memory. Undeadattack (2019): lack of integrity allows controlled ciphertext substitution. CVE-2021-26311: memory rearrangement attack. See CVE table below. SEV base (without ES/SNP) is considered insufficient for production confidential computing. |
| 10 | Compliance mapping | NIST SP 800-53 SC-28 (Protection of Information at Rest), SC-39 (Process Isolation), AC-4 (Information Flow Enforcement). Note: SEV alone (without SNP) provides confidentiality but NOT integrity -- this limits its compliance value. An auditor should require SEV-SNP for any claim of "hardware-enforced VM isolation." |
| 11 | Classification | **Important** |
| 12 | Classification rationale | SEV base provides memory encryption but its lack of integrity protection has been repeatedly exploited. Classification is Important rather than Critical because: (1) SEVered and related attacks demonstrate that SEV alone is insufficient for a strong confidential computing claim, (2) SEV is the foundation for SEV-ES and SEV-SNP which address these gaps, (3) for UMRS deployments, SEV-SNP is the minimum acceptable level for confidential VM claims. SEV's importance is as the foundation technology that later generations build upon. |
| 13 | Linux kernel support | **Host:** KVM AMD memory encryption support (`CONFIG_KVM_AMD_SEV`). SEV initialization at KVM module load. API: `/dev/sev` device for firmware commands. **Guest:** `CONFIG_AMD_MEM_ENCRYPT` for SME/SEV memory encryption. Kernel handles encryption bit (C-bit) in page tables. DMA must use shared (unencrypted) pages via `swiotlb` bounce buffers. |
| 14 | Detection method (safe Rust) | **Host:** Check `/sys/module/kvm_amd/parameters/sev` (value: `Y` when enabled, `N` when disabled). Check `/dev/sev` device node existence. Check `dmesg` for "SEV supported" or "SEV firmware" messages. **Guest:** Check `/proc/cpuinfo` for `sev` flag. Check `dmesg` for "AMD Memory Encryption Features active: SEV" message. Check `MSR 0xc0010131` bit 0 (SEV active -- requires ring-0). |
| 15 | Virtualization confidence | **CORE PURPOSE** -- SEV is designed exclusively for virtualization. All VMs on a SEV-capable host are either SEV-encrypted or standard (unencrypted). KVM assigns ASIDs to map to per-VM encryption keys. Guest can detect SEV via MSR but cannot independently verify the encryption is hardware-backed without attestation (LAUNCH_MEASURE command provides initial attestation). |
| 16 | ARM/AArch64 equivalent | ARM CCA (Confidential Compute Architecture) Realms provide the closest equivalent. TrustZone provides world-level isolation but not per-VM encryption. |
| 17 | References | AMD SEV developer documentation; AMD APM Vol 2; Linux kernel `Documentation/virt/kvm/x86/amd-memory-encryption.html`; kernel `Documentation/arch/x86/amd-memory-encryption.rst` |
| 18 | Disposition when unused | **LEAVE ENABLED** -- SEV hardware is dormant unless KVM is loaded with SEV support. No attack surface from unused SEV. The AMD-SP firmware runs regardless of SEV enablement. |
| 19 | Software utilization detection | **Host:** Check `/sys/module/kvm_amd/parameters/sev` for `Y`. Check for active SEV guests via KVM APIs. **Guest:** Kernel messages at boot (`dmesg`). `/proc/crypto` is NOT directly relevant (SEV uses memory controller encryption, not kernel crypto subsystem). |
| 20 | FIPS utilization requirement | SEV uses AES-128 encryption, a FIPS-approved algorithm. However, the AES engine is in the AMD-SP/memory controller, which requires its own FIPS 140-3 validation. AMD's FIPS validation status for the PSP/memory encryption engine varies by platform. |
| 21 | Active mitigation status | No entry in `/sys/devices/system/cpu/vulnerabilities/`. SEV is not a vulnerability mitigation. |
| 22 | Feature accessible vs advertised | **BIOS/FIRMWARE-GATED.** CPUID indicates SEV support, but BIOS must enable SEV in AMD-SP firmware configuration. Some BIOS implementations expose "SEV" or "Secure Encrypted Virtualization" options. AMD-SP firmware version must support SEV. ASID allocation is firmware-managed. |
| 23 | Guest-vs-host discrepancy risk | **MODERATE** -- Host may have SEV hardware but not enable it for KVM. Guests on such hosts run without encryption. A guest running with SEV can detect it (MSR), but a guest running without SEV cannot determine if the host supports it. |

## ASID-Based Key Management

SEV uses the hardware Address Space Identifier (ASID) mechanism to associate encryption keys with VMs:

- Each SEV-enabled VM is assigned a unique ASID (1 to max from CPUID 0x8000001f ECX)
- The memory controller uses the ASID to select the per-VM AES key
- Keys are generated and managed by the AMD Secure Processor (PSP)
- Maximum simultaneous encrypted guests = ASID count minus SNP-reserved ASIDs
- ASID 0 is reserved for the hypervisor (encrypted with the SME key or unencrypted)

## CVE / Vulnerability Table

| ID | Name | Year | CVSS | Impact | Fix Mechanism | SEV-Specific? |
|----|------|------|------|--------|---------------|--------------|
| N/A | SEVered | 2018 | N/A | Hypervisor manipulates nested page tables to extract full VM plaintext. Exploits lack of integrity protection in SEV. Extracted 2GB VM memory in practice. | SEV-SNP (architectural fix via RMP) | Yes |
| N/A | Undeadattack | 2019 | N/A | Controlled ciphertext manipulation due to lack of integrity. Allows replaying memory pages. | SEV-SNP | Yes |
| CVE-2021-26311 | Memory rearrangement | 2021 | 2.9 | Memory can be rearranged in guest address space, allowing guest kernel execution in the context of a different guest process. | Firmware update; SEV-SNP recommended | Yes |
| N/A | CrossLine | 2021 | N/A | Exploits shared ASID and VMCB state to break cross-VM isolation. Breaks "security-by-crash" assumption. | Firmware update | Yes |

### SEV Attack Evolution

The SEV base attack history reveals fundamental architectural limitations:

1. **2018 (SEVered):** Demonstrated that memory encryption without integrity is insufficient. The hypervisor controls the nested page table, so it can map any guest physical page to any host physical page, then coerce the guest into revealing plaintext via a service (e.g., web server).

2. **2019 (Undeadattack):** Extended the attack to show ciphertext replay is possible without integrity protection.

3. **2021 (CVE-2021-26311, CrossLine):** Memory rearrangement and cross-VM attacks using the same SEV ASID space.

**Conclusion:** SEV base is an important foundation but should be treated as insufficient for production confidential computing. SEV-ES adds register state protection; SEV-SNP adds integrity protection via the RMP. Only SEV-SNP addresses the full attack surface.

## Trust Model

**What must be trusted:**
- AMD Secure Processor / PSP (key generation, firmware commands)
- AMD CPU hardware (memory controller encryption)
- AMD-SP firmware (SEV API implementation)

**What is NOT trusted:**
- Hypervisor / VMM (cannot read encrypted memory)
- Host OS
- Physical DRAM bus (encrypted)
- Other VMs (different ASID = different key)

**What SEV does NOT protect against:**
- Integrity attacks (page rearrangement, replay -- SEVered, Undeadattack)
- Register state leakage (requires SEV-ES)
- Hypervisor-controlled DMA to shared memory regions
- Side-channel attacks (timing, cache)
- AMD-SP firmware vulnerabilities

## Hypervisor Behavior

| Hypervisor | SEV Support | Guest Detection | Notes |
|------------|------------|-----------------|-------|
| KVM | Full support since Linux 4.16 | MSR + dmesg | `/dev/sev` for firmware commands |
| VMware vSphere | SEV-ES support since 7.0 U1 | Guest OS detection | |
| Hyper-V | Not supported for SEV base | N/A | Azure uses SEV-SNP |
| Xen | Experimental support | Via hypercall | Limited |

## Recommended Audit Card Display

```
SEV Status: [Available/Enabled/Active/Not Available]
  CPUID: sev=[yes/no]
  C-bit: [bit position]
  Max Guests: [ASID count]
  KVM: sev parameter [Y/N/module not loaded]
  /dev/sev: [present/absent]
  Active SEV VMs: [count or N/A]

  Finding: [INFO: SEV without SNP is insufficient for confidential computing]
  Recommendation: [Use SEV-SNP for production; SEV alone lacks integrity protection]
```

## Sources

- [AMD SEV Developer Documentation](https://www.amd.com/content/dam/amd/en/documents/epyc-business-docs/white-papers/memory-encryption-white-paper.pdf)
- [AMD APM Vol 2: Chapter 15.34 (Secure Encrypted Virtualization)](https://www.amd.com/content/dam/amd/en/documents/processor-tech-docs/programmer-references/24593.pdf)
- [Linux kernel: Secure Encrypted Virtualization (SEV)](https://www.kernel.org/doc/html/latest/virt/kvm/x86/amd-memory-encryption.html)
- [Linux kernel: AMD Memory Encryption](https://docs.kernel.org/arch/x86/amd-memory-encryption.html)
- [SEVered Attack (Fraunhofer AISEC)](https://arxiv.org/abs/1805.09604)
- [CrossLine (CCS 2021)](https://yinqian.org/papers/ccs21.pdf)
- [AMD Product Security Bulletin AMD-SB-3011](https://www.amd.com/en/resources/product-security/bulletin/amd-sb-3011.html)
