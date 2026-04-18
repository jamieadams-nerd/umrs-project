# systemd file-hierarchy(7) -- Style Decision Record

**Familiarized:** 2026-04-18

---

## SDR-FH-001: /var/opt/umrs/ internal subdivision convention

**Tension:** FHS 3.0 section 5.12 says variable data for /opt packages goes in /var/opt/package/
with no imposed internal structure. file-hierarchy(7) provides a state/cache/log subdivision
convention for /var/lib/, /var/cache/, /var/log/ that is well-understood by systemd and operators.

**Ruling:** Adopt the systemd subdivision convention inside /var/opt/umrs/:
- /var/opt/umrs/state/ -- persistent state data (mirrors /var/lib/package/ semantics)
- /var/opt/umrs/cache/ -- regenerable cache (mirrors /var/cache/package/ semantics)
- /var/opt/umrs/log/ -- persistent logs (mirrors /var/log/package/ semantics)
- /var/opt/umrs/vaults/ -- vault data (UMRS-specific; no systemd analogue)

This satisfies FHS 3.0 (all under /var/opt/umrs/) and makes the semantics legible to
operators familiar with the systemd convention.

**Citation:** FHS 3.0 section 5.12 (authority), file-hierarchy(7) Table 2 (convention)
**Remediation owner:** tech-writer (document in directory-purpose-matrix.adoc)

---

## SDR-FH-002: RuntimeDirectory= for /run/umrs/

**Ruling:** UMRS systemd service units should use RuntimeDirectory=umrs to create /run/umrs/
automatically. Do not pre-create /run/umrs/ in the install script (it will be flushed at boot
anyway). tmpfiles.d(5) is acceptable as an alternative for parent directory creation needs.

**Citation:** file-hierarchy(7), SYSTEM PACKAGES Table 2, /run/package/ row
**Remediation owner:** coder (service unit configuration)

---

## SDR-FH-003: Node type discipline in SELinux policy

**Ruling:** UMRS SELinux policy must enforce node type discipline: socket files for UMRS
daemons must be labeled with types that are only valid under /run/umrs/. The file-hierarchy(7)
NODE TYPES section explicitly anticipates security policy enforcement of this rule. UMRS should
be proactive, not reactive, on this point.

**Citation:** file-hierarchy(7), NODE TYPES section
**Remediation owner:** coder (umrs.te policy)

---

## SDR-FH-004: Do not cite file-hierarchy(7) Table 2 for /opt package paths

**Ruling:** Table 2 in file-hierarchy(7) defines slots for packages installed under /usr/.
UMRS is an /opt package. Citing Table 2 for UMRS path justifications is incorrect -- it applies
to a different package installation model. Always cite FHS 3.0 section 3.13 (opt), section 3.7.4
(etc/opt), and section 5.12 (var/opt) for UMRS path decisions.

**Remediation owner:** all agents (when writing compliance documentation)

---

## SDR-FH-005: /usr/share/factory/ for UMRS default config -- deferred

**Context:** file-hierarchy(7) defines /usr/share/factory/etc/ as a location for vendor-pristine
configuration defaults. This could be used to ship default /etc/opt/umrs/ configuration.

**Ruling:** Deferred to Phase 2 / packaging work. Current UMRS install approach (install script
creates /etc/opt/umrs/) is sufficient for Phase 1. Flag for consideration when building an RPM
package.

**Remediation owner:** coder (future packaging pass)
