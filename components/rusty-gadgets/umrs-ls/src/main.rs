// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//
// umrs-ls — Security Focused Directory Listing
//
// Displays the output of `list_directory()` grouped by (SELinux type,
// security marking). SELinux type and marking appear in group headers only
// — they are not repeated in every row.
//
// Usage:
//   umrs-ls [PATH] [--color] [--no-iov] [--no-mtime]
//           [--with-size] [--with-inode]
//
// Default path: current directory. Color is off by default.
//
// NIST SP 800-53 AC-3, AC-4, AU-3 / NSA RTB Non-Bypassability & TOCTOU safety.

#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![deny(clippy::unwrap_used)]
// Lint suppressions — rationale mirrors umrs-selinux policy:
//   doc_markdown:       backtick-wrapping every field name in prose is disruptive
//   missing_errors_doc: # Errors sections on every Result fn adds noise
//   missing_panics_doc: # Panics sections for unreachable paths add no value
//   option_if_let_else: explicit if/else preferred over .map_or_else() chains
//   map_unwrap_or:      multi-step find/map/unwrap_or chains are clear as written
//   format_push_string: write! over push_str(&format!()) is micro-opt; readability wins
#![allow(clippy::option_if_let_else)]
#![allow(clippy::map_unwrap_or)]
// format_push_string: write! over push_str(&format!()) is a micro-opt; readability wins here.
// Rationale mirrors Cargo.toml [lints.clippy] entry — needed because xtask passes -D warnings
// which overrides Cargo.toml lint table entries.
#![allow(clippy::format_push_string)]

use std::borrow::Cow;
use std::fmt::Write;
use std::io;
use std::path::Path;
use std::time::SystemTime;

use chrono::{DateTime, Local};
use umrs_core::i18n;

use umrs_core::human::sizefmt::{SizeBase, auto_format as fmt_size};
use umrs_selinux::ObservationKind;
use umrs_selinux::mcs::colors::{
    ContextComponents, Rgb, SeColorConfig, load_secolors_cached, resolve_colors,
};
use umrs_selinux::secure_dirent::{FileType, InodeSecurityFlags};
use umrs_selinux::utils::dirlist::{
    Column, ColumnSet, DirGroup, GroupKey, ListEntry, list_directory,
};

const TERM_WIDTH: usize = 100;
const ROW_INDENT: &str = "  "; // 2-space left indent on every row
const NAME_PREFIX: &str = "   "; // 3-char icon zone before filename

const BOLD_RED: &str = "\x1b[1;31m";
const BOLD_UNDER: &str = "\x1b[1;4m";
const DIM_ITALIC: &str = "\x1b[2;3m";
const DIM: &str = "\x1b[2m";
const GREEN: &str = "\x1b[32m";
const RED: &str = "\x1b[31m";
const RESET: &str = "\x1b[0m";
const UNDERLINE: &str = "\x1b[4m";

// Runtime display configuration — colour switch, mount symbols, and loaded
// secolor config.
//
// Pass `&DisplayConfig` down to every rendering function so all display
// decisions are centralised here rather than scattered across flags.
//  -  use_color - When `false` (default) all ANSI escape codes are suppressed.
//  -  mount_symbol - Unicode symbol shown in the icon zone for ordinary mount points.
//  -  plaindir_symbol - Unicode symbol for plain folder.
//  -  encrypted_symbol - Unicode symbol shown in the icon zone for encrypted mount points.
//  -  secolor - Loaded secolor.conf — `None` if absent or unreadable.
//
struct DisplayConfig {
    use_color: bool,
    mount_symbol: &'static str,
    encrypted_symbol: &'static str,
    plaindir_symbol: &'static str,
    secolor: Option<SeColorConfig>,
}

impl DisplayConfig {
    fn build(use_color: bool) -> Self {
        let secolor = if use_color {
            load_secolors_cached(Path::new(
                "/etc/selinux/targeted/secolor.conf",
            ))
            .ok()
        } else {
            None
        };

        Self {
            use_color,
            mount_symbol: "\u{26C1}",
            plaindir_symbol: "\u{1F4C1}",
            encrypted_symbol: "\u{1F512}", // Lock icon
            secolor,
        }
    }
}

