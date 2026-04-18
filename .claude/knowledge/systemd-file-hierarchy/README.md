# systemd file-hierarchy(7) -- Knowledge Collection

**Collection:** systemd-file-hierarchy
**Date of familiarization pass:** 2026-04-18
**Familiarized by:** Knox (security-auditor)
**Source:** .claude/references/systemd-file-hierarchy/file-hierarchy.txt
**Version:** systemd 257-23.el10 (RHEL 10 deployment target -- ground truth)

---

## Summary

file-hierarchy(7) is the systemd project's specification of file system layout for
systemd-managed systems. It describes a "generalized, minimal, modernized subset" of FHS 3.0,
extended with systemd-specific conventions. This document is the ground truth for what RHEL 10
actually does at runtime: which directories exist, when they are created, whether they are
ephemeral (tmpfs) or persistent, and how unprivileged processes interact with them.

Key additions over FHS 3.0: systemd unit directives (StateDirectory=, RuntimeDirectory=,
CacheDirectory=, LogsDirectory=), /usr/share/factory/ for vendor-pristine config defaults,
explicit state-vs-cache semantics, per-package slot tables (Tables 1-4), XDG user directory
integration, and node-type discipline with an explicit SELinux enforcement expectation.

file-hierarchy(7) does not cover /opt packages -- that domain belongs to FHS 3.0 sections
3.13, 3.7.4, and 5.12. The two documents complement each other without contradiction.

---

## Document Coverage

Documents processed: 1 (file-hierarchy.txt, full man page)

Sections:
- GENERAL STRUCTURE
- RUNTIME DATA (/run/, /run/log/, /run/user/)
- VENDOR-SUPPLIED OPERATING SYSTEM RESOURCES (/usr/ hierarchy)
- PERSISTENT VARIABLE SYSTEM DATA (/var/ hierarchy)
- VIRTUAL KERNEL AND API FILE SYSTEMS (/dev/, /proc/, /sys/)
- COMPATIBILITY SYMLINKS
- HOME DIRECTORY (XDG user conventions)
- WRITE ACCESS (unprivileged access rules)
- NODE TYPES (device/socket discipline)
- SYSTEM PACKAGES (Tables 1 and 2)
- USER PACKAGES (Tables 3 and 4)

---

## Artifact Files

| File | Description |
|---|---|
| concept-index.md | Section-by-section summaries; UMRS implications; comparison table (more specific than FHS / defers to FHS) |
| term-glossary.md | StateDirectory=, RuntimeDirectory=, CacheDirectory=, LogsDirectory=, tmpfiles.d, $libdir, $XDG_RUNTIME_DIR, node type discipline |
| cross-reference-map.md | Agreements/tensions with FHS 3.0 and systemd UIDS-GIDS; chains; gaps (notably SELinux label gap for RuntimeDirectory-created dirs) |
| style-decision-record.md | Five SDRs: /var/opt/ subdivision, RuntimeDirectory=, SELinux node discipline, Table 2 scope boundary, factory/ deferral |

---

## Notable Findings

1. **/opt package paths are out of scope for file-hierarchy(7) Tables 1-2.** The slot tables
   apply to /usr/ system packages. Any agent citing Table 2 for UMRS path justifications is
   making a category error. FHS 3.0 sections 3.13/3.7.4/5.12 govern UMRS.

2. **RuntimeDirectory=umrs is the right mechanism for /run/umrs/.** This eliminates the need
   for the install script to manage the runtime directory and ensures it is always created
   with correct ownership before the service starts.

3. **Node type discipline is a SELinux enforcement expectation.** file-hierarchy(7) explicitly
   anticipates that a security policy may enforce device nodes in /dev/ only and sockets/FIFOs
   in /run/ only. UMRS umrs.te policy should implement this proactively.

4. **SELinux labeling gap with RuntimeDirectory=.** When systemd creates /run/umrs/, it does
   not run restorecon. If fcontext rules are loaded, the directory inherits the correct label
   from the parent /run/ type transition rules -- but only if the policy is loaded and the
   transition is defined. The UMRS install script should verify this or include a restorecon
   pass.

5. **State vs. cache semantics are now explicit.** file-hierarchy(7) makes the distinction
   operational: cache can be deleted without consequence; state cannot. UMRS /var/opt/umrs/
   subdirectory design should honor this distinction.

---

## Open Questions

- SDR-FH-005: /usr/share/factory/ for UMRS default config -- deferred to packaging phase
- SELinux label gap for RuntimeDirectory-created /run/umrs/ -- needs verification during
  service unit testing
