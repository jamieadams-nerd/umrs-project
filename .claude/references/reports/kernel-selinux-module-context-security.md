# Kernel Module Loading Controls & SELinux Context Change Security

**Report date:** 2026-03-10
**Prepared by:** UMRS Researcher Agent
**Status:** Final
**Relevance:** `umrs-selinux`, `umrs-platform`, `umrs-logspace`, `AC-3`, `AC-4`, `SI-7`, `CM-6`, `CM-7`, `SC-28`

---

## Sources Consulted

| Source | Collection | Key Files |
|---|---|---|
| SELinux Notebook 2024-04-30 | `selinux-notebook` | `object_classes_permissions.md`, `domain_object_transitions.md`, `mls_mcs.md`, `constraint_statements.md`, `mls_statements.md`, `libselinux_functions.md`, `modes.md` |
| Linux Kernel Docs | `kernel-docs` | `admin-guide/sysctl/kernel.rst`, `admin-guide/module-signing.rst`, `admin-guide/LSM/LoadPin.rst`, `admin-guide/LSM/ipe.rst`, `security/self-protection.rst` |

---

## Part 1 — SELinux Security Context Changes and MLS

### 1.1 Two Mechanisms for Changing a Process Context

A running process can acquire a new SELinux security context in exactly two ways.
They differ fundamentally in how they are triggered, what policy permissions are
required, and what audit trail they produce.

#### Exec-Time Transition (Standard Path)

A process does not change its own context directly. It calls `exec(2)` on a binary
and the kernel — via the SELinux LSM hook at `do_execve()` — assigns the new domain
based on policy. Three `allow` rules must all be present in policy for the transition
to succeed:

```
# 1. The source domain may transition to the target domain
allow source_t target_t : process transition;

# 2. The source domain may execute the entrypoint file
allow source_t entrypoint_exec_t : file execute;

# 3. The file type is a valid entrypoint for the target domain
allow target_t entrypoint_exec_t : file entrypoint;
```

The `runcon(1)` command uses `libselinux` functions (`getfilecon(3)`, `setexeccon(3)`)
to stage the desired context before calling `exec(2)`. The kernel then enforces all
three rules plus any applicable MLS constraints (see §1.3) before committing the
transition.

#### Dynamic Context Change (setcon — Restricted Path)

A process can change its own running context *without* calling `exec(2)` by using the
`libselinux` function `setcon(3)`. This path is intentionally difficult to reach:

- Policy must grant `allow source_t target_t : process dyntransition;`
- The kernel checks `process:setcurrent` at the `security_setprocattr` hook
- MLS `mlsvalidatetrans` constraints must pass on the transition triple
- The new context's MLS level must be within the process's **clearance**
  (the high bound of its current MLS range)

`setcon` bypasses the exec-time audit trail and is therefore restricted in all
well-formed confined policies. Most confined domains do not hold `process:setcurrent`.

### 1.2 The `process` Object Class Permissions Involved

The following permissions from the `process` (and `process2`) object class govern
context transitions directly:

| Permission | What it gates |
|---|---|
| `transition` | Exec-time domain transition via `execve(2)` |
| `dyntransition` | Dynamic context change without exec (the `setcon` path) |
| `setcurrent` | Kernel-level permission to call `security_setprocattr` to change own label |
| `siginh` | Inherit signal state across a transition |
| `rlimitinh` | Inherit resource limits across a transition |
| `noatsecure` | Suppress `AT_SECURE` in the `execve` auxiliary vector (governs `LD_PRELOAD` behavior) |
| `nnp_transition` | Allow transition under `no_new_privs` (`process2` class) |
| `nosuid_transition` | Allow transition on nosuid mounts (`process2` class) |

### 1.3 MLS Constraints on Context Changes

MLS enforcement adds three additional layers on top of TE (Type Enforcement) rules.
All three must pass before a transition is allowed on an MLS system.

#### `mlsconstrain` — Bell-LaPadula Dominance at Every Access

The `mlsconstrain` statement encodes the Bell-LaPadula model in policy. The notation
uses:

- `l1`, `h1` — the process's low (current) and high (clearance) MLS level
- `l2` — the object's (or target process's) MLS level
- `dom` — dominates: sensitivity ≥ AND categories ⊇
- `domby` — dominated-by: sensitivity ≤ AND categories ⊆

Standard read and write constraints on the `process transition` permission:

```
# Read-down: process can read objects it dominates (l1 dom l2)
mlsconstrain process transition ( l1 dom l2 );

# Write-up: process can write to objects that dominate it (l1 domby l2)
mlsconstrain process transition ( l1 domby l2 );
```

**No write-down:** A process at `s2` cannot write to an object at `s1` — that would
be a declassification. The constraint requires the object's level to dominate the
process's level for writes.

**No read-up:** A process at `s1` cannot read an object at `s2` — the process must
dominate the object for reads.

**Category sets matter independently:** A process at `s2:c1` cannot read `s2:c3` even
though the sensitivity levels are equal, because `{c1}` does not dominate `{c3}`.
Both the sensitivity *and* the category set must satisfy the dominance relation.

The `mlsfilereadtoclr` attribute creates a controlled exception allowing processes to
read up to their clearance (`h1 dom l2`) rather than only their current level.

#### `mlsvalidatetrans` — Label Change Validation

Applied specifically during label changes on objects, including process context
transitions. It validates the three-way transition triple
`(old_label, new_label, subject_label)`. A process at `s0` cannot transition to `s3`
without an explicit `mlsvalidatetrans` rule permitting that combination.

#### `range_transition` — MLS Range Assignment at Exec Time

```
# anaconda_t may be assigned any range from systemLow to systemHigh
range_transition anaconda_t init_script_file_type:process s0-s15:c0.c255;
```

Without a matching `range_transition`, the child inherits the parent's MLS range.
A process can only be assigned a range within its SELinux user's declared **clearance**
— the high bound of the `[low-high]` range declared in the user statement.

#### The Tranquility Principle

A process cannot silently upgrade its own sensitivity level. For a dynamic `setcon`
transition to a higher level to succeed, *all* of the following must hold:

1. The domain holds `process:dyntransition` and `process:setcurrent`
2. The `mlsvalidatetrans` rule for that `(old, new, subject)` triple exists in policy
3. The new level falls within the process's declared clearance

This is the SELinux expression of the Bell-LaPadula tranquility principle.

### 1.4 UMRS Codebase Implications

| Concern | Where it applies |
|---|---|
| `MlsRange` must model both `low` and `high` ends | `umrs-selinux/src/mls/range.rs` |
| `MlsLevel::dominates()` is the computational form of `mlsconstrain` logic | `umrs-selinux/src/mls/level.rs` |
| A range where `low` does not `domby high` is an invalid context — fail closed | `SecurityContext` TPI parsing, test in `mls_level_tests.rs` |
| `setcon` (dyntransition) AVC records are high-value audit events | `umrs-logspace` — treat as `ObservationKind::Risk` |
| `SecurityContext::parse()` encountering an impossible range is a security event, not just an error | Error information discipline: log to audit, not to user-visible output |

---

## Part 2 — Preventing Kernel Module Loading at Runtime

### Overview: Defense-in-Depth Stack

There are five independent, complementary mechanisms for controlling module loading.
On a hardened RHEL 10 deployment all of them should be active simultaneously — each
closes a different attack path and no single one is sufficient alone.

```
┌─────────────────────────────────────────────────────────────┐
│  SELinux  system:module_request  (policy-level autoload gate) │
├─────────────────────────────────────────────────────────────┤
│  IPE LSM  op=KMODULE_LOAD policy  (declarative per-op rules) │
├─────────────────────────────────────────────────────────────┤
│  LoadPin LSM  (pin to single dm-verity-backed filesystem)    │
├─────────────────────────────────────────────────────────────┤
│  Kernel Lockdown LSM  integrity mode  (key-trust enforcement) │
├─────────────────────────────────────────────────────────────┤
│  MODULE_SIG_FORCE  (cryptographic signature at load time)    │
├─────────────────────────────────────────────────────────────┤
│  modules_disabled sysctl  (one-way latch — blocks all loads)  │
└─────────────────────────────────────────────────────────────┘
```

