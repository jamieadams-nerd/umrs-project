// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//!
//! # RPM Header Blob Parser
//!
//! Pure-Rust parser for the binary header blobs stored in the `Packages`
//! table of an RPM SQLite database (`/var/lib/rpm/rpmdb.sqlite`).
//!
//! ## Format
//!
//! RPM SQLite blobs contain a header region with NO leading 8-byte magic
//! prefix (that prefix is only present in `.rpm` package files on disk).
//! The blob layout is:
//!
//! ```text
//! [ nindex : u32 BE ]   — number of index entries
//! [ hsize  : u32 BE ]   — size of the data store in bytes
//! [ index  : nindex × 16 bytes ]
//! [ store  : hsize bytes ]
//! ```
//!
//! Each 16-byte index entry:
//!
//! ```text
//! [ tag    : u32 BE ]   — RPM tag number
//! [ type   : u32 BE ]   — data type (STRING=6, STRING_ARRAY=8, INT32=3, INT32_ARRAY=4)
//! [ offset : u32 BE ]   — byte offset into the store
//! [ count  : u32 BE ]   — element count
//! ```
//!
//! ## TPI — Two-Path Independence
//!
//! Index entries are parsed by two independent paths:
//!
//! - **Path A** — `nom` big-endian integer combinator chain.
//! - **Path B** — manual `u32::from_be_bytes` slice extraction.
//!
//! The two sets are compared element-by-element; the function fails closed on
//! any disagreement. Tag lookups and data extraction use Path A results only.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 SI-7**: Software Integrity — two independent parses
//!   detect single-point-of-failure parse bugs before they affect a trust decision.
//! - **NIST SP 800-53 SI-10**: Information Accuracy — all offsets and counts
//!   are range-checked before use; fail-closed on any out-of-bounds condition.
//! - **NIST SP 800-53 CM-8**: Component Inventory — parsed output enables the
//!   pipeline to establish package ownership of files.
//! - **NIST SP 800-218 SSDF PW.4.1**: Secure arithmetic — every offset/size
//!   calculation uses `checked_*` operations.
//! - **NSA RTB**: Fail-closed — any disagreement between parse paths, any
//!   out-of-bounds offset, any malformed nul-terminated string → `Err`.

use std::fmt;

use nom::IResult;
use nom::multi::count;
use nom::number::complete::be_u32;

// ===========================================================================
// Constants — RPM tag numbers and type codes
// ===========================================================================

/// Maximum blob size we will attempt to parse. 16 MiB is generous headroom.
/// Legitimate RPM headers are typically < 1 MiB.
///
/// NIST SP 800-53 SI-10 — bounded input prevents excessive allocation.
pub const MAX_BLOB_BYTES: usize = 16 * 1024 * 1024;

/// Maximum number of index entries accepted per header. RPM headers for the
/// largest packages (kernel) have a few hundred entries; 4096 is safe headroom.
pub const MAX_INDEX_ENTRIES: u32 = 4096;

// ── Tag numbers ──────────────────────────────────────────────────────────────

const TAG_NAME: u32 = 1000;
const TAG_VERSION: u32 = 1001;
const TAG_RELEASE: u32 = 1002;
const TAG_FILEDIGESTS: u32 = 1035;
const TAG_DIRINDEXES: u32 = 1116;
const TAG_BASENAMES: u32 = 1117;
const TAG_DIRNAMES: u32 = 1118;
const TAG_FILEDIGESTALGO: u32 = 5011;

// ── Type codes ───────────────────────────────────────────────────────────────

const TYPE_INT32: u32 = 3;
const TYPE_INT32_ARRAY: u32 = 4;
const TYPE_STRING: u32 = 6;
const TYPE_STRING_ARRAY: u32 = 8;

// ===========================================================================
// Error type
// ===========================================================================

