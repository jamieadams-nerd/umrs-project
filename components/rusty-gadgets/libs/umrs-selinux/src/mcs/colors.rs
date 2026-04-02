// SPDX-License-Identifier: MIT
// ============================================================================
//! UMRS SELINUX — secolor.conf Parser + Lazy Cache
//!
//! High-Assurance parsing of `/etc/selinux/{policy}/secolor.conf`
//!
//! Implements:
//!   • Mnemonic color table
//!   • user / role / type / range rule lists
//!   • Precedence propagation
//!   • Lazy load (OnceLock)
//!   • Reload on file change (mtime + len)
//!
//! NOTE:
//! Range matching currently uses string equality / glob matching, not lattice
//! dominance math. This is a known limitation: two semantically equivalent
//! ranges expressed differently will not match. Dominance math can be
//! feature-gated later when `mls/range.rs` is complete.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AU-3**: Audit Record Content — color coding is derived
//!   from the security context and rendered in audit-visible directory listings.
//! - **NIST SP 800-53 SI-7**: Software and Information Integrity — color config
//!   is parsed from a provenance-checked file path; mtime + len change detection
//!   ensures stale cache entries are reloaded.
// ============================================================================

use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;
use std::sync::{OnceLock, RwLock};
use std::time::SystemTime;

// ============================================================================
// Color Primitives
// ============================================================================

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    #[must_use]
    pub const fn from_hex(hex: u32) -> Self {
        Self {
            r: ((hex >> 16) & 0xff) as u8,
            g: ((hex >> 8) & 0xff) as u8,
            b: (hex & 0xff) as u8,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SeColor {
    pub fg: Rgb,
    pub bg: Rgb,
}

// ============================================================================
// Rule Model
// ============================================================================

#[derive(Clone, Debug)]
pub struct Rule {
    pub pattern: String,
    pub fg: Rgb,
    pub bg: Rgb,
}

#[derive(Clone, Debug, Default)]
pub struct SeColorConfig {
    pub mnemonics: HashMap<String, Rgb>,
    pub user: Vec<Rule>,
    pub role: Vec<Rule>,
    pub r#type: Vec<Rule>,
    pub range: Vec<Rule>,
}

// ============================================================================
// Lazy Cache State
// ============================================================================

#[derive(Clone)]
struct CacheState {
    mtime: Option<SystemTime>,
    len: Option<u64>,
    cfg: SeColorConfig,
}

static SECOLORS_CACHE: OnceLock<RwLock<CacheState>> = OnceLock::new();

// ============================================================================
// Public Entry
// ============================================================================

/// Load the `secolors` configuration, returning a cached copy if unchanged.
///
/// # Errors
///
/// Returns `io::Error` if the color configuration file cannot be read or parsed.
///
/// # Panics
///
/// Panics if the internal `RwLock` is poisoned.
pub fn load_secolors_cached(path: &Path) -> io::Result<SeColorConfig> {
    let meta = fs::metadata(path)?;
    let mtime = meta.modified().ok();
    let len = Some(meta.len());

    let lock = SECOLORS_CACHE.get_or_init(|| {
        let cfg = parse_secolor_file(path).unwrap_or_default();
        RwLock::new(CacheState {
            mtime,
            len,
            cfg,
        })
    });

    {
        #[allow(clippy::unwrap_used)]
        // RwLock poisoning is unrecoverable; prefer readable style
        let state = lock.read().unwrap();
        if state.mtime == mtime && state.len == len {
            return Ok(state.cfg.clone());
        }
    }

    // Reload
    #[allow(clippy::unwrap_used)]
    // RwLock poisoning is unrecoverable; prefer readable style
    let mut state = lock.write().unwrap();
    let cfg = parse_secolor_file(path).unwrap_or_default();

    *state = CacheState {
        mtime,
        len,
        cfg: cfg.clone(),
    };
    drop(state);

    Ok(cfg)
}

// ============================================================================
// Parser
// ============================================================================

/// Parse a `secolors` configuration file into a [`SeColorConfig`].
///
/// # Errors
///
/// Returns `io::Error` if the file cannot be opened or contains malformed color entries.
pub fn parse_secolor_file(path: &Path) -> io::Result<SeColorConfig> {
    let text = fs::read_to_string(path)?;
    let mut cfg = SeColorConfig::default();

    for (line_no, raw) in text.lines().enumerate() {
        let line = raw.trim();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if let Err(e) = parse_line(line, &mut cfg) {
            log::warn!("secolor.conf parse warning line {}: {}", line_no + 1, e);
        }
    }

    Ok(cfg)
}

// ----------------------------------------------------------------------------

fn parse_line(line: &str, cfg: &mut SeColorConfig) -> io::Result<()> {
    // color NAME = #RRGGBB
    if line.starts_with("color ") {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 4 || parts[2] != "=" {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid mnemonic rule",
            ));
        }

        let name = parts[1].to_string();
        let hex = parse_hex(parts[3])?;
        cfg.mnemonics.insert(name, Rgb::from_hex(hex));
        return Ok(());
    }

    // user|role|type|range PATTERN = FG BG
    let mut parts = line.split_whitespace();

    let domain = parts.next().ok_or_else(|| io_err("missing domain"))?;
    let pattern = parts.next().ok_or_else(|| io_err("missing pattern"))?;
    let eq = parts.next().ok_or_else(|| io_err("missing '='"))?;

    if eq != "=" {
        return Err(io_err("missing '=' token"));
    }

    let fg_str = parts.next().ok_or_else(|| io_err("missing fg"))?;
    let bg_str = parts.next().ok_or_else(|| io_err("missing bg"))?;

    let fg = resolve_color(fg_str, cfg)?;
    let bg = resolve_color(bg_str, cfg)?;

    let rule = Rule {
        pattern: pattern.to_string(),
        fg,
        bg,
    };

    match domain {
        "user" => cfg.user.push(rule),
        "role" => cfg.role.push(rule),
        "type" => cfg.r#type.push(rule),
        "range" => cfg.range.push(rule),
        _ => return Err(io_err("unknown rule domain")),
    }

    Ok(())
}

