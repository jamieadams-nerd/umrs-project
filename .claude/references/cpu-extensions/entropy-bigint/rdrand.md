# RDRAND (Hardware Random Number Generator)

**Category:** Entropy & Random Generation (Category 4)
**Classification:** Critical/Operational (FIPS)
**Phase:** 1B
**Date:** 2026-03-18

---

## 23-Column Matrix Profile

| # | Column | Value |
|---|--------|-------|
| 1 | Feature name | RDRAND |
| 2 | Vendor | Both (Intel & AMD) |
| 3 | Category | Entropy & Random Generation |
| 4 | Purpose | Hardware random number generator: executes a DRBG (AES-CTR DRBG per NIST SP 800-90A) seeded by an on-die TRNG. Returns 16, 32, or 64-bit values on success. |
| 5 | Example instructions | `RDRAND r16`, `RDRAND r32`, `RDRAND r64` (CF=1 on success, CF=0 on failure/retry needed) |
| 6 | CPUID detection | Leaf 1, ECX bit 30 |
| 7 | Linux `/proc/cpuinfo` flag | `rdrand` |
| 8 | Linux authoritative path | `/proc/cpuinfo` flag `rdrand`; secondary: `/sys/devices/system/cpu/vulnerabilities/srbds` (SRBDS mitigation status) |
| 9 | Minimum CPU generations | Intel: Ivy Bridge (2012, 3rd gen Core); AMD: Zen (Ryzen 1000, 2017) — not present on all Ivy Bridge SKUs (see caveats) |
| 10 | Security benefit | Provides hardware-seeded DRBG output, eliminating reliance on software entropy collection (keyboard timing, disk I/O jitter, etc.) during early boot or low-entropy environments. |
| 11 | Performance benefit | ~1 Gbps throughput on modern hardware; avoids entropy starvation blocking `getrandom()`. Critical for VM boot-time entropy availability. |
| 12 | Assurance caveats | (1) RDRAND returns DRBG output, NOT raw entropy — it does NOT satisfy NIST SP 800-90B as a noise source directly. (2) SRBDS (CVE-2020-0543): RDRAND output leaked across cores via MDS staging buffer — fixed by microcode. (3) AMD family 15h/16h: RDRAND returns stale values after suspend-to-RAM (kernel patch: clear CPUID bit on affected models). (4) AMD Zen 5: RDSEED bug (separate — see rdseed.md). (5) Intel early Ivy Bridge errata BV54: some Celeron SKUs reported RDRAND absent despite CPUID claim. (6) `random.trust_cpu=on` kernel parameter allows RDRAND output to credit entropy directly to kernel pool — this bypasses 800-90B noise source requirements and is flagged by `IndicatorId::RandomTrustCpu`. |
| 13 | Virtualization behavior | KVM: passes through by default when host supports; `cpuid` KVM option can suppress. VMware: passed through on supported hardware; masked in older VM hardware versions. Hyper-V: passed through on Gen2 VMs. **Guest cannot distinguish genuine hardware DRBG from hypervisor software emulation via CPUID alone.** |
| 14 | Firmware / BIOS / microcode dependency | Microcode: SRBDS mitigation requires Intel microcode update (June 2020, `MCU_OPT_CTRL` MSR at 0x123). AMD family 15h/16h: kernel disables CPUID flag for affected models — no BIOS fix available. BIOS enable: not gated; present if CPU supports. |
| 15 | Audit-card relevance | **Critical/Operational (FIPS)** |
| 16 | Recommended disposition when unused | Do not disable. RDRAND is always-available once CPU supports it. Audit concern is NOT whether it is present, but whether `random.trust_cpu=on` is set (bypasses 800-90B discipline). On FIPS systems: verify `random.trust_cpu` is NOT set to `on`. |
| 17 | Software utilization detection | `/proc/cmdline`: check for `random.trust_cpu=on` (finding if set on FIPS system). `IndicatorId::RandomTrustCpu` in UMRS posture catalog monitors this. Kernel driver: `rdrand` in `/proc/crypto` is not applicable — RDRAND feeds the kernel CSPRNG, not `/proc/crypto`. |
| 18 | FIPS utilization requirement | FIPS 140-3: RDRAND output alone does NOT constitute a NIST SP 800-90B-validated entropy source. The on-die TRNG that seeds RDRAND's DRBG has been separately validated (Intel CMVP Entropy Certificates E65/E66/E232). The distinction matters: `random.trust_cpu=on` credits DRBG output as entropy, which is architecturally sound but was not the design intent of SP 800-90B noise source validation. RHEL 10 default: `random.trust_cpu` is NOT set to `on`. |
| 19 | Active mitigation status path | `/sys/devices/system/cpu/vulnerabilities/srbds` — values: `Not affected`, `Vulnerable`, `Vulnerable: No microcode`, `Mitigation: Microcode`, `Mitigation: TSX disabled`, `Unknown: Dependent on hypervisor status` |
| 20 | Feature accessible vs advertised | Generally not BIOS-gated. Exceptions: (a) kernel disables CPUID flag on AMD family 15h/16h after suspend fix; (b) some early Ivy Bridge Celeron SKUs (errata BV54) advertise via CPUID but may not function correctly on all steppings. |
| 21 | Guest-vs-host discrepancy risk | **HIGH** — hypervisors can emulate RDRAND in software. Guest sees same CPUID bit regardless. `/proc/crypto` does not reveal RDRAND utilization. No guest-side mechanism distinguishes hardware from software emulation. |
| 22 | Notes | RDRAND is the consumer-facing interface; RDSEED is the raw seeding interface. See rdseed.md for the noise source level. The `random.trust_cpu` kernel parameter is the primary UMRS audit surface for RDRAND. Intel ESV certificate numbers: E65, E66, E232 (publicly available from NIST CMVP). |
| 23 | Sources | NIST SP 800-90B (January 2018); Intel SDM Vol 2B (RDRAND instruction); CMVP Entropy Validation Documents E65/E66/E232; CVE-2020-0543 / SRBDS kernel docs; AMD-SB-7055 (Zen 5 RDSEED, distinct); Linux kernel `arch/x86/kernel/cpu/rdrand.c`; Red Hat Customer Portal (SRBDS mitigation); LWN.net "random: possible ways towards NIST SP800-90B compliance" |

