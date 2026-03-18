# ARM Access Control Extensions (PAC, BTI, MTE)

## ARM Pointer Authentication (PAC) — Feature #58

### 23-Column Matrix Profile

| # | Column | Value |
|---|--------|-------|
| 1 | Feature name | ARM Pointer Authentication Codes (PAC) |
| 2 | Vendor | ARM (ARMv8.3-A and later) |
| 3 | Category | 11 — CPU-Enforced Access Controls (ARM equivalent of CET-SS) |
| 4 | Purpose | Signs and verifies pointers (return addresses, function pointers) using a cryptographic MAC stored in the unused upper bits of the pointer. On function entry, PACIA signs the return address with a secret key and context value. On function exit, AUTIA verifies the signature before returning. A corrupted return address (ROP) will have an invalid PAC and trigger a fault. This is ARM's equivalent of Intel CET Shadow Stack — both defend against ROP, but PAC uses cryptographic signatures rather than a parallel stack. |
| 5 | Key instructions | PACIA/PACIB (sign pointer with key A/B and context), AUTIA/AUTIB (verify and strip PAC), PACDA/PACDB (sign data pointer), AUTDA/AUTDB (verify data pointer), XPAC/XPACD (strip PAC without verification — for address calculation). |
| 6 | CPUID detection | ARM ID_AA64ISAR1_EL1 register: APA/API fields (bits 7:4 and 11:8). `/proc/cpuinfo` Features line. |
| 7 | Linux `/proc/cpuinfo` flag | `paca` (PAC address key A), `pacg` (PAC generic key) |
| 8 | Linux detection — authoritative path | `/proc/cpuinfo` Features: `paca`, `pacg`. Kernel support: `CONFIG_ARM64_PTR_AUTH`. Per-process: kernel signs return addresses automatically when binary is compiled with PAC. |
| 9 | Minimum CPU generations | ARMv8.3-A: Apple A12 (2018), Cortex-A76 (2018), Neoverse N1. All Apple Silicon M-series. Qualcomm Kryo 485+. |
| 10 | Security benefit | Defeats ROP by cryptographically signing return addresses. An attacker who overwrites a return address on the stack cannot forge a valid PAC without knowing the secret key. Unlike CET-SS (which uses a parallel stack), PAC embeds the verification in the pointer itself — no additional memory structure to maintain. PAC also protects function pointers and data pointers when used with PACDA/AUTDA. |
| 11 | Performance benefit | Minimal overhead (~1% in typical workloads). PAC instructions are pipelined and execute in 1-2 cycles. |
| 12 | Assurance caveats | **PAC strength depends on available pointer bits:** On systems with large virtual address spaces (52-bit VA), fewer bits are available for PAC, reducing the key space. With 48-bit VA (common), ~7 bits available for PAC — 1/128 chance of forging a valid PAC by brute force. **PAC key management:** Kernel generates per-process random keys. Fork inherits keys; exec regenerates. Key extraction vulnerability would compromise all PAC for that process. **PACMAN attack (2022):** Speculative execution can be used to brute-force PAC values without triggering faults — demonstrated on Apple M1. Mitigated by combining PAC with BTI. |
| 13 | Virtualization behavior | KVM: PAC passed through to guests. PAC keys are part of the CPU state saved/restored on VM entry/exit. No known passthrough issues. |
| 14 | Firmware / BIOS / microcode dependency | `{ bios_enable_required: false, microcode_required: false }` — CPU feature, always available on ARMv8.3-A+. |
| 15 | Audit-card relevance | **Important** |
| 16 | Recommended disposition when unused | PAC-capable CPU + binary compiled without `-mbranch-protection=pac-ret` or `-mbranch-protection=standard` = finding. GCC/Clang on ARM enable PAC when `-mbranch-protection=standard` is used. Check compiler flags in build system. |
| 17 | Software utilization detection method | Layer 1: `paca`, `pacg` in `/proc/cpuinfo`. Layer 2: binary compiled with PAC instructions — `objdump -d <binary> | grep -c paciasp` (PAC on function entry). `readelf -n <binary>` for `.note.gnu.property` PAC marking. |
| 18 | FIPS utilization requirement | N/A |
| 19 | Active mitigation status path | No sysfs vulnerability entry — proactive hardening. |
| 20 | Feature accessible vs advertised | Always available on ARMv8.3-A+. No BIOS gate. |
| 21 | Guest-vs-host discrepancy risk | Low. |
| 22 | Notes | Jamie's octopussy VM shows `paca` and `pacg` in `/proc/cpuinfo` (Apple Silicon via Parallels). OpenSSL on octopussy was compiled with `-mbranch-protection=standard`, confirming PAC is active for the system crypto library. UMRS Rust binaries: Rust supports PAC on aarch64 targets when compiled with `-C link-arg=-mbranch-protection=standard` or via target specification. |
| 23 | Sources | ARM Architecture Reference Manual (ARMv8.3-A); ARM Learn: Pointer Authentication; PACMAN paper (MIT, 2022) |