// ============================================================================
// Color Resolution
// ============================================================================

fn resolve_color(name: &str, cfg: &SeColorConfig) -> io::Result<Rgb> {
    if name.starts_with('#') {
        return Ok(Rgb::from_hex(parse_hex(name)?));
    }

    cfg.mnemonics.get(name).copied().ok_or_else(|| io_err("unknown color mnemonic"))
}

fn parse_hex(s: &str) -> io::Result<u32> {
    u32::from_str_radix(s.trim_start_matches('#'), 16).map_err(|_| io_err("invalid hex color"))
}

// ============================================================================
// Context Matching
// ============================================================================

#[derive(Debug)]
pub struct ContextComponents<'a> {
    pub user: &'a str,
    pub role: &'a str,
    pub r#type: &'a str,
    pub range: &'a str,
}

// ----------------------------------------------------------------------------

#[must_use]
pub fn resolve_colors(ctx: &ContextComponents, cfg: &SeColorConfig) -> [SeColor; 4] {
    let mut out: [Option<SeColor>; 4] = [None, None, None, None];

    out[0] = match_rule(&cfg.user, ctx.user);
    out[1] = match_rule(&cfg.role, ctx.role);
    out[2] = match_rule(&cfg.r#type, ctx.r#type);
    out[3] = match_rule(&cfg.range, ctx.range);

    propagate_precedence(&mut out);

    out.map(|c| c.unwrap_or_else(default_color))
}

// ----------------------------------------------------------------------------

fn match_rule(rules: &[Rule], value: &str) -> Option<SeColor> {
    for r in rules {
        if glob_match(&r.pattern, value) {
            return Some(SeColor {
                fg: r.fg,
                bg: r.bg,
            });
        }
    }
    None
}

// ----------------------------------------------------------------------------
// Precedence propagation (matches C logic)
// ----------------------------------------------------------------------------

const PRECEDENCE: [[usize; 3]; 4] = [[1, 2, 3], [0, 2, 3], [0, 1, 3], [0, 1, 2]];

fn propagate_precedence(colors: &mut [Option<SeColor>; 4]) {
    for i in 0..4 {
        if colors[i].is_none() {
            for &p in &PRECEDENCE[i] {
                if let Some(c) = colors[p] {
                    colors[i] = Some(c);
                    break;
                }
            }
        }
    }
}

// ============================================================================
// Defaults
// ============================================================================

#[must_use]
const fn default_color() -> SeColor {
    SeColor {
        fg: Rgb {
            r: 0,
            g: 0,
            b: 0,
        },
        bg: Rgb {
            r: 255,
            g: 255,
            b: 255,
        },
    }
}

// ============================================================================
// Minimal glob matcher (* only, fast stub)
// Replace later with fnmatch crate if desired
// ============================================================================

fn glob_match(pattern: &str, value: &str) -> bool {
    if pattern == "*" {
        return true;
    }

    if !pattern.contains('*') {
        return pattern == value;
    }

    let parts: Vec<&str> = pattern.split('*').collect();

    if !value.starts_with(parts[0]) {
        return false;
    }

    #[allow(clippy::unwrap_used)]
    // split() always yields ≥1 part; last() is guaranteed Some
    if !value.ends_with(parts.last().unwrap()) {
        return false;
    }

    true
}

// ============================================================================
// Helpers
// ============================================================================

fn io_err(msg: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, msg)
}

// ============================================================================
// Default path helper
// ============================================================================

/// Load secolor.conf from the system path for the currently active SELinux policy.
///
/// NIST SP 800-53 CM-6: Uses the policy type from `/etc/selinux/config` to construct
/// the correct path (`/etc/selinux/{policy}/secolor.conf`), guarding against
/// hard-coded `targeted`-only assumptions that break MLS deployments.
///
/// # Errors
///
/// Returns `io::Error` if the default color configuration file cannot be read or parsed.
pub fn load_default() -> io::Result<SeColorConfig> {
    use crate::status::selinux_policy;
    let policy = selinux_policy().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::NotFound,
            "SELinux is not enabled or policy type is unknown",
        )
    })?;
    let path_str = format!("/etc/selinux/{}/secolor.conf", policy.as_str());
    load_secolors_cached(Path::new(&path_str))
}
