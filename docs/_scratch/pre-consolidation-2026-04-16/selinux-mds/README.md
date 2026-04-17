# UMRS SELinux Policy (Phase A)

This directory holds the source for the `umrs` SELinux policy module. Phase A
is labeling + awareness under targeted policy — it declares types for UMRS
binaries, configuration, data, and logs, and installs a `neverallow` guardrail
that keeps UMRS custody unreachable from `unconfined_t`. Phase A does not make
any CUI-enforcement claims; mandatory access control across sensitivity levels
is Phase 2 work (MLS policy).

## Files

| File | Role |
|---|---|
| `umrs.te` | Policy source: attributes, types, neverallow guardrails. |
| `umrs.if` | Interface macros (`umrs_exec`, `umrs_read_config`, `umrs_manage_config`, `umrs_read_data`, `umrs_manage_data`, `umrs_append_logs`, `umrs_manage_logs`). |
| `umrs.fc.in` | File-context template with `@PREFIX@` placeholder. Edit this; never edit `umrs.fc` directly. |
| `umrs.fc` | Generated artifact (produced by `make` from `umrs.fc.in`). Not committed intent — regenerate via `make`. |
| `Makefile` | Builds `umrs.pp` via the system refpolicy devel kit. |
| `archive/` | Retired policy drafts. Do not modify. |
| `junk/` | Scratch. Do not modify. |

## Types and attributes

Attributes (grouping):

- `umrs_exec_type` — all UMRS executable types.
- `umrs_config_type` — configuration files (RO baseline + RW overrides).
- `umrs_data_type` — UMRS data files (Phase A: placeholder working dir; Phase B: vault).
- `umrs_log_type` — UMRS log files.

Concrete types (Phase A):

- `umrs_bin_t` (bound to `umrs_exec_type`)
- `umrs_config_ro_t`, `umrs_config_rw_t` (bound to `umrs_config_type`)
- `umrs_data_rw_t` (bound to `umrs_data_type`)
- `umrs_log_rw_t` (bound to `umrs_log_type`)

### Deferred / removed in this revision

- `umrs_httpd_t` — removed. No HTTP-surfaced UMRS component is designed yet;
  re-declare when one is.
- `umrs_data_ro_t`, `umrs_log_ro_t` — removed for Phase A. The prior draft
  declared these without any fcontext rule pointing at them; re-introduce
  when a concrete read-only data or log path exists.
- `umrs_trust_list_t`, `umrs_secret_t` — Phase B.
- Per-tool exec types (`umrs_ls_exec_t`, etc.) — Phase B.

### Note on `share/umrs`

The proposal tagged `/opt/umrs/share/umrs/` as `umrs_data_type` (an attribute).
fcontext rules require a concrete type, so `umrs.fc.in` labels this path
`umrs_data_rw_t`. For Phase A the directory ships read-only via DAC
(`root:root:0644` files); the SELinux type is the RW data type only because
no RO data type exists in this revision. Revisit if / when an RO data type
is re-introduced.

## Build

```
make clean
make
```

Produces `umrs.pp` in this directory. The Makefile defaults `PREFIX=/opt/umrs`;
override with `make PREFIX=/usr/local/umrs` for a non-default install prefix.

The Makefile delegates the actual compile to `/usr/share/selinux/devel/Makefile`
(refpolicy devel kit). The three `duplicate definition` warnings from
`passt.if` and `smartmon.if` are a distribution-shipped policy issue unrelated
to UMRS — ignore.

## Install (Jamie runs these; Makefile only prints them)

After `make` produces `umrs.pp`:

```
sudo semodule -i umrs.pp
sudo restorecon -RF /opt/umrs
```

`semodule -i` bundles the file contexts from `umrs.fc` into the policy store;
a separate `semanage fcontext -a` pass is not required unless you need
prefix-equivalence or a path override (see `make load-fcontext` for the exact
command).

To reload after a policy change:

```
make
sudo semodule -r umrs || true
sudo semodule -i umrs.pp
sudo restorecon -RF /opt/umrs
```

## Phase A acceptance criteria (to verify after `semodule -i`)

From the Phase A proposal (`.claude/reports/2026-04-13-opt-umrs-deployment-layout-proposal.md`,
§0):

1. **Clean install.** The staging procedure below completes without error.
2. **Layout correctness.** `/opt/umrs/` exists; `stat -c '%U %G %a'` on each
   directory matches the DAC table in the proposal §7.
3. **Binary labeling.** `ls -Z /opt/umrs/bin/umrs-ls` reports `umrs_bin_t`;
   `matchpathcon /opt/umrs/bin/umrs-ls` agrees.
4. **Confined execution.** An operator re-mapped via
   `semanage login -m -s user_u <login>` can run `/opt/umrs/bin/umrs-ls`, and
   the resulting process (`ps -eZ`) is NOT `unconfined_t`.
5. **Neverallow holds.** A shell from `unconfined_t` is denied `execute` on
   `/opt/umrs/bin/umrs-ls`; the AVC shows the denial against `umrs_exec_type`.
6. **Configuration reads succeed in the confined domain.** A UMRS tool in the
   confined operator domain can read files labeled `umrs_config_ro_t` under
   `/opt/umrs/etc/` with no AVC denials.
7. **`verify-install` passes** (once implemented — coder owns this).

## Smoke test

Knox stages files non-privileged. Jamie runs the `sudo` commands.

```bash
# (Knox) produce the policy package
cd /DEVELOPMENT/umrs-project/components/rusty-gadgets/selinux
make clean && make

# (Jamie) load the policy
sudo semodule -i umrs.pp

# (Jamie) stage the demo install tree
sudo mkdir -p /opt/umrs/{bin,etc,share/man/man1,share/man/fr_CA/man1,share/templates,share/umrs,var/lib,var/log}
sudo groupadd umrs-admin 2>/dev/null || true
sudo useradd -m -g umrs-admin umrs-admin 2>/dev/null || true

# (Jamie) copy staged binaries
sudo cp /DEVELOPMENT/umrs-project/components/rusty-gadgets/staging/bin/* /opt/umrs/bin/
sudo chown -R root:root /opt/umrs/bin
sudo chmod 0755 /opt/umrs/bin/*
sudo chown root:umrs-admin /opt/umrs/etc
sudo chmod 2775 /opt/umrs/etc
sudo chown root:umrs-admin /opt/umrs/var/lib
sudo chmod 2775 /opt/umrs/var/lib
sudo chown root:umrs-admin /opt/umrs/var/log
sudo chmod 2770 /opt/umrs/var/log

# (Jamie) apply SELinux labels
sudo restorecon -RF /opt/umrs/

# (Jamie) verify labels
ls -Z /opt/umrs/bin/umrs-ls | grep umrs_bin_t
matchpathcon /opt/umrs/bin/umrs-ls
ls -Z /opt/umrs/etc | head

# (Jamie) map the operator login to a confined SELinux user
sudo semanage login -m -s user_u umrs

# (Jamie) confined exec check
ssh umrs@localhost 'id -Z && /opt/umrs/bin/umrs-ls > /dev/null; ps -eZ $$ | head'
# process context must NOT be unconfined_t

# (Jamie) neverallow check: confirm unconfined_t cannot execute
# From an unconfined shell:
/opt/umrs/bin/umrs-ls   # expected: permission denied + AVC against umrs_exec_type
sudo ausearch -m AVC -ts recent | grep umrs_bin_t
```

## One-line install command (for Jamie)

```
cd /DEVELOPMENT/umrs-project/components/rusty-gadgets/selinux && make && sudo semodule -i umrs.pp && sudo restorecon -RF /opt/umrs
```
