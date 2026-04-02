// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//! modprobe.d merge-tree reader and live `/sys/module/` cross-check.
//!
//! Reads kernel module parameter configuration from the modprobe.d merge tree
//! and cross-checks live values from `/sys/module/<mod>/parameters/<param>`.
//!
//! ## Scope
//!
//! Phase 2a covers `options` and `blacklist` directives — both have
//! well-defined, deterministic formats.
//!
//! Phase 2b adds `install` directive parsing. An `install <module> /bin/true`
//! (or `/bin/false`, `/usr/bin/true`, `/usr/bin/false`) directive is detected
//! as a hard blacklist and recorded in `hard_blacklisted`. `softdep`, `alias`,
//! and `remove` directives are logged at debug and excluded from the merged
//! configuration.
//!
//! ## Trust Boundary
//!
//! `/etc/modprobe.d/` files are **regular files** on the root filesystem.
//! They do NOT require `SecureReader` or `fstatfs` provenance verification —
//! they represent the *intended* configuration, not the *effective* kernel
//! state. The live value (from `/sys/module/`) is always authoritative.
//!
//! `/sys/module/<mod>/parameters/<param>` reads go through `SysfsText` with
//! `SYSFS_MAGIC` provenance verification. The Trust Gate Rule applies: only
//! read `/sys/module/<mod>/parameters/` if the module directory exists.
//!
//! ## Merge-Tree Precedence
//!
//! Follows `modprobe.d(5)` last-writer-wins, lexicographic within directory:
//!
//! 1. `/usr/lib/modprobe.d/*.conf` — distro/vendor defaults (lowest)
//! 2. `/run/modprobe.d/*.conf` — transient overrides
//! 3. `/etc/modprobe.d/*.conf` — admin overrides (highest)
//!
//! ## Applicable Patterns
//!
//! - **Compile-Time Path Binding** (NSA RTB RAIN): sysfs parameter paths are
//!   assembled from module/param names under `/sys/module/` and verified via
//!   `SYSFS_MAGIC`. No runtime path construction from user input.
//! - **Provenance Verification** (NIST SP 800-53 SI-7): `/sys/module/` reads go
//!   through `SecureReader` with `fstatfs` against `SYSFS_MAGIC`.
//! - **Trust Gate** (NIST SP 800-53 CM-6): only read `/sys/module/<mod>/parameters/`
//!   if `/sys/module/<mod>/` exists.
//! - **Fail-Closed Parsing** (NIST SP 800-53 SI-10): lines that fail to parse are
//!   rejected and logged; unrecognised directives are logged at debug and excluded.
//! - **Layered Separation** (NSA RTB / NIST SP 800-53 SC-3): this module collects
//!   data only; no formatting, display, or remediation logic.
//! - **Pattern Execution Measurement** (NIST SP 800-218 SSDF PW.4): timing logged
//!   at debug under `#[cfg(debug_assertions)]`.
//! - **Must-Use Contract** (NIST SP 800-53 SI-10, SA-11): all public functions
//!   returning `Result` or `Option` carry `#[must_use]`.
//! - **Security Findings as Data** (NIST SP 800-53 AU-3): contradictions are
//!   `ContradictionKind` enum variants, not log strings.
//!
//! ## Compliance
//!
//! NIST SP 800-53 CM-6: Configuration Settings — modprobe.d is the persistence
//! layer for module parameter and blacklist state.
//! NIST SP 800-53 CA-7: Continuous Monitoring — cross-check detects live vs.
//! configured divergence that escapes single-source monitoring.
//! NIST SP 800-53 AU-3: structured indicator reports enable machine-readable audit.
//! NIST SP 800-53 SI-7: provenance-verified `/sys/module/` reads.

use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};

use crate::posture::indicator::ConfiguredValue;

// ===========================================================================
// Constants
// ===========================================================================

/// modprobe.d directories in ascending precedence order.
///
/// Follows `modprobe.d(5)`: last writer wins, lexicographic within directory.
const MODPROBE_SEARCH_DIRS: &[&str] =
    &["/usr/lib/modprobe.d", "/run/modprobe.d", "/etc/modprobe.d"];

