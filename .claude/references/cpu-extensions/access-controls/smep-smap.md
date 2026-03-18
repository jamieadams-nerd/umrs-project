# SMEP / SMAP (Supervisor Mode Execution & Access Prevention)

## 23-Column Matrix Profile

| # | Column | Value |
|---|--------|-------|
| 1 | Feature name | SMEP (Supervisor Mode Execution Prevention) / SMAP (Supervisor Mode Access Prevention) |
| 2 | Vendor | Both (Intel & AMD) |
| 3 | Category | 11 — CPU-Enforced Access Controls |
| 4 | Purpose | SMEP prevents the kernel from executing code mapped in user-space pages. SMAP prevents the kernel from reading or writing user-space pages unless explicitly permitted (via STAC/CLAC). Together they eliminate the ret2usr exploit class and user-space data pivot attacks. |
| 5 | Key instructions | STAC (Set AC Flag — temporarily permit user access), CLAC (Clear AC Flag — re-enable SMAP), native_write_cr4 (kernel internal) |
| 6 | CPUID detection | SMEP: EAX=07H, ECX=0H, EBX bit 7. SMAP: EAX=07H, ECX=0H, EBX bit 20. |
| 7 | Linux `/proc/cpuinfo` flag | `smep`, `smap` |
| 8 | Linux detection — authoritative path | CR4 register bits 20 (SMEP) and 21 (SMAP). Kernel enforces automatically when CPUID indicates support. No sysfs interface — presence in `/proc/cpuinfo` is authoritative since kernel enables unconditionally. |
| 9 | Minimum CPU generations | SMEP: Intel Ivy Bridge (3rd gen, 2012), AMD Zen 1 (2017). SMAP: Intel Broadwell (5th gen, 2014), AMD Zen 1 (2017). |
| 10 | Security benefit | SMEP kills the entire ret2usr exploit class — kernel cannot be redirected to execute attacker-controlled user-space code. SMAP prevents kernel exploits from reading or writing attacker-controlled user-space data structures (user-land pivot attacks). Together they force kernel exploits to use ROP/JOP within kernel text, dramatically increasing exploit difficulty. |
| 11 | Performance benefit | Negligible performance impact. SMAP requires STAC/CLAC around legitimate copy_to_user/copy_from_user paths but these are single-instruction operations. |
| 12 | Assurance caveats | **ret2dir bypass**: Kemerlis et al. (USENIX Security 2014) demonstrated that the kernel's direct-mapped physical memory region (physmap) can be used to bypass both SMEP and SMAP, since physmap pages are kernel-mapped but contain user-controlled data. Linux mitigations: KASLR randomizes physmap base, and some distributions randomize the direct map. **CR4 pinning bypass**: Early SMEP bypasses used ROP to call `native_write_cr4()` to clear the SMEP bit. Modern kernels pin CR4 bits using `cr4_pinned_mask` (Linux 5.4+), preventing this. |
| 13 | Virtualization behavior | KVM: passes through SMEP/SMAP to guests natively when available on host CPU. VMware: supports SMEP/SMAP passthrough. Hyper-V: supports SMEP/SMAP. Guest cannot distinguish emulated from hardware — but these features are always hardware-enforced when present, not emulated. |
| 14 | Firmware / BIOS / microcode dependency | `{ bios_enable_required: false, microcode_required: false }` — SMEP/SMAP are purely CPU features controlled via CR4. No firmware gate. |
| 15 | Audit-card relevance | **Critical/Defensive** |
| 16 | Recommended disposition when unused | N/A — SMEP and SMAP cannot be "unused." If the CPU supports them, the Linux kernel enables them automatically. The only way to disable is `nosmep` / `nosmap` kernel command line parameters, which should be flagged as a **CRITICAL** audit finding. |
| 17 | Software utilization detection method | Check that `smep` and `smap` appear in `/proc/cpuinfo` flags. Verify `nosmep` / `nosmap` are NOT present in `/proc/cmdline`. |
| 18 | FIPS utilization requirement | N/A |
| 19 | Active mitigation status path | No dedicated sysfs vulnerability path. SMEP/SMAP are always-on protections, not per-vulnerability mitigations. |
| 20 | Feature accessible vs advertised | SMEP/SMAP cannot be BIOS-disabled on any known platform. If CPUID reports support, they are available. However, `nosmep`/`nosmap` kernel command line parameters disable them at the OS level. |
| 21 | Guest-vs-host discrepancy risk | Low. KVM passes through SMEP/SMAP when available. Older hypervisors that do not support SMEP/SMAP will simply not expose the CPUID bits. |
| 22 | Notes | SMEP and SMAP are foundational kernel hardening features. Their absence on any modern system (post-2017 x86_64) would indicate very old hardware. On RHEL 10 target hardware, both should always be present. They work in tandem with NX/XD — NX prevents executing data pages, SMEP prevents executing user pages from kernel mode. |
| 23 | Sources | Intel SDM Vol 3A Section 4.6; AMD APM Vol 2 Section 5.6.1; Kemerlis et al. "ret2dir: Rethinking Kernel Isolation" (USENIX Security 2014); LWN: Supervisor Mode Access Prevention (2012); Linux kernel SMAP commit by H. Peter Anvin |

