// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//! Configured-value reading for kernel security posture signals.
//!
//! Reads the *intended* configuration from the sysctl.d merge tree —
//! `/usr/lib/sysctl.d/`, `/run/sysctl.d/`, `/etc/sysctl.d/`, and
//! `/etc/sysctl.conf` — in precedence order (last writer wins, lexicographic
//! within each directory).
//!
//! ## Trust Boundary
//!
//! These are **regular files** on the root filesystem, NOT pseudo-filesystem
//! nodes under `/proc/` or `/sys/`. They do not require `SecureReader` or
//! `fstatfs` provenance verification. The Trust Gate Rule does not apply here:
//! we are reading the *intended* configuration, not the *effective* kernel
//! state. The live value (from `reader.rs`) is always authoritative; the
//! configured value is advisory.
//!
//! Configured values are best-effort: if files cannot be read (permissions,
//! container environment, absent paths), the configured value is `None` and
//! no error is raised to the caller. Failures are logged at `debug` level.
//!
//! ## Precedence Order
//!
//! Follows `sysctl(8)` precedence (last writer wins):
//!
//! 1. `/usr/lib/sysctl.d/*.conf` — distro defaults (lowest precedence)
//! 2. `/run/sysctl.d/*.conf` — transient overrides
//! 3. `/etc/sysctl.d/*.conf` — admin overrides
//! 4. `/etc/sysctl.conf` — legacy single-file (highest precedence)
//!
//! Within each directory, files are processed in lexicographic order (matching
//! `sysctl -p`).
//!
//! ## Phase 1 Scope
//!
//! Bootloader cmdline configured values (grub.cfg, loader entries) are
//! deferred to Phase 2. `configured_cmdline()` always returns `None` in
//! Phase 1.
//!
//! ## Compliance
//!
//! NIST SP 800-53 CM-6: Configuration Settings — reading the persistence layer
//! to compare intended vs. effective configuration.
//! NIST SP 800-53 CA-7: Continuous Monitoring — contradiction detection between
//! configured and live values is the primary use of this data.
//! NIST SP 800-53 SI-10: Input Validation — sysctl.d file content is validated
//! line-by-line via `parse_sysctl_line`; malformed lines are rejected and
//! logged rather than silently accepted or causing a parse error.

use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};

use crate::posture::signal::ConfiguredValue;

// ===========================================================================
// SysctlConfig — merged sysctl.d key→value map
// ===========================================================================

/// Merged sysctl configuration from the sysctl.d search tree.
///
/// Built by `SysctlConfig::load()`, which reads all sysctl.d sources in
/// precedence order and produces a final key→value map.
///
/// NIST SP 800-53 CM-6: provides the configured (persistence-layer) baseline
/// for contradiction detection.
pub struct SysctlConfig {
    /// Final merged map: sysctl key → (value, source file path).
    map: HashMap<String, (String, String)>,
}

impl SysctlConfig {
    /// Load and merge all sysctl.d configuration sources.
    ///
    /// Reads `/usr/lib/sysctl.d/`, `/run/sysctl.d/`, `/etc/sysctl.d/`, and
    /// `/etc/sysctl.conf` in precedence order. Missing directories and
    /// unreadable files are silently skipped (logged at `debug`).
    ///
    /// NIST SP 800-53 CM-6: collects the full configured baseline from all
    /// persistence layers.
    #[must_use = "sysctl config must be examined for configured values"]
    pub fn load() -> Self {
        #[cfg(debug_assertions)]
        let start = std::time::Instant::now();

        let mut map: HashMap<String, (String, String)> = HashMap::new();
        let mut file_count: usize = 0;

        // Process each source in ascending precedence order.
        // Later sources overwrite earlier ones (last-writer-wins).
        for dir in SYSCTL_SEARCH_DIRS {
            let dir_path = Path::new(dir);
            if dir_path.exists() {
                let loaded = load_conf_dir(dir_path, &mut map);
                file_count = file_count.saturating_add(loaded);
            } else {
                log::debug!("posture: sysctl.d dir absent: {dir}");
            }
        }

        // /etc/sysctl.conf — highest precedence legacy file.
        let legacy = Path::new("/etc/sysctl.conf");
        if legacy.exists()
            && let Ok(n) = load_conf_file(legacy, &mut map)
        {
            file_count = file_count.saturating_add(n);
        }

        let key_count = map.len();

        #[cfg(debug_assertions)]
        log::debug!(
            "posture: sysctl.d merge completed in {} µs: {} files, {} keys",
            start.elapsed().as_micros(),
            file_count,
            key_count
        );

        #[cfg(not(debug_assertions))]
        {
            let _ = file_count;
            let _ = key_count;
        }

        Self {
            map,
        }
    }

