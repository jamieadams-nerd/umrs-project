# NX/XD (No-Execute / Execute Disable)

## 23-Column Matrix Profile

| # | Column | Value |
|---|--------|-------|
| 1 | Feature name | NX/XD (No-Execute / Execute Disable) |
| 2 | Vendor | Both (Intel: XD, AMD: NX) |
| 3 | CPUID detection | EAX=80000001H, EDX bit 20 |
| 4 | Linux `/proc/cpuinfo` flag | `nx` |
| 5 | Key instructions | None (page-table attribute, not an instruction). Enabled by setting IA32_EFER.NXE (MSR bit 11). Page-table entry bit 63 = 1 marks page non-executable. |
| 6 | Introduced | AMD Athlon 64 (2003, first x86 NX implementation), Intel Pentium 4 Prescott (2004, as "XD"). Mandatory for all x86-64 processors (part of AMD64 spec). |
| 7 | Security relevance | **Foundational.** Hardware W^X enforcement. Without NX, the entire exploit mitigation stack collapses: ASLR, stack canaries, and other defenses are trivially bypassable when attackers can execute code on data pages (stack, heap). NX prevents code injection on any page not explicitly marked executable. |
| 8 | Performance benefit | Negligible — one additional PTE bit check per page walk. No measurable overhead. |
| 9 | Known vulnerabilities | No hardware CVEs against NX itself. The security value is entirely in what it *prevents*. Intel BIOS bug: ~10% of systems had MSR_IA32_MISC_ENABLE bit 34 set, silently disabling XD. Linux kernel commit ae84739 (2012) clears this bit unconditionally. |
| 10 | Compliance mapping | NIST SP 800-53 SI-16 (Memory Protection), SC-39 (Process Isolation); CMMC SC.L2-3.13.11; NSA RTB (Defense in Depth) |
| 11 | Classification | **Critical/Defensive** |
| 12 | Classification rationale | NX is the single most foundational hardware exploit mitigation. Every modern exploit mitigation chain (ASLR + NX + stack canaries + CFI) starts with NX. Without NX, code injection on data pages is trivial. Absence on x86-64 is essentially impossible (mandatory in AMD64 spec), but BIOS misconfiguration can disable it. |
| 13 | Linux kernel support | Kernel 2.6.8+ (merged June 2004 by Ingo Molnar). On x86-32, requires `CONFIG_X86_PAE=y`. On x86-64, NX is always available (long mode requires PAE). `CONFIG_X86_64` implies NX support. |
| 14 | Detection method (safe Rust) | Parse `/proc/cpuinfo` flags line for `nx`. On x86-64, NX is mandatory — absence indicates BIOS or virtualization misconfiguration. |
| 15 | Virtualization confidence | **LOW RISK** — NX is universally passed through by all hypervisors. Masking NX would break virtually all modern operating systems. KVM, VMware, and Hyper-V all expose NX to guests. |
| 16 | ARM/AArch64 equivalent | AArch64 has XN (Execute Never) and PXN (Privileged Execute Never) as page table attributes, always available. UXN (User Execute Never) also present. Not feature-flagged in `/proc/cpuinfo` because it is architecturally mandatory. |
| 17 | References | Intel SDM Vol 3A Section 4.6 (Access Rights); AMD APM Vol 2 Section 5.6; NX bit Wikipedia; Linux kernel commit ae84739; Red Hat KB 2936741 |
| 18 | Disposition when unused | N/A — NX cannot be "unused" in the traditional sense. It is a page-table attribute applied by the OS. However, BIOS disabling XD via MSR_IA32_MISC_ENABLE is a **CRITICAL finding** that must be remediated. |
| 19 | Software utilization detection | Check `/proc/cpuinfo` for `nx` flag. On x86-64, absence is anomalous. Also verify NX is not disabled via kernel boot parameter (no standard parameter disables NX on x86-64, but `noexec=off` existed for x86-32 PAE). |
| 20 | FIPS utilization requirement | N/A — NX is a platform integrity feature, not a cryptographic primitive. However, FIPS 140-3 physical security requirements imply a hardened platform, and NX is a baseline expectation. |
| 21 | Active mitigation status | N/A (not a speculative execution mitigation; foundational page-table attribute) |
| 22 | Feature accessible vs advertised | **BIOS gate exists (Intel only).** MSR_IA32_MISC_ENABLE bit 34 can disable XD at the BIOS level. Linux kernel has cleared this bit unconditionally since 2012 (commit ae84739), but bare-metal or non-Linux systems may still be affected. AMD processors do not have an equivalent disable mechanism. |
| 23 | Guest-vs-host discrepancy risk | **LOW** — NX is universally passed through. Only extremely old or misconfigured hypervisors would mask it. |

## Attack Class Blocked

**Code injection on data pages (stack/heap/BSS).** Without NX, an attacker who controls data on the stack or heap can place shellcode there and redirect execution to it. This is the classic buffer overflow exploit technique dating to the Morris Worm (1988) through modern exploitation.

NX transforms exploits from "write shellcode + jump to it" to requiring ROP/JOP chains, which are then addressed by CET, ASLR, and other mitigations. NX is the foundation of the modern defense-in-depth stack.

## Historical Context

| Year | Event |
|------|-------|
| 2003 | AMD ships first x86 NX in Athlon 64 (AMD64 spec mandates it) |
| 2004 | Intel adds XD to Pentium 4 Prescott |
| 2004 | Linux 2.6.8 merges NX support (Ingo Molnar) |
| 2004 | Windows XP SP2 adds DEP (Data Execution Prevention) using NX |
| 2012 | Linux kernel clears Intel XD-disable MSR bit unconditionally |

## Kernel Configuration

| Config | Purpose | x86-32 | x86-64 |
|--------|---------|--------|--------|
| `CONFIG_X86_PAE` | PAE paging (required for NX on 32-bit) | Required | N/A (always PAE in long mode) |
| `CONFIG_X86_64` | 64-bit mode (NX always available) | N/A | Always |

## Posture Check Specification

1. Verify `nx` flag in `/proc/cpuinfo` (Layer 1)
2. On x86-64, absence is a **CRITICAL anomaly** — investigate BIOS, hypervisor, or kernel misconfiguration
3. No Layer 2 "software utilization" check needed — NX is automatically used by the kernel for all non-executable pages
4. On x86-32, verify `CONFIG_X86_PAE=y` in kernel config

## Sources

- [NX bit — Wikipedia](https://en.wikipedia.org/wiki/NX_bit)
- [Red Hat: What is NX/XD feature?](https://access.redhat.com/solutions/2936741)
- [Ubuntu Security: CPU Features](https://wiki.ubuntu.com/Security/CPUFeatures)
- [Linux kernel NX support — LWN (2004)](https://lwn.net/Articles/87814/)
- [Linux kernel commit ae84739 — Clear XD_DISABLED on Intel](https://github.com/torvalds/linux/commit/ae84739c27b6b3725993202fe02ff35ab86468e1)
- Intel SDM Vol 3A Section 4.6
- AMD APM Vol 2 Section 5.6
