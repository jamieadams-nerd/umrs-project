---
name: SCAP/STIG Corpus Familiarization
description: Security-engineer findings from RHEL 10 STIG signal index and CCE-NIST cross-reference; deployment posture gaps, audit rule priorities, SELinux policy items, and UMRS install model implications
type: reference
---

# SCAP/STIG Corpus Familiarization — Security-Engineer Focus

**Source files:**
- `.claude/references/scap-security-guide/stig-signal-index.md` (451 signals)
- `.claude/references/scap-security-guide/cce-nist-crossref.md` (451 unique CCEs)
- `.claude/plans/scap-stig-corpus-plan.md` (plan context, researcher notes)

**Date familiarized:** 2026-03-17

---

## 1. RAG Limitation — Direct File Reads Required

The two index files are stored as single massive chunks in ChromaDB — the markdown
chunker cannot subdivide flat tables. Every RAG query returns the same two giant
chunks with no discrimination. For CCE/signal lookup, always read the files directly
with rg rather than relying on RAG semantic search for this collection.

---

## 2. SELinux-Specific STIG Signals

Only two signals directly address SELinux enforcement state:

| Signal | CCE | Severity | NIST | What it checks |
|---|---|---|---|---|
| `selinux_state` | CCE-89386-7 | HIGH | AC-3, AC-3(3)(a), AU-9, SC-7(21) | SELinux must be in enforcing mode |
| `selinux_policytype` | CCE-88366-0 | medium | AC-3, AC-3(3)(a), AU-9, SC-7(21) | Policy type set in /etc/selinux/config |
| `account_password_selinux_faillock_dir` | CCE-90568-7 | medium | AC-7(a) | SELinux context required for pam_faillock.so records dir |
| `accounts_passwords_pam_faillock_dir` | CCE-90182-7 | medium | AC-7(a), AC-7(b) | Lock accounts must persist — SELinux packages installed |

**UMRS coverage:** The `umrs-platform` posture catalog does not include an indicator for
`selinux_state` or `selinux_policytype`. These are handled by `umrs-selinux` (the `status.rs`
module reads `/sys/fs/selinux/enforce`), but there is no catalog entry with a CCE citation
and no contradiction detection between `/etc/selinux/config` (configured) and the live kernel
enforce value. This is a gap — STIG checks the config file only; UMRS could check both and
flag contradictions.

**Recommended action:** Add `SelinuxEnforce` and `SelinuxPolicyType` indicators to the posture
catalog with CCE citations. The live path for enforcement is `/sys/fs/selinux/enforce`; the
configured path is `/etc/selinux/config`. This is a Phase 3d candidate
(check methodology comparison documentation).

**UMRS policy note:** The `selinux_state` HIGH finding means any deployment checklist must
confirm the UMRS tool binaries are installed with correct SELinux types — if they run in
the wrong domain, `enforce` reads still succeed but the MAC audit trail is wrong.

---

## 3. Audit Rule Signals — Prioritized for CUI Systems

The STIG contains 51 `audit-rule` signals. Zero have UMRS coverage. Prioritized by
relevance to a CUI-handling system:

### Tier 1 — Direct UMRS relevance (xattr, SELinux management, kernel module loading)

| Signal | CCE | NIST | Why it matters for UMRS |
|---|---|---|---|
| `audit_rules_dac_modification_setxattr` | CCE-89571-4 | AU-12(c), AU-2(d) | UMRS reads `security.selinux` xattr — writes to this namespace must be audited |
| `audit_rules_dac_modification_fsetxattr` | CCE-89370-1 | AU-12(c), AU-2(d) | Same — fd-anchored xattr writes |
| `audit_rules_dac_modification_lsetxattr` | CCE-88052-6 | AU-12(c), AU-2(d) | Symlink-based xattr writes |
| `audit_rules_dac_modification_removexattr` | CCE-89677-9 | AU-12(c), AU-2(d) | xattr removal — label stripping |
| `audit_rules_dac_modification_fremovexattr` | CCE-88352-0 | AU-12(c), AU-2(d) | fd-anchored label stripping |
| `audit_rules_dac_modification_lremovexattr` | CCE-90100-9 | AU-12(c), AU-2(d) | Symlink-based label stripping |
| `audit_rules_execution_chcon` | CCE-87762-1 | AC-6(9), AU-12(c) | chcon changes SELinux labels — must be audited on CUI system |
| `audit_rules_execution_semanage` | CCE-89541-7 | AC-2(4), AC-6(9), AU-12(c) | semanage modifies policy — HIGH relevance |
| `audit_rules_execution_setsebool` | CCE-87741-5 | AC-6(9), AU-12(c) | setsebool changes runtime policy booleans |
| `audit_rules_execution_setfiles` | CCE-88818-0 | AC-6(9), AU-12(c) | setfiles relabels filesystem |
| `audit_rules_kernel_module_loading_init` | CCE-90172-8 | AC-6(9), AU-12(c) | init_module syscall — UMRS tracks module state |
| `audit_rules_kernel_module_loading_finit` | CCE-88638-2 | AC-6(9), AU-12(c) | finit_module — same |
| `audit_rules_kernel_module_loading_delete` | CCE-89982-3 | AC-6(9), AU-12(c) | delete_module — module unloading |
| `audit_rules_immutable` | CCE-89816-3 | AC-6(9), CM-6(a) | Audit rules must be immutable after boot (protection of audit trail) |

