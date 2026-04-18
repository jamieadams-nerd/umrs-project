# FHS 2.3 — Cross-Reference Map

**Familiarized:** 2026-04-18

---

## Agreements

### FHS 2.3 ↔ LSB 5.0

| Topic | FHS 2.3 | LSB 5.0 |
|---|---|---|
| `/opt` as add-on location | Ch 3.13 — required | LSB references FHS for filesystem layout |
| `/etc/opt/<pkg>` for config | Ch 3.5.1 — required | LSB defers to FHS |
| `/var/opt/<pkg>` for variable data | Ch 5.10 — required | LSB defers to FHS |
| UID/GID allocation | Not covered | §23 — LSB's domain exclusively |

Both documents were maintained by the Linux Foundation (LSB absorbed FHS). LSB explicitly
defers filesystem hierarchy questions to FHS.

### FHS 2.3 ↔ systemd UIDS-GIDS

| Topic | FHS 2.3 | systemd |
|---|---|---|
| Runtime data location | `/var/run` (§5.11) | `/run` (FHS 3.0 convention) |
| System user residency | `/etc/passwd` (implied by §3.5) | Local `/etc/passwd` for early-boot resolution |

---

## Tensions

### T1: `/var/run` vs `/run`

**FHS 2.3** specifies `/var/run` for runtime data (PID files, sockets).
**systemd** (and RHEL 10 by extension, following FHS 3.0) uses `/run` as a top-level
tmpfs mount. On RHEL 10, `/var/run` is a symlink to `/run`.

**Impact on UMRS:** If UMRS creates runtime socket or PID files, the code should target
`/run/umrs/` not `/var/run/umrs/`. The skill (`fhs-lsb-uid-gid`) does not currently
document this. Either path works on RHEL 10 (symlink), but `/run` is the correct
canonical form for FHS 3.0 systems.

**Resolution needed:** See `style-decision-record.md` — awaiting FHS version decision.

### T2: LANANA Provider Registration

**FHS 2.3** §3.13 states: "A package to be installed in /opt must locate its static files
in a separate `/opt/<package>` or `/opt/<provider>` directory tree, where `<provider>` is
the provider's LANANA registered name."

**UMRS uses `/opt/umrs/`** — `umrs` is not a LANANA-registered provider name.
Using the package name form (`/opt/<package>`) rather than the provider form (`/opt/<provider>`)
is acceptable, and the `<package>` name does not require LANANA registration. However, the
skill and documentation should clarify which form is being used.

**Status:** Low risk. Using the `<package>` form is clearly permitted. No registration needed
for that form.

### T3: FHS Section Numbers in Skill

The `fhs-lsb-uid-gid` skill cites:
- "FHS 3.13 (/opt)" for `/opt/umrs/`
- "FHS 3.8 (/etc/opt)" for `/etc/opt/umrs/`
- "FHS 5.12 (/var/opt)" for `/var/opt/umrs/`
- "FHS 4.5 vs NIST CM-6/SA-8" for `/etc/keys/umrs/`

**FHS 2.3 actual section numbers:**
- `/opt` is in **Chapter 3** with no numbered subsections in the text (unnumbered within Ch 3)
- `/etc/opt` is in Chapter 3, subsection labeled "/etc/opt : Configuration files for /opt"
- `/var/opt` is in Chapter 5, subsection labeled "/var/opt : Variable data for /opt"
- FHS 2.3 uses no decimal section numbers like "3.13" or "5.12"

The skill's section numbers appear to be FHS 3.0 section numbers applied to FHS 2.3 content.
This is a citation accuracy gap.

---

## Chains

### FHS → LSB → systemd → RHEL 10

FHS defines filesystem layout.
LSB incorporates FHS by reference for filesystem questions.
systemd builds on FHS 3.0 conventions (adding `/run`).
RHEL 10 implements systemd conventions (`/run` tmpfs, symlink from `/var/run`).

The compliance chain for UMRS path decisions:
`FHS 2.3/3.0 §/opt` → `LSB 5.0 (defers to FHS)` → `RHEL 10 package conventions`

### FHS /etc/opt → justification for `/etc/opt/umrs/`

FHS 2.3 §/etc/opt: "Host-specific configuration files for add-on application software
packages must be installed within the directory `/etc/opt/<subdir>`."
This is the direct normative basis for `/etc/opt/umrs/`.

### FHS /var/opt → justification for `/var/opt/umrs/`

FHS 2.3 §/var/opt: "Variable data of the packages in /opt must be installed in
`/var/opt/<subdir>`." Direct normative basis for `/var/opt/umrs/`.

---

## Gaps

| Gap | Description |
|---|---|
| UID/GID allocation | FHS 2.3 does not address user/group identity numbers. Go to LSB §23 and systemd spec. |
| SELinux labels | FHS 2.3 predates SELinux. No guidance on labeling paths. |
| `/run` | FHS 2.3 specifies `/var/run`. FHS 3.0 formalizes `/run`. Corpus gap. |
| Cryptographic key material paths | FHS 2.3 has no guidance on key storage conventions. The `/etc/keys/` location is UMRS-specific. |
| Immutable filesystems | FHS 2.3 does not address OSTree, read-only `/usr`, or image-based deployments common in RHEL 10. |
