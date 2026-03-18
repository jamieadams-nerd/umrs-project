# AES-NI (Advanced Encryption Standard New Instructions)

## 23-Column Matrix Profile

| # | Column | Value |
|---|--------|-------|
| 1 | Feature name | AES-NI (AESNI) |
| 2 | Vendor | Both (Intel & AMD) |
| 3 | CPUID detection | EAX=01H, ECX bit 25 |
| 4 | Linux `/proc/cpuinfo` flag | `aes` |
| 5 | Key instructions | AESENC, AESENCLAST, AESDEC, AESDECLAST, AESKEYGENASSIST, AESIMC |
| 6 | Introduced | Intel Westmere (2010), AMD Bulldozer (2011) |
| 7 | Security relevance | Hardware AES eliminates cache-timing side channels present in software T-table implementations. Provides constant-time AES operations critical for FIPS-validated cryptographic paths. |
| 8 | Performance benefit | 5x–10x over software AES; ~1.4–2 GB/s vs ~258 MB/s on typical hardware |
| 9 | Known vulnerabilities | No hardware CVEs against AES-NI itself. Risk is in software fallback: T-table AES is timing-vulnerable (Bernstein 2005, Osvik-Shamir-Tromer 2006). |
| 10 | Compliance mapping | NIST SP 800-53 SC-13 (Cryptographic Protection), SC-12 (Key Management); FIPS 140-2/3 (validated primitive); CMMC SC.L2-3.13.11 |
| 11 | Classification | **Critical/Operational (FIPS)** |
| 12 | Classification rationale | FIPS mode on RHEL 10 requires validated cryptographic primitives. AES-NI is the preferred hardware acceleration path for OpenSSL FIPS provider. Absence forces software fallback with timing-channel risk. |
| 13 | Linux kernel support | `CONFIG_CRYPTO_AES_NI_INTEL` — kernel module `aesni_intel`. Supported since Linux 2.6.32+. |
| 14 | Detection method (safe Rust) | `std::is_x86_feature_detected!("aes")` (safe, no unsafe needed); or parse `/proc/cpuinfo` flags line for `aes` |
| 15 | Virtualization confidence | **HIGH RISK** — VMware, KVM, and VirtualBox can mask CPUID bit 25 silently. Guest cannot independently verify hardware-backed AES without attestation. `/proc/crypto` showing `aesni_intel` driver is best guest-side indicator. |
| 16 | ARM/AArch64 equivalent | ARMv8 Cryptography Extension (`aes` flag in `/proc/cpuinfo`); kernel module `aes-ce` |
| 17 | References | Intel AES-NI White Paper; Intel SDM Vol 2A Ch 3; AMD APM Vol 3 App E; Bernstein cache-timing paper (2005) |
| 18 | Disposition when unused | **INVESTIGATE** — If AES-NI is present but `/proc/crypto` shows `aes-generic` at higher priority than `aesni_intel`, the system is using timing-vulnerable software AES. This is a HIGH finding on FIPS systems. |
| 19 | Software utilization detection | `/proc/crypto`: look for driver `aesni_intel` (x86) or `aes-ce` (ARM) with priority > `aes-generic`. If `aes-generic` wins priority, hardware accel is not being used. |
| 20 | FIPS utilization requirement | OpenSSL FIPS provider on RHEL 10 uses AES-NI when available. `OPENSSL_ia32cap` environment variable can disable it (testing only). FIPS validation certificate covers the AES-NI code path. |
| 21 | Active mitigation status | N/A (crypto acceleration, not a mitigation) |
| 22 | Feature accessible vs advertised | Generally not BIOS-gated — AES-NI is enabled if the CPU supports it. However, `OPENSSL_ia32cap` can force software fallback at the application level. Hypervisors can mask the CPUID bit. |
| 23 | Guest-vs-host discrepancy risk | **HIGH** — VM migration between AES-NI and non-AES-NI hosts can silently degrade to software AES. No guest-visible notification of the change. |

## CVE / Vulnerability Table

| ID | Name | Year | Impact | Relevance to AES-NI |
|----|------|------|--------|---------------------|
| N/A | Bernstein cache-timing attack | 2005 | Software T-table AES leaks key material via cache timing | AES-NI eliminates this attack class entirely |
| N/A | Osvik-Shamir-Tromer | 2006 | Practical cache-timing key extraction against software AES | AES-NI is the fix |
| CVE-2016-0701 | OpenSSL key exchange | 2016 | Not directly AES-NI, but FIPS path related | Shows importance of validated code paths |

**Note:** AES-NI has no known hardware vulnerabilities. The security value is in what it *prevents* — cache-timing side channels in software fallback paths.

## `/proc/crypto` Driver Mapping

| Driver | Module | Type | Hardware-backed |
|--------|--------|------|-----------------|
| `aesni_intel` | `aesni_intel` | cipher | Yes — AES-NI |
| `aes-generic` | `aes_generic` | cipher | No — software T-table |
| `aes-ce` | `aes_ce` | cipher | Yes — ARM Crypto Extension |

**Posture check:** Verify `aesni_intel` (or `aes-ce` on ARM) appears in `/proc/crypto` with `selftest: passed` and priority higher than `aes-generic`.

## Software Fallback Risk

| Fallback | Constant-time? | FIPS-validated? | Risk |
|----------|---------------|-----------------|------|
| `aes-generic` (T-table) | **NO** — timing-vulnerable | Yes (but timing-weak) | HIGH on systems processing sensitive data |
| VPAES (vector permutation AES) | Yes — bitsliced | Yes | LOW — safe fallback but slower |
| BSAES (bit-sliced AES) | Yes | Yes | LOW — safe fallback but slower |

## Sources

- [Intel AES-NI White Paper](https://www.intel.com/content/dam/doc/white-paper/advanced-encryption-standard-new-instructions-set-paper.pdf)
- [Bernstein: Cache-timing attacks on AES (2005)](https://cr.yp.to/antiforgery/cachetiming-20050414.pdf)
- [Red Hat: AES timing attacks and OpenSSL](https://www.redhat.com/en/blog/its-all-question-time-aes-timing-attacks-openssl)
- [Linux CRYPTO_AES_NI_INTEL config](https://cateee.net/lkddb/web-lkddb/CRYPTO_AES_NI_INTEL.html)
- [Rust `is_x86_feature_detected!` macro](https://doc.rust-lang.org/std/macro.is_x86_feature_detected.html)
- [RHEL 10 FIPS mode documentation](https://docs.redhat.com/en/documentation/red_hat_enterprise_linux/10/html/security_hardening/switching-rhel-to-fips-mode)