### Tier 2 — File integrity, DAC modification, privilege use

| Signal | CCE | NIST | Notes |
|---|---|---|---|
| `audit_rules_dac_modification_chmod` | CCE-90466-4 | AU-12(c), AU-2(d) | chmod on deployed UMRS files would be suspicious |
| `audit_rules_dac_modification_chown` | CCE-89540-9 | AU-12(c), AU-2(d) | chown changes DAC owner |
| `audit_rules_dac_modification_fchmod` | CCE-88200-1 | AU-12(c), AU-2(d) | fd-anchored chmod |
| `audit_rules_dac_modification_fchown` | CCE-90685-9 | AU-12(c), AU-2(d) | fd-anchored chown |
| `audit_rules_privileged_commands_sudo` | CCE-89698-5 | AC-6(9), AU-12(c) | sudo use must be audited |
| `audit_rules_privileged_commands_su` | CCE-89587-0 | AC-6(9), AU-12(c) | su use |
| `audit_rules_privileged_commands_kmod` | CCE-86727-5 | AU-12(a), AU-3, MA-4(1)(a) | kmod invocation |
| `audit_rules_privileged_commands_modprobe` | CCE-89893-2 | AU-12(a), AU-3, MA-4(1)(a) | modprobe invocation |
| `audit_rules_usergroup_modification_shadow` | CCE-88637-4 | AC-2(4), AC-6(9) | /etc/shadow modifications |
| `audit_rules_suid_privilege_function` | CCE-88933-7 | AC-6(9), AU-12(3), CM-5(1) | SUID function execution |

### Tier 3 — File deletion, login events, system administration

All remaining `audit_rules_file_deletion_events_*` and `audit_rules_login_events_*` signals
are standard system hygiene. Relevant for a CUI system but not uniquely tied to UMRS functionality.
Total: ~20 signals. All map to AU-12(c), AU-2(d), CM-6(a).

---

## 4. UMRS Deployment Model Implications (~/.local/bin, XDG paths)

The STIG's file-check signals cover system directories (`/etc/`, `/usr/`, `/var/log/`).
They do NOT cover user-local installs (`~/.local/bin/`, `~/.config/`, `~/.local/share/`).
This is a significant gap in STIG coverage for the initial UMRS deployment model.

**Concrete implications for security-engineer:**

### 4a. No STIG file-check coverage for ~/.local/bin

The STIG `file_ownership_binary_dirs` (CCE-89620-9, AC-6(1), CM-5(6)) and
`file_permissions_binary_dirs` (CCE-86978-4, AC-6(1), CM-5(6)) cover `/usr/bin/`, `/usr/sbin/`,
etc. — not user-local paths. UMRS binaries installed to `~/.local/bin/` would NOT be checked
by a standard STIG scan.

**Policy recommendation:** Until UMRS moves to native packages (RPM to `/usr/bin/`), the
installation procedure must explicitly require:
- `chmod 750 ~/.local/bin/umrs-*` (owner-execute only, no world-execute)
- `chown $USER:$USER ~/.local/bin/umrs-*`
- SELinux file context: these files will be labeled `user_home_t` or `user_home_bin_t`
  by default — NOT `bin_t`. This means they cannot be `execmod`'d by the `unconfined_t` domain
  without an explicit policy rule. The security posture is tighter than a system install
  for untrusted users, but may require `chcon` or a custom `.fc` entry to run correctly.

### 4b. XDG config paths (~/.config/umrs/) not covered

STIG `file_permissions_etc_*` checks are scoped to `/etc/`. CUI-sensitive configuration in
`~/.config/umrs/` has no STIG check. This is the operator's responsibility.

**Policy recommendation:** If UMRS reads any configuration from `~/.config/umrs/`, that path
must be documented in the deployment model with required permissions (mode 600, owner only).
No sensitive state (MLS labels, CUI markings, session tokens) should be persisted there until
a formal data classification policy is established for the XDG paths.

