# Intel Key Locker

## 23-Column Matrix Profile

| # | Column | Value |
|---|--------|-------|
| 1 | Feature name | Intel Key Locker (KL) |
| 2 | Vendor | Intel |
| 3 | CPUID detection | **KL support:** CPUID leaf 7, subleaf 0, ECX bit 23 (`KL` -- Key Locker supported). **AESKLE (AES Key Locker instructions):** CPUID leaf 19H, EBX bit 0 (`AESKLE`). **Wide Key Locker:** CPUID leaf 19H, EBX bit 2. **CR4.KL:** Must be set (bit 19) by OS to enable Key Locker instructions. When CR4.KL=0, all Key Locker instructions fault (#UD) and CPUID.AESKLE reads as 0. |
| 4 | Linux `/proc/cpuinfo` flag | `kl` (Key Locker capability), `aeskle` (AES Key Locker instructions enabled) |
| 5 | Key instructions | LOADIWKEY (load internal wrapping key), ENCODEKEY128/ENCODEKEY256 (convert AES key to handle), AESENC128KL/AESDEC128KL (AES encrypt/decrypt with 128-bit key handle), AESENCWIDE128KL/AESDECWIDE128KL (8-block wide AES with handle), AESENC256KL/AESDEC256KL (256-bit variants) |
| 6 | Introduced | Intel Tiger Lake (11th gen, 2020) and later. Alder Lake, Sapphire Rapids, and subsequent generations. |
| 7 | Security relevance | Key Locker replaces raw AES keys in memory with opaque "key handles." The actual AES key is encrypted under an Internal Wrapping Key (IWK) that lives only in CPU registers and is never software-accessible. AES operations use the handle -- the raw key is decrypted internally per-operation and never exposed. If memory is dumped (cold boot, DMA, kernel exploit), only handles are visible, not raw AES keys. |
| 8 | Performance benefit | Key Locker AES operations have slightly higher latency than direct AES-NI (~10-20% overhead per block) due to handle unwrapping. The security benefit is the trade-off. Wide Key Locker (8 blocks at once) amortizes the overhead. |
| 9 | Known vulnerabilities | No Key Locker-specific CVEs as of early 2026. The technology is relatively new and not yet widely deployed, which limits both attack research and deployment experience. Theoretical concern: the IWK is per-boot and per-socket; if the IWK generation (via RDRAND/RDSEED internally) has quality issues, all handles on that socket are compromised. |
| 10 | Compliance mapping | NIST SP 800-53 SC-12 (Cryptographic Key Establishment and Management), SC-28 (Protection of Information at Rest). Key Locker handles do NOT replace the need for FIPS-validated key storage -- the IWK is ephemeral and CPU-internal, making it difficult to map to a FIPS 140-3 boundary. |
| 11 | Classification | **Informational** |
| 12 | Classification rationale | Key Locker is a defense-in-depth mechanism for AES key protection. Classification is Informational because: (1) not widely deployed or required by any compliance framework, (2) Linux kernel support is still in patch review (not yet mainlined as of early 2026), (3) the security benefit requires application-level adoption of the Key Locker API, which does not exist in common software stacks yet, (4) does not affect FIPS compliance posture. |
| 13 | Linux kernel support | **NOT YET MAINLINED** as of Linux 6.x. Patch series (v8/v9, 2023-2024) from Intel adds `aeskl-intel` driver to kernel crypto subsystem. Patches include: CPUID leaf definition, IWK management at boot, ACPI S3/S4 sleep state IWK restoration, integration with kernel crypto API. The driver would register as `aeskl-intel` in `/proc/crypto`. Key Locker userspace API is not yet defined. |
| 14 | Detection method (safe Rust) | Check `/proc/cpuinfo` for `kl` and `aeskle` flags (Layer 1). If kernel support lands, check `/proc/crypto` for `aeskl-intel` driver (Layer 2). Without kernel support, Key Locker hardware is present but not usable from Linux. No sysfs interface. |
| 15 | Virtualization confidence | **COMPLEX** -- KVM Key Locker support was proposed (patch series 2021) but not merged. The IWK is per-physical-CPU -- virtualization requires IWK save/restore on vCPU migration. Live migration between hosts invalidates all key handles (different IWK per host). VMware, Hyper-V: no Key Locker guest support documented. Guest cannot verify hardware-backed Key Locker vs emulation. |
| 16 | ARM/AArch64 equivalent | No direct ARM equivalent. ARM TrustZone can be used for key isolation, but it is a fundamentally different model (world-level isolation vs instruction-level key handle). ARM CCA may provide similar key isolation in future iterations. |
| 17 | References | Intel Key Locker Specification (doc 343965); Linux kernel patch series (v8/v9); LWN.net coverage; Phoronix coverage |
| 18 | Disposition when unused | **LEAVE ENABLED** -- Key Locker hardware is dormant unless the OS sets CR4.KL and loads an IWK. No attack surface from unused Key Locker. No reason to disable in BIOS. |
| 19 | Software utilization detection | If kernel `aeskl-intel` driver is merged: check `/proc/crypto` for driver registration. Without kernel support: no software utilization possible. Application-level detection: not applicable until userspace API exists. |
| 20 | FIPS utilization requirement | N/A -- Key Locker is not part of any FIPS validation certificate. The IWK is ephemeral and CPU-internal, making it difficult to include in a FIPS 140-3 module boundary. Key Locker is a key protection mechanism, not a cryptographic primitive. |
| 21 | Active mitigation status | No entry in `/sys/devices/system/cpu/vulnerabilities/`. Key Locker is not a vulnerability mitigation. |
| 22 | Feature accessible vs advertised | **OS-GATED.** CPUID reports `kl` capability, but the OS must set CR4.KL (bit 19) and load an IWK via LOADIWKEY before Key Locker instructions become functional. Without OS support, CPUID.AESKLE reads as 0 even if hardware supports KL. BIOS does not need to explicitly enable Key Locker (no BIOS gate). |
| 23 | Guest-vs-host discrepancy risk | **HIGH** -- KVM does not yet support Key Locker passthrough. Guests will not see `kl`/`aeskle` flags even on Key Locker-capable hosts. Key handle invalidation on live migration is an unsolved problem for virtualization. |

## Key Locker Architecture

### Internal Wrapping Key (IWK)
- 256-bit key generated by CPU at boot (via hardware RNG)
- Stored in CPU-internal register -- never software-accessible
- Per-socket: each physical CPU has its own IWK
- Lost on power cycle, sleep (S3/S4), or reset
- Kernel must restore IWK after sleep (S3/S4 patches handle this)
- IWK backup for sleep: encrypted with CPU-internal key, stored in protected memory

### Key Handle Format
- Opaque byte sequence (128 bytes for AES-128, 256 bytes for AES-256)
- Contains: AES key encrypted under IWK + integrity tag
- Bound to the IWK that created it (invalidated if IWK changes)
- Cannot be decrypted by software -- only CPU hardware can unwrap

### Operation Flow
1. Application provides raw AES key to ENCODEKEY128/ENCODEKEY256
2. CPU encrypts key under IWK, returns handle
3. Application stores handle (can be in regular memory)
4. For encryption/decryption: application provides handle to AESENC128KL/AESDEC128KL
5. CPU internally unwraps handle, performs AES, returns result
6. Raw key never appears in memory after step 2

## Trust Model

**What must be trusted:**
- Intel CPU hardware (IWK generation, handle encryption/decryption)
- CPU hardware RNG (IWK quality)
- Kernel (IWK management, CR4.KL control)

**What is NOT trusted (by Key Locker):**
- Application memory (handles are safe to leak -- cannot be unwrapped without IWK)
- DMA-accessible memory (handles, not keys, are visible)
- Memory dumps / cold boot (only handles visible)

**What Key Locker does NOT protect against:**
- Active kernel compromise (attacker can call AES operations with existing handles)
- IWK compromise (if hardware RNG is flawed)
- Cross-boot correlation (IWK changes each boot; handles are not portable)

## Recommended Audit Card Display

```
Key Locker Status: [Present/Enabled/Not Available/No Kernel Support]
  CPUID: kl=[yes/no] aeskle=[yes/no]
  Kernel: aeskl-intel driver [loaded/not available]
  IWK: [loaded/not loaded/unknown]

  Finding: [INFO: present-no-kernel-support / NONE]
  Recommendation: [Monitor Linux kernel Key Locker support; no action required]
```

## Sources

- [Intel Key Locker Specification (doc 343965)](https://cdrdv2-public.intel.com/671438/343965-intel-key-locker-specification.pdf)
- [Phoronix: Intel Key Locker Linux Kernel Support](https://www.phoronix.com/news/Intel-Key-Locker-Linux-Kernel)
- [LWN.net: x86 Support Key Locker](https://lwn.net/Articles/878643/)
- [Linux kernel patch v9 (March 2024)](https://patchew.org/linux/20240329015346.635933-1-chang.seok.bae@intel.com/)
- [LWN.net: KVM Support Intel Key Locker](https://lwn.net/Articles/857783/)
