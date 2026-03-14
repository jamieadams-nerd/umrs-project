# Security Engineer Persistent Memory

## Policy naming conventions
- No SELinux policy modules have been authored yet (platforms/rhel10/ is empty placeholder)
- When the first module is needed, use `umrs_` prefix for all types: umrs_exec_t, umrs_conf_t, umrs_log_t, umrs_var_t
- CIL format preferred over .te/.fc for new modules (RHEL 10 ships CIL-capable semodule)

## Known type/architecture decisions

### posture module (2026-03-14)
- All /proc/ and /sys/ reads in posture module correctly route through SecureReader (PROC_SUPER_MAGIC / SECURITYFS_MAGIC verified)
- SignalId::Lockdown is classified as KernelCmdline in catalog but reads from /sys/kernel/security/lockdown (securityfs) — mismatch documented in finding, not yet resolved
- parse_sysctl_u32 uses u32 — perf_event_paranoid can legitimately emit "-1"; this causes silent live_value:None degradation (HIGH finding, unresolved as of 2026-03-14)
- parse_sysctl_line is private in configured.rs — cannot be unit tested directly from tests/; needs pub(crate) or a from_reader constructor
- Slash-vs-dot key normalization in sysctl.d is a known gap (MEDIUM, unresolved); sysctl.d files using kernel/kptr_restrict will not match catalog key kernel.kptr_restrict

## Modules with known security debt

| Component | Gap | Severity | Status |
|---|---|---|---|
| posture/reader.rs | perf_event_paranoid: u32 parser fails on -1 | High | Open |
| posture/configured.rs | slash-to-dot key normalization missing | Medium | Open |
| posture/snapshot.rs | collect(), iter(), findings(), contradictions(), by_impact() missing #[must_use] | Low | Open |
| posture/catalog.rs | Lockdown SignalClass::KernelCmdline mismatch | Low | Open |
| posture_demo.rs | .expect() call (policy violation) | Low | Open |

## Review patterns observed across umrs-platform

- SecureReader engine is consistently applied; no raw File::open on /proc/ or /sys/ observed
- #[must_use] is generally well-applied on Result-returning functions; iterator methods are the consistent gap
- Log messages at debug level in configured.rs emit raw sysctl.d key=value pairs — flag in future reviews
- kattrs MAX_KATTR_READ = 64 bytes — adequate for all current sysctl reads; watch for longer values if new signals are added (esp. /proc/cmdline-derived ones)

## References
- Detailed findings: .claude/reports/security-review-umrs-platform-posture.md
- Previous OS detection audit: .claude/reports/2026-03-11-os-detection-umrs-platform.md