fn main() -> io::Result<()> {
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("warn"),
    )
    .format_timestamp(None)
    .init();

    i18n::init("umrs-ls");

    let args: Vec<String> = std::env::args().collect();

    // First non-flag argument after the binary name is the path.
    let target = args
        .iter()
        .skip(1)
        .find(|a| !a.starts_with("--"))
        .map(String::as_str)
        .unwrap_or(".");

    // SelinuxType and Marking appear in the group header — omit from rows.
    let mut cols = ColumnSet::default()
        .without(Column::SelinuxType)
        .without(Column::Marking);

    if args.contains(&"--no-iov".to_owned()) {
        cols = cols.without(Column::Iov);
    }
    if args.contains(&"--no-mtime".to_owned()) {
        cols = cols.without(Column::Mtime);
    }
    if args.contains(&"--with-size".to_owned()) {
        cols = cols.with(Column::Size);
    }
    if args.contains(&"--with-inode".to_owned()) {
        cols = cols.with(Column::Inode);
    }

    let use_color = args.contains(&"--color".to_owned());
    let cfg = DisplayConfig::build(use_color);

    let listing = list_directory(Path::new(target))?;

    // Pre-scan column widths across all groups and entries.
    let widths = compute_widths(&listing.groups, &cols, &cfg);

    // Header row + rule.
    print_header(&cols, &widths);
    println!("{}", "=".repeat(TERM_WIDTH));

    // Groups.
    let mut total_entries = 0usize;
    for group in &listing.groups {
        println!(); // Seperate every group
        println!("{}", group_separator(&group.key, &cfg));
        for entry in &group.entries {
            print_row(entry, &group.key, &cols, &widths, &cfg);
            total_entries += 1;
        }
    }

    // Access-denied summary.
    if !listing.access_denied.is_empty() {
        println!();
        let n = listing.access_denied.len();
        // Translate the static label, then append the count in parentheses.
        let label = format!("{} ({n}) ", i18n::tr("access denied"));
        let fill = "=".repeat(TERM_WIDTH.saturating_sub(label.len()));
        println!("{label}{fill}");
        for name in &listing.access_denied {
            println!("{ROW_INDENT}{name}");
        }
    }

    // Timing footer.
    println!();
    println!(
        "{total_entries} entries  {}  {} groups  {} µs",
        listing.path.display(),
        listing.groups.len(),
        listing.elapsed_us,
    );

    Ok(())
}

// Column width pre-scan

fn compute_widths(
    groups: &[DirGroup],
    cols: &ColumnSet,
    cfg: &DisplayConfig,
) -> Vec<(Column, usize)> {
    cols.columns()
        .iter()
        .filter(|&&c| c != Column::Iov && c != Column::Name)
        .map(|&col| {
            let header_w = col_header(col).len();
            let data_w = groups
                .iter()
                .flat_map(|g| {
                    g.entries
                        .iter()
                        .map(|e| cell_plain(e, col, &g.key, cfg).len())
                })
                .max()
                .unwrap_or(0);
            (col, header_w.max(data_w) + 2)
        })
        .collect()
}

fn col_width(widths: &[(Column, usize)], col: Column) -> usize {
    for (c, w) in widths {
        if *c == col {
            return *w;
        }
    }
    14
}

//============================================================================
// HEADER
//============================================================================
fn print_header(cols: &ColumnSet, widths: &[(Column, usize)]) {
    let mut line = ROW_INDENT.to_owned();

    for &col in cols.columns() {
        if col == Column::Iov {
            // write! formats directly into 'line' without a temporary String
            let _ = write!(line, "{:<5}", col_header(col));
        } else if col == Column::Name {
            line.push_str(NAME_PREFIX);
            line.push_str(&col_header(col));
        } else {
            let w = col_width(widths, col);
            let _ = write!(line, "{:<w$}", col_header(col));
        }
    }
    println!("{line}");
}

// This will become table header for Textual & Graphical Unser Interfaces
fn col_header(col: Column) -> Cow<'static, str> {
    match col {
        // Wrap translations in Cow::Owned
        Column::Mode => Cow::Owned(i18n::tr("MODE")),
        Column::Marking => Cow::Owned(i18n::tr("MARKING")),
        Column::UidGid => Cow::Owned(i18n::tr("OWNER:GROUP")),
        Column::Size => Cow::Owned(i18n::tr("SIZE")),
        Column::Mtime => Cow::Owned(i18n::tr("MODIFIED")),
        Column::Name => Cow::Owned(i18n::tr("NAME")),

        // Wrap static literals in Cow::Borrowed
        Column::Iov => Cow::Borrowed("IOV"),
        Column::SelinuxType => Cow::Borrowed("SELINUX TYPE"),
        Column::Inode => Cow::Borrowed("INODE"),
    }
}

