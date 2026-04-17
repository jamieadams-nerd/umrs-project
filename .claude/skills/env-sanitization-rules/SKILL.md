---
name: env-sanitization-rules
description: >
  Environment sanitization rules for Rust binary crates: env variable tier classification,
  scrub reporting, init_tool entry points, Command::env_clear for child processes, and
  secret handling prohibitions. Use this skill when working with env, scrub, sanitize,
  init_tool, LD_PRELOAD, GLIBC_TUNABLES, ScrubReport, SanitizedEnv, process environment,
  environment variables, or CWE-454. Trigger when implementing or reviewing environment
  handling code, spawning child processes, or working on umrs-core::init.
---

# Environment Sanitization Rules

Applies when working on `umrs-core::init`, `scrub_env`, `SanitizedEnv`, `ScrubReport`,
any tool's `init_tool()` entry, or any code that reads `std::env::var()` / spawns
child processes.

**Full rationale and source citations:** `.claude/references/reports/2026-04-12-env-attack-prevention.md`
(Librarian report, 2026-04-12). Read the report before implementing or reviewing env-handling code.

---

## Axioms

[AXIOM] The Unix process environment is attacker-controlled input. Every variable is
external initialization of a trusted data store (CWE-454) until proven otherwise.

[AXIOM] glibc AT_SECURE secure-execution mode does NOT activate for normal UMRS tools.
AT_SECURE only strips dangerous variables when a process is setuid, setgid, capability-
elevated, or LSM-flagged. UMRS tools run as regular user processes — no automatic
protection. Application-level scrubbing is mandatory, not defense-in-depth.

---

## Constraints

[CONSTRAINT] UMRS tools MUST NOT call `std::env::set_var` or `std::env::remove_var`.
Both are unsafe in Rust 2024 Edition due to POSIX setenv/getenv thread-safety. The parent
process environment is read-only. `#![forbid(unsafe_code)]` enforces this at compile time.

[CONSTRAINT] Every binary crate's `main()` MUST call `init_tool()` (or equivalent) before
any other work, including before logger init and before any thread spawns. The env snapshot
must be taken single-threaded.

[CONSTRAINT] Secrets (API keys, tokens, passwords) MUST NOT be read from the process
environment. Environment variables are visible via `/proc/pid/environ` (CWE-526, violates
NIST SP 800-53 IA-5 and SC-28). Secret-pattern variable names (`*_KEY`, `*_SECRET`,
`*_TOKEN`, `*_PASSWORD`, `AWS_*`, `GITHUB_TOKEN`, etc.) present at startup are a scrub
finding — record the name, never the value.

---

## Rules

[RULE] Child processes MUST be spawned with `Command::env_clear()` followed by an
explicit minimal environment. Never inherit the parent environment:

```rust
Command::new("/usr/bin/helper")
    .env_clear()
    .env("PATH", "/usr/bin:/usr/sbin:/bin:/sbin")
    .env("LANG", "C")
    .env("TZ", "UTC")
    .spawn()?
```

[RULE] Env classification uses three tiers. All Tier 1 variables present at startup are
findings regardless of value:

**Tier 1 — Never Expected (WARN on presence):**

```
# glibc AT_SECURE strip list (22 variables from ld.so(8))
GCONV_PATH, GETCONF_DIR, GLIBC_TUNABLES, HOSTALIASES, LOCALDOMAIN,
LD_AUDIT, LD_DEBUG, LD_DEBUG_OUTPUT, LD_DYNAMIC_WEAK, LD_HWCAP_MASK,
LD_LIBRARY_PATH, LD_ORIGIN_PATH, LD_PRELOAD, LD_PROFILE, LD_SHOW_AUXV,
LOCPATH, MALLOC_TRACE, NIS_PATH, NLSPATH, RESOLV_HOST_CONF, RES_OPTIONS,
TMPDIR, TZDIR,

# Shell execution injection
BASH_ENV, ENV, SHELLOPTS, GLOBIGNORE, IFS,

# Interpreter module/library injection
PERL5LIB, PERLLIB, PERL5OPT,
PYTHONPATH, PYTHONSTARTUP, PYTHONHASHSEED,
RUBYLIB, RUBYOPT,
NODE_OPTIONS, NODE_PATH,
JAVA_TOOL_OPTIONS, _JAVA_OPTIONS, JVM_OPTS,
PHPRC
```

