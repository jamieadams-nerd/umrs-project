# Plan: UMRS Tool Initialization API (`umrs-core::init`)

**Status:** Approved — ready for implementation. Split into 8 sub-phases for manageable agent work units.

## Sub-Phase Execution Order

Each sub-phase is a focused, completable unit. No agent should work on more than one at a time.
Dependencies are explicit — do not start a phase until its prerequisites are done.

| Sub-Phase | Description | Prerequisites | Estimated scope |
|---|---|---|---|
| **1a** | `EnvValidationError` type + simple validators (locale, term, username, tz, integer, hostname, identifier, enum_set, log_level) | None | ~12 files, ~50 tests |
| **1b** | Complex validators (path safety, path list, dbus, device, selinux) | 1a (uses `EnvValidationError`) | ~6 files, ~25 tests |
| **1c** | `ScrubReport`, `SanitizedEnv`, `SensitiveValue` types + `scrub_env()` / `scrub_env_with()` | 1a + 1b (uses all validators) | ~2 files, ~14 tests |
| **1d** | `init_logging()` — journald backend + fallback cascade | None (independent) | ~1 file, ~5 tests |
| **1e** | `init_i18n()` — auto-domain detection wrapper | None (independent) | ~1 file, ~3 tests |
| **1f** | `init_tool()` convenience wrapper + `init/mod.rs` re-exports | 1a–1e (combines all) | ~2 files, ~3 tests |
| **2** | `umrs-env` binary crate — CLI tool | 1f (uses `init_tool`) | New crate, ~3 files, ~10 tests |
| **3** | Env var corpus research (researcher task) | None (independent, can run in parallel) | Research only |

**Parallel opportunities:**
- 1a, 1d, 1e, and 3 can all start simultaneously
- 1b starts after 1a completes
- 1c starts after 1b completes
- 1f starts after 1a–1e all complete
- 2 starts after 1f completes

**Swim buddy rule:** If any sub-phase takes more than ~45 minutes without visible progress (test passing, files committed to working tree), the agent must surface a status update.

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
- **Reusable for the ecosystem.** The validator set (path safety, locale, terminal, timezone, username, SELinux components, hostnames, device paths) doesn't exist as a Rust crate. This is independently publishable.
- **Zero side effects.** The parent process is untouched. Nothing breaks. The module is a read-only audit + validation layer.
- **Advisory, not prescriptive.** The library reports what's wrong and why. The `umrs-env` tool shows the operator their environment through a security lens — what's clean, what's suspicious, what failed validation. The operator chooses what to do about it. We tell them what's jacked up; they decide whether to fix it, source the clean output, or just take note.

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
    ├── integer.rs      ← validate_positive_int()
    ├── log_level.rs    ← validate_log_level()
    ├── device.rs       ← validate_device_path()
    ├── hostname.rs     ← validate_hostname()
    ├── enum_set.rs     ← validate_enum() — generic enum membership check
    ├── selinux.rs      ← validate_selinux_component() — role/level syntax via umrs-selinux
    └── identifier.rs   ← validate_safe_identifier()
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
19. **`RUST_LOG`** — Tier 2 (preserve + validate). Validated as a restricted enum: only `error`, `warn`, `info`, `debug`, `trace` are accepted (case-insensitive). Module-level filters (e.g., `umrs_selinux=debug`) are also accepted but only for `umrs_*` crate prefixes — arbitrary module paths are rejected. This prevents an attacker from enabling verbose logging on third-party dependencies to extract timing or data side-channels. Controls: AU-9 (protection of audit information), CM-7 (least functionality).
20. **`umrs-env` tool** — New binary crate. Acts like `env(1)` but produces scrubbed/sanitized output. Reads from inherited environment by default, or from stdin (pipe `env | umrs-env` or `echo $HOME | umrs-env`). Uses the `umrs_core::init` scrub engine. See dedicated section below.

---

## Phase 1a: Error Type + Simple Validators

**Scope:** Create `umrs-core/src/init/validate/` module tree. Implement `EnvValidationError` enum and these validators:
- `validate_lang()` — locale.rs
- `validate_term()` — term.rs
- `validate_username()` — username.rs
- `validate_tz()` — tz.rs
- `validate_positive_int()` — integer.rs
- `validate_hostname()` — hostname.rs
- `validate_safe_identifier()` — identifier.rs
- `validate_enum()` — enum_set.rs
- `validate_log_level()` — log_level.rs

**Tests:** `validate_lang_tests.rs`, `validate_term_tests.rs`, `validate_misc_tests.rs`, `validate_log_level_tests.rs`

**Deliverable:** All simple validators pass their tests. `EnvValidationError` is `#[non_exhaustive]` with `thiserror::Error`.

**Files:** `validate/mod.rs`, `validate/locale.rs`, `validate/term.rs`, `validate/username.rs`, `validate/tz.rs`, `validate/integer.rs`, `validate/hostname.rs`, `validate/identifier.rs`, `validate/enum_set.rs`, `validate/log_level.rs` + corresponding test files.

---

## Phase 1b: Complex Validators (Path, D-Bus, Device, SELinux)

**Prerequisites:** Phase 1a complete (uses `EnvValidationError`).

