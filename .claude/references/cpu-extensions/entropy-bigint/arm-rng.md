# ARMv8.5-RNG (ARM Random Number Generation Extension)

**Category:** Entropy & Random Generation (Category 4) / ARM/AArch64 Equivalents (Category 15)
**Classification:** Critical/Operational (FIPS)
**Phase:** 1B
**Date:** 2026-03-18

---

## 23-Column Matrix Profile

| # | Column | Value |
|---|--------|-------|
| 1 | Feature name | ARMv8.5-RNG (`rng` flag) |
| 2 | Vendor | ARM (AArch64 only) |
| 3 | Category | Entropy & Random Generation; ARM/AArch64 Equivalents |
| 4 | Purpose | Hardware random number generation on AArch64: provides two instructions — RNDR (analogous to RDRAND) and RNDRRS (analogous to RDSEED) — via system register reads. Both return 64-bit values from an on-chip TRNG-backed RBG. |
| 5 | Example instructions | `MRS x0, RNDR` — read 64-bit random number (DRBG output; PSTATE.NZCV.C=1 on success); `MRS x0, RNDRRS` — read 64-bit random number with fresh entropy (requests DRBG reseed from entropy source before output; C=1 on success, C=0 on failure/retry needed) |
| 6 | CPUID detection | System register `ID_AA64ISAR0_EL1`, field RNDR (bits [63:60]), value `0b0001` indicates ARMv8.5-RNG support. Not a CPUID leaf — ARM uses system register introspection. Reading `ID_AA64ISAR0_EL1` from EL0 requires `HWCAP_CPUID` capability. Preferred userspace method: `getauxval(AT_HWCAP2) & HWCAP2_RNG`. |
| 7 | Linux `/proc/cpuinfo` flag | `rng` (in the Features line of AArch64 `/proc/cpuinfo`) |
| 8 | Linux authoritative path | `getauxval(AT_HWCAP2) & HWCAP2_RNG` — authoritative at runtime. `/proc/cpuinfo` Features line `rng` flag — human-readable but less reliable for early-startup detection. No sysfs vulnerability path exists (no known Spectre-class vulnerability against ARMv8.5-RNG). |
| 9 | Minimum CPU generations | ARM Cortex-A76 and later with ARMv8.5 profile; Apple M1 and later; Qualcomm Snapdragon 8cx Gen 3 and later. Not present on pre-ARMv8.5 cores. As of 2025, widely available on server-class AArch64 (AWS Graviton3, Ampere Altra Max). |
| 10 | Security benefit | Provides hardware-seeded random generation eliminating reliance on software entropy during early boot. RNDRRS (like RDSEED) requests entropy source reseed, enabling DRBG reseeding from physical noise rather than continued output of a long-running DRBG. Eliminates the same category of entropy starvation risks as RDRAND on x86. |
| 11 | Performance benefit | RNDR: comparable to RDRAND on x86, ~1 Gbps throughput on modern ARM cores. RNDRRS: lower throughput by design (entropy source rate-limited). Both return 64-bit values, providing efficient entropy generation per call. |
| 12 | Assurance caveats | (1) No known ARM-specific bugs comparable to AMD-SB-7055 or the AMD family 15h/16h suspend bug (as of 2026-03-18). (2) ARM does not provide a separate CMVP Entropy Source Validation certificate for RNDR/RNDRRS — the ARM architecture specification describes the design requirements, but per-implementation validation is vendor-specific and not universally documented. (3) RNDR and RNDRRS are accessed as system register reads, not standalone instructions — the semantics are similar to RDRAND/RDSEED but the failure signaling is via PSTATE flags (C flag), same as x86 CF. (4) Hypervisors may trap and emulate RNDR/RNDRRS reads; guests cannot distinguish hardware-backed from software-emulated responses via register reads alone. (5) The `rng` flag in `/proc/cpuinfo` is the kernel-filtered view — checking `AT_HWCAP2 & HWCAP2_RNG` is more reliable for application-level detection. |
| 13 | Virtualization behavior | KVM/QEMU: RNDR/RNDRRS are trappable at EL2. KVM can pass through or emulate. When emulated, the guest sees the same `rng` feature flag but receives CSPRNG output from the host. **Guest cannot distinguish hardware-backed entropy from hypervisor-emulated entropy** via register reads. AWS Graviton3 in Nitro hypervisor: passes through hardware RNDR. |
| 14 | Firmware / BIOS / microcode dependency | BIOS/EFI: must not disable the feature via `ID_AA64ISAR0_EL1` masking. No microcode equivalent — ARMv8.5-RNG is a CPU core feature, not a separately updateable microcode component. Linux kernel must be compiled with `CONFIG_ARCH_RANDOM` to enable `arch_get_random_seed_long()` using RNDRRS. |
| 15 | Audit-card relevance | **Critical/Operational (FIPS)** |
| 16 | Recommended disposition when unused | Do not disable. On AArch64 systems with the `rng` feature, verify the Linux kernel entropy seeding path uses RNDRRS. There is no `random.trust_cpu` kernel parameter equivalent on ARM — the kernel integrates RNDR/RNDRRS into the entropy pool via `arch_get_random_seed_long()` when `CONFIG_ARCH_RANDOM` is enabled. |
| 17 | Software utilization detection | (a) Verify `rng` in `/proc/cpuinfo` Features line (Layer 1). (b) Check kernel config: `CONFIG_ARCH_RANDOM=y` must be set — verify via `/boot/config-$(uname -r)` or `/proc/config.gz`. (c) No `/proc/crypto` entry — RNDR/RNDRRS feed the kernel entropy pool, not the crypto subsystem directly. (d) OpenSSL FIPS provider on RHEL 10 AArch64: uses `HWCAP2_RNG` to detect RNDR support and enables the RNDR-based entropy path. |
| 18 | FIPS utilization requirement | FIPS 140-3 on AArch64: RNDRRS is the preferred hardware entropy source for seeding DRBGs, analogous to RDSEED on x86. ARM's design intends RNDRRS to satisfy SP 800-90B noise source requirements (requests fresh entropy before returning), while RNDR satisfies SP 800-90A DRBG output requirements. Actual FIPS validation coverage on specific ARM SoCs varies by implementation — no single universal CMVP certificate covers all ARM licensees. |
| 19 | Active mitigation status path | N/A — no equivalent to `/sys/devices/system/cpu/vulnerabilities/` entries for ARMv8.5-RNG. ARM has separate vulnerability paths for speculative execution mitigations (Spectre/Meltdown equivalents) but not for the RNG feature. |
| 20 | Feature accessible vs advertised | On compliant ARMv8.5 systems: if `ID_AA64ISAR0_EL1.RNDR == 0b0001`, the feature is accessible. Firmware could potentially mask this register field, but this is not documented as a common configuration. The `rng` cpuinfo flag reflects the kernel's view of the system register. |
| 21 | Guest-vs-host discrepancy risk | **HIGH** — same CPUID/system register passthrough opacity as x86 RDRAND/RDSEED. KVM can emulate RNDR in software, producing the feature flag with no hardware entropy backing. No guest-visible distinction mechanism. |
| 22 | Notes | ARMv8.5-RNG provides both RDRAND and RDSEED semantics in a single extension: RNDR (conditioned DRBG output) maps to RDRAND; RNDRRS (entropy source reseed then DRBG output) maps to RDSEED. The ARM architecture consolidates what x86 splits across two separate CPUID bits. Implementation quality varies across ARM licensees — AWS Graviton3 and Apple Silicon are known-good implementations. Embedded and IoT ARM cores may implement the feature with minimal physical entropy source. |
| 23 | Sources | ARM Architecture Reference Manual (ARMv8-A): RNDR, RNDRRS system registers; ARM DDI 0595 (ID_AA64ISAR0_EL1); Linux kernel `arch/arm64/include/asm/archrandom.h`; Linux kernel ELF hwcaps documentation; LWN ARMv8.5-RNG patch series; Intel DRNG guide (for RDRAND/RDSEED comparison); NIST SP 800-90B (January 2018) |