/// Errors produced by `parse_rpm_header`.
///
/// All variants carry only structural or size information — never file content,
/// security labels, or user data. NIST SP 800-53 SI-12.
#[derive(Debug)]
pub enum RpmHeaderError {
    /// Blob exceeds `MAX_BLOB_BYTES`.
    TooLarge(usize),
    /// Blob is shorter than the 8-byte header prefix.
    TooShort,
    /// `nindex` exceeds `MAX_INDEX_ENTRIES`.
    InvalidIndexCount(u32),
    /// Path A and Path B index-entry results disagreed.
    TpiDisagreement,
    /// An index entry's offset + data size exceeds the store length.
    OffsetOutOfBounds {
        tag: u32,
        offset: u32,
        store_len: u32,
    },
    /// A STRING or STRING_ARRAY entry has no nul terminator within the store.
    MissingNulTerminator {
        tag: u32,
    },
    /// A STRING or STRING_ARRAY entry is not valid UTF-8.
    Utf8Error {
        tag: u32,
    },
    /// A DIRINDEXES entry references a directory slot that does not exist.
    InvalidDirindex {
        file_idx: usize,
        dir_idx: u32,
        dir_count: usize,
    },
    /// The per-file tag arrays have inconsistent lengths — the header is malformed.
    ///
    /// All three arrays (basenames, dirindexes, filedigests) must have the same
    /// element count. A mismatch indicates corruption or tampering.
    ///
    /// NSA RTB fail-closed; NIST SP 800-53 SI-10 — information accuracy.
    ArrayLengthMismatch {
        /// Expected length (basenames count).
        expected: usize,
        /// Actual length of the dirindexes array.
        dirindexes: usize,
        /// Actual length of the filedigests array.
        digests: usize,
    },
}

impl fmt::Display for RpmHeaderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TooLarge(n) => {
                write!(f, "rpm header blob too large: {n} bytes")
            }
            Self::TooShort => {
                write!(f, "rpm header blob too short (< 8 bytes)")
            }
            Self::InvalidIndexCount(n) => {
                write!(
                    f,
                    "rpm header nindex={n} exceeds MAX_INDEX_ENTRIES={MAX_INDEX_ENTRIES}"
                )
            }
            Self::TpiDisagreement => {
                write!(
                    f,
                    "rpm header TPI: nom and manual parse paths disagreed"
                )
            }
            Self::OffsetOutOfBounds {
                tag,
                offset,
                store_len,
            } => write!(
                f,
                "rpm header tag={tag}: offset={offset} out of bounds (store_len={store_len})"
            ),
            Self::MissingNulTerminator {
                tag,
            } => {
                write!(
                    f,
                    "rpm header tag={tag}: missing nul terminator in store"
                )
            }
            Self::Utf8Error {
                tag,
            } => {
                write!(f, "rpm header tag={tag}: string is not valid UTF-8")
            }
            Self::InvalidDirindex {
                file_idx,
                dir_idx,
                dir_count,
            } => write!(
                f,
                "rpm header file[{file_idx}]: dirindex={dir_idx} \
                 out of bounds (dir_count={dir_count})"
            ),
            Self::ArrayLengthMismatch {
                expected,
                dirindexes,
                digests,
            } => write!(
                f,
                "rpm header file array length mismatch: \
                 basenames={expected} dirindexes={dirindexes} digests={digests}"
            ),
        }
    }
}

// ===========================================================================
// Public types
// ===========================================================================

/// Parsed RPM header index entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexEntry {
    pub tag: u32,
    pub data_type: u32,
    pub offset: u32,
    pub count: u32,
}

/// Digest algorithm from RPM's `FILEDIGESTALGO` tag.
///
/// Integer codes are defined in `rpmtypes.h` (pgpHashAlgo):
/// MD5=2, SHA256=8, SHA512=10.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RpmDigestAlgo {
    Md5,
    Sha256,
    Sha512,
    Unknown(u32),
}

impl From<u32> for RpmDigestAlgo {
    fn from(v: u32) -> Self {
        match v {
            2 => Self::Md5,
            8 => Self::Sha256,
            10 => Self::Sha512,
            other => Self::Unknown(other),
        }
    }
}

