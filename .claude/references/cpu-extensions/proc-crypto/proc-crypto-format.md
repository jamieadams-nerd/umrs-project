# /proc/crypto Interface Reference

**Phase:** 1H
**Completed:** 2026-03-18
**Scope:** Entry structure, field semantics, priority selection, FIPS mode interaction

## Overview

`/proc/crypto` is a read-only procfs interface that lists every cipher algorithm currently
registered with the Linux kernel Crypto API. Each entry corresponds to one registered
algorithm implementation — when multiple implementations offer the same algorithm name
(e.g., `aes-generic`, `aes-x86_64`, `aesni_intel` all satisfy the name `aes`), all of them
appear as separate entries with different priorities.

The kernel uses priority to resolve which implementation is selected when a caller requests
an algorithm by its generic name. The implementation with the highest numeric priority wins.

**Key insight for posture checking:** `/proc/crypto` is a Layer 2 detection interface. Its
presence of a hardware-accelerated driver (e.g., `aesni_intel`) is the strongest available
guest-side signal that hardware acceleration is in active use by the kernel. It is the
preferred posture check for software utilization of crypto extensions.

---

## Entry Block Format

Each entry is a multi-line block separated by a blank line. Fields are colon-separated
key-value pairs. Not all fields appear in every entry — fields that are not applicable to a
cipher type are omitted.

```
name         : <generic algorithm name>
driver       : <unique driver name>
module       : <kernel module name, or "kernel" for built-ins>
priority     : <integer>
refcnt       : <integer>
selftest     : passed | failed | unknown
fips_allowed : yes | no          # only appears for certain entries
type         : <cipher type>
async        : no | yes
blocksize    : <bytes>
min keysize  : <bytes>           # symmetric ciphers
max keysize  : <bytes>           # symmetric ciphers
ivsize       : <bytes>           # block modes with IV
chunksize    : <bytes>           # stream ciphers
digestsize   : <bytes>           # hash/digest algorithms
seedsize     : <bytes>           # RNG algorithms
```

### Example Entries

**Hardware-accelerated AES (x86):**
```
name         : aes
driver       : aesni_intel
module       : aesni_intel
priority     : 300
refcnt       : 3
selftest     : passed
type         : cipher
blocksize    : 16
min keysize  : 16
max keysize  : 32
```

**Software AES fallback:**
```
name         : aes
driver       : aes-generic
module       : aes_generic
priority     : 100
refcnt       : 1
selftest     : passed
type         : cipher
blocksize    : 16
min keysize  : 16
max keysize  : 32
```

**AES-GCM compound (hardware-merged):**
```
name         : gcm(aes)
driver       : generic-gcm-aesni
module       : aesni_intel
priority     : 400
refcnt       : 2
selftest     : passed
fips_allowed : yes
type         : aead
blocksize    : 1
ivsize       : 12
maxauthsize  : 16
```

**SHA-256 with hardware extension:**
```
name         : sha256
driver       : sha256-ni
module       : sha256_ni
priority     : 300
refcnt       : 4
selftest     : passed
type         : shash
blocksize    : 64
digestsize   : 32
```

**GHASH with CLMUL hardware:**
```
name         : ghash
driver       : ghash-clmulni-intel
module       : ghash_clmulni_intel
priority     : 400
refcnt       : 3
selftest     : passed
fips_allowed : yes
type         : shash
blocksize    : 16
digestsize   : 16
```

---

## Field Descriptions

### `name`

The generic name of the algorithm. Multiple entries share the same `name` value when multiple
implementations register for the same algorithm. Callers who allocate a cipher by generic name
receive the highest-priority registered implementation.

The `name` field also encodes template composition using parenthetical notation:
- `aes` — single block cipher
- `cbc(aes)` — AES cipher wrapped in CBC chaining template
- `gcm(aes)` — AES in Galois/Counter Mode (AEAD)
- `hmac(sha256)` — HMAC keyed digest built on SHA-256
- `rfc4106(gcm(aes))` — GCM-AES wrapped in RFC 4106 for IPsec

### `driver`

The unique name identifying a specific implementation. Unlike `name`, `driver` names are
distinct per implementation. Callers who allocate by driver name bypass priority selection
and always get the named implementation.

Driver name conventions:
- Hardware-specific names embed the instruction set or hardware: `aesni_intel`,
  `sha256-ni`, `ghash-clmulni-intel`, `aes-ce` (ARM), `sha256-ce` (ARM)
