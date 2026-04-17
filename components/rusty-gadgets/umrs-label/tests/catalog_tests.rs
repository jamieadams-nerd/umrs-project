// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams
//
// Integration tests for umrs-labels::cui::catalog.
//
// These tests exercise catalog loading, metadata, marking lookup,
// field presence predicates, and cross-catalog compatibility across the
// US CUI and Canadian Protected catalogs.

use std::path::PathBuf;
use umrs_labels::cui::catalog;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn us_catalog_path() -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(manifest_dir).join("config/US-CUI-LABELS.json")
}

fn ca_catalog_path() -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(manifest_dir).join("config/CANADIAN-PROTECTED.json")
}

fn levels_path() -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(manifest_dir).join("config/LEVELS.json")
}

// ---------------------------------------------------------------------------
// Catalog loading — error paths
// ---------------------------------------------------------------------------

#[test]
fn catalog_load_bad_path_returns_err() {
    let result = catalog::load_catalog("/nonexistent/path/catalog.json");
    assert!(result.is_err(), "expected error for missing file");
}

// ---------------------------------------------------------------------------
// US catalog — loading and metadata
// ---------------------------------------------------------------------------

#[test]
fn us_catalog_loads_without_error() {
    let result = catalog::load_catalog(us_catalog_path());
    assert!(result.is_ok(), "US catalog load failed: {:?}", result.err());
}

#[test]
fn us_catalog_metadata_is_present() {
    let cat = catalog::load_catalog(us_catalog_path()).expect("US catalog load");
    assert!(
        cat.metadata.is_some(),
        "US catalog should have _metadata block"
    );
}

#[test]
fn us_catalog_metadata_country_code() {
    let cat = catalog::load_catalog(us_catalog_path()).expect("US catalog load");
    assert_eq!(
        cat.country_code(),
        Some("US"),
        "US catalog country_code should be US"
    );
}

#[test]
fn us_catalog_metadata_version_present() {
    let cat = catalog::load_catalog(us_catalog_path()).expect("US catalog load");
    let meta = cat.metadata.as_ref().expect("metadata should be Some");
    assert!(
        !meta.version.is_empty(),
        "metadata version should not be empty"
    );
}

// ---------------------------------------------------------------------------
// US catalog — marking counts and lookup
// ---------------------------------------------------------------------------

#[test]
fn us_catalog_has_72_or_more_markings() {
    let cat = catalog::load_catalog(us_catalog_path()).expect("US catalog load");
    let count = cat.iter_markings().count();
    assert!(
        count >= 72,
        "US catalog should have at least 72 markings, got {count}"
    );
}

#[test]
fn us_catalog_marking_lookup_existing_key() {
    let cat = catalog::load_catalog(us_catalog_path()).expect("US catalog load");
    let marking = cat.marking("CUI//LEI");
    assert!(marking.is_some(), "expected CUI//LEI to exist in catalog");
}

#[test]
fn us_catalog_marking_lookup_missing_key_returns_none() {
    let cat = catalog::load_catalog(us_catalog_path()).expect("US catalog load");
    assert!(cat.marking("CUI//DOESNOTEXIST").is_none());
}

#[test]
fn us_catalog_marking_lookup_base_cui() {
    let cat = catalog::load_catalog(us_catalog_path()).expect("US catalog load");
    let m = cat.marking("CUI").expect("CUI base marking should exist");
    assert_eq!(m.abbrv_name, "CUI");
}

// ---------------------------------------------------------------------------
// US catalog — marking field values
// ---------------------------------------------------------------------------

#[test]
fn us_catalog_all_markings_have_level_s1() {
    let cat = catalog::load_catalog(us_catalog_path()).expect("US catalog load");
    for (key, m) in cat.iter_markings() {
        assert_eq!(
            m.level.as_deref(),
            Some("s1"),
            "US marking {key} should have level s1"
        );
    }
}

#[test]
fn us_catalog_markings_have_string_handling_or_none() {
    let cat = catalog::load_catalog(us_catalog_path()).expect("US catalog load");
    for (key, m) in cat.iter_markings() {
        if m.has_handling() {
            assert!(
                m.handling_as_str().is_some(),
                "US marking {key} with handling should be a plain string"
            );
        }
    }
}