**Scope:** Implement validators that require filesystem access or cross-crate dependencies:
- `validate_safe_path()` — path.rs (ownership, permissions, symlink chain, traversal)
- `validate_path_list()` — path.rs (colon-delimited PATH validation)
- `validate_dbus_address()` — dbus.rs (transport restriction)
- `validate_device_path()` — device.rs (`/dev/pts/*`, `/dev/tty*`)
- `validate_selinux_component()` — selinux.rs (role/level syntax via `umrs-selinux`)

**Tests:** `validate_path_tests.rs`, `validate_path_list_tests.rs` + additions to `validate_misc_tests.rs`

**Note:** `rustix` version conflict with `umrs-tui` must be resolved before adding to `umrs-core` Cargo.toml.

**Deliverable:** All path/dbus/device/selinux validators pass. Symlink chain validation confirmed.

---

## Phase 1c: Scrub Engine (`ScrubReport`, `SanitizedEnv`, `scrub_env`)

**Prerequisites:** Phases 1a + 1b complete (uses all validators).

**Scope:** Implement the core scrub engine:
- `ScrubReport`, `SanitizedEnv`, `SensitiveValue` types in `scrub.rs`
- `scrub_env()` and `scrub_env_with()` functions
- Three-tier classification (Reset, Preserve+Validate, Strip)
- Full denylist
- Logging behavior (warn/info levels per tier)

**Tests:** `init_scrub_tests.rs` (14 cases)

**Deliverable:** `scrub_env()` produces a correct `ScrubReport` for a real process environment.

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
| `SHELL` | Safe Path (file) | Must be executable; should exist in `/etc/shells` | Login shell identification |
| `MAIL` | Safe Path (file) | Typically `/var/spool/mail/$USER`; must be owned by current UID or root | Mail spool |
| `PWD` | Safe Path (directory) | Must match actual CWD (`getcwd()`) — mismatch is suspicious | Current working directory |
| `OLDPWD` | Safe Path (directory) | May not exist (directory was deleted); non-existence is a note, not rejection | Previous working directory |
#### Validation Class: Colon-Delimited Path List (`validate_path_list`)

For PATH-like variables that contain multiple directories separated by `:`.

**Rules (per component):**
1. Every component passes `validate_safe_path` (directory variant)
2. Empty components are removed (empty = `.` = current directory = trojan horse)
3. Relative components (not starting with `/`) are removed
4. Duplicate components are removed (first occurrence wins)
5. Total component count capped at 64 (sanity)

| Variable | Class | Additional Rules | Needed By |
|---|---|---|---|
| `XDG_DATA_DIRS` | Colon-Delimited Path List | Default: `/usr/local/share:/usr/share` if unset | System-wide data directories |

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
| `RUST_LOG` | Log Level Enum | See validation class below | `env_logger`, `tracing-subscriber`, debug diagnostics |

#### Validation Class: Log Level Enum (`validate_log_level`)

**Rules:**
1. Max 256 chars
2. Simple form: must be one of `error`, `warn`, `info`, `debug`, `trace` (case-insensitive)
3. Module-filtered form: `<module>=<level>` where `<module>` must start with `umrs_` — only UMRS crate modules are permitted. No arbitrary third-party module targeting.
4. Comma-separated list of the above forms is accepted (e.g., `info,umrs_selinux=debug`)
5. No NUL bytes, no shell metacharacters, no whitespace outside of the value
6. `off` is accepted (disables logging entirely)
7. Numeric level values are rejected (use named levels only)

**Rationale:** An attacker who controls `RUST_LOG` can enable verbose debug output on third-party dependencies, potentially extracting timing side-channels, internal state, or memory layout information through log output. Restricting module filters to `umrs_*` prefixes limits the blast radius.

| Variable | Class | Additional Rules | Needed By |
|---|---|---|---|
| `RUST_LOG` | Log Level Enum | Only `umrs_*` module prefixes; no arbitrary crate targeting | `env_logger`, `tracing-subscriber`, debug diagnostics |

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
| `HISTSIZE` | Positive Integer | Range 1–100000 | Shell history size |
| `SHLVL` | Positive Integer | Range 1–999 | Shell nesting level |
| `XDG_SESSION_ID` | Positive Integer | Range 1–999999 | logind session ID |

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

#### Validation Class: Device Path (`validate_device_path`)

**Rules:**
1. Must start with `/dev/`
2. Must match `^/dev/(pts/[0-9]+|tty[a-zA-Z0-9]*)$`
3. No `..` components, no symlink traversal outside `/dev/`
4. Must exist (stat check) and be a character device
5. Max 128 chars

| Variable | Class | Additional Rules | Needed By |
|---|---|---|---|
| `SSH_TTY` | Device Path | Must be `/dev/pts/*` | SSH session terminal |
| `GPG_TTY` | Device Path | Must be `/dev/pts/*` or `/dev/tty*` | GPG pinentry |

#### Validation Class: Hostname (`validate_hostname`)

**Rules:**
1. Max 253 chars (DNS limit)
2. Labels separated by `.`, each label max 63 chars
3. Charset per label: `[a-zA-Z0-9-]`, must not start or end with `-`
4. Single-label hostnames are valid (common on RHEL)
5. No NUL bytes, no whitespace, no shell metacharacters