/// Sysfs base path for loaded kernel modules.
const SYS_MODULE_BASE: &str = "/sys/module";

// ===========================================================================
// ParsedDirective — internal line-parse result
// ===========================================================================

/// Result of parsing one modprobe.d line.
///
/// Phase 2b adds `Install` for `install <module> <command>` directives.
/// An `install` directive with `/bin/true` or `/bin/false` as the command
/// is a **hard blacklist** — it prevents module loading even when explicitly
/// requested via `modprobe`. The probe detects this pattern and records it
/// as a stronger form of blacklist evidence.
///
/// `Unrecognised` is produced for known-unhandled directives (`softdep`,
/// `alias`, `override`) — they are logged at debug and silently excluded
/// from the merged configuration. This is the fail-closed behaviour for
/// unrecognised but non-malformed input.
///
/// Exposed as `pub` to enable integration testing of the parser without
/// filesystem interaction.
///
/// NIST SP 800-53 CM-7: Least Functionality — `install <mod> /bin/true`
/// is the sysadmin-standard mechanism for enforcing module load prevention.
/// Detecting it closes the gap where a soft `blacklist` entry could be
/// bypassed by an explicit `modprobe` call.
#[derive(Debug, PartialEq, Eq)]
pub enum ParsedDirective<'a> {
    /// `options <module> <param>=<value> [...]`
    Options {
        module: &'a str,
        params: Vec<(&'a str, &'a str)>,
    },
    /// `blacklist <module>` — soft blacklist. Prevents automatic loading but
    /// can be bypassed by an explicit `modprobe` call.
    Blacklist {
        module: &'a str,
    },
    /// `install <module> <command>` — may be a hard blacklist.
    ///
    /// When `command` is `/bin/true`, `/bin/false`, or `/usr/bin/true`,
    /// this is a hard blacklist: `modprobe <module>` silently succeeds but
    /// the kernel's load request executes the command instead of loading the
    /// module. The `is_hard_blacklist` field is `true` for these patterns.
    ///
    /// Other commands (e.g., `modprobe --ignore-install <module>`) are
    /// complex redirections not equivalent to a blacklist. These are recorded
    /// with `is_hard_blacklist: false` and logged for operator awareness.
    Install {
        module: &'a str,
        command: &'a str,
        /// `true` if `command` is a recognised hard-blacklist sentinel
        /// (`/bin/true`, `/bin/false`, or `/usr/bin/true`).
        is_hard_blacklist: bool,
    },
    /// Comment or blank line — safely ignored.
    Comment,
    /// Recognised directive not handled in Phase 2b (`softdep`, `alias`,
    /// `override`).
    Unrecognised {
        keyword: &'a str,
    },
    /// Line is non-empty but does not match any known format.
    Malformed,
}

// ===========================================================================
// ModprobeConfig — merged modprobe.d state
// ===========================================================================

/// Merged modprobe.d configuration.
///
/// Built by `ModprobeConfig::load()`, which reads all modprobe.d sources in
/// precedence order and produces final option and blacklist maps.
///
/// Two blacklist maps are maintained:
/// - `blacklisted` — soft blacklists from `blacklist <module>` directives.
///   Can be bypassed by an explicit `modprobe` invocation.
/// - `hard_blacklisted` — hard blacklists from `install <module> /bin/true`
///   (or `/bin/false`) directives. These redirect the kernel's load request
///   to a no-op command, preventing loading even with explicit `modprobe`.
///
/// NIST SP 800-53 CM-6: provides the configured (persistence-layer) baseline for
/// module parameter and blacklist contradiction detection.
/// NIST SP 800-53 CM-7: Least Functionality — hard blacklist evidence distinguishes
/// between bypass-resistant and bypass-susceptible module load prevention.
/// NSA RTB RAIN: `ModprobeConfig` is constructed via a validated builder;
/// callers receive a complete, validated value — not a partial `Result`.
#[must_use = "modprobe config carries module parameter findings — do not discard"]
pub struct ModprobeConfig {
    /// module_name → { param_name → (value, source_file) }
    options: HashMap<String, HashMap<String, (String, String)>>,
    /// module_name → source_file (soft-blacklisted modules via `blacklist <mod>`)
    blacklisted: HashMap<String, String>,
    /// module_name → source_file (hard-blacklisted modules via
    /// `install <mod> /bin/true` or `install <mod> /bin/false`)
    hard_blacklisted: HashMap<String, String>,
}

