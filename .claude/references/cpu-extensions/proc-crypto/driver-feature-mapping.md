# CPU Feature → Kernel Crypto Driver → /proc/crypto Mapping

**Phase:** 1H
**Completed:** 2026-03-18
**Scope:** Complete mapping of CPU features to kernel crypto driver names and /proc/crypto entries,
covering x86 and ARM/AArch64. Includes priority ranges, module names, and posture check guidance.

---

## Overview

The Linux Crypto API registers multiple implementations for each algorithm. When the kernel selects
an implementation for a caller, it picks the highest-priority registered driver for the requested
algorithm name. `/proc/crypto` exposes all registered implementations with their priorities, enabling
Layer 2 (software utilization) posture checks.

**Critical posture principle:** The presence of a hardware-accelerated driver in `/proc/crypto` with
a higher priority than any software fallback, combined with `selftest: passed`, is the strongest
guest-side signal that hardware crypto acceleration is the kernel's default selection.

---

## Priority Convention

Priority numbers are assigned by each driver at registration time. There is no strict POSIX
standard for the values, but observed clusters across mainline kernels are:

| Priority range | Typical source | Examples |
|----------------|---------------|---------|
| 0–99 | Built-in generic (C, no ISA) | `aes-generic` (100), `sha256-generic` (100) |
| 100–199 | x86 assembler, no hardware instructions | `aes-x86_64` (200 historically), `sha256-ssse3` (150) |
| 200–299 | Hardware instruction acceleration | `sha256-ni` (250), `aesni_intel` (300), `aes-ce` (200, ARM) |
| 300–499 | Hardware-merged compound/AEAD modes | `generic-gcm-aesni` (400), `rfc4106-gcm-aesni` (401) |

**Note on x86_64 assembler:** The `aes-x86_64` driver (pure assembler, no AES-NI) historically
appeared at priority ~150-200. It was removed from mainline kernels around 5.4 in favor of always
using `aesni_intel`. On modern RHEL 9/10 kernels it is not present; only `aes-generic` (100) and
`aesni_intel` (300) appear for the `aes` algorithm name on supported hardware.

---

## x86 Driver Mapping Table

### AES (Block Cipher)

| CPU Feature | `/proc/crypto` entry | driver | module | priority | type | Notes |
|-------------|---------------------|--------|--------|----------|------|-------|
| AES-NI | `aes` | `aesni_intel` | `aesni_intel` | 300 | cipher | Hardware AES; loadable module |
| None (software) | `aes` | `aes-generic` | `aes_generic` | 100 | cipher | T-table software fallback |
| SSSE3 (no AES-NI) | `aes` | (removed in RHEL 9+) | — | — | — | `aes-x86_64` removed ~Linux 5.4 |

### AES Compound Modes (AEAD / skcipher)

| CPU Feature | `/proc/crypto` entry | driver | module | priority | type | Notes |
|-------------|---------------------|--------|--------|----------|------|-------|
| AES-NI | `cbc(aes)` | `cbc-aes-aesni` | `aesni_intel` | 400 | skcipher | |
| AES-NI | `ctr(aes)` | `ctr-aes-aesni` | `aesni_intel` | 400 | skcipher | |
| AES-NI | `xts(aes)` | `xts-aes-aesni` | `aesni_intel` | 401 | skcipher | |
| AES-NI | `gcm(aes)` | `generic-gcm-aesni` | `aesni_intel` | 400 | aead | |
| AES-NI | `rfc4106(gcm(aes))` | `rfc4106-gcm-aesni` | `aesni_intel` | 400 | aead | IPsec GCM |
| None | `cbc(aes)` | `cbc(aes-generic)` | `aes_generic` | 100 | skcipher | Software fallback |
| None | `gcm(aes)` | `gcm(aes-generic)` | `aes_generic` | 100 | aead | Software fallback |

### SHA-2 Hashes

