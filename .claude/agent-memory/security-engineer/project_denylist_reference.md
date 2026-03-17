---
name: env_denylist_rhel10
description: Canonical glibc AT_SECURE / ld.so secure-mode env var denylist for RHEL 10 (glibc 2.39)
type: project
---

When reviewing env scrubbing designs, verify ALL of these are in the unconditional strip tier.

**glibc ld.so secure-execution mode (ld.so man page + glibc manual):**
```
LD_PRELOAD, LD_LIBRARY_PATH, LD_AUDIT, LD_DEBUG, LD_BIND_NOW, LD_BIND_NOT,
LD_PROFILE, LD_PROFILE_OUTPUT, LD_VERBOSE, LD_SHOW_AUXV, LD_ORIGIN_PATH,
LD_DYNAMIC_WEAK, LD_USE_LOAD_BIAS
LOCPATH, GCONV_PATH, NLSPATH, GETCONF_DIR
MALLOC_CHECK_, MALLOC_TRACE
TZDIR
GLIBC_TUNABLES  ← RHEL 10 specific: glibc 2.35+; CVE-2023-4911 (heap overflow, privesc)
```

**Why TZDIR matters:** `validate_tz()` blocks traversal in the TZ value itself. But if TZDIR
is inherited and attacker-controlled, glibc redirects all zoneinfo file lookups regardless of
the TZ value. TZDIR must be stripped BEFORE TZ validation runs for TZ validation to be meaningful.

**Additional userspace interpreter vars (not glibc, still dangerous):**
```
PYTHONPATH, PYTHONSTARTUP, PERL5LIB, PERL5OPT, RUBYLIB, RUBYOPT,
NODE_PATH, NODE_OPTIONS, CDPATH, ENV, BASH_ENV, HISTFILE,
HOSTALIASES, RESOLV_HOST_CONF, NIS_PATH, IFS, POSIXLY_CORRECT
```

**IFS placement:** IFS belongs in the unconditional strip tier, NOT the reset tier.
Rust processes do not use IFS; it only matters for spawned shells.

Source: umrs-tool-init plan security review, 2026-03-17.
Report: .claude/reports/2026-03-17-umrs-tool-init-security-review.md
