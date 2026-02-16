// tests/test_mcs_translator.rs
//
// Integration tests for umrs_selinux::mcs::translator
//
// Adjust SETRANS_PATH if your fixture moves.
//

use std::str::FromStr;
use std::sync::OnceLock;

use umrs_selinux::mcs::translator::{
    load_setrans_file,
    SecurityRange,
    GLOBAL_TRANSLATOR,
};

/// Update this if you relocate the fixture.
const SETRANS_PATH: &str = "setrans.conf";

/// Ensure the setrans file loads exactly once for the test binary.
static INIT: OnceLock<()> = OnceLock::new();

fn ensure_loaded() {
    INIT.get_or_init(|| {
        load_setrans_file(SETRANS_PATH)
            .expect("Failed to load test setrans.conf");
    });
}

//
// -----------------------------------------------------------------------------
// Forward Lookup Tests
// -----------------------------------------------------------------------------

#[test]
fn forward_lookup_returns_non_missing() {
    ensure_loaded();

    let guard = GLOBAL_TRANSLATOR
        .read()
        .expect("GLOBAL_TRANSLATOR lock poisoned");

    let raw = "s0:c90,c91"; // adjust if needed
    let range = SecurityRange::from_str(raw)
        .expect("Failed to parse SecurityRange");

    let marking = guard
        .lookup(&range)
        .unwrap_or_else(|| "MISSING".to_string());

    assert_ne!(
        marking,
        "MISSING",
        "Forward lookup returned MISSING for range {}",
        raw
    );
}

#[test]
fn forward_lookup_detail_accessible() {
    ensure_loaded();

    let guard = GLOBAL_TRANSLATOR
        .read()
        .expect("GLOBAL_TRANSLATOR lock poisoned");

    let raw = "s0:c90,c91";
    let range = SecurityRange::from_str(raw)
        .expect("Failed to parse SecurityRange");

    // Detail may legitimately be empty — we just verify API path works.
    let _detail = guard.get_detail(&range);
}

//
// -----------------------------------------------------------------------------
// Reverse Lookup Tests
// -----------------------------------------------------------------------------

#[test]
fn reverse_lookup_known_marking_returns_ranges() {
    ensure_loaded();

    let guard = GLOBAL_TRANSLATOR
        .read()
        .expect("GLOBAL_TRANSLATOR lock poisoned");

    let marking = "CUI//PRIVACY/CONTRACT"; // adjust if needed

    let results = guard.lookup_by_marking(marking);

    assert!(
        !results.is_empty(),
        "Reverse lookup returned no ranges for marking [{}]",
        marking
    );

    // Ensure reverse results parse back into SecurityRange
    for (kernel_str, _detail) in results {
        SecurityRange::from_str(&kernel_str)
            .unwrap_or_else(|e| {
                panic!(
                    "Reverse lookup returned unparsable range '{}': {}",
                    kernel_str, e
                )
            });
    }
}

#[test]
fn reverse_lookup_unknown_marking_returns_empty() {
    ensure_loaded();

    let guard = GLOBAL_TRANSLATOR
        .read()
        .expect("GLOBAL_TRANSLATOR lock poisoned");

    let marking = "UMRS//THIS/SHOULD/NOT/EXIST";

    let results = guard.lookup_by_marking(marking);

    assert!(
        results.is_empty(),
        "Expected empty result for unknown marking [{}], got {:?}",
        marking,
        results
    );
}

//
// -----------------------------------------------------------------------------
// Normalization / Determinism Tests
// -----------------------------------------------------------------------------

#[test]
fn category_order_normalization_should_not_change_lookup() {
    ensure_loaded();

    let guard = GLOBAL_TRANSLATOR
        .read()
        .expect("GLOBAL_TRANSLATOR lock poisoned");

    let a = "s0:c90,c91";
    let b = "s0:c91,c90";

    let ra = SecurityRange::from_str(a)
        .expect("Failed to parse first range");
    let rb = SecurityRange::from_str(b)
        .expect("Failed to parse second range");

    let la = guard.lookup(&ra)
        .unwrap_or_else(|| "MISSING".to_string());
    let lb = guard.lookup(&rb)
        .unwrap_or_else(|| "MISSING".to_string());

    assert_ne!(la, "MISSING", "Lookup failed for {}", a);
    assert_ne!(lb, "MISSING", "Lookup failed for {}", b);

    assert_eq!(
        la, lb,
        "Normalization mismatch: {} -> '{}' but {} -> '{}'",
        a, la, b, lb
    );
}
