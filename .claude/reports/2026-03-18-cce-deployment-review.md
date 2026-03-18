# CCE Deployment and Policy Review — Addendum

```
Audit date: 2026-03-18
Depth: surface (supplementary to security-auditor CCE source code report)
Scope: components/platforms/rhel10/ policy files, UMRS deployment model,
       audit rule composite indicator design, fapolicyd deployment risk
```

This report is a targeted addendum from the deployment/policy perspective. It covers
what the source code CCE audit cannot see: installation procedures, SELinux policy
artifacts, audit rule deployment mechanics, and host-level enforcement dependencies.

---

## Section 1 — CCE Identifiers Relevant to Deployment That Source Audit Misses

The source code audit traces CCE identifiers into `catalog.rs` and `IndicatorDescriptor`.
The following CCEs affect the deployment procedure itself, not indicator definitions,
and are invisible to that audit.

### 1a. SELinux Enforcement Posture at Install Time

| CCE | Signal | Severity | Deployment Implication |
|---|---|---|---|
| `CCE-89386-7` | `selinux_state` | HIGH | Deployment must verify SELinux is in enforcing mode before running `semodule -i`. Installing policy modules on a permissive system produces a false-success; modules load but enforcement is not active. The RPM `%pre` scriptlet must gate on `getenforce`. |
| `CCE-88366-0` | `selinux_policytype` | medium | RPM install must confirm `targeted` (or MLS) policy is loaded, not `disabled` or `minimum`. A `semodule -i` call against a minimal policy tree may silently succeed with incomplete type resolution. |

**Deployment doc gap:** Neither the current `registry.txt` notes nor any installation procedure in the policy directory checks the enforce state before running `semodule`. This is not caught by source code review.

### 1b. Crypto Policy Must Be Verified Before FIPS-Dependent Operations

| CCE | Signal | Severity | Deployment Implication |
|---|---|---|---|
| `CCE-89085-5` | `configure_crypto_policy` | HIGH | UMRS binaries consume `ProcFips` state and assume the system crypto policy is set to `FIPS`. The deployment procedure must explicitly verify `update-crypto-policies --show` returns `FIPS` before declaring the system ready. A system can have FIPS-mode Rust binaries running under a `DEFAULT` crypto policy and the binaries will silently operate without FIPS-validated primitives at the openssl/kernel layer. |

### 1c. RPM Package Signing Requirements

| CCE | Signal | Severity | Deployment Implication |
|---|---|---|---|
| `CCE-88404-9` | `ensure_gpgcheck_globally_activated` | HIGH | UMRS RPM must be GPG-signed and delivered from a repo with `gpgcheck=1`. An unsigned RPM on a STIG-hardened host will be rejected by `dnf`. The RPM spec must include `%{gpg_sign}` in the build pipeline. |
| `CCE-88176-3` | `ensure_gpgcheck_never_disabled` | HIGH | Even if individual repo gpgcheck is managed by policy, UMRS must not be distributed via a channel that requires disabling gpgcheck. |
| `CCE-88256-3` | `ensure_redhat_gpgkey_installed` | HIGH | Relevant only if UMRS uses a Red Hat cosigned key. If UMRS uses a project-specific signing key, the key must be imported via RPM `%pre` or pre-installed by the site's key management procedure. |

### 1d. File Integrity — AIDE Coverage of UMRS Binaries

| CCE | Signal | Severity | Deployment Implication |
|---|---|---|---|
| `CCE-86441-3` | `aide_check_audit_tools` | medium | When UMRS is installed to `/usr/bin/`, AIDE must include the binaries. The UMRS RPM `%post` should either install an AIDE rule fragment to `/etc/aide.conf.d/` or document that the site admin must run `aide --update` after install. |
| `CCE-90260-1` | `aide_use_fips_hashes` | medium | Any AIDE rule fragment for UMRS binaries must use FIPS-approved hashes. The `md5` attribute alone is insufficient. Use `sha512` (and verify AIDE was built against OpenSSL for FIPS compliance). |
| `CCE-86738-2` | `aide_periodic_cron_checking` | medium | The deployment procedure must note that AIDE periodic checks must be active for UMRS binary integrity monitoring to be meaningful. If `aide_periodic_cron_checking` is not satisfied on the host, AIDE rule coverage of UMRS binaries provides no ongoing protection. |

### 1e. Auditd Infrastructure Dependencies

