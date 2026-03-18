# Phase 1H Summary: /proc/crypto & Software Utilization Reference

**Phase:** 1H
**Completed:** 2026-03-18
**Agent:** researcher
**Input phases:** 1A (crypto-accel), 1B (entropy-bigint); Phase 0.5 spec (cpu-matrix.md v3)

---

## What Phase 1H Produced

Four reference files in `.claude/references/cpu-extensions/proc-crypto/`:

| File | Content |
|------|---------|
| `proc-crypto-format.md` | `/proc/crypto` entry structure, field semantics, priority selection, FIPS mode interaction |
| `driver-feature-mapping.md` | Complete CPU feature → kernel driver → `/proc/crypto` entry mapping for x86 and ARM |
| `openssl-fips-chain.md` | OpenSSL FIPS provider architecture, CMVP certificate status, OPENSSL_ia32cap, hardware path selection |
| `software-fallback-risk.md` | Per-primitive fallback risk matrix: timing properties, FIPS validation scope, risk ratings |

---

## Layer 2 Detection Concept

Phase 1A-1B established Layer 1 signals: **does the CPU offer this feature?** Layer 1 reads
`/proc/cpuinfo` flags, CPUID, or sysfs paths to determine hardware capability.

Phase 1H establishes the conceptual and technical foundation for Layer 2 signals: **is
deployed software actually using it?**

The security finding at the intersection:

```
Layer 1: CPU has AES-NI → aes flag present in /proc/cpuinfo
Layer 2: Kernel uses AES-NI → aesni_intel present in /proc/crypto with priority 300

IF Layer 1 = YES AND Layer 2 = NO:
    → AES-NI present but kernel is using software AES (aes-generic, priority 100)
    → Classification: CRITICAL (timing-vulnerable fallback active on hardware that could eliminate it)
```

This is the `CapabilityUnused` contradiction kind proposed for Phase 1J's Rust data
structure review.

---

## Posture Check Specifications

The following posture checks emerge from Phase 1H research. Each is a candidate for a
proposed `IndicatorId` in the UMRS posture catalog.

### ProcCrypto::AesHardwareActive

**What it checks:** Kernel crypto API is routing AES through hardware acceleration.

```
Read: /proc/crypto
Parse: all entries where name = "aes"
Verify:
  - Entry with driver = "aesni_intel" (x86) or "aes-ce" (ARM) exists
  - selftest = "passed" on that entry
  - fips_allowed = "yes" on that entry (FIPS-mode systems only)
  - priority of hardware entry > priority of "aes-generic" entry
```

**Pass:** Hardware AES driver registered, tested, and prioritized above software fallback.
**Fail:** Hardware driver absent → software T-table AES is the kernel default.
**Finding severity when hardware present (Layer 1 YES, Layer 2 NO):** CRITICAL

---

### ProcCrypto::AesGcmHardwareActive

**What it checks:** AES-GCM (the primary FIPS AEAD) uses both AES-NI and PCLMULQDQ.

```
Read: /proc/crypto
Parse: all entries where name matches "gcm(aes)" or "rfc4106(gcm(aes))"
Verify:
  - Entry with driver = "generic-gcm-aesni" (x86) or "gcm-aes-ce" (ARM) exists
  - selftest = "passed"
  - fips_allowed = "yes"
  Separately verify:
  - Entry with driver = "ghash-clmulni-intel" (x86) or "ghash-ce" (ARM) exists
  - selftest = "passed"
```

**Rationale:** AES-GCM requires both the AES acceleration (AES-NI) and the GHASH
acceleration (PCLMULQDQ/PMULL). If PCLMULQDQ is masked while AES-NI is present,
the GCM authentication path degrades to software GHASH.

---

### ProcCrypto::Sha256HardwareActive

**What it checks:** SHA-256 kernel path uses hardware SHA extension.

```
Read: /proc/crypto
Parse: all entries where name = "sha256"
Verify:
  - Entry with driver = "sha256-ni" (x86) or "sha256-ce" (ARM) exists
  - selftest = "passed"
  - priority of hardware entry > all assembler and generic entries
```

**Pass:** SHA-NI (or ARM SHA2-CE) is registered and selected by default.
**Fail — hardware absent:** Assembler fallback (`sha256-avx2`, priority 170) or generic
(priority 100) is the default.
**Finding severity (Layer 1 SHA-NI YES, Layer 2 NO):** MEDIUM (SHA software is
constant-time; the risk is performance, not timing attack).
**Finding severity (Layer 1 SHA-NI NO, software only):** LOW (SHA-2 generic is safe).

---

### ProcCrypto::SelftestAllPassed

**What it checks:** No registered crypto driver has a failed self-test.

```
Read: /proc/crypto
Parse: all entries
Verify: no entry has selftest = "failed"
```

**Finding:** Any `selftest: failed` entry is a Critical finding in FIPS mode — the
kernel may be silently falling back past the failed implementation to the next priority.

---

### ProcFips::KernelAndDriversConsistent

**What it checks:** When `/proc/sys/crypto/fips_enabled = 1`, hardware crypto drivers
carry `fips_allowed: yes`.

```
Read: /proc/sys/crypto/fips_enabled
If value = 1:
  Read: /proc/crypto
  For each hardware driver entry (aesni_intel, sha256-ni, ghash-clmulni-intel, aes-ce, sha256-ce):
    Verify: fips_allowed = "yes"
```

**Finding:** FIPS mode active but hardware driver does not carry `fips_allowed: yes`
indicates a kernel version mismatch or missing patch. The kernel would fall back to the
`fips_allowed: yes` software driver — meaning the system is FIPS-compliant but not
hardware-accelerated.

