# CET-IBT (Control-flow Enforcement Technology: Indirect Branch Tracking)

## 23-Column Matrix Profile

| # | Column | Value |
|---|--------|-------|
| 1 | Feature name | CET-IBT (Indirect Branch Tracking) |
| 2 | Vendor | Intel (Tiger Lake+, 11th gen, 2020), AMD (Zen 3+, 2020) |
| 3 | Category | 11 — CPU-Enforced Access Controls |
| 4 | Purpose | Hardware enforcement that indirect branches (JMP/CALL through register or memory) must land on an ENDBR64 instruction. CPU enters a "tracker" state after every indirect JMP or CALL; the next instruction MUST be ENDBR64 or the CPU raises #CP (Control Protection) fault. This defeats JOP (Jump-Oriented Programming) and COP (Call-Oriented Programming) attacks. ENDBR64 opcode is `F3 0F 1E FA` — a NOP on pre-CET CPUs, ensuring backward compatibility. |
| 5 | Key instructions | ENDBR32/ENDBR64 (End Branch — valid landing pad marker). No new control instructions — IBT modifies the behavior of existing indirect JMP/CALL. NOTRACK prefix (3EH) allows bypassing IBT for specific indirect branches (must be used sparingly). |
| 6 | CPUID detection | EAX=07H, ECX=0H, EDX bit 20 (IBT). Separate from Shadow Stack (ECX bit 7). |
| 7 | Linux `/proc/cpuinfo` flag | `ibt` |
| 8 | Linux detection — authoritative path | `/proc/cpuinfo` flags: `ibt`. Kernel IBT: `CONFIG_X86_KERNEL_IBT=y`. Binary IBT: `readelf -n <binary>` for `GNU_PROPERTY_X86_FEATURE_1_IBT`. |
| 9 | Minimum CPU generations | Intel Tiger Lake (11th gen, 2020). AMD Zen 3 (2020). ARM equivalent: BTI (Branch Target Identification) — see arm-access-controls.md. |
| 10 | Security benefit | Defeats JOP and COP attack classes. JOP chains indirect jumps through attacker-chosen gadgets; COP chains indirect calls. Without IBT, any address in executable memory is a valid indirect branch target. With IBT, only addresses marked with ENDBR64 are valid — reducing the gadget surface by orders of magnitude. Combined with CET-SS (which defeats ROP), IBT provides comprehensive hardware CFI. |
| 11 | Performance benefit | Negligible overhead. ENDBR64 executes as a NOP on the fast path. The tracker state check is a microarchitectural pipeline check with no measurable throughput impact. |
| 12 | Assurance caveats | **Same shared-library chain requirement as CET-SS:** All libraries must be IBT-capable. Single non-IBT .so disables IBT for the process. **NOTRACK prefix:** Legacy code using NOTRACK weakens IBT by allowing unvalidated indirect branches. **JIT compilers:** JIT engines (JavaScript, Java, eBPF) must emit ENDBR64 at all indirect branch targets or IBT will fault. Kernel eBPF JIT emits ENDBR64 since Linux 5.18. **Same Rust limitation:** Stable Rust does not support CET (rust-lang/rust#93754). |
| 13 | Virtualization behavior | KVM: IBT passed through to guests when hardware supports it. Kernel IBT (CONFIG_X86_KERNEL_IBT) is active on the host kernel regardless of guest capability. Guest IBT requires guest kernel support + guest binary ENDBR64 marking. |
| 14 | Firmware / BIOS / microcode dependency | `{ bios_enable_required: false, microcode_required: false }` — same as CET-SS. |
| 15 | Audit-card relevance | **Critical/Defensive** |
| 16 | Recommended disposition when unused | C/C++ binary without `-fcf-protection=full` or `-fcf-protection=branch` on CET-capable system = **HIGH** finding. Rust binary = **INFORMATIONAL**. |
| 17 | Software utilization detection method | Layer 1: `ibt` in `/proc/cpuinfo`. Layer 2 (static): `readelf -n <binary>` for `IBT` in `.note.gnu.property`. Layer 2 (kernel): `CONFIG_X86_KERNEL_IBT=y` in boot config. |
| 18 | FIPS utilization requirement | N/A |
| 19 | Active mitigation status path | No sysfs vulnerability entry — proactive hardening. |
| 20 | Feature accessible vs advertised | No BIOS gate. CPUID EDX bit 20 at leaf 7.0 is authoritative. Hypervisors may mask. |
| 21 | Guest-vs-host discrepancy risk | Medium — same as CET-SS. |
| 22 | Notes | CET-IBT complements CET-SS. SS defeats ROP (return-based); IBT defeats JOP/COP (jump/call-based). Both should be active. Kernel IBT (CONFIG_X86_KERNEL_IBT) has been active since Linux 5.18 — kernel modules must also be compiled with IBT or they are rejected. RHEL 10: GCC 14.2 default `-fcf-protection=full` covers both IBT and SHSTK. |
| 23 | Sources | Intel CET Specification 334525-003; Linux kernel IBT documentation; GCC 14 changes |

## Attack Class Blocked: JOP/COP

**JOP (Jump-Oriented Programming):** Attacker chains indirect jumps through gadgets ending in `jmp [reg]`. Unlike ROP, JOP does not use the stack — shadow stack alone cannot stop it.

**COP (Call-Oriented Programming):** Similar to JOP but uses indirect `call` instructions. Both bypass NX/DEP and SMEP because they use existing executable code.

**With CET-IBT:** Every indirect branch target must begin with ENDBR64. Code not starting with ENDBR64 is not a valid gadget target. This eliminates the vast majority of potential gadgets and makes JOP/COP chain construction infeasible in practice.

## CVE / Vulnerability Table

| ID | Name | Year | Impact | CET-IBT Relevance |
|----|------|------|--------|-------------------|
| N/A | Generic JOP exploitation | 2011+ | Bypass NX + shadow stack via indirect jumps | IBT eliminates JOP gadget availability |
| CVE-2022-0185 | Linux fs_context heap overflow | 2022 | Container escape using code reuse | IBT would restrict valid branch targets |
| N/A | eBPF JIT gadget injection | Various | Attacker-controlled indirect branches | Kernel IBT requires ENDBR64 in eBPF JIT output |

## Kernel Build Dependencies

| Config Option | Feature | Default (RHEL 10) | Since |
|---------------|---------|-------------------|-------|
| `CONFIG_X86_KERNEL_IBT` | Kernel-mode IBT | `=y` | Linux 5.18 |
| `CONFIG_X86_USER_SHADOW_STACK` | User-space CET (companion) | `=y` | Linux 6.6 |

## Compliance Mapping

- **NIST SP 800-53 SI-16** (Memory Protection) — hardware-enforced CFI
- **NIST SP 800-218 SSDF PW.5** — CFI mechanisms
- **NSA RTB RAIN** (Non-Bypassability)

## Posture Check Specification

1. `ibt` in `/proc/cpuinfo` (Layer 1)
2. `CONFIG_X86_KERNEL_IBT=y` in boot config (kernel IBT)
3. `readelf -n <binary>` for IBT in `.note.gnu.property` (Layer 2)
4. C/C++ binary missing IBT on CET system → **HIGH**
5. Rust binary missing IBT → **INFORMATIONAL**

## Sources

- [Intel CET Specification 334525-003](https://kib.kiev.ua/x86docs/Intel/CET/334525-003.pdf)
- [CET Shadow Stack — Linux Kernel Documentation](https://docs.kernel.org/arch/x86/shstk.html)
- [Kernel IBT — LWN](https://lwn.net/Articles/889475/)
- [GCC 14 Release Changes](https://gcc.gnu.org/gcc-14/changes.html)
- [Rust CET Tracking Issue #93754](https://github.com/rust-lang/rust/issues/93754)