/// A single file entry reconstructed from RPM header tags.
#[derive(Debug, Clone)]
pub struct RpmFileEntry {
    /// Absolute path as stored in the RPM database.
    pub full_path: String,
    /// Hex-encoded digest string from `FILEDIGESTS`.
    pub digest_hex: String,
    /// Algorithm that produced `digest_hex`.
    pub digest_algo: RpmDigestAlgo,
}

/// Parsed RPM header with package metadata and file list.
#[derive(Debug)]
pub struct RpmHeader {
    /// Package name (`NAME` tag, 1000).
    pub name: Option<String>,
    /// Package version string (`VERSION` tag, 1001).
    pub version: Option<String>,
    /// Package release string (`RELEASE` tag, 1002).
    /// Not used in the current pipeline but retained for completeness and
    /// future use (e.g., full `name-version-release` NVR display).
    #[allow(dead_code)]
    pub release: Option<String>,
    /// Reconstructed file list with digests.
    pub files: Vec<RpmFileEntry>,
}

// ===========================================================================
// Checked store_len helper
// ===========================================================================

/// Convert `store.len()` to `u32`, saturating rather than truncating.
///
/// Real RPM headers are bounded to ≤ 16 MiB, well within `u32::MAX`.
/// Saturating is safe here because the value is only used in error messages.
#[inline]
fn store_len_u32(store: &[u8]) -> u32 {
    u32::try_from(store.len()).unwrap_or(u32::MAX)
}

// ===========================================================================
// Path A — nom index-entry parser
// ===========================================================================

/// Parse one 16-byte index entry using `nom`.
fn parse_index_entry_nom(input: &[u8]) -> IResult<&[u8], IndexEntry> {
    let (input, tag) = be_u32(input)?;
    let (input, data_type) = be_u32(input)?;
    let (input, offset) = be_u32(input)?;
    let (input, cnt) = be_u32(input)?;
    Ok((
        input,
        IndexEntry {
            tag,
            data_type,
            offset,
            count: cnt,
        },
    ))
}

/// Parse all index entries using `nom`.
fn parse_index_nom(
    input: &[u8],
    nindex: u32,
) -> Result<Vec<IndexEntry>, RpmHeaderError> {
    let n = nindex as usize;
    match count(parse_index_entry_nom, n)(input) {
        Ok((_, entries)) => Ok(entries),
        Err(_) => Err(RpmHeaderError::TooShort),
    }
}

// ===========================================================================
// Path B — manual byte-slicing index-entry parser
// ===========================================================================

/// Parse all index entries via manual `u32::from_be_bytes` extraction.
fn parse_index_manual(
    blob: &[u8],
    nindex: u32,
) -> Result<Vec<IndexEntry>, RpmHeaderError> {
    let n = nindex as usize;
    // Each entry is 16 bytes; entries start at byte 8 (after the 8-byte prefix).
    let required =
        n.checked_mul(16).ok_or(RpmHeaderError::InvalidIndexCount(nindex))?;
    let index_region =
        blob.get(8..8 + required).ok_or(RpmHeaderError::TooShort)?;

    let mut entries = Vec::with_capacity(n);
    for i in 0..n {
        let base = i
            .checked_mul(16)
            .ok_or(RpmHeaderError::InvalidIndexCount(nindex))?;
        let slice = index_region
            .get(base..base + 16)
            .ok_or(RpmHeaderError::TooShort)?;

        let tag = u32::from_be_bytes(
            slice
                .get(0..4)
                .ok_or(RpmHeaderError::TooShort)?
                .try_into()
                .unwrap_or([0u8; 4]),
        );
        let data_type = u32::from_be_bytes(
            slice
                .get(4..8)
                .ok_or(RpmHeaderError::TooShort)?
                .try_into()
                .unwrap_or([0u8; 4]),
        );
        let offset = u32::from_be_bytes(
            slice
                .get(8..12)
                .ok_or(RpmHeaderError::TooShort)?
                .try_into()
                .unwrap_or([0u8; 4]),
        );
        let cnt = u32::from_be_bytes(
            slice
                .get(12..16)
                .ok_or(RpmHeaderError::TooShort)?
                .try_into()
                .unwrap_or([0u8; 4]),
        );
        entries.push(IndexEntry {
            tag,
            data_type,
            offset,
            count: cnt,
        });
    }
    Ok(entries)
}

