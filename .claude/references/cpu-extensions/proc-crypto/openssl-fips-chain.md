# OpenSSL FIPS Dependency Chain on RHEL 10

**Phase:** 1H
**Completed:** 2026-03-18
**Scope:** OpenSSL FIPS provider architecture on RHEL 10, CMVP validation status, hardware
acceleration path selection, OPENSSL_ia32cap control, and connection to UMRS IndicatorId::ProcFips.

---

## Overview

On RHEL 10, OpenSSL 3 uses a provider-based architecture where the FIPS provider (`fips.so`)
is a separate loadable module validated by NIST's Cryptographic Module Validation Program
(CMVP). The FIPS provider contains only FIPS-approved algorithm implementations. When FIPS
mode is active, OpenSSL routes all cryptographic operations through this validated provider.

**Key chain:** `fips=1` (kernel boot) → `/proc/sys/crypto/fips_enabled = 1` → OpenSSL detects
FIPS mode → loads `fips.so` provider → hardware acceleration selected by runtime CPU capability
detection → `/proc/crypto` hardware drivers active.

---

## FIPS Activation Path

### 1. Kernel Boot Parameter

FIPS mode is activated at kernel boot via the `fips=1` command-line parameter. This is
reflected in the kernel's crypto subsystem:

```
/proc/sys/crypto/fips_enabled
```

Value `1` indicates FIPS mode is active. Value `0` means non-FIPS operation.

**RHEL 10 restriction:** The only supported method to activate FIPS mode on RHEL 10 is to
enable it during installation. Enabling FIPS mode post-installation is not supported in RHEL
10. This is a significant departure from RHEL 9, where `fips-mode-setup` could be run on
an existing installation.

**UMRS connection:** `IndicatorId::ProcFips` reads `/proc/sys/crypto/fips_enabled`. This is the
authoritative Layer 1 signal for kernel-level FIPS enforcement. The value here gates whether
OpenSSL uses the FIPS provider.

### 2. OpenSSL FIPS Provider Loading

When OpenSSL 3 initializes on a FIPS-mode system, it reads `/proc/sys/crypto/fips_enabled`.
If the value is `1`, OpenSSL automatically activates the FIPS provider from `fips.so`.

**RHEL 10 package:** `openssl-fips-provider` RPM. Contains the validated binary module
`fips.so` whose hash is verified at load time against the embedded HMAC.

**RHEL 10 module version and status as of 2026-03-18:**
- Module: `openssl-fips-provider-3.0.7-6.el10`
- Status: **Pending Operational Environment update** (validation in progress)
- The underlying binary is the same CMVP-validated binary from the RHEL 9.2 certification

### 3. CMVP Certificate Status

| Platform | Certificate # | Algorithm | Status |
|----------|--------------|-----------|--------|
| RHEL 9.0 OpenSSL FIPS Provider | #4746 | FIPS 140-3 | Active |
| RHEL 9.2, 9.4, 9.6 OpenSSL FIPS Provider | #4857 | FIPS 140-3 | Active |
| RHEL 9.4 Kernel Cryptographic API | #4796 | FIPS 140-3 | Active |
| RHEL 10.0 OpenSSL FIPS Provider | Pending | FIPS 140-3 | **Under review** |

**Key fact:** Red Hat reuses the same CMVP-validated binary (`fips.so`) across RHEL 9
and early RHEL 10 releases. The RHEL 10 pending status is an Operational Environment (OE)
update to the existing certificate, not a full re-validation of the cryptographic code.

**Posture implication for UMRS:** Until RHEL 10's OE update is finalized, systems running
RHEL 10 with `fips_enabled=1` use a provisionally validated provider. The binary is the same
as the RHEL 9 certificate holder. This is a compliance nuance worth flagging in audit reports.

---

## Hardware Path Selection Within the FIPS Provider

The OpenSSL FIPS provider selects hardware acceleration at runtime using x86 (and ARM)
CPU capability detection. This happens independently from `/proc/crypto` — OpenSSL has its
own capability detection layer.

