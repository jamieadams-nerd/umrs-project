# AMD SME / TSME (Secure Memory Encryption / Transparent SME)

## 23-Column Matrix Profile

| # | Column | Value |
|---|--------|-------|
| 1 | Feature name | AMD SME (Secure Memory Encryption) / TSME (Transparent Secure Memory Encryption, also called TMME) |
| 2 | Vendor | AMD |
| 3 | CPUID detection | CPUID function 0x8000001f, EAX bit 0 (SME supported). EBX bits [5:0]: C-bit (encryption bit) position in page table entry. EBX bits [11:6]: physical address space reduction when encryption is enabled. |
| 4 | Linux `/proc/cpuinfo` flag | `sme` (SME capability). TSME does not have a separate cpuinfo flag -- it is BIOS-activated SME applying to all pages. |
| 5 | Key instructions | No new instructions. SME is controlled via: (1) MSR 0xc0010010 (MSR_AMD64_SYSCFG) bit 23 -- memory encryption features enabled, (2) C-bit in page table entries -- per-page encryption control, (3) C-bit in CR3 -- PGD table encryption. |
| 6 | Introduced | AMD EPYC Naples (1st Gen EPYC 7001, 2017). Zen 1 architecture. Available on all subsequent EPYC generations. Also available on some Ryzen PRO processors. |
| 7 | Security relevance | SME encrypts DRAM contents using AES-128 with a hardware-managed ephemeral key. Protects against physical memory attacks: cold boot attacks, DRAM removal, bus snooping, DMA attacks from malicious peripherals. **SME (per-page):** OS controls which pages are encrypted via the C-bit in page table entries. **TSME (transparent):** BIOS enables encryption for ALL memory transparently -- OS and software are unaware. TSME is Intel TME's equivalent. |
| 8 | Performance benefit | Near-zero overhead. AES encryption/decryption in the memory controller (inline, pipelined). No measurable performance impact for compute workloads. DMA requires bounce buffers for encrypted pages (swiotlb), which adds I/O overhead. |
| 9 | Known vulnerabilities | No SME-specific CVEs. SME protects against physical attacks only -- running software has plaintext access. C-bit manipulation by a compromised kernel could selectively decrypt pages (software-level, not a hardware flaw). TSME has no per-page control -- simpler but less flexible. Physical address space reduction (CPUID 0x8000001f EBX[11:6]) limits usable memory when encryption is active. |
| 10 | Compliance mapping | NIST SP 800-53 SC-28 (Protection of Information at Rest), MP-5 (Media Transport). Same compliance scope as Intel TME. |
| 11 | Classification | **Important** |
| 12 | Classification rationale | SME/TSME provides transparent memory encryption against physical attacks. Classification is Important because: (1) does not protect against software attacks, (2) physical security assumptions in most UMRS deployment environments may already mitigate the threat, (3) TSME is a prerequisite for SEV (which uses the same memory controller encryption engine), making it architecturally important even when not a direct security control. |
| 13 | Linux kernel support | **SME:** `CONFIG_AMD_MEM_ENCRYPT`. Kernel activates SME by setting the C-bit in page table entries. Kernel parameter `mem_encrypt=on` activates SME if BIOS has enabled it (MSR_AMD64_SYSCFG bit 23). Kernel parameter `mem_encrypt=off` disables (default varies by distribution). **TSME:** No kernel configuration needed -- BIOS encrypts all memory transparently. Kernel detects active SME/TSME at boot. `dmesg` shows "AMD Memory Encryption Features active: SME" or "AMD Memory Encryption Features active: SEV SME". |
| 14 | Detection method (safe Rust) | Check `/proc/cpuinfo` for `sme` flag (Layer 1 -- CPU support). Check `dmesg` for "AMD Memory Encryption Features active" (Layer 2 -- active encryption). Check `/proc/cmdline` for `mem_encrypt=on/off`. For TSME: if SME is active but no `mem_encrypt=` kernel parameter was set, BIOS likely activated TSME. MSR 0xc0010010 bit 23 (ring-0) indicates firmware enablement. |
| 15 | Virtualization confidence | **TRANSPARENT** -- SME/TSME encrypts host memory. VMs benefit from memory encryption without awareness. Guest VMs see encrypted physical memory at the memory controller level. KVM guests with SEV use the same memory controller but with per-VM keys (ASID-based). Guest cannot directly detect whether host has SME/TSME active without `dmesg` or hypervisor cooperation. |
| 16 | ARM/AArch64 equivalent | No direct ARM equivalent for per-page memory encryption. ARM TrustZone Address Space Controllers can partition memory into secure/non-secure but do not encrypt. ARM CCA Realms provide per-realm encryption (closer to SEV than SME). |
| 17 | References | AMD APM Vol 2 section 15.34; Linux kernel `Documentation/arch/x86/amd-memory-encryption.rst`; AMD Memory Encryption white paper |
| 18 | Disposition when unused | **ENABLE IF AVAILABLE** -- SME/TSME has near-zero performance cost and provides defense-in-depth against physical attacks. There is no reason to leave it disabled on EPYC systems. TSME (BIOS-activated) is preferred for simplicity. SME with `mem_encrypt=on` provides the same protection with OS awareness. |
| 19 | Software utilization detection | Check `dmesg` for active SME messages. Check `/proc/cmdline` for `mem_encrypt=on`. If SEV is active (dmesg), SME is implicitly active (SEV uses the same memory controller). `/proc/crypto` NOT relevant. |
| 20 | FIPS utilization requirement | SME uses AES-128, a FIPS-approved algorithm. The encryption engine is in the memory controller / AMD-SP. AMD's FIPS 140-3 validation for the memory encryption engine varies by platform. |
| 21 | Active mitigation status | No entry in `/sys/devices/system/cpu/vulnerabilities/`. |
| 22 | Feature accessible vs advertised | **BIOS-GATED.** CPUID reports SME support, but BIOS must set MSR_AMD64_SYSCFG bit 23 to enable memory encryption features. Linux cannot enable SME if BIOS has not set this bit. BIOS options: "TSME" (transparent, all memory) or "SME" (OS-controlled per-page). Some BIOS implementations combine these under "Memory Encryption" with sub-options. Physical address space is reduced when SME is active (CPUID 0x8000001f EBX[11:6] reports the reduction in bits). |
| 23 | Guest-vs-host discrepancy risk | **LOW** -- SME/TSME is transparent to guests. SEV provides per-VM encryption that is detectable by the guest. The host SME state does not directly affect guest security posture (SEV provides the VM-relevant encryption). |

