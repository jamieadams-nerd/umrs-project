# Linux Kernel CET Support

**Local reference:** `.claude/references/kernel-docs/arch/x86/shstk.rst`

## Kernel Configuration

| Option | Purpose | Since |
|--------|---------|-------|
| CONFIG_X86_USER_SHADOW_STACK | User-space shadow stack support | Linux 6.6+ |
| CONFIG_X86_KERNEL_IBT | Kernel-mode Indirect Branch Tracking | Linux 5.18+ |

## User-Space Shadow Stack

### arch_prctl Interface

| Operation | Purpose |
|-----------|---------|
| ARCH_SHSTK_ENABLE | Enable shadow stack for calling thread |
| ARCH_SHSTK_DISABLE | Disable shadow stack |
| ARCH_SHSTK_LOCK | Lock shadow stack (cannot be disabled after this) |
| ARCH_SHSTK_STATUS | Query current shadow stack status |
| ARCH_SHSTK_UNLOCK | Unlock (only if not locked) |

### /proc/pid/status Detection

When shadow stack is active for a process:

    x86_Thread_features:    shstk
    x86_Thread_features_locked:     shstk

Fields appear in /proc/pid/status when shadow stack is active. The _locked variant
indicates the feature has been locked and cannot be disabled.

### Signal Handling

- Shadow stack is unwound during signal delivery
- Signal return (sigreturn) validates shadow stack token
- SA_SHADOW_STACK_AWARE flag for signal handlers that understand CET

### Fork / Exec Semantics

- fork: child inherits shadow stack state
- exec: shadow stack is reset; new program gets fresh shadow stack if:
  1. Binary is marked CET-capable (ELF property)
  2. glibc activates shadow stack during startup
  3. Kernel has CONFIG_X86_USER_SHADOW_STACK=y

### glibc 2.39+ Shadow Stack Activation

- glibc 2.39 added shadow stack support
- Activation is opt-in: glibc enables it during startup if the binary is marked CET-capable
- --enable-cet configure option required for glibc build (Red Hat builds with CET support)
- Shadow stack is NOT forced on all processes, only on CET-capable binaries

## Kernel IBT

- CONFIG_X86_KERNEL_IBT (Linux 5.18+)
- Kernel itself compiled with -fcf-protection=branch
- All kernel indirect branch targets must have ENDBR64
- Kernel modules must also be compiled with IBT support
- Disabled automatically if CPU does not support IBT

## Sources

- CET Shadow Stack, Linux Kernel Documentation: https://docs.kernel.org/arch/x86/shstk.html
- Kernel support for control-flow enforcement, LWN.net: https://lwn.net/Articles/758245/
- GNU C Library 2.39, LWN.net: https://lwn.net/Articles/960309/