impl ModprobeConfig {
    /// Load and merge all modprobe.d configuration sources.
    ///
    /// Reads `/usr/lib/modprobe.d/`, `/run/modprobe.d/`, and
    /// `/etc/modprobe.d/` in ascending precedence order. Missing directories
    /// and unreadable files are silently skipped (logged at debug).
    ///
    /// NIST SP 800-53 CM-6: collects the full configured baseline from all
    /// modprobe.d persistence layers.
    #[must_use = "modprobe config must be examined for configured values"]
    pub fn load() -> Self {
        #[cfg(debug_assertions)]
        let start = std::time::Instant::now();

        let mut options: HashMap<String, HashMap<String, (String, String)>> = HashMap::new();
        let mut blacklisted: HashMap<String, String> = HashMap::new();
        let mut hard_blacklisted: HashMap<String, String> = HashMap::new();
        let mut file_count: usize = 0;

        for dir in MODPROBE_SEARCH_DIRS {
            let dir_path = Path::new(dir);
            // The `exists()` pre-check is omitted intentionally. `load_conf_dir`
            // calls `read_dir` and handles `NotFound` in its error arm, collapsing
            // the TOCTOU window from a two-step check-then-open to a single
            // operation. NIST SP 800-53 SI-10.
            log::debug!("posture: modprobe.d merge: scanning {dir} ...");
            let loaded = load_conf_dir(
                dir_path,
                &mut options,
                &mut blacklisted,
                &mut hard_blacklisted,
            );
            file_count = file_count.saturating_add(loaded);
        }

        #[cfg(debug_assertions)]
        {
            let module_count = options.len();
            let blacklist_count = blacklisted.len();
            let hard_count = hard_blacklisted.len();
            log::debug!(
                "posture: modprobe.d merge completed in {} µs: {} files, {} modules, \
                 {} soft-blacklisted, {} hard-blacklisted",
                start.elapsed().as_micros(),
                file_count,
                module_count,
                blacklist_count,
                hard_count
            );
        }

        #[cfg(not(debug_assertions))]
        let _ = file_count;

        Self {
            options,
            blacklisted,
            hard_blacklisted,
        }
    }

    /// Look up the configured value for a module parameter.
    ///
    /// Returns `Some(ConfiguredValue)` if a matching `options <module>
    /// <param>=<value>` was found in any modprobe.d file, or `None` if absent.
    ///
    /// NIST SP 800-53 CM-6: returns the last-writer-wins effective configured value.
    #[must_use = "module parameter configured value result must be examined"]
    pub fn get_option(&self, module: &str, param: &str) -> Option<ConfiguredValue> {
        self.options.get(module)?.get(param).map(|(value, source)| ConfiguredValue {
            raw: value.clone(),
            source_file: source.clone(),
        })
    }

    /// Check whether a module is blacklisted (soft or hard).
    ///
    /// Returns `Some(true)` if a `blacklist <module>` **or** an
    /// `install <module> /bin/true` entry was found in any modprobe.d source.
    /// Returns `None` if the module is not present in any blacklist map
    /// (absence means "not configured in modprobe.d", not "explicitly allowed").
    ///
    /// Use `is_hard_blacklisted()` to distinguish bypass-resistant hard
    /// blacklists (`install <mod> /bin/true`) from bypass-susceptible soft
    /// blacklists (`blacklist <mod>`).
    ///
    /// NIST SP 800-53 CM-6: configured blacklist state for contradiction detection.
    /// NIST SP 800-53 CM-7: Least Functionality — both forms prevent module
    /// loading in the intended configuration; both are security-relevant findings.
    #[must_use = "blacklist check result must be examined — None means absent from config, not allowed"]
    pub fn is_blacklisted(&self, module: &str) -> Option<bool> {
        if self.blacklisted.contains_key(module) || self.hard_blacklisted.contains_key(module) {
            Some(true)
        } else {
            None
        }
    }