| Variable | Class | Additional Rules | Needed By |
|---|---|---|---|
| `HOSTNAME` | Hostname | — | System identification, logging |

#### Validation Class: Enum Set (`validate_enum`)

For variables with a small, fixed set of acceptable values. Validation is a case-sensitive membership check.

| Variable | Accepted Values | Needed By |
|---|---|---|
| `HISTCONTROL` | `ignorespace`, `ignoredups`, `ignoreboth`, `erasedups` | Shell history |
| `XDG_SESSION_TYPE` | `x11`, `wayland`, `tty`, `mir`, `unspecified` | logind session type |
| `XDG_SESSION_CLASS` | `user`, `greeter`, `lock-screen` | logind session class |

#### Validation Class: SELinux Component (`validate_selinux_component`)

These variables are set by PAM (`pam_selinux`) at login. They contain SELinux role or level strings that UMRS can validate using `umrs-selinux`'s existing parsers.

**Rules:**
1. Max 256 chars
2. `SELINUX_ROLE_REQUESTED` — must match SELinux role syntax: `[a-zA-Z_][a-zA-Z0-9_]*_r` or empty
3. `SELINUX_LEVEL_REQUESTED` — must parse as a valid MLS level/range (delegate to `umrs-selinux` if available) or match `s[0-9]+-s[0-9]+(:[cC][0-9]+)*` pattern
4. No NUL bytes, no shell metacharacters

**Note:** These are validated structurally. Whether the role/level is _permitted_ for the user is a policy question, not a scrubbing question. The scrubber confirms the value isn't malformed or injected.

| Variable | Class | Additional Rules | Needed By |
|---|---|---|---|
| `SELINUX_ROLE_REQUESTED` | SELinux Component (role) | Empty string is valid (means "default") | PAM/login |
| `SELINUX_LEVEL_REQUESTED` | SELinux Component (level) | Empty string is valid (means "default") | PAM/login |

#### Validation Class: Safe Identifier (`validate_safe_identifier`)

For variables that contain simple identifiers — command names, function names, etc.

**Rules:**
1. Max 256 chars
2. Charset: `[a-zA-Z0-9_./-]` (alphanumeric + dot + underscore + hyphen + slash)
3. No NUL bytes, no shell metacharacters, no whitespace
4. Must not be empty

| Variable | Class | Additional Rules | Needed By |
|---|---|---|---|
| `_` | Safe Identifier | Set by bash; typically last command name | Shell introspection |

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
| `RUST_LOG` | Log Level Enum (`umrs_*` only) | Remove | Debug diagnostics |
| `COLUMNS` | Positive Integer (1–9999) | Remove | `textwrap`, terminal width |
| `LINES` | Positive Integer (1–9999) | Remove | `ratatui` terminal height |
| `TZ` | Timezone | Remove | Time formatting, timestamps |
| `XDG_RUNTIME_DIR` | Safe Path (dir, mode 0700) | Remove | systemd, D-Bus |
| `XDG_CONFIG_HOME` | Safe Path (dir) | Remove | UMRS config |
| `XDG_DATA_HOME` | Safe Path (dir) | Remove | UMRS data |
| `XDG_CACHE_HOME` | Safe Path (dir) | Remove | UMRS cache |
| `XDG_STATE_HOME` | Safe Path (dir) | Remove | UMRS state |
| `DBUS_SESSION_BUS_ADDRESS` | D-Bus Address (unix only; tcp rejected) | Remove | Desktop integration |
| `SHELL` | Safe Path (file, executable) | Remove | Login shell identification |
| `MAIL` | Safe Path (file) | Remove | Mail spool |
| `PWD` | Safe Path (dir) | Remove | Current working directory |
| `OLDPWD` | Safe Path (dir) | Remove | Previous working directory |
| `SSH_TTY` | Device Path | Remove | SSH terminal device |
| `GPG_TTY` | Device Path | Remove | GPG pinentry terminal |
| `HISTSIZE` | Positive Integer (1–100000) | Remove | Shell history |
| `SHLVL` | Positive Integer (1–999) | Remove | Shell nesting level |
| `XDG_SESSION_ID` | Positive Integer (1–999999) | Remove | logind session ID |
| `XDG_SESSION_TYPE` | Enum (`x11`/`wayland`/`tty`/`unspecified`) | Remove | Session type |
| `XDG_SESSION_CLASS` | Enum (`user`/`greeter`/`lock-screen`) | Remove | Session class |
| `XDG_DATA_DIRS` | Colon-Delimited Path List | Remove | System data dirs |
| `HOSTNAME` | Hostname | Remove | System hostname |
| `HISTCONTROL` | Enum (see below) | Remove | Shell history control |
| `SELINUX_ROLE_REQUESTED` | SELinux Component | Remove | PAM/login SELinux role |
| `SELINUX_LEVEL_REQUESTED` | SELinux Component | Remove | PAM/login MLS level |
| `SELINUX_USE_CURRENT_RANGE` | Boolean Presence | Remove | PAM/login range flag |
| `_` | Safe Identifier | Remove | Last command (set by bash) |

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
| `SSH_CLIENT` / `SSH_CONNECTION` | Leaks source IP, port — information exposure in process listing |
| `DEBUGINFOD_URLS` | External URLs — network access from air-gapped system is anomalous |
| `DEBUGINFOD_IMA_CERT_PATH` | Debug infrastructure path — not needed by UMRS tools |
| `OTEL_*` (all OpenTelemetry vars) | Telemetry export endpoints — network exfiltration vector on air-gapped systems |
| `LESSOPEN` / `LESSCLOSE` | Shell command execution via `less` preprocessor — command injection vector |
| `LS_COLORS` | Excessively long values possible; no UMRS tool uses it (we have our own color logic) |
| `GIT_EDITOR` / `EDITOR` / `VISUAL` | Editor invocation paths — UMRS tools never spawn editors; opt-in via `scrub_env_with` |
| `MOTD_SHOWN` | PAM/login marker — no value to UMRS tools |

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

