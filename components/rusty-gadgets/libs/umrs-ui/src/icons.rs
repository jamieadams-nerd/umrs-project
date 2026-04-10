// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams

//! # icons — Shared Unicode Glyph Catalog for UMRS TUI Tools
//!
//! Single source of truth for every Unicode glyph used in any UMRS TUI
//! binary. Every `pub const` here is available to `umrs-ls`, `umrs-stat`,
//! `umrs-uname`, `umrs-label`, and any future tool that depends on
//! `umrs-ui`. Consistency across tools is the design goal: if two tools
//! display a "mount point" marker, it is the same glyph in both.
//!
//! Constants are grouped by visual role — not by Unicode block — so you
//! can find "the thing that shows a directory" without knowing whether it
//! is emoji or Supplemental Symbols and Pictographs:
//!
//! - **Prompt & text symbols** — arrows, prompts, cursors used in text UI
//! - **Filesystem objects** — directory, file, symlink, mount, encrypted
//! - **Tree & navigation** — chevrons, sibling connectors, parent nav
//! - **Group headers** — banner transitions
//! - **Security posture** — flags, shields, placeholder, denied
//! - **Configuration & activity** — gears, truck, actors
//!
//! ## Adding a New Icon
//!
//! 1. Search `.claude/references/unicode-symbols-corpus.txt` for the
//!    glyph's official name and codepoint. Do not guess codepoints.
//! 2. Add the constant in the topical section that fits its visual role.
//! 3. Include the glyph literal and the official Unicode name in the doc
//!    comment so `cargo doc` renders a usable reference.
//! 4. Reference the constant via `umrs_ui::icons::ICON_*` — never
//!    redefine glyphs inside a binary crate, and never embed `\u{...}`
//!    literals in a binary's render code.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AU-3**: Visual security state must be consistent
//!   across all UMRS tools. Centralizing the glyph catalog guarantees
//!   that "immutable", "world-writable", "labeled", and "denied" look
//!   identical wherever they appear.

// ===========================================================================
// Prompt & text symbols
// Arrows, prompt glyphs, and text-mode cursors used in status bars,
// prompt bars, and help text.  These are primarily text-rendered (not
// emoji), so they align correctly in monospace layouts.
// ===========================================================================

/// Upwards arrow (↑ — U+2191 UPWARDS ARROW).  Used in help text for key
/// hints (e.g. "↑↓: navigate").
pub const ARROW_UP: &str = "\u{2191}";

/// Downwards arrow (↓ — U+2193 DOWNWARDS ARROW).
pub const ARROW_DOWN: &str = "\u{2193}";

/// Leftwards arrow (← — U+2190 LEFTWARDS ARROW).
pub const ARROW_LEFT: &str = "\u{2190}";

/// Rightwards arrow (→ — U+2192 RIGHTWARDS ARROW).
pub const ARROW_RIGHT: &str = "\u{2192}";

/// Heavy round-tipped rightwards arrow (➜ — U+279C).
///
/// The UMRS prompt glyph — used in the search bar and the Go to bar.
/// Visually distinct from the plain `→` so operators can tell "this is
/// an input prompt" from "this is a key hint".
pub const PROMPT_ARROW: &str = "\u{279C}";

/// Full-block cursor for search/prompt input (█ — U+2588 FULL BLOCK).
pub const ICON_CURSOR: &str = "\u{2588}";

/// Horizontal ellipsis (… — U+2026 HORIZONTAL ELLIPSIS).
///
/// Always use this single glyph — never three ASCII periods (`...`) —
/// for any truncation indicator in UMRS output.  One codepoint, one
/// display column, consistent width across monospace fonts.
pub const ELLIPSIS: &str = "\u{2026}";

// ===========================================================================
// Filesystem objects
// Icons that represent a type of inode: directory, file, symlink, mount
// point, encrypted container.  One glyph per concept — if two tools show
// "encrypted", they show the same lock.
// ===========================================================================

/// Plain directory (📁 — U+1F4C1 FILE FOLDER).
pub const ICON_DIR: &str = "\u{1F4C1}";

/// "Old school" two-cell folder glyph (🮹🮺 — U+1FBB9 U+1FBBA).  Reserved
/// alternate folder style.
pub const ICON_FOLDER_OLD_SKOOL: &str = "\u{1FBB9}\u{1FBBA}";

/// Symbolic link (🔗 — U+1F517 LINK SYMBOL).
pub const ICON_SYMLINK: &str = "\u{1F517}";

/// Mount point (⛁ — U+26C1 BLACK DRAUGHTS KING, used as a database
/// cylinder).
pub const ICON_MOUNT: &str = "\u{26C1}";

/// Encrypted directory or file (🔒 — U+1F512 LOCK).
pub const ICON_ENCRYPTED: &str = "\u{1F512}";

// ===========================================================================
// Tree & navigation
// Glyphs used to draw tree structure (expand/collapse chevrons, sibling
// connectors) and to indicate navigation actions (parent directory).
// ===========================================================================

/// Parent directory marker (⇪ — U+21EA UPWARDS WHITE ARROW FROM BAR).
pub const ICON_PARENT: &str = "\u{21EA}";

