# Phase 1B Summary: Entropy & Big Integer Extensions

**Phase:** 1B
**Date:** 2026-03-18
**Files in this directory:**
- `rdrand.md` — RDRAND (DRBG output interface)
- `rdseed.md` — RDSEED (raw entropy source interface)
- `arm-rng.md` — ARMv8.5-RNG (RNDR/RNDRRS)
- `adx-bmi.md` — ADX, BMI1, BMI2 (big integer/bit manipulation)
- `phase-1b-summary.md` — this file

---

## Feature Classification Summary

| Feature | Classification | Vendor | CPUID / Detection | Primary Audit Surface |
|---------|---------------|--------|-------------------|-----------------------|
| RDRAND | Critical/Operational (FIPS) | Both | Leaf 1, ECX[30] / `rdrand` | `/proc/cmdline`: `random.trust_cpu=on`; `/sys/.../srbds` |
| RDSEED | Critical/Operational (FIPS) | Both | Leaf 7, EBX[18] / `rdseed` | AMD Zen 5 firmware version; `/sys/.../srbds` |
| ARMv8.5-RNG | Critical/Operational (FIPS) | ARM | `AT_HWCAP2 & HWCAP2_RNG` / `rng` | `CONFIG_ARCH_RANDOM` kernel config |
| ADX | Informational | Both | Leaf 7, EBX[19] / `adx` | OpenSSL RSA/ECC throughput |
| BMI1 | Informational | Both | Leaf 7, EBX[3] / `bmi1` | Compiler output quality |
| BMI2 | Informational | Both | Leaf 7, EBX[8] / `bmi2` | PDEP/PEXT latency on AMD pre-Zen 3 |

---

## The RDRAND / RDSEED / 800-90B Compliance Chain

Understanding the relationship between these features and NIST SP 800-90B is essential for correct FIPS audit reasoning.

### Architecture

```
Physical Noise Source (TRNG)
        |
        | [raw entropy samples — not exposed to software]
        |
        +---> [Health Tests: Repetition Count + Adaptive Proportion]
        |         per SP 800-90B Section 4
        |
        +---> RDSEED: returns raw/lightly-conditioned noise source output
        |         → SP 800-90B noise source interface
        |         → use to seed external DRBGs
        |
        +---> [AES-CTR DRBG per SP 800-90A]
                  |
                  v
             RDRAND: returns DRBG output
                  → SP 800-90A DRBG interface
                  → use for high-throughput random number consumption
```

### ARM Equivalents

```
Physical Noise Source (TRNG)
        |
        +---> RNDRRS: requests entropy source reseed, returns DRBG output
        |         → closest ARM analog to RDSEED (SP 800-90B-backed seeding)
        |
        +---> RNDR: returns DRBG output at implementation-defined reseed rate
                  → closest ARM analog to RDRAND (SP 800-90A DRBG output)
```

### FIPS Compliance Implications

