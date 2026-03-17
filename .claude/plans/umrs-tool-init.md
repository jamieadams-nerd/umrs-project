# Plan: UMRS Tool Initialization API (`umrs-core::init`)

**Status:** Approved — ready for implementation

**ROADMAP alignment:** G2 (Platform Library), G4 (Tool Ecosystem), G8 (Human-Centered Design)

**Source:** `.claude/jamies_brain/env-scrubbing.txt` (Jamie Adams)

---

## Problem

Every process inherits its parent's environment — unvalidated, mutable, externally observable via `/proc/<pid>/environ`, and historically one of the most exploited attack surfaces in Unix systems (CVE-2023-4911 `GLIBC_TUNABLES`, `LD_PRELOAD` injection, `PATH` trojan horses, `IFS` manipulation). UMRS tools run in high-assurance environments where an adversary who can influence the environment of a security tool has already undermined the trust model.

Additionally, every UMRS CLI/TUI/GUI tool needs the same startup boilerplate:

1. **Environment audit + validated access** — inspect inherited env vars, log anomalies, provide validated accessors (CWE-526, CERT ENV03-C, NIST SP 800-53 CM-7/AC-3/SI-7)
2. **i18n initialization** — `umrs_core::i18n::init(domain)` (already exists)
3. **Logging initialization** — structured journald-native logging (not yet in umrs-core)

Today each tool would have to roll its own. This leads to inconsistency, missed hardening, and duplicated code.

## Design Philosophy

### Environment Audit + Validated Accessor Pattern

This module is **not** a `sudo`-style environment mutator. UMRS tools follow a strict rule: **never fork out and run external binaries if avoidable.** This eliminates the primary attack surface that traditional environment scrubbing defends against (malicious `PATH`, `LD_PRELOAD` in child processes).

Instead, this module provides three things:

1. **Environment audit at startup.** The tool reads its inherited environment, classifies every variable against a three-tier security model, and produces an audit trail. Dangerous variables (`GLIBC_TUNABLES`, `LD_PRELOAD`, etc.) that should never appear in a UMRS tool's environment are logged at `warn` — their presence is a **security finding**, regardless of whether the tool uses them. This answers the auditor's question: *"Does the tool know what's in its environment?"*

2. **Validated accessors.** When a tool needs `LANG`, `TERM`, `HOME`, or any other environment value, it reads from the `SanitizedEnv` — a pre-validated, immutable snapshot where every value has passed its validation class (path safety, locale syntax, terminal identifier rules, etc.). Raw `std::env::var()` calls against untrusted inherited values are replaced by lookups against a validated store. This answers the auditor's question: *"Does the tool trust its inputs?"*

3. **Clean child environment (if ever needed).** In the rare case a tool must spawn a subprocess, `SanitizedEnv.apply(&mut Command)` provides a `Command::env_clear()` + validated re-injection in one call. The parent process environment is never modified — pure safe Rust, `#![forbid(unsafe_code)]` fully honored.

### What makes this valuable

- **Posture signal.** If `LD_PRELOAD` or `GLIBC_TUNABLES` appear in the environment of a UMRS tool, that is an anomaly worth recording. The scrub report creates auditable evidence.
- **Defense in depth.** Even though UMRS tools don't fork, validating the environment prevents a class of bugs where a tool accidentally reads a poisoned `HOME`, `TMPDIR`, or `TZ` value.
- **Reusable for the ecosystem.** The validator set (path safety, locale, terminal, timezone, username) doesn't exist as a Rust crate. This is independently publishable.
- **Zero side effects.** The parent process is untouched. Nothing breaks. The module is a read-only audit + validation layer.

## Design

### Principle: Composable, Not Monolithic

Four independent public functions, each usable standalone. Plus one convenience function that calls them all in the recommended order.

```
// Environment scrubbing
umrs_core::init::scrub_env()                      // sanitize with default allowlist
umrs_core::init::scrub_env_with(extra: &[&str])   // sanitize + tool-specific extras

// Reusable validators (useful independently, used internally by scrub_env)
umrs_core::init::validate_lang(value: &str)        // POSIX locale syntax validation      [SI-10]
umrs_core::init::validate_safe_path(path, kind)    // path safety: ownership, perms        [CM-7, AC-3, SI-7]
umrs_core::init::validate_path_list(value: &str)   // colon-delimited PATH safety          [CM-7, AC-3, SI-7]
umrs_core::init::validate_term(value: &str)        // terminal identifier validation       [SI-10]
umrs_core::init::validate_username(value: &str)    // POSIX username validation            [AU-3, AC-3]
umrs_core::init::validate_tz(value: &str)          // timezone string validation           [SI-10, AU-8]

// Tool initialization
umrs_core::init::init_i18n(domain)                 // i18n setup; auto-derives domain from binary name
umrs_core::init::init_logging(verbose: bool)       // journald-native structured logging → returns LoggingBackend
umrs_core::init::init_tool(verbose: bool)          // calls all of the above in order
```

Tools that need custom behavior call the pieces individually. Tools that just want "make me ready" call `init_tool()`.

### Module: `umrs_core::init`

New module directory at `umrs-core/src/init/`, re-exported from `lib.rs`.

```
umrs-core/src/init/
├── mod.rs              ← public API re-exports
├── scrub.rs            ← scrub_env(), scrub_env_with(), ScrubReport
├── logging.rs          ← init_logging()
├── i18n.rs             ← init_i18n()
├── tool.rs             ← init_tool() convenience wrapper
└── validate/
    ├── mod.rs          ← EnvValidationError, validator re-exports
    ├── path.rs         ← validate_safe_path(), validate_path_list()
    ├── locale.rs       ← validate_lang()
    ├── term.rs         ← validate_term()
    ├── username.rs     ← validate_username()
    ├── tz.rs           ← validate_tz()
    ├── dbus.rs         ← validate_dbus_address()
    └── integer.rs      ← validate_positive_int()
```

---

## Decisions (Jamie, 2026-03-17)

1. **Configurable allowlist** — Yes. `scrub_env_with(extra: &[&str])` for tools needing additional vars. `scrub_env()` uses the default allowlist only.
2. **`TMPDIR`** — Reset to `/tmp` (not stripped). Safe known value.
3. **Logging backend** — Journald-native from the start. Tag: `"umrs"`. No `env_logger`.
4. **Verbose control** — Boolean (`verbose: bool`).
5. **i18n textdomain** — Default should auto-derive from the binary name (`std::env::current_exe()` stem). `init_i18n(None)` = auto-detect, `init_i18n(Some("umrs-ls"))` = explicit override.
6. **LANG validation** — Dedicated `validate_lang()` function that scrubs and validates locale syntax. Reusable by external consumers. Applied to `LANG` and all `LC_*` vars during scrub.
7. **Display vars** — Preserve and validate `TERM`, `COLORTERM`, `NO_COLOR`, `COLUMNS`, `LINES` (needed by `colored` crate, `textwrap`, and `umrs-core::console`). Validate values are syntactically safe.
8. **Reset vs strip** — Dangerous vars (`LD_*`, `PATH`, `TMPDIR`) get **reset** to safe defaults rather than stripped, where a safe default exists. Truly dangerous vars with no safe default (`LD_PRELOAD`, `IFS`, interpreter paths) get stripped.

