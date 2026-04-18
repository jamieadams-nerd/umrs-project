# FHS 3.0 — Cross-Reference Map

**Familiarized:** 2026-04-18

---

## Agreements

### A1: /opt layout (FHS 3.0 §3.13 ↔ systemd file-hierarchy(7) SYSTEM PACKAGES table)

Both agree that `/opt/<package>/` holds static vendor data and that variable data goes
elsewhere. FHS 3.0 directs variable data to `/var/opt/<subdir>` and config to `/etc/opt/<subdir>`.
systemd `file-hierarchy(7)` Table 1 directs config to `/etc/package/`, runtime data to
`/run/package/`, cache to `/var/cache/package/`, state to `/var/lib/package/`. These are
not contradictory: FHS applies specifically to `/opt` add-on packages; systemd's table applies
to system packages installed in `/usr/`. UMRS falls squarely under FHS §3.13 as an `/opt`
package. Use FHS §3.13 for UMRS path justifications.

### A2: /run primacy (FHS 3.0 §3.15 ↔ systemd file-hierarchy(7) RUNTIME DATA section)

Both agree that `/run` is the canonical location for runtime data, PID files, and sockets.
systemd `file-hierarchy(7)` describes `/run/` as a tmpfs for system packages. FHS 3.0 §3.15
formalizes this as a required top-level directory. Strong agreement — cite either or both.

### A3: /var/run is a compatibility symlink (FHS 3.0 §5.13 ↔ systemd file-hierarchy COMPATIBILITY SYMLINKS)

FHS 3.0 §5.13 explicitly states `/var/run` exists only for compatibility; functions have moved
to `/run`. systemd `file-hierarchy(7)` COMPATIBILITY SYMLINKS section confirms `/var/run/`
points to `/run/`. Complete agreement: never use `/var/run` in new UMRS code.

---

## Tensions

### T1: /opt package runtime data — /var/opt vs /var/lib vs /run (FHS 3.0 vs systemd file-hierarchy)

**FHS 3.0 §3.13 / §5.12:** Variable data for `/opt` packages must be in `/var/opt/<subdir>`.

**systemd file-hierarchy(7) Table 2 (SYSTEM PACKAGES):** Recommends `/var/lib/package/` for
persistent private data, `/var/cache/package/` for cache, `/run/package/` for runtime data.
This table does NOT address `/opt` packages specifically — it addresses system packages under
`/usr/`.

**Resolution:** For UMRS (an `/opt` package per FHS §3.13), use `/var/opt/umrs/` as the
variable data root per FHS §5.12. Within that tree, UMRS can apply the state/cache/log
subdivision that systemd recommends. Example: `/var/opt/umrs/state/`, `/var/opt/umrs/cache/`,
`/var/opt/umrs/log/`. This satisfies both standards without conflict.

### T2: /usr/libexec (FHS 3.0 §4.7, optional) vs systemd /usr/lib/<package>/ convention

FHS 3.0 §4.7 adds `/usr/libexec` as optional for internal binaries. systemd `file-hierarchy(7)`
recommends `/usr/lib/package/` for private static vendor resources including private binaries.
RHEL 10 in practice uses both. This is a style tension, not a compliance issue. For UMRS
(an `/opt` package), this does not apply — UMRS internal binaries go under `/opt/umrs/`.

### T3: FHS 3.0 §4.5 (/usr/include) cited in skill for key material rationale

**Skill claim:** "FHS Section 4.5 says `/etc/opt/<package>/` is for add-on package configuration.
Key material is not configuration — it is cryptographic trust anchors."

**FHS 3.0 reality:** §4.5 covers `/usr/include` (C header files). It says nothing about key
material or `/etc/opt`. The correct FHS basis for the `/etc/keys/umrs/` exception is §3.13.2's
exception clause: "No other package files may exist outside the /opt, /var/opt, and /etc/opt
hierarchies **except for those package files that must reside in specific locations within the
filesystem tree in order to function properly**." Key material qualifies under this clause.

**Severity:** MEDIUM citation error in the skill. The underlying logic (key material exception)
is sound; the §4.5 citation is wrong. See SDR-FHS-003 in this collection's style-decision-record.

---

## Chains

### C1: FHS 3.0 → systemd file-hierarchy(7)

systemd `file-hierarchy(7)` explicitly cites FHS 3.0 in its NOTES section (reference [1]).
systemd extends and refines FHS for systemd-managed systems, adding credentials directories,
state vs. cache separation, and XDG user directory conventions. FHS is the base layer;
systemd adds systemd-specific detail on top. For any path not covered by systemd, defer to FHS.

### C2: FHS 3.0 §3.13 → FHS 3.0 §3.7.4 → FHS 3.0 §5.12

The `/opt` section (§3.13) explicitly cross-references `/etc/opt` and `/var/opt`:
- "Host-specific configuration files must be installed in `/etc/opt`. See the section on /etc."
- "Package files that are variable must be installed in `/var/opt`. See the section on /var/opt."
These three sections form a mandatory triad. Citing §3.13 alone is insufficient for a complete
UMRS path compliance justification — all three must be in scope.

### C3: FHS 3.0 §3.15 (/run) → FHS 3.0 §5.13 (/var/run deprecation)

§3.15 defines `/run` as the new canonical directory. §5.13 explains why `/var/run` still exists
(backward compatibility) and explicitly permits implementing it as a symlink to `/run`. These
two sections must be cited together when justifying the `/run` vs `/var/run` choice.

---

## Gaps

### G1: Key material placement

FHS 3.0 does not address cryptographic key material specifically. The nearest applicable text is
the §3.13.2 exception clause. NIST SP 800-53 CM-6 and SA-8 provide the security engineering
rationale for keeping key material in a separately hardened path. FHS provides permission (via
exception clause); NIST provides motivation. Both citations are required in UMRS documentation.

### G2: SELinux labels

Neither FHS 3.0 nor systemd `file-hierarchy(7)` addresses SELinux fcontext labeling. This is
governed entirely by the `selinux-rules` skill and UMRS policy.

### G3: Subordinate UID/GID for file ownership

FHS 3.0 says nothing about who owns the files numerically. Defer to LSB 5.0 §23 and systemd
UIDS-GIDS for ownership decisions. See `fhs-lsb-systemd-CHAIN.md`.

### G4: /etc/keys

FHS 3.0 does not define `/etc/keys`. This path follows IMA/EVM tooling convention and is
outside the FHS-governed hierarchy by justified exception (§3.13.2). Document explicitly in
deployment docs to preempt auditor questions.
