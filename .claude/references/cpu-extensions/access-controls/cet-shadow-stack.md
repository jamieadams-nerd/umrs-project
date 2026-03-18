# CET-SS (Control-flow Enforcement Technology: Shadow Stack)

## 23-Column Matrix Profile

| # | Column | Value |
|---|--------|-------|
| 1 | Feature name | CET-SS (Shadow Stack) |
| 2 | Vendor | Intel (Tiger Lake+, 11th gen, 2020), AMD (Zen 3+, 2020) |
| 3 | Category | 11 — CPU-Enforced Access Controls |
| 4 | Purpose | Hardware shadow stack defeats Return-Oriented Programming (ROP) by maintaining a separate, CPU-protected stack storing only return addresses. On every CALL the CPU pushes to both the normal stack and the shadow stack. On every RET the CPU compares the two; any mismatch raises a #CP (Control Protection) fault. Shadow stack pages have a special page-table encoding and cannot be written by normal memory operations. |
| 5 | Key instructions | WRSS/WRSSD/WRSSQ (write to shadow stack — privileged), SAVEPREVSSP/RSTORSSP (shadow stack pointer save/restore for context switching), SETSSBSY/CLRSSBSY (busy-bit management). CALL/RET are implicitly extended to maintain the shadow stack pointer. |
| 6 | CPUID detection | EAX=07H, ECX=0H, ECX bit 7 (SHSTK). Separate from IBT (EDX bit 20). Both bits advertise CET hardware; the OS must independently activate each sub-feature. |
| 7 | Linux `/proc/cpuinfo` flag | `shstk` |
| 8 | Linux detection — authoritative path | Primary: `/proc/cpuinfo` flags line for `shstk`. Per-process runtime status: `grep x86_Thread_features /proc/<pid>/status` — field `shstk` present when shadow stack is active for that process. `x86_Thread_features_locked: shstk` indicates locked (cannot be disabled). |
| 9 | Minimum CPU generations | Intel Tiger Lake (11th gen, 2020). AMD Zen 3 (2020). ARM equivalent: PAC (Pointer Authentication Codes) — see arm-access-controls.md. |
| 10 | Security benefit | Defeats the entire ROP attack class. ROP constructs chains of existing code fragments ending in RET; shadow stack mismatch detection makes every such RET raise #CP before control is diverted. A successful ROP gadget chain requires the shadow stack to be disabled or bypassed — which requires a write-primitive targeting shadow-stack-mapped memory (using WRSS), a significantly higher bar. Shadow stack does not protect against JOP — use CET-IBT for that. |
| 11 | Performance benefit | Negligible runtime overhead (< 1% in microbenchmarks). Context switches require saving/restoring the shadow stack pointer (SSP), adding a small but constant per-switch cost. |
| 12 | Assurance caveats | **Shared-library chain requirement:** All shared libraries in a process's dependency chain must be CET-capable. A single non-CET `.so` disables shadow stack for the entire process (enforced by the dynamic linker via GNU_PROPERTY_X86_FEATURE_1_AND). **glibc activation required:** glibc 2.39+ performs shadow stack activation at startup if the binary ELF property marks SHSTK and the kernel has CONFIG_X86_USER_SHADOW_STACK=y. **Rust limitation (CRITICAL):** Stable Rust does not support `-Z cf-protection=full` (tracking issue rust-lang/rust#93754). UMRS binaries compiled with stable Rust will NOT have SHSTK ELF notes and will NOT activate shadow stack on CET-capable hardware. Classify as INFORMATIONAL — Rust memory safety provides comparable ROP resistance. |
| 13 | Virtualization behavior | KVM: shadow stack passed through to guests when CONFIG_X86_USER_SHADOW_STACK=y and CPU capable. VMware: CET passthrough supported in recent releases. Guest OS must independently activate via arch_prctl(ARCH_SHSTK_ENABLE). Hardware page-table encoding of shadow stack pages is enforced at physical CPU level. |
| 14 | Firmware / BIOS / microcode dependency | `{ bios_enable_required: false, microcode_required: false }` — CET is a native CPU feature requiring no firmware gate. Unlike SGX or SEV, no BIOS option is needed. |
| 15 | Audit-card relevance | **Critical/Defensive** |
| 16 | Recommended disposition when unused | CET-capable CPU + GCC-compiled C/C++ binary without `-fcf-protection=return` or `-fcf-protection=full` = **HIGH** finding. CET-capable CPU + Rust binary without CET ELF note = **INFORMATIONAL** (Rust memory safety mitigates). Absence of `shstk` in `/proc/cpuinfo` on post-2020 hardware may indicate hypervisor masking. |
| 17 | Software utilization detection method | Layer 1: `shstk` in `/proc/cpuinfo`. Layer 2 (static): `readelf -n <binary>` — look for `x86 feature: IBT, SHSTK` in `.note.gnu.property`. Layer 2 (runtime): `grep x86_Thread_features /proc/<pid>/status`. Layer 2 (kernel): `CONFIG_X86_USER_SHADOW_STACK=y` in `/boot/config-$(uname -r)`. |
| 18 | FIPS utilization requirement | N/A — CET is control-flow integrity, not a cryptographic primitive. NIST SP 800-218 SSDF recommends CFI where available. |
| 19 | Active mitigation status path | No `/sys/devices/system/cpu/vulnerabilities/` entry — CET is proactive hardening, not a vulnerability mitigation. Per-process activation via `/proc/<pid>/status` is the authoritative runtime check. |
| 20 | Feature accessible vs advertised | No known BIOS gate. CPUID ECX bit 7 at leaf 7.0 is authoritative. Hypervisors may mask the CPUID bit. No kernel cmdline parameter to disable — only CONFIG_X86_USER_SHADOW_STACK=n at build time prevents activation. |
| 21 | Guest-vs-host discrepancy risk | **Medium.** CET passthrough requires explicit hypervisor support. Older VMware ESXi or KVM without CET capability forwarding will mask the CPUID bit. |
| 22 | Notes | CET-SS and CET-IBT are complementary — SS defeats ROP, IBT defeats JOP/COP. Both should be active on capable systems. RHEL 10 kernel: CONFIG_X86_USER_SHADOW_STACK=y. GCC 14.2 (RHEL 10 default): `-fcf-protection=full` by default — all C/C++ binaries get SHSTK ELF notes automatically. Critical gap: Rust binaries and third-party binaries not built with `-fcf-protection`. |
| 23 | Sources | Intel CET Specification 334525-003; Linux arch/x86/shstk.rst; glibc 2.39 release; GCC 14 changes; rust-lang/rust#93754 |