#[test]
fn us_catalog_designation_values_are_basic_or_specified() {
    let cat = catalog::load_catalog(us_catalog_path()).expect("US catalog load");
    for (key, m) in cat.iter_markings() {
        if let Some(d) = &m.designation {
            assert!(
                d == "basic" || d == "specified",
                "US marking {key} designation '{d}' is not 'basic' or 'specified'"
            );
        }
    }
}

#[test]
fn us_catalog_index_group_is_optional() {
    // At least one US marking has a null index_group; None is a valid value.
    let cat = catalog::load_catalog(us_catalog_path()).expect("US catalog load");
    let null_count = cat.iter_markings().filter(|(_, m)| m.index_group.is_none()).count();
    assert!(
        null_count > 0,
        "expected at least one US marking with a null index_group"
    );
}

#[test]
fn us_catalog_handling_group_id_is_optional_string() {
    // US entries have a handling_group_id string; it should deserialize as Some.
    let cat = catalog::load_catalog(us_catalog_path()).expect("US catalog load");
    let has_hgid = cat
        .iter_markings()
        .any(|(_, m)| m.handling_group_id.as_deref().is_some_and(|s| !s.is_empty()));
    assert!(
        has_hgid,
        "at least one US marking should have a non-empty handling_group_id"
    );
}

// ---------------------------------------------------------------------------
// US catalog — iteration
// ---------------------------------------------------------------------------

#[test]
fn us_iter_markings_is_nonempty() {
    let cat = catalog::load_catalog(us_catalog_path()).expect("US catalog load");
    assert!(
        cat.iter_markings().count() > 0,
        "expected at least one marking"
    );
}

#[test]
fn us_iter_markings_contains_lei() {
    let cat = catalog::load_catalog(us_catalog_path()).expect("US catalog load");
    let found = cat.iter_markings().any(|(k, _)| k == "CUI//LEI");
    assert!(found, "expected CUI//LEI in iter_markings");
}

#[test]
fn us_all_markings_returns_same_as_iter_markings() {
    let cat = catalog::load_catalog(us_catalog_path()).expect("US catalog load");
    assert_eq!(
        cat.all_markings().count(),
        cat.iter_markings().count(),
        "all_markings and iter_markings should return the same count"
    );
}

// ---------------------------------------------------------------------------
// US catalog — field presence predicates
// ---------------------------------------------------------------------------

#[test]
fn us_has_description_reflects_field_content() {
    let cat = catalog::load_catalog(us_catalog_path()).expect("US catalog load");
    for (key, m) in cat.iter_markings() {
        // has_description() delegates to LocaleText::has_content() which checks
        // the trimmed English value.
        let expected = !m.description.en().trim().is_empty();
        assert_eq!(
            m.has_description(),
            expected,
            "has_description() mismatch for {key}"
        );
    }
}

#[test]
fn us_has_handling_reflects_field_content() {
    let cat = catalog::load_catalog(us_catalog_path()).expect("US catalog load");
    for (key, m) in cat.iter_markings() {
        let expected = match &m.handling {
            serde_json::Value::String(s) => !s.trim().is_empty(),
            serde_json::Value::Object(o) => !o.is_empty(),
            serde_json::Value::Null => false,
            _ => true,
        };
        assert_eq!(
            m.has_handling(),
            expected,
            "has_handling() mismatch for {key}"
        );
    }
}

// ---------------------------------------------------------------------------
// Canadian catalog — loading and metadata
// ---------------------------------------------------------------------------

#[test]
fn ca_catalog_loads_without_error() {
    let result = catalog::load_catalog(ca_catalog_path());
    assert!(result.is_ok(), "CA catalog load failed: {:?}", result.err());
}

#[test]
fn ca_catalog_metadata_country_code() {
    let cat = catalog::load_catalog(ca_catalog_path()).expect("CA catalog load");
    assert_eq!(
        cat.country_code(),
        Some("CA"),
        "CA catalog country_code should be CA"
    );
}

// ---------------------------------------------------------------------------
// Canadian catalog — marking counts and lookup
// ---------------------------------------------------------------------------

#[test]
fn ca_catalog_has_three_markings() {
    let cat = catalog::load_catalog(ca_catalog_path()).expect("CA catalog load");
    let count = cat.iter_markings().count();
    assert_eq!(
        count, 3,
        "CA catalog should have exactly 3 Protected markings"
    );
}

