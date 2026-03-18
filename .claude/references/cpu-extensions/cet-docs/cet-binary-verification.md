# CET Binary Verification Guide

## Purpose

This guide documents how to verify whether a binary has CET (Control-flow Enforcement
Technology) support compiled in, and how to check runtime CET status for running processes.
This is the "Layer 2" software utilization check for the CET CPU feature.

## Static Binary Verification

### Method 1: `readelf -n` (GNU binutils)

```bash
readelf -n /usr/bin/example
```

Look for:
```
Displaying notes found in: .note.gnu.property
  Owner                Data size    Description
  GNU                  0x00000030   NT_GNU_PROPERTY_TYPE_0
    Properties: x86 feature: IBT, SHSTK
```

- `IBT` = Indirect Branch Tracking enabled
- `SHSTK` = Shadow Stack enabled
- Both should be present for full CET support

### Method 2: `eu-readelf -n` (elfutils)

```bash
eu-readelf -n /usr/bin/example
```

Equivalent output. RHEL 10 ships both tools.

### Method 3: `checksec` (pwntools)

```bash
checksec --file=/usr/bin/example
```

Shows CET status in the security feature summary.

## ELF Property Details

The CET capability is recorded in the `.note.gnu.property` ELF section:

| Property | Value | Meaning |
|----------|-------|---------|
| `GNU_PROPERTY_X86_FEATURE_1_AND` | bit 0 set | IBT supported |
| `GNU_PROPERTY_X86_FEATURE_1_AND` | bit 1 set | SHSTK supported |
| `GNU_PROPERTY_X86_FEATURE_1_AND` | bits 0+1 set | Full CET support |

The `_AND` suffix means: when the dynamic linker loads shared libraries, it ANDs all
properties together. If ANY shared library in the dependency chain lacks CET support,
the entire process runs without CET.

**This is critical:** A single non-CET shared library disables CET for the whole process.

## Runtime Verification

### Method: `/proc/<pid>/status`

```bash
grep -i thread_features /proc/<pid>/status
```

Output when shadow stack is active:
```
x86_Thread_features:	shstk
x86_Thread_features_locked:	shstk
```

No output (or missing field) = shadow stack not active for this process.

### Method: `arch_prctl` from within process

```c
unsigned long features;
arch_prctl(ARCH_SHSTK_STATUS, &features);
// features & 1 == shadow stack enabled
```

## Audit Procedure

### System-Wide CET Audit

1. **Check CPU support:**
   ```bash
   grep -o 'shstk\|ibt' /proc/cpuinfo | sort -u
   ```
   Expected: both `shstk` and `ibt` present

2. **Check kernel support:**
   ```bash
   grep CONFIG_X86_USER_SHADOW_STACK /boot/config-$(uname -r)
   grep CONFIG_X86_KERNEL_IBT /boot/config-$(uname -r)
   ```
   Expected: both `=y`

3. **Audit critical binaries:**
   ```bash
   for bin in /usr/sbin/sshd /usr/bin/sudo /usr/sbin/httpd /usr/bin/passwd; do
     echo "=== $bin ==="
     readelf -n "$bin" 2>/dev/null | grep -A1 "x86 feature"
   done
   ```

4. **Audit shared libraries in dependency chain:**
   ```bash
   ldd /usr/sbin/sshd | awk '{print $3}' | while read lib; do
     result=$(readelf -n "$lib" 2>/dev/null | grep "x86 feature")
     if [ -z "$result" ]; then
       echo "NO CET: $lib"
     fi
   done
   ```

## Audit Classification

| Condition | Classification | Rationale |
|-----------|---------------|-----------|
| CET CPU + C binary without `-fcf-protection` | **HIGH** | Capability present but unused; CFI gap |
| CET CPU + Rust binary (stable compiler) | **INFORMATIONAL** | Rust memory safety provides alternate CFI; CET not yet available in stable Rust |
| CET CPU + mixed C/Rust process | **MEDIUM** | C components may lack CET even if primary binary is CET-capable |
| Non-CET library in dependency chain | **MEDIUM** | Single non-CET .so disables CET for entire process |
| CET CPU + all binaries CET-capable | **PASS** | Full hardware CFI active |

## Rust-Specific Guidance

Rust binaries on RHEL 10 will NOT have CET ELF properties because:
- `-Z cf-protection=full` is unstable (nightly only)
- Stable Rust does not support CET
- Track: https://github.com/rust-lang/rust/issues/93754

When auditing UMRS binaries, classify CET absence as INFORMATIONAL, not HIGH:
- Rust's ownership model + borrow checker prevents buffer overflows that enable ROP
- `#![forbid(unsafe_code)]` in UMRS crates eliminates most unsafe memory operations
- CET would be defense-in-depth, not primary protection

## Sources

- [Intel CET Specification 334525-003](https://kib.kiev.ua/x86docs/Intel/CET/334525-003.pdf)
- [CET Shadow Stack — Linux Kernel Documentation](https://docs.kernel.org/arch/x86/shstk.html)
- [GCC 14 Release Changes](https://gcc.gnu.org/gcc-14/changes.html)
- [Rust CET Tracking Issue #93754](https://github.com/rust-lang/rust/issues/93754)
- [CET Shadow Stack in checksec (pwntools #2288)](https://github.com/Gallopsled/pwntools/issues/2288)
- [LPC 2020: Enable Intel CET in Linux](https://lpc.events/event/7/contributions/729/attachments/496/903/CET-LPC-2020.pdf)
