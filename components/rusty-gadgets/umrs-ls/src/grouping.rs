//! # grouping — File Cuddling / Compact View
//!
//! Groups a sorted slice of [`ListEntry`] values into [`FileGroup`] records,
//! where each group has a *base* file and zero or more *siblings* that share
//! the base name as a prefix (separated by `.`, `-`, or `_`).
//!
//! The grouping algorithm is O(n) single-pass over the already-sorted input —
//! no additional sorting, no quadratic prefix scanning.
//!
//! ## Key Exported Types
//!
//! - [`SiblingKind`] — classification of a sibling relative to its base
//! - [`Sibling`] — a single sibling entry with its kind
//! - [`FileGroup`] — a base entry with its associated siblings
//! - [`group_entries`] — the primary grouping function
//! - [`aggregate_size`] — total byte count for all siblings in a group
//! - [`sibling_summary`] — human-readable summary string for a group
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AU-3**: Complete audit record content — grouped output
//!   preserves all identity, label, and observation fields from the base entry.
//! - **NIST SP 800-53 AC-3**: Access Enforcement — grouping is display-layer
//!   only; no access decisions are derived or modified here.
//! - **NSA RTB**: Deterministic Execution — O(n) single-pass algorithm with
//!   stack-allocated state; no heap allocation in the inner loop.

// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)

use std::path::Path;

use umrs_selinux::utils::dirlist::ListEntry;

/// Classification of a sibling file relative to its base.
///
/// Used by display and JSON layers to render kind-specific labels
/// (e.g., "3 rotations, 1 signature").
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SiblingKind {
    /// Numeric rotation suffix: `.1`, `.2`, `-20260301`.
    Rotation,
    /// Compressed rotation: `.1.gz`, `.2.xz`, `.gz`, `.bz2`, `.xz`, `.zst`.
    CompressedRotation,
    /// Detached signature: `.sig`, `.asc`, `.p7s`.
    Signature,
    /// Integrity checksum: `.sha256`, `.sha512`, `.md5`.
    Checksum,
    /// Backup copy: `.bak`, `.orig`, `.old`.
    Backup,
    /// Any other sibling that matches the prefix rule.
    Related,
}

/// A sibling file paired with its classification.
#[derive(Debug, Clone)]
pub struct Sibling {
    /// The full `ListEntry` — all security metadata is preserved.
    pub entry: ListEntry,
    /// How this sibling relates to the base file.
    pub kind: SiblingKind,
}

/// A base file and all its siblings discovered in the same directory.
///
/// A standalone file (no siblings) is represented as a `FileGroup` with
/// `siblings` equal to an empty `Vec`.
#[derive(Debug, Clone)]
pub struct FileGroup {
    /// The shortest-prefix entry that anchors this group.
    pub base: ListEntry,
    /// All sibling entries, in the order they appeared in the input.
    pub siblings: Vec<Sibling>,
}

/// Determine whether `candidate` is a sibling of `base_name`.
///
/// Exposed as `pub(crate)` for direct testing of the separator invariant.
///
/// A sibling must:
/// 1. Have `base_name` as a strict prefix (i.e., be longer), AND
/// 2. Have one of `.`, `-`, or `_` immediately at `base_name.len()`, AND
/// 3. If the separator is `-`, the next character must be an ASCII digit.
///
/// The separator check prevents `file.log` from absorbing `file.logging`.
/// The digit-after-dash rule prevents named siblings like `jvm-common` from
/// cuddling under `jvm`; only numeric/date-stamped dash suffixes (e.g.,
/// `boot.log-20260301`) are treated as rotations. Dot- and underscore-
/// separated siblings are unaffected.
#[must_use = "caller must decide what to do with the sibling decision"]
pub fn is_sibling(base_name: &str, candidate: &str) -> bool {
    if candidate.len() <= base_name.len() {
        return false;
    }
    if !candidate.starts_with(base_name) {
        return false;
    }
    let bytes = candidate.as_bytes();
    let sep = bytes[base_name.len()];
    match sep {
        b'.' | b'_' => true,
        b'-' => {
            // Only treat `-` as a sibling separator when followed by a digit.
            // This keeps date/rotation suffixes (`-20260301`, `-1`) while
            // rejecting named siblings (`jvm-common` under `jvm`).
            bytes
                .get(base_name.len().saturating_add(1))
                .is_some_and(u8::is_ascii_digit)
        }
        _ => false,
    }
}