- Software fallbacks use the `-generic` suffix: `aes-generic`, `sha256-generic`,
  `ghash-generic`
- x86 assembler (SSSE3/AVX, not AES-NI) uses architecture-specific names:
  `aes-x86_64`, `sha256_ssse3`, `sha256-avx2`

### `module`

The kernel module that provides this implementation. The special value `kernel` indicates
the implementation is statically compiled into the kernel. Module-provided implementations
can be unloaded; kernel-provided implementations cannot.

For posture checking: if a module-provided hardware accelerator is not loaded (e.g.,
`aesni_intel` module not loaded), the entry will not appear in `/proc/crypto` at all —
meaning the software fallback will take over silently. The absence of a hardware driver
entry is itself a posture finding.

### `priority`

An integer assigned by each implementation at registration time. Higher values win when
multiple implementations share the same `name`. There is no fixed scale, but observed values
cluster as follows:

| Priority range | Typical source |
|----------------|---------------|
| 0–100 | Generic software implementations (C code) |
| 100–200 | Assembler-optimized software (SSSE3, AVX2) |
| 200–300 | Hardware instruction implementations (AES-NI, SHA-NI) |
| 300–500 | Hardware-merged compound modes (AES-GCM-NI) |

**Key rule:** A posture check must confirm that the hardware driver entry has a higher
priority than any software fallback entry with the same `name`. If the hardware driver is
registered but its priority is lower than a software implementation, the hardware is present
but not in active use by default callers.

In practice, hardware implementations are always registered with priority > software, so
finding a hardware entry present in `/proc/crypto` is strong evidence it is the default
selection. Explicit priority comparison adds defense-in-depth.

### `refcnt`

Reference count: the number of active users of this cipher instance. A non-zero refcnt
indicates the implementation is actively being used. For posture analysis, refcnt is
informational — it can be zero even for correctly configured hardware acceleration, because
refcnt reflects current users, not historical or configured users.

### `selftest`

Indicates whether the kernel's built-in known-answer test (KAT) for this implementation
passed at registration time. Values:

| Value | Meaning |
|-------|---------|
| `passed` | Self-test ran and passed |
| `failed` | Self-test ran and failed (implementation disabled in FIPS mode) |
| `unknown` | Implementation does not implement self-test reporting |

**FIPS significance:** When the kernel is booted with `fips=1`, algorithms with
`selftest: failed` cannot be used. In FIPS mode, the kernel refuses to register algorithm
implementations that fail their KAT. A `selftest: passed` entry is therefore a prerequisite
for that algorithm to be usable on a FIPS-mode system.

For posture checking: always verify `selftest: passed` on the hardware driver entry. A
hardware driver that fails self-test is not providing its acceleration — the kernel will fall
back to the next-highest-priority implementation.

### `fips_allowed`

A field that appears on certain algorithm entries to indicate that the implementation is
approved for use in FIPS mode. The presence and value of this field depends on the kernel
version and how the algorithm was registered.

| Value | Meaning |
|-------|---------|
| `yes` | Algorithm is FIPS-approved and usable when `fips=1` |
| `no` or absent | Algorithm is not FIPS-approved; blocked in FIPS mode |

**Relationship to CRYPTO_ALG_FIPS_INTERNAL:** The `CRYPTO_ALG_FIPS_INTERNAL` flag
(value `0x00020000`) marks algorithms that are not themselves FIPS-approved but are used
internally to implement FIPS-approved algorithms (e.g., a raw DH implementation used inside
an approved FFDHE key agreement scheme). This is distinct from `fips_allowed`.

**Historical note:** Early kernels did not mark accelerated drivers as fips_allowed by
default. For example, `aesni_intel` and `ghash-clmulni-intel` required explicit patches to
gain FIPS approval marks. On modern kernels (5.x+) used in RHEL 9/10, these hardware
drivers are marked fips_allowed.

**Posture check rule:** On FIPS-mode systems, verify that the hardware driver entry shows
`fips_allowed: yes`. If the field is absent or shows `no`, the hardware driver cannot be
used in FIPS mode and the system falls back to a software implementation.

### `type`

The cipher type classification:

