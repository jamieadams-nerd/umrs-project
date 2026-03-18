# Intel SGX (Software Guard Extensions)

## 23-Column Matrix Profile

| # | Column | Value |
|---|--------|-------|
| 1 | Feature name | Intel SGX (Software Guard Extensions) |
| 2 | Vendor | Intel |
| 3 | CPUID detection | EAX=07H, ECX=0: EBX bit 2 (SGX supported). EAX=12H, ECX=0: EAX bits [3:0] (SGX1=bit 0, SGX2=bit 1). EAX=12H, ECX=2+: EPC section enumeration (base, size). |
| 4 | Linux `/proc/cpuinfo` flag | `sgx` (SGX1), `sgx_lc` (SGX Launch Control) |
| 5 | Key instructions | ENCLS (ring-0: ECREATE, EADD, EINIT, EREMOVE, EWB), ENCLU (ring-3: EENTER, ERESUME, EEXIT, EACCEPT), ENCLV |
| 6 | Introduced | Intel Skylake (6th gen, 2015); SGX2 with Ice Lake (10th gen, 2019) |
| 7 | Security relevance | Provides hardware-enforced memory isolation enclaves for code and data. Enclave memory is encrypted by the CPU and inaccessible to all software outside the enclave, including the OS and hypervisor. Enables remote attestation of enclave integrity. |
| 8 | Performance benefit | No general performance benefit. Enclave transitions (EENTER/EEXIT) add overhead (~8000-13000 cycles). EPC memory limited (128-512 MB typical, BIOS-configured). Paging EPC to regular memory is expensive. |
| 9 | Known vulnerabilities | **Extensive attack history.** Foreshadow/L1TF (CVE-2018-3615), SGAxe (CVE-2020-0549), CrossTalk/SRBDS (CVE-2020-0543), LVI (CVE-2020-0551), AEPIC Leak (CVE-2022-21233), Plundervolt (CVE-2019-11157). See CVE table below. |
| 10 | Compliance mapping | NIST SP 800-53 SC-39 (Process Isolation), SC-28 (Protection of Information at Rest); CMMC SC.L2-3.13.16. Note: SGX does NOT contribute to FIPS 140-2/3 boundary unless the enclave itself is a FIPS-validated module. |
| 11 | Classification | **Important** (with attack-surface caveat) |
| 12 | Classification rationale | SGX provides strong isolation guarantees in theory, but its extensive CVE history undermines confidence. **Presence-without-use is attack surface, not assurance gain** -- SGX exposes additional microarchitectural state that has been repeatedly exploited. Deprecated on consumer processors (11th/12th gen+); remains on Xeon only. Classification is Important rather than Critical because most UMRS deployments will not use enclaves, and the feature's attack history is significant. |
| 13 | Linux kernel support | `CONFIG_X86_SGX` -- in-tree since Linux 5.11. Device nodes: `/dev/sgx_enclave` (enclave creation/management), `/dev/sgx_vepc` (virtual EPC for KVM guests). Kernel thread `ksgxd` manages EPC sanitization and page reclaim. Requires Flexible Launch Control (FLC) MSRs to be writable -- kernel rejects non-FLC SGX as non-functional. |
| 14 | Detection method (safe Rust) | Check `/proc/cpuinfo` for `sgx` flag (Layer 1). Check `/dev/sgx_enclave` device node existence (authoritative -- different provenance model from sysfs). Check `/dev/sgx_vepc` for KVM guest SGX support. |
| 15 | Virtualization confidence | **COMPLEX** -- KVM supports SGX passthrough via virtual EPC (`/dev/sgx_vepc`). Guest sees `/dev/sgx_enclave` if host exposes virtual EPC. VMware: SGX passthrough supported since vSphere 7.0. Hyper-V: SGX not supported. Guest cannot independently verify hardware-backed SGX vs emulated without remote attestation (DCAP). EPID attestation deprecated (EOL April 2025); only DCAP supported going forward. |
| 16 | ARM/AArch64 equivalent | ARM TrustZone (different model -- whole-world isolation vs per-enclave isolation). See `arm-trustzone.md`. |
| 17 | References | Intel SDM Vol 3D Ch 36-43; Linux kernel `Documentation/arch/x86/sgx.rst`; sgx.fail (attack catalog); Intel Product Security Center |
| 18 | Disposition when unused | **DISABLE IN BIOS** -- SGX that is present but unused increases attack surface (Foreshadow, SGAxe, AEPIC Leak all target SGX-specific microarchitectural state). If no enclave workloads are planned, disable SGX in BIOS to eliminate the attack surface entirely. This is a HIGH-priority recommendation for DoD systems not running enclave workloads. |
| 19 | Software utilization detection | Check for running enclave processes: existence of `/dev/sgx_enclave` open file descriptors. `/proc/crypto` is NOT relevant (SGX is not a crypto accelerator). Check for SGX runtime libraries (e.g., `libsgx-enclave-common`, Intel SGX SDK/PSW packages). |
| 20 | FIPS utilization requirement | N/A -- SGX is not a FIPS cryptographic primitive. If enclaves perform cryptographic operations, those operations must use FIPS-validated modules independently. |
| 21 | Active mitigation status | SGX-specific vulnerabilities appear in `/sys/devices/system/cpu/vulnerabilities/l1tf` (Foreshadow) and `/sys/devices/system/cpu/vulnerabilities/srbds` (CrossTalk). |
| 22 | Feature accessible vs advertised | **HEAVILY BIOS-GATED.** CPUID may report SGX capability, but BIOS must: (1) enable SGX, (2) allocate EPC memory (PRMRR), (3) set FLC MSRs to writable. BIOS options: "Disabled", "Enabled", "Software Controlled". Linux kernel requires "Enabled" mode with writable FLC MSRs. Many BIOS implementations default to Disabled. |
| 23 | Guest-vs-host discrepancy risk | **HIGH** -- Host may have SGX hardware but not expose virtual EPC to guests. Guest CPUID will not show `sgx` flag unless host VMM explicitly configures SGX passthrough. Live migration between SGX and non-SGX hosts is problematic. |