---

## ARM Branch Target Identification (BTI) — Feature #59

### 23-Column Matrix Profile

| # | Column | Value |
|---|--------|-------|
| 1 | Feature name | ARM Branch Target Identification (BTI) |
| 2 | Vendor | ARM (ARMv8.5-A and later) |
| 3 | Category | 11 — CPU-Enforced Access Controls (ARM equivalent of CET-IBT) |
| 4 | Purpose | Hardware enforcement that indirect branches must land on a BTI instruction. Like Intel's CET-IBT with ENDBR64, ARM BTI requires specific landing-pad instructions at every valid indirect branch target. Indirect branches to non-BTI instructions raise a Branch Target Exception. This defeats JOP/COP attacks on ARM. |
| 5 | Key instructions | BTI (Branch Target Identification — landing pad marker). Variants: `BTI c` (valid for indirect calls), `BTI j` (valid for indirect jumps), `BTI jc` (valid for both). BTI instruction encoding overlaps with HINT space — executes as NOP on pre-BTI CPUs (backward compatible). |
| 6 | CPUID detection | ARM ID_AA64PFR1_EL1 register: BT field (bits 3:0). `/proc/cpuinfo` Features line. |
| 7 | Linux `/proc/cpuinfo` flag | `bti` |
| 8 | Linux detection — authoritative path | `/proc/cpuinfo` Features: `bti`. Kernel: `CONFIG_ARM64_BTI`. Per-binary: ELF `.note.gnu.property` with BTI marking. Kernel BTI: `CONFIG_ARM64_BTI_KERNEL`. |
| 9 | Minimum CPU generations | ARMv8.5-A: Cortex-A77 (2019), Cortex-X1, Neoverse V1. Apple M1+ (2020). |
| 10 | Security benefit | Eliminates JOP/COP gadgets by restricting valid indirect branch targets to BTI-marked instructions. Same principle as Intel CET-IBT. Combined with PAC (return address protection), provides comprehensive CFI on ARM. |
| 11 | Performance benefit | Negligible — BTI instructions execute as NOPs on the fast path. |
| 12 | Assurance caveats | **Same shared-library chain issue as CET-IBT:** All libraries must be BTI-marked. Dynamic linker ANDs BTI properties. **ELF marking required:** Binary must be compiled with `-mbranch-protection=standard` or `-mbranch-protection=bti`. **Guarded pages:** Linux uses PROT_BTI flag on mmap to enable BTI enforcement per-page. |
| 13 | Virtualization behavior | KVM: BTI passed through. BTI enforcement is per-page (PROT_BTI in page table), so guest controls its own BTI policy. |
| 14 | Firmware / BIOS / microcode dependency | `{ bios_enable_required: false, microcode_required: false }` |
| 15 | Audit-card relevance | **Important** |
| 16 | Recommended disposition when unused | BTI-capable CPU + binary without BTI ELF marking = finding (severity depends on language — C/C++ = HIGH, Rust = INFORMATIONAL). |
| 17 | Software utilization detection method | Layer 1: `bti` in `/proc/cpuinfo`. Layer 2: `readelf -n <binary>` for BTI in `.note.gnu.property`. Compiler: `-mbranch-protection=standard` enables both PAC+BTI. |
| 18 | FIPS utilization requirement | N/A |
| 19 | Active mitigation status path | No sysfs vulnerability entry. |
| 20 | Feature accessible vs advertised | Always available on ARMv8.5-A+. |
| 21 | Guest-vs-host discrepancy risk | Low. |
| 22 | Notes | GCC/Clang `-mbranch-protection=standard` enables BOTH PAC and BTI. This is the recommended flag for ARM security hardening. Jamie's OpenSSL build on octopussy uses this flag. RHEL 10 aarch64 builds should also use it. |
| 23 | Sources | ARM Architecture Reference Manual (ARMv8.5-A); ARM Learn: Branch Target Identification |