**Tier 2 — Validate Before Use:**

```
PATH          -> validate_path_list(): root-owned entries, no world-writable, no "."
LANG          -> validate_lang(): POSIX locale syntax
LC_ALL        -> validate_lang()
LC_MESSAGES   -> validate_lang()
TERM          -> validate_term(): known terminal identifiers only
HOME          -> validate_safe_path(): exists, correct ownership, 0700+
TZ            -> validate_tz(): valid timezone identifier
LOGNAME/USER  -> validate_username(): POSIX username syntax
HOSTNAME      -> validate_hostname(): RFC 1123
```

**Tier 3 — Pass Through (Low Risk):**

```
TERM_PROGRAM, COLORTERM, NO_COLOR, LINES, COLUMNS,
DBUS_SESSION_BUS_ADDRESS (validate before use),
XDG_RUNTIME_DIR, XDG_SESSION_TYPE, DISPLAY, WAYLAND_DISPLAY
```

[RULE] `validate_safe_path()` for PATH entries fails closed unless ALL conditions hold:
directory exists; not world-writable; not `.` or relative; owned by root or current user;
no symlinks to world-writable directories.

[RULE] GLIBC_TUNABLES presence is a Tier 1 finding regardless of value. Cite CVE-2023-4911
in the scrub report.

[RULE] PYTHONPATH, RUBYLIB, PERL5LIB, PERL5OPT, NODE_OPTIONS presence are Tier 1 findings.
Cite CVE-2024-48990 / CVE-2024-48992 (needrestart cluster).

[RULE] Finding classification maps to CWE and NIST controls:

| Finding class                          | CWE     | NIST SP 800-53 |
|----------------------------------------|---------|----------------|
| Dangerous loader variable present      | CWE-427 | CM-7, SI-7     |
| Secret-pattern variable present        | CWE-526 | IA-5, SC-28    |
| External initialization of trusted var | CWE-454 | SI-10, AC-3    |
| Security decision from env var         | CWE-807 | SI-10          |
| Invalid value in validated var         | CWE-20  | SI-10          |

---

## Patterns

[PATTERN] Tier 1 source comments should cite the canonical authority:

```rust
// glibc ld.so(8) secure-execution stripped variables — NIST SP 800-53 CM-7, SI-7
```

[PATTERN] `ScrubReport` and `SanitizedEnv` are read-only snapshots taken once at
`init_tool()`. They never mutate the process environment. Callers access validated values
through typed accessors, never via `std::env::var()` after init.

[PATTERN] For systemd-packaged UMRS services, the unit file should include:

```ini
Environment=LANG=en_US.UTF-8
UnsetEnvironment=LD_PRELOAD LD_LIBRARY_PATH GLIBC_TUNABLES LD_AUDIT LD_DEBUG
PrivateTmp=true
ProtectProc=invisible
NoNewPrivileges=true
```

---

## Anti-Patterns

[ANTI-PATTERN] Reading `std::env::var("SOMETHING")` anywhere outside `init_tool()` in a
binary crate. All env access must go through the validated snapshot.

[ANTI-PATTERN] Passing attacker-influenced environment to a child process. `Command::new()`
inherits the full parent environment by default — always `.env_clear()` first.

[ANTI-PATTERN] Using environment variables to gate security decisions (e.g.,
`if env::var("DEBUG_MODE") ...`). This is CWE-807 by construction.

[ANTI-PATTERN] Storing secrets in environment variables (CWE-526). Use file descriptors,
systemd `LoadCredential=`, or a secrets manager. Process environment is visible in
`/proc/pid/environ`, `ps`, crash dumps, and container orchestrator metadata.

---

## Compliance

- NIST SP 800-53 CM-7 (Least Functionality) — scrub disallowed developer-only variables
- NIST SP 800-53 SI-7 (Software Integrity) — detect unexpected runtime configuration
- NIST SP 800-53 SI-10 (Input Validation) — all Tier 2 validators
- NIST SP 800-53 SC-3 (Security Function Isolation) — env cannot influence security logic
- NIST SP 800-53 IA-5, SC-28 — no secrets in environment
- NIST SP 800-218 SSDF PW.4.1 — vetted env-handling pattern
- NSA RTB RAIN (Non-Bypassability) — scrub runs before any other init work
- CWE-427, CWE-454, CWE-526, CWE-807, CWE-20