    /// Check whether a module is hard-blacklisted via `install <mod> /bin/true`.
    ///
    /// Returns `Some(true)` only if an `install <module> /bin/true` (or
    /// `/bin/false`) directive was found. Returns `None` if absent from the
    /// hard-blacklist map (which may still be soft-blacklisted).
    ///
    /// A hard blacklist is bypass-resistant: even an explicit `modprobe <module>`
    /// invocation will silently succeed without loading the module. Soft
    /// blacklists (`blacklist <mod>`) can be bypassed by explicit modprobe.
    ///
    /// NIST SP 800-53 CM-7: hard blacklist provides stronger load prevention
    /// than soft blacklist; detecting it provides higher-confidence evidence.
    #[must_use = "hard blacklist check result must be examined — None means not hard-blacklisted"]
    pub fn is_hard_blacklisted(&self, module: &str) -> Option<bool> {
        if self.hard_blacklisted.contains_key(module) {
            Some(true)
        } else {
            None
        }
    }

    /// Return the source file that established the blacklist entry for `module`.
    ///
    /// Prefers the hard-blacklist source file when both are present, because
    /// the `install` directive provides stronger evidence. Returns `None` if
    /// the module is not present in either blacklist map.
    #[must_use = "blacklist source file result must be examined"]
    pub fn blacklist_source(&self, module: &str) -> Option<&str> {
        // Prefer hard blacklist evidence (stronger guarantee).
        self.hard_blacklisted
            .get(module)
            .or_else(|| self.blacklisted.get(module))
            .map(String::as_str)
    }
}

// ===========================================================================
// Directory and file loading helpers
// ===========================================================================

/// Load all `.conf` files from a directory in lexicographic order.
///
/// Returns the number of files successfully opened (not necessarily parsed
/// without issues — partial parses count). Unreadable files are skipped and
/// logged at debug.
fn load_conf_dir(
    dir: &Path,
    options: &mut HashMap<String, HashMap<String, (String, String)>>,
    blacklisted: &mut HashMap<String, String>,
    hard_blacklisted: &mut HashMap<String, String>,
) -> usize {
    let mut files: Vec<PathBuf> = match std::fs::read_dir(dir) {
        Ok(entries) => entries
            .filter_map(|e| e.ok().map(|e| e.path()))
            .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("conf") && p.is_file())
            .collect(),
        Err(e) => {
            log::debug!("posture: cannot read modprobe.d dir {}: {e}", dir.display());
            return 0;
        }
    };

    // Lexicographic sort — matches modprobe.d(5) processing order.
    files.sort();

    let mut count = 0usize;
    for path in &files {
        match load_conf_file(path, options, blacklisted, hard_blacklisted) {
            Ok(()) => count = count.saturating_add(1),
            Err(e) => {
                log::debug!("posture: modprobe.d: skipping {}: {e}", path.display());
            }
        }
    }
    count
}