---

## RNDR vs RNDRRS: Instruction Semantics

### RNDR (analogous to RDRAND)

- Returns a 64-bit value from an approved Random Bit Generator
- The DRBG is reseeded from the entropy source at an **implementation-defined rate**
- Success: PSTATE.NZCV.C = 1 (Carry flag set), destination register contains the value
- Failure: PSTATE.NZCV.C = 0, destination register set to 0 — caller must retry
- Design intent: high-throughput random number consumption

```asm
// Safe Rust equivalent (nightly, or via core::arch):
// mrs x0, RNDR
// Carry flag clear = retry; carry flag set = success
```

### RNDRRS (analogous to RDSEED)

- Returns a 64-bit value from an approved Random Bit Generator
- Before returning, the DRBG is **reseeded from the physical entropy source**
- Success: PSTATE.NZCV.C = 1
- Failure: PSTATE.NZCV.C = 0 — entropy source exhausted, retry later
- Design intent: DRBG seeding; lower throughput than RNDR
- This is the closer analog to the 800-90B noise source interface on ARM

### ARM Naming vs x86 Naming

| ARM instruction | x86 equivalent | 800-90 analogy | Failure handling |
|-----------------|---------------|----------------|-----------------|
| `RNDR` | `RDRAND` | SP 800-90A DRBG output | C=0 on failure; retry |
| `RNDRRS` | `RDSEED` | SP 800-90B noise source-backed | C=0 on exhaustion; retry with backoff |

ARM consolidates both capabilities into a single ISA extension (`rng` flag), while x86 uses separate CPUID bits (`rdrand` and `rdseed`).

---

## Linux Detection

### Layer 1 — Hardware Presence

**Authoritative userspace method:**
```c
#include <sys/auxv.h>
#include <asm/hwcap.h>

// HWCAP2_RNG = (1 << 11) on Linux arm64
unsigned long hwcap2 = getauxval(AT_HWCAP2);
bool rng_supported = (hwcap2 & HWCAP2_RNG) != 0;
```

