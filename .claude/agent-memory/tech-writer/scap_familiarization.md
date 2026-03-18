---
name: SCAP/STIG corpus familiarization — RHEL 10 signal index and CCE cross-reference
description: Analysis of STIG descriptor language, CCE identifier format, and operator terminology from the RHEL 10 SCAP Security Guide corpus. Produced during Phase 2 of scap-stig-corpus-plan.
type: reference
---

# SCAP/STIG Corpus Familiarization

**Sources read:**
- `.claude/references/scap-security-guide/stig-signal-index.md` — 451 signals, tabular: Signal Name / CCE / NIST Controls / Severity / Description / Check Method
- `.claude/references/scap-security-guide/cce-nist-crossref.md` — 451 CCEs, tabular: CCE / NIST Controls / Signal Name / Description

---

## 1. Descriptive Text Patterns

### 1.1 Action verb conventions

STIG descriptions consistently open with an imperative infinitive phrase. The dominant patterns:

| Pattern | Examples |
|---|---|
| **Ensure X is Y** | "Ensure AIDE is installed", "Ensure SELinux State is Enforcing", "Ensure auditd Collects Information..." |
| **Enable/Disable X** | "Enable Kernel Parameter to Use TCP Syncookies", "Disable SSH Access via Empty Passwords" |
| **Set X** | "Set Password Maximum Age", "Set SSH Client Alive Interval", "Set SSH Daemon LogLevel to VERBOSE" |
| **Configure X** | "Configure auditd mail_acct Action on Low Disk Space", "Configure Fapolicy Module to Employ a Deny-all, Permit-by-exception Policy" |
| **Restrict X** | "Restrict Access to Kernel Message Buffer", "Restrict Exposed Kernel Pointer Addresses Access" |
| **Prevent X** | "Prevent Login to Accounts With Empty Password", "Prevent remote hosts from connecting to the proxy display" |
| **Add X Option to Y** | "Add nodev Option to /boot", "Add noexec Option to /tmp" |
| **Record Any Attempts to Run X** | "Record Any Attempts to Run chcon", "Record Any Attempts to Run semanage" |
| **Verify/Require** | "Verify that Interactive Boot is Disabled", "Require Authentication for Single User Mode" |

**Recommendation:** Our indicator descriptions should follow the same imperative structure. "Ensure kernel module 'bluetooth' is disabled" is more scannable than "kernel bluetooth disabled check". The STIG form also makes the indicator's intent obvious without context.

### 1.2 Signal naming conventions

Signal names use `snake_case` with consistent semantic prefixes that encode domain and check type:

| Prefix | Domain | Example |
|---|---|---|
| `account_` / `accounts_` | PAM, login, user account policy | `accounts_maximum_age_login_defs` |
| `audit_rules_` | auditd rule presence | `audit_rules_dac_modification_chmod` |
| `auditd_` | auditd daemon configuration | `auditd_data_retention_space_left_action` |
| `configure_` | policy-level configuration checks | `configure_crypto_policy` |
| `dconf_gnome_` | GNOME/GUI-layer settings | `dconf_gnome_screensaver_lock_enabled` |
| `dir_` / `file_` | filesystem ownership and permissions | `file_permissions_etc_shadow` |
| `grub2_` | boot loader arguments | `grub2_pti_argument` |
| `kernel_module_` | kernel module load state | `kernel_module_usb-storage_disabled` |
| `mount_option_` | filesystem mount flags | `mount_option_tmp_noexec` |
| `package_` | installed/removed package state | `package_aide_installed` |
| `selinux_` | SELinux configuration | `selinux_state`, `selinux_policytype` |
| `service_` | systemd service state | `service_systemd-coredump_disabled` |
| `sshd_` | SSH daemon configuration | `sshd_disable_root_login` |
| `sysctl_` | kernel runtime parameter | `sysctl_kernel_randomize_va_space` |

**Observation:** UMRS currently uses `SignalId` (or the in-progress rename to `IndicatorId`) with enum variants. The STIG naming convention is a good model for how variant names should read when serialized: each name is self-describing without external documentation.

### 1.3 Severity levels

The STIG uses exactly three levels: `high`, `medium`, `low`. Two entries use `unknown` (these are genuinely unrated). The distribution across 451 signals:
- `medium` — overwhelming majority (~390 signals)
- `high` — ~30 signals (notably: crypto policy, selinux_state, empty passwords, GNOME reboot/shutdown, telnet/tftp/vsftpd removal, no GDM auto-login)
- `low` — ~25 signals
- `unknown` — 2 signals

**Recommendation:** If UMRS surfaces STIG severity in the TUI or reports, use `High / Medium / Low` (title case). Do not expose `unknown` to operators — map it to `Unrated` or suppress.

### 1.4 Check method taxonomy

The STIG encodes how compliance is verified:

| Method | Meaning |
|---|---|
| `other` | Custom Ansible/XCCDF task (most common by far) |
| `audit-rule` | auditd rule presence check |
| `file-check` | file existence, ownership, or permission check |
| `package-check` | RPM package installed/removed check |
| `sysctl` | kernel parameter value check |
| `cmdline` | kernel command line argument check |
| `service-check` | systemd service enabled/active state |