### 4c. fapolicyd and user-local binaries

`package_fapolicyd_installed` (CCE-89813-0, CM-6(a), SI-4(22)) is a medium-severity STIG item.
fapolicyd enforces application allowlisting. User-local binaries in `~/.local/bin/` require an
explicit fapolicyd rule to be executable. Without a fapolicyd allow rule, UMRS binaries will
be blocked silently on a STIG-hardened system with fapolicyd enforcing.

**This is a HIGH deployment risk.** The UMRS deployment documentation and RPM spec must
include a fapolicyd rule (or RPM trust database entry via `fapolicyd-cli --update`).

---

## 5. Deployment Hardening Gaps — Items UMRS Should Verify

These STIG signals have no UMRS coverage but apply to the host that runs UMRS:

| Signal | CCE | Severity | Gap |
|---|---|---|---|
| `configure_crypto_policy` | CCE-89085-5 | HIGH | UMRS assumes FIPS but does not verify the system-wide crypto policy is set to `FIPS` |
| `aide_use_fips_hashes` | CCE-90260-1 | medium | AIDE file integrity must use FIPS hashes — UMRS has no AIDE integration/awareness |
| `aide_check_audit_tools` | CCE-86441-3 | medium | AIDE must monitor audit tools — UMRS binary should be in AIDE database |
| `audit_rules_immutable` | CCE-89816-3 | medium | Audit rules must be immutable — UMRS depends on auditd functioning correctly |
| `grub2_audit_argument` | CCE-88376-9 | low | `audit=1` kernel cmdline — UMRS does not check this; no IndicatorId for it |
| `grub2_audit_backlog_limit_argument` | CCE-88192-0 | low | `audit_backlog_limit` — no coverage |
| `package_fapolicyd_installed` | CCE-89813-0 | medium | UMRS binaries need fapolicyd allowlist entry |
| `sysctl_kernel_core_pattern` | CCE-86714-3 | medium | Core dump storage — no UMRS indicator; maps to SC-7(10) |
| `sysctl_net_core_bpf_jit_harden` | CCE-89631-6 | medium | BPF JIT hardening — no UMRS indicator |
| `kernel_module_bluetooth_disabled` | CCE-87455-2 | medium | Covered by UMRS `BluetoothBlacklisted` indicator — CCE annotation MISSING |
| `kernel_module_usb-storage_disabled` | CCE-89301-6 | medium | Covered by UMRS `UsbStorageBlacklisted` — CCE annotation MISSING |

---

## 6. CCE Annotations Missing From Existing UMRS Indicators

UMRS indicators that already match a STIG signal but lack CCE citations in `catalog.rs`:

| IndicatorId | Matching Signal | CCE to add |
|---|---|---|
| `KptrRestrict` | `sysctl_kernel_kptr_restrict` | CCE-88686-1 |
| `RandomizeVaSpace` | `sysctl_kernel_randomize_va_space` | CCE-87876-9 |
| `UnprivBpfDisabled` | `sysctl_kernel_unprivileged_bpf_disabled` | CCE-89405-5 |
| `PerfEventParanoid` | `sysctl_kernel_perf_event_paranoid` | CCE-90142-1 |
| `YamaPtraceScope` | `sysctl_kernel_yama_ptrace_scope` | CCE-88785-1 |
| `DmesgRestrict` | `sysctl_kernel_dmesg_restrict` | CCE-89000-4 |
| `KexecLoadDisabled` | `sysctl_kernel_kexec_load_disabled` | CCE-89232-3 |
| `ProtectedSymlinks` | `sysctl_fs_protected_symlinks` | CCE-88796-8 |
| `ProtectedHardlinks` | `sysctl_fs_protected_hardlinks` | CCE-86689-7 |
| `BluetoothBlacklisted` | `kernel_module_bluetooth_disabled` | CCE-87455-2 |
| `UsbStorageBlacklisted` | `kernel_module_usb-storage_disabled` | CCE-89301-6 |
| `Pti` | `grub2_pti_argument` | CCE-88971-7 |
| `FipsEnabled` | (no direct STIG signal — FIPS is a cross-cutting env constraint) | N/A |

This CCE annotation work is Phase 3a in the scap-stig-corpus-plan; owner is rust-developer.
Security-engineer should review the `nist_controls` field format after rust-developer adds
the CCE field to `IndicatorDescriptor`.

---

## 7. SELinux Policy Authoring Takeaways

### 7a. pam_faillock.so SELinux context requirement (CCE-90568-7)

`account_password_selinux_faillock_dir` requires a specific SELinux type for the
pam_faillock records directory. If UMRS deployment adds a pam module or interacts with
PAM state, this context requirement must be satisfied. Current UMRS scope does not touch
PAM — but any future authentication-adjacent feature must include a file context for the
pam_faillock dir.

