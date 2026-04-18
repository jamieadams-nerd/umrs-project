# systemd UIDS-GIDS — Cross-Reference Map

**Familiarized:** 2026-04-18

---

## Agreements

### systemd ↔ LSB 5.0 §23

| Topic | systemd | LSB 5.0 §23 |
|---|---|---|
| Root = UID 0 | "root superuser" | "User ID and Group ID for root SHALL equal 0" |
| 0–99 = distro-static | 1–4 distro-assigned, 5 = tty (systemd), 6–999 distro-assigned | 0–99 shall not be created by applications |
| nobody = 65534 | Formally defined at 65534 | Listed in Table 23-2 (optional) without numeric assignment |
| System users in local database | Must resolve without network (pre-udevd) | Applications use POSIX API to read databases |

### systemd ↔ FHS 2.3

| Topic | systemd | FHS 2.3 |
|---|---|---|
| Runtime data location | `/run` (FHS 3.0 convention) | `/var/run` |
| System users in `/etc/passwd` | Must be locally resolvable | `/etc/passwd` referenced in §/etc (implied) |

Agreement on substance: system user identity must be local. Tension on path: `/run` vs `/var/run`.

---

## Tensions

### T1: systemd 1–999 vs LSB 100–499 dynamic range

**Most important tension in the corpus.**

**LSB 5.0** says dynamic system UIDs are 100–499.
**systemd** says system users are 1–999 (all of 1–999, with 1–4 and 6–999 shown as "Distributions").

The gap: UIDs 500–999 are within the systemd "system user" tier but outside LSB's stated
dynamic allocation range. However, LSB uses "should" — so this is a recommendation gap,
not a hard conflict.

**RHEL 10 resolution:** `SYS_UID_MIN=201`, `SYS_UID_MAX=999` — RHEL's implementation
spans both standards' ranges and resolves the gap in practice.

**For UMRS audits:** This is why three sources are required. Cite: LSB §23.3 (100–499 recommendation) + systemd (1–999 system boundary) + RHEL 10 `login.defs` (201–999 implementation).

### T2: `/run` vs `/var/run`

**systemd / RHEL 10:** Uses `/run` as a tmpfs mounted at early boot. `/var/run` is a
compatibility symlink to `/run`.

**FHS 2.3:** Specifies `/var/run` as the location for PID files and UNIX domain sockets.

**Resolution (SDR):** For UMRS on RHEL 10, use `/run/umrs/`. Either path works due to
the symlink, but `/run` is the canonical modern path. See `linux-fhs-2.3/style-decision-record.md` SDR-FHS-004.

### T3: UID 1 vs "0–99 static" — systemd's fine-grained breakdown

**LSB** says 0–99 are statically allocated. **systemd** breaks this down further:
- 0 = root (Linux)
- 1–4 = early distro-assigned daemons
- 5 = tty (systemd, GID only, but illustrates fine-grained assignment)
- 6–999 = distro-assigned system users

This is not a conflict but a precision refinement. systemd's breakdown is the more
operationally useful definition for RHEL 10 deployments.

---

## Chains

### Three-Source Compliance Chain (canonical for UMRS)

```
LSB 5.0 §23.3
  "UIDs 100–499 should be reserved for dynamic system allocation"
    ↓
systemd UIDS-GIDS
  "1–999 are system users; 999/1000 boundary strongly recommended"
    ↓
RHEL 10 /etc/login.defs
  SYS_UID_MIN=201, SYS_UID_MAX=999
    ↓
useradd -r umrs
  allocates from 201–999 on RHEL 10
    ↓
Compliance satisfied across all three sources
```

### NSS Resolution Chain

```
systemd-udevd.service startup
  requires system users resolvable
    ↓
UMRS umrs account in local /etc/passwd
  satisfies early-boot resolvability
    ↓
No LDAP/NIS dependency for umrs account
  compliant with systemd NSS requirement
```

### Container UID Alignment Chain

```
systemd --private-users=pick
  allocates from 524288–1879048191
    ↓
Container base UID has lower 16 bits = 0
  enables INTERNAL_UID = EXTERNAL_UID & 0xFFFF
    ↓
NSS check via getpwuid() + lckpwdf()
  race-free allocation
```

---

## Gaps

| Gap | Description |
|---|---|
| RHEL 10 reservation of 0–200 | systemd does not document the 0–200 static reservation that RHEL 10 implements (setting SYS_UID_MIN=201). This is a distro-specific extension beyond what systemd defines. |
| GID independence | systemd notes UIDs and GIDs are "mostly the same" but does not provide an equivalently detailed GID range table. GID 5 (tty) is the only systemd-defined hard GID assignment. |
| SELinux interaction | systemd UIDS-GIDS spec does not address how SELinux labels interact with UID-based privilege separation. This is a gap for high-assurance systems like UMRS. |
| `DynamicUser=` and persistent state | When `DynamicUser=` is used, the allocated UID is transient. systemd handles state directory ownership via `StateDirectory=` and `RuntimeDirectory=`. UMRS must not use `DynamicUser=` if the `umrs` account needs to own persistent vault data. |
| Subuid/subgid ranges | RHEL 10 `login.defs` defines `SUB_UID_MIN=524288`, `SUB_UID_MAX=600100000`. These overlap with systemd's container UID range (524288–1879048191). The systemd spec does not document this potential collision. |