### 2.1 `modules_disabled` Sysctl — One-Way Latch

**Source:** `admin-guide/sysctl/kernel.rst`, chunk 29

```bash
# Lock module loading permanently for this boot session
echo 1 > /proc/sys/kernel/modules_disabled
```

This is a **one-way latch**. Once written to `1`, module loading is completely
disabled for the remainder of the boot — `init_module(2)` and `finit_module(2)`
are both blocked. The value cannot be reset to `0` without a reboot.

**Properties:**

- Affects both explicit loads (`insmod`, `modprobe`) and kernel-initiated autoloads
- Does not unload modules already in memory
- Is not persistent across reboots — must be set in an early systemd unit or initrd
  *after* all required modules have been loaded
- Pairs with `kexec_load_disabled` (same one-way latch semantics for `kexec`)

**Companion sysctl — disabling autoloading only:**

```bash
# Suppress the usermode helper entirely (autoloading only)
sysctl kernel.modprobe=""
```

Setting `kernel.modprobe` to the empty string prevents the kernel from invoking
any usermode helper for autoloads and suppresses the `kernel_module_request` LSM
hook entirely. Explicit `insmod` calls are unaffected.

If `CONFIG_STATIC_USERMODEHELPER=y` is set in the kernel build, the configured
static helper is used instead, providing a controlled allowlist.

**UMRS relevance:** The `ModuleLoadLatch` type in `umrs-platform::kattrs` reads
`/proc/sys/kernel/modules_disabled`. The one-way latch semantics are correctly
modeled: reading `0` on a production MLS node is a **security finding** and should
be surfaced as `ObservationKind::Risk`. Reading `1` is `ObservationKind::Good`.

### 2.2 Module Signing — `CONFIG_MODULE_SIG_FORCE`

**Source:** `admin-guide/module-signing.rst`, chunks 1, 4, 5, 6, 7

The kernel module signing facility cryptographically signs `.ko` files at
build/install time and verifies the signature at load time.

**Relevant kernel config options:**

| Config | Effect |
|---|---|
| `CONFIG_MODULE_SIG=y` | Enable signing infrastructure |
| `CONFIG_MODULE_SIG_FORCE=y` | **Reject all unsigned or invalidly-signed modules** |
| `CONFIG_MODULE_SIG_ALL=y` | Sign all modules during `make modules_install` |
| `CONFIG_SYSTEM_TRUSTED_KEYS` | PEM file of trusted CA certificates baked into the kernel image |
| `CONFIG_SECONDARY_TRUSTED_KEYRING=y` | Allow runtime key injection (restricted by Lockdown) |

**Signing algorithms supported:** RSA and NIST P-384 ECDSA.
**Permitted hash algorithms:** SHA-2 and SHA-3 families (SHA-256, SHA-384, SHA-512, SHA3-256, etc.)

```bash
# Manually sign a module
scripts/sign-file sha512 kernel-signkey.priv kernel-signkey.x509 module.ko
```

**Critical notes from the documentation:**

1. A signature appended to a `.ko` file confirms only that a signature *is present*,
   not that it is *valid*. The kernel performs the actual cryptographic check against
   `builtin_trusted_keys` at load time.
2. Signed modules **must never be stripped** after signing — the signature is appended
   outside the ELF container and stripping destroys it.
3. Runtime key injection via `keyctl padd asymmetric` is blocked in Lockdown mode —
   on a Secure Boot / Lockdown system the key set is frozen at boot.

**FIPS interaction:** For FIPS 140-2/3 compliance, signing keys must use
FIPS-approved algorithms. NIST P-384 ECDSA and RSA-3072+ are acceptable.
SHA-1 is not acceptable for new signatures in a FIPS context.

### 2.3 Kernel Lockdown LSM

**Source:** `admin-guide/LSM/index.rst`, `security/self-protection.rst`

