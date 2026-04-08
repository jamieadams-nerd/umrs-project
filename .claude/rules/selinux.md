## SELinux Architectural Rules

Applies when working with `umrs-selinux`, security contexts, MLS/MCS levels,
or SELinux policy.

### Axioms

[AXIOM] SELinux access decisions are based on security labels, not file paths.
[AXIOM] SELinux is always upper case 'S', uppercase 'E', and uppercase 'L' followed by 'inux'
[AXIOM] When SELinux is expanded. It's always "Security-Enhanced Linux" with a dash. Always.

[AXIOM] Targeted policy has exactly one sensitivity level: `s0`. All MCS
categories exist at `s0`. There is no `s1`, `s2`, or `s3` in targeted policy.

[AXIOM] In targeted policy, SELinux enforces type enforcement only. MCS
categories are advisory labels, not enforcement boundaries.

[AXIOM] MLS policy adds Bell-LaPadula dominance enforcement across sensitivity
levels (`s0`–`s3`). Categories become enforcement-relevant only under MLS.

[AXIOM] A security context has the form `user:role:type:level`, where level
is `sensitivity[:category_set]` or `low_level-high_level` (a range).

### Constraints

[CONSTRAINT] UMRS Phase 1 is targeted policy only. Do not implement MLS
enforcement logic in Phase 1 code. Labeling fidelity and awareness — not
enforcement — is the Phase 1 deliverable.

[CONSTRAINT] UMRS must not execute external binaries (e.g., `chcon`, `semanage`,
`restorecon`) for trust decisions. All SELinux state must be read from kernel
interfaces (`/sys/fs/selinux/`, `/proc/`, xattrs) or parsed from policy files.

### Rules

[RULE] Gate configuration file reads behind a kernel status check. Only read
SELinux config files if the kernel confirms SELinux is enabled. If the kernel
says disabled, return `None` — do not guess from config.

[RULE] Use `security context` in documentation and UI, not `label` or `security label`.
Use `sensitivity level`, not `sensitivity label`.

[RULE] `SelinuxUser` and `SelinuxRole` are lowercase `[a-z0-9_]` only.
`SelinuxType` allows mixed case.

### Patterns

[PATTERN] Use `restorecon` after applying `fcontext` rules to ensure consistent labeling.

[PATTERN] Read SELinux state from `/sys/fs/selinux/` (kernel truth) rather than
`/etc/selinux/config` (admin intent). The kernel is authoritative.

### Anti-Patterns

[ANTI-PATTERN] Do not rely on `chcon` for persistent file labeling — it does
not survive relabeling operations. Use `semanage fcontext` + `restorecon`.

[ANTI-PATTERN] Do not assume a file's SELinux type from its path. Always read
the actual xattr (`security.selinux`).