## Phase 1d: Logging Initialization (`init_logging`)

**Prerequisites:** None — can start in parallel with Phase 1a.

**Scope:** `umrs-core/src/init/logging.rs` only. Journald backend + stderr fallback cascade + `LoggingBackend` return type.

**Deliverable:** `init_logging(verbose)` works standalone, returns correct `LoggingBackend` variant.

---

## — Phase 1d Detail —

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

## Phase 1e: i18n with Auto-Domain (`init_i18n`)

**Prerequisites:** None — can start in parallel with Phase 1a.

**Scope:** `umrs-core/src/init/i18n.rs` only. Wraps existing `umrs_core::i18n::init()` with binary name auto-detection.

**Deliverable:** `init_i18n(None)` auto-detects domain; `init_i18n(Some("umrs-ls"))` uses explicit domain. Warns on fallback.

---

## — Phase 1e Detail —

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

## Phase 1f: Convenience Wrapper (`init_tool`)

**Prerequisites:** Phases 1a–1e ALL complete.

**Scope:** `umrs-core/src/init/tool.rs` + `umrs-core/src/init/mod.rs` (public re-exports) + update `lib.rs`. Calls scrub → logging → i18n in order.

**Deliverable:** `init_tool(verbose)` runs the full initialization sequence. `umrs_core::init::*` is the public API surface.

---

## — Phase 1f Detail —

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

## Phase 1 Tests (distributed across sub-phases)

Each sub-phase includes its own tests. This section is the consolidated test inventory for reference.

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
| `umrs-core/src/init/validate/log_level.rs` | `validate_log_level()` — RUST_LOG enum + `umrs_*` module filter validation |
| `umrs-core/src/init/validate/device.rs` | `validate_device_path()` — `/dev/pts/*` and `/dev/tty*` validation |
| `umrs-core/src/init/validate/hostname.rs` | `validate_hostname()` — RFC 1123 hostname syntax |
| `umrs-core/src/init/validate/enum_set.rs` | `validate_enum()` — generic fixed-set membership check |
| `umrs-core/src/init/validate/selinux.rs` | `validate_selinux_component()` — SELinux role/level syntax validation |
| `umrs-core/src/init/validate/identifier.rs` | `validate_safe_identifier()` — safe alphanumeric identifier |
| `umrs-core/tests/validate_log_level_tests.rs` | RUST_LOG validation tests (accepted levels, rejected modules, comma lists) |
| `umrs-env/Cargo.toml` | New binary crate — depends on `umrs-core`, `clap`, `serde_json` |
| `umrs-env/src/main.rs` | CLI entry point — stdin/env reading, output formatting, `--debug` comments |
| `umrs-env/tests/cli_tests.rs` | Integration tests via `assert_cmd` (default, debug, json, stdin, --var, --allow modes) |
| docs | Developer guide section on tool initialization pattern + validator API reference + `umrs-env` operator guide |

---

## Phase 2: `umrs-env` — Scrubbed Environment Tool

**Prerequisites:** Phase 1f complete (uses `init_tool` and full scrub engine).

**Scope:** New binary crate `umrs-env/`. CLI parsing, stdin mode, output formatting, `--list` dictionary, `--debug` comments, `--json` mode, `--allow` extension, exit codes.

**Deliverable:** `cargo run -p umrs-env` produces scrubbed environment output. All CLI modes work. Integration tests via `assert_cmd`.

---

## — Phase 2 Detail —

**ROADMAP alignment:** G4 (Tool Ecosystem), G8 (Human-Centered Design)

### Concept

A standalone CLI tool that acts like `env(1)` but runs every variable through the `umrs_core::init` scrub engine. **It never modifies the caller's environment** — it reports what's clean, what's jacked up, and why. The operator decides what to do about it. If they want to act on the results, they can source the output or pipe it into their workflow. The tool is an advisor, not a mutator.

Designed for three use cases:

1. **Operator inspection** — "What does my environment look like after UMRS scrubbing?" Run `umrs-env` to see only what UMRS tools would trust.
2. **Shell integration** — `eval $(umrs-env)` or `umrs-env > ~/.umrs-env && source ~/.umrs-env` to launch a scrubbed subshell or source a clean environment file.
3. **Pipe scrubbing** — `env | umrs-env` or `echo "$HOME" | umrs-env --var HOME` to scrub arbitrary input through the validator pipeline.

### CLI Interface