## CVE / Vulnerability Table

| ID | Name | Year | CVSS | Impact | Fix Mechanism | SGX-Specific? |
|----|------|------|------|--------|---------------|--------------|
| CVE-2018-3615 | Foreshadow (L1TF) | 2018 | 7.9 | Read enclave memory via speculative L1 cache access. Breaks SGX confidentiality completely on affected CPUs. | Microcode + kernel L1D flush on VM entry | Yes (original variant) |
| CVE-2018-3620 | Foreshadow-NG (L1TF OS/VMM) | 2018 | 7.1 | Extension to OS/VMM memory. L1 cache timing side channel. | Microcode + PTE inversion + L1D flush | No (general) |
| CVE-2018-3646 | Foreshadow-NG (L1TF VMM) | 2018 | 7.1 | VM-to-VM or VM-to-host data leakage via L1 cache. | Microcode + L1D flush on VM entry | No (general) |
| CVE-2019-11157 | Plundervolt | 2019 | 7.1 | Undervolting CPU to induce enclave computation faults, extracting AES keys. | Microcode to lock voltage controls | Yes |
| CVE-2020-0543 | CrossTalk (SRBDS) | 2020 | 6.5 | Cross-core leakage of RDRAND/RDSEED output and SGX attestation keys via shared staging buffer. | Microcode (VERW-based clear) | Yes (primary target) |
| CVE-2020-0549 | SGAxe (L1DES + CacheOut) | 2020 | 6.5 | Extract SGX attestation keys via L1D eviction sampling. Compromises remote attestation trust chain. | Microcode + attestation key rotation | Yes |
| CVE-2020-0551 | LVI (Load Value Injection) | 2020 | 5.6 | Inject data into enclave execution via transient execution. Bypasses all prior mitigations. | Compiler barriers (LFENCE after loads); extremely expensive software mitigation. No transparent hardware fix. | Yes (primary target) |
| CVE-2022-21233 | AEPIC Leak | 2022 | 6.0 | **Architectural bug** (not transient execution). Uninitialized APIC register reads leak SGX enclave data. First non-speculative SGX attack. | Microcode | Yes |

### Attack Evolution Analysis

The SGX attack history reveals a troubling pattern:

