# RDSEED (Raw Hardware Entropy Source)

**Category:** Entropy & Random Generation (Category 4)
**Classification:** Critical/Operational (FIPS)
**Phase:** 1B
**Date:** 2026-03-18

---

## 23-Column Matrix Profile

| # | Column | Value |
|---|--------|-------|
| 1 | Feature name | RDSEED |
| 2 | Vendor | Both (Intel & AMD) |
| 3 | Category | Entropy & Random Generation |
| 4 | Purpose | Raw hardware entropy source: returns output from the on-die physical noise source (TRNG / non-deterministic entropy source) before DRBG conditioning. Intended for seeding software DRBGs and other entropy sources, not for direct use as a random number stream. |
| 5 | Example instructions | `RDSEED r16`, `RDSEED r32`, `RDSEED r64` (CF=1 on success, CF=0 if entropy source is exhausted — retry required; no bounded retry guarantee unlike RDRAND) |
| 6 | CPUID detection | Leaf 7 (EAX=07H, ECX=0), EBX bit 18 |
| 7 | Linux `/proc/cpuinfo` flag | `rdseed` |
| 8 | Linux authoritative path | `/proc/cpuinfo` flag `rdseed`; secondary: `/sys/devices/system/cpu/vulnerabilities/srbds` (SRBDS mitigation status applies to RDSEED as well as RDRAND) |
| 9 | Minimum CPU generations | Intel: Broadwell (2014, 5th gen Core); AMD: Zen (Ryzen 1000, 2017) |
| 10 | Security benefit | Provides raw entropy from the physical noise source, satisfying NIST SP 800-90B noise source requirements. Enables software DRBGs to be seeded with hardware-backed entropy of documented quality. Closer to a true TRNG interface than RDRAND, which only exposes DRBG output. |
| 11 | Performance benefit | Lower throughput than RDRAND by design — entropy source has finite rate. Suitable for periodic seeding operations, not high-rate random number generation. On modern Intel hardware: ~100–200 MB/s maximum. |
| 12 | Assurance caveats | (1) **AMD-SB-7055 / CVE-2025-62626 (Zen 5 critical bug):** On AMD Zen 5 processors, the 16-bit and 32-bit forms of RDSEED return 0 at a rate inconsistent with randomness while incorrectly signaling success (CF=1). The 64-bit form is not affected. CVSS 7.2 (High). Software unaware of the bug silently uses zero-biased "entropy." Fix: AGESA firmware update (released November 2025 for EPYC 9005, Ryzen 9000, Ryzen AI 300). Workaround: use only the 64-bit form or switch to a software entropy source. (2) **SRBDS (CVE-2020-0543):** RDSEED output can leak across cores via MDS staging buffer, same as RDRAND. Fixed by Intel microcode update (June 2020). (3) **Exhaustion behavior:** When the entropy source cannot keep up, RDSEED returns CF=0. Applications must handle retry correctly. Unlike RDRAND, there is no bounded retry loop guarantee — the entropy source may remain exhausted under heavy concurrent load. (4) RDSEED output is NOT conditioned (or is minimally conditioned relative to RDRAND) — raw noise source samples have lower per-bit entropy density than RDRAND DRBG output. It must be run through an approved conditioning function before use as key material. |
| 13 | Virtualization behavior | KVM: passes RDSEED through when host supports; can be suppressed via `cpuid` option. VMware: passed through on supported hardware versions. Hyper-V: passed through on Gen2 VMs. **Guest cannot distinguish hardware TRNG from hypervisor software emulation via CPUID.** Same guest-opacity problem as RDRAND, but with higher impact: seeding a DRBG from a hypervisor-emulated "entropy source" provides no additional entropy over what the hypervisor already controls. |
| 14 | Firmware / BIOS / microcode dependency | BIOS enable: not gated — present if CPU supports. Microcode: SRBDS mitigation requires Intel microcode update (June 2020, MCU_OPT_CTRL MSR at 0x123). AMD Zen 5 AMD-SB-7055 fix: requires AGESA firmware update (not CPU microcode alone — platform firmware update required). |
| 15 | Audit-card relevance | **Critical/Operational (FIPS)** |
| 16 | Recommended disposition when unused | Do not disable. Like RDRAND, RDSEED is always-available once the CPU supports it. Audit concern is whether software correctly uses 64-bit form only (on Zen 5 until patched), handles CF=0 exhaustion correctly, and seeds through an approved conditioning function. |
| 17 | Software utilization detection | No `/proc/crypto` entry — RDSEED feeds DRBGs, not the kernel crypto algorithm layer. Relevant detection: (a) check kernel source for `arch_get_random_seed_long()` use in the CSPRNG seed path; (b) check AMD Zen 5 firmware version against AGESA fix release; (c) `/proc/cmdline` for `random.trust_cpu=on` (less relevant for RDSEED than RDRAND, but same signal). |
| 18 | FIPS utilization requirement | **RDSEED is the 800-90B noise source interface.** Intel's CMVP Entropy Source Validation certificates (E65/E66/E232) cover the underlying TRNG that RDSEED exposes. RDSEED output satisfies NIST SP 800-90B requirements when used correctly as a seed source for an approved DRBG (SP 800-90A). However: on AMD Zen 5 systems without the AGESA fix, RDSEED (16/32-bit forms) fails 800-90B noise source requirements because it produces non-random output while signaling success. |
| 19 | Active mitigation status path | `/sys/devices/system/cpu/vulnerabilities/srbds` — same values as RDRAND (see rdrand.md). SRBDS applies to both RDRAND and RDSEED. |
| 20 | Feature accessible vs advertised | Generally not BIOS-gated. On AMD Zen 5 without the AGESA fix: CPUID correctly reports RDSEED present, but 16/32-bit forms produce biased output — the feature is advertised but functionally broken for those operand sizes. |
| 21 | Guest-vs-host discrepancy risk | **HIGH** — same CPUID passthrough opacity as RDRAND. Additionally: a guest on an unpatched AMD Zen 5 host cannot detect the Zen 5 RDSEED bug from inside the guest. If the hypervisor does not filter RDSEED, the guest's DRBG seeding may incorporate biased entropy. |
| 22 | Notes | RDSEED and RDRAND have a clear architectural distinction: RDRAND returns DRBG output (SP 800-90A), RDSEED returns raw noise source output (SP 800-90B). Intel's design intent is that RDSEED is used to seed external DRBGs; RDRAND is the high-throughput consumer interface. For FIPS compliance, what matters is that the DRBG being seeded (whether Linux's internal CSPRNG or an application-level DRBG) receives adequate entropy — RDSEED is the preferred hardware path for that seeding. |
| 23 | Sources | AMD Security Bulletin AMD-SB-7055; CVE-2025-62626; NIST SP 800-90B (January 2018); Intel SDM Vol 2B (RDSEED instruction); CMVP Entropy Validation Documents E65/E66/E232; CVE-2020-0543 / SRBDS; Intel Digital DRNG Software Implementation Guide; Phoronix AMD-SB-7055 coverage |