// ===========================================================================
// TPI agreement check
// ===========================================================================

/// Compare Path A and Path B results element-by-element.
/// Fails closed on any disagreement.
///
/// NIST SP 800-53 SI-7 — two independent parse paths; fail closed on divergence.
fn tpi_agree(a: &[IndexEntry], b: &[IndexEntry]) -> Result<(), RpmHeaderError> {
    if a.len() != b.len() {
        return Err(RpmHeaderError::TpiDisagreement);
    }
    for (ea, eb) in a.iter().zip(b.iter()) {
        if ea != eb {
            return Err(RpmHeaderError::TpiDisagreement);
        }
    }
    Ok(())
}

// ===========================================================================
// Store extraction helpers
// ===========================================================================

/// Read a single nul-terminated string from `store` at the given `offset`.
///
/// Returns `Err` if `offset` is out of bounds or no nul terminator is found
/// before the end of the store.
fn read_string(
    store: &[u8],
    offset: u32,
    tag: u32,
) -> Result<String, RpmHeaderError> {
    let start = offset as usize;
    if start >= store.len() {
        return Err(RpmHeaderError::OffsetOutOfBounds {
            tag,
            offset,
            store_len: store_len_u32(store),
        });
    }
    let slice = store.get(start..).ok_or_else(|| {
        RpmHeaderError::OffsetOutOfBounds {
            tag,
            offset,
            store_len: store_len_u32(store),
        }
    })?;
    let nul_pos = slice.iter().position(|&b| b == 0).ok_or(
        RpmHeaderError::MissingNulTerminator {
            tag,
        },
    )?;
    let bytes =
        slice.get(..nul_pos).ok_or(RpmHeaderError::MissingNulTerminator {
            tag,
        })?;
    std::str::from_utf8(bytes).map(str::to_owned).map_err(|_| {
        RpmHeaderError::Utf8Error {
            tag,
        }
    })
}

/// Read `count` nul-terminated strings from `store` starting at `offset`.
///
/// Strings are packed consecutively (nul-separated). Returns `Err` on any
/// bounds or encoding violation.
fn read_string_array(
    store: &[u8],
    offset: u32,
    count: u32,
    tag: u32,
) -> Result<Vec<String>, RpmHeaderError> {
    let mut results = Vec::with_capacity(count as usize);
    let mut cursor = offset as usize;

    for _ in 0..count {
        if cursor >= store.len() {
            return Err(RpmHeaderError::OffsetOutOfBounds {
                tag,
                offset: u32::try_from(cursor).unwrap_or(u32::MAX),
                store_len: store_len_u32(store),
            });
        }
        let cursor_u32 = u32::try_from(cursor).unwrap_or(u32::MAX);
        let slice = store.get(cursor..).ok_or_else(|| {
            RpmHeaderError::OffsetOutOfBounds {
                tag,
                offset: cursor_u32,
                store_len: store_len_u32(store),
            }
        })?;
        let nul_pos = slice.iter().position(|&b| b == 0).ok_or(
            RpmHeaderError::MissingNulTerminator {
                tag,
            },
        )?;
        let bytes = slice.get(..nul_pos).ok_or(
            RpmHeaderError::MissingNulTerminator {
                tag,
            },
        )?;
        let s = std::str::from_utf8(bytes).map_err(|_| {
            RpmHeaderError::Utf8Error {
                tag,
            }
        })?;
        results.push(s.to_owned());
        // Advance past the string and its nul terminator.
        cursor = cursor
            .checked_add(nul_pos)
            .and_then(|c| c.checked_add(1))
            .ok_or_else(|| RpmHeaderError::OffsetOutOfBounds {
                tag,
                offset: cursor_u32,
                store_len: store_len_u32(store),
            })?;
    }
    Ok(results)
}

