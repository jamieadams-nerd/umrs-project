# AMD SEV-ES (Secure Encrypted Virtualization -- Encrypted State)

## 23-Column Matrix Profile

| # | Column | Value |
|---|--------|-------|
| 1 | Feature name | AMD SEV-ES (Secure Encrypted Virtualization -- Encrypted State) |
| 2 | Vendor | AMD |
| 3 | CPUID detection | CPUID function 0x8000001f, EAX bit 3 (SEV-ES supported). Requires SEV support (bit 1) as prerequisite. |
| 4 | Linux `/proc/cpuinfo` flag | `sev_es` (in addition to `sev`) |
| 5 | Key instructions | No new user-visible instructions beyond SEV. SEV-ES introduces VMGEXIT (guest-initiated VM exit to hypervisor via GHCB protocol). GHCB (Guest-Hypervisor Communication Block) is shared memory for structured guest-to-hypervisor communication when register state is encrypted. |
| 6 | Introduced | AMD EPYC Rome (2nd Gen EPYC 7002, 2019). Zen 2 architecture. |
| 7 | Security relevance | SEV-ES extends SEV by encrypting the VM's CPU register state (VMSA -- VM Save Area). On standard VM exits, the hypervisor reads guest registers from the VMCB save area. With SEV-ES, the save area is encrypted and integrity-protected -- the hypervisor cannot read or modify guest register values. Guest-to-hypervisor communication uses the GHCB (a shared memory page) where the guest explicitly copies only the registers needed for a specific VM exit. |
| 8 | Performance benefit | Slight additional overhead over SEV due to GHCB protocol for VM exits. Instead of direct VMCB register access, the hypervisor reads GHCB fields. VMGEXIT is typically faster than VMEXIT for exits that need few registers. Performance impact: <2% additional over SEV for most workloads. |
| 9 | Known vulnerabilities | SEV-ES inherits SEV's lack of memory integrity protection (SEVered still applies). Additional attacks targeting GHCB protocol: the hypervisor can inject malicious values into GHCB response fields since GHCB is shared memory. WeSee attack (2024): hypervisor injects malicious #VC exceptions (interrupt 29) to trigger GHCB handler execution, causing the guest to leak sensitive registers. CVE-2023-20592 (CacheWarp) affects SEV-ES. |
| 10 | Compliance mapping | NIST SP 800-53 SC-39 (Process Isolation), SC-28 (Protection of Information at Rest), AC-4 (Information Flow Enforcement). SEV-ES adds register state confidentiality (SC-28 expanded scope). Still lacks integrity -- same limitation as SEV for compliance claims. |
| 11 | Classification | **Important** |
| 12 | Classification rationale | SEV-ES closes the register state exposure gap in SEV but retains the fundamental memory integrity weakness. Classification is Important because: (1) essential intermediate step between SEV and SEV-SNP, (2) widely deployed in cloud (VMware, Azure), (3) still lacks integrity protection that SEV-SNP provides. For UMRS, SEV-SNP remains the minimum acceptable level. |
| 13 | Linux kernel support | **Host:** KVM SEV-ES support since Linux 5.11 (`CONFIG_KVM_AMD_SEV`). VMGEXIT handling in KVM. LAUNCH_UPDATE_VMSA command encrypts initial register state. **Guest:** `CONFIG_AMD_MEM_ENCRYPT` + `CONFIG_SEV_ES` (not a separate config in newer kernels). Guest #VC exception handler processes VMGEXIT/GHCB protocol. Early boot SEV-ES detection for register encryption. |
| 14 | Detection method (safe Rust) | **Host:** Check `/sys/module/kvm_amd/parameters/sev_es` (value: `Y`/`N`). **Guest:** Check `/proc/cpuinfo` for `sev_es` flag. Check `dmesg` for "SEV-ES" in AMD Memory Encryption Features line. MSR 0xc0010131 (SEV_STATUS) bit 1 indicates SEV-ES active. |
| 15 | Virtualization confidence | **CORE PURPOSE** -- SEV-ES is designed exclusively for virtualization. Guest can detect SEV-ES activation via MSR. The guest knows its register state is encrypted. However, guest cannot independently verify that the GHCB protocol is not being manipulated (WeSee attack demonstrates this). Remote attestation (LAUNCH_MEASURE) provides initial state verification but not runtime GHCB integrity. |
| 16 | ARM/AArch64 equivalent | ARM CCA Realms provide register state isolation via Realm Management Monitor (RMM). TrustZone provides world-level register bank switching, which is stronger (hardware register banks) but coarser (two worlds only). |
| 17 | References | AMD SEV-ES white paper; AMD APM Vol 2 section 15.35; Linux kernel SEV-ES documentation; GHCB specification (AMD doc 56421) |
| 18 | Disposition when unused | **LEAVE ENABLED** -- Same as SEV. Dormant hardware has no attack surface. |
| 19 | Software utilization detection | **Host:** `/sys/module/kvm_amd/parameters/sev_es` = `Y`. Active SEV-ES guests visible via KVM APIs. **Guest:** `dmesg` messages, MSR check. `/proc/crypto` NOT relevant. |
| 20 | FIPS utilization requirement | Same as SEV -- AES-128 in memory controller. VMSA encryption uses the same per-VM key. FIPS validation depends on AMD-SP/memory controller certification. |
| 21 | Active mitigation status | No entry in `/sys/devices/system/cpu/vulnerabilities/`. |
| 22 | Feature accessible vs advertised | **BIOS/FIRMWARE-GATED.** CPUID indicates SEV-ES support, but BIOS must enable SEV in AMD-SP firmware. SEV-ES availability depends on AMD-SP firmware version supporting the LAUNCH_UPDATE_VMSA command. Some older firmware on Rome-era boards may report CPUID support but have firmware bugs. |
| 23 | Guest-vs-host discrepancy risk | **MODERATE** -- Same pattern as SEV. Host may have SEV-ES capability but not enable it. A SEV guest (without ES) on a SEV-ES capable host has exposed register state. |