/// Parse one modprobe.d `.conf` file, inserting directives into the maps.
///
/// `options`, `blacklist`, and `install` (hard blacklist) directives are
/// applied with last-writer-wins semantics (later files in the precedence
/// order overwrite earlier ones).
///
/// NIST SP 800-53 SI-10: Input Validation — malformed lines are logged and
/// skipped rather than silently ignored or causing a parse error.
fn load_conf_file(
    path: &Path,
    options: &mut HashMap<String, HashMap<String, (String, String)>>,
    blacklisted: &mut HashMap<String, String>,
    hard_blacklisted: &mut HashMap<String, String>,
) -> io::Result<()> {
    let content = std::fs::read_to_string(path)?;
    let source = path.to_string_lossy().into_owned();

    for (line_no, raw_line) in content.lines().enumerate() {
        let human_no = line_no.saturating_add(1);
        match parse_modprobe_line(raw_line) {
            ParsedDirective::Options {
                module,
                params,
            } => {
                for (param, value) in params {
                    // Error Information Discipline (NIST SP 800-53 SI-11, SC-28):
                    // Log the parameter name but not the value. Module parameter
                    // values in production modprobe.d files may reflect security
                    // policy choices (e.g., crypto driver tuning, DMA settings)
                    // that should not be broadcast in debug logs on CUI/DoD systems.
                    // This matches the discipline applied in configured.rs for
                    // sysctl.d logging.
                    log::debug!(
                        "posture: modprobe.d merge: {source}:{human_no} options \
                         {module} {param}=<value>"
                    );
                    options
                        .entry(module.to_owned())
                        .or_default()
                        .insert(param.to_owned(), (value.to_owned(), source.clone()));
                }
            }
            ParsedDirective::Blacklist {
                module,
            } => {
                log::debug!("posture: modprobe.d merge: {source}:{human_no} blacklist {module}");
                blacklisted.insert(module.to_owned(), source.clone());
            }
            ParsedDirective::Install {
                module,
                command,
                is_hard_blacklist,
            } => {
                if is_hard_blacklist {
                    log::debug!(
                        "posture: modprobe.d merge: {source}:{human_no} \
                         install {module} (hard blacklist via '{command}')"
                    );
                    hard_blacklisted.insert(module.to_owned(), source.clone());
                } else {
                    log::debug!(
                        "posture: modprobe.d merge: {source}:{human_no} \
                         install {module} (complex command — not a hard blacklist)"
                    );
                }
            }
            ParsedDirective::Comment => {}
            ParsedDirective::Unrecognised {
                keyword,
            } => {
                log::debug!(
                    "posture: modprobe.d merge: {source}:{human_no} unrecognised \
                     directive '{keyword}' — skipped"
                );
            }
            ParsedDirective::Malformed => {
                log::debug!(
                    "posture: modprobe.d merge: {source}:{human_no} malformed line \
                     — skipped"
                );
            }
        }
    }
    Ok(())
}

// ===========================================================================
// parse_modprobe_line — public for integration tests
// ===========================================================================

/// Parse a single modprobe.d config line.
///
/// Recognises:
/// - Blank lines and comments (`#`)
/// - `options <module> <param>=<value> [...]`
/// - `blacklist <module>`
/// - `install <module> <command>` — Phase 2b. Hard blacklists are detected
///   when `command` is `/bin/true`, `/bin/false`, or `/usr/bin/true`.
/// - Unrecognised keywords (`softdep`, `alias`, `override`) —
///   returned as `Unrecognised` for caller to log at debug.
/// - Anything else → `Malformed`.
///
/// Exposed as `pub` to enable integration testing of the parser without
/// filesystem interaction.
///
/// NIST SP 800-53 SI-10: Input Validation — fails closed on unrecognised content.
/// NIST SP 800-53 CM-7: `install <mod> /bin/true` detection enables hard-blacklist
/// evidence collection.
#[must_use = "modprobe.d line parse result must be examined — Malformed means the line is invalid"]
pub fn parse_modprobe_line(line: &str) -> ParsedDirective<'_> {
    let trimmed = line.trim();
    if trimmed.is_empty() || trimmed.starts_with('#') {
        return ParsedDirective::Comment;
    }

    let mut parts = trimmed.splitn(2, char::is_whitespace);
    let Some(keyword) = parts.next() else {
        return ParsedDirective::Comment;
    };
    let rest = parts.next().unwrap_or("").trim();

    match keyword {
        "options" => parse_options_directive(rest),
        "blacklist" => parse_blacklist_directive(rest),
        "install" => parse_install_directive(rest),
        "softdep" | "alias" | "override" | "remove" => ParsedDirective::Unrecognised {
            keyword,
        },
        _ => ParsedDirective::Malformed,
    }
}