### Post-Review Decisions (Jamie, 2026-03-17)

9. **Thread safety / `#![forbid(unsafe_code)]`** — **Option A (child-process scrubbing only).** `scrub_env` does NOT mutate the parent process environment. Instead, it builds a validated, sanitized environment map and returns it as a `SanitizedEnv` that can be applied to `Command` via `env_clear()` + selective re-injection. Pure safe Rust. No `unsafe` exception. No new sub-crate needed. This is architecturally cleaner: the scrubber is a validator and builder, not a mutator.
10. **`SSH_AUTH_SOCK`** — **Tier 3 (strip by default).** No UMRS tool needs SSH agent access. The socket provides credential access to SSH keys — a lateral movement vector if a tool is compromised. Tools that genuinely need it can opt in via `scrub_env_with(&["SSH_AUTH_SOCK"])`.
11. **D-Bus `tcp:` transport** — **Reject.** Air-gapped RHEL 10 deployment has no legitimate use for TCP D-Bus. Only `unix:path=` and `unix:abstract=` are accepted. Rejected transports are logged at `warn` level (not just debug — this is an anomaly worth noticing).
12. **Stripped variable logging** — Dangerous vars stripped from Tier 3 denylist are logged at `log::warn!` (these are anomalies — they shouldn't be in the environment in the first place). Vars removed for failing Tier 2 validation are also logged at `log::warn!` — a `HOME` with path traversal or a `TERM` with shell metacharacters is suspicious, not routine. Tier 1 resets (PATH, TMPDIR) are logged at `log::info!` (expected behavior). The `ScrubReport` return value provides full programmatic detail for callers who need it.
13. **`IFS`** — Reclassified from Tier 1 (reset) to Tier 3 (strip). Rust doesn't use IFS; no safe default exists.
14. **`init_i18n` fallback** — Log at `log::warn!` when auto-detection falls back to `"umrs"` domain. Silent fallback is unacceptable for troubleshooting. **Init order resolved (compliance review):** execution order is scrub → logging → i18n. This resolves the dead-warning issue — `init_i18n` fallback warnings are now emitted after logging is initialized, so they actually reach the journal.
15. **`EnvValidationError`** — Must derive `thiserror::Error`, use `#[non_exhaustive]`, and `#[must_use]` goes on validator functions (not the error type).
16. **`rustix` not `nix`** — Use `rustix = { version = "0.38", features = ["fs", "process"] }` for path validation. Matches existing workspace usage. `umrs-tui` version conflict (`rustix = "1"`) must be resolved before adding to `umrs-core`.
17. **`USER`/`LOGNAME` cross-check** — Validate syntax AND cross-check against actual UID via `getuid()` + `/etc/passwd` lookup. Syntactically valid `USER=root` from a non-root process is rejected. Controls: AU-3 (audit record content integrity).
18. **Symlink chain validation** — `validate_safe_path` must check every directory component in the resolved path using `open(O_PATH | O_NOFOLLOW)` + `fstat()`, not just the leaf. O(depth) cost is acceptable for startup-once validation.

---

## Phase 1: Environment Scrubbing (`scrub_env` / `scrub_env_with`)

### Three-tier approach: Reset, Preserve+Validate, Strip

**Tier 1: Reset to safe defaults**

These vars are always overwritten to a known-safe value, regardless of inherited content.

| Variable | Safe Default | Reason |
|---|---|---|
| `PATH` | `/usr/bin:/bin:/usr/sbin:/sbin` | Prevent path injection |
| `TMPDIR` | `/tmp` | Prevent temp dir redirect attacks |

**Tier 2: Preserve + Validate**

These vars survive scrubbing but their values are validated. Invalid values cause the var to be removed and the removal logged in the `ScrubReport.failed_validation` vector.

Validation is organized by _validation class_ — variables that represent the same kind of value (path, identifier, locale, etc.) share the same validation function. This keeps the validator set small and reusable.

#### Validation Class: Safe Path (`validate_safe_path`)

Prevents classic trojan horse attacks: an attacker places a malicious binary in a world-writable directory that appears early in PATH, or symlinks a path component to a location they control.

**Rules:**
1. Must be absolute (starts with `/`)
2. No NUL bytes
3. No `..` components (path traversal)
4. No shell metacharacters (`;`, `|`, `&`, `` ` ``, `$`, `(`, `)`, `{`, `}`, `<`, `>`, `\n`)
5. Path must resolve to an existing directory (for dir-type paths) or file (for socket-type)
6. Path must not be world-writable (`o+w`) — the classic trojan horse vector
7. Path must be owned by root or the current user — not by an arbitrary third party
8. No sticky-bit-less world-writable parent directories in the chain
9. Max length: `PATH_MAX` (4096 on Linux)

**Applies to:** `HOME`, `XDG_RUNTIME_DIR`, `SSH_AUTH_SOCK`, `TMPDIR` (Tier 1 reset target is also validated)

| Variable | Class | Additional Rules | Needed By |
|---|---|---|---|
| `HOME` | Safe Path (directory) | Must be a directory; must be owned by current UID | General — user home resolution |
| `XDG_RUNTIME_DIR` | Safe Path (directory) | Must be mode `0700`; must be owned by current UID (per XDG spec) | systemd, D-Bus |
| `XDG_CONFIG_HOME` | Safe Path (directory) | Default: `$HOME/.config`; must be owned by current UID | UMRS config files, tool preferences |
| `XDG_DATA_HOME` | Safe Path (directory) | Default: `$HOME/.local/share`; must be owned by current UID | UMRS data storage, quote corpus |
| `XDG_CACHE_HOME` | Safe Path (directory) | Default: `$HOME/.cache`; must be owned by current UID | UMRS cache (compiled translations, etc.) |
| `XDG_STATE_HOME` | Safe Path (directory) | Default: `$HOME/.local/state`; must be owned by current UID | UMRS state files, persistent tool state |
#### Validation Class: Colon-Delimited Path List (`validate_path_list`)

For PATH-like variables that contain multiple directories separated by `:`.

**Rules (per component):**
1. Every component passes `validate_safe_path` (directory variant)
2. Empty components are removed (empty = `.` = current directory = trojan horse)
3. Relative components (not starting with `/`) are removed
4. Duplicate components are removed (first occurrence wins)
5. Total component count capped at 64 (sanity)

**Note:** This validator is used for Tier 1 reset _verification_ (confirm the reset value is safe) and could be offered to external consumers who build PATH-like strings.

#### Validation Class: POSIX Locale (`validate_lang`)

**Rules:**
1. Max 128 chars
2. `C` and `POSIX` are always valid (short-circuit)
3. Must match: `^[a-zA-Z]{2,3}(_[a-zA-Z]{2,3})?(\.[\w-]+)?(@[\w]+)?$`
   - Language: 2-3 letter ISO 639 code
   - Territory: optional, 2-3 letter ISO 3166 code after `_`
   - Codeset: optional, after `.` — alphanumeric + `-` (e.g., `UTF-8`, `ISO-8859-1`)
   - Modifier: optional, after `@` — alphanumeric (e.g., `euro`)
4. No NUL bytes
5. No path separators (`/`), shell metacharacters, or whitespace
6. Codeset, if present, should be from a known set (warning in report if unknown, not rejection)

| Variable | Class | Additional Rules | Needed By |
|---|---|---|---|
| `LANG` | POSIX Locale | Primary locale; validated and preserved for i18n | `gettext-rs`, `setlocale()` |
| `LC_COLLATE` | POSIX Locale | — | Collation ordering |
| `LC_CTYPE` | POSIX Locale | — | Character classification |
| `LC_MESSAGES` | POSIX Locale | — | Message translation |
| `LC_MONETARY` | POSIX Locale | — | Currency formatting |
| `LC_NUMERIC` | POSIX Locale | — | Number formatting |
| `LC_TIME` | POSIX Locale | — | Date/time formatting |
| `LC_ALL` | POSIX Locale | Overrides all other LC_* | All locale facets |

#### Validation Class: Terminal Identifier (`validate_term`)

**Rules:**
1. Max 64 chars
2. Charset: `[a-zA-Z0-9._-]` only (alphanumeric + dot + hyphen + underscore)
3. No NUL bytes, no whitespace, no shell metacharacters
4. Preferred known values (not enforced, but logged as note if unknown):
   `xterm`, `xterm-256color`, `xterm-color`, `screen`, `screen-256color`,
   `tmux`, `tmux-256color`, `linux`, `vt100`, `vt220`, `dumb`, `rxvt`,
   `rxvt-unicode`, `rxvt-unicode-256color`, `alacritty`, `kitty`,
   `foot`, `ghostty`, `wezterm`
5. Value `dumb` is valid but triggers a warning in the report (no ANSI support)

| Variable | Class | Additional Rules | Needed By |
|---|---|---|---|
| `TERM` | Terminal Identifier | — | `crossterm`, `ratatui`, `colored` crate (implicit), `console::ansi` |
| `COLORTERM` | Enum | Must be exactly `truecolor` or `24bit` | True-color detection |

#### Validation Class: Positive Integer (`validate_positive_int`)

**Rules:**
1. Digits only (`^[0-9]+$`)
2. No leading zeros (except `0` itself, which is rejected as invalid dimension)
3. Range: 1–9999
4. No whitespace, no sign prefix

| Variable | Class | Additional Rules | Needed By |
|---|---|---|---|
| `COLUMNS` | Positive Integer | Range 1–9999 | `textwrap`, terminal width detection |
| `LINES` | Positive Integer | Range 1–9999 | `ratatui` terminal height |

#### Validation Class: Username (`validate_username`)

**Rules:**
1. Max 256 chars (Linux `LOGIN_NAME_MAX` is 256)
2. Charset: `[a-zA-Z0-9._-]` (POSIX portable username characters)
3. Must not start with `-` (option confusion)
4. Must not be empty
5. No NUL bytes, no whitespace

| Variable | Class | Additional Rules | Needed By |
|---|---|---|---|
| `USER` | Username | Cross-checked against `getuid()` — rejected if name doesn't match actual UID (AU-3) | Identity, logging |
| `LOGNAME` | Username | Cross-checked against `getuid()`; should match `USER` if both set (warning if mismatch) | POSIX identity |

#### Validation Class: Boolean Presence (`validate_presence`)

Per the `NO_COLOR` specification (https://no-color.org/): any non-empty value means "yes".

**Rules:**
1. Variable exists and is not empty → valid
2. No further content validation needed

| Variable | Class | Additional Rules | Needed By |
|---|---|---|---|
| `NO_COLOR` | Boolean Presence | Per https://no-color.org/ spec | `colored` crate (auto-honored) |

#### Validation Class: Timezone (`validate_tz`)

**Rules:**
1. Max 256 chars
2. If it looks like a POSIX TZ string (e.g., `EST5EDT`): charset `[a-zA-Z0-9:.,+-]`
3. If it looks like an Olson name (e.g., `America/New_York`): charset `[a-zA-Z0-9/_+-]`
4. No `..` components (path traversal — TZ is used to open files under `/usr/share/zoneinfo/`)
5. No NUL bytes, no shell metacharacters
6. Must not start with `/` unless it is an absolute zoneinfo path (which we reject — Olson names only)

| Variable | Class | Additional Rules | Needed By |
|---|---|---|---|
| `TZ` | Timezone | No absolute paths; no traversal | Time formatting, log timestamps |

#### Validation Class: D-Bus Address (`validate_dbus_address`)

**Rules:**
1. Max 1024 chars
2. Must start with `unix:` transport prefix only
3. If `unix:path=...` — the path portion passes `validate_safe_path` (socket)
4. If `unix:abstract=...` — alphanumeric + standard separators only
5. No NUL bytes, no shell metacharacters outside of the value portions
6. **Rejected transports (logged at `warn`):** `tcp:`, `nonce-tcp:` (no network D-Bus in air-gapped deployment), `unixexec:` (arbitrary command execution)

| Variable | Class | Additional Rules | Needed By |
|---|---|---|---|
| `DBUS_SESSION_BUS_ADDRESS` | D-Bus Address | Only `unix:` transport; `tcp:`/`unixexec:` rejected and logged at `warn` | Desktop integration |

#### Validation Summary Table

| Variable | Validation Class | Fail Action | Needed By |
|---|---|---|---|
| `HOME` | Safe Path (dir) | Remove | General |
| `USER` | Username | Remove | Identity, logging |
| `LOGNAME` | Username | Remove | POSIX identity |
| `LANG` | POSIX Locale | Remove | `gettext-rs` (i18n) |
| `LC_*` (all facets) | POSIX Locale | Remove individually | `setlocale()` |
| `TERM` | Terminal Identifier | Remove (fallback: `dumb`) | `crossterm`, `ratatui`, `colored` |
| `COLORTERM` | Enum (`truecolor`/`24bit`) | Remove | True-color detection |
| `NO_COLOR` | Boolean Presence | Remove | `colored` crate |
| `COLUMNS` | Positive Integer (1–9999) | Remove | `textwrap`, terminal width |
| `LINES` | Positive Integer (1–9999) | Remove | `ratatui` terminal height |
| `TZ` | Timezone | Remove | Time formatting, timestamps |
| `XDG_RUNTIME_DIR` | Safe Path (dir, mode 0700) | Remove | systemd, D-Bus |
| `XDG_CONFIG_HOME` | Safe Path (dir) | Remove | UMRS config |
| `XDG_DATA_HOME` | Safe Path (dir) | Remove | UMRS data |
| `XDG_CACHE_HOME` | Safe Path (dir) | Remove | UMRS cache |
| `XDG_STATE_HOME` | Safe Path (dir) | Remove | UMRS state |
| `DBUS_SESSION_BUS_ADDRESS` | D-Bus Address (unix only; tcp rejected) | Remove | Desktop integration |

**Tier 3: Strip unconditionally**

Everything not in Tier 1 or 2 is removed. Additionally, these are _always_ removed even if a tool passes them in `extra_allow`:

| Variable | Risk |
|---|---|
| `LD_PRELOAD` | Library injection |
| `LD_LIBRARY_PATH` | Library path hijack |
| `LD_AUDIT` | Audit hook injection |
| `LD_DEBUG` | Information leak |
| `LD_BIND_NOW` / `LD_BIND_NOT` | Linker behavior manipulation |
| `LD_PROFILE` / `LD_PROFILE_OUTPUT` | Profiling data leak |
| `LD_ORIGIN_PATH` | RPATH `$ORIGIN` override — path injection |
| `LD_SHOW_AUXV` | Dumps load addresses and AT_SECURE value — info leak |
| `LD_DYNAMIC_WEAK` | Linker behavior manipulation |
| `LD_USE_LOAD_BIAS` | Linker behavior manipulation |
| `GLIBC_TUNABLES` | **CVE-2023-4911 "Looney Tunables"** — heap overflow privilege escalation; RHEL 10 glibc affected |
| `TZDIR` | Overrides zoneinfo lookup directory — undermines `validate_tz()` |
| `NLSPATH` | glibc locale message catalog path — path injection |
| `GETCONF_DIR` | glibc `getconf` directory override |
| `PYTHONPATH` / `PYTHONSTARTUP` | Interpreter hijack |
| `PERL5LIB` / `PERL5OPT` | Interpreter hijack |
| `RUBYLIB` / `RUBYOPT` | Interpreter hijack |
| `NODE_PATH` / `NODE_OPTIONS` | Interpreter hijack |
| `IFS` | Shell parsing manipulation — no safe default in Rust |
| `CDPATH` | Shell behavior manipulation |
| `ENV` / `BASH_ENV` | Shell startup injection |
| `HISTFILE` | History file redirect |
| `POSIXLY_CORRECT` | Alters behavior of many tools unpredictably |
| `LOCPATH` | Locale data injection (glibc secure mode strips this) |
| `GCONV_PATH` | glibc charset conversion path injection |
| `HOSTALIASES` | DNS resolution manipulation |
| `RESOLV_HOST_CONF` | DNS config override |
| `NIS_PATH` | NIS lookup manipulation |
| `MALLOC_CHECK_` / `MALLOC_TRACE` | Memory debugging exposure |
| `SSH_AUTH_SOCK` | SSH agent credential access — lateral movement vector; opt-in via `scrub_env_with` |

This denylist mirrors what glibc's `__libc_secure_getenv()` / AT_SECURE / secure-execution mode blocks in the dynamic linker (`ld.so(8)`), extended for interpreter paths and recent CVEs.

**Key insight:** UMRS tools don't fork, so these vars can't directly harm the tool. But their *presence* in the environment is a finding. A process that inherits `GLIBC_TUNABLES` or `LD_PRELOAD` is running in an environment that someone has tampered with — or that has been carelessly configured. Either way, the auditor wants to know.

### Reusable Validators — Public API

All validators are public functions, independently useful outside of `scrub_env`. Each returns `Result<(), ValidationError>` where `ValidationError` is an enum with per-class variants describing exactly what failed.

```rust
/// Validation error types for environment variable values.
///
/// Each variant describes a specific validation failure, enabling callers
/// to programmatically handle or report failures.
///
/// # Controls
/// - NIST SP 800-53 SI-10 (information input validation)
/// - CWE-526 (environment variable exposure)
#[must_use = "validation failures indicate unsafe environment values"]
pub enum EnvValidationError {
    /// Path failed safety checks (ownership, permissions, traversal, etc.)
    UnsafePath { path: String, reason: String },
    /// PATH-list component failed validation
    UnsafePathComponent { component: String, reason: String },
    /// Locale string is syntactically invalid
    InvalidLocale { value: String, reason: String },
    /// Terminal identifier contains unsafe characters
    InvalidTerm { value: String, reason: String },
    /// Username contains invalid characters or is empty
    InvalidUsername { value: String, reason: String },
    /// Timezone string contains traversal or unsafe content
    InvalidTimezone { value: String, reason: String },
    /// D-Bus address uses rejected transport or unsafe paths
    InvalidDbusAddress { value: String, reason: String },
    /// Numeric value out of range or non-numeric
    InvalidInteger { value: String, reason: String },
    /// Generic: value too long, contains NUL, etc.
    Malformed { variable: String, reason: String },
}
```

The full validation rules for each class are documented in the Tier 2 specification above. Key design points:

- **`validate_safe_path`** — NIST SP 800-53 CM-7, AC-3, SI-7. Checks ownership, permissions (no world-writable), traversal, and symlink chains — this is the classic trojan horse prevention that `sudo`, `pkexec`, and `ld.so` all implement
- **`validate_path_list`** — NIST SP 800-53 CM-7, AC-3, SI-7. Applies `validate_safe_path` per component and strips empty/relative entries (empty PATH component = `.` = current directory = trojan horse)
- **`validate_lang`** — NIST SP 800-53 SI-10. The one most likely to be reused by external consumers
- **`validate_term`** — NIST SP 800-53 SI-10. Terminal identifier syntax safety
- **`validate_username`** — NIST SP 800-53 AU-3, AC-3. Username syntax + UID cross-check
- **`validate_tz`** — NIST SP 800-53 SI-10, AU-8. Timezone string traversal and injection prevention
- **`validate_dbus_address`** — NIST SP 800-53 CM-7, AC-3. D-Bus transport restriction (unix only)
- **`validate_positive_int`** — NIST SP 800-53 SI-10. Numeric range check for display dimensions
- All validators are pure functions with no side effects — they don't modify the environment

### `scrub_env_with()` — Extended allowlist

```rust
/// Scrub the process environment, preserving additional tool-specific variables.
///
/// Extra variables are subject to the same validation as Tier 2 defaults.
/// Variables on the hardcoded denylist (LD_PRELOAD, etc.) cannot be preserved
/// even if listed in `extra_allow`.
///
/// # Controls
/// - NIST SP 800-53 CM-7 (least functionality), AC-3 (access enforcement), SI-7
/// - CWE-526, CERT ENV03-C
#[must_use = "environment scrub results should be inspected or logged"]
pub fn scrub_env_with(extra_allow: &[&str]) -> ScrubReport
```

### Return Type

```rust
#[must_use = "environment scrub results should be inspected or logged"]
pub struct ScrubReport {
    /// The validated environment, ready to apply to child processes.
    pub env: SanitizedEnv,
    /// Denylist vars that were present and stripped (anomalies — logged at warn).
    pub stripped: Vec<ScrubEntry>,
    /// Vars reset to safe defaults (PATH, TMPDIR).
    pub reset: Vec<ScrubEntry>,
    /// Vars that passed validation and are in the sanitized env.
    pub preserved: Vec<String>,
    /// Vars removed because their values failed validation.
    pub failed_validation: Vec<ScrubEntry>,
    /// Vars removed because they weren't in any tier (unknown).
    pub unknown_removed: Vec<String>,
}

/// Opaque wrapper for environment values from untrusted sources.
/// Does not implement Display — cannot accidentally leak into log output.
/// Use `.reveal()` for programmatic audit access only.
///
/// # Controls
/// - NIST SP 800-53 SI-11 (error handling — prevent sensitive value leakage)
pub struct SensitiveValue(String);
impl SensitiveValue {
    pub fn reveal(&self) -> &str { &self.0 }
}

/// Detail record for a scrubbing action.
pub struct ScrubEntry {
    pub name: String,
    pub reason: String,                        // why it was stripped/reset/rejected
    pub original_value: Option<SensitiveValue>, // None if suppressed for security; use .reveal() for audit
    pub new_value: Option<String>,             // Some for reset vars, None for stripped
}
```

**Logging behavior:**
- `stripped` entries → `log::warn!("env scrub: stripped {name}: {reason}")` — these are anomalies
- `failed_validation` entries → `log::warn!("env scrub: rejected {name}: {reason}")` — funky values are suspicious, not routine
- `reset` entries → `log::info!("env scrub: reset {name} to safe default")` — expected behavior
- `original_value` is **never** included in log output for denylist vars (they may contain injected content). Only the name and reason are logged. The value is wrapped in `SensitiveValue` (no `Display` impl) — accidental logging is a compile error. Use `.reveal()` for programmatic audit access only.

### Architecture: Validator + Builder, Not Mutator (Option A)

`scrub_env` does **not** mutate the parent process environment. This preserves `#![forbid(unsafe_code)]` in `umrs-core` — `std::env::remove_var` / `set_var` are `unsafe` in edition 2024 when threads may exist, and `#![forbid]` cannot be overridden by inner `#[allow]`.

Instead, `scrub_env` reads the current environment, validates it, and returns a `ScrubReport` containing a `SanitizedEnv` — a validated environment map ready to apply to child processes via `Command::env_clear()` + selective re-injection.

```rust
/// A validated, immutable snapshot of the process environment.
///
/// Every value in this map has passed its validation class — path safety,
/// locale syntax, terminal identifier rules, etc. Use this instead of raw
/// `std::env::var()` for trusted access to environment values.
///
/// For the rare case where a tool must spawn a subprocess:
/// `env.apply(&mut cmd)` provides `Command::env_clear()` + validated
/// re-injection in one call.
///
/// The parent process environment is never modified.
///
/// # Controls
/// - NIST SP 800-53 SI-10 (information input validation)
/// - NIST SP 800-53 SI-7 (runtime configuration integrity)
/// - CWE-526 (environment variable exposure)
#[must_use = "sanitized environment contains validated values — use instead of raw env access"]
pub struct SanitizedEnv { /* BTreeMap<String, String> internally */ }

impl SanitizedEnv {
    /// Get a validated environment variable by name.
    /// Returns None if the variable was not in the allowlist or failed validation.
    /// This is the primary API — use this instead of std::env::var().
    pub fn get(&self, key: &str) -> Option<&str>

    /// Iterate over all validated environment variables.
    pub fn vars(&self) -> impl Iterator<Item = (&str, &str)>

    /// Apply this sanitized environment to a Command (env_clear + re-inject).
    /// Use only when spawning a subprocess is unavoidable.
    pub fn apply(&self, cmd: &mut Command) -> &mut Command

    /// Check if a specific variable survived validation.
    pub fn contains(&self, key: &str) -> bool
}
```

**Usage pattern in tools:**
```rust
fn main() {
    let report = umrs_core::init::init_tool(verbose);

    // Instead of: std::env::var("HOME")  ← untrusted
    // Use:        report.env.get("HOME") ← validated
    if let Some(home) = report.env.get("HOME") {
        // home is guaranteed: absolute path, owned by current UID,
        // not world-writable, no traversal, no NUL bytes
    }
}
```

### Behavior

1. Read all current env vars (snapshot, no mutation)
2. Classify each var against the three tiers
3. Strip denylist entries unconditionally — log at `warn` (these are anomalies)
4. Reset Tier 1 vars to safe defaults
5. Validate Tier 2 vars; reject those that fail — log at `info` with reason
6. Discard everything not in any tier or `extra_allow`
7. Return `ScrubReport` containing `SanitizedEnv` + full audit trail

### Controls

- NIST SP 800-53 CM-7 (least functionality — strip unnecessary env vars)
- NIST SP 800-53 AC-3 (access enforcement — prevent env-based privilege manipulation)
- NIST SP 800-53 SI-7 (runtime configuration integrity)
- NIST SP 800-53 SI-10 (information input validation — LANG/TERM/display var validation)
- CWE-526 (exposure of sensitive information through environment variables)
- CERT ENV03-C (sanitize environment when invoking external programs)
- OWASP A2/A3 (sensitive data exposure / security misconfiguration — secrets must not be stored in env vars when the runtime allows other processes or logs to read them)

### Reference: glibc `__libc_secure_getenv()` / AT_SECURE

The glibc runtime provides `secure_getenv()` which returns NULL for sensitive variables when the process is in secure-execution mode (setuid, capability elevation, AT_SECURE set by kernel). The dynamic linker (`ld.so`) blocks a defined set of variables in this mode including `LD_PRELOAD`, `LD_LIBRARY_PATH`, `LD_AUDIT`, `LD_DEBUG`, `LOCPATH`, `GCONV_PATH`, `MALLOC_TRACE`, and others.

Our denylist is a superset of what glibc blocks in secure-execution mode, extended with interpreter paths (`PYTHONPATH`, `NODE_PATH`, etc.) that glibc doesn't handle because they are userspace concerns.

We implement this in pure Rust (no FFI to `clearenv()` or `secure_getenv()`) because:
- `#![forbid(unsafe_code)]` — project-wide constraint
- We need allowlist semantics, not just denylist (glibc only denylists)
- We need validation, not just presence/absence checks
- We want the report for auditability

---

## Phase 2: Logging Initialization (`init_logging`)

### Design

Journald-native structured logging with `"umrs"` as the syslog identifier tag.

**Requirements:**
- Native journald backend — structured fields, not just text lines
- Tag: `"umrs"` (all tools share a single syslog identifier for unified filtering)
- Default level: `Warn` (operators see problems, not noise)
- Verbose level: `Debug`
- No log output to stdout (that's for tool output)
- High-availability fallback cascade (see below)

### Logging Fallback Cascade

UMRS takes its audit logging responsibility seriously. If the primary backend is unavailable, the tool does not silently drop records — it falls through to the next available backend and reports what happened. Silent logging failure is a compliance violation (AU-9).

```
  ┌─────────────┐    socket present?    ┌───────────────────┐
  │  journald   │ ──── yes ───────────→ │ ACTIVE (primary)  │
  │  (primary)  │                        │ Structured fields │
  └──────┬──────┘                        │ "umrs" syslog tag │
         │ no                            └───────────────────┘
         ▼
  ┌─────────────┐    always available    ┌───────────────────┐
  │   stderr    │ ──── yes ───────────→ │ FALLBACK          │
  │  (fallback) │                        │ Timestamp + level │
  └──────┬──────┘                        │ prefix, plain text│
         │ somehow no                    └───────────────────┘
         ▼
  ┌─────────────┐
  │   Failed    │ → LoggingBackend::Failed returned to caller
  │  (nothing)  │   Tool decides: abort or continue unlogged
  └─────────────┘
```

**Why stderr is a solid fallback:**
- If the tool runs under systemd, stderr is captured by journald anyway (the record still reaches the journal)
- If the tool runs interactively, the operator sees problems immediately
- No FFI, no new dependencies, no security surface
- Always available — stderr cannot "fail to connect"

### Why syslog is explicitly excluded

| Concern | Detail |
|---|---|
| On RHEL 10, syslog IS journald | `/dev/log` is a symlink to journald's socket — "falling back to syslog" is just a different path to the same destination that already failed |
| FFI required | The Rust `syslog` crate calls libc's `openlog`/`syslog` — violates `#![forbid(unsafe_code)]` and the "avoid FFI" rule |
| Plaintext only | No structured fields, no reliable parsing — fails AU-3 (content of audit records) |
| Not a real fallback | If journald's socket is gone, `/dev/log` is also gone on any systemd system |

**Decision:** Syslog is not part of the cascade. Do not add it. If someone proposes it in the future, point them here.

### AU-5 Response to Audit Processing Failures — Why UMRS Continues Operating

NIST SP 800-53 AU-5 requires defined actions when audit processing fails. In high-assurance enforcement systems (MLS guards, reference monitors), the strongest response is "halt processing" — if you can't audit decisions, stop making them.

**UMRS tools are observation and reporting tools, not enforcement mechanisms.** They do not make security decisions — they observe and display security posture. The appropriate AU-5 response is:

1. Fall back to stderr (best effort — operator may still see it)
2. Display a visible warning in the tool's UI that logging is degraded
3. **Continue operating** — refusing to show the operator their security posture because the logger is degraded makes the situation *worse*, not better

If journald is down on RHEL 10, the system has catastrophic problems far beyond a UMRS tool not being able to log. Our tool stopping would deprive the operator of the security visibility they need most during a crisis.

> **AU-5 compliance statement:** UMRS tools detect audit processing failure
> (journald unavailable), alert the operator (stderr warning + UI indication),
> and continue operation in degraded logging mode. Halting is not appropriate
> because UMRS is an observation tool — stopping observation during a system
> failure deprives the operator of security-relevant information. The tool
> reports its logging backend status via `LoggingBackend` so automated
> monitoring can detect and escalate degraded audit posture.

### Auditor-Facing Rationale (AU-9 Compliance)

> UMRS tools implement a documented, two-tier logging fallback cascade:
> primary (journald with structured fields) → fallback (stderr with
> timestamps). The tool reports which backend is active via the
> `LoggingBackend` return value, enabling the operator and audit system
> to detect degraded logging posture. Traditional syslog was evaluated
> and explicitly excluded: on the target platform (RHEL 10), the syslog
> socket is a symlink to journald, providing no additional availability;
> the Rust syslog crate requires FFI to libc, which violates the
> project's `#![forbid(unsafe_code)]` constraint; and plaintext syslog
> lacks the structured fields required by AU-3. The tool never silently
> drops audit records — if all backends fail, `LoggingBackend::Failed`
> is returned and the tool decides whether to abort or continue with
> explicit acknowledgement that logging is unavailable.

### Signature

```rust
/// Logging backend status returned by `init_logging`.
///
/// The cascade is: journald → stderr → Failed.
/// Syslog is explicitly excluded (see plan rationale — FFI required,
/// redundant with journald on RHEL 10, no structured fields).
///
/// - `Journald` — primary backend active (expected path)
/// - `StderrFallback` — journald unavailable; stderr active; logged at `warn`
/// - `Failed` — no backend available; logged at `error`; tool decides next step
///
/// # Controls
/// - NIST SP 800-53 AU-9 (protection of audit information — fallback ensures records are not silently dropped)
/// - NIST SP 800-53 SI-11 (error handling — degradation is reported, not hidden)
/// - NSA RTB Fail Secure (logging failure is explicit, never silent)
#[must_use = "logging backend status must be inspected — silent logging failure violates AU-9"]
pub enum LoggingBackend {
    /// Primary: journald with structured fields and "umrs" syslog tag.
    Journald,
    /// Fallback: stderr with timestamp + level prefix.
    /// If running under systemd, stderr still reaches journald.
    StderrFallback,
    /// All backends failed. Tool must decide: abort or continue unlogged.
    Failed(String),
}

/// Initialize the UMRS logging subsystem.
///
/// Attempts journald first (socket at `/run/systemd/journal/socket`),
/// falls back to stderr if unavailable, returns `Failed` if neither works.
///
/// # Controls
/// - NIST SP 800-53 AU-3 (content of audit records)
/// - NIST SP 800-53 AU-8 (time stamps)
/// - NIST SP 800-53 AU-9 (protection of audit information)
/// - NIST SP 800-53 AU-12 (audit record generation)
/// - NIST SP 800-53 SI-11 (error handling)
/// - NSA RTB Fail Secure
#[must_use = "logging backend status must be inspected — silent logging failure violates AU-9"]
pub fn init_logging(verbose: bool) -> LoggingBackend
```

**Dependencies:** `log` + `systemd-journal-logger` (pure Rust journald backend, no FFI)

---

## Phase 3: i18n with Auto-Domain (`init_i18n`)

### Design

Wraps existing `umrs_core::i18n::init()` with auto-detection of the binary name as the gettext domain.

**Signature:**

```rust
/// Initialize UMRS internationalization.
///
/// If `domain` is `None`, the textdomain is derived from the current
/// binary name (e.g., `umrs-ls` → domain `"umrs-ls"`).
///
/// # Controls
/// - NSA RTB RAIN (non-bypassable locale setup)
/// - NIST SP 800-53 AU-3 (audit record content — locale affects log message format)
pub fn init_i18n(domain: Option<&'static str>)
```

**Auto-detection logic:**
1. `std::env::current_exe()` → extract filename stem
2. Strip platform suffixes (`.exe`, etc.)
3. Fall back to `"umrs"` if detection fails

---

## Phase 4: Convenience Wrapper (`init_tool`)

### Signature

```rust
/// Initialize a UMRS tool with default settings.
///
/// Performs environment scrubbing, logging initialization, and i18n setup
/// (domain auto-detected from binary name) in the recommended order.
/// Returns a ScrubReport containing the SanitizedEnv for child process use.
///
/// # Controls
/// - NIST SP 800-53 CM-7, AC-3, SI-7, SI-10, AU-3, AU-8, AU-9, AU-12, SI-11
/// - NSA RTB RAIN
/// - CWE-526, CERT ENV03-C
#[must_use = "scrub report contains sanitized environment for child processes"]
pub fn init_tool(verbose: bool) -> ScrubReport
```

### Behavior

```
1. scrub_env()           — validate environment, build SanitizedEnv
2. init_logging(verbose) — journald-native logging → returns LoggingBackend
3. log logging backend status:
   - LoggingBackend::Journald     → (no log — expected path)
   - LoggingBackend::StderrFallback → log::warn!
   - LoggingBackend::Failed(msg)  → log::error!
4. log scrub report:
   - stripped (denylist) entries at warn
   - validation failures at warn
   - reset entries at info
5. init_i18n(None)       — auto-detect domain from binary name (warns on fallback)
```

Order matters:
- Scrub first (i18n reads `LANG`/`LC_*`, which survive the allowlist after validation)
- Logging before i18n — init-time diagnostic messages (scrub report, logging status) are operator-facing and stay in English for audit trail consistency. Translation applies to user-facing tool output, not internal diagnostics.
- i18n last (so tool output is translatable, but init diagnostics are stable English)

---

## Phase 5: Tests

Tests split across files by validation class in `umrs-core/tests/`:

### `init_scrub_tests.rs` — Environment scrubbing integration
1. `scrub_env` removes denylist vars (`LD_PRELOAD`, etc.)
2. `scrub_env` preserves default allowlist vars
3. `scrub_env` resets `PATH` to safe default
4. `scrub_env` resets `TMPDIR` to `/tmp`
5. `scrub_env` returns accurate `ScrubReport` (all four vectors)
6. `scrub_env_with` preserves extra allowlist entries
7. `scrub_env_with` refuses to preserve denylist vars even when requested
8. `scrub_env` removes vars that fail validation (e.g., `HOME=../../../etc`)
9. `init_tool` runs without panic (integration test)
10. `GLIBC_TUNABLES` is present in the denylist and stripped unconditionally
11. Denylist completeness: loop ALL denylist entries and verify each is stripped by `scrub_env`
12. Symlink chain: path with symlink to a different-user-owned directory is rejected by `validate_safe_path`
13. `init_tool` idempotency: calling `init_tool` twice does not panic
14. `init_tool` logs scrub report after logging is initialized (scrub report entries appear in log output)

### `validate_path_tests.rs` — Path safety (trojan horse prevention)
15. Accepts absolute path owned by current user
16. Rejects relative paths
17. Rejects paths with `..` components
18. Rejects world-writable directories (the classic trojan horse)
19. Rejects paths owned by arbitrary third-party UID
20. Rejects paths containing NUL bytes
21. Rejects paths containing shell metacharacters (`;`, `|`, `$`, etc.)
22. Rejects paths exceeding `PATH_MAX`
23. `XDG_RUNTIME_DIR` requires mode `0700` per XDG Base Directory spec
24. `SSH_AUTH_SOCK` requires `S_ISSOCK` file type

### `validate_path_list_tests.rs` — PATH-list component validation
25. Accepts well-formed PATH with standard system directories
26. Strips empty components (empty = `.` = current dir = trojan)
27. Strips relative components
28. Strips world-writable directory components
29. Strips duplicates (first occurrence wins)
30. Rejects lists exceeding 64 components

### `validate_lang_tests.rs` — POSIX locale validation
31. Accepts `C`, `POSIX`
32. Accepts `en_US`, `en_US.UTF-8`, `fr_CA.UTF-8@euro`
33. Rejects path traversal (`../../etc/passwd`)
34. Rejects NUL bytes, shell metacharacters
35. Rejects overlength strings (>128 chars)
36. Warns on unknown codeset (does not reject)

### `validate_term_tests.rs` — Terminal identifier validation
37. Accepts known values (`xterm-256color`, `screen`, `linux`, `kitty`, `alacritty`)
38. Accepts unknown but syntactically valid values (logged as note)
39. Rejects shell metacharacters
40. Rejects overlength strings (>64 chars)
41. Notes `dumb` terminal as warning in report

### `validate_misc_tests.rs` — Username, integer, TZ, D-Bus
42. Username: accepts `jadams`, `root`, `j.adams-2`
43. Username: rejects empty, starts with `-`, shell metacharacters
44. `COLUMNS`/`LINES`: accepts `80`, `132`, `9999`; rejects `0`, `-1`, `abc`, `99999`
45. TZ: accepts `UTC`, `EST5EDT`, `America/New_York`
46. TZ: rejects `../../../etc/passwd`, absolute paths
47. D-Bus: accepts `unix:path=/run/user/1000/bus`
48. D-Bus: rejects `unixexec:` transport
49. D-Bus: rejects `tcp:` transport (air-gapped deployment — no network D-Bus)
50. `USER`/`LOGNAME`: rejects when username does not match actual UID via `getuid()` cross-check

---

## Implementation Notes

- **No unsafe — by design.** Option A (child-process scrubbing) means `scrub_env` only reads the environment via `std::env::vars()` (safe) and builds a `SanitizedEnv` map. No `remove_var` / `set_var` calls. `#![forbid(unsafe_code)]` is fully honored.
- **No FFI** — no `clearenv()` or `secure_getenv()` needed.
- **`rustix` for path validation** — use `rustix = { version = "0.38", features = ["fs", "process"] }`. Pin to 0.38 to match `umrs-platform`, `umrs-selinux`, `umrs-hw`. Resolve `umrs-tui` version conflict (`rustix = "1"`) before adding.
- **`thiserror` for error types** — `EnvValidationError` derives `thiserror::Error`. Already in workspace.
- **`#[non_exhaustive]` on `EnvValidationError`** — new variants expected as the validator set grows.
- **`#[must_use]` on validator functions** (not on the error type) — per project rules, with message string.
- **`NO_COLOR` dual path** — the `colored` crate reads `NO_COLOR` independently at output time. Preserving it in `SanitizedEnv` is for subprocess inheritance. Document this.
- **`USER`/`LOGNAME` cross-check** — validate syntax AND cross-check against `getuid()`. Reject `USER=root` from a non-root process. Controls: AU-3.
- **Error information discipline** — `original_value` for denylist vars is wrapped in `SensitiveValue` (no `Display` impl), making accidental log leakage a compile error. Only name + reason are logged. The value is available via `.reveal()` in `ScrubReport` for programmatic audit.
- **Pre-existing `unwrap`/`expect` in `umrs-core`** — fix `validate.rs` and `typography.rs` before merging new `init` module. Otherwise clippy blocks the whole crate.
- **Compliance annotations** — module-level and on each public function. Logging module needs AU-8 (time stamps), AU-9 (protection of audit information), SI-11 (error handling).

---

## File Changes

| File | Change |
|---|---|
| `umrs-core/src/init/mod.rs` | Module root — re-exports public API (`scrub_env`, `init_tool`, validators) |
| `umrs-core/src/init/scrub.rs` | `scrub_env()`, `scrub_env_with()`, `ScrubReport`, allowlist/denylist definitions |
| `umrs-core/src/init/logging.rs` | `init_logging()`, journald backend setup |
| `umrs-core/src/init/i18n.rs` | `init_i18n()`, binary name auto-detection |
| `umrs-core/src/init/tool.rs` | `init_tool()` convenience wrapper |
| `umrs-core/src/init/validate/mod.rs` | Validator module root — re-exports all validators, `EnvValidationError` enum |
| `umrs-core/src/init/validate/path.rs` | `validate_safe_path()`, `validate_path_list()` — ownership, perms, trojan horse checks |
| `umrs-core/src/init/validate/locale.rs` | `validate_lang()` — POSIX locale syntax |
| `umrs-core/src/init/validate/term.rs` | `validate_term()` — terminal identifier, known-value catalog |
| `umrs-core/src/init/validate/username.rs` | `validate_username()` — POSIX portable username rules |
| `umrs-core/src/init/validate/tz.rs` | `validate_tz()` — timezone string safety |
| `umrs-core/src/init/validate/dbus.rs` | `validate_dbus_address()` — D-Bus transport validation |
| `umrs-core/src/init/validate/integer.rs` | `validate_positive_int()` — COLUMNS/LINES range check |
| `umrs-core/src/lib.rs` | Add `pub mod init;` |
| `umrs-core/Cargo.toml` | Add `log`, `systemd-journal-logger`, `nix` (for ownership/permission checks) |
| `umrs-core/tests/init_scrub_tests.rs` | Scrubbing integration tests (14 cases) |
| `umrs-core/tests/validate_path_tests.rs` | Path safety tests (10 cases) |
| `umrs-core/tests/validate_path_list_tests.rs` | PATH-list tests (6 cases) |
| `umrs-core/tests/validate_lang_tests.rs` | POSIX locale tests (6 cases) |
| `umrs-core/tests/validate_term_tests.rs` | Terminal identifier tests (5 cases) |
| `umrs-core/tests/validate_misc_tests.rs` | Username, integer, TZ, D-Bus tests (9 cases) |
| docs | Developer guide section on tool initialization pattern + validator API reference |

---

## Future Considerations

- **`SanitizedEnv` as the tool's canonical environment** — long-term, tools should never call `std::env::var()` directly. All env access goes through `SanitizedEnv.get()`. This is enforceable via clippy custom lints or code review.
- **`detect_secure_execution()`** — check if the process is running in glibc secure-execution mode (AT_SECURE flag, set when real/effective UID/GID differ — i.e., setuid/setgid/capability-elevated). When detected, the scrub report should flag this at `warn` level. Could read `/proc/self/auxv` for `AT_SECURE` or compare real vs effective UID/GID via `rustix::process`. This enables tools to automatically escalate their logging posture when running privileged.
- **Posture integration** — the `ScrubReport` stripped/failed entries are security findings. They could feed into the `umrs-platform` posture assessment as `SecurityObservation` variants (per the Security Findings as Data rule). An environment with `LD_PRELOAD` set is a posture signal, not just a log line.
- **Audit integration** — log scrub report to `umrs-logspace` audit trail (not just debug log).
- **Standalone crate potential** — The validator set (`validate_safe_path`, `validate_lang`, `validate_path_list`, `validate_term`, `validate_username`, `validate_tz`) plus `scrub_env` is independently valuable to the Rust ecosystem. Environment scrubbing is a long-standing pain point — there is no good Rust crate for it. Consider publishing as a standalone crate (e.g., `env-scrub` or `secure-env`) after the API stabilizes in UMRS.
- **Configurable strictness levels** — Some deployments may want stricter validation (e.g., reject unknown TERM values entirely). A builder pattern or strictness enum could support this without breaking the simple API.
- **`/proc/<pid>/environ` watcher** — For long-running daemons, periodically verify that the process environment hasn't been modified externally (defense in depth, not a replacement for scrubbing).
