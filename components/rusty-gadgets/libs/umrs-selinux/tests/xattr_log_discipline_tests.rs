// -----------------------------------------------------------------------------
// UMRS SELinux — SI-12 Log Discipline Tests for xattrs.rs
//
// NIST SP 800-53 SI-12: Information Management and Retention
//
// These tests verify that the nom_error_kind() helper — which is used to
// sanitize log output from TPI Path A parse failures — never includes raw
// input bytes in its output. If it did, MLS sensitivity levels and category
// identifiers (e.g., "s3:c100,c200") could appear in error logs, violating
// SI-12.
// -----------------------------------------------------------------------------

use umrs_selinux::xattrs::nom_error_kind;

// A representative set of MLS-sensitive input strings that must never appear
// in log output derived from a nom parse failure on that input.
const SENSITIVE_INPUTS: &[&str] = &[
    "s3:c100,c200",
    "s15:c0.c1023",
    "secret_password",
    "system_u:object_r:NetworkManager_etc_t:s0:c55,c99",
    "user_u:user_r:user_t:s3:c1,c7,c9",
];

// ---------------------------------------------------------------------------
// nom_error_kind — Tag error kind
// ---------------------------------------------------------------------------

/// A nom Tag failure is the most common parse error in context parsing.
/// Verify the returned string is only the kind name and never the input.
#[test]
fn nom_error_kind_tag_excludes_input() {
    for input in SENSITIVE_INPUTS {
        let nom_err = nom::Err::Failure(nom::error::Error::new(*input, nom::error::ErrorKind::Tag));
        let kind_str = nom_error_kind(&nom_err);
        assert!(
            !kind_str.contains(*input),
            "nom_error_kind output for Tag must not contain raw input; \
             got: {kind_str:?}"
        );
        // The output should be only the kind name — a short static string.
        assert_eq!(
            kind_str, "Tag",
            "nom_error_kind for Tag error should return \"Tag\""
        );
    }
}

// ---------------------------------------------------------------------------
// nom_error_kind — TakeUntil error kind
// ---------------------------------------------------------------------------

#[test]
fn nom_error_kind_take_until_excludes_input() {
    for input in SENSITIVE_INPUTS {
        let nom_err = nom::Err::Error(nom::error::Error::new(
            *input,
            nom::error::ErrorKind::TakeUntil,
        ));
        let kind_str = nom_error_kind(&nom_err);
        assert!(
            !kind_str.contains(*input),
            "nom_error_kind output for TakeUntil must not contain raw input; \
             got: {kind_str:?}"
        );
    }
}

// ---------------------------------------------------------------------------
// nom_error_kind — Incomplete variant
// ---------------------------------------------------------------------------

#[test]
fn nom_error_kind_incomplete_is_safe() {
    let nom_err: nom::Err<nom::error::Error<&str>> = nom::Err::Incomplete(nom::Needed::Unknown);
    let kind_str = nom_error_kind(&nom_err);
    assert_eq!(kind_str, "Incomplete");
}

// ---------------------------------------------------------------------------
// nom_error_kind — output is never longer than a kind name
// ---------------------------------------------------------------------------

/// Sanity check: the output of nom_error_kind must be a short string
/// (a variant name). Any output over 64 chars would suggest raw input
/// is leaking through.
#[test]
fn nom_error_kind_output_is_concise() {
    for input in SENSITIVE_INPUTS {
        let nom_err = nom::Err::Failure(nom::error::Error::new(*input, nom::error::ErrorKind::Tag));
        let kind_str = nom_error_kind(&nom_err);
        assert!(
            kind_str.len() <= 64,
            "nom_error_kind output is suspiciously long ({} chars); \
             raw input may be leaking: {kind_str:?}",
            kind_str.len()
        );
    }
}

// ---------------------------------------------------------------------------
// ContextParseError — Display is safe for logging
// ---------------------------------------------------------------------------
//
// Path B uses log::warn!("TPI Path B (FromStr) parse failure: {e}") where
// `e` is a ContextParseError. These tests confirm that every variant's
// Display output contains no raw user-supplied data.

use umrs_selinux::context::ContextParseError;

#[test]
fn context_parse_error_display_excludes_raw_input() {
    let variants = [
        ContextParseError::InvalidFormat,
        ContextParseError::InvalidUser,
        ContextParseError::InvalidRole,
        ContextParseError::InvalidType,
        ContextParseError::InvalidLevel,
    ];

    for variant in &variants {
        let display = variant.to_string();
        for input in SENSITIVE_INPUTS {
            assert!(
                !display.contains(*input),
                "ContextParseError::{variant:?} Display must not contain \
                 raw input; got: {display:?}"
            );
        }
        // Each message should be a short, descriptive static phrase.
        assert!(
            display.len() <= 128,
            "ContextParseError Display is unexpectedly long ({} chars): \
             {display:?}",
            display.len()
        );
    }
}
