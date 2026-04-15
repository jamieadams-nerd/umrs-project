# UMRS Install Pipeline — Step-by-Step

This guide walks through installing UMRS into `/opt/umrs/` on a RHEL 10 host
with SELinux in targeted enforcing mode. Each step explains **what** is
happening and **why**, so the sequence is understandable rather than a recipe.

Scope: Phase 1 (labeling only). No enforcement claims are made here — the
policy module declares types and file contexts that will be used later for
MLS enforcement.

---

## Prerequisites (already done)

These were completed manually and do not need to be repeated:

- `umrs` user and group exist (uid/gid 1004, shell `/bin/bash`, home `/home/umrs`)
- `/opt/umrs` exists, owned `umrs:umrs`

Verify:

```bash
getent passwd umrs
getent group umrs
ls -ld /opt/umrs
```

---

## Step 1 — Create the directory skeleton

The SELinux policy assigns a different type to each subdirectory under
`/opt/umrs` based on what the subdirectory holds (binaries, read-only config,
writable data, logs). Those directories must exist before we apply labels,
otherwise `restorecon` has nothing to label.

Run as the `umrs` user so ownership is correct **and** the process runs in
the `umrs` account's SELinux context. Do not use `sudo -u umrs` — `sudo`
does not perform a proper SELinux role/type transition, so the resulting
files may end up with the invoking user's context rather than `umrs`'s.
Use `ssh umrs` instead (Knox has this configured):

```bash
ssh umrs mkdir -p /opt/umrs/{bin,etc,share/templates,share/umrs,var/lib,var/log}
```

Resulting layout:

| Path                        | Purpose                                    |
|-----------------------------|--------------------------------------------|
| `/opt/umrs/bin/`            | UMRS binaries + operator scripts           |
| `/opt/umrs/etc/`            | Read-only site configuration (shipped)     |
| `/opt/umrs/share/templates/`| Read-only template files                   |
| `/opt/umrs/share/umrs/`     | Writable shared data                       |
| `/opt/umrs/var/lib/`        | Writable runtime state                     |
| `/opt/umrs/var/log/`        | Log spool (currently journald-only)        |

Verify:

```bash
ls -l /opt/umrs
```

All subdirectories should be owned `umrs:umrs`.

---

## Step 2 — Build the SELinux policy module

The policy lives in this directory as three source files:

- `umrs.te`   — type enforcement: declares types, attributes, neverallow rules
- `umrs.if`   — interface file: macros other modules can call
- `umrs.fc.in`— file context template with `@PREFIX@` placeholder

The Makefile substitutes `@PREFIX@` (default `/opt/umrs`) to produce `umrs.fc`,
then compiles the module into a loadable package `umrs.pp`:

```bash
cd components/rusty-gadgets/selinux
make
```

Output: `umrs.pp`.

To build for a non-default install prefix:

```bash
make PREFIX=/some/other/path
```

---

## Step 3 — Load the policy module

Loading a `.pp` file does two things at once:
1. Registers the types declared in `umrs.te` with the kernel
2. Installs the file-context entries from `umrs.fc` into the system's
   persistent file-context database (so `restorecon` knows the correct labels)

```bash
sudo semodule -i umrs.pp
sudo semodule -l | grep '^umrs$'
```

The second command confirms the module is loaded. If an older iteration
(`umrs_cui`, `cui_lei`) is still loaded, remove it first:

```bash
sudo semodule -r umrs_cui cui_lei 2>/dev/null || true
```

---

## Step 4 — Apply the labels

At this point the kernel knows the types and the fcontext database knows
which paths get which types, but the existing files under `/opt/umrs/` are
still labeled `usr_t` (the default inherited from `/opt`). `restorecon`
walks the tree and relabels each file according to the fcontext database:

```bash
sudo restorecon -RFv /opt/umrs
```

Flags:
- `-R` recursive
- `-F` force relabel even if the current label already looks plausible
- `-v` verbose (shows each relabel)

Verify:

```bash
ls -lZ /opt/umrs
ls -lZ /opt/umrs/bin /opt/umrs/etc /opt/umrs/var/lib
```