```
umrs-env [OPTIONS]

OPTIONS:
    --debug             Show scrubbing decisions as # comments in output
    --json              Output as JSON (ScrubReport structure)
    --list              Dump the full variable dictionary: every variable we know about,
                        its tier, validation class, and what we check for
    --list <VAR>        Show details for a specific variable (tier, validator, rules)
    --allow <VAR>       Add variable(s) to the allowlist (repeatable)
    --var <NAME>        When reading stdin, treat input as the value of NAME
    --stdin             Read KEY=VALUE pairs from stdin instead of inherited env
    -v, --verbose       Verbose operation (passed to init_tool)
    -h, --help          Show help
    --version           Show version
```

### `--list` — Variable Dictionary

Dumps the complete dictionary of environment variables the scrub engine knows about. For each variable: its tier, validation class, what the validator checks for, and the NIST control mapping. This lets an operator answer: "Does umrs-env know about my variable? What does it do with it?"

```bash
$ umrs-env --list
VARIABLE              TIER   VALIDATION CLASS         ACTION    CONTROLS
─────────────────────────────────────────────────────────────────────────
BASH_ENV              3      (denylist)               strip     CM-7
CDPATH                3      (denylist)               strip     CM-7
COLORTERM             2      Enum (truecolor|24bit)   validate  SI-10
COLUMNS               2      Positive Integer 1-9999  validate  SI-10
DBUS_SESSION_BUS_ADDRESS  2  D-Bus Address (unix)     validate  CM-7, AC-3
GLIBC_TUNABLES        3      (denylist)               strip     CM-7, SI-7
GPG_TTY               2      Device Path (/dev/*)     validate  SI-10
HISTCONTROL           2      Enum (4 values)          validate  SI-10
HISTSIZE              2      Positive Integer 1-100k  validate  SI-10
HOME                  2      Safe Path (dir, owned)   validate  CM-7, AC-3, SI-7
HOSTNAME              2      Hostname (RFC 1123)      validate  SI-10
LANG                  2      POSIX Locale             validate  SI-10
...
LD_PRELOAD            3      (denylist)               strip     CM-7, SI-7
...
PATH                  1      (reset)                  reset     CM-7
...
RUST_LOG              2      Log Level Enum (umrs_*)  validate  AU-9, CM-7
SELINUX_LEVEL_REQUESTED  2   SELinux Component        validate  AC-4, SI-10
SELINUX_ROLE_REQUESTED   2   SELinux Component        validate  AC-4, SI-10
...
TMPDIR                1      (reset)                  reset     CM-7

Tier 1: 2 variables (reset to safe defaults)
Tier 2: 33 variables (preserve + validate)
Tier 3: 35+ variables (strip unconditionally)
Unknown variables not listed above are removed silently.
```

**Single-variable detail** (`--list HOME`):

```bash
$ umrs-env --list HOME
Variable:         HOME
Tier:             2 (preserve + validate)
Validation class: Safe Path (directory)
Validator:        validate_safe_path()
Action on fail:   Remove from sanitized environment

Checks performed:
  ✓ Must be an absolute path (starts with /)
  ✓ No NUL bytes
  ✓ No '..' path traversal components
  ✓ No shell metacharacters (; | & ` $ ( ) { } < >)
  ✓ Path must resolve to an existing directory
  ✓ Directory must not be world-writable (o+w)
  ✓ Must be owned by root or current user
  ✓ No sticky-bit-less world-writable parent directories
  ✓ Max length: 4096 (PATH_MAX)
  ✓ Symlink chain: every component checked via O_PATH|O_NOFOLLOW

Controls: NIST SP 800-53 CM-7, AC-3, SI-7
          CWE-426 (untrusted search path)

Current value: /home/jadams
Status:        ✓ PASS
```

```bash
$ umrs-env --list SELINUX_LEVEL_REQUESTED
Variable:         SELINUX_LEVEL_REQUESTED
Tier:             2 (preserve + validate)
Validation class: SELinux Component (MLS level)
Validator:        validate_selinux_component()
Action on fail:   Remove from sanitized environment

Checks performed:
  ✓ Max 256 chars
  ✓ Must parse as valid MLS level or range (s0-s15, optional categories)
  ✓ Empty string accepted (means "use default")
  ✓ No NUL bytes or shell metacharacters
  ✓ Structural validation only — policy permit check is out of scope

Controls: NIST SP 800-53 AC-4 (information flow), SI-10
          Uses umrs-selinux parsing engine for validation

Current value: s0-s0:c0.c1023
Status:        ✓ PASS
```

```bash
$ umrs-env --list EDITOR
Variable:         EDITOR
Tier:             3 (strip — not in allowlist)
Action:           Removed from sanitized environment
Reason:           UMRS tools never spawn editors; opt-in via --allow EDITOR

To include: umrs-env --allow EDITOR
```

**JSON variant** (`--list --json`): dumps the full dictionary as a JSON array for programmatic consumption.

### Output Formats

**Default output** — sourceable shell `KEY=VALUE` pairs, one per line. Only variables that survived scrubbing appear. Values are shell-quoted (single quotes with escaping).

```bash
$ umrs-env
HOME='/home/jadams'
LANG='en_US.UTF-8'
TERM='xterm-256color'
PATH='/usr/bin:/bin:/usr/sbin:/sbin'
TMPDIR='/tmp'
NO_COLOR=''
XDG_RUNTIME_DIR='/run/user/1000'
```

**Debug output** (`--debug`) — same as default, but scrubbing decisions appear as `#` comment lines. Rejected/stripped/reset variables are shown with the reason. This makes the output self-documenting while remaining sourceable (comments are ignored by the shell).