---

## NIST SP 800-90B Analysis

### RDRAND Architecture vs. 800-90B Requirements

NIST SP 800-90B specifies requirements for **entropy sources** — specifically the noise source component that provides unpredictable raw data. RDRAND's architecture is:

```
[On-Die TRNG / Physical Noise Source]
        |
        v (raw entropy, never exposed)
[DRBG: AES-128-CTR per SP 800-90A]
        |
        v (conditioned, pseudorandom output)
[RDRAND instruction output]
```

**Critical distinction for FIPS compliance:**

- The **TRNG** is the entropy source under 800-90B scrutiny.
- **RDRAND output** is DRBG output — it satisfies SP 800-90A (DRBG mechanisms), not 800-90B (entropy sources) directly.
- Intel separately submitted the underlying TRNG for SP 800-90B validation through NIST CMVP Entropy Source Validation (ESV).

### Intel CMVP Entropy Source Validation

Intel's Digital Random Number Generator (DRNG) has received NIST CMVP entropy source validation:

- **Certificate E65** (Document 714144): Intel DRNG SP800-90B non-IID assessment
- **Certificate E66** (Document 714145): Intel DRNG SP800-90B non-IID assessment (updated)
- **Certificate E232** (Document 714056): Intel DRNG SP800-90B non-IID assessment (additional platform coverage)

Key finding from public documents: The DRNG entropy source meets SP 800-90B non-IID requirements with min(H_r, H_c, H_I) = **0.6 bits of entropy per bit** of data from the noise source. The CBC-MAC conditioning component achieves H_out where 1-ε and ε << 2^-32.

This means Intel's TRNG has been validated as a **non-IID noise source** (the outputs are not independent and identically distributed, but there is documented and tested entropy). The DRBG conditioning step is what delivers the full-entropy 128-bit or 64-bit values from RDRAND.

### 800-90B Model Components

Per SP 800-90B Section 2.2 (Entropy Source Model):

| Component | RDRAND Implementation |
|-----------|----------------------|
| Noise source | On-die thermal/shot noise source (physical) |
| Digitization | Hardware analog-to-digital conversion |
| Conditioning | AES-CTR DRBG (SP 800-90A approved) |
| Health tests | Continuous: Repetition Count Test + Adaptive Proportion Test (SP 800-90B Section 4) |

### Health Test Requirements (SP 800-90B Section 4)

NIST SP 800-90B requires two approved continuous health tests (Section 4.4):

1. **Repetition Count Test** — detects catastrophic failures where noise source is "stuck" on a single value. Cutoff C = 1 + ceil(-log2(α) / H) where α is the false positive probability.

