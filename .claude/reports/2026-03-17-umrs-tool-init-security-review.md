# Security Review: UMRS Tool Initialization Plan (`umrs-core::init`)

```
Audit date: 2026-03-17
Depth: in-depth
Scope: .claude/plans/umrs-tool-init.md — environment scrubbing, logging init, i18n init, validation API
```

---

## Summary

The plan is well-structured and demonstrates clear understanding of the privilege-separation problem.
The three-tier model is sound, the denylist is strong, and the validator designs are largely correct.
Several specific gaps are identified below — two are HIGH severity (one of which is a hard blocker),
the rest are MEDIUM and LOW. None invalidate the architecture, but several require explicit decisions
before implementation.

---

## File: `.claude/plans/umrs-tool-init.md`

---

### Finding 1

**Location:** Phase 1, Tier 3 denylist (line ~271–295)

**Finding:** The denylist is missing several variables that glibc AT_SECURE / `ld.so` secure-execution
mode and `sudo env_reset` strip but the plan does not mention.

The following are absent and should be in Tier 3:

| Variable | Risk |
|---|---|
| `LD_ORIGIN_PATH` | Overrides `$ORIGIN` expansion in `RPATH`/`RUNPATH` — allows shared library hijack even when `LD_LIBRARY_PATH` is stripped |
| `LD_DYNAMIC_WEAK` | Changes weak symbol override behavior — can redirect function calls |
| `LD_USE_LOAD_BIAS` | Affects ASLR base address computation for loaded libraries |
| `LD_VERBOSE` | Information leak of linker internals |
| `LD_SHOW_AUXV` | Dumps the auxiliary vector to stderr — exposes AT_SECURE, AT_RANDOM, load addresses |
| `NLSPATH` | glibc `catopen()` locale message path override — arbitrary file read under glibc NLS |
| `GETCONF_DIR` | `getconf(1)` config directory override; used by some glibc internals |
| `TZDIR` | Overrides the tzdata directory; can redirect `TZ` file lookups to attacker-controlled paths even if `TZ` itself validates as an Olson name |
| `POSIXLY_CORRECT` | Changes POSIX compliance behavior of libc functions — can affect input parsing |
| `GLIBC_TUNABLES` | (glibc 2.35+, RHEL 10 ships glibc 2.39) Tunes glibc allocator and other internals; CVE-2023-4911 (Looney Tunables) demonstrated this is a privilege escalation vector |

`GLIBC_TUNABLES` is particularly important for RHEL 10 because RHEL 10 ships glibc 2.39 and
`GLIBC_TUNABLES` was the vector for CVE-2023-4911 — a heap buffer overflow that achieved
privilege escalation on systems with setuid binaries. Even if UMRS tools are not setuid today,
this variable has no legitimate use in a scrubbed process environment and should be stripped.

`TZDIR` also requires special attention because the `validate_tz()` function correctly blocks
absolute paths and `..` traversal in the `TZ` value itself, but if `TZDIR` is inherited, an
attacker can redirect the zoneinfo file lookup entirely — bypassing the `TZ` validation.
These two defenses must both be present to be effective.

**Severity:** HIGH

**Control reference:** NIST SP 800-53 SI-7 (runtime configuration integrity), NIST SP 800-53 CM-7
(least functionality — strip what is not needed), CWE-526, CVE-2023-4911 (GLIBC_TUNABLES)

**Remediation owner:** coder

**Recommended action:**
Add the following to the Tier 3 unconditional strip table:

```
LD_ORIGIN_PATH, LD_DYNAMIC_WEAK, LD_USE_LOAD_BIAS, LD_VERBOSE, LD_SHOW_AUXV
NLSPATH
GETCONF_DIR
TZDIR          ← CRITICAL: must be stripped before validate_tz() is meaningful
POSIXLY_CORRECT
GLIBC_TUNABLES ← CRITICAL: CVE-2023-4911 vector; RHEL 10 / glibc 2.39
```

`TZDIR` must be stripped in Tier 3 processing (before `TZ` validation), not afterward, because
if `TZDIR` is present and attacker-controlled, the `TZ` validation is incomplete regardless of
correctness.

---

### Finding 2

**Location:** Phase 1, Implementation Notes — thread safety (line ~557)

**Finding:** The plan correctly identifies the thread safety problem with `std::env::remove_var` /
`set_var` but leaves the resolution as "needs Jamie's call." This is a hard design gate, not a
style preference, and the plan cannot be correctly implemented without resolving it first.