Expected types:

| Path                | Type                |
|---------------------|---------------------|
| `/opt/umrs`         | `usr_t`             |
| `/opt/umrs/bin`     | `umrs_bin_t`        |
| `/opt/umrs/etc`     | `umrs_config_ro_t`  |
| `/opt/umrs/share/templates` | `umrs_config_ro_t` |
| `/opt/umrs/share/umrs`      | `umrs_data_rw_t`   |
| `/opt/umrs/var/lib` | `umrs_data_rw_t`    |
| `/opt/umrs/var/log` | `umrs_log_rw_t`     |

---

## Step 5 — Confine the `umrs` login account

`umrs.te` contains `neverallow` rules that forbid `unconfined_t` from touching
UMRS types. This is deliberate: an operator running UMRS tools from the
default unconfined user would bypass the custody model. The `umrs` login
account must therefore be mapped to a confined SELinux user:

```bash
sudo semanage login -a -s user_u umrs
sudo semanage login -l | grep '^umrs'
```

This maps Linux login `umrs` → SELinux user `user_u` (the standard confined
user). Subsequent logins as `umrs` will run in `user_t`, which is allowed to
interact with UMRS types per the policy.

Verify by logging in and checking the process context:

```bash
ssh umrs id -Z
```

Expected: `user_u:user_r:user_t:s0`

NOTE: Always use `ssh umrs` (not `sudo -u umrs` or `su - umrs`). Only SSH
performs the full SELinux login transition via PAM. `sudo` and `su` retain
the invoker's role/type and will give misleading results when verifying
confinement.

---

## Current status

Steps 1–5 above establish the **custody container**: a labeled directory
structure with a confined login account and loaded policy. Nothing is
installed into it yet.

## Next step — stage the build output into /opt/umrs

The `cargo xtask stage` command produces a flat tree at
`components/rusty-gadgets/staging/`:

```
staging/
  bin/       ← compiled binaries + operator scripts
  config/    ← merged config/ directories from every crate
```

The install step copies from staging into `/opt/umrs`:

- `staging/bin/*`    → `/opt/umrs/bin/`
- `staging/config/*` → `/opt/umrs/etc/`

After copying, re-run `restorecon` so the new files pick up the correct
labels:

```bash
sudo restorecon -RFv /opt/umrs/bin /opt/umrs/etc
```

The install script for this step is not written yet. That is the next piece
of work.

---

## Uninstall

The uninstall path reverses steps 1–5 but **preserves `/opt/umrs` itself**
(the top-level directory, owned `umrs:umrs`, stays in place so a
reinstall can land cleanly). Only the managed subtree and the policy
state are removed.

```bash
# 1. Remove staged content and subdirectories (as the umrs user, not sudo)
ssh umrs 'rm -rf /opt/umrs/bin /opt/umrs/etc /opt/umrs/share /opt/umrs/var'

# 2. Remove the login mapping so `umrs` returns to default confinement
sudo semanage login -d umrs

# 3. Unload the policy module (this also removes its fcontext entries)
sudo semodule -r umrs

# 4. Reset any residual labels under /opt/umrs to the default
sudo restorecon -RFv /opt/umrs
```

Verify:

```bash
ls -lZ /opt/umrs                        # should be empty, type usr_t
sudo semodule -l | grep '^umrs$'        # should produce no output
sudo semanage login -l | grep '^umrs'   # should produce no output
```

`/opt/umrs` itself is intentionally left behind — it was created outside
the install pipeline and its existence is a prerequisite, not an artifact.

---

## Troubleshooting

**Module won't load** — check for conflicting legacy modules:
```bash
sudo semodule -l | grep -E '^(umrs|cui)'
```

**`restorecon` reports no changes** — verify the fcontext entries are in the
database:
```bash
sudo semanage fcontext -l | grep /opt/umrs
```

**A file still shows `usr_t` after restorecon** — the path did not match any
fcontext pattern. Check `umrs.fc` against the actual path.

**Operator can't run a UMRS binary after login** — confirm the login mapping:
```bash
sudo semanage login -l | grep umrs
id -Z                           # as the umrs user
```