---

## ARM Memory Tagging Extension (MTE) — Feature #60

### 23-Column Matrix Profile

| # | Column | Value |
|---|--------|-------|
| 1 | Feature name | ARM Memory Tagging Extension (MTE) |
| 2 | Vendor | ARM (ARMv8.5-A and later) |
| 3 | Category | 11 — CPU-Enforced Access Controls (unique to ARM — no x86 equivalent) |
| 4 | Purpose | Hardware memory safety: every 16-byte aligned memory granule is assigned a 4-bit tag in physical memory. Pointers also carry a 4-bit tag in their upper bits. On every memory access, the CPU compares the pointer tag with the memory tag; a mismatch can raise a fault (synchronous mode) or log asynchronously. This detects use-after-free, heap buffer overflow, stack buffer overflow, and other spatial/temporal memory safety bugs at the hardware level. |
| 5 | Key instructions | IRG (Insert Random Tag — generate random tag in pointer), ADDG (Add with Tag — pointer arithmetic preserving tag), STG/ST2G/STZ2G (Store Allocation Tags — set memory tags), LDG (Load Allocation Tag), CMPP (Compare with Tag). |
| 6 | CPUID detection | ARM ID_AA64PFR1_EL1 register: MTE field (bits 11:8). Value 0=none, 1=MTE (EL0 only), 2=MTE2 (full), 3=MTE3 (asymmetric). |
| 7 | Linux `/proc/cpuinfo` flag | `mte`, `mte2`, `mte3` |
| 8 | Linux detection — authoritative path | `/proc/cpuinfo` Features: `mte`/`mte2`/`mte3`. Kernel: `CONFIG_ARM64_MTE`. Per-process: `prctl(PR_SET_TAGGED_ADDR_CTRL)` to enable. `/proc/<pid>/smaps` shows tagged regions. |
| 9 | Minimum CPU generations | ARMv8.5-A MTE: Cortex-A510/A710/X2 (2021). Google Pixel 8 (2023) — first mass-market MTE device. Not yet available on Apple Silicon. Limited server availability. |
| 10 | Security benefit | Detects entire classes of memory safety bugs at runtime with hardware speed: use-after-free (freed memory re-tagged, stale pointer has old tag), heap overflow (adjacent allocations have different tags), stack overflow (stack frames tagged differently). This is the hardware equivalent of AddressSanitizer but with production-viable overhead. MTE is unique — x86 has no equivalent. |
| 11 | Performance benefit | Synchronous mode: ~3-5% overhead (comparable to production use). Asymmetric mode (MTE3): reads checked synchronously, writes checked asynchronously — lower overhead. Significantly faster than software sanitizers (ASan: 2-3x overhead). |
| 12 | Assurance caveats | **4-bit tag = 1/16 collision probability:** An attacker with sufficient attempts can brute-force the correct tag. MTE is probabilistic, not deterministic. **Tag granularity:** 16-byte aligned — sub-granule overflows not detected. **Availability:** Limited hardware availability as of 2026. Not on Apple Silicon. Server-class ARM with MTE is emerging but not ubiquitous. **Kernel support maturity:** Linux MTE support is still evolving. Some edge cases in signal handling and fork semantics. |
| 13 | Virtualization behavior | KVM: MTE passthrough supported on MTE-capable hardware. Guest MTE requires host kernel MTE support. Tag storage requires additional physical memory (~3% overhead for tag storage). |
| 14 | Firmware / BIOS / microcode dependency | `{ bios_enable_required: false, microcode_required: false }` — CPU feature. Tag storage in memory controller is transparent. |
| 15 | Audit-card relevance | **Important** |
| 16 | Recommended disposition when unused | MTE present but not enabled by applications = **INFORMATIONAL**. MTE is opt-in per-process. Its primary value is for memory-safe deployment of C/C++ code. Rust code benefits less (memory safety at compile time), but MTE can catch unsafe block bugs. |
| 17 | Software utilization detection method | Layer 1: `mte`/`mte2`/`mte3` in `/proc/cpuinfo`. Layer 2: per-process via `prctl(PR_SET_TAGGED_ADDR_CTRL)`. Heap allocators (scudo, glibc malloc) can be compiled with MTE support. |
| 18 | FIPS utilization requirement | N/A |
| 19 | Active mitigation status path | No sysfs vulnerability entry. |
| 20 | Feature accessible vs advertised | Always available when CPUID indicates. No BIOS gate. |
| 21 | Guest-vs-host discrepancy risk | Medium — MTE requires host kernel support for guest passthrough. |
| 22 | Notes | MTE is the most novel feature in this category. It provides hardware memory safety — a capability x86 lacks entirely. For UMRS, MTE is relevant on ARM deployment targets: it could harden C library dependencies (glibc, OpenSSL) against memory corruption. Rust code with `#![forbid(unsafe_code)]` benefits minimally since Rust's borrow checker provides compile-time guarantees. Monitor MTE server availability for future UMRS ARM deployments. |
| 23 | Sources | ARM Architecture Reference Manual (ARMv8.5-A MTE); ARM MTE whitepaper; Google Android MTE documentation; Linux kernel MTE documentation |