| Question | Answer |
|----------|--------|
| Does RDRAND output satisfy 800-90B? | No — it is DRBG output (800-90A), not a noise source |
| Does RDSEED output satisfy 800-90B? | Yes — it exposes the 800-90B-validated noise source output |
| Is Intel's TRNG 800-90B-validated? | Yes — CMVP ESV certificates E65/E66/E232 cover the on-die TRNG |
| Does `random.trust_cpu=on` violate 800-90B? | Architecturally no (Intel's TRNG is validated), but it bypasses OS-level entropy accounting |
| Does AMD Zen 5 RDSEED satisfy 800-90B? | No — until AGESA fix applied, 16/32-bit forms violate noise source behavioral requirements |
| Does RNDRRS satisfy 800-90B on ARM? | Architecturally yes — designed to satisfy 800-90B intent; per-implementation validation varies |

### The `random.trust_cpu` Signal

The UMRS posture catalog already contains `IndicatorId::RandomTrustCpu`, which flags the presence of `random.trust_cpu=on` in `/proc/cmdline`. This is the primary entropy-related posture signal in the current catalog.

The connection to Phase 1B research:
- `random.trust_cpu=on` causes the kernel to credit RDRAND **output** as entropy
- RDRAND output is DRBG output, not raw noise source output
- Intel's TRNG validation (CMVP E65/E66/E232) covers the underlying noise source but not the DRBG output layer as an entropy source
- RHEL 10 default: `random.trust_cpu` is NOT set — this is the correct FIPS posture
- The signal fires as a Medium finding when `random.trust_cpu=on` is detected

---

## Key Audit Findings from Phase 1B Research

### Finding 1: AMD Zen 5 RDSEED Defect (AMD-SB-7055)

**Severity:** HIGH (CVSS 7.2)
**Affects:** AMD Zen 5 processors (Ryzen 9000, EPYC 9005, Ryzen AI 300, Threadripper 9000)
**Description:** 16-bit and 32-bit RDSEED returns zero while signaling success (CF=1).
**Status:** Fixed by AGESA firmware update. EPYC 9005: October 2025. Consumer: November 25, 2025.
**Detection gap:** No current UMRS signal covers this. A new `IndicatorId` for AMD Zen 5 firmware version check would be needed.

**Audit implication:** On any AMD Zen 5 deployment, audit the BIOS/firmware version against the AGESA fix dates. Application-level code using 16/32-bit RDSEED is affected; the Linux kernel uses 64-bit RDSEED and is not affected.

### Finding 2: SRBDS (CVE-2020-0543) Applies to Both RDRAND and RDSEED

**Severity:** HIGH
**Affects:** Intel CPUs from Ivy Bridge through Ice Lake (not affected: Atom family, some server SKUs)
**Description:** RDRAND and RDSEED values leak across cores via MDS staging buffer.
**Status:** Fixed by Intel microcode update (June 2020).
**Detection:** `/sys/devices/system/cpu/vulnerabilities/srbds`
**Current UMRS coverage:** No current indicator for SRBDS mitigation status. Future CPU probe should add this.

### Finding 3: RDRAND Guest Opacity

**Severity:** INFORMATIONAL for FIPS; MEDIUM for confidential computing
**Description:** Hypervisors can emulate RDRAND/RDSEED in software, returning the same CPUID flags as hardware-backed implementations. Guests have no mechanism to distinguish hardware-backed entropy from hypervisor CSPRNG output.
**Implication:** In confidential computing contexts (AMD SEV-SNP, Intel TDX), entropy source trustworthiness cannot be verified from within the guest via RDRAND/RDSEED alone.

### Finding 4: BMI2 PDEP/PEXT Latency Asymmetry

**Severity:** INFORMATIONAL (no direct security finding)
**Affects:** AMD pre-Zen 3 (Zen 1, Zen 2) — PDEP/PEXT at 18-cycle microcode latency vs 3 cycles
**Description:** Constant-time cryptographic code that uses PDEP/PEXT may fall back to variable-timing software paths on pre-Zen 3 AMD, potentially reintroducing timing side channels.
**Implication:** Code review finding for any UMRS components using BMI2 PEXT/PDEP in security-sensitive paths.

### Finding 5: ARMv8.5-RNG Lacks Universal CMVP Coverage

**Severity:** INFORMATIONAL
**Description:** Unlike Intel's DRNG (which has CMVP ESV certificates E65/E66/E232), ARM's RNDR/RNDRRS have no universal CMVP validation. Per-implementation validation by ARM licensees varies.
**Implication:** On AArch64 FIPS deployments, the entropy source quality assertion rests on the ARM architecture specification and the specific SoC vendor's claims, not a CMVP certificate. AWS Graviton3 and Apple Silicon are known-quality implementations; others may not be.

---

## New UMRS Signals Identified (for future CPU probe phase)

The following signals are not in the current posture catalog but are indicated by Phase 1B research:

| Proposed Signal | Class | Detection Path | Impact | Rationale |
|-----------------|-------|----------------|--------|-----------|
| `SRBDS mitigation status` | `CpuVulnSysfs` | `/sys/.../vulnerabilities/srbds` | HIGH if vulnerable | RDRAND/RDSEED values leak across cores without microcode fix |
| `AMD Zen 5 RDSEED firmware` | `DmiBiosVersion` (new) | `/sys/class/dmi/id/bios_date` + CPU family | HIGH | Zen 5 RDSEED 16/32-bit defect; firmware-dependent fix |
| `RNDR kernel config` | `KernelConfig` (new) | `/boot/config-$(uname -r): CONFIG_ARCH_RANDOM` | MEDIUM | ARMv8.5-RNG not used if kernel built without CONFIG_ARCH_RANDOM |
| `ADX present for FIPS RSA/ECC` | `CpuProcInfo` | `/proc/cpuinfo: adx flag` | Informational | Performance context for FIPS RSA/ECDSA on FIPS systems |

---

## IndicatorId::RandomTrustCpu — Connection Summary

The existing `IndicatorId::RandomTrustCpu` in `umrs-platform/src/posture/catalog.rs` maps to:

- **Layer 1:** `rdrand` or `rdseed` flag in `/proc/cpuinfo` — hardware entropy source present
- **Layer 2:** `/proc/cmdline` — whether `random.trust_cpu=on` is set

The Phase 1B corpus establishes the full compliance context for this signal:
1. Intel's DRNG is 800-90B validated at the TRNG level (CMVP E65/E66/E232)
2. RDRAND output is DRBG output, not direct noise source exposure
3. `random.trust_cpu=on` is architecturally sound for Intel hardware but bypasses the OS entropy accounting layer that 800-90B requires be respected
4. RHEL 10 default (`random.trust_cpu` absent) is the correct FIPS posture
5. The signal's Medium impact rating is appropriate and well-supported by this analysis

---

## Phase 1B vs Phase 1A: Classification Comparison

Phase 1A features (AES-NI, SHA-NI, PCLMULQDQ, VAES) are **Critical/Operational** because their absence forces software fallbacks with timing side-channel risks (particularly AES T-table implementations).

Phase 1B entropy features (RDRAND, RDSEED, ARMv8.5-RNG) are **Critical/Operational** for a different reason: their absence or misuse affects FIPS 140-3 entropy source compliance, not algorithmic timing. The risk is entropy quality and audit compliance, not a directly exploitable timing channel.

Phase 1B big integer features (ADX, BMI1, BMI2) are **Informational** because their absence degrades performance but does not introduce timing-channel risk (the fallback arithmetic is functionally correct and, if written carefully, constant-time).

---

## Cross-References

- `rdrand.md` — full RDRAND 800-90B analysis, SRBDS detail, `IndicatorId::RandomTrustCpu` connection
- `rdseed.md` — RDSEED 800-90B analysis, AMD-SB-7055 Zen 5 bug detail
- `arm-rng.md` — RNDR/RNDRRS semantics, HWCAP2_RNG detection, RDRAND/RDSEED comparison table
- `adx-bmi.md` — ADX parallel multiplication pattern, PEXT/PDEP timing caveat
- `.claude/references/nist/sp800-90B.pdf` — NIST SP 800-90B (January 2018), full text
- `umrs-platform/src/posture/catalog.rs` — `IndicatorId::RandomTrustCpu` implementation
- Phase 1A summary: `.claude/references/cpu-extensions/crypto-accel/phase-1a-summary.md`