| CCE | Signal | Severity | Deployment Implication |
|---|---|---|---|
| `CCE-89816-3` | `audit_rules_immutable` | medium | UMRS audit rule fragments (e.g., a future `/etc/audit/rules.d/umrs-xattr.rules`) must be installed BEFORE audit rule immutability (`-e 2`) is set. The RPM `%post` scriptlet must call `augenrules --load` and must order rule files to appear before the `-e 2` trailer. If a site has already applied `-e 2`, the host requires a reboot after UMRS rule installation. This must be documented. |
| `CCE-88376-9` | `grub2_audit_argument` | low | If `audit=1` is not on the kernel cmdline, early-boot events (including the UMRS binary's first exec) are not captured. Deployment checklist should verify this, even though UMRS does not manage it. |

---

## Section 2 — SELinux Policy Files in `components/platforms/rhel10/` — CCE Annotation Status

Three policy modules exist under `components/platforms/rhel10/umrs-selinux/policy/`:

- `umrs.te` + `umrs.fc` + `umrs.if` — base UMRS custody types
- `umrs_cui.te` + `umrs_cui.fc` + `umrs_cui.if` — CUI custody extension
- `backup-cui_lei.te` / `cui_lei.te` / `cui_lei.if` — LEI custody (backup and active)

**Finding:** None of the policy files contain CCE comments. For policy modules, CCE comments
serve two purposes: (1) they identify which STIG requirements the policy satisfies, enabling
automated compliance mapping; (2) they inform an auditor reviewing the policy whether each
`neverallow` or `allow` rule was written to satisfy a specific control or for operational reasons.

The following CCEs are directly satisfied by rules in these policy files and should be cited:

### `umrs.te` and `umrs_cui.te` — `neverallow` rules block unconfined domain access

```
# CCE-89386-7 (RHEL 10 STIG — selinux_state, AC-3, SC-7(21))
# The neverallow rules below enforce mandatory separation by ensuring
# unconfined_t cannot reach UMRS custody types even when SELinux mode is enforcing.
# This policy satisfies the separation requirement that selinux_state verifies
# at the host level.
neverallow unconfined_t umrs_data_type:file *;
...
```

### `umrs.fc` — file context assignments satisfy DAC/MAC deployment requirements

The `.fc` file assigns `system_u:object_r:umrs_config_ro_t:s0` and
`system_u:object_r:umrs_data_ro_t:s0` to install paths. These assignments directly implement
the file-labeling posture that a STIG scan would verify. The following CCEs are relevant:

| Path | Type Assigned | Relevant CCE | NIST |
|---|---|---|---|
| `/etc/umrs(/.*)?` | `umrs_config_ro_t` | CCE-89386-7 (selinux_state) | AC-3 |
| `/var/lib/umrs(/.*)?` | `umrs_data_ro_t` | CCE-89386-7 | AC-3 |
| `/var/log/umrs(/.*)?` | `umrs_log_ro_t` | CCE-89386-7 | AU-9 |

**Recommended action (security-engineer):** Add a module-level comment block to each `.te` file
citing the CCEs the module satisfies. Add a restorecon reminder comment to each `.fc` file.
No structural policy changes are required — the types and neverallow rules are correct.
This is a documentation gap, not a policy gap. Severity: LOW.

**Critical gap in `.fc`:** There is no file context entry for UMRS binary paths (`/usr/bin/umrs-*`
or the current `~/.local/bin/umrs-*`). There is no `exec_t` or dedicated `umrs_exec_t` type
defined. The domain transition path — how a user shell transitions into a confined UMRS domain
when executing a UMRS binary — is not defined in any existing policy file. Severity: HIGH (see
Section 4 for the full finding).

---

## Section 3 — fapolicyd Blocking `~/.local/bin` — Does This Have a CCE?

**Yes.**

| CCE | Signal | Severity | NIST Controls |
|---|---|---|---|
| `CCE-89813-0` | `package_fapolicyd_installed` | medium | `CM-6(a)`, `SI-4(22)` |

The CCE is for `package_fapolicyd_installed` — the STIG requires fapolicyd to be installed,
which implicitly requires all trusted binaries to be in the fapolicyd trust database. There
is no separate CCE for "user-local binaries are blocked by fapolicyd" — that is a consequence
of the enforcement posture the STIG mandates.

**Full deployment risk description:**

On a STIG-compliant RHEL 10 host with fapolicyd in enforcing mode:
- `~/.local/bin/umrs-ls` will be denied execution with `EPERM` (or silently killed by the
  kernel, depending on fapolicyd mode).
- The denial is not logged at the application level — only in the fapolicyd audit log
  (`/var/log/fapolicyd/fapolicyd.log`). An operator who sees no output from `umrs-ls` will
  not understand why without checking that log.
- This affects the entire current deployment model until UMRS ships an RPM.

**Remediation path (two options):**

Option A — RPM-based (preferred, satisfies CCE-89813-0 automatically):
When UMRS ships as an RPM installed to `/usr/bin/`, fapolicyd trusts RPM-managed files
automatically via the RPM trust backend. No explicit fapolicyd rule is needed.

Option B — Manual trust entry (user-local deployment, interim):
```bash
fapolicyd-cli --file add ~/.local/bin/umrs-ls
fapolicyd-cli --file add ~/.local/bin/umrs-state
fapolicyd-cli --update
```
This must be documented in the deployment procedure as a mandatory step for user-local installs
on hardened hosts. Without it, UMRS silently fails to execute.

**Owner:** tech-writer (deployment procedure), security-engineer (RPM spec `%post` integration note).

---

## Section 4 — Missing Binary File Context (HIGH)

```
File: components/platforms/rhel10/umrs-selinux/policy/umrs.fc
Location: entire file
Finding: No file context entry exists for UMRS binary paths. No umrs_exec_t type is defined
         in umrs.te. The domain transition from user_t or staff_t into a confined UMRS execution
         domain is not specified. UMRS binaries running without a dedicated exec type will inherit
         the caller's domain (unconfined_t for interactive users), making the neverallow rules
         in umrs.te ineffective at enforcing the intended separation — the binary runs in the
         same domain as the caller and can access umrs_data_type objects only if the caller's
         domain allows it, not through UMRS-specific policy.
Severity: HIGH
Control reference: NIST SP 800-53 AC-3, AC-6(1); NSA RTB RAIN (Non-Bypassability)
Remediation owner: security-engineer
Recommended action: Define umrs_exec_t and a domain transition in umrs.te, and add the
                    corresponding .fc entry. Minimum additions:

                    In umrs.te:
                      type umrs_t;          # UMRS execution domain
                      type umrs_exec_t;     # UMRS binary type
                      domain_type(umrs_t)
                      application_domain(umrs_t, umrs_exec_t)
                      role system_r types umrs_t;
                      # Grant umrs_t read access to its own config and data
                      umrs_read_config(umrs_t)
                      umrs_read_data(umrs_t)
                      umrs_append_logs(umrs_t)

                    In umrs.fc (system install path):
                      /usr/bin/umrs-.*  --  system_u:object_r:umrs_exec_t:s0

                    The transition rule (staff_t -> umrs_t on umrs_exec_t) must be
                    written once the caller domain is decided (staff_t for interactive,
                    or a dedicated operator role for MLS). This is a Phase 3e item.
```

---

## Section 5 — Audit Rule Composite Indicators — CCE Grouping for AuditdDaemonConfig, AuditRulesIntegrity, AuditRulesSelinux

The security-auditor recommended three composite indicators to replace 51 individual
audit-rule signals. From the deployment perspective, the CCE groupings below support
that recommendation and provide the correct citations for each composite.

### AuditRulesSelinux (Tier 1 from familiarization notes)

This composite covers audit rules that monitor SELinux management operations and xattr
manipulation. These are the audit rules most directly relevant to UMRS operation.

| CCE | Signal | Note |
|---|---|---|
| `CCE-89571-4` | `audit_rules_dac_modification_setxattr` | UMRS reads security.selinux xattr — write auditing required |
| `CCE-89370-1` | `audit_rules_dac_modification_fsetxattr` | fd-anchored xattr writes |
| `CCE-88052-6` | `audit_rules_dac_modification_lsetxattr` | symlink xattr writes |
| `CCE-89677-9` | `audit_rules_dac_modification_removexattr` | xattr removal — label stripping |
| `CCE-88352-0` | `audit_rules_dac_modification_fremovexattr` | fd-anchored label stripping |
| `CCE-90100-9` | `audit_rules_dac_modification_lremovexattr` | symlink label stripping |
| `CCE-87762-1` | `audit_rules_execution_chcon` | chcon label changes |
| `CCE-89541-7` | `audit_rules_execution_semanage` | semanage policy modification |
| `CCE-87741-5` | `audit_rules_execution_setsebool` | runtime policy boolean changes |
| `CCE-88818-0` | `audit_rules_execution_setfiles` | filesystem relabeling |

NIST controls (composite): `AC-6(9)`, `AU-2(d)`, `AU-12(c)`, `NIST SP 800-53 AC-4`

### AuditRulesIntegrity (Tier 1 — module loading + immutability)

| CCE | Signal | Note |
|---|---|---|
| `CCE-90172-8` | `audit_rules_kernel_module_loading_init` | init_module syscall |
| `CCE-88638-2` | `audit_rules_kernel_module_loading_finit` | finit_module syscall |
| `CCE-89982-3` | `audit_rules_kernel_module_loading_delete` | delete_module syscall |
| `CCE-89816-3` | `audit_rules_immutable` | `-e 2` immutability requirement |
| `CCE-86727-5` | `audit_rules_privileged_commands_kmod` | kmod invocation |
| `CCE-89893-2` | `audit_rules_privileged_commands_modprobe` | modprobe invocation |

NIST controls (composite): `AC-6(9)`, `AU-12(c)`, `CM-6(a)`

### AuditdDaemonConfig (daemon-level, not per-rule)

These cover the auditd daemon configuration — separate from specific audit rule content.

| CCE | Signal | Note |
|---|---|---|
| `CCE-88376-9` | `grub2_audit_argument` | `audit=1` kernel cmdline |
| `CCE-88192-0` | `grub2_audit_backlog_limit_argument` | `audit_backlog_limit` |
| (various) | `auditd_data_retention_*` | Retention, flush, disk space signals |

NIST controls (composite): `AU-3`, `AU-8`, `AU-11`, `CM-6(a)`

**Deployment note:** `AuditRulesSelinux` is the highest-priority composite from a UMRS deployment
standpoint. The xattr audit rules directly cover the kernel operations that UMRS's
`SecureXattrReader` reads — any unauthorized write to `security.selinux` after UMRS reads it
is only detectable via these audit rules. Without them, the TOCTOU window between UMRS's
fd-anchored xattr read and the next read is not audited. This should be noted in the composite
indicator description.

---

## Gap Analysis Summary

```
Files reviewed: 6 (umrs.te, umrs.fc, umrs.if, umrs_cui.te, umrs_cui.fc, umrs_cui.if)
                + scap_familiarization.md + stig-signal-index.md (reference)
Total findings: 4 (1 HIGH, 1 MEDIUM, 2 LOW)

HIGH:
  - Binary exec type (umrs_exec_t) not defined; domain transition not specified.
    UMRS binaries run in caller domain, making custody neverallow rules ineffective.

MEDIUM:
  - fapolicyd blocks ~/.local/bin binaries on STIG-hardened hosts.
    CCE-89813-0 applies. No fapolicyd trust entry in deployment procedure.
    Affects all users of the current deployment model.

LOW:
  - Policy files have no CCE comment annotations. Compliance mapping
    from policy to STIG is manual-only.
  - No AIDE rule fragment for UMRS binaries. CCE-86441-3, CCE-90260-1 apply.
    Risk deferred until system install path (/usr/bin/) is adopted.

Policy artifacts written: none (supplementary report only)
Policy artifacts needed:
  - umrs.te: add umrs_t domain, umrs_exec_t type, domain transition rules
  - umrs.fc: add /usr/bin/umrs-.* --  system_u:object_r:umrs_exec_t:s0
  - /etc/audit/rules.d/umrs-xattr.rules: audit rule fragment for xattr/SELinux ops
  - /etc/aide.conf.d/umrs.conf: AIDE rule fragment for UMRS binary monitoring
  Owner: security-engineer (policy), tech-writer (deployment docs for fapolicyd/AIDE)

Documentation gaps:
  - Deployment procedure does not gate on getenforce / crypto policy before semodule
  - fapolicyd trust requirement not documented for user-local install model
  - AIDE coverage requirement not documented
  - audit_rules_immutable ordering constraint not documented for RPM %post

Code-vs-policy inconsistencies:
  - umrs_cui.fc is essentially empty (one comment line, no path entries).
    CUI custody types are defined in umrs_cui.te but no filesystem paths are
    labeled with CUI types. Either CUI path labeling is deferred (acceptable —
    document it) or it was inadvertently omitted.
```