### OPENSSL_ia32cap

`OPENSSL_ia32cap` is an environment variable that controls which x86 CPU capabilities
OpenSSL reports as available at runtime. It overrides CPUID results, allowing specific
hardware features to be disabled for testing or debugging.

**Format:**
```
OPENSSL_ia32cap=~<mask>
```

The tilde (`~`) prefix masks (disables) the specified bits. There is no syntax to add
capability bits that the CPU does not actually support.

**Capability vector layout (two 64-bit words, colon-separated):**

Word 1 (bits 0–63): reflects CPUID EAX=1 ECX and EDX leaves:
| Bit | Capability | Notes |
|-----|-----------|-------|
| 1 | SSE2 | |
| 9 | SSSE3 | Required for VPAES |
| 20 | SSE4.2 | |
| 25 | AES-NI | AESNI instructions |
| 33 | PCLMULQDQ | Carry-less multiplication |

Word 2 (bits 64–127): reflects extended CPUID leaves (EAX=7 EBX/ECX):
| Bit (in word 2) | Capability | Notes |
|----------------|-----------|-------|
| 5 (global bit 69) | AVX2 | |
| 29 (global bit 93) | SHA-NI | Intel SHA Extensions |

**Disabling AES-NI example:**
```bash
OPENSSL_ia32cap=~0x200000200000000 openssl speed aes-256-cbc
```
This disables AES-NI (bit 25) and PCLMULQDQ (bit 33) in word 1.

**FIPS validation scope note:** The FIPS provider is validated for both the hardware (AES-NI)
and software paths, provided both were tested during CAVP evaluation. Disabling AES-NI via
`OPENSSL_ia32cap` in a production FIPS system is not recommended — the software path (T-table
AES or VPAES) may fall outside the tested boundary for that OE.

### ARM Capability Detection

On AArch64, OpenSSL uses the AT_HWCAP and AT_HWCAP2 ELF auxiliary vectors to detect
ARMv8 Crypto Extension features. No equivalent of `OPENSSL_ia32cap` exists for ARM; the
equivalent is the `OPENSSL_armcap` environment variable, which uses the same masking syntax.

---

## Hardware vs Software Path Decision Tree

```
OpenSSL function call (e.g., EVP_aes_256_cbc)
  │
  ├─ FIPS provider active? (fips_enabled=1)
  │    ├─ Yes → use fips.so validated code path
  │    │         │
  │    │         └─ AES-NI available? (checked via OPENSSL_ia32cap / CPUID)
  │    │               ├─ Yes → AES-NI instructions (constant-time, hardware)
  │    │               ├─ SSSE3 available? → VPAES (constant-time, software)
  │    │               └─ No hardware → integer-only software AES (not T-table)
  │    │
  │    └─ No → use default provider
  │              └─ (same capability detection applies)
  │
  └─ Algorithm implemented in kernel (e.g., dm-crypt, IPsec)?
       └─ Uses Linux Crypto API → driver selected by /proc/crypto priority
```

**Key distinction:** OpenSSL uses its own capability detection separate from the Linux
Crypto API. Even if `aesni_intel` appears in `/proc/crypto`, OpenSSL might still fall
back to software if `OPENSSL_ia32cap` masks AES-NI. Both layers need posture verification.

---

## Connection to UMRS Signals

| UMRS Signal | Interface | What it verifies |
|-------------|-----------|-----------------|
| `IndicatorId::ProcFips` | `/proc/sys/crypto/fips_enabled` | Kernel-level FIPS enforcement active |
| Layer 2 (proposed) | `/proc/crypto` driver list | Kernel crypto using hardware acceleration |
| Layer 2 (proposed) | `openssl speed -evp aes-256-gcm` output | OpenSSL using hardware path (indirect) |