/// Read `count` u32 big-endian values from `store` at `offset`.
///
/// NIST SP 800-218 SSDF PW.4.1 — all arithmetic uses `checked_*`.
fn read_int32_array(
    store: &[u8],
    offset: u32,
    count: u32,
    tag: u32,
) -> Result<Vec<u32>, RpmHeaderError> {
    let sl = store_len_u32(store);
    let start = offset as usize;
    let byte_len = (count as usize).checked_mul(4).ok_or(
        RpmHeaderError::OffsetOutOfBounds {
            tag,
            offset,
            store_len: sl,
        },
    )?;
    let end = start.checked_add(byte_len).ok_or(
        RpmHeaderError::OffsetOutOfBounds {
            tag,
            offset,
            store_len: sl,
        },
    )?;
    let slice =
        store.get(start..end).ok_or(RpmHeaderError::OffsetOutOfBounds {
            tag,
            offset,
            store_len: sl,
        })?;

    let mut results = Vec::with_capacity(count as usize);
    for i in 0..count as usize {
        let b = i.checked_mul(4).ok_or(RpmHeaderError::OffsetOutOfBounds {
            tag,
            offset,
            store_len: sl,
        })?;
        let end_b =
            b.checked_add(4).ok_or(RpmHeaderError::OffsetOutOfBounds {
                tag,
                offset,
                store_len: sl,
            })?;
        let word =
            slice.get(b..end_b).ok_or(RpmHeaderError::OffsetOutOfBounds {
                tag,
                offset,
                store_len: sl,
            })?;
        let arr: [u8; 4] =
            word.try_into().map_err(|_| RpmHeaderError::OffsetOutOfBounds {
                tag,
                offset,
                store_len: sl,
            })?;
        results.push(u32::from_be_bytes(arr));
    }
    Ok(results)
}

/// Read a single u32 big-endian value from `store` at `offset`.
fn read_int32(
    store: &[u8],
    offset: u32,
    tag: u32,
) -> Result<u32, RpmHeaderError> {
    let sl = store_len_u32(store);
    let start = offset as usize;
    let end =
        start.checked_add(4).ok_or(RpmHeaderError::OffsetOutOfBounds {
            tag,
            offset,
            store_len: sl,
        })?;
    let slice =
        store.get(start..end).ok_or(RpmHeaderError::OffsetOutOfBounds {
            tag,
            offset,
            store_len: sl,
        })?;
    let arr: [u8; 4] =
        slice.try_into().map_err(|_| RpmHeaderError::OffsetOutOfBounds {
            tag,
            offset,
            store_len: sl,
        })?;
    Ok(u32::from_be_bytes(arr))
}

// ===========================================================================
// Public entry point
// ===========================================================================