---

## NIST SP 800-90B Analysis

### RDSEED vs RDRAND: The Fundamental Distinction

The architectural difference between RDSEED and RDRAND is critical for understanding 800-90B compliance:

```
RDSEED output path:
[Physical Noise Source / TRNG]
        |
        v (raw entropy samples — higher variance, lower per-bit density)
[Minimal or no conditioning]
        |
        v
[RDSEED instruction output]
        |
        v (caller must condition before use as key material)
[Application DRBG seed input]

RDRAND output path:
[Physical Noise Source / TRNG]
        |
        v (internal, never exposed)
[AES-CTR DRBG per SP 800-90A]
        |
        v (conditioned, pseudorandom, full-entropy output)
[RDRAND instruction output]
```

**800-90B relevance:** SP 800-90B governs the noise source — the component that provides unpredictable raw data to the entropy source model. RDSEED is closer to exposing this noise source directly. RDRAND exposes only the conditioned DRBG output.

Intel's CMVP Entropy Source Validation covers the TRNG that both RDSEED and RDRAND ultimately derive from. The key audit question for FIPS deployments is whether the RDSEED-to-DRBG seeding chain preserves the validated entropy properties.

### SP 800-90B Entropy Source Model Components

Per SP 800-90B Section 2.2 (Entropy Source Model), applied to RDSEED:

| Component | RDSEED Implementation |
|-----------|----------------------|
| Noise source | On-die thermal/shot noise source (physical, non-deterministic) |
| Digitization | Hardware analog-to-digital conversion |
| Conditioning | Minimal — RDSEED provides raw or lightly conditioned noise source output |
| Health tests | Continuous: Repetition Count Test + Adaptive Proportion Test (SP 800-90B Section 4) |

### Health Tests

NIST SP 800-90B Section 4 requires two approved continuous health tests:

1. **Repetition Count Test (Section 4.4.1):** Detects catastrophic failures where the noise source is stuck. If C consecutive identical values are observed, the health test fails. Cutoff C = 1 + ceil(-log2(α) / H) where H is the assessed min-entropy per sample.

2. **Adaptive Proportion Test (Section 4.4.2):** Detects large entropy reductions from physical environmental changes. Window W=1024 for non-binary sources. If a single value appears T or more times in W samples, the health test fails.

Intel's on-die hardware runs both tests continuously on raw noise source samples. A health test failure causes RDSEED to return CF=0 (exhaustion/failure), not CF=1 with a bad value. This is the correct behavior per 800-90B.

**AMD Zen 5 violation:** The AMD-SB-7055 bug causes RDSEED (16/32-bit) to return biased values (zero) while signaling CF=1 (success), bypassing the correct failure signaling. This is a direct violation of the behavioral contract that SP 800-90B health test failures must be reported to the caller via the failure interface.

### Restart Tests

SP 800-90B Section 3.1.4 requires restart tests: the entropy source must produce outputs after a restart that are statistically indistinguishable from post-initialization outputs. RDSEED's hardware entropy source satisfies this at the physics level; the AMD Zen 5 bug may affect this guarantee until the AGESA fix is applied.

---

## AMD-SB-7055 Detail (Zen 5 RDSEED Bug)

**Bulletin:** AMD-SB-7055
**CVE:** CVE-2025-62626
**CVSS:** 7.2 (High)
**Disclosed:** October 2025 (discovered via Linux kernel mailing list by Meta engineer)
**Affected:** All AMD Zen 5 processors with 16-bit or 32-bit RDSEED instruction use

### Affected Processor Lines

- AMD Ryzen 9000 series (desktop, Strix Point, Granite Ridge)
- AMD Ryzen AI 300 series (mobile)
- AMD Ryzen Z2 series
- AMD Threadripper 9000 series
- AMD EPYC 9005 series (Genoa-based Zen 5)

### Technical Description

On Zen 5 CPUs, the RDSEED instruction has an operand-size-specific defect:

- `RDSEED r16`: returns 0 at a statistically anomalous rate, CF=1 (falsely signals success)
- `RDSEED r32`: returns 0 at a statistically anomalous rate, CF=1 (falsely signals success)
- `RDSEED r64`: **not affected** — returns valid entropy, CF=1 on success

The 16-bit and 32-bit forms are documented as returning the lower 16 or 32 bits of the 64-bit noise source output. The defect is that these truncated forms produce zero output rather than truncated valid entropy, while the success flag (CF) is not cleared to signal failure.

### Impact Assessment

Software that calls `RDSEED r32` to seed a DRBG receives a stream of zeros as "entropy," producing a DRBG with no effective entropy. Key generation, nonce generation, and session key establishment seeded via affected RDSEED forms on Zen 5 prior to the fix may produce predictable output.

This is not a side-channel — it is a functional defect that produces incorrect output rather than leaking correct output.

### Fix

- **EPYC 9005:** AGESA firmware update, available late October 2025
- **Ryzen 9000 / Ryzen AI 300 / Ryzen Z2 / Threadripper 9000:** AGESA update, available November 25, 2025
- **Workaround until patched:** Use only `RDSEED r64`. Software that already used only 64-bit RDSEED is unaffected.

### UMRS Relevance