/// Cuddled sibling connector (└ — U+2514 BOX DRAWINGS LIGHT UP AND RIGHT).
pub const ICON_SIBLING: &str = "\u{2514}";

/// Expanded group chevron (▼ ).  Trailing space is intentional — keeps
/// alignment across open/closed states in a tree view.
pub const CHEVRON_OPEN: &str = "▼ ";

/// Collapsed group chevron (▶ ).  Trailing space is intentional.
pub const CHEVRON_CLOSED: &str = "▶ ";

/// Cuddled-base expand chevron (🢰 — U+1F8B0 RIGHTWARDS ARROW WITH NOTCHED
/// TAIL, expanded state).  Used at end-of-line next to the sibling summary.
pub const CHEVRON_CUDDLE_OPEN: &str = "\u{1F8B0}";

/// Cuddled-base collapse chevron (🢱 — U+1F8B1 DOWNWARDS ARROW WITH NOTCHED
/// TAIL, collapsed state).
pub const CHEVRON_CUDDLE_CLOSED: &str = "\u{1F8B1}";

// ===========================================================================
// Group headers
// Glyphs used to draw banner/streamer transitions at the head of a
// grouped listing (e.g. SELinux type :: marking headers in umrs-ls).
// ===========================================================================

/// Banner transition triangle (🭬 — U+1FB6C LEFT TRIANGULAR ONE QUARTER
/// BLOCK).  Used between styled segments of a group header line.
pub const ICON_BANNER: &str = "\u{1FB6C}";

// ===========================================================================
// Security posture
// Glyphs that communicate a security state: findings, clear slots,
// labels, access denial.  These carry semantic weight — do not repurpose
// them for unrelated visuals.
// ===========================================================================

/// Label / tag (🏷 — U+1F3F7 LABEL).  Used to mark a security-label field.
pub const ICON_LABEL: &str = "\u{1F3F7}";

/// Shield with plus (⛨ — U+26E8 BLACK CROSS ON SHIELD).  Reserved for
/// "integrity-protected" indicators.
pub const ICON_SHIELD_PLUS: &str = "\u{26E8}";

/// Prohibited / access-denied marker (🚫 — U+1F6AB NO ENTRY SIGN).
pub const ICON_DENIED: &str = "\u{1F6AB}";

/// Solid flag (⚑ — U+2691 BLACK FLAG).  Used in the IOV column to mark
/// an entry carrying at least one `Risk` or `Warning`-kind security
/// observation.  Solid fill makes the marker read at scan-speed.
pub const ICON_FLAG: &str = "\u{2691}";

/// Outline flag (⚐ — U+2690 WHITE FLAG).  Reserved alternate glyph for
/// "potential finding, not yet evaluated" states.
pub const ICON_FLAG_OUTLINE: &str = "\u{2690}";

/// Subtle column placeholder (· — U+00B7 MIDDLE DOT).  Used when a
/// security-posture slot has no finding, so the column stays aligned
/// without the visual noise of a `-`.
pub const ICON_PLACEHOLDER: &str = "\u{00B7}";

/// Check mark (✓ — U+2713 CHECK MARK).
///
/// Used in posture columns to mark a positive state — e.g., the `E` slot
/// of IOVE when at-rest encryption is detected on the containing mount
/// point.  Single display column.
pub const ICON_CHECK: &str = "\u{2713}";

/// Ballot X (✗ — U+2717 BALLOT X).
///
/// Used in posture columns to mark a negative state that is a compliance
/// violation — e.g., the `E` slot of IOVE when a marked (CUI/Protected)
/// entry sits on unencrypted storage.  Single display column.
pub const ICON_CROSS: &str = "\u{2717}";

// ===========================================================================
// Configuration & activity
// Gears, trucks, and activity glyphs for future tools that visualise
// configuration edits, ingestion pipelines, or agent activity.
// ===========================================================================

/// Gear (⚙ — U+2699 GEAR).
pub const ICON_GEAR: &str = "\u{2699}";

/// Gear without hub (⛬ — U+26EC).
pub const ICON_GEAR_NO_HUB: &str = "\u{26EC}";

/// Black gear with handles (⛭ — U+26ED).
pub const ICON_GEAR_HANDLES: &str = "\u{26ED}";

/// Delivery truck (⛟ — U+26DF BLACK TRUCK).  Reserved for package
/// delivery / ingest pipeline visuals.
pub const ICON_CARGO_TRUCK: &str = "\u{26DF}";

// ===========================================================================
// Actors
// Figure glyphs for agent personification in future TUI tools.
// ===========================================================================

/// Standing actor figure (🯅 — U+1FBC5 STANDING FIGURE).
pub const ICON_ACTOR: &str = "\u{1FBC5}";

/// Running figure pair (🮲🮳 — U+1FBB2 U+1FBB3).
pub const ICON_RUNNING_FIGURE: &str = "\u{1FBB2}\u{1FBB3}";

// ===========================================================================
// Marking & catalog nodes
// Glyphs used in the label registry tree to distinguish leaf nodes
// (individual markings) from group branch nodes.
// ===========================================================================

/// ☐ Ballot Box — marking leaf nodes in tree displays (☐ — U+2610 BALLOT BOX).
pub const ICON_MARKING: &str = "\u{2610}";