`CONFIG_SECURITY_LOCKDOWN_LSM=y` adds a two-level LSM that restricts kernel
modification paths.

| Mode | What it restricts |
|---|---|
| `integrity` | Blocks modules from non-trusted keys, raw `/dev/mem` writes, unsigned kexec images, `/proc/kcore` reads, BPF JIT spraying, and other kernel-modification paths |
| `confidentiality` | All of `integrity` plus: blocks extraction of kernel secrets, hibernation (which writes RAM to disk), and access to kernel memory via various interfaces |

On UEFI Secure Boot systems, Lockdown automatically activates in `integrity` mode
when the firmware verifies the bootloader chain. This is the out-of-the-box state
on a RHEL 10 system booted with Secure Boot enabled.

**Setting explicitly:**
```bash
# Kernel command line
lockdown=integrity
# or
lockdown=confidentiality
```

Lockdown mode cannot be lowered at runtime once set.

### 2.4 LoadPin LSM

**Source:** `admin-guide/LSM/LoadPin.rst`, score 0.84

```
CONFIG_SECURITY_LOADPIN=y
CONFIG_SECURITY_LOADPIN_ENFORCE=y
```

LoadPin ensures that all kernel-loaded files — modules, firmware — originate from
the **same filesystem**, identified at the time of the first load. The expectation
is that this filesystem is backed by a read-only, integrity-verified device:

- `dm-verity` — block-level integrity verification with a cryptographic root hash
- CDROM / read-only media

Once pinned, any attempt to load a module from a *different* filesystem is denied.
Combined with a `dm-verity` root device, this creates a chain:

```
dm-verity root hash (measured at boot)
    → filesystem is cryptographically immutable
        → LoadPin pins to that filesystem
            → all module loads implicitly integrity-verified
```

### 2.5 IPE (Integrity Policy Enforcement) LSM

**Source:** `admin-guide/LSM/ipe.rst`, score 0.90 (highest in session)

IPE (`CONFIG_SECURITY_IPE=y`) allows expressing explicit, declarative rules for
kernel operations including module loading. It uses *immutable properties* —
properties that are set at device/file construction time and cannot be spoofed.

**Policy examples:**

```
# Deny all module loads by default
op=KMODULE_LOAD action=DENY

# Allow modules from a specific dm-verity volume (by root hash)
op=KMODULE_LOAD dmverity_roothash=<sha256-hash> action=ALLOW

# Allow modules from any dm-verity volume with a valid signed roothash
op=KMODULE_LOAD dmverity_signature=TRUE action=ALLOW

# Allow modules from fs-verity protected files with a valid builtin signature
op=KMODULE_LOAD fsverity_signature=TRUE action=ALLOW
```

IPE is the most expressive of the five mechanisms — it can express allow/deny by
cryptographic identity of the source volume or file, not just by filesystem or
key trust. Policies can be updated at runtime (unlike `modules_disabled` or
Lockdown). IPE integrates with `dm-verity` and `fs-verity`; the `fsverity_signature`
property requires `CONFIG_SECURITY_IPE`, `CONFIG_FS_VERITY`, and
`CONFIG_FS_VERITY_BUILTIN_SIGNATURES` all enabled.

### 2.6 SELinux — `system:module_request`

**Source:** `selinux-notebook/object_classes_permissions.md`, chunk 70 (`system` class)

When the kernel needs to autoload a module (e.g., for a network protocol not yet
loaded, or a filesystem type), it invokes the `kernel_module_request` LSM hook.
SELinux checks whether the requesting domain holds the `module_request` permission
on the `system` object class:

```
# Grant autoloading to the kernel domain only
allow kernel_t self:system module_request;

# All other domains: no grant → AVC denial → autoload blocked
```

This targets the *autoloading* path only — explicit `insmod`/`modprobe` by a
sufficiently privileged domain is a separate check governed by file execute
permissions on the `.ko` file and `init_module` syscall access. On a well-confined
MLS policy, the set of domains that can trigger autoloading should be minimal and
explicitly enumerated.

