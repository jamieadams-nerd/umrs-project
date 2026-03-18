# Software Fallback Risk Matrix

**Phase:** 1H
**Completed:** 2026-03-18
**Scope:** Per-primitive analysis of software fallback implementations: timing properties,
cache-timing attack history, FIPS validation status, and risk ratings for UMRS posture decisions.

---

## Overview

Every hardware crypto extension has a software fallback path that the kernel and OpenSSL use
when the hardware feature is unavailable. The security properties of these fallbacks vary
significantly. Some software implementations introduce timing side-channels that can leak
key material; others are constant-time by design.

**UMRS posture principle:** "Hardware present, software not using it" is a Critical finding
(see Phase 1H summary). "Hardware absent, safe software fallback active" is an Important
finding. "Hardware absent, timing-vulnerable software fallback active" is a Critical finding
on systems processing CUI or handling FIPS-encrypted data.

---

## AES

### Background: The T-Table Cache-Timing Vulnerability

Classical AES software implementations use four 1024-byte lookup tables (T-tables) to
implement the SubBytes, ShiftRows, and MixColumns operations efficiently. The key vulnerability:
array indices are derived from key material, and cache line access patterns vary depending on
the key and plaintext values.

In 2005, Daniel J. Bernstein demonstrated a practical key-recovery attack against OpenSSL's
AES implementation on an Intel Pentium III using this timing channel (Bernstein, "Cache-timing
attacks on AES," 2005-04-14). The attack required only ~1.4 million AES queries and ~60 hours
of analysis. Subsequent work by Osvik, Shamir, and Tromer (2006) refined the attack to operate
in cross-core contexts.

**The fix is hardware:** AES-NI executes in data-independent time, completely eliminating
this attack class. The hardware performs the SubBytes, ShiftRows, and MixColumns operations
inside the CPU in constant time.

### Fallback Implementations in Order of Safety

| Implementation | Constant-time? | ISA requirement | FIPS-validated? | Risk rating | Notes |
|---------------|---------------|-----------------|-----------------|-------------|-------|
| `aesni_intel` (AES-NI) | Yes — hardware | AES-NI | Yes (RHEL 9 #4746, #4857) | NONE | Preferred; eliminates all known timing attacks |
| VPAES (Vector Permutation AES) | Yes — bitsliced | SSSE3 | Yes | LOW | Used when AES-NI absent, SSSE3 present |
| BSAES (Bit-sliced AES) | Yes — bitsliced | SSSE3 or NEON | Yes | LOW | Used in some paths (TLS); constant-time |
| `aes-generic` (T-table, C) | **NO** | None | Yes (CAVP tested) | **HIGH** | Validated but timing-vulnerable |

**Critical note on `aes-generic`:** It is FIPS-validated at the algorithm level (CAVP known-
answer tests pass), but FIPS validation does not guarantee resistance to side-channel attacks.
Bernstein's attack and its successors work against CAVP-validated T-table implementations.
Running `aes-generic` on a system handling CUI is a posture finding regardless of FIPS mode.

### VPAES / BSAES Technical Details

VPAES (Vector Permutation AES) was designed by Mike Hamburg as a SSSE3-based constant-time
AES implementation. It uses the SSSE3 PSHUFB (byte shuffle) instruction to implement the
AES S-box via precomputed vector permutation tables. Since PSHUFB accesses are index-
independent with respect to cache behavior (the full 128-bit vector is loaded), the
implementation avoids data-dependent cache lines.

In OpenSSL, the fallback selection logic is:
1. AES-NI available → `aesni_intel` (hardware, constant-time)
2. No AES-NI, SSSE3 available → VPAES or BSAES (software, constant-time)
3. No AES-NI, no SSSE3 → integer-only software AES (a reduced S-box form, not full T-table;
   reportedly more resistant than classic T-table but not fully analyzed for timing)

On modern RHEL 10 targets (Westmere+), SSSE3 is always present if the CPU lacks AES-NI
only due to hypervisor masking. In practice, if `aesni_intel` is absent on a
Westmere-or-newer system, VPAES is the fallback — not T-table AES.

**Risk reassessment for RHEL 10 targets:** The highest-risk scenario is not "AES-NI absent
→ T-table AES" but rather "AES-NI masked by hypervisor → VPAES active, which is
constant-time but slower." The T-table scenario primarily affects pre-Westmere or
deliberately misconfigured systems.

---

## SHA-2 (SHA-256, SHA-512)

### Timing Properties of SHA Software Implementations

SHA-256 and SHA-512 are Merkle-Damgard hash functions. The compression function consists
entirely of bitwise operations (AND, OR, XOR, rotate) and modular addition. These operations
are inherently data-independent from a cache-timing perspective: there are no table lookups,
no conditional branches on data values, and no key-dependent memory accesses.

**Conclusion:** SHA-2 software implementations are inherently constant-time with respect
to cache-timing attacks. The `sha256-generic` fallback does not introduce timing
vulnerability in the same sense as T-table AES.

| Implementation | Constant-time? | ISA requirement | FIPS-validated? | Risk rating | Notes |
|---------------|---------------|-----------------|-----------------|-------------|-------|
| `sha256-ni` / `sha512-ni` | Yes — hardware | SHA-NI | Yes | NONE | Intel SHA Extensions |
| `sha256-avx2` / `sha512-avx2` | Yes | AVX2 | Yes | LOW | Assembler optimized |
| `sha256-avx` / `sha512-avx` | Yes | AVX | Yes | LOW | Assembler optimized |
| `sha256-ssse3` / `sha512-ssse3` | Yes | SSSE3 | Yes | LOW | |
| `sha256-generic` / `sha512-generic` | Yes | None | Yes | LOW | Inherently constant-time |

**Risk justification for sha256-generic LOW:** SHA software is not subject to the cache-
timing attack class that afflicts AES T-tables. The only timing concern with SHA is
length-extension attacks, which are a protocol-level issue unrelated to the CPU feature
or the specific implementation variant.

**Performance note:** `sha256-generic` is 3–7x slower than `sha256-ni` on modern hardware.
For high-throughput systems, the absence of SHA-NI is a performance finding, not a
security finding.

---

## GHASH (GCM Authentication Tag)

GHASH is the Galois field multiplication used for the authentication tag in AES-GCM. Unlike
AES, software GHASH implementations using table-based approaches have known timing concerns.

| Implementation | Constant-time? | ISA requirement | FIPS-validated? | Risk rating | Notes |
|---------------|---------------|-----------------|-----------------|-------------|-------|
| `ghash-clmulni-intel` | Yes — hardware | PCLMULQDQ | Yes | NONE | Carry-less multiply instruction |
| `ghash-ce` (ARM) | Yes — hardware | ARMv8 PMULL | Yes | NONE | |
| `ghash-generic` (software) | Depends on impl | None | Yes | MEDIUM | Table-based GHASH can have timing issues |

**Concern with `ghash-generic`:** Some software GHASH implementations use 4-bit or 8-bit
windowed tables, which can leak information about the authentication key via cache timing
in multi-tenant environments. In OpenSSL, the software GHASH is implemented using a
64-entry 128-bit table with pclmulqdq-inspired reduction; the constant-time properties
depend on the specific OpenSSL version and build flags.

**For UMRS posture:** On FIPS-mode systems, AES-GCM is the primary AEAD mode. If
`ghash-clmulni-intel` is absent in `/proc/crypto`, the GCM authentication path is
software-only. This warrants a MEDIUM posture finding when AES-NI is available but
PCLMULQDQ is masked.

---

## CRC-T10DIF

| Implementation | Constant-time? | ISA requirement | FIPS-validated? | Risk rating | Notes |
|---------------|---------------|-----------------|-----------------|-------------|-------|
| `crct10dif-pclmul` | Yes — hardware | PCLMULQDQ | N/A (not FIPS algorithm) | LOW | Storage integrity only |
| `crct10dif-generic` | Depends | None | N/A | LOW | Not a security-critical primitive |

CRC-T10DIF is a storage integrity primitive, not a cryptographic hash. It does not appear
in FIPS-validated code paths. The software fallback carries negligible security risk.

---

## Summary Risk Matrix

| Primitive | Hardware driver | Software fallback | Fallback timing-safe? | FIPS-validated fallback? | Posture finding if HW absent |
|-----------|----------------|-------------------|----------------------|--------------------------|------------------------------|
| AES | `aesni_intel` / `aes-ce` | `aes-generic` (T-table) | **NO** | Yes (algorithm only) | **CRITICAL** |
| AES (SSSE3 fallback) | `aesni_intel` | VPAES / BSAES | **Yes** | Yes | MEDIUM (performance degradation) |
| AES-GCM | `generic-gcm-aesni` | `gcm(aes-generic)` | **NO** (via T-table AES) | Yes | **CRITICAL** |
| GHASH | `ghash-clmulni-intel` | `ghash-generic` | Partial | Yes | MEDIUM |
| SHA-256 | `sha256-ni` | `sha256-generic` | Yes | Yes | LOW (performance only) |
| SHA-512 | `sha512-ni` | `sha512-generic` | Yes | Yes | LOW (performance only) |
| CRC-T10DIF | `crct10dif-pclmul` | `crct10dif-generic` | Not applicable | Not a FIPS algorithm | INFO |

---

## FIPS Validation Scope Clarification

FIPS validation (CAVP + CMVP) tests algorithm correctness via known-answer tests (KATs). It
does not test or certify resistance to side-channel attacks unless the security policy
explicitly claims side-channel resistance.

**Implication:** `aes-generic` is FIPS-validated in the sense that it produces correct
AES outputs. It is NOT validated as side-channel resistant. A FIPS-mode system that falls
back to `aes-generic` is technically operating within its FIPS certificate boundary (because
the algorithm correctness is validated), but is exposed to the Bernstein cache-timing attack
class.

The RHEL OpenSSL FIPS Security Policy (#4746, #4857) covers both the AES-NI and software
paths as part of the Operational Environment testing. However, the policy does not make
claims about resistance to cache-timing attacks on the software path.

For UMRS posture purposes: treat `aes-generic` active on a FIPS system as a CRITICAL finding,
not merely an informational one.

---

## Sources

- [Bernstein: Cache-timing attacks on AES (2005-04-14)](https://cr.yp.to/antiforgery/cachetiming-20050414.pdf)
- [Red Hat blog: AES timing attacks on OpenSSL](https://www.redhat.com/en/blog/its-all-question-time-aes-timing-attacks-openssl)
- [Osvik, Shamir, Tromer: Cache Attacks and Countermeasures (2006)](https://link.springer.com/chapter/10.1007/11605805_1)
- [A Cache Timing Attack on AES in Virtualization Environments (FC 2012)](https://fc12.ifca.ai/pre-proceedings/paper_70.pdf)
- [Ubuntu 20.04 Kernel Crypto API CMVP Security Policy (FIPS scope)](https://csrc.nist.gov/CSRC/media/projects/cryptographic-module-validation-program/documents/security-policies/140sp3928.pdf)
- [RHEL 9 OpenSSL FIPS Provider Security Policy #4746](https://csrc.nist.gov/CSRC/media/projects/cryptographic-module-validation-program/documents/security-policies/140sp4746.pdf)
- [SHA-2 Wikipedia — timing properties of bitwise-operations-only design](https://en.wikipedia.org/wiki/SHA-2)
- [Linaro: Accelerated AES for Arm64 Linux Kernel](https://old.linaro.org/blog/accelerated-aes-for-the-arm64-linux-kernel/)
- Linux kernel `arch/x86/crypto/aesni-intel_glue.c` — VPAES/BSAES integration
