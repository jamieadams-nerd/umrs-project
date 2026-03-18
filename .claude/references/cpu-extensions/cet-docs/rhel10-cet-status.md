# RHEL 10 CET Enablement Status

## Compiler Defaults

### GCC 14.2 (RHEL 10 default)
- `-fcf-protection=full` is **enabled by default** on x86_64 starting with GCC 14
- This means C/C++ binaries compiled on RHEL 10 get IBT + SHSTK ELF properties automatically
- The flag generates `ENDBR64` prologues for IBT and marks the binary as shadow-stack compatible

### Clang/LLVM
- Supports `-fcf-protection=full` but not enabled by default
- RHEL 10 packages built with GCC; Clang-compiled code needs explicit flag

## glibc (RHEL 10)
- Red Hat builds glibc with CET support (`--enable-cet`)
- Shadow stack activation is **opt-in at runtime** — not forced on all processes
- glibc checks binary ELF properties during startup; if CET-capable, activates shadow stack
- Runtime default: shadow stack enabled for CET-capable binaries, disabled for others

## Kernel
- RHEL 10 kernel ships with `CONFIG_X86_USER_SHADOW_STACK=y` and `CONFIG_X86_KERNEL_IBT=y`
- Kernel IBT is active on CET-capable hardware
- User-space shadow stack requires glibc activation + binary support

## Rust CET Status — CRITICAL FINDING

**Tracked at:** https://github.com/rust-lang/rust/issues/93754

- `-Z cf-protection=full` exists as an **unstable (nightly-only)** compiler flag
- The flag is NOT available on stable Rust
- Using it requires rebuilding the standard library (`-Z build-std`)
- Assembly code in the Rust standard library has **not been audited** for `ENDBR64` placement
- Consequence: **Rust binaries compiled on RHEL 10 lack SHSTK/IBT ELF notes by default**
- These binaries are incompatible with strict CET enforcement

### UMRS Impact

UMRS binaries (compiled with stable Rust) will NOT have CET support. This means:
- On a CET-capable RHEL 10 system, UMRS binaries run without shadow stack protection
- This is an **acceptable known limitation** — Rust's memory safety provides equivalent protection against most buffer-overflow-based ROP attacks
- However, it should be documented as a security posture gap for auditors
- Monitor rust-lang/rust#93754 for stabilization

### Audit Classification
- System with CET-capable CPU + RHEL 10 + Rust binaries without CET: **INFORMATIONAL** finding
- System with CET-capable CPU + RHEL 10 + C binaries without `-fcf-protection`: **HIGH** finding
- The distinction matters: Rust's memory safety model provides alternate CFI guarantees

## Summary Table

| Component | CET-SS (Shadow Stack) | CET-IBT |
|-----------|----------------------|---------|
| RHEL 10 kernel | Supported | Supported |
| GCC 14.2 | Default on | Default on |
| glibc 2.39 | Opt-in activation | N/A (compiler-only) |
| Clang | Manual flag needed | Manual flag needed |
| **Rust (stable)** | **NOT available** | **NOT available** |
| UMRS binaries | **No** | **No** |

## Sources

- [GCC 14 Release Series Changes](https://gcc.gnu.org/gcc-14/changes.html)
- [Rust CET Tracking Issue #93754](https://github.com/rust-lang/rust/issues/93754)
- [Rust CET Issue #73820](https://github.com/rust-lang/rust/issues/73820)
- [cf_protection — Rust Unstable Book](https://doc.rust-lang.org/beta/unstable-book/compiler-flags/cf-protection.html)
- [RHEL 10 Security Hardening](https://docs.redhat.com/en/documentation/red_hat_enterprise_linux/10/html-single/security_hardening/index)
- [RHEL 10 Considerations in Adopting](https://docs.redhat.com/en/documentation/red_hat_enterprise_linux/10/html-single/considerations_in_adopting_rhel_10/index)