    /// Look up the configured value for a given sysctl key.
    ///
    /// Returns `Some(ConfiguredValue)` if the key was present in any
    /// sysctl.d file, or `None` if it was not found in any source.
    ///
    /// NIST SP 800-53 CM-6: returns the last-writer-wins effective configured value.
    #[must_use = "configured value lookup result must be examined"]
    pub fn get(&self, sysctl_key: &str) -> Option<ConfiguredValue> {
        self.map.get(sysctl_key).map(|(raw, source)| ConfiguredValue {
            raw: raw.clone(),
            source_file: source.clone(),
        })
    }

    /// Return the total number of keys loaded across all sources.
    #[must_use]
    pub fn key_count(&self) -> usize {
        self.map.len()
    }
}

// ===========================================================================
// Precedence table
// ===========================================================================

/// sysctl.d directories in ascending precedence order.
///
/// This matches the order specified in `sysctl(8)` and `sysctl.d(5)`.
/// Phase 1 intentionally omits `/lib/sysctl.d/` (deprecated path alias for
/// `/usr/lib/sysctl.d/` on modern RHEL) to avoid double-counting.
const SYSCTL_SEARCH_DIRS: &[&str] =
    &["/usr/lib/sysctl.d", "/run/sysctl.d", "/etc/sysctl.d"];

// ===========================================================================
// File loading helpers
// ===========================================================================

/// Load all `.conf` files from a directory in lexicographic order.
///
/// Returns the number of successfully parsed lines contributed across
/// all files. Unreadable files and lines that fail to parse are skipped
/// and logged at `debug`.
fn load_conf_dir(
    dir: &Path,
    map: &mut HashMap<String, (String, String)>,
) -> usize {
    let mut files: Vec<PathBuf> = match std::fs::read_dir(dir) {
        Ok(entries) => entries
            .filter_map(|e| e.ok().map(|e| e.path()))
            .filter(|p| {
                p.extension().and_then(|e| e.to_str()) == Some("conf")
                    && p.is_file()
            })
            .collect(),
        Err(e) => {
            log::debug!(
                "posture: cannot read sysctl.d dir {}: {e}",
                dir.display()
            );
            return 0;
        }
    };

    // Lexicographic sort — matches sysctl(8) processing order.
    files.sort();

    let mut total = 0usize;
    for path in &files {
        match load_conf_file(path, map) {
            Ok(n) => total = total.saturating_add(n),
            Err(e) => log::debug!("posture: skipping {}: {e}", path.display()),
        }
    }
    total
}