## GHCB Protocol

The Guest-Hypervisor Communication Block is the central mechanism for SEV-ES communication:

### GHCB Structure
- Shared memory page (unencrypted) between guest and hypervisor
- Structure mirrors the VMCB save area layout for register fields
- Guest explicitly copies only needed registers into GHCB before VMGEXIT
- Hypervisor reads GHCB, processes the exit, writes response registers
- Guest reads response and copies back to its private register state

### GHCB Protocol Flow
1. Guest encounters an operation requiring hypervisor assistance (I/O, MSR, CPUID)
2. Guest writes needed register values + exit reason into GHCB
3. Guest executes VMGEXIT
4. Hypervisor reads GHCB, processes request
5. Hypervisor writes result into GHCB
6. Guest reads result from GHCB, resumes execution

### GHCB Security Properties
- **Controlled disclosure:** Guest decides which registers to expose per exit
- **Minimal surface:** Only registers relevant to the exit are shared
- **No integrity guarantee:** GHCB is shared memory -- hypervisor can write arbitrary values
- **No confidentiality for shared data:** Anything in GHCB is visible to hypervisor

### GHCB Weaknesses (WeSee Attack)
The WeSee attack (IEEE S&P 2024) demonstrated that a hypervisor can:
1. Inject interrupt 29 (#VC -- Virtualization Exception) into a SEV-ES/SNP guest
2. Guest #VC handler believes it needs to handle a legitimate VMGEXIT
3. Handler copies sensitive registers into GHCB (leaking data) or processes malicious GHCB values
4. Attack can: leak kTLS keys (NGINX), corrupt kernel data (iptables rules), inject arbitrary code

## VMSA (VM Save Area) Protection

| Component | SEV | SEV-ES |
|-----------|-----|--------|
| Guest memory | Encrypted (per-VM key) | Encrypted (per-VM key) |
| VMCB control area | Unencrypted (hypervisor-managed) | Unencrypted (hypervisor-managed) |
| VMCB save area (registers) | **Unencrypted** (hypervisor can read/modify all guest registers) | **Encrypted + integrity-protected** (hypervisor cannot read or modify) |
| Register transfer on VM exit | Automatic (hardware copies to VMCB) | Via GHCB (guest controls what is shared) |

## CVE / Vulnerability Table

| ID | Name | Year | CVSS | Impact | Fix Mechanism | SEV-ES-Specific? |
|----|------|------|------|--------|---------------|-----------------|
| N/A | SEVered | 2018 | N/A | Memory integrity attack. Applies to SEV-ES (only memory is affected, not registers). | SEV-SNP | No (SEV) |
| CVE-2023-20592 | CacheWarp | 2023 | 5.3 | Cache manipulation reverts VMSA modifications. Can bypass authentication in SEV-ES guests. | Microcode update (EPYC Naples/Rome/Milan) | Yes (targets VMSA) |
| N/A | WeSee | 2024 | N/A | #VC injection causes guest GHCB handler to leak registers or accept malicious input. Demonstrated key extraction and code injection. | Guest kernel hardening (GHCB handler input validation). AMD bulletin AMD-SB-3008. | Yes (targets GHCB protocol) |

## Trust Model

**What must be trusted (beyond SEV):**
- AMD hardware register encryption logic
- Guest #VC exception handler (critical -- WeSee shows this is an attack surface)
- GHCB protocol correctness in guest kernel

**What SEV-ES adds over SEV:**
- Register state confidentiality (hypervisor cannot read guest GPRs, FPU, etc.)
- Register state integrity (hypervisor cannot modify saved registers)

**What SEV-ES still does NOT protect against:**
- Memory integrity attacks (SEVered still applies)
- GHCB manipulation (shared memory)
- Side-channel attacks
- AMD-SP firmware vulnerabilities

## Hypervisor Behavior

| Hypervisor | SEV-ES Support | Guest Detection | Notes |
|------------|---------------|-----------------|-------|
| KVM | Full support since Linux 5.11 | MSR + dmesg | VMGEXIT handling |
| VMware vSphere | Since 7.0 U1 | Guest OS detection | Recommended minimum for vSphere SEV |
| Hyper-V | SEV-SNP preferred | N/A | Azure uses SNP |
| QEMU | Full support | Standard detection | `-object sev-guest,policy=0x5` for SEV-ES |

## Recommended Audit Card Display

```
SEV-ES Status: [Available/Enabled/Active/Not Available]
  CPUID: sev_es=[yes/no]
  KVM: sev_es parameter [Y/N/module not loaded]
  Register State: [Encrypted/Not Encrypted]
  GHCB: [active/N/A]

  Finding: [INFO: SEV-ES without SNP lacks memory integrity]
  Recommendation: [Use SEV-SNP for production; SEV-ES adds register protection only]
```

## Sources

- [AMD SEV-ES White Paper: Protecting VM Register State](https://www.amd.com/content/dam/amd/en/documents/epyc-business-docs/white-papers/Protecting-VM-Register-State-with-SEV-ES.pdf)
- [AMD APM Vol 2: Section 15.35 (SEV-ES)](https://www.amd.com/content/dam/amd/en/documents/processor-tech-docs/programmer-references/24593.pdf)
- [GHCB Specification (AMD doc 56421)](https://docs.amd.com/api/khub/documents/oJly8EPzLO1Bt7ncrkCytw/content)
- [Linux kernel: SEV-ES Support (LWN.net)](https://lwn.net/Articles/836719/)
- [WeSee: Using Malicious #VC Interrupts to Break AMD SEV-SNP (IEEE S&P 2024)](https://arxiv.org/html/2404.03526v1)
- [AMD Security Bulletin AMD-SB-3008: Disrupting AMD SEV-SNP with Interrupts](https://www.amd.com/en/resources/product-security/bulletin/amd-sb-3008.html)
- [KVM Forum 2018: Extending SEV with SEV-ES](https://events19.linuxfoundation.org/wp-content/uploads/2017/12/Extending-Secure-Encrypted-Virtualization-with-SEV-ES-Thomas-Lendacky-AMD.pdf)
