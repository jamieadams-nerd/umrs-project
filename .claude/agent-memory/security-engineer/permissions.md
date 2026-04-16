---
name: Knox permissions granted by Jamie
description: Durable privilege grants for Knox on the UMRS host
type: reference
---

# Permissions Granted to Knox

## sudo semodule (granted 2026-04-13)

Jamie has granted Knox permission to run `sudo semodule` on the UMRS development host. Use this for:

- Loading built policy packages: `sudo semodule -i umrs.pp`
- Reloading after rebuild: `sudo semodule -r umrs && sudo semodule -i umrs.pp`
- Listing loaded modules: `sudo semodule -l | grep umrs`
- Removing modules for clean tests: `sudo semodule -r umrs`

**Module naming axiom (Jamie, 2026-04-13):** Going forward there is exactly ONE UMRS
policy module — the single `umrs` module Knox is actively working on. Previous
iterations (`umrs_cui`, `cui_lei`, etc.) are archived and must not be reintroduced.

**Transitional pre-load cleanup (one-time, may still be needed on existing systems):**
Before loading the current `umrs` module on a system that participated in earlier
iterations, enumerate every loaded `umrs*` module and remove them to clear shadows:

```bash
# One-time cleanup of legacy names on systems that saw the earlier experiments
for mod in $(sudo semodule -l | awk '/^umrs/ {print $1}'); do
    sudo semodule -r "$mod"
done
# Then load the one true umrs module
sudo semodule -i umrs.pp
```

On a clean system (or after the one-time cleanup has been done), the normal reload
cycle is `sudo semodule -r umrs && sudo semodule -i umrs.pp`.

Log every module removed and installed to the task log for traceability.

This means Knox no longer needs to hand off .pp files to Jamie for the install step — Knox installs directly.

## /opt/umrs install pipeline (granted 2026-04-13)

Jamie has granted Knox permission to run the following commands directly on the
UMRS development host, in support of the `/opt/umrs` install pipeline:

- `sudo semanage fcontext -a|-m|-d ...` — register/modify/remove fcontext entries
- `sudo semanage login -a|-m|-d ...` — map login accounts to confined SELinux users
  (e.g., `sudo semanage login -a -s user_u umrs`)
- `sudo restorecon -RFv /opt/umrs` (and subpaths) — apply labels after module load
- `sudo -u umrs mkdir -p /opt/umrs/...` — create the directory skeleton under the
  umrs-owned install prefix
- `sudo chown umrs:umrs /opt/umrs/...` and `sudo chmod` on paths under `/opt/umrs`
  — only when correcting ownership/mode on paths Knox created

The `umrs` login account and group already exist (uid/gid 1004). Knox does NOT
need `sudo useradd` or `sudo groupadd` for routine install work — flag to Jamie
if a new account or group becomes necessary.

**Standard install sequence (FHS 2.3 compliant — document this in the operator guide):**

The UMRS layout spans three FHS roots per FHS 2.3 §3.7.4, §3.12, §4.9.2,
§5.15:

- `/opt/umrs/`       — static package files (binaries, reference data, docs)
- `/etc/opt/umrs/`   — host-editable configuration (reserved for future use)
- `/var/opt/umrs/`   — variable data (logs, state)

All three roots are provisioned once as `umrs:umrs 0755` before any install runs.

```bash
# 1. Provision the three FHS roots (one-time, out of band)
sudo mkdir -p /opt/umrs /etc/opt/umrs /var/opt/umrs
sudo chown umrs:umrs /opt/umrs /etc/opt/umrs /var/opt/umrs
sudo chmod 0755 /opt/umrs /etc/opt/umrs /var/opt/umrs

# 2. Build the SELinux policy module
cd components/rusty-gadgets/selinux
make
sudo semodule -i umrs.pp
sudo semodule -l | grep '^umrs$'

# 3. Stage and install the workspace
cd ..
cargo xtask stage
./scripts/umrs-install.sh

# 4. Apply labels (install script already runs restorecon, but repeat if
#    the policy module is reloaded afterward)
sudo restorecon -RFv /opt/umrs
# Future: also /etc/opt/umrs and /var/opt/umrs once umrs.fc.in is updated
#         with FHS-compliant entries for those paths.

# 5. Map the umrs login account to a confined SELinux user
sudo semanage login -a -s user_u umrs
sudo semanage login -l | grep '^umrs'

# 6. Verify
./scripts/umrs-install.sh --verify
ls -lZ /opt/umrs/bin /opt/umrs/share/umrs /var/opt/umrs/log
```

**Known gap (2026-04-16):** `umrs.fc.in` still encodes the pre-FHS paths
(`/opt/umrs/etc`, `/opt/umrs/var`). Until it is refreshed, `restorecon`
will not label the new `/etc/opt/umrs` and `/var/opt/umrs` trees. The
installer still runs cleanly because the files landing in those trees
receive the parent directory's context via inheritance.

**Still needs Jamie for:**
- `sudo useradd` / `sudo groupadd` — not yet granted
- Writes under `/opt/umrs/` as root (Knox must use `sudo -u umrs` or act as `umrs`
  via `ssh umrs@localhost`)

**Safety rules:**
- Never run `sudo semodule` on a production system without explicit Jamie approval for that system
- Always note what was loaded/removed in the task log so the system state is traceable
- If a policy load fails, do not retry silently — report the exact error to Jamie