2. **Adaptive Proportion Test** — detects large entropy loss from physical environmental changes (temperature, humidity, electromagnetic interference). Window size W=1024 for binary sources.

Intel's implementation includes both tests running on raw noise source samples before the DRBG conditioning. The health tests operate continuously while the entropy source is running.

**Startup test requirement (Section 4.3):** Startup tests must run the continuous health tests over at least 1024 consecutive samples before the entropy source produces output. FIPS 140 determines the specific startup conditions for cryptographic modules.

### `random.trust_cpu` and 800-90B

The Linux kernel parameter `random.trust_cpu=on` instructs the kernel to credit RDRAND output directly as entropy to the kernel's internal CSPRNG pool. This is the relevant audit surface:

- **With `random.trust_cpu=off` (RHEL 10 default):** RDRAND output is mixed into the kernel entropy pool but does not directly credit entropy. The kernel uses additional entropy sources (interrupt timing, device I/O) per standard entropy accounting.
- **With `random.trust_cpu=on`:** RDRAND DRBG output is credited as entropy. This is architecturally sound IF Intel's TRNG 800-90B validation covers the deployment, but it bypasses independent entropy accounting.

**RHEL 10 default:** `random.trust_cpu` is NOT enabled by default. UMRS `IndicatorId::RandomTrustCpu` flags presence of `random.trust_cpu=on` as a Medium-impact finding.

---

## CVE / Vulnerability Table

| CVE | Name | Year | CVSS | Description | Fix | Relevance to RDRAND |
|-----|------|------|------|-------------|-----|---------------------|
| CVE-2020-0543 | SRBDS (Special Register Buffer Data Sampling / CrossTalk) | 2020 | 6.5 (CVSS 3.1) | RDRAND and RDSEED values leak across cores via shared staging buffer using MDS techniques. Unprivileged user on same physical host can extract values. | Intel microcode update (June 2020); introduces `IA32_MCU_OPT_CTRL` MSR at 0x123; RNGDS_MITG_DIS bit opt-out available. | **DIRECT** — values returned by RDRAND can be observed by a malicious co-tenant on the same physical core. |
| N/A (AMD errata) | AMD family 15h/16h suspend-resume RDRAND failure | ~2019 | N/A | After suspend-to-RAM, RDRAND on AMD family 15h and 16h processors returns stale/repeated values rather than fresh random data, while still signaling CF=1 (success). | Linux kernel patch: clears CPUID rdrand bit for affected CPU families; kernel will not use RDRAND on these CPUs. No BIOS fix. | **DIRECT** — silent failure mode producing non-random output. |
| Intel errata BV54 | Ivy Bridge Celeron RDRAND CPUID mismatch | ~2012–2013 | N/A | Some Ivy Bridge Celeron SKUs advertise RDRAND support via CPUID but the feature is absent or malfunctioning in specific steppings. | Errata documented; affected SKUs avoided in server deployments. | Informational — affects only specific low-end SKUs. |

### SRBDS Detail

SRBDS (CVE-2020-0543, Intel-SA-00232) is the most significant RDRAND security issue. Key facts:

- **Attack vector:** Another process on the same physical core (or sibling HT thread) can sample the shared staging buffer during RDRAND execution to recover the random value.
- **Why it matters:** RDRAND output is often used as key material. If an adversary recovers RDRAND output from another process, they can potentially derive session keys.
- **Microcode fix behavior:** After the June 2020 microcode update, RDRAND/RDSEED/EGETKEY instructions **overwrite** the staging buffer contents before any other logical processor can read them. This serializes RDRAND execution.
- **Performance impact:** Serialization increases RDRAND latency and reduces throughput when multiple threads call RDRAND simultaneously.
- **sysfs detection:** `/sys/devices/system/cpu/vulnerabilities/srbds`

---

## Linux Detection

### Layer 1 — Hardware Presence

```
/proc/cpuinfo: look for 'rdrand' in flags line
```

Safe Rust detection:
```rust
// std::is_x86_feature_detected!("rdrand")  -- safe, no unsafe needed
// Or parse /proc/cpuinfo flags line for "rdrand"
```

Note: On AMD family 15h/16h, the kernel may clear the CPUID flag even if the CPU originally advertised it. `/proc/cpuinfo` reflects the kernel-filtered view.

### Layer 2 — Utilization Audit

The primary Layer 2 surface for RDRAND is not `/proc/crypto` — RDRAND feeds the kernel's internal CSPRNG, not a kernel crypto algorithm. The audit surface is:

```
/proc/cmdline: check for 'random.trust_cpu=on'
```

UMRS posture catalog: `IndicatorId::RandomTrustCpu` (class: `KernelCmdline`, desired: `CmdlineAbsent("random.trust_cpu=on")`)

### SRBDS Mitigation Status

```
/sys/devices/system/cpu/vulnerabilities/srbds
```

Possible values:
- `Not affected` — CPU not in affected Family_Model/stepping list
- `Vulnerable` — Affected, mitigation disabled
- `Vulnerable: No microcode` — Affected, microcode update not installed
- `Mitigation: Microcode` — Protected by Intel microcode update
- `Mitigation: TSX disabled` — Only vulnerable with TSX; TSX is disabled
- `Unknown: Dependent on hypervisor status` — Guest VM, cannot determine host status

---

## Posture Check Specification

### RDRAND Posture Check (for future CPU probe)

1. Verify `rdrand` flag in `/proc/cpuinfo` (Layer 1 — hardware present)
2. Check `/proc/cmdline` for `random.trust_cpu=on` — if present on a FIPS system, flag as **MEDIUM finding**
3. Check `/sys/devices/system/cpu/vulnerabilities/srbds`:
   - `Mitigation: Microcode` — pass
   - `Vulnerable` or `Vulnerable: No microcode` — **HIGH finding** (RDRAND values can leak to co-tenant)
   - `Unknown: Dependent on hypervisor status` — flag as **INFORMATIONAL** with note about VM uncertainty

### Connection to Existing UMRS Signal

`IndicatorId::RandomTrustCpu` in `umrs-platform/src/posture/catalog.rs`:
- Class: `KernelCmdline`
- Live path: `/proc/cmdline`
- Desired: `CmdlineAbsent("random.trust_cpu=on")`
- Impact: `AssuranceImpact::Medium`
- Rationale: "Trusting CPU RNG unconditionally may not satisfy NIST SP 800-90B; RHEL 10 defaults to not trusting it."

The SRBDS vulnerability check (`/sys/devices/system/cpu/vulnerabilities/srbds`) is a **new signal** not yet in the catalog — it belongs in a future CPU probe phase.

---

## Software Fallback Risk

RDRAND has no software fallback — either the hardware is present or it is not. The kernel falls back to software entropy collection (interrupt jitter, device I/O, etc.) when RDRAND is absent. This fallback is well-studied and FIPS-validated in RHEL's OpenSSL FIPS provider.

The real fallback risk is the AMD suspend bug: the CPU returns CF=1 (success) with stale data. The kernel patch silently removes the feature for affected CPUs, so software effectively never sees the broken hardware. Any system running the patched kernel on affected AMD hardware is safe.

---

## Sources

- [NIST SP 800-90B (January 2018)](https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-90B.pdf)
- [Intel CMVP Entropy Certificate E66](https://csrc.nist.gov/CSRC/media/projects/cryptographic-module-validation-program/documents/entropy/E66_PublicUse.pdf)
- [Intel CMVP Entropy Certificate E232](https://csrc.nist.gov/CSRC/media/projects/cryptographic-module-validation-program/documents/entropy/E232_PublicUse.pdf)
- [NIST CMVP Entropy Validations](https://csrc.nist.gov/projects/cryptographic-module-validation-program/entropy-validations)
- [Linux kernel SRBDS documentation](https://docs.kernel.org/admin-guide/hw-vuln/special-register-buffer-data-sampling.html)
- [Intel SRBDS advisory (Intel-SA-00232)](https://www.intel.com/content/www/us/en/developer/articles/technical/software-security-guidance/advisory-guidance/special-register-buffer-data-sampling.html)
- [Intel SRBDS mitigation impact on Secure Key](https://www.intel.com/content/www/us/en/developer/articles/technical/software-security-guidance/best-practices/srbds-mitigation-impact-intel-secure-key.html)
- [LWN: random: possible ways towards NIST SP800-90B compliance](https://lwn.net/Articles/832027/)
- [AMD RDRAND suspend bug — LinuxReviews](https://linuxreviews.org/RDRAND_stops_returning_random_values_on_older_AMD_CPUs_after_suspend)
- [atsec: Entropy Source Validation for Intel DRNG](https://www.atsec.com/entropy-source-validation-esv-certificate-issued-for-the-intel-drng/)
- [LWN: FIPS-compliant random numbers for the kernel](https://lwn.net/Articles/877607/)
