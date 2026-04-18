# LSB 5.0 — Cross-Reference Map

**Familiarized:** 2026-04-18

---

## Agreements

### LSB 5.0 §23 ↔ systemd UIDS-GIDS

| Topic | LSB 5.0 §23 | systemd |
|---|---|---|
| 0 = root | root UID and GID SHALL equal 0 (§23.2) | 0 = root superuser |
| 1–99 = static | "should be statically allocated by the system" (§23.3) | 1–999 = system users (distro-assigned) — LSB's 1–99 is a subset |
| 100–499 = dynamic system | "should be reserved for dynamic allocation" (§23.3) | Subsumed into 100–999 dynamic system range |
| No login for system accounts | Implied by daemon rationale (§23.2 note b) | Explicit: system users do not log in |
| nobody at 65534 | Listed as optional name in Table 23-2 | Formally defined at 65534 as "overflow" UID |

### LSB 5.0 §23 ↔ FHS 2.3

No direct conflict. LSB and FHS were co-maintained by the Linux Foundation. LSB defers all
filesystem layout questions to FHS. FHS defers all UID/GID questions to LSB §23.

### LSB 5.0 §23 ↔ RHEL 10 `login.defs`

| Topic | LSB 5.0 §23 | RHEL 10 `login.defs` |
|---|---|---|
| Static system UIDs | 0–99 | 0–200 (RHEL extends, reserves more) |
| Dynamic system UIDs | 100–499 ("should") | 201–999 (`SYS_UID_MIN=201`, `SYS_UID_MAX=999`) |
| Regular users | 500+ (undefined above 499) | 1000–60000 (`UID_MIN=1000`, `UID_MAX=60000`) |

---

## Tensions

### T1: LSB "should" vs. systemd hard boundaries

**LSB 5.0 §23.3** uses "should" (RFC 2119: recommended but not required) for the 100–499
dynamic system range. An implementor could legally use a different range.

**systemd UIDS-GIDS** specifies 1–999 as the system user range and explicitly discourages
changing the 999/1000 boundary. This is a de facto hard requirement for systemd-based distros.

**Impact:** When citing compliance, LSB alone is weak evidence. The three-source chain
(LSB + systemd + RHEL 10 `login.defs`) is required to show that `useradd -r` will produce
the correct result on RHEL 10.

**Resolution:** Three-source compliance report (`2026-04-17-uid-gid-compliance-reference.md`)
correctly handles this tension. Do not cite LSB alone.

### T2: LSB 100–499 vs systemd/RHEL 201–999

**LSB 5.0** says dynamic system range is 100–499.
**RHEL 10** implements dynamic allocation as 201–999 (via `SYS_UID_MIN=201`, `SYS_UID_MAX=999`).

The RHEL 10 range is a strict superset and extension of LSB's range:
- RHEL 201–499: within LSB's stated range (compliant)
- RHEL 500–999: beyond LSB's definition, filled by systemd convention
- RHEL 100–200: reserved for static distro accounts (more conservative than LSB's 0–99)

**Impact:** A `umrs` account allocated at, say, UID 350 would be in both the LSB-defined
and RHEL-defined dynamic range. A `umrs` account at UID 700 would be in the RHEL-defined
range but NOT in LSB's 100–499 range. Since LSB uses "should" this is not a violation, but
it weakens the pure-LSB citation.

**Resolution:** Always cite all three sources together. The compliance report handles this correctly.

### T3: LSB §23.2 `operator` group = `root`

**LSB 5.0 Table 23-2** lists the `operator` account's group as `root`, not `operator`.
This is counterintuitive and worth noting explicitly for auditors who may expect a dedicated
group. This is LSB's definition; it takes precedence.

---

## Chains

### §23.3 Deference Chain

LSB §23.3 → systemd UIDS-GIDS (extends 499 to 999) → RHEL 10 `login.defs` (implements as 201–999)

### §23.2 Name Conflict Avoidance Chain

LSB §23 Tables 23-1 and 23-2 → UMRS account naming decision → `umrs` confirmed not in either table

### LSB Filesystem Deference

LSB Core §§ on filesystem → FHS 2.3/3.0 (explicit deference for all path conventions)

---

## Gaps

| Gap | Description |
|---|---|
| 500–999 range | LSB §23.3 says nothing about UIDs 500–999. systemd fills this gap. |
| Password locking | LSB §23 does not require locked passwords for system accounts. Systemd spec and security best practice fill this gap. |
| Login shell | LSB §23 does not prohibit login shells for system accounts. Systemd spec and security best practice fill this gap. |
| Home directories | LSB §23 says the location of user home directories is not defined by policy "other than the recommendations of FHS." No prohibition on system accounts having home directories. |
| 32-bit UID range above 65535 | LSB §23 does not define container UIDs or the extended 32-bit range. Systemd fills this entirely. |
| GID allocation | LSB §23 is primarily about UIDs. GIDs are stated to follow the same policy ("in mostly the same way" per systemd; implied by LSB's parallel tables). |