/// Parse one sysctl.conf-format file, inserting key=value pairs into `map`.
///
/// Format rules (matching `sysctl.conf(5)`):
/// - Lines starting with `#` or `;` are comments.
/// - Blank lines are ignored.
/// - `key = value` and `key=value` are both accepted.
/// - Keys may use `/` (path-style) or `.` (dotted-style); both are stored as-is
///   so lookups use the same style as the catalog's `sysctl_key` field.
///
/// Returns the number of key=value pairs successfully parsed.
///
/// NIST SP 800-53 SI-10: Input Validation — malformed lines are rejected and
/// logged rather than silently ignored or causing a parse error.
fn load_conf_file(
    path: &Path,
    map: &mut HashMap<String, (String, String)>,
) -> io::Result<usize> {
    let content = std::fs::read_to_string(path)?;
    let source = path.to_string_lossy().into_owned();
    let mut count = 0usize;

    for (line_no, line) in content.lines().enumerate() {
        let trimmed = line.trim();

        // Skip comments and blank lines.
        if trimmed.is_empty()
            || trimmed.starts_with('#')
            || trimmed.starts_with(';')
        {
            continue;
        }

        match parse_sysctl_line(trimmed) {
            Some((raw_key, value)) => {
                // Normalise slash-style keys (e.g., kernel/kptr_restrict) to dot-style
                // (kernel.kptr_restrict) at insertion time. The catalog uses dot-style
                // keys exclusively. Without this normalisation, sysctl.d files using
                // slash-style keys produce ConfiguredValue: None for every signal,
                // silently disabling contradiction detection.
                let key: String = raw_key.replace('/', ".");
                // Error Information Discipline (NIST SP 800-53 SI-11, SC-28):
                // Log only the key and line location — not the raw value — to limit
                // exposure of configuration values in the debug stream on DoD/CUI
                // systems where debug logging may be enabled during troubleshooting.
                log::debug!(
                    "posture: sysctl.d: {}:{} key={}",
                    source,
                    line_no.saturating_add(1),
                    key
                );
                map.insert(key, (value.to_owned(), source.clone()));
                count = count.saturating_add(1);
            }
            None => {
                log::debug!(
                    "posture: sysctl.d: {}:{} skipped (not a key=value line)",
                    source,
                    line_no.saturating_add(1)
                );
            }
        }
    }

    Ok(count)
}

/// Parse a single sysctl.conf line into a raw (key, value) pair.
///
/// Accepts `key = value`, `key=value`, `key=\tvalue` forms.
/// Returns `None` for malformed lines (no `=` separator, empty key).
///
/// The key is trimmed of leading/trailing whitespace but is NOT normalised
/// from slash-style to dot-style. Slash→dot normalisation is performed by
/// `load_conf_file` at insertion time so that keys stored in the map always
/// use dot-style, matching the catalog's `sysctl_key` fields.
///
/// The value is trimmed of leading/trailing whitespace.
///
/// Exposed as `pub` to allow direct integration testing of the parser
/// without filesystem interaction. The caller is responsible for slash→dot
/// normalisation (performed by `load_conf_file` at insertion time).
///
/// NIST SP 800-53 SI-10: Input Validation — malformed lines are rejected and
/// logged rather than silently ignored.
#[must_use = "sysctl.d parse result must be examined — None means the line is a comment or malformed"]
pub fn parse_sysctl_line(line: &str) -> Option<(&str, &str)> {
    let trimmed = line.trim();
    if trimmed.starts_with('#') || trimmed.starts_with(';') {
        return None;
    }

    let eq_pos = line.find('=')?;
    let key = line[..eq_pos].trim();
    let value = line[eq_pos.saturating_add(1)..].trim();

    if key.is_empty() {
        return None;
    }
    Some((key, value))
}

// ===========================================================================
// configured_cmdline — Phase 2 placeholder
// ===========================================================================

/// Read the configured cmdline from the bootloader configuration.
///
/// **Phase 1**: always returns `None`. Bootloader configuration parsing
/// is deferred to Phase 2 due to distro-specific path and format variation.
///
/// NIST SP 800-53 CM-6: boot-persistence layer for cmdline signals.
#[must_use = "configured cmdline result must be examined"]
pub fn configured_cmdline() -> Option<String> {
    log::debug!(
        "posture: configured_cmdline: deferred to Phase 2, returning None"
    );
    None
}
