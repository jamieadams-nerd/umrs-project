# systemd UIDS-GIDS — Style Decision Record

**Familiarized:** 2026-04-18

---

## SDR-SYSTEMD-001: Do Not Use DynamicUser= for the `umrs` Service Account

**Tension:** systemd's `DynamicUser=` feature allocates a transient UID from 61184–65519.
This is cleaner from a security standpoint (no persistent UID, no `/etc/passwd` entry),
but it conflicts with UMRS's need to own persistent vault data in `/var/opt/umrs/`.

**Ruling:** UMRS systemd units MUST use `User=umrs` (referencing the static `/etc/passwd`
entry), NOT `DynamicUser=yes`. Persistent data ownership requires a stable UID.

If a future UMRS unit performs only transient, stateless work, `DynamicUser=` can be
reconsidered. Requires explicit documentation and Jamie approval.

**Remediation owner:** coder (systemd unit files)

---

## SDR-SYSTEMD-002: NSS Resolvability — Local /etc/passwd Required

**Tension:** The systemd spec requires system users to resolve before `systemd-udevd.service`.
This prohibits LDAP/NIS for system accounts unless a local cache is maintained in the initrd.

**Ruling:** The `umrs` account MUST be created in local `/etc/passwd` via `useradd -r`.
UMRS installation scripts must never provision the `umrs` account through a directory
service. This is already the correct practice.

---

## SDR-SYSTEMD-003: Avoid UID Range Above 2^31

**Tension:** The range 2147483648–4294967294 is labeled "HIC SVNT LEONES" in the systemd
spec. Various kernel subsystems (`devpts`, `setfsuid()`) treat UIDs as signed integers and
fail with values ≥ 2^31.

**Ruling:** UMRS MUST NOT generate, store, or process UID values ≥ 2147483648. This
includes any future code that maps foreign UIDs or processes container identity data.
If such values are encountered in input, they must be rejected with an error, not silently
truncated or wrapped.

**Remediation owner:** coder — ensure any UID-processing Rust code validates against this bound.

---

## SDR-SYSTEMD-004: tty Group GID = 5

**Tension:** The `tty` group GID must be 5 on all systemd systems (hardcoded in `devpts`
mount parameters before NSS is available).

**Ruling:** If UMRS ever needs to add the `umrs` account to the `tty` group for terminal
access, verify GID 5 is indeed the `tty` group on the target system before scripting this.
Do not assume by name alone.

**RHEL 10 status:** `tty` group is GID 5 on standard RHEL 10. Verify after any RHEL 10
upgrade that modifies base group assignments.

---

## SDR-SYSTEMD-005: pkg-config Query for Boundary Verification

**Ruling:** UMRS install scripts that need to verify UID boundary compliance may use:
```sh
pkg-config --variable=system_uid_max systemd
```
This returns the compile-time (and optionally runtime-configured) system UID maximum.
On RHEL 10 this returns `999`. Scripts should compare this against the `SYS_UID_MAX`
in `/etc/login.defs` to detect any non-standard configuration.

This is more robust than hardcoding `999`. Implement if UMRS installs on non-RHEL systems.