/// Parse an RPM header blob from an SQLite `Packages` table row.
///
/// The blob must begin with `[nindex: u32 BE][hsize: u32 BE]` with NO
/// leading 8-byte magic prefix (that prefix appears only in `.rpm` files).
///
/// ## High-Assurance Properties
///
/// - **Bounded**: rejects blobs > 16 MiB (`MAX_BLOB_BYTES`).
/// - **TPI**: parses index entries via two independent paths (`nom` and manual
///   byte-slicing); fails closed if they disagree.
/// - **Secure arithmetic**: every offset/size calculation uses `checked_*`.
/// - **Bounds-safe indexing**: all slice access via `.get()`.
/// - **Fail-closed**: any parse error returns `Err` — no partial results.
///
/// ## Compliance
///
/// - NIST SP 800-53 SI-7 — TPI prevents single-parser bugs from propagating.
/// - NIST SP 800-53 SI-10 — all inputs are validated before use.
/// - NIST SP 800-53 CM-8 — enables package ownership mapping.
/// - NIST SP 800-218 SSDF PW.4.1 — secure arithmetic throughout.
/// - NSA RTB — fail-closed on any disagreement or out-of-bounds condition.
pub fn parse_rpm_header(blob: &[u8]) -> Result<RpmHeader, RpmHeaderError> {
    #[cfg(debug_assertions)]
    let t0 = std::time::Instant::now();

    // ── Bounds check ─────────────────────────────────────────────────────────
    if blob.len() > MAX_BLOB_BYTES {
        return Err(RpmHeaderError::TooLarge(blob.len()));
    }
    if blob.len() < 8 {
        return Err(RpmHeaderError::TooShort);
    }

    // ── Read header prefix ────────────────────────────────────────────────────
    let nindex_bytes: [u8; 4] = blob
        .get(0..4)
        .ok_or(RpmHeaderError::TooShort)?
        .try_into()
        .map_err(|_| RpmHeaderError::TooShort)?;
    let hsize_bytes: [u8; 4] = blob
        .get(4..8)
        .ok_or(RpmHeaderError::TooShort)?
        .try_into()
        .map_err(|_| RpmHeaderError::TooShort)?;
    let nindex = u32::from_be_bytes(nindex_bytes);
    let hsize = u32::from_be_bytes(hsize_bytes);

    if nindex > MAX_INDEX_ENTRIES {
        return Err(RpmHeaderError::InvalidIndexCount(nindex));
    }

    // ── Validate total size ───────────────────────────────────────────────────
    let index_bytes = (nindex as usize)
        .checked_mul(16)
        .ok_or(RpmHeaderError::InvalidIndexCount(nindex))?;
    let header_end =
        8usize.checked_add(index_bytes).ok_or(RpmHeaderError::TooShort)?;
    let store_end = header_end
        .checked_add(hsize as usize)
        .ok_or(RpmHeaderError::TooShort)?;
    if blob.len() < store_end {
        return Err(RpmHeaderError::TooShort);
    }

    // ── Path A: nom ───────────────────────────────────────────────────────────
    let index_region =
        blob.get(8..header_end).ok_or(RpmHeaderError::TooShort)?;
    let entries_a = parse_index_nom(index_region, nindex)?;

    // ── Path B: manual ────────────────────────────────────────────────────────
    let entries_b = parse_index_manual(blob, nindex)?;

    // ── TPI agreement ─────────────────────────────────────────────────────────
    tpi_agree(&entries_a, &entries_b)?;

    // ── Store region ──────────────────────────────────────────────────────────
    let store =
        blob.get(header_end..store_end).ok_or(RpmHeaderError::TooShort)?;

    // ── Tag extraction ────────────────────────────────────────────────────────
    let result = extract_tags(&entries_a, store)?;

    #[cfg(debug_assertions)]
    log::debug!(
        "TPI rpm_header parse completed in {} µs",
        t0.elapsed().as_micros()
    );

    Ok(result)
}

/// Extract all known tags from the parsed index entries and store region.
fn extract_tags(
    entries: &[IndexEntry],
    store: &[u8],
) -> Result<RpmHeader, RpmHeaderError> {
    let find = |tag: u32| entries.iter().find(|e| e.tag == tag);

    // ── NAME ──────────────────────────────────────────────────────────────────
    let name = if let Some(e) = find(TAG_NAME) {
        if e.data_type == TYPE_STRING {
            Some(read_string(store, e.offset, TAG_NAME)?)
        } else {
            None
        }
    } else {
        None
    };

    // ── VERSION ───────────────────────────────────────────────────────────────
    let version = if let Some(e) = find(TAG_VERSION) {
        if e.data_type == TYPE_STRING {
            Some(read_string(store, e.offset, TAG_VERSION)?)
        } else {
            None
        }
    } else {
        None
    };

    // ── RELEASE ───────────────────────────────────────────────────────────────
    let release = if let Some(e) = find(TAG_RELEASE) {
        if e.data_type == TYPE_STRING {
            Some(read_string(store, e.offset, TAG_RELEASE)?)
        } else {
            None
        }
    } else {
        None
    };

    // ── File list ─────────────────────────────────────────────────────────────
    let files = extract_file_list(entries, store)?;

    Ok(RpmHeader {
        name,
        version,
        release,
        files,
    })
}