## SME vs TSME vs Intel TME Comparison

| Aspect | AMD SME | AMD TSME | Intel TME |
|--------|---------|----------|-----------|
| Granularity | Per-page (C-bit) | All memory (BIOS) | All memory (BIOS) |
| OS awareness | Required (`mem_encrypt=on`) | Not required | Not required |
| Key management | Single ephemeral key | Single ephemeral key | Single ephemeral key |
| Multi-key | No (SEV provides per-VM keys) | No | MKTME provides KeyIDs |
| Algorithm | AES-128 | AES-128 | AES-XTS-128 |
| Address space impact | Reduced (C-bit consumes 1 bit) | Reduced | None (KeyIDs in upper bits) |
| Detection | CPUID + MSR + kernel param | CPUID + MSR (BIOS-only) | CPUID + MSR (BIOS-only) |
| DMA handling | swiotlb bounce buffers | swiotlb bounce buffers | Transparent (all memory) |

## SME Memory Encryption Architecture

### C-bit (Encryption Bit)
- Position reported by CPUID 0x8000001f EBX[5:0] (typically bit 47 on current EPYC)
- Set in page table entries to mark a page as encrypted
- Can also be set in CR3 to encrypt the top-level page table
- Each level of the page table hierarchy can independently have C-bit set or clear
- When C-bit=1: memory controller encrypts on write, decrypts on read
- When C-bit=0: plaintext access (needed for DMA, shared memory)

### SME States (from kernel docs)
1. **Supported:** CPU supports SME (CPUID)
2. **Enabled:** Supported AND MSR_AMD64_SYSCFG bit 23 set (BIOS enabled)
3. **Active:** Supported, Enabled, AND kernel applying C-bit to page tables (SME mask non-zero)

### TSME vs SME Activation Path
- **TSME:** BIOS enables AND activates. All memory is encrypted. Kernel detects active SME at boot. No `mem_encrypt=` parameter needed.
- **SME (BIOS enables only):** BIOS sets MSR bit 23 but does not activate. Kernel must explicitly activate via `mem_encrypt=on`. If kernel does not activate, memory is unencrypted despite BIOS enablement.
- **Neither:** If BIOS does not enable (MSR bit 23 = 0), Linux cannot activate SME regardless of kernel parameters.

## Trust Model

**What must be trusted:**
- AMD CPU hardware (memory controller encryption engine)
- CPU hardware RNG (ephemeral key generation)
- BIOS/firmware (MSR enablement for SME; full activation for TSME)

**What SME/TSME protects against:**
- Cold boot attacks (key is ephemeral, lost on power cycle)
- Physical DRAM removal and offline analysis
- DMA attacks from malicious PCIe devices (encrypted pages are ciphertext to DMA -- but requires IOMMU for complete DMA protection)
- Memory bus snooping

**What SME/TSME does NOT protect against:**
- Software attacks (running OS has plaintext access via page tables)
- Hypervisor memory access (hypervisor controls page tables -- use SEV for VM protection)
- Cache-based side channels
- Kernel memory corruption

## Recommended Audit Card Display

```
SME Status: [Active/Enabled/Supported/Not Available]
  CPUID: sme=[yes/no]
  C-bit position: [bit number]
  BIOS: [Enabled/Disabled]
  Kernel: mem_encrypt=[on/off/not set]
  Mode: [SME (per-page) / TSME (transparent) / Inactive]
  Address space reduction: [N bits]

  Finding: [NONE / LOW: supported-but-not-enabled]
  Recommendation: [Enable TSME in BIOS for physical attack protection]
```

## Sources

- [AMD APM Vol 2: Section 15.34 (Secure Memory Encryption)](https://www.amd.com/content/dam/amd/en/documents/processor-tech-docs/programmer-references/24593.pdf)
- [Linux kernel: AMD Memory Encryption](https://docs.kernel.org/arch/x86/amd-memory-encryption.html)
- [AMD Memory Encryption White Paper](https://www.amd.com/content/dam/amd/en/documents/epyc-business-docs/white-papers/memory-encryption-white-paper.pdf)
- [AMD EPYC TSME Documentation](https://www.amd.com/en/developer/sev.html)