```bash
$ umrs-env --debug
# umrs-env: environment scrub report
# scrub engine: umrs_core::init v0.1.0
# timestamp: 2026-03-17T14:22:03Z

# --- Tier 1: Reset to safe defaults ---
# PATH: reset from inherited value (CM-7)
PATH='/usr/bin:/bin:/usr/sbin:/sbin'
# TMPDIR: reset from inherited value (CM-7)
TMPDIR='/tmp'

# --- Tier 2: Preserved (validated) ---
HOME='/home/jadams'
LANG='en_US.UTF-8'
TERM='xterm-256color'
NO_COLOR=''
XDG_RUNTIME_DIR='/run/user/1000'

# --- Tier 3: Stripped (denied) ---
# LD_PRELOAD: stripped — library injection vector (CM-7, SI-7)
# GLIBC_TUNABLES: stripped — CVE-2023-4911 (CM-7, SI-7)

# --- Failed validation ---
# DBUS_SESSION_BUS_ADDRESS: rejected — tcp: transport not permitted (CM-7)

# --- Unknown (not in any allowlist) ---
# EDITOR: removed — not in allowlist
# VISUAL: removed — not in allowlist

# summary: 5 preserved, 2 reset, 2 stripped, 1 rejected, 2 unknown removed
```

**JSON output** (`--json`) — serializes the full `ScrubReport` structure. For programmatic consumption, piping into `jq`, or feeding into `umrs-logspace` audit records.

### Stdin Mode

When `--stdin` is given (or stdin is not a TTY), reads `KEY=VALUE` pairs from stdin (one per line, same format as `env(1)` output). Each pair is run through the scrub engine and the result is output per the selected format.

**Single-variable mode** (`--var NAME`) — reads a bare value from stdin (no `KEY=`) and validates it as the named variable. Useful for quick checks:

```bash
$ echo "/home/jadams" | umrs-env --var HOME
HOME='/home/jadams'

$ echo "/home/../etc/shadow" | umrs-env --var HOME --debug
# HOME: rejected — path contains '..' traversal component (SI-7)

$ echo "trace,hyper=debug" | umrs-env --var RUST_LOG --debug
# RUST_LOG: rejected — module filter 'hyper' does not match umrs_* prefix (AU-9)
```

**Pipe full environment:**

```bash
$ env | umrs-env --debug
# (full scrub report with comments)

$ env | umrs-env --json | jq '.stripped[].name'
"LD_PRELOAD"
"GLIBC_TUNABLES"
```

### Custom Allowlist

`--allow` extends the default Tier 2 allowlist. Variables added via `--allow` fall into three categories:

1. **Known variable, no dedicated validator.** The variable is in our dictionary (Tier 3 strip-by-default like `EDITOR`, `SSH_AUTH_SOCK`) but not on the hardcoded denylist. It gets promoted to Tier 2 and validated with a generic safety check (no NUL, no shell metacharacters, max 4096 chars). No warning.

2. **Unknown variable — not in our dictionary at all.** The variable is accepted but the tool emits a warning: we have no dedicated validator for it, so only the generic safety check applies. The operator should know they're getting reduced assurance.

   ```bash
   $ umrs-env --allow MY_CUSTOM_VAR
   # WARNING: MY_CUSTOM_VAR is not in the umrs-env variable dictionary.
   #          No dedicated validator exists — only generic safety checks applied.
   #          Use --list MY_CUSTOM_VAR for details.
   HOME='/home/jadams'
   MY_CUSTOM_VAR='some_value'
   ...
   ```

   In `--json` mode, these appear in a `warnings` array in the output with `"kind": "unvalidated_allow"`.

3. **Hardcoded denylist variable.** `LD_PRELOAD`, `GLIBC_TUNABLES`, etc. **cannot** be allowed via `--allow` — they are always stripped regardless. The tool emits a hard warning explaining why.

   ```bash
   $ umrs-env --allow LD_PRELOAD
   # ERROR: LD_PRELOAD is on the hardcoded denylist and cannot be allowed.
   #        Reason: library injection vector (NIST SP 800-53 CM-7, SI-7)
   #        This restriction cannot be overridden.
   ```

```bash
$ umrs-env --allow EDITOR --allow VISUAL
HOME='/home/jadams'
EDITOR='/usr/bin/vim'
VISUAL='/usr/bin/vim'
...
```

### Exit Codes