1. **2018**: Foreshadow demonstrated speculative execution can bypass SGX isolation
2. **2019**: Plundervolt showed voltage manipulation can corrupt enclave computations
3. **2020**: SGAxe broke attestation key confidentiality; CrossTalk leaked keys cross-core; LVI showed injection attacks with no efficient hardware fix
4. **2022**: AEPIC Leak -- first architectural (non-speculative) bug, leaking data through APIC registers

Each generation of attacks has required increasingly expensive mitigations, culminating in LVI where the only mitigation is inserting LFENCE after every load instruction inside enclaves, with severe performance impact.

### SGX Deprecation Timeline

- **Consumer processors (11th gen / Tiger Lake, 12th gen / Alder Lake onward)**: SGX **deprecated**. CPUID may still enumerate SGX capability on some SKUs, but Intel officially lists it as deprecated and unsupported on client platforms.
- **Server processors (Xeon Scalable)**: SGX continues to be supported. 3rd/4th/5th gen Xeon Scalable include SGX with increased EPC sizes (up to 512 GB on some SKUs).
- **Implication**: For UMRS deployments on desktop/workstation hardware, SGX will not be available. Server deployments may encounter it. The posture check must handle the "present but deprecated" state.

## Trust Model

**What must be trusted:**
- Intel CPU hardware (microcode correctness)
- Intel attestation infrastructure (DCAP/PCCS or legacy EPID -- EPID EOL April 2025)
- BIOS/firmware (correct EPC allocation, FLC MSR configuration)
- Enclave developer (correct partitioning of trust boundary)
- Enclave signing key holder

**What is NOT trusted:**
- Operating system kernel
- Hypervisor / VMM
- Other enclaves (absent multi-enclave attestation)
- Physical memory bus (EPC is encrypted)
- Other CPU cores (in theory -- violated by CrossTalk)

**Trust model weaknesses:**
- Intel is both the hardware manufacturer and the root of attestation trust
- Attestation key compromise (SGAxe) undermines the entire remote attestation chain
- TCB recovery after attestation key compromise requires platform-wide re-attestation
- Side-channel attacks have repeatedly broken confidentiality guarantees

## Hypervisor Behavior

| Hypervisor | SGX Support | Guest Detection | Notes |
|------------|------------|-----------------|-------|
| KVM | Passthrough via virtual EPC (`/dev/sgx_vepc`) | `/dev/sgx_enclave` in guest if VMM configures it | Requires host kernel 5.13+ with `CONFIG_X86_SGX_KVM` |
| VMware vSphere | Passthrough since 7.0 | Guest CPUID shows `sgx` | Must be explicitly enabled per-VM |
| Hyper-V | Not supported | SGX not visible to guests | N/A |
| Xen | Experimental support | Varies | Not production-ready |

## Recommended Audit Card Display

```
SGX Status: [Present/Absent/Deprecated]
  CPUID: sgx=[yes/no] sgx_lc=[yes/no]
  Device: /dev/sgx_enclave [exists/absent]
  BIOS: [Enabled/Disabled/Unknown]
  EPC Size: [X MB / not allocated]
  Active Enclaves: [count or N/A]

  Finding: [NONE / HIGH: present-unused-attack-surface / INFO: deprecated-on-this-SKU]
  Recommendation: [Disable in BIOS if no enclave workloads]
```

## Sources

- [Intel SDM Vol 3D: Intel SGX](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html)
- [Linux kernel: Documentation/arch/x86/sgx.rst](https://docs.kernel.org/arch/x86/sgx.html)
- [sgx.fail -- SGX Attack Catalog](https://sgx.fail/)
- [Foreshadow Attack](https://foreshadowattack.eu/)
- [LVI Attack](https://lviattack.eu/)
- [AEPIC Leak](https://aepicleak.com/)
- [Intel Product Security Center](https://www.intel.com/content/www/us/en/security-center/default.html)
- [Intel SGX Deprecation (12th Gen Datasheet)](https://edc.intel.com/content/www/us/en/design/ipla/software-development-platforms/client/platforms/alder-lake-desktop/12th-generation-intel-core-processors-datasheet-volume-1-of-2/001/deprecated-technologies/)
- [Overview of SGX Vulnerabilities (cyber.ee 2025)](https://cyber.ee/uploads/report_2025_sgx_19b89d79ed.pdf)
- [Gramine SGX Introduction](https://gramine.readthedocs.io/en/stable/sgx-intro.html)
