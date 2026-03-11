// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//! Integration tests for the RPM header blob parser.
//!
//! All tests operate on programmatically constructed blobs — no real RPM
//! database or filesystem access is required. Each test validates a specific
//! parse path, error condition, or high-assurance property.
//!
//! ## Compliance
//!
//! - NIST SP 800-53 SI-7 — TPI correctness tests verify both paths agree.
//! - NIST SP 800-53 SI-10 — bounds/overflow tests verify fail-closed behaviour.
//! - NIST SP 800-53 CM-8 — file list reconstruction tests verify inventory accuracy.

use umrs_platform::detect::substrate::rpm_header::{
    MAX_BLOB_BYTES, MAX_INDEX_ENTRIES, RpmDigestAlgo, RpmHeaderError,
    parse_rpm_header,
};

// ===========================================================================
// Blob construction helpers
// ===========================================================================

/// A (tag, type, data) tuple for building test blobs.
#[derive(Debug, Clone)]
enum TagData {
    /// A single nul-terminated string (type=6).
    Str(u32, &'static str),
    /// An array of nul-terminated strings (type=8).
    StrArray(u32, Vec<&'static str>),
    /// An array of u32 big-endian integers (type=4).
    Int32Array(u32, Vec<u32>),
    /// A single u32 big-endian integer (type=3).
    Int32(u32, u32),
}

const TYPE_INT32: u32 = 3;
const TYPE_INT32_ARRAY: u32 = 4;
const TYPE_STRING: u32 = 6;
const TYPE_STRING_ARRAY: u32 = 8;

/// Build a valid RPM header blob from a list of tag descriptors.
///
/// The resulting blob starts with `[nindex: u32 BE][hsize: u32 BE]` and
/// contains valid index entries and store data. This mirrors the format stored
/// in `/var/lib/rpm/rpmdb.sqlite`.
fn build_blob(tags: &[TagData]) -> Vec<u8> {
    // Phase 1: build the store and collect index entries.
    let mut store: Vec<u8> = Vec::new();
    let mut entries: Vec<(u32, u32, u32, u32)> = Vec::new(); // (tag, type, offset, count)

    for tag_data in tags {
        match tag_data {
            TagData::Str(tag, s) => {
                let offset = store.len() as u32;
                store.extend_from_slice(s.as_bytes());
                store.push(0); // nul terminator
                entries.push((*tag, TYPE_STRING, offset, 1));
            }
            TagData::StrArray(tag, strings) => {
                let offset = store.len() as u32;
                let count = strings.len() as u32;
                for s in strings {
                    store.extend_from_slice(s.as_bytes());
                    store.push(0);
                }
                entries.push((*tag, TYPE_STRING_ARRAY, offset, count));
            }
            TagData::Int32Array(tag, values) => {
                // INT32_ARRAY requires 4-byte alignment.
                while !store.len().is_multiple_of(4) {
                    store.push(0);
                }
                let offset = store.len() as u32;
                let count = values.len() as u32;
                for v in values {
                    store.extend_from_slice(&v.to_be_bytes());
                }
                entries.push((*tag, TYPE_INT32_ARRAY, offset, count));
            }
            TagData::Int32(tag, value) => {
                while !store.len().is_multiple_of(4) {
                    store.push(0);
                }
                let offset = store.len() as u32;
                store.extend_from_slice(&value.to_be_bytes());
                entries.push((*tag, TYPE_INT32, offset, 1));
            }
        }
    }

    // Phase 2: write the blob header (nindex, hsize).
    let nindex = entries.len() as u32;
    let hsize = store.len() as u32;
    let mut blob: Vec<u8> = Vec::new();
    blob.extend_from_slice(&nindex.to_be_bytes());
    blob.extend_from_slice(&hsize.to_be_bytes());

    // Phase 3: write index entries (16 bytes each).
    for (tag, data_type, offset, count) in &entries {
        blob.extend_from_slice(&tag.to_be_bytes());
        blob.extend_from_slice(&data_type.to_be_bytes());
        blob.extend_from_slice(&offset.to_be_bytes());
        blob.extend_from_slice(&count.to_be_bytes());
    }

    // Phase 4: write store.
    blob.extend_from_slice(&store);
    blob
}

// ===========================================================================
// Tests
// ===========================================================================

/// Parse a minimal blob with only NAME and VERSION tags.
#[test]
fn parse_minimal_header() {
    let blob =
        build_blob(&[TagData::Str(1000, "bash"), TagData::Str(1001, "5.2.21")]);
    let header = parse_rpm_header(&blob).expect("minimal header should parse");
    assert_eq!(header.name.as_deref(), Some("bash"));
    assert_eq!(header.version.as_deref(), Some("5.2.21"));
    assert!(header.files.is_empty());
}

/// Parse a blob with BASENAMES, DIRNAMES, and DIRINDEXES — verify path reconstruction.
#[test]
fn parse_file_list() {
    // /usr/bin/bash and /usr/lib64/libc.so.6
    let blob = build_blob(&[
        TagData::Str(1000, "bash"),
        TagData::Str(1001, "5.2.21"),
        TagData::StrArray(1117, vec!["bash", "libc.so.6"]), // BASENAMES
        TagData::StrArray(1118, vec!["/usr/bin/", "/usr/lib64/"]), // DIRNAMES
        TagData::Int32Array(1116, vec![0, 1]),              // DIRINDEXES
        TagData::StrArray(1035, vec!["aabbcc", "ddeeff"]),  // FILEDIGESTS
    ]);
    let header = parse_rpm_header(&blob).expect("file list blob should parse");
    assert_eq!(header.files.len(), 2);

    let f0 = &header.files[0];
    assert_eq!(f0.full_path, "/usr/bin/bash");
    assert_eq!(f0.digest_hex, "aabbcc");

    let f1 = &header.files[1];
    assert_eq!(f1.full_path, "/usr/lib64/libc.so.6");
    assert_eq!(f1.digest_hex, "ddeeff");
}

/// Parse a blob with SHA-256 digest algorithm tag.
#[test]
fn parse_file_digests_sha256() {
    let sha256_hex =
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
    let blob = build_blob(&[
        TagData::Str(1000, "testpkg"),
        TagData::Str(1001, "1.0"),
        TagData::StrArray(1117, vec!["test.txt"]),
        TagData::StrArray(1118, vec!["/etc/"]),
        TagData::Int32Array(1116, vec![0]),
        TagData::StrArray(1035, vec![sha256_hex]),
        TagData::Int32(5011, 8), // FILEDIGESTALGO = 8 (SHA256)
    ]);
    let header =
        parse_rpm_header(&blob).expect("sha256 digest blob should parse");
    assert_eq!(header.files.len(), 1);
    assert_eq!(header.files[0].digest_hex, sha256_hex);
    assert_eq!(header.files[0].digest_algo, RpmDigestAlgo::Sha256);
}

/// Parse a blob with SHA-512 digest algorithm tag.
#[test]
fn parse_file_digests_sha512() {
    let blob = build_blob(&[
        TagData::Str(1000, "pkg"),
        TagData::Str(1001, "2.0"),
        TagData::StrArray(1117, vec!["file"]),
        TagData::StrArray(1118, vec!["/opt/"]),
        TagData::Int32Array(1116, vec![0]),
        TagData::StrArray(1035, vec!["aabbccdd"]),
        TagData::Int32(5011, 10), // FILEDIGESTALGO = 10 (SHA512)
    ]);
    let header =
        parse_rpm_header(&blob).expect("sha512 digest blob should parse");
    assert_eq!(header.files[0].digest_algo, RpmDigestAlgo::Sha512);
}

/// MD5 digest algorithm (tag absent — defaults to MD5).
#[test]
fn parse_file_digests_md5_default() {
    let blob = build_blob(&[
        TagData::Str(1000, "pkg"),
        TagData::Str(1001, "1.0"),
        TagData::StrArray(1117, vec!["file"]),
        TagData::StrArray(1118, vec!["/opt/"]),
        TagData::Int32Array(1116, vec![0]),
        TagData::StrArray(1035, vec!["deadbeef"]),
        // No FILEDIGESTALGO tag — should default to MD5
    ]);
    let header =
        parse_rpm_header(&blob).expect("md5 default blob should parse");
    assert_eq!(header.files[0].digest_algo, RpmDigestAlgo::Md5);
}

/// Empty blob → TooShort error.
#[test]
fn parse_empty_blob() {
    let result = parse_rpm_header(&[]);
    assert!(matches!(result, Err(RpmHeaderError::TooShort)));
}

/// Blob shorter than declared nindex → TooShort error.
#[test]
fn parse_truncated_index() {
    // Declare nindex=5 but only provide 1 index entry worth of data.
    let mut blob = vec![];
    blob.extend_from_slice(&5u32.to_be_bytes()); // nindex=5
    blob.extend_from_slice(&0u32.to_be_bytes()); // hsize=0
    // Only write 16 bytes (1 entry) instead of 80 (5 entries).
    blob.extend_from_slice(&[0u8; 16]);
    let result = parse_rpm_header(&blob);
    assert!(matches!(result, Err(RpmHeaderError::TooShort)));
}

/// Absurdly large nindex → InvalidIndexCount error.
#[test]
fn parse_overflow_nindex() {
    let mut blob = vec![];
    blob.extend_from_slice(&(MAX_INDEX_ENTRIES + 1).to_be_bytes());
    blob.extend_from_slice(&0u32.to_be_bytes());
    // Don't need to add index data — the count check fires first.
    let result = parse_rpm_header(&blob);
    assert!(matches!(result, Err(RpmHeaderError::InvalidIndexCount(_))));
}

/// Blob larger than MAX_BLOB_BYTES → TooLarge error.
#[test]
fn parse_too_large_blob() {
    let oversized = vec![0u8; MAX_BLOB_BYTES + 1];
    let result = parse_rpm_header(&oversized);
    assert!(matches!(result, Err(RpmHeaderError::TooLarge(_))));
}

/// An offset that points past the end of the store → OffsetOutOfBounds.
#[test]
fn parse_misaligned_offset() {
    // Build a valid blob, then corrupt the offset of the first index entry
    // to point 1 byte past the store end.
    let base = build_blob(&[TagData::Str(1000, "bash")]);
    let hsize = u32::from_be_bytes(base[4..8].try_into().unwrap());
    // Corrupt offset field (bytes 10..14 of the first index entry, which
    // starts at byte 8 + 8 = offset 8 in the blob, field 2 = bytes 8..24):
    // index entry layout: [tag:4][type:4][offset:4][count:4], starting at byte 8.
    let mut corrupt = base.clone();
    let bad_offset = (hsize + 100).to_be_bytes();
    corrupt[16..20].copy_from_slice(&bad_offset);
    let result = parse_rpm_header(&corrupt);
    // Either TpiDisagreement (paths agree on the corrupt value but store is OOB)
    // or OffsetOutOfBounds when string extraction runs.
    assert!(
        matches!(result, Err(RpmHeaderError::OffsetOutOfBounds { .. }))
            || matches!(
                result,
                Err(RpmHeaderError::MissingNulTerminator { .. })
            )
    );
}

/// TPI agreement: both parse paths must produce identical index entries.
#[test]
fn tpi_agreement() {
    // Build a multi-tag blob and verify it parses successfully (agreement).
    let blob = build_blob(&[
        TagData::Str(1000, "mypkg"),
        TagData::Str(1001, "1.2.3"),
        TagData::Str(1002, "1.el10"),
        TagData::StrArray(1117, vec!["file1", "file2"]),
        TagData::StrArray(1118, vec!["/opt/mypkg/"]),
        TagData::Int32Array(1116, vec![0, 0]),
        TagData::StrArray(1035, vec!["aabb", "ccdd"]),
    ]);
    let result = parse_rpm_header(&blob);
    assert!(result.is_ok(), "TPI should agree on a valid blob");
}

/// TPI disagreement: inject a one-bit corruption in the middle of the index
/// region — the two paths must both see the same corruption but the test
/// indirectly validates fail-closed: a structurally invalid blob (offset OOB)
/// must not succeed.
///
/// True TPI disagreement would require corrupting *after* Path A runs but
/// *before* Path B runs, which is not possible with an immutable slice. Instead
/// we verify that Path A results are used for extraction and that corrupt data
/// → a parse error.
#[test]
fn tpi_corruption_causes_error() {
    let blob =
        build_blob(&[TagData::Str(1000, "pkg"), TagData::Str(1001, "1.0")]);
    // Corrupt the offset field of the first index entry (bytes 16..20)
    // to a value that points past the end of the store.
    let hsize = u32::from_be_bytes(blob[4..8].try_into().unwrap());
    let mut corrupt = blob.clone();
    let bad = (hsize + 1000).to_be_bytes();
    corrupt[16..20].copy_from_slice(&bad);
    // Both paths will parse the same corrupt offset — extraction will fail.
    let result = parse_rpm_header(&corrupt);
    assert!(
        result.is_err(),
        "corrupt offset must not parse successfully"
    );
}

/// Digest algorithm code mapping: 8→SHA256, 10→SHA512, 2→MD5, unknown→Unknown.
#[test]
fn digest_algo_mapping() {
    assert_eq!(RpmDigestAlgo::from(8u32), RpmDigestAlgo::Sha256);
    assert_eq!(RpmDigestAlgo::from(10u32), RpmDigestAlgo::Sha512);
    assert_eq!(RpmDigestAlgo::from(2u32), RpmDigestAlgo::Md5);
    assert_eq!(RpmDigestAlgo::from(99u32), RpmDigestAlgo::Unknown(99));
}

/// A meta-package blob with no file tags — must parse cleanly with empty files list.
#[test]
fn parse_meta_package_no_files() {
    let blob =
        build_blob(&[TagData::Str(1000, "meta"), TagData::Str(1001, "0.1")]);
    let header = parse_rpm_header(&blob).expect("meta-package should parse");
    assert_eq!(header.name.as_deref(), Some("meta"));
    assert!(header.files.is_empty());
}

/// RELEASE tag is parsed correctly.
#[test]
fn parse_release_tag() {
    let blob = build_blob(&[
        TagData::Str(1000, "glibc"),
        TagData::Str(1001, "2.38"),
        TagData::Str(1002, "35.el10"),
    ]);
    let header =
        parse_rpm_header(&blob).expect("release tag blob should parse");
    assert_eq!(header.release.as_deref(), Some("35.el10"));
}

/// An index entry with an offset pointing exactly at the last byte of the
/// store (leaving no room for even a nul terminator) → MissingNulTerminator.
#[test]
fn parse_offset_at_store_end_no_nul() {
    // Build store with exactly 1 byte ("x"), then set offset to 0 but
    // the string "x" has no nul terminator.
    let mut blob: Vec<u8> = Vec::new();
    blob.extend_from_slice(&1u32.to_be_bytes()); // nindex=1
    blob.extend_from_slice(&1u32.to_be_bytes()); // hsize=1 (1 byte store)
    // Index entry: TAG_NAME=1000, TYPE_STRING=6, offset=0, count=1
    blob.extend_from_slice(&1000u32.to_be_bytes());
    blob.extend_from_slice(&6u32.to_be_bytes());
    blob.extend_from_slice(&0u32.to_be_bytes());
    blob.extend_from_slice(&1u32.to_be_bytes());
    // Store: just 'x' with no nul terminator.
    blob.push(b'x');
    let result = parse_rpm_header(&blob);
    assert!(matches!(
        result,
        Err(RpmHeaderError::MissingNulTerminator {
            tag: 1000
        })
    ));
}
