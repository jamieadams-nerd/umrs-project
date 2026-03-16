# High-Assurance Linux Capability Matrix — Display Domains

**Source:** `.claude/jamies_brain/kernel-probe-grouping.txt` (Jamie Adams)
**Purpose:** Defines the 7 operator-facing display domains for organizing kernel posture
signals in TUI tabs, reports, and `--json` output.

These domains answer **assurance questions**, not compliance questions:
- Is the kernel runtime mutable?
- Can root load arbitrary code?
- Can logs be tampered with?
- Can policies be bypassed?

---

## The 7 Domains

### 1. Kernel Enforcement Controls

**Purpose:** Kernel mechanisms that restrict privileged behavior.
**Assurance property:** Prevents runtime kernel modification; reduces attack surface for root compromise.

| Signal | Probe | Desired |
|--------|-------|---------|
| Kernel Lockdown Mode | `/sys/kernel/security/lockdown` | integrity or confidentiality |
| Disable Module Loading | `/proc/sys/kernel/modules_disabled` | 1 |
| Kexec Disabled | `/proc/sys/kernel/kexec_load_disabled` | 1 |
| BPF JIT Harden | `/proc/sys/net/core/bpf_jit_harden` | 2 |
| Unprivileged BPF Disabled | `/proc/sys/kernel/unprivileged_bpf_disabled` | 1 |

### 2. Boot Chain Integrity

**Purpose:** Ensures system trust begins at boot.
**Assurance property:** Prevents unsigned kernel execution; protects kernel memory integrity.

| Signal | Probe | Desired |
|--------|-------|---------|
| Secure Boot | `/sys/firmware/efi/efivars/SecureBoot-*` | enabled |
| Bootloader Verification | firmware policy | verified |
| Kernel Lockdown Auto Mode | kernel command line | enforced |

### 3. Cryptographic Enforcement

**Purpose:** Ensures approved cryptography is used.
**Assurance property:** FIPS-validated primitives in use; restricted crypto policy active.

| Signal | Probe | Desired |
|--------|-------|---------|
| FIPS Mode | `/proc/sys/crypto/fips_enabled` | 1 |
| Kernel Crypto Policy | `/etc/crypto-policies/state/current` | restricted |
| OpenSSL Provider Policy | `openssl.cnf` | restricted providers |

### 4. Mandatory Access Control

**Purpose:** Strong enforcement boundaries.
**Assurance property:** Non-bypassable access control; domain isolation.

| Signal | Probe | Desired |
|--------|-------|---------|
| SELinux Mode | `/sys/fs/selinux/enforce` | 1 |
| SELinux Policy | `/etc/selinux/config` | targeted/MLS |
| SELinux Boolean Integrity | selinuxfs booleans | locked |

### 5. Kernel Integrity Monitoring

**Purpose:** Detects runtime tampering.
**Assurance property:** File integrity validation; runtime tamper detection.

| Signal | Probe | Desired |
|--------|-------|---------|
| IMA Enabled | `/sys/kernel/security/ima` | active |
| EVM Enabled | `/sys/kernel/security/evm` | active |
| IMA Policy | `/sys/kernel/security/ima/policy` | defined |

### 6. Logging and Forensics Assurance

**Purpose:** Ensures evidence integrity.
**Assurance property:** Tamper-resistant logging; forensic traceability.

| Signal | Probe | Desired |
|--------|-------|---------|
| Audit Enabled | `/proc/sys/kernel/audit` | 1 |
| Audit Immutable | `/proc/sys/kernel/audit_immutable` | 1 |
| Journald Persistent | `journald.conf` | persistent |
| Log Forwarding | journald/audit config | remote |

### 7. Memory and Exploit Mitigation

**Purpose:** Protects runtime memory.
**Assurance property:** Exploit resistance; memory isolation.

| Signal | Probe | Desired |
|--------|-------|---------|
| ASLR | `/proc/sys/kernel/randomize_va_space` | 2 |
| SMEP/SMAP | `/proc/cpuinfo` flags | present |
| KASLR | kernel cmdline | enabled |

---

## Mapping to Existing Catalog Groups

| Capability Matrix Domain | Current `catalog.rs` Group(s) |
|--------------------------|-------------------------------|
| 1. Kernel Enforcement Controls | Kernel Self-Protection, modprobe.d |
| 2. Boot Chain Integrity | Boot-time / kernel cmdline |
| 3. Cryptographic Enforcement | Special (FIPS) |
| 4. Mandatory Access Control | (partially in Boot-time, SELinux probes) |
| 5. Kernel Integrity Monitoring | Kernel Integrity |
| 6. Logging and Forensics Assurance | (not yet in catalog — future signals) |
| 7. Memory and Exploit Mitigation | Process Isolation, CPU mitigation sub-signals |

---

## Assurance Scoring Weights

| Level | Meaning |
|-------|---------|
| Critical | System trust boundary |
| Strong | Major integrity mechanism |
| Moderate | Defense-in-depth |
| Informational | Posture visibility |

---

## Implementation Notes

- **TUI:** Each domain becomes a display group within the Kernel tab (or its own sub-tab)
- **Reports:** Each domain becomes a section in `--json` and printable report output
- **Scoring:** Domains feed into the overall High Assurance Score calculation
- The domain names and human descriptions are operator-approved — use them verbatim
- The `SignalDescriptor` struct may need a `domain: CapabilityDomain` field to enable
  grouping at the type level (rust-developer decision)

## Integration With Existing Architecture

```
kernel state → probe layer → capability matrix → evaluation engine
                                ↑
                    Jamie's 7 display domains
                    organize the output here
```

The capability matrix sits between the raw posture signals and the evaluation/display layer.
It provides the human-readable grouping that operators and auditors actually think in.
