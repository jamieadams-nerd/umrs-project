# systemd file-hierarchy(7) -- Cross-Reference Map

**Familiarized:** 2026-04-18

---

## Agreements with FHS 3.0

### A1: /run is canonical for runtime data
Both FHS 3.0 section 3.15 and file-hierarchy(7) RUNTIME DATA agree that /run/ is the correct
location for PID files, sockets, and runtime data. Both agree /var/run/ is a compatibility
symlink only.

### A2: /var/run is a compatibility symlink
FHS 3.0 section 5.13 and file-hierarchy(7) COMPATIBILITY SYMLINKS agree on this exactly.
file-hierarchy(7) names it explicitly in the symlink table.

### A3: State/cache/log separation under /var/
FHS 3.0 defines /var/lib/ (state), /var/cache/, /var/log/ as distinct hierarchy sections.
file-hierarchy(7) reinforces the distinction: state cannot be flushed without consequence;
cache can be flushed; logs should prefer journald. Both agree on the three-way separation.

### A4: /tmp/ requires mkstemp/mkdtemp
Both FHS 3.0 and file-hierarchy(7) explicitly require mkstemp(3)/mkdtemp(3) for temp file
creation due to world-write access. file-hierarchy(7) cites "Using /tmp/ and /var/tmp/ Safely"
as a reference.

---

## Agreements with systemd UIDS-GIDS

### A5: System service unprivileged write access pattern
file-hierarchy(7) WRITE ACCESS confirms that unprivileged system processes have writable
access only to /tmp/, /var/tmp/, /dev/shm/. For private writable directories in /var/ or
/run/, the process must create them before dropping privileges, or use tmpfiles.d(5), or use
StateDirectory=/RuntimeDirectory= in the service unit. This is consistent with systemd
UIDS-GIDS guidance on DynamicUser and system account directory management.

---

## Tensions

### T1: /opt package variable data -- /var/opt/ vs /var/lib/
**FHS 3.0 section 5.12:** Variable data for /opt packages must go in /var/opt/package/.
**file-hierarchy(7) Table 2:** Recommends /var/lib/package/ as the primary persistent data
slot for system packages.

**Resolution:** Table 2 applies to /usr/ system packages. UMRS is an /opt package.
FHS section 5.12 is authoritative for UMRS. Within /var/opt/umrs/, the internal
subdivision (/var/opt/umrs/state/, /var/opt/umrs/cache/, /var/opt/umrs/log/) can mirror
the systemd convention without violating either standard. Use /var/opt/umrs/ as the root;
adopt systemd-style subdivision internally.

### T2: /etc/package/ (file-hierarchy Table 2) vs /etc/opt/package/ (FHS section 3.7.4)
**FHS 3.0 section 3.7.4:** /opt package configuration must be in /etc/opt/package/.
**file-hierarchy(7) Table 2:** Recommends /etc/package/ for system packages.

**Resolution:** Same as T1 -- Table 2 applies to /usr/ system packages, not /opt packages.
UMRS uses /etc/opt/umrs/ per FHS section 3.7.4. Not a conflict; different scopes.

### T3: /run/package/ creation (file-hierarchy vs FHS)
FHS 3.0 section 3.15 does not specify who creates /run/package/. file-hierarchy(7) Table 2
clarifies: "Packages must be able to create the necessary subdirectories in this tree on their
own, since the directory is flushed automatically on boot." Alternatives: tmpfiles.d(5) fragment
or RuntimeDirectory= directive.

**Resolution:** For UMRS, use RuntimeDirectory=umrs in the systemd service unit. This is
the preferred mechanism and removes the need for the install script to handle this.

---

## Chains

### C1: FHS 3.0 -> file-hierarchy(7) -> systemd UIDS-GIDS
file-hierarchy(7) explicitly cites FHS 3.0 as reference [1]. systemd UIDS-GIDS (a companion
document from the same project) covers the UID/GID side of what file-hierarchy(7) covers on
the path side. Together, they define the full systemd deployment model. FHS 3.0 is the
upstream standard; both systemd documents refine and extend it for systemd-managed systems.

### C2: file-hierarchy(7) NODE TYPES -> SELinux policy
"Applications should expect that a security policy might be enforced on a system that enforces
these rules [node types in designated directories]." This defers to SELinux for actual enforcement.
UMRS umrs.te policy should implement this discipline: UMRS socket files labeled with types
valid only under /run/umrs/.

---

## Gaps

### G1: /opt package runtime directory creation
Neither file-hierarchy(7) nor FHS 3.0 specifies who is responsible for creating
/run/opt-package/ subdirectories. The convention from file-hierarchy(7) Table 2 for /run/package/
applies: use RuntimeDirectory= or tmpfiles.d(5). UMRS should adopt RuntimeDirectory=umrs in
its service unit.

### G2: /usr/share/factory/ for /etc/opt/ population
file-hierarchy(7) defines /usr/share/factory/etc/ for vendor-pristine configuration. It does
not explicitly address whether this applies to /etc/opt/ config files. By analogy, UMRS could
ship pristine config defaults in /usr/share/factory/etc/opt/umrs/ and use tmpfiles.d to
copy them to /etc/opt/umrs/ on first boot. This is a future consideration, not a current gap.

### G3: /var/opt/ subdirectory state vs cache semantics
Neither FHS 3.0 nor file-hierarchy(7) specifies what goes inside /var/opt/package/. The
systemd convention from /var/lib/ and /var/cache/ (state cannot be deleted; cache can) is
not formally extended to /var/opt/. UMRS must define this internally. See SDR-FH-001 in
style-decision-record.md.

### G4: SELinux labels for systemd-created directories
When RuntimeDirectory=umrs creates /run/umrs/, it does so without restorecon. If the
fcontext rule exists, the directory should receive the correct label. But if the service
starts before policy is loaded (unusual on RHEL 10, but possible during install), labels
may be wrong. The install script should include a restorecon pass on /run/umrs/ if it
exists, or ensure the service unit runs after policy load. This is a UMRS-specific gap.