#[test]
fn ca_catalog_pa_pb_pc_present() {
    let cat = catalog::load_catalog(ca_catalog_path()).expect("CA catalog load");
    assert!(
        cat.marking("PROTECTED-A").is_some(),
        "PROTECTED-A should exist"
    );
    assert!(
        cat.marking("PROTECTED-B").is_some(),
        "PROTECTED-B should exist"
    );
    assert!(
        cat.marking("PROTECTED-C").is_some(),
        "PROTECTED-C should exist"
    );
}

// ---------------------------------------------------------------------------
// Canadian catalog — marking field values
// ---------------------------------------------------------------------------

#[test]
fn ca_catalog_pa_has_level_s1() {
    let cat = catalog::load_catalog(ca_catalog_path()).expect("CA catalog load");
    let pa = cat.marking("PROTECTED-A").expect("PROTECTED-A should exist");
    assert_eq!(pa.level.as_deref(), Some("s1"), "Protected A should be s1");
}

#[test]
fn ca_catalog_pb_has_level_s2() {
    let cat = catalog::load_catalog(ca_catalog_path()).expect("CA catalog load");
    let pb = cat.marking("PROTECTED-B").expect("PROTECTED-B should exist");
    assert_eq!(pb.level.as_deref(), Some("s2"), "Protected B should be s2");
}

#[test]
fn ca_catalog_pc_has_level_s3() {
    let cat = catalog::load_catalog(ca_catalog_path()).expect("CA catalog load");
    let pc = cat.marking("PROTECTED-C").expect("PROTECTED-C should exist");
    assert_eq!(pc.level.as_deref(), Some("s3"), "Protected C should be s3");
}

#[test]
fn ca_catalog_markings_have_bilingual_names() {
    let cat = catalog::load_catalog(ca_catalog_path()).expect("CA catalog load");
    for (key, marking) in cat.iter_markings() {
        assert!(
            !marking.name.fr().is_empty(),
            "CA marking {key} should have a non-empty French name"
        );
    }
}

#[test]
fn ca_catalog_pa_has_abbrv_name() {
    let cat = catalog::load_catalog(ca_catalog_path()).expect("CA catalog load");
    let pa = cat.marking("PROTECTED-A").expect("PROTECTED-A should exist");
    assert_eq!(
        pa.abbrv_name.as_str(),
        "PA",
        "Protected A abbrv_name should be PA"
    );
}

#[test]
fn ca_catalog_handling_group_id_is_null() {
    // Canadian entries have null handling_group_id.
    let cat = catalog::load_catalog(ca_catalog_path()).expect("CA catalog load");
    for (key, m) in cat.iter_markings() {
        assert!(
            m.handling_group_id.is_none(),
            "CA marking {key} should have null handling_group_id"
        );
    }
}