**System register method (privileged or HWCAP_CPUID required):**
```
ID_AA64ISAR0_EL1, bits [63:60] (RNDR field):
  0b0000 = not implemented
  0b0001 = RNDR and RNDRRS implemented
```

**`/proc/cpuinfo` method (human-readable, less reliable for application code):**
```
# grep rng /proc/cpuinfo
Features : ... rng ...
```

### Layer 2 — Kernel Utilization

Verify `CONFIG_ARCH_RANDOM` is enabled in the running kernel:
```bash
grep CONFIG_ARCH_RANDOM /boot/config-$(uname -r)
# Expected: CONFIG_ARCH_RANDOM=y
```

The kernel function `arch_get_random_seed_long()` calls RNDRRS when:
1. `CONFIG_ARCH_RANDOM=y`
2. `HWCAP2_RNG` is present
3. The entropy source is not exhausted (C flag check)

---

## Comparison to x86 RDRAND/RDSEED

| Property | RDRAND | RDSEED | ARM RNDR | ARM RNDRRS |
|----------|--------|--------|----------|------------|
| Output type | DRBG (SP 800-90A) | Raw entropy (SP 800-90B) | DRBG (SP 800-90A) | Entropy-seeded DRBG (SP 800-90B) |
| Operand sizes | 16/32/64-bit | 16/32/64-bit | 64-bit only | 64-bit only |
| CPUID / detection | Leaf 1, ECX[30] | Leaf 7, EBX[18] | `ID_AA64ISAR0_EL1.RNDR` / `HWCAP2_RNG` | Same flag — both instructions |
| `/proc/cpuinfo` flag | `rdrand` | `rdseed` | `rng` | `rng` (same flag) |
| Known bugs | AMD family 15h suspend bug; SRBDS | AMD-SB-7055 Zen 5 (16/32-bit); SRBDS | None known (2026-03) | None known (2026-03) |
| FIPS classification | Critical/Operational | Critical/Operational | Critical/Operational | Critical/Operational |
| Virtualization risk | HIGH | HIGH | HIGH | HIGH |

**Key ARM advantage:** RNDR and RNDRRS are 64-bit only, which eliminates the class of bug seen in AMD-SB-7055 (where 16/32-bit truncation was implemented incorrectly). The ARM ISA design avoids this risk entirely.

---

## Posture Check Specification

### ARMv8.5-RNG Posture Check (for future CPU probe, AArch64 systems)

1. Check `/proc/cpuinfo` Features for `rng` flag (Layer 1 — hardware present)
2. Verify `getauxval(AT_HWCAP2) & HWCAP2_RNG` matches (or use file-based check via `/proc/self/auxv` parsing)
3. Check kernel config: `CONFIG_ARCH_RANDOM=y` — if absent, flag **MEDIUM** (hardware entropy not used by kernel)
4. No SRBDS-equivalent known for ARM; no `/sys/devices/system/cpu/vulnerabilities/` path to check
5. On virtualized systems: note **INFORMATIONAL** that RNDR/RNDRRS may be hypervisor-emulated; guest cannot independently verify

### UMRS Signal Mapping

No current `IndicatorId` covers AArch64 RNDR/RNDRRS. The `IndicatorId::RandomTrustCpu` signal is x86-specific (checks `/proc/cmdline` for `random.trust_cpu=on`). A future AArch64-specific signal should check `CONFIG_ARCH_RANDOM` kernel config and `HWCAP2_RNG` presence together, without a `random.trust_cpu` equivalent (ARM does not use that parameter).

---

## Sources

- [ARM DDI 0595: ID_AA64ISAR0_EL1 RNDR field](https://developer.arm.com/documentation/ddi0595/2020-12/AArch64-Registers/ID-AA64ISAR0-EL1--AArch64-Instruction-Set-Attribute-Register-0)
- [ARM64 ELF hwcaps — Linux kernel documentation](https://docs.kernel.org/arch/arm64/elf_hwcaps.html)
- [LWN: ARMv8.5-RNG kernel patch series](https://lwn.net/Articles/809929/)
- [Linux kernel `arch/arm64/kernel/cpuinfo.c`](https://github.com/torvalds/linux/blob/master/arch/arm64/kernel/cpuinfo.c)
- [ARM Developer: Runtime detection of CPU features on ARMv8-A](https://developer.arm.com/community/arm-community-blogs/b/operating-systems-blog/posts/runtime-detection-of-cpu-features-on-an-armv8-a-cpu)
- [Mbed-TLS: Implement ARMv8.5-A RNDRRS as entropy source](https://github.com/Mbed-TLS/mbedtls/issues/7509)
- [rng-tools: ARM RNDR support PR](https://github.com/nhorman/rng-tools/pull/128)
- [Bitcoin Core: HRNG support for ARM RNDR/RNDRRS](https://github.com/bitcoin/bitcoin/issues/26796)
- [NIST SP 800-90B (January 2018)](https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-90B.pdf)