**Proposed Layer 2 signal for FIPS chain:** A signal that reads `/proc/crypto` and verifies
that `aesni_intel` (or `aes-ce` on ARM) is present with `selftest: passed` and
`fips_allowed: yes` when `ProcFips = 1`. This creates a compound posture check:
- Layer 1: kernel says FIPS is on → `ProcFips`
- Layer 2: kernel crypto is using the hardware AES path → `CryptoDriverActive::AesNi`

If Layer 1 passes but Layer 2 fails (hardware AES absent in FIPS mode), the system is
operating with software AES under FIPS certification — a compliance gap worth surfacing.

---

## Kernel Crypto API FIPS Mode Behavior

Separately from OpenSSL, the Linux Kernel Cryptographic API itself enforces FIPS constraints
when `fips_enabled=1`. In FIPS mode:

1. Algorithm implementations that fail self-test are not registered (or deregistered if
   already loaded) — they will not appear in `/proc/crypto` with `selftest: passed`.
2. Algorithms not marked `fips_allowed` are unavailable to kernel callers (dm-crypt, IPsec,
   TLS offload) in FIPS mode.
3. Non-FIPS algorithms (e.g., MD5, RC4) remain technically registered but attempts to
   allocate them via the generic API fail with `ENOENT` or `EPERM` depending on kernel version.

The RHEL 9.4 Kernel Cryptographic API (CMVP #4796) covers the kernel's FIPS-mode crypto
path. A RHEL 10 kernel crypto certificate is expected but not yet issued as of 2026-03-18.

---

## Diagnostic Commands

```bash
# Check kernel FIPS mode
cat /proc/sys/crypto/fips_enabled

# List all registered crypto drivers with FIPS status
grep -A 8 "^name" /proc/crypto | grep -E "name|driver|priority|selftest|fips_allowed"

# Verify OpenSSL is using FIPS provider
openssl list -providers 2>/dev/null | grep -A 3 fips

# Check which AES implementation OpenSSL is using (look for AES-NI rate vs software rate)
openssl speed -evp aes-256-gcm 2>/dev/null | tail -5

# Verify RHEL FIPS package
rpm -q openssl-fips-provider

# Disable AES-NI in OpenSSL for testing (NOT for production)
OPENSSL_ia32cap=~0x200000200000000 openssl speed -evp aes-256-gcm
```

---

## Sources

- [Red Hat FIPS compliance page](https://access.redhat.com/compliance/fips)
- [Red Hat: Common Criteria and FIPS certificates (blog)](https://www.redhat.com/en/blog/red-hat-enterprise-linux-common-criteria-and-fips-certificates)
- [RHEL 9 OpenSSL FIPS Provider Security Policy (CMVP #4746)](https://csrc.nist.gov/CSRC/media/projects/cryptographic-module-validation-program/documents/security-policies/140sp4746.pdf)
- [NIST CMVP Certificate #4857 (RHEL 9.2/9.4/9.6)](https://csrc.nist.gov/projects/cryptographic-module-validation-program/certificate/4857)
- [NIST CMVP Certificate #4985](https://csrc.nist.gov/projects/cryptographic-module-validation-program/certificate/4985)
- [RHEL 10 Security Hardening: Switching to FIPS mode (docs.redhat.com)](https://docs.redhat.com/en/documentation/red_hat_enterprise_linux/10/html/security_hardening/switching-rhel-to-fips-mode)
- [RHEL core cryptographic components (access.redhat.com)](https://access.redhat.com/articles/3655361)
- [Handling FIPS mode in upstream projects for RHEL (Red Hat Developer)](https://developers.redhat.com/articles/2024/02/27/handling-fips-mode-upstream-projects-rhel)
- [OPENSSL_ia32cap documentation (docs.openssl.org)](https://docs.openssl.org/master/man3/OPENSSL_ia32cap)
- [Force OpenSSL AES-NI usage (romanrm.net)](https://romanrm.net/force-enable-openssl-aes-ni-usage)
- [Red Hat blog: AES timing attacks on OpenSSL](https://www.redhat.com/en/blog/its-all-question-time-aes-timing-attacks-openssl)