| CPU Feature | `/proc/crypto` entry | driver | module | priority | type | Notes |
|-------------|---------------------|--------|--------|----------|------|-------|
| SHA-NI | `sha256` | `sha256-ni` | `sha256_ni` | 250 | shash | SHA Extensions (Intel SHA) |
| SHA-NI | `sha224` | `sha224-ni` | `sha256_ni` | 250 | shash | SHA Extensions |
| AVX2 | `sha256` | `sha256-avx2` | `sha256_ssse3` | 170 | shash | AVX2 assembler |
| AVX | `sha256` | `sha256-avx` | `sha256_ssse3` | 160 | shash | AVX assembler |
| SSSE3 | `sha256` | `sha256-ssse3` | `sha256_ssse3` | 150 | shash | SSSE3 assembler |
| None | `sha256` | `sha256-generic` | `sha256_generic` | 100 | shash | Software fallback |
| SHA-NI | `sha512` | `sha512-ni` | `sha512_ni` | 250 | shash | SHA Extensions (SHA-512) |
| AVX2 | `sha512` | `sha512-avx2` | `sha512_ssse3` | 170 | shash | |
| AVX | `sha512` | `sha512-avx` | `sha512_ssse3` | 160 | shash | |
| SSSE3 | `sha512` | `sha512-ssse3` | `sha512_ssse3` | 150 | shash | |
| None | `sha512` | `sha512-generic` | `sha512_generic` | 100 | shash | Software fallback |

**Module note:** `sha256-ni`, `sha256-avx2`, `sha256-avx`, and `sha256-ssse3` are all provided
by the same kernel module `sha256_ssse3` (CONFIG_CRYPTO_SHA256_SSSE3). The module registers
whichever variants the runtime CPU supports. If SHA-NI is available, `sha256-ni` (priority 250)
wins over the assembler variants.

### GHASH / GMAC (PCLMULQDQ)

| CPU Feature | `/proc/crypto` entry | driver | module | priority | type | Notes |
|-------------|---------------------|--------|--------|----------|------|-------|
| PCLMULQDQ | `ghash` | `ghash-clmulni-intel` | `ghash_clmulni_intel` | 400 | shash | GCM authentication tag |
| None | `ghash` | `ghash-generic` | `ghash_generic` | 100 | shash | Software fallback |

**Security note:** `ghash-clmulni-intel` is required for hardware-accelerated AES-GCM. Without
it, GCM falls back to software GHASH, which can introduce timing variation in the authentication
tag computation.

### CRC-T10DIF (PCLMULQDQ)

| CPU Feature | `/proc/crypto` entry | driver | module | priority | type | Notes |
|-------------|---------------------|--------|--------|----------|------|-------|
| PCLMULQDQ | `crct10dif` | `crct10dif-pclmul` | `crct10dif_pclmul` | 200 | shash | T10 DIF CRC for storage |
| None | `crct10dif` | `crct10dif-generic` | `crct10dif_generic` | 100 | shash | Software fallback |

**UMRS relevance:** CRC-T10DIF is used for storage integrity, not direct cryptography. It is
lower priority for FIPS posture checks but relevant for audit of storage encryption paths.

### FIPS Allowed Status Summary (x86)

| driver | fips_allowed | Notes |
|--------|-------------|-------|
| `aesni_intel` | yes | Patched in mainline to allow FIPS mode |
| `aes-generic` | yes | Always FIPS-allowed; timing-vulnerable |
| `sha256-ni` | yes | |
| `sha256-ssse3` / `sha256-avx` / `sha256-avx2` | yes | Assembler; no timing risk for SHA |
| `sha256-generic` | yes | No timing concern for SHA-2 |
| `ghash-clmulni-intel` | yes | Patched in mainline |
| `ghash-generic` | yes | |
| `crct10dif-pclmul` | yes | Not a FIPS algorithm per se |

---

## ARM/AArch64 Driver Mapping Table

ARM Crypto Extension drivers follow the same priority model. The `*-ce` suffix denotes
Cryptography Extension (hardware-backed) drivers.

### AES (ARMv8 Crypto Extension)

| CPU Feature | `/proc/crypto` entry | driver | module | priority | type | Notes |
|-------------|---------------------|--------|--------|----------|------|-------|
| ARMv8 Crypto (aes) | `aes` | `aes-ce` | `aes_ce_cipher` | 200 | cipher | Crypto Extension hardware |
| ARMv8 Crypto (aes) | `cbc(aes)` | `cbc-aes-ce` | `aes_ce_blk` | 300 | skcipher | |
| ARMv8 Crypto (aes) | `ctr(aes)` | `ctr-aes-ce` | `aes_ce_blk` | 300 | skcipher | |
| ARMv8 Crypto (aes) | `xts(aes)` | `xts-aes-ce` | `aes_ce_blk` | 300 | skcipher | |
| ARMv8 Crypto (aes) | `gcm(aes)` | `gcm-aes-ce` | `ghash_ce` | 300 | aead | Requires pmull too |
| None | `aes` | `aes-generic` | `aes_generic` | 100 | cipher | Software fallback |