#[test]
fn ca_catalog_markings_handling_is_object() {
    let cat = catalog::load_catalog(ca_catalog_path()).expect("CA catalog load");
    for (key, marking) in cat.iter_markings() {
        if marking.has_handling() {
            assert!(
                marking.handling_as_object().is_some(),
                "CA marking {key} handling should be a structured object"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Cross-catalog compatibility
// ---------------------------------------------------------------------------

#[test]
fn us_and_ca_load_with_same_catalog_type() {
    let us = catalog::load_catalog(us_catalog_path()).expect("US catalog load");
    let ca = catalog::load_catalog(ca_catalog_path()).expect("CA catalog load");

    assert_eq!(us.country_code(), Some("US"));
    assert_eq!(ca.country_code(), Some("CA"));
    // Both catalogs use the markings key in the unified schema.
    assert!(!us.markings.is_empty(), "US should have markings");
    assert!(!ca.markings.is_empty(), "CA should have markings");
}

// ---------------------------------------------------------------------------
// LEVELS.json
// ---------------------------------------------------------------------------

#[test]
fn levels_loads_without_error() {
    let result = catalog::load_levels(levels_path());
    assert!(result.is_ok(), "levels load failed: {:?}", result.err());
}

#[test]
fn levels_has_four_entries() {
    let reg = catalog::load_levels(levels_path()).expect("levels load");
    let count = reg.levels.len();
    assert_eq!(
        count, 4,
        "LEVELS.json should define s0-s3 (4 levels), got {count}"
    );
}

#[test]
fn levels_s0_through_s3_all_present() {
    let reg = catalog::load_levels(levels_path()).expect("levels load");
    for key in ["s0", "s1", "s2", "s3"] {
        assert!(
            reg.level(key).is_some(),
            "LEVELS.json should contain level {key}"
        );
    }
}

#[test]
fn levels_s1_name_is_nonempty() {
    let reg = catalog::load_levels(levels_path()).expect("levels load");
    let s1 = reg.level("s1").expect("s1 should exist");
    assert!(!s1.name.is_empty(), "s1 name should not be empty");
    assert!(
        !s1.description.is_empty(),
        "s1 description should not be empty"
    );
}

#[test]
fn levels_s1_nations_includes_us_cui() {
    let reg = catalog::load_levels(levels_path()).expect("levels load");
    let s1 = reg.level("s1").expect("s1 should exist");
    let nations = s1.nations.as_ref().expect("s1 should have nations");
    assert!(
        nations.iter().any(|n: &String| n.contains("US CUI")),
        "s1 nations should reference US CUI"
    );
}

#[test]
fn levels_iter_yields_four_items() {
    let reg = catalog::load_levels(levels_path()).expect("levels load");
    assert_eq!(
        reg.iter_levels().count(),
        4,
        "iter_levels should yield 4 items"
    );
}

#[test]
fn levels_load_bad_path_returns_err() {
    let result = catalog::load_levels("/nonexistent/path/LEVELS.json");
    assert!(result.is_err(), "expected error for missing file");
}

// ---------------------------------------------------------------------------
// country_flag — standalone function
// ---------------------------------------------------------------------------

#[test]
fn country_flag_us() {
    assert_eq!(catalog::country_flag("US"), Some("🇺🇸".to_string()));
}

#[test]
fn country_flag_ca() {
    assert_eq!(catalog::country_flag("CA"), Some("🇨🇦".to_string()));
}

#[test]
fn country_flag_gb() {
    assert_eq!(catalog::country_flag("GB"), Some("🇬🇧".to_string()));
}

#[test]
fn country_flag_au() {
    assert_eq!(catalog::country_flag("AU"), Some("🇦🇺".to_string()));
}

#[test]
fn country_flag_nz() {
    assert_eq!(catalog::country_flag("NZ"), Some("🇳🇿".to_string()));
}

#[test]
fn country_flag_lowercase() {
    assert_eq!(catalog::country_flag("us"), Some("🇺🇸".to_string()));
}

#[test]
fn country_flag_mixed_case() {
    assert_eq!(catalog::country_flag("Ca"), Some("🇨🇦".to_string()));
}

#[test]
fn country_flag_invalid_length() {
    assert_eq!(catalog::country_flag("USA"), None);
}

#[test]
fn country_flag_single_char() {
    assert_eq!(catalog::country_flag("U"), None);
}

#[test]
fn country_flag_empty() {
    assert_eq!(catalog::country_flag(""), None);
}

#[test]
fn country_flag_digits() {
    assert_eq!(catalog::country_flag("12"), None);
}

#[test]
fn country_flag_special() {
    assert_eq!(catalog::country_flag("U!"), None);
}

// ---------------------------------------------------------------------------
// country_flag — Catalog convenience method
// ---------------------------------------------------------------------------

#[test]
fn catalog_country_flag_us() {
    let cat = catalog::load_catalog(us_catalog_path()).expect("US catalog load");
    assert_eq!(cat.country_flag(), Some("🇺🇸".to_string()));
}

#[test]
fn catalog_country_flag_ca() {
    let cat = catalog::load_catalog(ca_catalog_path()).expect("CA catalog load");
    assert_eq!(cat.country_flag(), Some("🇨🇦".to_string()));
}

// ---------------------------------------------------------------------------
// marking_by_mcs_level — Canadian Protected fallback lookup
//
// These tests cover the regression path where setrans.conf has no translation
// for Canadian MCS categories (c300-c302), so the group header carries the
// raw MCS level string rather than a human-readable marking key.
// ---------------------------------------------------------------------------

/// # TEST-ID: CAT-MCS-001
/// # REQUIREMENT: marking_by_mcs_level resolves raw "s1:c300" to PROTECTED-A
/// # COMPLIANCE: NIST SP 800-53 AC-16
#[test]
fn ca_catalog_mcs_level_s1_c300_resolves_protected_a() {
    let cat = catalog::load_catalog(ca_catalog_path()).expect("CA catalog load");
    let result = cat.marking_by_mcs_level("s1:c300");
    assert!(
        result.is_some(),
        "s1:c300 should resolve to a Canadian marking"
    );
    let (key, marking) = result.unwrap();
    assert_eq!(key, "PROTECTED-A", "s1:c300 should map to PROTECTED-A");
    assert_eq!(marking.abbrv_name.as_str(), "PA");
}

/// # TEST-ID: CAT-MCS-002
/// # REQUIREMENT: marking_by_mcs_level resolves raw "s2:c301" to PROTECTED-B
/// # COMPLIANCE: NIST SP 800-53 AC-16
#[test]
fn ca_catalog_mcs_level_s2_c301_resolves_protected_b() {
    let cat = catalog::load_catalog(ca_catalog_path()).expect("CA catalog load");
    let result = cat.marking_by_mcs_level("s2:c301");
    assert!(
        result.is_some(),
        "s2:c301 should resolve to a Canadian marking"
    );
    let (key, _) = result.unwrap();
    assert_eq!(key, "PROTECTED-B", "s2:c301 should map to PROTECTED-B");
}

/// # TEST-ID: CAT-MCS-003
/// # REQUIREMENT: marking_by_mcs_level resolves raw "s3:c302" to PROTECTED-C
/// # COMPLIANCE: NIST SP 800-53 AC-16
#[test]
fn ca_catalog_mcs_level_s3_c302_resolves_protected_c() {
    let cat = catalog::load_catalog(ca_catalog_path()).expect("CA catalog load");
    let result = cat.marking_by_mcs_level("s3:c302");
    assert!(
        result.is_some(),
        "s3:c302 should resolve to a Canadian marking"
    );
    let (key, _) = result.unwrap();
    assert_eq!(key, "PROTECTED-C", "s3:c302 should map to PROTECTED-C");
}

/// # TEST-ID: CAT-MCS-004
/// # REQUIREMENT: marking_by_mcs_level returns None for raw level with no match
/// # COMPLIANCE: NIST SP 800-53 AC-16
#[test]
fn ca_catalog_mcs_level_no_match_returns_none() {
    let cat = catalog::load_catalog(ca_catalog_path()).expect("CA catalog load");
    // s0:c0 is a US CUI base level, not in the Canadian catalog.
    let result = cat.marking_by_mcs_level("s0:c0");
    assert!(
        result.is_none(),
        "s0:c0 should not match any Canadian marking"
    );
}

/// # TEST-ID: CAT-MCS-005
/// # REQUIREMENT: marking_by_mcs_level returns None for plain sensitivity with no category
/// # COMPLIANCE: NIST SP 800-53 AC-16
#[test]
fn ca_catalog_mcs_level_no_category_returns_none() {
    let cat = catalog::load_catalog(ca_catalog_path()).expect("CA catalog load");
    // Plain sensitivity-only level has no category to match against category_base.
    let result = cat.marking_by_mcs_level("s1");
    assert!(
        result.is_none(),
        "plain sensitivity-only level s1 should not match"
    );
}

/// # TEST-ID: CAT-MCS-006
/// # REQUIREMENT: marking_by_mcs_level returns None on empty input
/// # COMPLIANCE: NIST SP 800-53 AC-16
#[test]
fn ca_catalog_mcs_level_empty_returns_none() {
    let cat = catalog::load_catalog(ca_catalog_path()).expect("CA catalog load");
    let result = cat.marking_by_mcs_level("");
    assert!(result.is_none(), "empty string should return None");
}

/// # TEST-ID: CAT-MCS-007
/// # REQUIREMENT: US catalog does not return Canadian entries via mcs_level fallback
/// # COMPLIANCE: NIST SP 800-53 AC-16
#[test]
fn us_catalog_mcs_level_canadian_range_returns_none() {
    // Canadian categories (c300+) are not in the US catalog.
    let cat = catalog::load_catalog(us_catalog_path()).expect("US catalog load");
    let result = cat.marking_by_mcs_level("s1:c300");
    assert!(
        result.is_none(),
        "US catalog should not resolve s1:c300 (Canadian range)"
    );
}

/// # TEST-ID: CAT-MCS-008
/// # REQUIREMENT: marking_by_mcs_level accepts level with multiple categories
/// # COMPLIANCE: NIST SP 800-53 AC-16
#[test]
fn ca_catalog_mcs_level_compound_category_uses_first() {
    let cat = catalog::load_catalog(ca_catalog_path()).expect("CA catalog load");
    // A compound level like "s1:c300,c303" — c300 is the base for Protected A.
    let result = cat.marking_by_mcs_level("s1:c300,c303");
    assert!(
        result.is_some(),
        "compound level s1:c300,c303 should resolve via first category c300"
    );
    let (key, _) = result.unwrap();
    assert_eq!(key, "PROTECTED-A");
}

// ---------------------------------------------------------------------------
// marking_by_banner — setrans-translated string lookup
// ---------------------------------------------------------------------------

/// # TEST-ID: CAT-BANNER-001
/// # REQUIREMENT: marking_by_banner resolves Canadian French translated banner text
/// # COMPLIANCE: NIST SP 800-53 AC-16
///
/// `setrans.conf` translates `s0:c300` to `"PROTÉGÉ A"`. This test verifies that
/// `marking_by_banner` can resolve the translated French form back to the
/// `PROTECTED-A` catalog entry, enabling the popup to display correctly for
/// files labeled with Canadian Protected MCS categories.
#[test]
fn marking_by_banner_finds_canadian_french() {
    let cat = catalog::load_catalog(ca_catalog_path()).expect("CA catalog load");
    let result = cat.marking_by_banner("PROTÉGÉ A");
    assert!(
        result.is_some(),
        "expected PROTÉGÉ A to resolve to PROTECTED-A via banner lookup"
    );
    let (key, _) = result.unwrap();
    assert_eq!(key, "PROTECTED-A");
}

/// # TEST-ID: CAT-BANNER-002
/// # REQUIREMENT: marking_by_banner resolves English banner text for Canadian entries
/// # COMPLIANCE: NIST SP 800-53 AC-16
///
/// Verifies that the English form `"PROTECTED A"` stored in `marking_banner.en_US`
/// also resolves correctly. Both locale variants must work because English-locale
/// setups may use the English form.
#[test]
fn marking_by_banner_finds_canadian_english() {
    let cat = catalog::load_catalog(ca_catalog_path()).expect("CA catalog load");
    let result = cat.marking_by_banner("PROTECTED A");
    assert!(
        result.is_some(),
        "expected PROTECTED A to resolve to PROTECTED-A via banner lookup"
    );
    let (key, _) = result.unwrap();
    assert_eq!(key, "PROTECTED-A");
}

/// # TEST-ID: CAT-BANNER-003
/// # REQUIREMENT: marking_by_banner performs case-insensitive ASCII comparison
/// # COMPLIANCE: NIST SP 800-53 AC-16, NIST SP 800-53 SI-10
///
/// The comparison uses `eq_ignore_ascii_case` — ASCII letter case differences
/// must not prevent a match. Accented characters (e.g., `É` U+00C9 vs `é`
/// U+00E9) are distinct Unicode scalar values; `eq_ignore_ascii_case` does not
/// perform Unicode case folding on them. This test exercises the ASCII folding
/// path using a string whose non-accented letters differ from the stored value
/// (`"PROTÉGÉ A"`) while keeping the accented characters identical.
#[test]
fn marking_by_banner_case_insensitive() {
    let cat = catalog::load_catalog(ca_catalog_path()).expect("CA catalog load");
    // "PROTÉGÉ a" shares the same accented É characters as "PROTÉGÉ A" in the
    // catalog, but the trailing letter 'a' differs in ASCII case. This exercises
    // eq_ignore_ascii_case folding without requiring Unicode case folding.
    let result = cat.marking_by_banner("PROTÉGÉ a");
    assert!(
        result.is_some(),
        "case-insensitive banner lookup should match 'PROTÉGÉ a' to PROTECTED-A"
    );
    let (key, _) = result.unwrap();
    assert_eq!(key, "PROTECTED-A");
}

/// # TEST-ID: CAT-BANNER-004
/// # REQUIREMENT: marking_by_banner returns None for unknown banner text
/// # COMPLIANCE: NIST SP 800-53 AC-16, NIST SP 800-53 SI-10
///
/// Fail-closed: an unrecognised banner string must not match any entry.
/// This guards against inadvertent label resolution for garbage input.
#[test]
fn marking_by_banner_returns_none_for_unknown() {
    let cat = catalog::load_catalog(ca_catalog_path()).expect("CA catalog load");
    let result = cat.marking_by_banner("BOGUS");
    assert!(result.is_none(), "BOGUS banner text should return None");
}
