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
    PathBuf::from(manifest_dir).join("config/us/US-CUI-LABELS.json")
}

fn ca_catalog_path() -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(manifest_dir).join("config/ca/CANADIAN-PROTECTED.json")
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