In Rust 1.83+ (stabilized October 2024 — available in the Rust edition used here), `std::env::set_var`
and `std::env::remove_var` are marked `unsafe`. The reason is fundamental: `getenv` / `setenv` in
POSIX are not thread-safe — a concurrent read from any thread (including threads spawned by library
code the tool does not control) constitutes a data race on the environment. This is undefined
behavior in C and unsound in Rust.

The plan's proposed `#[allow(unsafe_code)]` override on the scrubbing function is **incompatible
with `#![forbid(unsafe_code)]`** at the crate root. `#![forbid]` cannot be overridden by an inner
`#[allow]`; the CLAUDE.md explicitly states this is a compile-time proof. The plan itself notes
this on the same line but then presents it as a viable option — it is not. The function as described
cannot be written within this crate under current project rules.

The alternative the plan mentions — `Command::env_clear()` scoping to child processes — does not
solve the parent process environment problem. UMRS tools read their own scrubbed environment
(e.g., for `LANG`, `TERM`, `HOME`) after scrubbing, so the parent process environment must actually
be modified, not just withheld from children.

**Two conformant options exist:**

**Option A — Move scrubbing to a separate crate or module without `forbid`:**
Create a thin `umrs-init` crate or a `scrub.rs` module in `umrs-core` that explicitly does NOT
carry `#![forbid(unsafe_code)]` but does carry `#![deny(unsafe_code)]` plus a targeted
`#[allow(unsafe_code)]` only on `scrub_env`. This isolates the unsafe surface to one function
in one file. The `// SAFETY:` comment must state: "Single-threaded precondition: caller guarantees
no other threads exist at call site. Environment modification via `set_var`/`remove_var` is safe
when no concurrent reads occur (POSIX)."