/// Parse the body of an `options` directive after the keyword.
///
/// Format: `<module> <param1>=<value1> [<param2>=<value2> ...]`
///
/// Returns `Malformed` if the module name is absent or no valid params follow.
fn parse_options_directive(rest: &str) -> ParsedDirective<'_> {
    let mut parts = rest.splitn(2, char::is_whitespace);
    let Some(module) = parts.next().filter(|s| !s.is_empty()) else {
        return ParsedDirective::Malformed;
    };
    let params_str = parts.next().unwrap_or("").trim();

    let params: Vec<(&str, &str)> = params_str
        .split_whitespace()
        .filter_map(|kv| {
            let eq = kv.find('=')?;
            let key = kv[..eq].trim();
            let val = kv[eq.saturating_add(1)..].trim();
            if key.is_empty() {
                None
            } else {
                Some((key, val))
            }
        })
        .collect();

    if params.is_empty() && !params_str.is_empty() {
        // Non-empty rest but no parseable params — malformed.
        return ParsedDirective::Malformed;
    }

    ParsedDirective::Options {
        module,
        params,
    }
}

/// Parse the body of a `blacklist` directive.
///
/// Format: `<module>`
fn parse_blacklist_directive(rest: &str) -> ParsedDirective<'_> {
    let module = rest.trim();
    if module.is_empty() {
        return ParsedDirective::Malformed;
    }
    ParsedDirective::Blacklist {
        module,
    }
}

/// Parse the body of an `install` directive after the keyword.
///
/// Format: `<module> <command>` where `<command>` is the shell command
/// that `modprobe` executes instead of loading the module.
///
/// Hard blacklist detection: `/bin/true`, `/bin/false`, and `/usr/bin/true`
/// are recognised as sentinel no-op commands that unconditionally prevent
/// module loading. Other commands (e.g., `modprobe --ignore-install <mod>`,
/// `/sbin/modprobe --ignore-install`) are recorded as non-hard-blacklist
/// `Install` directives — logged for operator awareness but not treated as
/// security-relevant blacklist evidence.
///
/// Returns `Malformed` if the module name or command is absent.
///
/// NIST SP 800-53 CM-7: hard blacklist sentinel detection.
/// NIST SP 800-53 SI-10: Input Validation — command string is not executed;
/// only compared against a fixed set of known safe sentinel patterns.
fn parse_install_directive(rest: &str) -> ParsedDirective<'_> {
    // Split into module name and the rest of the command.
    let mut parts = rest.splitn(2, char::is_whitespace);
    let Some(module) = parts.next().filter(|s| !s.is_empty()) else {
        return ParsedDirective::Malformed;
    };
    let command = parts.next().unwrap_or("").trim();
    if command.is_empty() {
        return ParsedDirective::Malformed;
    }

    // Hard blacklist sentinel: the first token of the command must be one of
    // the recognised no-op paths. Only compare the command executable, not
    // any arguments, to avoid false positives from complex command strings.
    // We do not execute the command — only classify it by string comparison.
    let cmd_executable = command.split_whitespace().next().unwrap_or("");
    let is_hard_blacklist = matches!(
        cmd_executable,
        "/bin/true" | "/usr/bin/true" | "/bin/false" | "/usr/bin/false"
    );

    ParsedDirective::Install {
        module,
        command,
        is_hard_blacklist,
    }
}

// ===========================================================================
// Live sysfs cross-check helpers
// ===========================================================================

/// Validate a module or parameter name for use in sysfs path construction.
///
/// Rejects names that are empty, contain a `/` (path separator), contain a
/// null byte `\0`, or are the `..` component. These are the minimal guards
/// required to prevent path-traversal attacks when the name is joined with
/// `SYS_MODULE_BASE` to form a sysfs path.
///
/// Kernel module names in practice consist of alphanumeric characters,
/// underscores, and hyphens only. This function does not enforce that positive
/// constraint — it only rejects known-dangerous patterns.
///
/// NIST SP 800-53 SI-10: Input Validation.
fn is_valid_module_name(name: &str) -> bool {
    !name.is_empty() && !name.contains('/') && !name.contains('\0') && name != ".."
}

