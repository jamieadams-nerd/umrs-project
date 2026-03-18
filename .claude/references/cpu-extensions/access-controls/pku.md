# PKU (Protection Keys for User-space)

## 23-Column Matrix Profile

| # | Column | Value |
|---|--------|-------|
| 1 | Feature name | PKU (Protection Keys for User-space) / MPK (Memory Protection Keys) |
| 2 | Vendor | Intel (Skylake server / Xeon Scalable, 2017), AMD (Zen 3+, 2020) |
| 3 | Category | 11 — CPU-Enforced Access Controls |
| 4 | Purpose | Hardware memory domain isolation without page table changes. Each page can be assigned one of 16 protection key domains (4-bit key in PTE bits 62:59). The PKRU register holds per-domain access/write-disable bits. Changing domain permissions requires only a WRPKRU instruction (no syscall, no TLB flush), making domain transitions extremely fast. Use cases: protecting crypto keys in memory, isolating sensitive data structures, sandboxing within a single address space. |
| 5 | Key instructions | WRPKRU (Write Protection Key Rights for User pages) — sets access/write-disable bits per domain. RDPKRU (Read Protection Key Rights) — reads current PKRU state. Both execute in user mode at ring 3 with ~20 cycle latency. No syscall required for domain transitions. |
| 6 | CPUID detection | PKU: EAX=07H, ECX=0H, ECX bit 3. OSPKE (OS has enabled PKU via CR4.PKE): EAX=07H, ECX=0H, ECX bit 4. Both must be present for PKU to be active. |
| 7 | Linux `/proc/cpuinfo` flag | `pku`, `ospke` |
| 8 | Linux detection — authoritative path | `/proc/cpuinfo` flags: `pku` and `ospke`. Kernel enables via CR4.PKE when CPUID indicates support. `pkey_alloc()` / `pkey_mprotect()` / `pkey_free()` syscalls for application use. |
| 9 | Minimum CPU generations | Intel Skylake-SP / Xeon Scalable (2017). AMD Zen 3 (2020). Not available on client Skylake (different silicon). |
| 10 | Security benefit | Enables intra-process isolation without page table overhead. Applications can isolate sensitive data (cryptographic keys, passwords, authentication tokens) into a protected domain and restrict access to specific code paths. This limits the blast radius of memory corruption vulnerabilities — a buffer overflow in one domain cannot read crypto keys in another without first executing WRPKRU to change permissions. |
| 11 | Performance benefit | Domain transitions via WRPKRU are ~20 cycles (vs ~1000+ cycles for mprotect syscall). No TLB flush required. Enables fine-grained memory protection that would be prohibitively expensive with page-table-based approaches. |
| 12 | Assurance caveats | **WRPKRU is unprivileged:** Any code in the process can execute WRPKRU to change domain permissions. PKU does not provide protection against an attacker with arbitrary code execution — if the attacker can run WRPKRU, they can unlock any domain. PKU is defense-in-depth against data-only attacks and memory corruption, not a hard isolation boundary. **16 domain limit:** Only 16 protection key domains available. Complex applications may exhaust domains. **Signal handlers:** Linux delivers signals with PKRU reset to default (all domains accessible), creating a window where protected data is accessible during signal handling. |
| 13 | Virtualization behavior | KVM: PKU passed through when available. PKRU is part of the XSAVE state, saved/restored on VM entry/exit. VMware: supported. Guest PKU is fully functional with hardware backing. |
| 14 | Firmware / BIOS / microcode dependency | `{ bios_enable_required: false, microcode_required: false }` — controlled via CR4.PKE. |
| 15 | Audit-card relevance | **Important** |
| 16 | Recommended disposition when unused | PKU absent = acceptable on pre-2017 hardware. PKU present but unused by applications = **INFORMATIONAL** — most applications do not yet use PKU. PKU would be particularly valuable for applications handling cryptographic material. |
| 17 | Software utilization detection method | Layer 1: `pku` and `ospke` in `/proc/cpuinfo`. Layer 2: application must use `pkey_alloc()` / `pkey_mprotect()` syscalls. `strace` for pkey syscalls. `/proc/<pid>/smaps` shows `ProtectionKey:` field per VMA when PKU is in use. |
| 18 | FIPS utilization requirement | N/A — memory protection, not a cryptographic primitive. However, PKU could protect FIPS module key material in memory. |
| 19 | Active mitigation status path | No sysfs vulnerability entry — proactive hardening feature. |
| 20 | Feature accessible vs advertised | No BIOS gate. CPUID is authoritative. `ospke` flag confirms OS has enabled CR4.PKE. |
| 21 | Guest-vs-host discrepancy risk | Low — PKU is straightforward XSAVE state passthrough. |
| 22 | Notes | PKU is an underutilized security feature. Most applications do not use it yet because it requires explicit adoption (pkey_alloc + pkey_mprotect). UMRS could potentially use PKU to protect cryptographic key material in memory — e.g., signing keys used by the log sealing pipeline. The 16-domain limit and WRPKRU accessibility limit its use as a hard security boundary, but it provides meaningful defense-in-depth. |
| 23 | Sources | Intel SDM Vol 3A Section 4.6.2 (Protection Keys); AMD APM Vol 2; Linux kernel pkeys documentation; LWN: Memory Protection Keys (2015) |