This bug affects any Zen 5 deployment where:
- The system boots on unpatched firmware
- Software (including the Linux kernel's `arch_get_random_seed_long()`) uses the 16-bit or 32-bit RDSEED form

The Linux kernel uses `RDSEED r64` via `arch_get_random_seed_long()`, so the kernel's internal entropy pool seeding is **not affected**. However, application-level code using 16/32-bit RDSEED intrinsics directly is affected.

For UMRS audit purposes: a future CPU probe should check BIOS/firmware version on Zen 5 systems against AMD's AGESA fix release version.

---

## CVE / Vulnerability Table

| CVE | Name | Year | CVSS | Description | Fix | Relevance |
|-----|------|------|------|-------------|-----|-----------|
| CVE-2025-62626 | AMD-SB-7055 Zen 5 RDSEED defect | 2025 | 7.2 (High) | 16/32-bit RDSEED returns zero with CF=1 on Zen 5, producing non-random output while signaling success | AGESA firmware update (Nov 2025) | **DIRECT** — RDSEED produces non-random output on affected hardware |
| CVE-2020-0543 | SRBDS (Special Register Buffer Data Sampling) | 2020 | 6.5 | RDSEED and RDRAND values leak across cores via shared staging buffer | Intel microcode update June 2020 | **DIRECT** — RDSEED values observable by co-tenant on same physical core |

---

## Linux Detection

### Layer 1 — Hardware Presence

```
/proc/cpuinfo: look for 'rdseed' in flags line
```

Safe Rust detection:
```rust
// std::is_x86_feature_detected!("rdseed")  -- safe, no unsafe needed
// Or parse /proc/cpuinfo flags line for "rdseed"
```

### Zen 5 Firmware Version Check

For AMD Zen 5 systems, a complete RDSEED audit requires checking the BIOS/firmware version:

```
/sys/class/dmi/id/bios_version  -- OEM firmware version string
/sys/class/dmi/id/bios_date     -- firmware release date
```

Compare against AMD's AGESA fix release dates:
- EPYC 9005: AGESA fix released late October 2025
- Ryzen 9000 / AI 300 / Threadripper 9000: AGESA fix released November 25, 2025

If the system is AMD Zen 5 family and the firmware predates these fix dates, flag as **HIGH finding** with recommendation to apply AGESA update.

### SRBDS Mitigation Status

```
/sys/devices/system/cpu/vulnerabilities/srbds
```

Same values as RDRAND — see rdrand.md for full value table.

---

## Posture Check Specification

### RDSEED Posture Check (for future CPU probe)

1. Verify `rdseed` flag in `/proc/cpuinfo` (Layer 1 — hardware present)
2. If AMD Zen 5 CPU family:
   - Check `/sys/class/dmi/id/bios_date` against AGESA fix release date
   - If firmware predates fix: flag **HIGH** — 16/32-bit RDSEED produces non-random output
   - Note: Linux kernel uses 64-bit form, so kernel entropy pool is safe; application code may not be
3. Check `/sys/devices/system/cpu/vulnerabilities/srbds`:
   - `Mitigation: Microcode` — pass
   - `Vulnerable` or `Vulnerable: No microcode` — **HIGH finding**
   - `Unknown: Dependent on hypervisor status` — **INFORMATIONAL** with VM uncertainty note

### Connection to Existing UMRS Signal

`IndicatorId::RandomTrustCpu` applies to RDRAND (see rdrand.md). RDSEED itself is not directly surfaced by an existing posture indicator. The AMD-SB-7055 finding (firmware version check for Zen 5) is a **new signal** not yet in the catalog — it belongs in a future CPU probe phase as a dedicated `IndicatorId::RdseedZen5Firmware` or similar.

---

## Software Fallback Risk

RDSEED has no software fallback — either the hardware is present or it is not. Linux falls back to software entropy collection (interrupt jitter, `/dev/random` pool) when RDSEED is absent, which is well-studied and FIPS-validated in RHEL's entropy collection path.

The critical fallback case is the AMD Zen 5 bug: the hardware is present and signals success, but produces non-random output. There is no automatic kernel-level detection of this condition — the firmware fix is required to restore correct behavior.

---

## Sources

- [AMD Security Bulletin AMD-SB-7055](https://www.amd.com/en/resources/product-security/bulletin/amd-sb-7055.html)
- [AMD-SB-7055 Phoronix coverage](https://www.phoronix.com/news/AMD-SB-7055-RDSEED-Zen-5)
- [Tom's Hardware: AMD Zen 5 RDSEED vulnerability](https://www.tomshardware.com/pc-components/cpus/amd-confirms-security-vulnerability-on-zen-5-based-cpus-that-generates-potentially-predictable-keys-rdseed-fix-coming-through-an-agesa-firmware-update-for-desktop-chips)
- [NIST SP 800-90B (January 2018)](https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-90B.pdf)
- [Intel CMVP Entropy Certificate E66](https://csrc.nist.gov/CSRC/media/projects/cryptographic-module-validation-program/documents/entropy/E66_PublicUse.pdf)
- [Intel Digital DRNG Software Implementation Guide](https://www.intel.com/content/www/us/en/developer/articles/guide/intel-digital-random-number-generator-drng-software-implementation-guide.html)
- [Linux kernel SRBDS documentation](https://docs.kernel.org/admin-guide/hw-vuln/special-register-buffer-data-sampling.html)
- [LWN: Pitchforks for RDSEED](https://lwn.net/Articles/961121/)
- [SecPod: RDSEED Vulnerability AMD Zen 5](https://www.secpod.com/blog/rdseed-vulnerability-in-amd-zen-5-a-threat-to-hardware-randomness-integrity/)