// ===========================================================================
// ROW Rendering
//
// Column Sizing:
//   Right-justify the value within (w - 2) chars and append 2 trailing
//   spaces.  Total output is still `w` chars — same as a left-justified
//   column — so the NAME icon that follows always has a visible gap.
//   Without this, the +2 padding from compute_widths ends up as leading
//   space only, leaving zero separation before the NAME icon (e.g. ⊕).
//
fn print_row(
    entry: &ListEntry,
    key: &GroupKey,
    cols: &ColumnSet,
    widths: &[(Column, usize)],
    cfg: &DisplayConfig,
) {
    let mut line = ROW_INDENT.to_owned();
    for &col in cols.columns() {
        if col == Column::Iov {
            line.push_str(&cell_iov(entry, cfg));
            line.push_str("  ");
        } else if col == Column::Name {
            line.push_str(&cell_plain(entry, col, key, cfg));
        } else if col == Column::Size {
            let w = col_width(widths, col);
            let inner = w.saturating_sub(2);
            line.push_str(&format!(
                "{:>inner$}  ",
                cell_plain(entry, col, key, cfg)
            ));
        } else {
            let w = col_width(widths, col);
            line.push_str(&format!("{:<w$}", cell_plain(entry, col, key, cfg)));
        }
    }
    println!("{line}");
}

// ==========================================================================
// Cell renderers
// ==========================================================================
// Plain cell value — used for both width pre-scan and row display.
//
// `Column::Iov` and `Column::Name` are handled separately in `print_row`;
// their arms here are used only during the width pre-scan.
fn cell_plain(
    entry: &ListEntry,
    col: Column,
    key: &GroupKey,
    cfg: &DisplayConfig,
) -> String {
    match col {
        Column::Mode => {
            // Standard 10-char mode string; append '+' when POSIX ACL present.
            let mut s = format!(
                "{}{}",
                file_type_char(entry.dirent.file_type),
                entry.dirent.mode.as_mode_str()
            );
            if entry.dirent.sec_flags.contains(InodeSecurityFlags::ACL_PRESENT)
            {
                s.push('+');
            }
            s
        }
        Column::Iov => "---".to_owned(),
        Column::SelinuxType => key.selinux_type.clone(),
        Column::Marking => key.marking.clone(),
        Column::UidGid => {
            let uid = entry.dirent.ownership.user.uid.as_u32();
            let gid = entry.dirent.ownership.group.gid.as_u32();
            let owner = resolve_username(uid);
            let group = resolve_groupname(gid);
            format!("{owner}:{group}")
        }
        Column::Size => {
            fmt_size(u128::from(entry.dirent.size.as_u64()), SizeBase::Binary)
        }
        Column::Mtime => format_mtime(entry.mtime),
        Column::Inode => entry.dirent.inode.as_u64().to_string(),
        Column::Name => {
            // 3-char icon zone: encrypted takes priority over plain mount.
            let icon = if entry.dirent.has_encryption() {
                format!("{}  ", cfg.encrypted_symbol)
            } else if entry.dirent.is_mountpoint {
                format!("{}  ", cfg.mount_symbol)
            } else if file_type_char(entry.dirent.file_type) == 'd' {
                // Only one trailing space because the icon is so big.
                format!("{} ", cfg.plaindir_symbol)
            } else {
                NAME_PREFIX.to_owned()
            };

            let mut name = entry.dirent.name.as_str().to_owned();
            if entry.dirent.file_type.is_directory() {
                name.push('/');
            } else if entry.dirent.mode.owner_can_execute() {
                name.push('*');
            }

            format!("{icon}{name}")
        }
    }
}