| Code | Meaning |
|---|---|
| 0 | All variables processed; output produced |
| 1 | At least one variable was stripped from the denylist (anomaly detected) |
| 2 | Fatal error (can't read stdin, bad CLI args) |

Exit code 1 on denylist hits enables use in shell scripts: `umrs-env || echo "WARNING: environment anomaly detected"`.

### Crate Structure

New binary crate: `components/rusty-gadgets/umrs-env/`

```
umrs-env/
├── Cargo.toml          ← depends on umrs-core (init module)
├── src/
│   └── main.rs         ← CLI parsing (clap), stdin handling, output formatting
└── tests/
    └── cli_tests.rs    ← integration tests via assert_cmd
```

**Dependencies:** `umrs-core` (workspace), `clap` (CLI parsing), `serde_json` (for `--json`), `assert_cmd` + `predicates` (test only).

### Controls

- NIST SP 800-53 CM-7 (least functionality — only validated vars survive)
- NIST SP 800-53 SI-10 (input validation — every value is validated)
- NIST SP 800-53 AU-3 (audit content — `--json` provides structured audit record)
- CWE-526 (exposure of sensitive information through environment variables)
- CERT ENV03-C (sanitize the environment when invoking external programs)

---

## Future Considerations

- **`SanitizedEnv` as the tool's canonical environment** — long-term, tools should never call `std::env::var()` directly. All env access goes through `SanitizedEnv.get()`. This is enforceable via clippy custom lints or code review.
- **`detect_secure_execution()`** — check if the process is running in glibc secure-execution mode (AT_SECURE flag, set when real/effective UID/GID differ — i.e., setuid/setgid/capability-elevated). When detected, the scrub report should flag this at `warn` level. Could read `/proc/self/auxv` for `AT_SECURE` or compare real vs effective UID/GID via `rustix::process`. This enables tools to automatically escalate their logging posture when running privileged.
- **Posture integration** — the `ScrubReport` stripped/failed entries are security findings. They could feed into the `umrs-platform` posture assessment as `SecurityObservation` variants (per the Security Findings as Data rule). An environment with `LD_PRELOAD` set is a posture signal, not just a log line.
- **Audit integration** — log scrub report to `umrs-logspace` audit trail (not just debug log).
- **Standalone crate potential** — The validator set (`validate_safe_path`, `validate_lang`, `validate_path_list`, `validate_term`, `validate_username`, `validate_tz`) plus `scrub_env` is independently valuable to the Rust ecosystem. Environment scrubbing is a long-standing pain point — there is no good Rust crate for it. Consider publishing as a standalone crate (e.g., `env-scrub` or `secure-env`) after the API stabilizes in UMRS.
- **Configurable strictness levels** — Some deployments may want stricter validation (e.g., reject unknown TERM values entirely). A builder pattern or strictness enum could support this without breaking the simple API.
- **`/proc/<pid>/environ` watcher** — For long-running daemons, periodically verify that the process environment hasn't been modified externally (defense in depth, not a replacement for scrubbing).

---

## Phase 3: Env Var Corpus Research (Researcher Task)

**Prerequisites:** None — can run in parallel with all other phases.

**Executor:** researcher agent (not rust-developer)

**Scope:** Acquire authoritative env var source material for the Tier 1/2 allowlists. Research only — no code.

---

## — Phase 3 Detail —

Source: `.claude/jamies_brain/more_env_stuff.txt` (Jamie Adams research)

### Additions to incorporate at implementation time

**1. systemd runtime vars → add to Tier 2 allowlist:**
- `INVOCATION_ID` — systemd invocation UUID (validate: 128-bit hex)
- `JOURNAL_STREAM` — journald socket (validate: device:inode format)
- `NOTIFY_SOCKET` — sd_notify path (validate: safe path)
- `SYSTEMD_EXEC_PID` — PID of the current unit (validate: positive integer)

Rationale: UMRS tools use journald natively. These vars are present in every systemd-managed service and should be recognized, not flagged as unknown.

**2. Container detection vars → informational signals in `ScrubReport`:**
- `KUBERNETES_SERVICE_HOST`, `KUBERNETES_PORT` — Kubernetes pod indicator
- Docker-injected `HOSTNAME` override patterns
- Container engine indicators (`container`, `PODMAN_*`)

Rationale: If these appear, the tool knows it's running in a container. This is a posture signal — containerized UMRS tools may have different trust assumptions. Report as informational, not stripped.

**3. Future research — authoritative env var corpus:**

The env var allowlist should be built from authoritative sources, not hand-curated from memory.
Sources are prioritized by signal quality:

*High-yield (automated extraction possible):*

| Source | What it provides | Extraction method |
|---|---|---|
| POSIX / Open Group Base Specs Issue 7 | Ground truth: `PATH`, `HOME`, `IFS`, `LANG`, `LC_*`, `TZ` — small set, high assurance | Manual (spec text) |
| Linux man-pages (`man 7 environ`) | Cross-cutting aggregation of libc, tools, kernel interfaces | Parse `ENVIRONMENT` / `ENVIRONMENT VARIABLES` sections from all man pages |
| glibc / dynamic linker (`man ld.so`, `elf/rtld.c`) | Security-critical: `LD_PRELOAD`, `LD_LIBRARY_PATH`, `LD_AUDIT`, `LD_DEBUG`, `MALLOC_*` | Source extraction |
| systemd (`man systemd.exec`, `src/basic/env-util.c`) | `INVOCATION_ID`, `JOURNAL_STREAM`, `NOTIFY_SOCKET`, `SYSTEMD_EXEC_PID` — directly relevant to journald work | Man page + source |
| XDG Base Directory Spec (freedesktop.org) | Clean, spec-driven: `XDG_CONFIG_HOME`, `XDG_DATA_HOME`, `XDG_RUNTIME_DIR`, `XDG_CACHE_HOME` | Spec text |

*Medium-yield (language runtimes, compilers):*

| Source | What it provides |
|---|---|
| Cargo Book — Environment Variables | `CARGO_HOME`, `RUSTFLAGS`, `RUST_LOG`, `OUT_DIR`, `TARGET` — directly relevant |
| Python docs — Environment Variables | `PYTHONPATH`, `PYTHONHOME`, `PYTHONHASHSEED` — clean `PYTHON*` namespace |
| Go (`go help environment`) | `GOPATH`, `GOROOT`, `GOOS`, `GOARCH` |
| Node.js / npm docs | `NODE_ENV`, `NODE_PATH`, `NPM_CONFIG_*` |
| GCC / Clang docs | `CPATH`, `LIBRARY_PATH`, `C_INCLUDE_PATH`, `LD_RUN_PATH` |

*Low-yield but valuable for posture signals:*

| Source | What it provides |
|---|---|
| sudo (`man sudo`, `man sudoers`) | `SUDO_USER`, `SUDO_UID`, `SUDO_COMMAND` |
| PAM module docs | Session-injected vars |
| Docker / Kubernetes | Container detection signals, downward API vars |

*Empirical validation:*

`/proc/self/environ` sampling across login shells, systemd services, containers, and build
systems provides **spec-vs-reality reconciliation** — discover real-world variables, then
backfill with authoritative sources. This is very UMRS-aligned.

Starting with just POSIX + Linux man-pages + glibc + systemd + Cargo/Python gives ~150–300
high-quality variables with real semantics.

**Pre-implementation researcher task:** Acquire and index these sources before Rusty begins implementation. This gives Rusty an authoritative reference for the Tier 1/2 allowlists.

---

## Appendix: String Sanitization Doctrine (Future Scope)

The environment scrubbing in this plan is one surface of a broader sanitization architecture
that UMRS will need as vault/custody features develop. This appendix captures the doctrine
for future reference. It does NOT change the scope of this plan.

### Governing standards

| Standard | Relevant controls |
|---|---|
| NIST SP 800-53 SI-10 | Information Input Validation — type, length, format, range |
| NIST SP 800-53 SI-7 | Software, Firmware, Information Integrity |
| NIST SP 800-53 SI-11 | Error Handling |
| NIST SP 800-53 AU-3 | Audit record integrity (log scrubbing) |
| NIST SP 800-171 3.1.20 | Validate input |
| NIST SP 800-95 | Normalize → validate → store (canonical order) |
| CERT STR/IDS rules | Bounded string ops, reject on first failure, allow-list only |
| OWASP Input Validation | Allow-list > deny-list, canonicalize paths, escape output not input |
| CWE-20 | Improper Input Validation |
| CWE-22 | Path Traversal |
| CWE-73 | External Control of File Name |
| CWE-116 | Improper Output Encoding |
| CWE-176 | Unicode Encoding Issues |
| CWE-180 | Incorrect Canonicalization |

### High-assurance sanitization principles

1. **Sanitize for context, not generically.** No universal "sanitize string" function —
   the consumption surface (filesystem, shell, log, SQL, label) determines the rules.
2. **Prefer structured types over strings.** `Path`/`PathBuf` over `String` for paths;
   typed structs for labels; `.arg()` APIs for command args.
3. **Normalize → validate → store.** Order matters: Unicode normalize, canonicalize path,
   apply allow-list, enforce length, resolve collisions.
4. **Treat filenames as untrusted metadata.** Never authoritative identifiers. Generate
   internal object IDs; preserve originals as metadata only.
5. **Allow-list, never deny-list.** Accept known-good input only.
6. **Strip control characters.** Newline, tab, null, non-printables — prevents log and UI injection.
7. **Separate ingestion naming from archive naming.** User name ≠ archive name.
   Prevents spoofing and collision abuse.
8. **Never shell out with string interpolation.** Use `Command::new().arg()` — Rust does
   not invoke a shell, eliminating injection risk.

### Sanitization stack for vault-grade Rust tooling

```
1. UTF-8 validation          (automatic in String)
2. Null byte rejection
3. Unicode normalization      (NFC via unicode-normalization crate)
4. Character allow-listing
5. Path canonicalization
6. Base path containment check
7. Collision resolution
8. Structured metadata parsing
```

### Logging sanitization (from NIST AU controls)

- Escape CR/LF in user-supplied strings before logging
- Prevent log forging (injected fake audit entries)
- Preserve original string as evidence; store sanitized + raw forms separately if needed
- Structured logging (JSON) mitigates most injection vectors

### Relevant crates

| Crate | Purpose |
|---|---|
| `unicode-normalization` | NFC/NFD handling |
| `regex` | Structured validation / allow-listing |
| `camino` | UTF-8 safe paths |

### Source material

Derived from Jamie's research notes:
- `jamies_brain/sanitization.txt` (archived 2026-03-19)
- `jamies_brain/scrub-strings.txt` (archived 2026-03-19)
- `jamies_brain/more_env_stuff.txt` (archived 2026-03-19)