**Observation:** This taxonomy maps closely to how UMRS posture signals are organized by domain. Our internal check-method concept aligns with this. Consider adopting these exact terms in documentation when explaining signal categories.

### 1.5 Description artifacts to avoid

The STIG descriptions show two weaknesses we should not replicate:
1. **Truncation at the dash** — many descriptions end mid-sentence, e.g., "Ensure auditd Collects Information on the Use of Privileged Commands - gpasswd". This is an artifact of the playbook's task description field being truncated. Our indicator descriptions must be complete sentences.
2. **Check-step leakage** — some descriptions describe the implementation check step, not the requirement: "Get all /etc/passwd file entries", "Set fact for sysctl paths". This is internal machinery. Our descriptions should state what the compliant state looks like, not how we detect it.

---

## 2. CCE Identifier Format

### 2.1 Observed format in the corpus

All CCEs in the corpus use the format: `CCE-NNNNN-N`

Examples:
```
CCE-88966-7
CCE-86241-7
CCE-90212-2
CCE-89307-3
```

The CCE identifier is a two-part number with a hyphen separator: a five-digit body and a single-digit check digit. The `CCE-` prefix is always present and always uppercase. There are no variants (no `cce-`, no `CCE_`, no bare numbers) in the corpus.

### 2.2 NIST SP 800-53 control citation format in the corpus

The corpus uses a consistent citation style for controls:

```
AC-3, AC-3(3)(a), AU-9, SC-7(21)
CM-6(a), CM-7(a), CM-7(b)
AU-12(a), AU-12.1(ii), AU-12.1(iv)AU-12(c), AU-3, AU-3.1
```

Key observations:
- Family prefix (`AC`, `CM`, `AU`, `SC`, `IA`, `SI`, `MA`, `MP`, `SA`) — always uppercase, no space before the hyphen
- Control number — no leading zero
- Enhancement in parentheses — `AC-3(3)` not `AC-3 Enh 3`
- Sub-enhancements — `AC-3(3)(a)` (parenthesized letter, lowercase)
- Rev 5 section refs — `AU-12.1(ii)` (dot notation with Roman numerals, lowercase)
- Some entries use a space before the letter: `AC-6 (1)` — this is inconsistent in the corpus and should be standardized to `AC-6(1)` without the space

### 2.3 Proposed CCE citation format for UMRS

The corpus confirms the canonical CCE format. Proposed standard for each UMRS context:

#### In Rust doc comments (`///` and `//!`)

```rust
/// Controls the kernel module load state.
///
/// # Compliance
///
/// - NIST SP 800-53 CM-7(a), CM-7(b) — least functionality; disable unnecessary capabilities
/// - CCE-87455-2 (RHEL 10 STIG) — Ensure kernel module 'bluetooth' is disabled
```

Rules:
- CCE citation always follows NIST on a separate bullet, never alone
- Format: `CCE-NNNNN-N (RHEL 10 STIG) — <descriptive text>`
- The parenthetical `(RHEL 10 STIG)` provides provenance — the CCE number alone is meaningless without context
- Descriptive text is the STIG signal description in imperative form (normalized — no truncation)

#### In Antora documentation pages

For compliance cross-reference tables (e.g., in `reference/pages/` or compliance registries):

```asciidoc
|CCE-87455-2
|RHEL 10 STIG
|`kernel_module_bluetooth_disabled`
|CM-7(a), CM-7(b)
|Ensure kernel module 'bluetooth' is disabled
```

For inline citations in body text:

```asciidoc
The SELinux enforcement check (CCE-89386-7) maps to NIST SP 800-53 AC-3 and SC-7(21).
```

Rules:
- Inline: parenthetical, no surrounding markup beyond code font if the CCE appears in a control list
- Tables: CCE in its own column; signal name in `monospace`; description normalized (no truncation)
- Always include the STIG profile identifier in a table header or column: "RHEL 10 STIG"

#### In CLI/TUI output

In structured output (e.g., `--json` mode), the CCE identifier is a machine-readable field:

```json
{
  "indicator": "kernel_module_bluetooth_disabled",
  "cce": "CCE-87455-2",
  "stig_profile": "RHEL 10",
  "nist_controls": ["CM-7(a)", "CM-7(b)"],
  "severity": "medium"
}
```

In human-readable TUI display, CCE should appear as a secondary detail, not a primary label:

```
[MEDIUM] kernel bluetooth module disabled
         CCE-87455-2 · CM-7(a) CM-7(b)
```

Rules:
- CCE never appears on the primary indicator line — it belongs in a detail or secondary row
- If space is constrained (list view), suppress CCE; show only in detail/expanded view
- `CCE-NNNNN-N` format preserved exactly in all output — no abbreviation

---

## 3. Operator-Facing Terminology Alignment

### 3.1 STIG terms that differ from current UMRS usage