/// Reconstruct the file list from BASENAMES, DIRNAMES, DIRINDEXES, FILEDIGESTS,
/// and FILEDIGESTALGO tags.
///
/// All of these tags must be present and consistent for file entries to be
/// produced. If any required tag is absent, returns an empty file list (not an
/// error — the package may have no files, e.g., meta-packages).
fn extract_file_list(
    entries: &[IndexEntry],
    store: &[u8],
) -> Result<Vec<RpmFileEntry>, RpmHeaderError> {
    let find = |tag: u32| entries.iter().find(|e| e.tag == tag);

    // Locate all required file-list tags.
    let basename_entry = match find(TAG_BASENAMES) {
        Some(e) if e.data_type == TYPE_STRING_ARRAY => e,
        _ => return Ok(Vec::new()),
    };
    let dirname_entry = match find(TAG_DIRNAMES) {
        Some(e) if e.data_type == TYPE_STRING_ARRAY => e,
        _ => return Ok(Vec::new()),
    };
    let dirindex_entry = match find(TAG_DIRINDEXES) {
        Some(e) if e.data_type == TYPE_INT32_ARRAY => e,
        _ => return Ok(Vec::new()),
    };
    let filedigest_entry = match find(TAG_FILEDIGESTS) {
        Some(e) if e.data_type == TYPE_STRING_ARRAY => e,
        _ => return Ok(Vec::new()),
    };

    // Determine digest algorithm (default MD5 if tag absent — old RPM DBs).
    let digest_algo: RpmDigestAlgo = if let Some(e) = find(TAG_FILEDIGESTALGO) {
        if e.data_type == TYPE_INT32 || e.data_type == TYPE_INT32_ARRAY {
            let raw = read_int32(store, e.offset, TAG_FILEDIGESTALGO)?;
            RpmDigestAlgo::from(raw)
        } else {
            RpmDigestAlgo::Md5
        }
    } else {
        RpmDigestAlgo::Md5
    };

    // Read the arrays.
    let basenames = read_string_array(
        store,
        basename_entry.offset,
        basename_entry.count,
        TAG_BASENAMES,
    )?;
    let dirnames = read_string_array(
        store,
        dirname_entry.offset,
        dirname_entry.count,
        TAG_DIRNAMES,
    )?;
    let dirindexes = read_int32_array(
        store,
        dirindex_entry.offset,
        dirindex_entry.count,
        TAG_DIRINDEXES,
    )?;
    let filedigests = read_string_array(
        store,
        filedigest_entry.offset,
        filedigest_entry.count,
        TAG_FILEDIGESTS,
    )?;

    // All per-file arrays must have the same count.
    // A mismatch indicates a malformed or tampered header blob — fail closed.
    // NSA RTB fail-closed; NIST SP 800-53 SI-10.
    let file_count = basenames.len();
    if dirindexes.len() != file_count || filedigests.len() != file_count {
        return Err(RpmHeaderError::ArrayLengthMismatch {
            expected: file_count,
            dirindexes: dirindexes.len(),
            digests: filedigests.len(),
        });
    }

    let mut result = Vec::with_capacity(file_count);
    for i in 0..file_count {
        let basename = basenames.get(i).ok_or(RpmHeaderError::TooShort)?;
        let dir_idx = *dirindexes.get(i).ok_or(RpmHeaderError::TooShort)?;
        let digest_hex =
            filedigests.get(i).ok_or(RpmHeaderError::TooShort)?.clone();

        let dirname = dirnames.get(dir_idx as usize).ok_or(
            RpmHeaderError::InvalidDirindex {
                file_idx: i,
                dir_idx,
                dir_count: dirnames.len(),
            },
        )?;

        // Reconstruct the full path. DIRNAMES entries already end with '/'.
        let full_path = format!("{dirname}{basename}");

        result.push(RpmFileEntry {
            full_path,
            digest_hex,
            digest_algo,
        });
    }

    Ok(result)
}