---

## Connection to Existing UMRS Signals

### SignalId::ProcFips

`SignalId::ProcFips` is the existing Layer 1 signal for `/proc/sys/crypto/fips_enabled`.

Phase 1H signals complement `ProcFips` rather than replacing it:

| Signal | Layer | Interface | What it tells you |
|--------|-------|-----------|------------------|
| `ProcFips` (existing) | 1 | `/proc/sys/crypto/fips_enabled` | Kernel FIPS enforcement on/off |
| `ProcCrypto::AesHardwareActive` (proposed) | 2 | `/proc/crypto` | Hardware AES is default kernel selection |
| `ProcCrypto::AesGcmHardwareActive` (proposed) | 2 | `/proc/crypto` | Full GCM path (AES-NI + PCLMULQDQ) active |
| `ProcCrypto::Sha256HardwareActive` (proposed) | 2 | `/proc/crypto` | SHA hardware active |
| `ProcCrypto::SelftestAllPassed` (proposed) | 2 | `/proc/crypto` | No driver failures |

### SignalId::RandomTrustCpu

The `RandomTrustCpu` signal (Phase 1B) is a Layer 1 signal for entropy hardware quality.
Phase 1H's Layer 2 concept extends similarly: future work could verify that the kernel's
RNG implementation is actually consuming hardware entropy rather than falling back to
software DRBG. This would be a `ProcCrypto::RngHardwareActive` signal reading the `rng`
and `drbg` entries in `/proc/crypto`.

---

## Proposed IndicatorId Signals for Phase 1J Review

The following new `IndicatorId` values are proposed for consideration during Phase 1J
(post-research review by rust-developer and security-engineer):

```
CryptoDriverAesNi           — /proc/crypto: aesni_intel present, passed, fips_allowed
CryptoDriverAesGcmNi        — /proc/crypto: generic-gcm-aesni present, passed, fips_allowed
CryptoDriverGhashClmul      — /proc/crypto: ghash-clmulni-intel present, passed, fips_allowed
CryptoDriverSha256Ni        — /proc/crypto: sha256-ni present, passed
CryptoDriverArmAesCe        — /proc/crypto: aes-ce present, passed, fips_allowed (ARM)
CryptoDriverArmSha256Ce     — /proc/crypto: sha256-ce present, passed (ARM)
CryptoSelftestAllPassed     — /proc/crypto: no selftest: failed entries
CryptoFipsConsistent        — /proc/sys/crypto/fips_enabled ↔ fips_allowed alignment
```

**Contradiction kind for Phase 1J:** `ContradictionKind::CapabilityUnused` — Layer 1
hardware feature present (e.g., AES-NI in CPUID), Layer 2 utilization signal absent
(aesni_intel not in `/proc/crypto`). This is distinct from existing contradiction kinds.

---

## Key Findings for Phase 1I Synthesis

1. `/proc/crypto` is the preferred Layer 2 detection interface for all crypto extensions.
   It is more reliable than inspecting ELF binaries for AES-NI opcodes.

2. The T-table AES timing vulnerability (Bernstein 2005) is the highest-risk fallback
   scenario. AES-NI elimination of this attack class is the primary security benefit of
   the feature.

3. SHA-2 software is inherently constant-time. SHA-NI absence is a performance finding,
   not a security finding.

4. RHEL 10 FIPS validation is in progress as of 2026-03-18 (OE update pending). The
   validated binary is the same as the RHEL 9 certificate. This is a compliance nuance
   for audit reports.

5. OPENSSL_ia32cap is a testing/debugging mechanism. Its presence in a production
   environment masking AES-NI would be a Critical configuration finding.

6. ARM platforms use `aes-ce`, `sha256-ce`, `sha512-ce`, `ghash-ce` drivers — same
   structural role as x86 hardware drivers, lower priority numbers (200 vs 300).

7. The `fips_allowed` field on hardware drivers required explicit kernel patches (mainline
   accepted; present in RHEL 9/10 kernels). Older kernels may lack these patches.

---

## Files Produced

```
.claude/references/cpu-extensions/proc-crypto/
├── proc-crypto-format.md          (written before this phase — entry format, field semantics)
├── driver-feature-mapping.md      (this phase — CPU feature → driver → /proc/crypto mapping)
├── openssl-fips-chain.md          (this phase — OpenSSL FIPS architecture, CMVP status)
├── software-fallback-risk.md      (this phase — timing risk per primitive)
└── phase-1h-summary.md            (this file)
```

---

## Next Phase

Phase 1I (Matrix Synthesis) — aggregate all Phase 1A-1H findings into the 23-column master
matrix and produce the knowledge index. Phase 1H material contributes:
- Column 17 (Software utilization detection method): `/proc/crypto` driver name + priority check
- Column 18 (FIPS utilization requirement): per-feature FIPS path requirements
- Layer 2 posture check specifications for the audit-card recommendations

Sources for this phase:
- [Kernel Crypto API Architecture (docs.kernel.org)](https://docs.kernel.org/crypto/architecture.html)
- [Red Hat FIPS compliance](https://access.redhat.com/compliance/fips)
- [Bernstein: Cache-timing attacks on AES](https://cr.yp.to/antiforgery/cachetiming-20050414.pdf)
- [RHEL 9 OpenSSL FIPS Security Policy CMVP #4746](https://csrc.nist.gov/CSRC/media/projects/cryptographic-module-validation-program/documents/security-policies/140sp4746.pdf)