| Type | Description |
|------|-------------|
| `cipher` | Single block cipher (no chaining mode); used as building block by templates |
| `skcipher` | Symmetric key cipher (includes chaining mode) |
| `aead` | Authenticated Encryption with Associated Data |
| `shash` | Synchronous hash/message digest |
| `ahash` | Asynchronous hash/message digest |
| `rng` | Random number generator |
| `kpp` | Key-agreement Protocol Primitive (ECDH, DH) |
| `akcipher` | Asymmetric cipher (RSA) |

For crypto extension posture checks, the relevant types are `cipher`, `skcipher`, `aead`,
and `shash`. RNG types (`rng`) are relevant for RDRAND/RDSEED posture checks (Phase 1B).

---

## Priority Selection Mechanism

When the kernel crypto API receives a request for an algorithm by generic name:

1. The API searches all registered implementations with that `name`.
2. It filters to implementations matching the requested type (if specified).
3. It selects the implementation with the **highest `priority` value**.
4. If the selected implementation's `selftest: failed` (on FIPS-mode kernels), it is
   skipped and the next-highest priority is tried.

This means:
- If `aesni_intel` (priority 300) and `aes-generic` (priority 100) are both registered,
  all generic-name requests for `aes` receive `aesni_intel`.
- If `aesni_intel` fails its self-test in FIPS mode, the kernel falls back to `aes-generic`
  (which has its own self-test status).

**Implications for posture checking:**

The presence of a hardware entry in `/proc/crypto` with higher priority than any software
entry, combined with `selftest: passed`, provides strong assurance that the hardware path is
the kernel's default selection. This is the Layer 2 hardware utilization signal for crypto
extensions.

---

## Reading /proc/crypto Programmatically

The file can be parsed as a sequence of blank-line-delimited blocks. Each block is a set of
`key: value` pairs. A robust parser must:

1. Split on double-newline to separate entries.
2. Within each entry, split on `\n` to get lines; split each line on `: ` (colon-space) to
   get key and value.
3. Trim whitespace from both key and value.
4. Handle missing fields gracefully — not all entries have all fields.

**Rust parsing approach:** Read the file via `ProcfsText` (KATTRS pattern for provenance
verification), then split and parse. Expected path: `/proc/crypto`.

**RAIN principle:** The path `/proc/crypto` must be confirmed as a procfs mount before
trusting its contents. The kernel ensures this is a legitimate procfs entry, but a
posture-checking tool should verify the mount type (statfs magic = `PROC_SUPER_MAGIC`
= `0x9fa0`) before reading.

---

## FIPS Mode Interaction Summary

| Kernel boot | Algorithm state | Posture implication |
|-------------|----------------|---------------------|
| No `fips=1` | All registered implementations available | Hardware entries present means hardware is default |
| `fips=1` | Failed-selftest entries blocked; non-FIPS algorithms excluded | `selftest: passed` + `fips_allowed: yes` required |
| `fips=1`, HW present | `aesni_intel` (passed, fips_allowed) selected over `aes-generic` | Normal FIPS mode; hardware acceleration active |
| `fips=1`, HW absent | `aes-generic` (passed, fips_allowed) selected | Hardware acceleration unavailable; software fallback active |
| `fips=1`, HW self-test fails | `aesni_intel` blocked; `aes-generic` selected | Hardware malfunctioning; posture finding |

---

## Sources

- [Kernel Crypto API Architecture (kernel.org)](https://www.kernel.org/doc/html/latest/crypto/architecture.html)
- [proc_crypto(5) man page (man7.org)](https://man7.org/linux/man-pages/man5/proc_crypto.5.html)
- [Kernel crypto/proc.c — proc_crypto implementation (github.com/torvalds/linux)](https://github.com/torvalds/linux/blob/master/crypto/proc.c)
- [linux/include/linux/crypto.h — CRYPTO_ALG_FIPS_INTERNAL](https://github.com/torvalds/linux/blob/master/include/linux/crypto.h)
- [Patch: allow aesni-intel and ghash_clmulni-intel in FIPS mode (linux-crypto ML)](https://linux-crypto.vger.kernel.narkive.com/oU2eLCdP/patch-crypto-allow-aesni-intel-and-ghash-clmulni-intel-in-fips-mode)
- [The Linux Crypto API for user applications (Cloudflare blog)](https://blog.cloudflare.com/the-linux-crypto-api-for-user-applications/)
- Linux kernel `Documentation/crypto/architecture.rst` (in-repo: `.claude/references/kernel-docs/crypto/architecture.rst`)