## Attack Classes Blocked

### SMEP — Blocks: ret2usr (Return-to-User)

**Attack pattern:** Kernel vulnerability allows attacker to redirect kernel execution (via corrupted function pointer, return address, etc.) to code in user-space memory. Without SMEP, the kernel happily executes the attacker's user-space shellcode at ring 0 privilege.

**With SMEP:** CPU raises a page fault (#PF) when ring 0 attempts to fetch instructions from a user-mode page (page table U/S bit = User). The attack is stopped at the hardware level.

### SMAP — Blocks: User-space Data Pivot

**Attack pattern:** Kernel exploit reads or writes attacker-controlled data structures in user-space memory. Even without executing user code, the kernel can be manipulated via crafted data (e.g., fake struct operations tables placed in user memory).

**With SMAP:** CPU raises a page fault when ring 0 attempts to read or write a user-mode page, except inside STAC/CLAC windows used by legitimate copy_to_user/copy_from_user paths.

## CVE / Vulnerability Table

| ID | Name | Year | Impact | Relevance |
|----|------|------|--------|-----------|
| N/A | ret2usr class | Pre-2012 | Trivial kernel privilege escalation via user-space shellcode | SMEP eliminates this entire class |
| N/A | ret2dir (Kemerlis et al.) | 2014 | Bypasses SMEP/SMAP via kernel physmap | Requires KASLR and physmap randomization as defense |
| CVE-2013-1763 | Linux kernel netlink | 2013 | Kernel privilege escalation, originally used ret2usr | Post-SMEP, exploit required ROP chain adaptation |
| CVE-2017-7308 | Linux packet socket AF_PACKET | 2017 | Kernel privilege escalation with SMEP/SMAP bypass via ROP | Demonstrates that SMEP/SMAP raise the bar but do not eliminate kernel exploitation |
| N/A | CR4 bit-flip SMEP bypass | 2016 | ROP chain calls native_write_cr4 to clear SMEP bit | Fixed by CR4 pinning in Linux 5.4+ |

## Kernel Build Dependencies

| Config Option | Feature | Default (RHEL 10) | Since |
|---------------|---------|-------------------|-------|
| `CONFIG_X86_SMEP` | Supervisor Mode Execution Prevention | `=y` (always on for x86_64) | Linux 3.0 (2011) |
| `CONFIG_X86_SMAP` | Supervisor Mode Access Prevention | `=y` (always on for x86_64) | Linux 3.7 (2012) |

Both are non-optional in modern x86_64 kernels. The config options exist for architecture completeness but are always enabled.

## Compliance Mapping

- **NIST SP 800-53 SC-39** (Process Isolation) — SMEP/SMAP enforce hardware boundary between kernel and user address spaces
- **NIST SP 800-53 SI-16** (Memory Protection) — hardware-enforced execution prevention
- **NSA RTB RAIN** (Non-Bypassability) — CPU-enforced, not software policy
- **CMMC SC.L2-3.13.4** — separation of user functionality from system management functionality

## Posture Check Specification

1. Verify `smep` AND `smap` in `/proc/cpuinfo` flags (Layer 1)
2. Verify `nosmep` and `nosmap` are NOT in `/proc/cmdline` (Layer 2 — not disabled)
3. If CPU lacks SMEP/SMAP → **CRITICAL** finding (hardware too old for security posture)
4. If `nosmep` or `nosmap` in cmdline → **CRITICAL** finding (protection intentionally disabled)

## Sources

- [Intel SDM — CPUID Enumeration](https://www.intel.com/content/www/us/en/developer/articles/technical/software-security-guidance/technical-documentation/cpuid-enumeration-and-architectural-msrs.html)
- [Supervisor Mode Access Prevention — Wikipedia](https://en.wikipedia.org/wiki/Supervisor_Mode_Access_Prevention)
- [ret2dir: Rethinking Kernel Isolation — USENIX Security 2014](https://www.usenix.org/conference/usenixsecurity14/technical-sessions/presentation/kemerlis)
- [LWN: Supervisor Mode Access Prevention (2012)](https://lwn.net/Articles/517475/)
- [SMEP — OSDev Wiki](https://wiki.osdev.org/Supervisor_Memory_Protection)
- [Linux kernel SMAP commit](https://lore.kernel.org/lkml/tip-52b6179ac87d33c2eeaff5292786a10fe98cff64@git.kernel.org/)
