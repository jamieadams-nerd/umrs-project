# UMIP (User Mode Instruction Prevention)

## 23-Column Matrix Profile

| # | Column | Value |
|---|--------|-------|
| 1 | Feature name | UMIP (User Mode Instruction Prevention) |
| 2 | Vendor | Both (Intel Coffee Lake+, AMD Zen+) |
| 3 | Category | 11 — CPU-Enforced Access Controls |
| 4 | Purpose | Prevents user-mode execution of SGDT, SIDT, SLDT, SMSW, and STR instructions. These instructions return addresses of kernel descriptor tables (GDT, IDT, LDT) and task state — information that facilitates KASLR bypass, kernel exploit development, and privilege escalation. Without UMIP, any unprivileged process can read these addresses. |
| 5 | Key instructions | Affected instructions: SGDT (Store Global Descriptor Table), SIDT (Store Interrupt Descriptor Table), SLDT (Store Local Descriptor Table Register), SMSW (Store Machine Status Word), STR (Store Task Register). With UMIP active, executing these from ring 3 raises #GP (General Protection fault). |
| 6 | CPUID detection | EAX=07H, ECX=0H, ECX bit 2. |
| 7 | Linux `/proc/cpuinfo` flag | `umip` |
| 8 | Linux detection — authoritative path | `/proc/cpuinfo` flags: `umip`. Kernel enables automatically via CR4.UMIP when CPUID indicates support. On CPUs without UMIP, kernel uses software emulation (fixup_umip_exception) that returns dummy/zero values instead of real addresses. |
| 9 | Minimum CPU generations | Intel Coffee Lake (8th gen, 2017). AMD Zen+ (2018, Ryzen 2000). |
| 10 | Security benefit | Eliminates KASLR information leaks from descriptor table addresses. SGDT/SIDT return addresses that reveal the kernel's virtual address layout. Attackers use this to defeat KASLR and construct reliable kernel exploits. UMIP blocks this information channel at the hardware level. |
| 11 | Performance benefit | None — this is purely a security feature. The instructions it blocks are rarely used legitimately in user-mode code. |
| 12 | Assurance caveats | **Software emulation fallback:** On CPUs without UMIP, Linux provides software emulation that intercepts #GP from SGDT/SIDT in user mode and returns dummy values (zeros). This provides equivalent protection but through a software path — slightly less assured than hardware enforcement. **Wine/DOSEMU:** Some legacy applications (Wine, DOSEMU) execute SGDT/SIDT legitimately. The kernel software emulation handles this by returning safe dummy values. Hardware UMIP raises #GP which the kernel fixup handler catches. |
| 13 | Virtualization behavior | KVM: UMIP passed through to guests when available. VMware: supported. UMIP enforcement is per-CR4 bit, so hypervisor controls guest UMIP independently. |
| 14 | Firmware / BIOS / microcode dependency | `{ bios_enable_required: false, microcode_required: false }` — controlled via CR4 bit, no firmware gate. |
| 15 | Audit-card relevance | **Important** |
| 16 | Recommended disposition when unused | If `umip` absent from `/proc/cpuinfo` on post-2018 hardware, verify CPU generation. If present but disabled (kernel would need explicit intervention — unlikely), flag as finding. Software emulation fallback provides equivalent protection on older CPUs. |
| 17 | Software utilization detection method | Layer 1: `umip` in `/proc/cpuinfo`. Layer 2: kernel enables automatically — no additional software requirement. Verify kernel is not built with UMIP disabled (extremely unusual). |
| 18 | FIPS utilization requirement | N/A |
| 19 | Active mitigation status path | No sysfs vulnerability entry — proactive hardening. |
| 20 | Feature accessible vs advertised | No BIOS gate. CPUID bit is authoritative. |
| 21 | Guest-vs-host discrepancy risk | Low — UMIP is a simple CR4 bit passthrough. |
| 22 | Notes | UMIP is complementary to KASLR. KASLR randomizes kernel addresses; UMIP prevents leaking them through descriptor table instructions. Together they make kernel address prediction significantly harder. On modern RHEL 10 target hardware (Zen+ or Coffee Lake+), UMIP should always be present. |
| 23 | Sources | Intel SDM Vol 3A Section 2.5 (CR4.UMIP); AMD APM Vol 2; Linux kernel fixup_umip_exception in arch/x86/kernel/umip.c; LWN: User Mode Instruction Prevention (2017) |

## Attack Pattern Blocked: KASLR Bypass via Descriptor Tables

**Without UMIP:** User-mode process executes `SGDT` or `SIDT`, receiving a pointer into kernel virtual address space. This reveals the kernel's base address, defeating KASLR. Attacker can then reliably target kernel structures for exploitation.

**With UMIP:** `SGDT`/`SIDT` in user mode raises #GP. On CPUs without UMIP, kernel software emulation returns dummy values. Either way, the real kernel addresses are not disclosed.

## CVE / Vulnerability Table

| ID | Name | Year | Impact | UMIP Relevance |
|----|------|------|--------|---------------|
| CVE-2017-16995 | Linux eBPF verifier | 2017 | KASLR bypass + privesc | SGDT/SIDT leak used in exploit chain |
| CVE-2019-14821 | KVM MMIO coalesced write | 2019 | Guest-to-host escape | Descriptor table addresses aid exploit reliability |
| N/A | Generic KASLR bypass | Ongoing | Kernel address leak enables reliable exploitation | UMIP closes the descriptor table vector |

## Kernel Build Dependencies

| Config Option | Feature | Default (RHEL 10) | Since |
|---------------|---------|-------------------|-------|
| `CONFIG_X86_UMIP` | UMIP hardware support + software emulation | `=y` | Linux 4.15 (hardware), 4.7 (software emulation) |

## Compliance Mapping

- **NIST SP 800-53 SC-39** (Process Isolation) — preventing cross-privilege information disclosure
- **NIST SP 800-53 SI-16** (Memory Protection) — hardware enforcement of access boundaries

## Sources

- Intel SDM Vol 3A Section 2.5 (Control Register CR4)
- AMD APM Vol 2 Section 5.5
- [Linux kernel umip.c](https://elixir.bootlin.com/linux/latest/source/arch/x86/kernel/umip.c)
- [LWN: User Mode Instruction Prevention (2017)](https://lwn.net/Articles/726210/)