AVC denials on `system:module_request` are **high-signal audit events** — a confined
process attempting to trigger an autoload it was not expected to need warrants
investigation.

### 2.7 Interaction Matrix for RHEL 10 / UMRS Deployments

| Control | What it stops | Reversible at runtime? | Persistent across reboots? |
|---|---|---|---|
| `modules_disabled=1` sysctl | All module loads (explicit + auto) | No — reboot required | No — set in init/systemd |
| `kernel.modprobe=""` | Autoloading via usermode helper only | Yes | Via `/etc/sysctl.conf` |
| `MODULE_SIG_FORCE=y` | Unsigned / tampered `.ko` files | No — compile-time | Yes — kernel build |
| Lockdown `integrity` | Unsigned kexec + untrusted-key modules | No — per-boot or Secure Boot | Via firmware / cmdline |
| LoadPin + dm-verity | Modules not from the pinned verified filesystem | No — first load pins it | Yes — device-level |
| IPE policy | Declarative per-operation deny/allow | Yes — policy can be updated | Yes — policy persisted |
| SELinux `system:module_request` | Autoload requests from confined domains | Yes — policy update | Yes — policy persisted |

### 2.8 UMRS Codebase Implications

**`umrs-platform::kattrs` — `ModuleLoadLatch`**

This type reads `/proc/sys/kernel/modules_disabled` via the `ProcfsText` secure
reader path (provenance-verified, fd-anchored). The one-way latch semantics are
correct. On a production MLS node:

- `modules_disabled == 0` → **finding** → `ObservationKind::Risk`
- `modules_disabled == 1` → **good posture** → `ObservationKind::Good`

The same pattern applies to `kexec_load_disabled` — both should be checked and
reported together in any posture summary.

**`umrs-logspace` — AVC audit monitoring**

`system:module_request` AVC denials should be classified as `ObservationKind::Risk`.
These represent an unexpected autoload attempt from a confined domain and warrant
alerting, not just logging.

**FIPS mode interaction**

`/proc/sys/crypto/fips_enabled` (read via `ProcFips` in `umrs-platform::kattrs`)
and `modules_disabled` should be checked together. A FIPS-mode system that can still
load arbitrary modules has a gap: a rogue module could introduce non-validated
cryptographic primitives. The correct posture is:

```
fips_enabled == 1  AND  modules_disabled == 1
```

If `fips_enabled == 1` but `modules_disabled == 0`, the FIPS posture is weaker than
it could be — surface this as a correlated finding.

**Module signing key algorithms**

Any UMRS tooling that inspects or validates module signing (e.g., for audit trail
purposes) must verify that the signing algorithm is FIPS-approved: NIST P-384 ECDSA
or RSA-3072+. SHA-1 signatures are not acceptable in a FIPS 140-3 context.

---

## Summary of Key Findings

| # | Finding | NIST Control | Severity |
|---|---|---|---|
| 1 | `process:setcurrent` + `dyntransition` grants are a privileged capability — any domain holding both can silently change its own label | AC-3, AC-6 | High |
| 2 | MLS `mlsvalidatetrans` is the tranquility gate — without it, a process could self-upgrade its sensitivity level | AC-4, SC-28 | Critical |
| 3 | `modules_disabled=0` on a production MLS node is an open attack surface — arbitrary modules can expand kernel capabilities | CM-7, SI-7 | High |
| 4 | `MODULE_SIG_FORCE` without FIPS-approved hash and key algorithms weakens the integrity chain | SI-7, SC-13 | Medium |
| 5 | SELinux `system:module_request` denials are high-value audit signals — should trigger alert, not just log entry | AU-12, SI-4 | Medium |
| 6 | `fips_enabled=1` + `modules_disabled=0` is a correlated gap — FIPS posture is incomplete | SC-13, CM-6 | High |
| 7 | IPE LSM provides the strongest declarative control over module loading by cryptographic identity | CM-7, SI-7 | Recommended |

---

*Report ends. Next review recommended when kernel-docs ingest completes or when
RHEL 10 releases an updated kernel documentation set.*