/// Classify the suffix of a candidate name relative to its base.
///
/// `suffix` is the portion of the candidate name after `base_name` (including
/// the separator character).  For example, if `base_name = "boot.log"` and
/// `candidate = "boot.log-20260301.gz"`, then `suffix = "-20260301.gz"`.
#[must_use = "classification result must be used to build the Sibling record"]
pub fn classify_suffix(suffix: &str) -> SiblingKind {
    // Strip the leading separator (`.`, `-`, `_`) for easier matching.
    let rest = &suffix[1..];

    // Use Path::extension() for extension-based matching to satisfy the
    // case_sensitive_file_extension_comparisons lint and get correct semantics
    // for dotted suffixes (e.g., "1.gz" → extension "gz").
    let ext = Path::new(rest).extension().and_then(|e| e.to_str()).unwrap_or("");

    // Compressed rotation: the final extension is a known compression format.
    // Also handle the bare-extension case (rest == "gz" etc.) where there is
    // no dot in `rest` — Path::extension() returns None for bare words.
    if ext.eq_ignore_ascii_case("gz")
        || ext.eq_ignore_ascii_case("bz2")
        || ext.eq_ignore_ascii_case("xz")
        || ext.eq_ignore_ascii_case("zst")
        || rest.eq_ignore_ascii_case("gz")
        || rest.eq_ignore_ascii_case("bz2")
        || rest.eq_ignore_ascii_case("xz")
        || rest.eq_ignore_ascii_case("zst")
    {
        return SiblingKind::CompressedRotation;
    }

    // Signature files.
    if ext.eq_ignore_ascii_case("sig")
        || ext.eq_ignore_ascii_case("asc")
        || ext.eq_ignore_ascii_case("p7s")
        || rest.eq_ignore_ascii_case("sig")
        || rest.eq_ignore_ascii_case("asc")
        || rest.eq_ignore_ascii_case("p7s")
    {
        return SiblingKind::Signature;
    }

    // Checksum files.
    if ext.eq_ignore_ascii_case("sha256")
        || ext.eq_ignore_ascii_case("sha512")
        || ext.eq_ignore_ascii_case("md5")
        || rest.eq_ignore_ascii_case("sha256")
        || rest.eq_ignore_ascii_case("sha512")
        || rest.eq_ignore_ascii_case("md5")
    {
        return SiblingKind::Checksum;
    }

    // Backup files.
    if ext.eq_ignore_ascii_case("bak")
        || ext.eq_ignore_ascii_case("orig")
        || ext.eq_ignore_ascii_case("old")
        || rest.eq_ignore_ascii_case("bak")
        || rest.eq_ignore_ascii_case("orig")
        || rest.eq_ignore_ascii_case("old")
    {
        return SiblingKind::Backup;
    }

    // Rotation: pure numeric suffix, or a date-like numeric string.
    // Accept: "1", "2", "20260301" — anything that is entirely ASCII digits.
    if rest.chars().all(|c| c.is_ascii_digit()) {
        return SiblingKind::Rotation;
    }

    // Rotation with a dotted-numeric prefix: the first component before any
    // extension is purely numeric (e.g., "1" in "1.old" is already Backup
    // above; "1" in "1" alone is Rotation; "20260301" is Rotation).
    let numeric_part = rest.split('.').next().unwrap_or("");
    if !numeric_part.is_empty() && numeric_part.chars().all(|c| c.is_ascii_digit()) {
        return SiblingKind::Rotation;
    }

    SiblingKind::Related
}