### SHA-2 (ARMv8 Crypto Extension)

| CPU Feature | `/proc/crypto` entry | driver | module | priority | type | Notes |
|-------------|---------------------|--------|--------|----------|------|-------|
| ARMv8 Crypto (sha2) | `sha256` | `sha256-ce` | `sha2_ce` | 200 | shash | Crypto Extension hardware |
| ARMv8 Crypto (sha2) | `sha224` | `sha224-ce` | `sha2_ce` | 200 | shash | |
| ARMv8 Crypto (sha512) | `sha512` | `sha512-ce` | `sha512_ce` | 200 | shash | ARMv8.2+ only |
| None | `sha256` | `sha256-generic` | `sha256_generic` | 100 | shash | Software fallback |

### GHASH / PMULL (ARMv8 Crypto Extension)

| CPU Feature | `/proc/crypto` entry | driver | module | priority | type | Notes |
|-------------|---------------------|--------|--------|----------|------|-------|
| ARMv8 Crypto (pmull) | `ghash` | `ghash-ce` | `ghash_ce` | 200 | shash | Polynomial multiplication |
| None | `ghash` | `ghash-generic` | `ghash_generic` | 100 | shash | Software fallback |

---

## Posture Check Reference

For each algorithm, the Layer 2 posture check verifies:

1. The hardware driver entry is present in `/proc/crypto`.
2. `selftest: passed` on the hardware driver entry.
3. `fips_allowed: yes` on the hardware driver entry (FIPS-mode systems only).
4. Hardware driver `priority` > software fallback driver `priority`.

| Algorithm | Hardware driver (x86) | Software fallback (x86) | Hardware driver (ARM) | Check priority exceeds |
|-----------|----------------------|------------------------|-----------------------|-----------------------|
| AES | `aesni_intel` (300) | `aes-generic` (100) | `aes-ce` (200) | 100 |
| AES-GCM | `generic-gcm-aesni` (400) | `gcm(aes-generic)` (100) | `gcm-aes-ce` (300) | 100 |
| SHA-256 | `sha256-ni` (250) | `sha256-generic` (100) | `sha256-ce` (200) | 100 |
| SHA-512 | `sha512-ni` (250) | `sha512-generic` (100) | `sha512-ce` (200) | 100 |
| GHASH | `ghash-clmulni-intel` (400) | `ghash-generic` (100) | `ghash-ce` (200) | 100 |

---

## Sources

- [Linux Kernel Driver DB: CONFIG_CRYPTO_AES_NI_INTEL](https://cateee.net/lkddb/web-lkddb/CRYPTO_AES_NI_INTEL.html)
- [Linux Kernel Driver DB: CONFIG_CRYPTO_SHA256_SSSE3](https://cateee.net/lkddb/web-lkddb/CRYPTO_SHA256_SSSE3.html)
- [Linux Kernel Driver DB: CONFIG_CRYPTO_CRCT10DIF_PCLMUL](https://cateee.net/lkddb/web-lkddb/CRYPTO_CRCT10DIF_PCLMUL.html)
- [Kernel Crypto API Architecture (docs.kernel.org)](https://docs.kernel.org/crypto/architecture.html)
- [Accelerated AES for Arm64 Linux Kernel (Linaro blog)](https://old.linaro.org/blog/accelerated-aes-for-the-arm64-linux-kernel/)
- [sha256_ssse3_glue.c — WSL2 Linux Kernel mirror](https://github.com/microsoft/WSL2-Linux-Kernel/blob/master/arch/x86/crypto/sha256_ssse3_glue.c)
- [aesni-intel_glue.c — Intel Linux LTS mirror](https://github.com/intel/linux-intel-lts/blob/master/arch/x86/crypto/aesni-intel_glue.c)
- [Cloudflare: The Linux Crypto API for user applications](https://blog.cloudflare.com/the-linux-crypto-api-for-user-applications/)
- Linux kernel source: `arch/x86/crypto/`, `arch/arm64/crypto/`