/// Check whether a kernel module is currently loaded by testing for the
/// existence of `/sys/module/<module_name>/` in sysfs.
///
/// This is the **Trust Gate** for modprobe parameter reads: only attempt
/// to read `/sys/module/<mod>/parameters/<param>` if the module directory
/// exists. If the module is absent, the live value is `None` (not a
/// contradiction — the module is not loaded, which confirms a blacklist).
///
/// The directory existence check is a regular filesystem call, not a
/// provenance-verified read. This is intentional: we are not reading content
/// from the kernel — we are testing for the presence of a sysfs directory,
/// which is a metadata operation. The actual parameter value read (below)
/// is provenance-verified.
///
/// NIST SP 800-53 CM-6: Trust Gate — do not attempt parameter reads when the
/// module is absent.
/// NIST SP 800-53 SI-10: Input Validation — `module_name` is validated against
/// path-traversal characters before use. Module names containing `/`, `\0`, or
/// `..` components are rejected; only the catalog-internal callers (which use
/// compile-time constant names) are expected in practice, but the public API
/// must guard against adversary-supplied input.
///
/// Returns `false` immediately for an empty `module_name` or a name containing
/// path-traversal characters, to prevent path construction anomalies.
///
/// # Security note — SELinux MAC enforcement
///
/// The `/sys/module/` directory hierarchy is labeled `sysfs_t` under SELinux.
/// On an enforcing system with the RHEL 10 targeted or MLS policy, symlink
/// substitution attacks against entries under `/sys/module/` are blocked by
/// type enforcement: the sysfs object class constraints prevent creating
/// symlinks in `sysfs_t`-labeled directories that point outside the sysfs
/// pseudo-filesystem. This makes the metadata-only `is_dir()` check safe as
/// a trust gate without requiring full `SYSFS_MAGIC` provenance verification —
/// provenance verification occurs in the subsequent `read_module_param` call
/// if the trust gate passes. This design depends on SELinux being in enforcing
/// mode; the posture probe documents this dependency here so reviewers have
/// explicit context. NIST SP 800-53 SI-7; NSA RTB RAIN.
#[must_use = "module loaded check result must be examined"]
pub fn is_module_loaded(module_name: &str) -> bool {
    if !is_valid_module_name(module_name) {
        log::debug!(
            "posture: modprobe: is_module_loaded: \
             rejected invalid module name (empty, contains '/', '\\0', or '..')"
        );
        return false;
    }
    Path::new(SYS_MODULE_BASE).join(module_name).is_dir()
}