## Memory Domain Model

```
Domain 0: Default (all existing memory)
Domain 1: Crypto key material (restricted read/write)
Domain 2: Audit log buffers (write-only from logging code)
Domain 3: User input buffers (isolated from crypto)
...
Domain 15: Available
```

Each domain is controlled by 2 bits in PKRU:
- AD (Access Disable): prevents all access (read/write) to pages in this domain
- WD (Write Disable): prevents writes but allows reads

## Syscall Interface

| Syscall | Purpose |
|---------|---------|
| `pkey_alloc(flags, access_rights)` | Allocate a protection key domain |
| `pkey_mprotect(addr, len, prot, pkey)` | Assign memory region to a domain |
| `pkey_free(pkey)` | Release a protection key domain |

Application code uses WRPKRU directly (inline assembly or compiler intrinsic) for fast domain transitions.

## CVE / Vulnerability Table

| ID | Name | Year | Impact | PKU Relevance |
|----|------|------|--------|--------------|
| N/A | WRPKRU bypass | Theoretical | Attacker with code exec can unlock domains | PKU is not a hard boundary — defense-in-depth only |
| N/A | Signal handler PKRU reset | Design | Protected data accessible during signal handling | Linux resets PKRU on signal delivery |
| CVE-2022-2153 | KVM PKRU state leak | 2022 | Guest PKRU state leaked across VM boundary | Fixed in kernel; PKRU save/restore on VM exit |

## Kernel Build Dependencies

| Config Option | Feature | Default (RHEL 10) | Since |
|---------------|---------|-------------------|-------|
| `CONFIG_X86_INTEL_MEMORY_PROTECTION_KEYS` | PKU support | `=y` | Linux 4.6 (2016) |
| `CONFIG_ARCH_HAS_PKEYS` | Architecture has pkeys | `=y` | Linux 4.6 |

## Compliance Mapping

- **NIST SP 800-53 SC-4** (Information in Shared Resources) — isolating sensitive data within process
- **NIST SP 800-53 SC-39** (Process Isolation) — intra-process domain isolation
- **NIST SP 800-53 SC-12** (Cryptographic Key Establishment and Management) — protecting keys in memory

## Sources

- Intel SDM Vol 3A Section 4.6.2 (Protection Keys)
- AMD APM Vol 2
- [LWN: Memory Protection Keys (2015)](https://lwn.net/Articles/643797/)
- [Linux kernel pkeys documentation](https://docs.kernel.org/core-api/protection-keys.html)
- [pkey_alloc(2) man page](https://man7.org/linux/man-pages/man2/pkey_alloc.2.html)