| STIG term | Current UMRS usage | Recommendation |
|---|---|---|
| **signal** (in index file header) | UMRS uses "signal" (being renamed to "indicator") | The STIG corpus uses "signal" loosely as a label for a check entry. UMRS "indicator" is more precise. Keep UMRS term; note alignment in documentation. |
| **rule** (as in `audit_rules_*`) | UMRS uses "audit event" | STIG "rule" = an auditd syscall filter rule. UMRS "audit event" = the output of that rule. Different concepts — do not conflate. |
| **check** (as in check method) | UMRS uses "probe" conceptually | STIG "check method" = how compliance is assessed. Document that our "probe" is the UMRS equivalent of the STIG "check". |
| **severity** (high/medium/low) | UMRS does not yet surface STIG severity | If we surface STIG severity, adopt the STIG terms exactly: `High`, `Medium`, `Low` (title case in UI; lowercase in JSON). |
| **profile** | Not used in UMRS yet | When citing STIGs, always qualify: "RHEL 10 STIG" (not just "STIG"). More than one STIG exists. |
| **XCCDF** | Not used in UMRS | Acceptable to use in internal/developer docs. Operators do not need to know this term. |

### 3.2 Terms we can adopt without change

These STIG terms are compatible with UMRS terminology and should be used when referencing STIG content:

- **CCE** — Common Configuration Enumeration. Spell out on first use per page.
- **SCAP** — Security Content Automation Protocol. Spell out on first use per page.
- **STIG** — Security Technical Implementation Guide. Spell out on first use per page (DoD audience will know it; civilian audience may not).
- **policytype** (SELinux) — exact STIG term for the `SELINUXTYPE=` config value. Use in SELinux context documentation.
- **enforcing** — exact STIG term and kernel term. Already preferred in UMRS. Good.

### 3.3 STIG descriptions for SELinux-relevant signals

These four STIG entries are directly relevant to UMRS core functionality:

| Signal | CCE | NIST | STIG Description |
|---|---|---|---|
| `selinux_state` | CCE-89386-7 | AC-3, AC-3(3)(a), AU-9, SC-7(21) | Ensure SELinux State is Enforcing |
| `selinux_policytype` | CCE-88366-0 | AC-3, AC-3(3)(a), AU-9, SC-7(21) | Insert correct line to /etc/selinux/config |
| `account_password_selinux_faillock_dir` | CCE-90568-7 | AC-7(a) | An SELinux Context must be configured for the pam_faillock.so records directory |
| `accounts_passwords_pam_faillock_dir` | CCE-90182-7 | AC-7(a), AC-7(b), AC-7.1(ii) | Lock Accounts Must Persist — Ensure necessary SELinux packages are installed |

Note: The STIG description for `selinux_policytype` ("Insert correct line to /etc/selinux/config") is a check-step artifact, not a requirement statement. Our documentation should normalize it to: "Ensure SELinux policy type is set to the required profile."

### 3.4 FIPS-relevant signals

| Signal | CCE | STIG Description |
|---|---|---|
| `configure_crypto_policy` | CCE-89085-5 | Configure System Cryptography Policy |
| `aide_use_fips_hashes` | CCE-90260-1 | Configure AIDE to Use FIPS 140-2 for Validating Hashes |
| `configure_bind_crypto_policy` | CCE-86874-5 | Configure BIND to use System Crypto Policy |

---

## 4. Summary — Proposed Additions to Approved Terminology

These terms should be proposed for addition to `.claude/agent-memory/doc-team/approved_terminology.md`:

| Proposed term | Use | Do not use |
|---|---|---|
| `CCE` | Common Configuration Enumeration identifier (SCAP) | No variants; always uppercase |
| `SCAP` | Security Content Automation Protocol | "SCAP content" is acceptable shorthand after first use |
| `STIG` | Security Technical Implementation Guide | "STIG check" (imprecise); use "STIG indicator" or "STIG requirement" |
| `RHEL 10 STIG` | Full profile name | "the STIG" (without qualification) — multiple STIGs exist |
| `XCCDF` | Extensible Configuration Checklist Description Format | Developer/internal docs only; not operator-facing |

---

## 5. Open Items / Flags for Jamie

1. **CCE citation in Rust source**: The `cce_crossref.md` requirement in `project_cce_crossref.md` (per MEMORY.md) means existing source files need CCE sweep. This is a rust-developer task — tech-writer produces the format standard (this document), not the edits.

2. **STIG severity in TUI**: No decision yet on whether UMRS surfaces STIG severity. Propose: include in `--json` output from the start; show in TUI detail view only. Needs Jamie sign-off before implementation.

3. **Truncated STIG descriptions**: ~50 signals in the corpus have truncated descriptions (cut at a dash). When we write indicator descriptions that align to STIG signals, we must complete these. A list of affected signals can be produced if needed.

4. **`selinux_policytype` description normalization**: The STIG description is a check-step artifact. If we reference this signal in UMRS docs, use the normalized form: "Ensure the SELinux policy type is set to the required profile."