/// IOV security-posture marker — `I`=immutable, `O`=observations, `V`=IMA.
///
/// Visible width is always 3.  When `cfg.use_color` is `false`, ANSI
/// codes are omitted and plain ASCII characters are used instead.
///
/// `O` lights up (bold red) whenever `security_observations()` returns a
/// non-empty vec — the detail is left to the forthcoming `stat` command.
/// ACL presence is already expressed by `+` in the Mode column.
fn cell_iov(entry: &ListEntry, cfg: &DisplayConfig) -> String {
    let flags = &entry.dirent.sec_flags;

    let i = if flags.contains(InodeSecurityFlags::IMMUTABLE) {
        if cfg.use_color {
            format!("{RED}I{RESET}")
        } else {
            "I".to_owned()
        }
    } else if cfg.use_color {
        format!("{DIM}-{RESET}")
    } else {
        "-".to_owned()
    };

    // O lights up for any Risk-kind observation.
    // Using kind() keeps this logic stable as new observations are added —
    // new Risk variants automatically light up O without touching this code.
    // Warning and Good observations do not light up O.
    let posture_obs = entry
        .dirent
        .security_observations()
        .into_iter()
        .any(|o| o.kind() == ObservationKind::Risk);

    let o = if posture_obs {
        if cfg.use_color {
            format!("{BOLD_RED}O{RESET}")
        } else {
            "O".to_owned()
        }
    } else if cfg.use_color {
        format!("{DIM}-{RESET}")
    } else {
        "-".to_owned()
    };

    let v = if flags.contains(InodeSecurityFlags::IMA_PRESENT) {
        if cfg.use_color {
            format!("{GREEN}V{RESET}")
        } else {
            "V".to_owned()
        }
    } else if cfg.use_color {
        format!("{DIM}-{RESET}")
    } else {
        "-".to_owned()
    };

    format!("{i}{o}{v}")
}

// ================================================================
// GROUP HEADER - SELINUX TYPE + MARKING
// ================================================================
// Render the ` type :: marking ...` group separator line.
//
// Fill is computed from the plain text length so `=` characters always
// reach [`TERM_WIDTH`].  When `cfg.use_color` is `true` and secolor.conf
// is available, the type and marking are wrapped in ANSI 24-bit true-color.
fn group_separator(key: &GroupKey, cfg: &DisplayConfig) -> String {
    let plain = format!("{} :: {} ", key.selinux_type, key.marking);
    let fill = " ".repeat(TERM_WIDTH.saturating_sub(plain.len()));

    if cfg.use_color
        && let Some(ref sc) = cfg.secolor
    {
        let ctx = ContextComponents {
            user: "",
            role: "",
            r#type: &key.selinux_type,
            range: &key.marking,
        };
        let colors = resolve_colors(&ctx, sc);
        let type_out = ansi_fg(colors[2].fg, &key.selinux_type);
        let marking_out = ansi_fg(colors[3].fg, &key.marking);
        return format!("{type_out} :: {marking_out} {fill}");
    }

    if key.selinux_type == "<restricted>" {
        let selinux_type = i18n::tr("<restricted>");
        format!(
            "{DIM_ITALIC}{UNDERLINE}{0} :: {1} {fill}{RESET}",
            selinux_type, key.marking
        )
    } else {
        format!(
            "{BOLD_UNDER}{0} :: {1} {fill}{RESET}",
            key.selinux_type, key.marking
        )
    }
}

//
//Identity resolution
//
fn resolve_username(uid: u32) -> String {
    match nix::unistd::User::from_uid(nix::unistd::Uid::from_raw(uid)) {
        Ok(Some(u)) => u.name,
        _ => uid.to_string(),
    }
}

fn resolve_groupname(gid: u32) -> String {
    match nix::unistd::Group::from_gid(nix::unistd::Gid::from_raw(gid)) {
        Ok(Some(g)) => g.name,
        _ => gid.to_string(),
    }
}

fn ansi_fg(rgb: Rgb, text: &str) -> String {
    format!("\x1b[38;2;{};{};{}m{text}\x1b[0m", rgb.r, rgb.g, rgb.b)
}

const fn file_type_char(ft: FileType) -> char {
    match ft {
        FileType::Directory => 'd',
        FileType::Symlink => 'l',
        FileType::BlockDevice => 'b',
        FileType::CharDevice => 'c',
        FileType::Fifo => 'p',
        FileType::Socket => 's',
        FileType::RegularFile | FileType::Unknown => '-',
    }
}

fn format_mtime(mtime: Option<SystemTime>) -> String {
    if let Some(t) = mtime {
        let dt: DateTime<Local> = t.into();
        dt.format("%Y-%m-%d %H:%M").to_string()
    } else {
        "?".to_owned()
    }
}