## Cross-Reference: ARM vs x86 Access Controls

| Protection | x86 Equivalent | ARM Feature | Notes |
|------------|---------------|-------------|-------|
| Return address protection | CET Shadow Stack | PAC | PAC uses crypto; SS uses parallel stack |
| Indirect branch validation | CET-IBT (ENDBR64) | BTI | Same concept, different instruction |
| Memory tagging | None | MTE | Unique to ARM |
| Supervisor execution prevention | SMEP | PXN (Privileged Execute Never) | Equivalent |
| Supervisor access prevention | SMAP | PAN (Privileged Access Never) | Equivalent |
| User instruction prevention | UMIP | N/A | ARM doesn't expose equivalent tables |
| Memory domain isolation | PKU | MTE domains (partial) | Different mechanisms |

## Compiler Flag Summary

| Flag | Effect | PAC | BTI |
|------|--------|-----|-----|
| `-mbranch-protection=none` | No protection | No | No |
| `-mbranch-protection=pac-ret` | Sign return addresses only | Yes | No |
| `-mbranch-protection=bti` | BTI landing pads only | No | Yes |
| `-mbranch-protection=standard` | PAC + BTI (recommended) | Yes | Yes |
| `-mbranch-protection=pac-ret+bti` | Explicit PAC + BTI | Yes | Yes |

**Recommendation:** Always use `-mbranch-protection=standard` for ARM builds.

## Sources

- ARM Architecture Reference Manual (ARMv8-A)
- [ARM Learn: Pointer Authentication](https://developer.arm.com/documentation/102433/latest/)
- [ARM Learn: Branch Target Identification](https://developer.arm.com/documentation/102433/latest/)
- [ARM MTE Whitepaper](https://developer.arm.com/documentation/102433/latest/)
- [PACMAN: Attacking ARM Pointer Authentication (MIT, 2022)](https://pacmanattack.com/)
- [Google Android MTE Documentation](https://source.android.com/docs/security/test/memory-safety/arm-mte)
- [Linux kernel MTE documentation](https://docs.kernel.org/arch/arm64/memory-tagging-extension.html)
