---
name: Environment sanitization familiarization
description: Key facts from the 2026-04-12 env-attack-prevention report and env_sanitization_rules.md, internalized for umrs-core::init / SanitizedEnv / ScrubReport implementation
type: project
---

## Source documents

- Report: `.claude/references/reports/2026-04-12-env-attack-prevention.md` (928 lines, 2026-04-12)
- Rules: `.claude/rules/env_sanitization_rules.md` (distilled Tier list, CWE↔NIST, patterns)
- Plan context: `.claude/plans/umrs-tool-init.md` Phase 3

## The central constraint — Rust 2024 Edition

`std::env::set_var` and `std::env::remove_var` are **unsafe** in Rust 2024 Edition due to
POSIX setenv/getenv thread-safety (data race with any concurrent libc::getenv call from C
libraries, including loggers).

Design consequence: **snapshot-at-startup, never mutate**. Call `std::env::vars()` once,
single-threaded, before any thread spawns or logger init. The result is `SanitizedEnv` —
a read-only validated snapshot. No code path ever calls `set_var` or `remove_var`.
`#![forbid(unsafe_code)]` enforces this at compile time.

## AT_SECURE does NOT protect us

glibc's secure-execution mode (`AT_SECURE=1`) strips the 22 dangerous ld.so variables,
but **only for setuid/setgid/capability-elevated binaries**. UMRS tools run as normal
user processes (uid == euid, no capabilities) — `AT_SECURE` stays 0. No automatic
glibc stripping. Application-level scrubbing is the only protection.

Same for `secure_getenv(3)`: returns NULL when `AT_SECURE=1`. Since we're never setuid,
it behaves identically to `getenv`. And it requires unsafe anyway — not available to us.

## Tier 1 — Never Expected (warn on presence, regardless of value)

```
# glibc AT_SECURE strip list (22 vars — cite ld.so(8), NIST SP 800-53 CM-7, SI-7)
GCONV_PATH, GETCONF_DIR, GLIBC_TUNABLES(*), HOSTALIASES, LOCALDOMAIN,
LD_AUDIT, LD_DEBUG, LD_DEBUG_OUTPUT, LD_DYNAMIC_WEAK, LD_HWCAP_MASK,
LD_LIBRARY_PATH, LD_ORIGIN_PATH, LD_PRELOAD, LD_PROFILE, LD_SHOW_AUXV,
LOCPATH, MALLOC_TRACE, NIS_PATH, NLSPATH, RESOLV_HOST_CONF, RES_OPTIONS,
TMPDIR, TZDIR

# Shell execution injection
BASH_ENV, ENV, SHELLOPTS, GLOBIGNORE, IFS

# Interpreter module/library injection
PERL5LIB, PERLLIB, PERL5OPT,
PYTHONPATH, PYTHONSTARTUP, PYTHONHASHSEED,
RUBYLIB, RUBYOPT,
NODE_OPTIONS, NODE_PATH,
JAVA_TOOL_OPTIONS, _JAVA_OPTIONS, JVM_OPTS,
PHPRC
```

(*) GLIBC_TUNABLES: cite CVE-2023-4911 in scrub report.
PYTHONPATH/RUBYLIB/PERL5LIB/NODE_OPTIONS: cite CVE-2024-48990 / CVE-2024-48992 (needrestart).

## Tier 2 — Validate Before Use

```
PATH      → validate_path_list(): root-owned, no world-writable, no ".", no relative paths
LANG      → validate_lang(): POSIX locale syntax
LC_ALL    → validate_lang()
LC_MESSAGES → validate_lang()
TERM      → validate_term(): known terminal identifiers only
HOME      → validate_safe_path(): exists, correct ownership, 0700+
TZ        → validate_tz(): valid timezone identifier
LOGNAME / USER → validate_username(): POSIX username syntax
HOSTNAME  → validate_hostname(): RFC 1123
```

## Tier 3 — Pass Through (low risk)

```
TERM_PROGRAM, COLORTERM, NO_COLOR, LINES, COLUMNS,
DBUS_SESSION_BUS_ADDRESS (validate before use),
XDG_RUNTIME_DIR, XDG_SESSION_TYPE, DISPLAY, WAYLAND_DISPLAY
```

## Secret-pattern finding (CWE-526)

Variables matching `*_KEY`, `*_SECRET`, `*_TOKEN`, `*_PASSWORD`, `*_PASS`, `*_API_KEY`,
`*_CREDENTIAL`, `AWS_*`, `GITHUB_TOKEN` — record the **name only, never the value**, as a
CWE-526 finding. These are visible in `/proc/pid/environ` and violate NIST SP 800-53 IA-5, SC-28.

## Child process isolation — mandatory pattern

```rust
Command::new("/usr/bin/helper")
    .env_clear()
    .env("PATH", "/usr/bin:/usr/sbin:/bin:/sbin")
    .env("LANG", "C")
    .env("TZ", "UTC")
    .spawn()?
```

Never inherit the parent environment. `.env_clear()` + explicit minimal set is the only
safe approach. Mirrors `sudo env_reset` and systemd service unit behavior.

## CWE ↔ NIST mapping

| Finding class                          | CWE     | NIST SP 800-53 |
|----------------------------------------|---------|----------------|
| Dangerous loader variable present      | CWE-427 | CM-7, SI-7     |
| Secret-pattern variable present        | CWE-526 | IA-5, SC-28    |
| External initialization of trusted var | CWE-454 | SI-10, AC-3    |
| Security decision from env var         | CWE-807 | SI-10          |
| Invalid value in validated var         | CWE-20  | SI-10          |

## Implementation architecture

- `init_tool()` called as FIRST act of `main()`, before logger init, before thread spawn
- Returns `(SanitizedEnv, ScrubReport)` — both read-only, immutable for process lifetime
- After `init_tool()`, no code path uses `std::env::var()` directly — only typed accessors
- `ScrubReport` is a structured type (findings as enum variants, not log strings) — callers can
  query, count, and match. Satisfies Security Findings as Data Rule.

## NSA RTB RAIN placement

`init_tool()` is non-bypassable by design. No code path exists that skips it and reaches env
access. The private constructor pattern on `SanitizedEnv` enforces this.

## Open questions for implementation plan

1. **validate_safe_path() symlink depth**: The rule says "no symlinks to world-writable directories."
   Does this mean we follow symlinks recursively (stat the real target) or only check immediate
   directory permissions? On RHEL 10 `/usr/bin` → `/bin` symlink chains exist. Need to clarify
   whether symlink resolution uses `O_NOFOLLOW`-style checks or `rustix::fs::stat` on the resolved
   path. The latter seems right for PATH validation.

2. **TMPDIR**: Tier 1 (warn on presence). But TMPDIR is commonly set by users and systemd units.
   Is the finding a WARN log entry only, or does it also appear in the ScrubReport as an
   actionable item? Context: if UMRS tools never use TMPDIR themselves (using `tempfile::Builder`
   instead), the warning is informational. Should we distinguish "loader-injection risk" Tier 1
   vars from "execution-context anomaly" Tier 1 vars in the finding enum?

3. **GLIBC_TUNABLES on RHEL 10**: RHEL 10 ships glibc patched for CVE-2023-4911. Should the
   scrub report still cite the CVE as justification for the Tier 1 classification, or should we
   note "patched on RHEL 10, risk mitigated at OS level, finding is defense-in-depth"?

**Why:** Drives how `ScrubFinding` severity is modeled — a flat WARN for all Tier 1 vs. a
severity-annotated finding that carries platform context.