/// Group a sorted slice of [`ListEntry`] values into [`FileGroup`] records.
///
/// The input **must** be sorted lexically by filename (the contract from
/// `list_directory`).  The algorithm is O(n) single-pass: each entry is
/// examined exactly once, accumulated into the current group or promoted to
/// start a new group.
///
/// # Grouping Rule
///
/// An entry `C` is a sibling of the current base `B` when:
/// - `C.name` starts with `B.name`, AND
/// - The character at `B.name.len()` is `.`, `-`, or `_`.
///
/// The separator requirement prevents `file.log` from absorbing `file.logging`.
///
/// # NIST SP 800-53 AC-3 / NSA RTB
///
/// Grouping is purely a display concern.  No SELinux label, mode bit, or
/// security observation is modified or suppressed.
#[must_use = "grouped result drives all subsequent display and JSON output"]
pub fn group_entries(entries: &[ListEntry]) -> Vec<FileGroup> {
    let mut result: Vec<FileGroup> = Vec::new();
    let mut current: Option<FileGroup> = None;

    for entry in entries {
        let name = entry.dirent.name.as_str();

        // Attempt to extend the current group.
        let is_sib = if let Some(ref base_group) = current {
            is_sibling(base_group.base.dirent.name.as_str(), name)
        } else {
            false
        };

        if is_sib {
            // Safety: current is Some — we just checked it.
            if let Some(ref mut base_group) = current {
                let base_name = base_group.base.dirent.name.as_str();
                let suffix = &name[base_name.len()..];
                let kind = classify_suffix(suffix);
                base_group.siblings.push(Sibling {
                    entry: entry.clone(),
                    kind,
                });
            }
        } else {
            // Flush the current group and start a new one.
            if let Some(finished) = current.take() {
                result.push(finished);
            }
            current = Some(FileGroup {
                base: entry.clone(),
                siblings: Vec::new(),
            });
        }
    }

    // Flush the final group.
    if let Some(finished) = current.take() {
        result.push(finished);
    }

    result
}

/// Total byte size of all siblings in a group (base file excluded).
///
/// Returns `0` for standalone files (no siblings).
#[must_use = "aggregate size is used in summary display and JSON output"]
pub fn aggregate_size(group: &FileGroup) -> u64 {
    group.siblings.iter().fold(0u64, |acc, s| {
        acc.saturating_add(s.entry.dirent.size.as_u64())
    })
}

/// Human-readable summary of the sibling kinds in a group.
///
/// Returns a string like `"3 rotations, 1 signature"` or `"7 rotations"`.
/// Returns an empty string for standalone files (no siblings).
///
/// Mixed-compression and rotation entries are counted separately by kind so
/// the operator knows the exact composition of the group.
#[must_use = "summary string drives the cuddle line display"]
pub fn sibling_summary(group: &FileGroup) -> String {
    if group.siblings.is_empty() {
        return String::new();
    }

    let mut rotations: u32 = 0;
    let mut compressed: u32 = 0;
    let mut signatures: u32 = 0;
    let mut checksums: u32 = 0;
    let mut backups: u32 = 0;
    let mut related: u32 = 0;

    for sib in &group.siblings {
        match sib.kind {
            SiblingKind::Rotation => rotations = rotations.saturating_add(1),
            SiblingKind::CompressedRotation => {
                compressed = compressed.saturating_add(1);
            }
            SiblingKind::Signature => signatures = signatures.saturating_add(1),
            SiblingKind::Checksum => checksums = checksums.saturating_add(1),
            SiblingKind::Backup => backups = backups.saturating_add(1),
            SiblingKind::Related => related = related.saturating_add(1),
        }
    }

    let mut parts: Vec<String> = Vec::new();
    if rotations > 0 {
        parts.push(format!(
            "{rotations} {}",
            plural(rotations, "rotation", "rotations")
        ));
    }
    if compressed > 0 {
        parts.push(format!("{compressed} compressed"));
    }
    if signatures > 0 {
        parts.push(format!(
            "{signatures} {}",
            plural(signatures, "signature", "signatures")
        ));
    }
    if checksums > 0 {
        parts.push(format!(
            "{checksums} {}",
            plural(checksums, "checksum", "checksums")
        ));
    }
    if backups > 0 {
        parts.push(format!(
            "{backups} {}",
            plural(backups, "backup", "backups")
        ));
    }
    if related > 0 {
        parts.push(format!("{related} related"));
    }

    parts.join(", ")
}

const fn plural(n: u32, singular: &'static str, plural_form: &'static str) -> &'static str {
    if n == 1 {
        singular
    } else {
        plural_form
    }
}