## Attack Class Blocked: ROP (Return-Oriented Programming)

**Attack pattern:** Memory corruption (buffer overflow, use-after-free) overwrites return addresses on the normal stack. Attacker chains short existing code sequences ending in `RET` (gadgets) to construct arbitrary computation without injecting shellcode. SMEP and NX do not stop ROP because gadgets are in existing executable code.

**With CET-SS:** Every `RET` compares return address from normal stack against shadow stack. Overwriting normal stack does not overwrite shadow stack (shadow stack pages require WRSS instructions). Mismatch triggers `#CP` immediately.

**Residual risk:** Attacker with write primitive reaching shadow stack memory (requires kernel-mode access or WRSS gadget) could forge entries. Dramatically narrower attack surface.

## CVE / Vulnerability Table

| ID | Name | Year | Impact | CET-SS Relevance |
|----|------|------|--------|-----------------|
| CVE-2021-33909 | Sequoia fs/seq_file | 2021 | Local root via stack overflow | Stack corruption blocked by shadow stack |
| CVE-2023-32233 | Netfilter nf_tables UAF | 2023 | Kernel priv esc via ROP | Kernel shadow stack would raise #CP |
| N/A | Generic ROP exploitation | 2007+ | Bypass NX/DEP via code gadgets | CET-SS is the hardware answer to ROP |

## ELF Marking

CET capability recorded in `.note.gnu.property`:

| Property bit | Meaning |
|---|---|
| `GNU_PROPERTY_X86_FEATURE_1_SHSTK` (bit 1) | Binary supports shadow stack |
| `GNU_PROPERTY_X86_FEATURE_1_IBT` (bit 0) | Binary supports IBT |

Dynamic linker ANDs all properties across executable + all shared libraries. A single non-CET shared library disables CET for the entire process.

Verification: `readelf -n <binary> | grep "x86 feature"` → expect `IBT, SHSTK`

## Kernel Build Dependencies

| Config Option | Feature | Default (RHEL 10) | Since |
|---------------|---------|-------------------|-------|
| `CONFIG_X86_USER_SHADOW_STACK` | User-space shadow stack | `=y` | Linux 6.6 |
| `CONFIG_X86_KERNEL_SHADOW_STACK` | Kernel-mode shadow stack | Experimental | Linux 6.13+ |

## RHEL 10 / UMRS Status

| Component | CET-SS Status | Notes |
|-----------|--------------|-------|
| RHEL 10 kernel | Supported | CONFIG_X86_USER_SHADOW_STACK=y |
| GCC 14.2 | Default on | `-fcf-protection=full` |
| glibc 2.39 | Opt-in activation | Checks ELF property at startup |
| **Stable Rust** | **NOT available** | rust-lang/rust#93754 |
| **UMRS binaries** | **No CET-SS** | INFORMATIONAL; Rust memory safety compensates |

## Compliance Mapping

- **NIST SP 800-53 SI-16** (Memory Protection) — hardware-enforced CFI
- **NIST SP 800-218 SSDF PW.5** — CFI mechanisms
- **NSA RTB RAIN** (Non-Bypassability) — hardware enforcement
- **CMMC SC.L2-3.13.11** — architectural designs for information security

## Posture Check Specification

1. `shstk` in `/proc/cpuinfo` (Layer 1 — hardware)
2. `CONFIG_X86_USER_SHADOW_STACK=y` in boot config (kernel support)
3. `readelf -n <binary>` for SHSTK in `.note.gnu.property` (Layer 2 — binary)
4. `grep x86_Thread_features /proc/<pid>/status` (Layer 2 — runtime)
5. C/C++ binary missing SHSTK on CET system → **HIGH**
6. Rust binary missing SHSTK → **INFORMATIONAL**

## Sources

- [Intel CET Specification 334525-003](https://kib.kiev.ua/x86docs/Intel/CET/334525-003.pdf)
- [CET Shadow Stack — Linux Kernel Documentation](https://docs.kernel.org/arch/x86/shstk.html)
- [GCC 14 Release Changes](https://gcc.gnu.org/gcc-14/changes.html)
- [Rust CET Tracking Issue #93754](https://github.com/rust-lang/rust/issues/93754)
- [glibc 2.39 Shadow Stack — LWN](https://lwn.net/Articles/960309/)
- [RHEL 10 Security Hardening](https://docs.redhat.com/en/documentation/red_hat_enterprise_linux/10/html-single/security_hardening/index)