**Option B — Use `std::env::set_var` before any thread is spawned and document the binary entry
point contract:**
Tools that call `init_tool()` must call it as the very first statement in `main()`, before any
`tokio::main` or `rayon` pool initialization. This is already noted in the plan ("must be called
early in `main()` before spawning threads"). The safety argument holds only if this contract is
mechanically enforced — ideally via a procedural macro or lint, not just documentation.

The plan must commit to one of these options. Option A (isolated unsafe boundary) is the security
engineer's recommendation — it is consistent with the existing `umrs-hw` unsafe isolation model
already established in this workspace (`umrs-hw` is the workspace's unsafe isolation boundary
for the same architectural reason).

**Severity:** HIGH — without resolution, the plan cannot be implemented without either violating
`#![forbid(unsafe_code)]` or leaving the parent process environment unscrubbed.

**Control reference:** NIST SP 800-53 SA-11 (developer testing), NIST SP 800-218 SSDF PW.4
(safe-code guarantees), NSA RTB RAIN (non-bypassability)

**Remediation owner:** coder (with Jamie decision on which option)

**Recommended action:**
Before implementation begins, resolve the unsafe boundary question:
- If Option A: add `umrs-init` as a new workspace crate, or explicitly carve out a module with
  `#![allow(unsafe_code)]` that is architecturally isolated (like `umrs-hw`). Update
  the dependency table in CLAUDE.md.
- If Option B: add a procedural attribute macro (e.g., `#[umrs::tool_entry_point]`) that wraps
  `main()` and statically guarantees `init_tool()` is the first call — or document the limitation
  clearly and accept best-effort.
- Either way: document the chosen unsafe boundary in the plan and in the crate root.

---

### Finding 3

**Location:** Phase 1, Tier 1 — Reset to safe defaults, `IFS` row (line ~97)

**Finding:** `IFS` is listed as "removed entirely — no safe default; shell parsing manipulation."
This rationale is correct but the implication is under-stated. `IFS` is a shell variable and has
no meaning in a Rust process — Rust does not use `IFS` for any standard library function. The risk
is that the process might spawn subshells or shell-interpreting child processes (via `sh -c` etc.)
that inherit the variable. The correct placement for `IFS` is therefore Tier 3 (unconditional
strip), not Tier 1 (reset to safe default), because there is no safe reset target and the current
Tier 1 table implies it is in the same category as `PATH` and `TMPDIR`.

This is a minor classification error that could confuse an implementer into treating `IFS` as
a "reset" target and searching for a safe value.

**Severity:** LOW

**Control reference:** CWE-526, CERT ENV03-C

**Remediation owner:** coder

**Recommended action:**
Move `IFS` from Tier 1 to Tier 3. Tier 1 should contain only variables that have a meaningful
safe default and that the process itself reads. Update the Tier 1 table note to reflect: "Tier 1
is for variables this process reads and for which a safe default exists."

---

### Finding 4

**Location:** Phase 1, Tier 2, Validation Class: Safe Path — rule 8 (line ~118)

**Finding:** Rule 8 states "No sticky-bit-less world-writable parent directories in the chain"
but the implementation guidance does not address symlink chain validation. The classic TOCTOU
issue with path ownership checks is:

1. Attacker controls a parent directory in the path that is not world-writable (passes rule 6)
2. Attacker replaces a non-final path component with a symlink pointing to an attacker-controlled
   directory between the `stat()` check and the actual use

The plan's rule 8 partially mitigates this, but full TOCTOU protection requires that every
component in the resolved path (not just the terminal node and its parent) be validated for
ownership. The `HOME` check is the most sensitive because if an attacker can predict or influence
`HOME` before `scrub_env()` runs, rule 7 ("owned by root or current user") must apply to every
directory in the path chain, not just the leaf.

Additionally, the plan does not specify whether `validate_safe_path` follows symlinks
(`std::fs::metadata`) or does not follow them (`std::fs::symlink_metadata`). For socket-type
paths (e.g., `SSH_AUTH_SOCK`), the file type check `S_ISSOCK` requires following the symlink
to reach the socket. The behavior must be explicit and documented per path kind.

**Severity:** MEDIUM

**Control reference:** NIST SP 800-53 SI-7 (software integrity), NSA RTB RAIN (non-bypassability)

**Remediation owner:** coder

**Recommended action:**
1. Specify in the `validate_safe_path` design that the function iterates every component of the
   canonicalized path and verifies ownership (rule 7) on each directory, not only the leaf.
   Use `std::fs::canonicalize()` to resolve the full path first, then walk components.
2. Document per path kind whether symlinks are followed: for directory kinds, follow to reach the
   actual target; for socket kinds, follow to check `S_ISSOCK`. Use `std::fs::metadata()` (follows
   symlinks) for both, after canonicalization.
3. Add a test case: `HOME` contains a symlink component that resolves to a directory owned by UID 0
   but the symlink itself is owned by an attacker. Verify rejection.

---

### Finding 5

**Location:** Phase 1, Tier 2, `USER` / `LOGNAME` validation (line ~206–209)

**Finding:** The plan validates `USER` and `LOGNAME` as syntactically valid POSIX usernames and
warns if they mismatch. However, it does not specify whether the validated values are then
cross-checked against the actual process UID via `getuid()` + `/etc/passwd` (or equivalent).
Without this cross-check, an attacker who can control the inherited environment can supply a
syntactically valid but incorrect username (e.g., `USER=root` when the process runs as UID 1000).
Downstream code that logs `USER` for audit purposes would emit misleading identity claims.

For a tool suite that produces audit-relevant output (NIST AU-3 integrity), logging an unverified
`USER` value is a direct audit integrity gap.

**Severity:** MEDIUM

**Control reference:** NIST SP 800-53 AU-3 (content of audit records — identity must be accurate),
NIST SP 800-53 IA-5 (authenticator management), NIST SP 800-53 AC-6 (least privilege)

**Remediation owner:** coder

**Recommended action:**
Add a `validate_username_for_uid()` variant or a note in the `USER`/`LOGNAME` row specifying
that after syntax validation, the value is cross-checked against the process UID via
`nix::unistd::getuid()` and a passwd lookup. If the `USER` value does not correspond to the
process UID, the variable should be reset to the value derived from the actual UID (not the
inherited environment). This ensures the post-scrub environment has accurate identity.
Add a `ScrubReport` flag for `uid_mismatch` to surface this to the caller for audit logging.

---

### Finding 6

**Location:** Phase 2, Logging Initialization — tag design (line ~406–414)

**Finding:** Using `"umrs"` as a single shared `SYSLOG_IDENTIFIER` for all tools has a
log-correlation limitation that is worth making explicit in the plan. When multiple UMRS tools
run concurrently (e.g., `umrs-ls` and a future `umrs-audit` daemon), journal entries are
distinguishable only by `_COMM` (the process name field journald populates automatically) and
`_PID`. The `SYSLOG_IDENTIFIER` filter (`journalctl -t umrs`) will aggregate all tools together,
which is convenient for unified monitoring but means that `journalctl -t umrs-ls` will not work.

This is a tradeoff, not a flaw — single-tag makes filtering simpler for operators and matches
the plan's intent. However, the plan should explicitly state the rationale so future implementers
don't "fix" it by adding per-tool tags without understanding why a single tag was chosen.

A secondary concern: the fallback to `stderr` when journald is unavailable (development/container)
should explicitly strip ANSI color codes and structured field markers from stderr output to avoid
log injection artifacts in environments that capture stderr for SIEM ingestion. Structured journal
fields (like `CODE_FILE=`, `CODE_LINE=`) should not appear on stderr since they are journald
protocol, not plain text.

**Severity:** LOW

**Control reference:** NIST SP 800-53 AU-3 (audit record content), NIST SP 800-92 (log management)

**Remediation owner:** coder

**Recommended action:**
1. Add a note to the logging design explaining the single-tag rationale.
2. Specify that the stderr fallback emits plain-text log lines only — no structured journal fields,
   no ANSI codes, no escape sequences that could be interpreted by a receiving log pipeline.
3. Consider logging the binary name (`std::env::current_exe()` stem) as a structured `CODE_FILE`-
   adjacent field so operators can filter by tool even under the shared tag.

---

### Finding 7

**Location:** Phase 2, logging control citations (line ~421–426)

**Finding:** The logging initialization design cites AU-3 and AU-12 only. For a journald-backed
logging subsystem on RHEL 10, the following additional controls are relevant and should be cited:

- **AU-9** (Protection of Audit Information) — journald stores logs in binary format with forward
  sealing support (`journalctl --verify`). If UMRS tools emit audit-relevant events, the plan should
  note whether journal forward sealing (FSS — Forward Secure Sealing) is expected to be configured.
  If it is not, an attacker with journal write access could inject retrospective entries.
- **AU-8** (Time Stamps) — the plan specifies no timestamp format. Journald captures
  `_SOURCE_REALTIME_TIMESTAMP` automatically for native log entries. Tools should use monotonic
  clock offsets where available to correlate with kernel audit records. This connects to the
  `umrs-hw` `RDTSCP`-based timestamps already in the project.
- **SI-11** (Error Handling) — the Debug Information Discipline rule applies to log output.
  Confirm the logging initialization does not include variable data from environment variables
  in the scrub report log output at any level other than debug, and only in non-CUI mode.

**Severity:** LOW

**Control reference:** NIST SP 800-53 AU-8, AU-9, AU-12, SI-11

**Remediation owner:** coder

**Recommended action:**
Add AU-8, AU-9, and SI-11 to the `init_logging` control citations. Add a note that journal
forward sealing is expected to be configured at the systemd level for deployed systems (this is
a deployment concern, not a code concern, but should be referenced in the plan's deployment
notes). Add a note that the stderr fallback must not emit CUI-adjacent data even in debug mode.

---

### Finding 8

**Location:** Phase 3, i18n auto-detection (line ~452–456)

**Finding:** The auto-detection of textdomain from `std::env::current_exe()` introduces a
TOCTOU-like ambiguity: `current_exe()` is called after `scrub_env()` has already run. At that
point, `PATH` has been reset to a safe default and the environment is clean. However,
`current_exe()` itself reads `/proc/self/exe` — a symlink that the kernel maintains. This is
provenance-safe (kernel-maintained, not attacker-controllable via the environment). However,
the plan does not note this distinction.

More importantly: if `current_exe()` fails (returns `Err`), the fallback is `"umrs"` as the
textdomain. This means a tool with a gettext domain name other than `"umrs"` will silently fail
to load translations, with no error surfaced to the caller. The `init_i18n` signature returns `()`,
so there is no way to detect this failure. In a production CUI-handling tool, silent fallback to
an untranslated or wrong-domain textdomain is an operational gap.

**Severity:** LOW

**Control reference:** NSA RTB RAIN (non-bypassability — tracing the source of the binary name
to a kernel-provided path rather than env)

**Remediation owner:** coder

**Recommended action:**
1. Add a note that `current_exe()` reads `/proc/self/exe` (kernel-maintained symlink, not
   environment-derived), making this provenance-safe per the project's `kattrs` pattern.
2. Change `init_i18n` return type to `Result<(), InitError>` or at minimum log a warning
   (not silently succeed) when auto-detection falls back to `"umrs"`. The caller must be able
   to detect this condition.

---

### Finding 9

**Location:** Phase 1, Validation Class: D-Bus Address — `unixexec:` rejection (line ~240–248)

**Finding:** The plan correctly rejects `unixexec:` transport. However, `tcp:` and `nonce-tcp:`
transports pass validation as long as they start with those prefixes. In the UMRS deployment
model (no outbound network, isolated systems), TCP D-Bus connections are not a legitimate use
case. An attacker who can inject `DBUS_SESSION_BUS_ADDRESS=tcp:host=attacker.internal,port=12345`
into the environment will have it preserved after scrubbing if it passes syntactic validation.

In an air-gapped/isolated deployment (which UMRS assumes), TCP D-Bus transport should either
be rejected outright (cleanest) or trigger a warning in `ScrubReport` that a network-capable
D-Bus address was preserved.

**Severity:** MEDIUM

**Control reference:** NIST SP 800-53 SC-7 (boundary protection — no unexpected outbound connections),
NIST SP 800-53 CM-7 (least functionality)

**Remediation owner:** coder

**Recommended action:**
In `validate_dbus_address()`, reject `tcp:` and `nonce-tcp:` transports for the UMRS deployment
profile. If there is a future use case requiring TCP D-Bus (unlikely given the network posture),
it can be re-enabled via `scrub_env_with()` as an explicitly opt-in validator override.
Update the D-Bus validation rules table to document this rejection.

---

### Finding 10

**Location:** Phase 1, Tier 2, `SSH_AUTH_SOCK` preservation (line ~126–127)

**Finding:** Preserving `SSH_AUTH_SOCK` in the scrubbed environment is functionally reasonable
but is worth revisiting in the UMRS deployment context. UMRS tools run on isolated RHEL 10 systems.
SSH agent forwarding is a privileged communications channel — a compromised UMRS tool with access
to a valid `SSH_AUTH_SOCK` can use the agent to authenticate to other systems as the forwarding
user, even on an air-gapped network, if SSH agent connections are active.

The plan validates the socket path (ownership, `S_ISSOCK`) but does not ask whether UMRS tools
actually require SSH agent access for any operation. If no UMRS tool calls `ssh-add`, `ssh`, or
an SSH-related library, there is no functional reason to preserve `SSH_AUTH_SOCK` — and preserving
it silently extends the attack surface.

**Severity:** MEDIUM

**Control reference:** NIST SP 800-53 AC-17 (remote access — SSH agent forwarding is a remote
access channel), NIST SP 800-53 AC-6 (least privilege — preserve only what is needed)

**Remediation owner:** coder

**Recommended action:**
Audit which UMRS tools (if any) require SSH agent access. If none currently do, move
`SSH_AUTH_SOCK` to Tier 3 (strip unconditionally) with a note: "Preserved only when a tool
explicitly opts in via `scrub_env_with(&[\"SSH_AUTH_SOCK\"])`." This enforces least-privilege
at the environment level — tools that don't need SSH agent access don't get it.

---

### Finding 11

**Location:** Phase 5, Tests — `init_tool` integration test (line ~508)

**Finding:** Test 9 states "`init_tool` runs without panic (integration test)" but does not
specify what this test verifies beyond absence of panic. For a security-critical initialization
function, a test that only checks "no panic" is insufficient. The post-scrub environment must
be verified: the test should assert that after `init_tool()`, the resulting process environment
contains exactly the expected variables (no more, no fewer) in the expected state.

Additionally, the test suite does not include a test for the critical GLIBC_TUNABLES denylist
entry identified in Finding 1. Each newly-added denylist entry should have a corresponding test
case in `init_scrub_tests.rs`.

**Severity:** LOW

**Control reference:** NIST SP 800-53 CA-7 (continuous monitoring — tests are the verification
mechanism for runtime scrubbing behavior), NIST SP 800-218 SSDF PW.8 (test security aspects)

**Remediation owner:** coder

**Recommended action:**
1. Expand test 9 to snapshot the post-scrub environment and assert: (a) no denylist variables
   present, (b) `PATH` equals the safe default, (c) `TMPDIR` equals `/tmp`, (d) no unknown
   variables survive.
2. Add one test per newly-added denylist entry (GLIBC_TUNABLES, TZDIR, NLSPATH, etc.) to
   `init_scrub_tests.rs`.

---

## Architectural Comments (No Severity — Informational)

**Three-tier model — correct.** Reset / Preserve+Validate / Strip is the right architecture.
It mirrors what `sudo env_reset` does (reset PATH and HOME, preserve TERM/DISPLAY/LC_*, strip
everything else) and extends it with validation. The composable design (individual validators
are public) is correct for this project.

**`detect_secure_execution()` in Future Considerations — escalate to near-term.**
The plan defers `detect_secure_execution()` (AT_SECURE / real vs. effective UID check) to
"future considerations." For UMRS tools that may be invoked via `sudo` or from setuid contexts,
this detection should run _before_ or _as part of_ `scrub_env()`, not as a future enhancement.
Without it, a setuid-invoked UMRS tool performs the same scrubbing posture as a user-invoked one,
rather than escalating to the stricter posture. Consider moving this to Phase 1 scope.
Reading `/proc/self/status` for `CapEff` or comparing `getuid()` vs `geteuid()` is pure Rust,
no unsafe code required.

**`NLSPATH` vs. `validate_lang()` interaction — design note.**
Once NLSPATH is added to Tier 3 (Finding 1), the `validate_lang()` function must document that
it validates the `LANG` value string only, not the locale data lookup path. The `NLSPATH`/`LOCPATH`
strip in Tier 3 is what prevents locale-data injection; `validate_lang()` alone is not sufficient.
This should be a doc-comment note on `validate_lang()` to prevent future confusion.

**`Command` helper future consideration — strong endorsement.**
The `sanitize_command()` helper mentioned in Future Considerations is the right long-term shape.
Any subprocess spawned by a UMRS tool should inherit only what that subprocess needs, not the
full scrubbed parent environment. This is especially important if a future UMRS tool wraps
system utilities. Recommend promoting this from "future" to Phase N planning for any release that
includes subprocess-spawning tools.

---

## Gap Analysis Summary

```
Files reviewed: 1 (.claude/plans/umrs-tool-init.md)
Total findings: 11 (2 HIGH, 4 MEDIUM, 5 LOW)

Policy artifacts written: none (plan review; policy artifacts not required for a library module)
Policy artifacts needed: none (umrs-core::init is a library; SELinux policy governs the tool
  binaries that call it, not the library itself)

Documentation gaps:
  - validate_tz() needs a doc note that TZDIR strip (Tier 3) is required for TZ validation to
    be meaningful (Finding 1 / Finding 3 interaction)
  - init_i18n() doc comment should reference /proc/self/exe as the provenance source (Finding 8)
  - Single syslog tag rationale should be captured in logging.rs module doc (Finding 6)

Code-vs-policy inconsistencies:
  - #![forbid(unsafe_code)] at crate root is incompatible with the plan's Option 1
    (#[allow(unsafe_code)] on scrub_env) — this must be resolved before implementation.
    The workspace already has a precedent for isolating unsafe code in umrs-hw; the same
    model applies here (Finding 2).
  - TZDIR missing from denylist makes validate_tz() only partially effective (Finding 1).
  - TCP D-Bus transport allowed by validator conflicts with the deployment network posture
    of isolated RHEL 10 systems (Finding 9).
```

---

## Denylist Completeness Reference

For the implementer, the authoritative glibc `ld.so` secure-mode variable list (from the glibc
manual and `ld.so(8)` man page) that must all appear in Tier 3:

```
LD_PRELOAD, LD_LIBRARY_PATH, LD_AUDIT, LD_DEBUG, LD_BIND_NOW, LD_BIND_NOT,
LD_PROFILE, LD_PROFILE_OUTPUT, LD_VERBOSE, LD_SHOW_AUXV, LD_ORIGIN_PATH,
LD_DYNAMIC_WEAK, LD_USE_LOAD_BIAS
LOCPATH, GCONV_PATH, NLSPATH, GETCONF_DIR
MALLOC_CHECK_, MALLOC_TRACE
TZDIR
GLIBC_TUNABLES  (glibc 2.35+ / RHEL 10 glibc 2.39 — CVE-2023-4911 vector)
```

Additional userspace-interpreter variables already in the plan:
```
PYTHONPATH, PYTHONSTARTUP, PERL5LIB, PERL5OPT, RUBYLIB, RUBYOPT,
NODE_PATH, NODE_OPTIONS, CDPATH, ENV, BASH_ENV, HISTFILE,
HOSTALIASES, RESOLV_HOST_CONF, NIS_PATH
```

IFS should be moved from Tier 1 to Tier 3 (Finding 3).
