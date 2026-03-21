# M3.5 Deployment Security Plan

**Date:** 2026-03-21
**Status:** draft — awaiting Jamie decisions (install path, CUI data path)
**Author:** Knox (security-engineer)
**ROADMAP Goals:** G5 (Security Tools), G6 (Multi-Platform)
**Milestones:** M3.5 (Full Deployment)
**Tech Lead:** Knox (security-engineer)
**LOE:** Large (~5-6 sessions total across all phases)
**Reviewers:** Herb (security-auditor) for policy review, Rusty for RPM integration

---

## Problem

UMRS currently installs to `~/.local/bin` — entirely outside SELinux's managed binary
execution model. The custody type hierarchy (data, config, log, CUI types) exists in
policy but is inert: no process domain exists, no binaries are labeled, no domain
transitions occur. Moving to system-installed changes the trust boundary.

---

## Current State — What Exists

The policy foundation at `components/platforms/rhel10/umrs-selinux/policy/` is solid:

- `umrs.te` — 7 file types grouped under attributes, `neverallow` guardrails
- `umrs.fc` — 5 filesystem path mappings (data, config, log, share, httpd)
- `umrs.if` — clean interface macros (read, manage, append)
- `umrs_cui.te` / `cui_lei.te` — CUI custody hierarchy with subcategory types
- Makefile — compiles and installs all three modules

## Critical Gaps — What Is Absent

| # | Gap | Impact |
|---|---|---|
| 1 | No process domain (`umrs_t` / `umrs_exec_t`) | Binaries run as `unconfined_t`; custody rules inert |
| 2 | `umrs_cui.fc` is empty | CUI types defined but no paths assigned |
| 3 | No binary file contexts | No domain transition possible |
| 4 | No fapolicyd artifacts | STIG hosts silently block execution |
| 5 | No AIDE rules | Integrity changes undetected |
| 6 | No RPM spec file | No packaging pipeline |
| 7 | No systemd unit hardening | No capability bounding for daemon tools |

---

## Decisions Needed from Jamie

1. **Binary install path:** `/usr/bin` (Knox recommends — STIG-aligned, covered by
   default AIDE rules) vs `/usr/local/bin` (not covered by default AIDE/fapolicyd)

2. **CUI data path:** Where do CUI files live on disk? Needed for `umrs_cui.fc`.
   Candidates: `/var/lib/umrs/cui/`, `/home/*/Documents/cui/`, or operator-defined

3. **`umrs-cui` crate naming:** `Cargo.toml` says `cui-labels` but directory is
   `umrs-cui`. Resolve before `umrs-mcs` is created.

---

## Phase 1 — Process Domain and Binary Contexts

**LOE:** Medium (~2 sessions)
**Depends on:** Install path decision, source trace of binary access vectors

### Deliverables

- `umrs_tools.te` — defines `umrs_exec_t` (entry point) and `umrs_t` (domain)
- `umrs_tools.fc` — binary file contexts for `/usr/bin/umrs-*`
- `umrs_tools.if` — interface macros for the process domain
- Domain transition rules: `domain_auto_trans(init_t, umrs_exec_t, umrs_t)`
- Access vector analysis from source code (what `/proc/`, `/sys/`, `/etc/` the tools read)

### Open Question

`umrs-ls` reads extended attributes on arbitrary user-supplied paths. This requires
either broad `allow` rules or `files_read_all_files` — trading type isolation for
usefulness. **Knox needs to resolve this tension with Jamie.**

## Phase 2 — fapolicyd and AIDE Integration

**LOE:** Small (~1 session)

### Deliverables

- `components/platforms/rhel10/fapolicyd/umrs-dev-trust.rules` — development use only,
  with clear comment that it must be removed before production assessment
- `components/platforms/rhel10/aide/umrs.conf` — fragment for `/etc/aide/aide.conf.d/`
  covering binaries (CONTENT_EX), policy modules (sha512+perms), and config (hash only)
- Documentation: fapolicyd trust is automatic for signed RPMs; dev rules are for
  `cargo install` / manual install workflows only

### AIDE Coverage

| Path Class | AIDE Group | Notes |
|---|---|---|
| `/usr/bin/umrs-*` | CONTENT_EX (sha512+inode+mtime+perms+ownership) | Binary integrity |
| `/usr/share/selinux/targeted/umrs*.pp` | sha512+perms | Policy module integrity |
| `/etc/umrs/` | CONFIG (hash, exclude mtime) | Config content, not rotation timestamps |

## Phase 3 — RPM Packaging

**LOE:** Medium (~2 sessions)

### `%post` Scriptlet (exact order — Knox's requirement)

1. `semodule -i /usr/share/selinux/packages/umrs.pp umrs_cui.pp cui_lei.pp`
2. `restorecon -R /usr/bin/umrs-* /etc/umrs /var/lib/umrs /var/log/umrs /usr/share/umrs`
3. Service reload (when applicable)

### `%preun` Scriptlet

- `semodule -r umrs umrs_cui cui_lei` — prevents orphaned types

### RPM Requirements

- `Requires: policycoreutils-python-utils` (semanage)
- `Requires: policycoreutils` (restorecon, semodule)
- `BuildRequires: selinux-policy-devel` (compile .pp from source)
- RPM **must be GPG-signed** — unsigned = fapolicyd denial on STIG hosts
- Binary stripped in `%install` phase
- Source tarball verified by sha512sum (SSDF PW.4)

## Phase 4 — MLS-Preparatory Policy (Phase 2 readiness)

**LOE:** Small (~1 session)

- `mlsconstrain` stanzas for CUI data types (write-equal, read-dominated semantics)
- Dormant in targeted policy, activates when host transitions to MLS
- Write them now to prevent policy rewrite at Phase 2

## Phase 5 — Ubuntu .deb Equivalent

**LOE:** Medium (~1-2 sessions)

- Graceful degradation: no SELinux, no fapolicyd, no AIDE by default
- AppArmor profile as alternative confinement (if applicable)
- Document what security layers are absent vs RHEL

---

## Defense-in-Depth Model (6 layers)

```
Layer 1: SELinux type enforcement (MAC) — primary confinement
Layer 2: fapolicyd (application allowlist) — catches binary substitution
Layer 3: AIDE (integrity measurement) — offline forensic detection
Layer 4: IMA/EVM (Phase 2) — kernel pre-execution verification
Layer 5: Capability bounding (systemd units) — damage limitation
Layer 6: xattr custody chain — chain-of-custody evidence
```

Layers 2+3 close the xattr-sanitization gap: attacker modifies binary on disk →
fapolicyd blocks (hash mismatch) + AIDE alerts. IMA (Layer 4) adds kernel-level
pre-execution denial.

---

## Compliance

- NIST SP 800-53 AC-3: Access Enforcement (SELinux domain transition)
- NIST SP 800-53 CM-6: Configuration Settings (fapolicyd trust, AIDE rules)
- NIST SP 800-53 SI-7: Software/Information Integrity (AIDE, IMA)
- NIST SP 800-53 AU-9: Protection of Audit Information (log type isolation)
- NIST SP 800-218 SSDF PW.4: Supply chain verification (RPM signing, source hash)
- CCE-89813-0: fapolicyd blocking unsigned binaries
- CCE-86441-3: AIDE binary monitoring
- CCE-90260-1: AIDE configuration monitoring