### 7b. chcon/semanage/setsebool audit coverage requirement

The STIG requires auditing of `chcon`, `semanage`, and `setsebool`. Any UMRS deployment
step that runs these tools (e.g., `restorecon -R` after install, `semodule -i` in RPM %post)
will generate auditd events. The RPM %post scriptlet should expect this and not suppress errors.

### 7c. AIDE must include UMRS binaries

`aide_check_audit_tools` (CCE-86441-3, AU-9(3)) requires AIDE to monitor audit tools.
`aide_periodic_cron_checking` (CCE-86738-2, SI-7) requires periodic AIDE runs.
`aide_use_fips_hashes` (CCE-90260-1, SI-7) requires FIPS 140-2 hashes in AIDE.

When UMRS moves to a system install path (`/usr/bin/umrs-*`), those binaries should be
included in the AIDE database. The AIDE configuration rule would be:
```
/usr/bin/umrs-ls       p+i+n+u+g+s+b+m+c+md5+sha512
/usr/bin/umrs-state    p+i+n+u+g+s+b+m+c+md5+sha512
```
(and sha512 must be in the FIPS hash set — AIDE built with OpenSSL provides this.)

### 7d. Audit rules immutability (-e 2)

`audit_rules_immutable` (CCE-89816-3, AC-6(9), CM-6(a)) requires `/etc/audit/rules.d/*.rules`
to end with `-e 2` (immutable mode). Any UMRS audit rule file placed in `/etc/audit/rules.d/`
must appear BEFORE the `-e 2` line. The RPM %post scriptlet must order rule installation
carefully and restart auditd before `-e 2` takes effect, or the rules will not load without
a reboot.

---

## 8. Signals STIG Cannot Catch That UMRS Can

This is the architectural differentiator documented in the plan (Phase 3d). Key examples:

- **Contradiction detection:** sysctl.d sets `kernel.kexec_load_disabled=1` but live
  `/proc/sys/kernel/kexec_load_disabled` reads `0` (because the sysctl was set before the
  kernel parameter became available, or an override was applied). STIG reads the config file
  and passes. UMRS reads both and flags the contradiction.

- **SELinux enforce contradiction:** `/etc/selinux/config` says `SELINUX=enforcing` but
  `/sys/fs/selinux/enforce` reads `0` (permissive, set at boot by `enforcing=0` on kernel
  cmdline). STIG `selinux_state` checks the live value and would catch this, but does not
  show the configured-vs-live delta. UMRS can surface both values with a contradiction finding.

- **Module loading:** STIG `audit_rules_kernel_module_loading_init` audits the `init_module`
  syscall. UMRS `ModulesDisabled` indicator checks whether the kernel one-way latch has been
  set — a stronger posture check (no modules can load at all, even with CAP_SYS_MODULE).

---

## 9. HIGH Severity Signals Affecting UMRS Deployment Host

| Signal | CCE | NIST | Action required |
|---|---|---|---|
| `selinux_state` | CCE-89386-7 | AC-3, SC-7(21) | Verify before deploying UMRS |
| `configure_crypto_policy` | CCE-89085-5 | SC-12(2), SC-13 | Verify system crypto policy = FIPS |
| `ensure_gpgcheck_globally_activated` | CCE-88404-9 | CM-5(3), SI-7 | UMRS RPM must be GPG-signed |
| `ensure_gpgcheck_never_disabled` | CCE-88176-3 | CM-5(3), SI-7 | Same — repo must have gpgcheck=1 |
| `ensure_redhat_gpgkey_installed` | CCE-88256-3 | CM-5(3), SI-7 | Install from signed repo only |

---

## 10. Cross-Agent Handoffs From This Familiarization

- **rust-developer (Phase 3a):** Add `cce: Option<&'static str>` to `IndicatorDescriptor`;
  annotate 13 indicators listed in section 6 above.

- **security-auditor (Phase 3c):** Coverage gap report. Reference section 5 above for
  the prioritized list of uncovered signals. Reference section 3 Tier 1 for the 14 audit
  rule signals that matter most for CUI.

- **tech-writer (Phase 3b):** CCE citation format established by plan: `CCE-89232-3 (NIST SP 800-53 CM-6)`.
  When documenting deployment prerequisites, cite CCEs for the HIGH signals in section 9.

- **security-engineer (self, Phase 3d):** Write the configured-vs-live methodology
  comparison for `docs/modules/architecture/`. Key differentiators are in section 8.
  Also: write fapolicyd deployment guidance and AIDE configuration for UMRS binaries.
