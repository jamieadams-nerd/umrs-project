# systemd file-hierarchy(7) -- Term Glossary

**Source:** systemd 257-23.el10 file-hierarchy(7), RHEL 10 | **Familiarized:** 2026-04-18

---

## Runtime and State Terms

### StateDirectory=
- **Definition:** systemd unit directive that causes systemd to create and manage
  /var/lib/package/ (or a specified path) with correct ownership and permissions before
  the service starts. The directory persists across service restarts.
- **Source:** file-hierarchy(7), PERSISTENT VARIABLE SYSTEM DATA section; systemd.exec(5)
- **Usage note:** Preferred over creating directories manually in install scripts for
  /usr/ system packages. For UMRS (an /opt package), equivalent outcome is achieved
  by the install script creating /var/opt/umrs/state/ with correct ownership.

### RuntimeDirectory=
- **Definition:** systemd unit directive that creates /run/package/ before service start
  and removes it after service stop. Contents are ephemeral.
- **Source:** file-hierarchy(7), RUNTIME DATA section; systemd.exec(5)
- **UMRS application:** If UMRS uses UNIX domain sockets, add RuntimeDirectory=umrs to the
  service unit. This creates /run/umrs/ automatically and removes it on service stop.

### CacheDirectory=
- **Definition:** systemd unit directive that creates /var/cache/package/ and manages
  ownership. Cache contents may be deleted without breaking the application.
- **Source:** file-hierarchy(7), PERSISTENT VARIABLE SYSTEM DATA section; systemd.unit(5)

### LogsDirectory=
- **Definition:** systemd unit directive for /var/log/package/. Equivalent to CacheDirectory
  for log directories.
- **Source:** file-hierarchy(7), PERSISTENT VARIABLE SYSTEM DATA section; systemd.exec(5)

### tmpfiles.d(5)
- **Definition:** systemd mechanism for creating, deleting, and cleaning up files and
  directories during boot. Can create directories in /run/, /var/, and elsewhere with
  specified permissions and ownership.
- **Source:** file-hierarchy(7), multiple sections
- **Usage note:** Alternative to RuntimeDirectory= for packages that need to pre-create
  runtime directories before the service unit starts (e.g., parent directories shared between
  multiple services).

---

## Path Terms

### $libdir
- **Definition:** The architecture-specific library directory. On aarch64 RHEL 10, typically
  /usr/lib/aarch64-linux-gnu/ or /usr/lib64/. Legacy locations: /usr/lib/, /usr/lib64/.
  Query with: systemd-path system-library-arch
- **Source:** file-hierarchy(7), VENDOR-SUPPLIED OS RESOURCES section
- **Usage note:** UMRS does not install public shared libraries, so $libdir is informational.

### $XDG_RUNTIME_DIR
- **Definition:** Per-user runtime directory. Each user's entry is under /run/user/uid/,
  individually mounted as tmpfs. Applications must reference this via the environment variable,
  not the path directly.
- **Source:** file-hierarchy(7), RUNTIME DATA /run/user/ section
- **Usage note:** UMRS is a system service; $XDG_RUNTIME_DIR applies to user-context
  applications only. Informational for UMRS.

### /usr/share/factory/
- **Definition:** Repository for vendor-supplied default configuration and variable data.
  /usr/share/factory/etc/ holds pristine vendor copies of /etc/ files;
  /usr/share/factory/var/ holds vendor copies of /var/ files. Used by tmpfiles.d to
  populate /etc/ and /var/ via "L" (symlink) or "C" (copy) directives.
- **Source:** file-hierarchy(7), VENDOR-SUPPLIED OS RESOURCES section
- **Usage note:** Useful for UMRS if shipped config files in /etc/opt/umrs/ should have
  a vendor-pristine reference. Consider /usr/share/factory/etc/opt/umrs/ for shipped defaults.

---

## Architecture Terms

### arch-id
- **Definition:** Architecture identifier string per the Multiarch Architecture Specifiers
  (Tuples) list. Appears in paths as /usr/lib/arch-id/.
- **Source:** file-hierarchy(7), VENDOR-SUPPLIED OS RESOURCES section

### Compatibility symlinks (systemd 257 / RHEL 10)
- /bin/ points to /usr/bin/
- /sbin/ points to /usr/bin/
- /usr/sbin/ points to /usr/bin/
- /lib/ points to /usr/lib/
- /lib64/ points to $libdir (architecture-dependent)
- /var/run/ points to /run/
- **Source:** file-hierarchy(7), COMPATIBILITY SYMLINKS section
- **Usage note:** Do not rely on these in new UMRS code. Use canonical paths.

---

## Security Terms

### nosuid / nodev (tmpfs mounts)
- **Definition:** Mount options applied to /tmp/, /var/tmp/, /dev/shm/. nosuid prevents
  set-user-id execution. nodev prevents device file interpretation. noexec is generally
  NOT applied because some software requires executable code in temp directories.
- **Source:** file-hierarchy(7), WRITE ACCESS section
- **Usage note:** UMRS must not create SUID files in temp directories.

### Node type discipline
- **Definition:** The systemd expectation that device nodes reside only in /dev/, and sockets
  and FIFOs reside only in /run/. "Applications should expect that a security policy might
  be enforced on a system that enforces these rules."
- **Source:** file-hierarchy(7), NODE TYPES section
- **Usage note:** This is an implicit SELinux reference. UMRS SELinux policy (umrs.te) should
  enforce this: socket files for UMRS daemons labeled for /run/umrs/ only, not for arbitrary
  locations.

---

## Where file-hierarchy(7) Is More Specific Than FHS 3.0

| Topic | FHS 3.0 says | file-hierarchy(7) adds |
|---|---|---|
| /run/user/uid/ | Not mentioned | tmpfs, use $XDG_RUNTIME_DIR, flushed on logout |
| /usr/share/factory/ | Not mentioned | Vendor-pristine config/var copies for tmpfiles.d population |
| State vs cache distinction | Not mentioned | Explicit: state cannot be deleted; cache can be deleted |
| systemd unit directives | Not mentioned | StateDirectory=, CacheDirectory=, RuntimeDirectory=, LogsDirectory= |
| /dev/shm/ security | Not mentioned | World-writable; prefer /run/ for sensitive shared memory |
| Node type enforcement | Not mentioned | Explicit expectation of SELinux-enforced node-type discipline |
| Compatibility symlinks | /var/run symlink noted | Full list: /bin, /sbin, /usr/sbin, /lib, /lib64, /var/run |

## Where file-hierarchy(7) Defers to FHS 3.0

- /opt package layout -- file-hierarchy(7) does not cover /opt; defers to FHS 3.0 section 3.13
- /etc/opt and /var/opt -- not mentioned in file-hierarchy(7); governed by FHS sections 3.7.4 and 5.12
- Traditional directories like /usr/include/, /var/spool/ -- explicitly out of scope
- FHS is cited in NOTES as reference [1]; systemd explicitly defers to it as the upstream standard