/// Read a module parameter value from sysfs using provenance-verified
/// `SysfsText` with `SYSFS_MAGIC`.
///
/// Returns `Ok(Some(value))` on success, `Ok(None)` if the parameter node
/// is absent (module loaded but parameter file does not exist), or `Err`
/// on I/O or provenance failure.
///
/// **Prerequisite**: call `is_module_loaded()` first (Trust Gate). If the
/// module is not loaded, do not call this function.
///
/// `module_name` and `param_name` are validated against path-traversal
/// characters before use. Names containing `/`, `\0`, or `..` components are
/// rejected with `io::ErrorKind::InvalidInput`. In normal usage, both names
/// come from compile-time catalog constants and are safe; this guard protects
/// the public API surface against adversary-supplied names.
///
/// NIST SP 800-53 SI-7: provenance-verified via `SYSFS_MAGIC`.
/// NIST SP 800-53 SI-10: Input Validation — module and param names are
/// validated against path-traversal characters before path construction.
/// NSA RTB RAIN: Non-bypassable path through `SysfsText` + `SecureReader`.
///
/// # Errors
///
/// Returns `io::Error` if the modprobe configuration directory cannot be read.
#[must_use = "sysfs parameter read result must be examined"]
pub fn read_module_param(module_name: &str, param_name: &str) -> io::Result<Option<String>> {
    use crate::kattrs::sysfs::SysfsText;
    use crate::kattrs::traits::SecureReader;

    if !is_valid_module_name(module_name) || !is_valid_module_name(param_name) {
        log::debug!(
            "posture: modprobe: read_module_param: \
             rejected invalid name (empty, contains '/', '\\0', or '..')"
        );
        return Err(io::Error::from(io::ErrorKind::InvalidInput));
    }

    #[cfg(debug_assertions)]
    let start = std::time::Instant::now();

    let param_path =
        PathBuf::from(SYS_MODULE_BASE).join(module_name).join("parameters").join(param_name);

    let node = match SysfsText::new(param_path) {
        Ok(n) => n,
        Err(e) => {
            log::debug!(
                "posture: modprobe cross-check: {module_name}/{param_name}: \
                 path construction failed: {e}"
            );
            return Err(e);
        }
    };

    let result = match SecureReader::<SysfsText>::new().read_generic_text(&node) {
        Ok(s) => Ok(Some(s.trim_end_matches('\n').to_owned())),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(e),
    };

    #[cfg(debug_assertions)]
    log::debug!(
        "posture: modprobe cross-check: {module_name}/{param_name} \
         sysfs read completed in {} µs, result={:?}",
        start.elapsed().as_micros(),
        result.as_ref().map(|r| r.as_deref())
    );

    result
}

/// Evaluate the configured value and live state for a blacklist indicator.
///
/// For blacklist indicators, `ConfiguredValue::raw` is set to `"blacklisted"`
/// when a `blacklist <module>` entry exists in modprobe.d, and `"absent"`
/// when no such entry is found.
///
/// The live value assessment:
/// - Module absent from sysfs (`!is_module_loaded`) → `LiveValue::Bool(true)`
///   (blacklist effective — module not loaded).
/// - Module present in sysfs → `LiveValue::Bool(false)` (module loaded despite
///   potential blacklist entry).
///
/// NIST SP 800-53 CM-6: contradiction detection for module blacklist state.
/// NIST SP 800-53 AU-3: structured evidence for audit.
#[must_use = "blacklist configured-value result must be examined"]
pub fn blacklist_configured_value(
    module_name: &str,
    config: &ModprobeConfig,
) -> Option<ConfiguredValue> {
    config.is_blacklisted(module_name).map(|_| {
        let source = config.blacklist_source(module_name).unwrap_or("<unknown>").to_owned();
        ConfiguredValue {
            raw: "blacklisted".to_owned(),
            source_file: source,
        }
    })
}

/// Evaluate a module-parameter configured value and derive the live check.
///
/// Returns `(configured_value, live_raw)` where `live_raw` is the raw string
/// from sysfs (if readable and module is loaded), or `None` if unavailable.
///
/// **Trust Gate**: returns `(configured, None)` without attempting a sysfs read
/// if `is_module_loaded(module_name)` is false.
///
/// NIST SP 800-53 CM-6: Trust Gate and configured-value lookup.
/// NIST SP 800-53 SI-7: sysfs parameter read provenance-verified via SYSFS_MAGIC.
#[must_use = "module param evaluation result must be examined"]
pub fn param_configured_and_live(
    module_name: &str,
    param_name: &str,
    config: &ModprobeConfig,
) -> (Option<ConfiguredValue>, Option<String>) {
    let configured = config.get_option(module_name, param_name);

    // Trust Gate: only read sysfs if module is loaded.
    if !is_module_loaded(module_name) {
        log::debug!(
            "posture: modprobe cross-check: {module_name}: module not loaded \
             — skipping sysfs parameter read (Trust Gate)"
        );
        return (configured, None);
    }

    let live_raw = match read_module_param(module_name, param_name) {
        Ok(v) => v,
        Err(e) => {
            log::debug!(
                "posture: modprobe cross-check: {module_name}/{param_name}: \
                 sysfs read failed: {e}"
            );
            None
        }
    };

    (configured, live_raw)
}
