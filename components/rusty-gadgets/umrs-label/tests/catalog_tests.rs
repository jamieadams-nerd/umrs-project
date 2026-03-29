// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams
//
// Integration tests for umrs-labels::cui::catalog.
//
// These tests exercise catalog loading, metadata, label/marking lookup,
// field presence predicates, and cross-catalog compatibility across the
// US CUI and Canadian Protected catalogs.

use std::path::PathBuf;
use umrs_labels::cui::catalog;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Returns the absolute path to the cui-labels.json fixture shipped with this
/// crate. Using CARGO_MANIFEST_DIR ensures the path is resolved correctly
/// regardless of where `cargo test` is invoked from.
fn fixture_path() -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(manifest_dir).join("cui-labels.json")
}

fn us_catalog_path() -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(manifest_dir).join("data/us/US-CUI-LABELS.json")
}

fn ca_catalog_path() -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(manifest_dir).join("data/ca/CANADIAN-PROTECTED.json")
}

fn levels_path() -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(manifest_dir).join("data/LEVELS.json")
}

// ---------------------------------------------------------------------------
// Catalog loading — fixture
// ---------------------------------------------------------------------------

#[test]
fn catalog_loads_without_error() {
    let result = catalog::load_catalog(fixture_path());
    assert!(result.is_ok(), "catalog load failed: {:?}", result.err());
}

#[test]
fn catalog_load_bad_path_returns_err() {
    let result = catalog::load_catalog("/nonexistent/path/catalog.json");
    assert!(result.is_err(), "expected error for missing file");
}

// ---------------------------------------------------------------------------
// Metadata — fixture
// ---------------------------------------------------------------------------

#[test]
fn fixture_metadata_is_present() {
    let cat = catalog::load_catalog(fixture_path()).expect("catalog load");
    assert!(cat.metadata.is_some(), "fixture should have _metadata block");
}

#[test]
fn fixture_metadata_country_code_us() {
    let cat = catalog::load_catalog(fixture_path()).expect("catalog load");
    assert_eq!(
        cat.country_code(),
        Some("US"),
        "fixture country_code should be US"
    );
}

#[test]
fn fixture_metadata_version_present() {
    let cat = catalog::load_catalog(fixture_path()).expect("catalog load");
    let meta = cat.metadata.as_ref().expect("metadata should be Some");
    assert!(!meta.version.is_empty(), "metadata version should not be empty");
}

// ---------------------------------------------------------------------------
// Marking lookup — fixture
// ---------------------------------------------------------------------------

#[test]
fn marking_lookup_existing_key() {
    let cat = catalog::load_catalog(fixture_path()).expect("catalog load");
    let marking = cat.marking("CUI//LEI");
    assert!(marking.is_some(), "expected CUI//LEI to exist in catalog");
    let m = marking.expect("marking is Some");
    assert_eq!(m.parent_group, "CUI");
}

#[test]
fn marking_lookup_missing_key_returns_none() {
    let cat = catalog::load_catalog(fixture_path()).expect("catalog load");
    assert!(cat.marking("CUI//DOESNOTEXIST").is_none());
}

#[test]
fn marking_lookup_base_cui() {
    let cat = catalog::load_catalog(fixture_path()).expect("catalog load");
    let m = cat.marking("CUI").expect("CUI base marking should exist");
    assert_eq!(m.abbrv_name, "CUI");
}

// ---------------------------------------------------------------------------
// Marking level field — fixture
// ---------------------------------------------------------------------------

#[test]
fn marking_level_field_present_for_base_cui() {
    let cat = catalog::load_catalog(fixture_path()).expect("catalog load");
    let m = cat.marking("CUI").expect("CUI base marking should exist");
    assert_eq!(m.level.as_deref(), Some("s1"), "CUI should have level s1");
}

#[test]
fn marking_optional_fields_palette_ref_and_risk_domains() {
    let cat = catalog::load_catalog(fixture_path()).expect("catalog load");
    let m = cat.marking("CUI//LEI").expect("CUI//LEI should exist");
    assert_eq!(
        m.palette_ref.as_deref(),
        Some("police_blue"),
        "CUI//LEI should have palette_ref"
    );
    let domains = m.risk_domains.as_ref().expect("CUI//LEI should have risk_domains");
    assert!(!domains.is_empty(), "risk_domains should not be empty");
}

// ---------------------------------------------------------------------------
// Marking iteration — fixture
// ---------------------------------------------------------------------------

#[test]
fn iter_markings_is_nonempty() {
    let cat = catalog::load_catalog(fixture_path()).expect("catalog load");
    let count = cat.iter_markings().count();
    assert!(count > 0, "expected at least one marking");
}

#[test]
fn iter_markings_contains_lei() {
    let cat = catalog::load_catalog(fixture_path()).expect("catalog load");
    let found = cat.iter_markings().any(|(k, _)| k == "CUI//LEI");
    assert!(found, "expected CUI//LEI in iter_markings");
}

#[test]
fn all_markings_returns_same_as_iter_markings() {
    let cat = catalog::load_catalog(fixture_path()).expect("catalog load");
    assert_eq!(
        cat.all_markings().count(),
        cat.iter_markings().count(),
        "all_markings and iter_markings should return the same count"
    );
}

// ---------------------------------------------------------------------------
// Marking children — fixture
// ---------------------------------------------------------------------------

#[test]
fn marking_children_lei_returns_subcategories() {
    let cat = catalog::load_catalog(fixture_path()).expect("catalog load");
    let children: Vec<_> = cat.marking_children("CUI//LEI").collect();
    assert!(
        !children.is_empty(),
        "expected CUI//LEI to have child markings"
    );
}

#[test]
fn marking_children_all_have_correct_parent_group() {
    let cat = catalog::load_catalog(fixture_path()).expect("catalog load");
    for (_, child) in cat.marking_children("CUI//LEI") {
        assert_eq!(
            child.parent_group, "LEI",
            "child parent_group should be 'LEI'"
        );
    }
}

#[test]
fn marking_children_unknown_parent_returns_empty() {
    let cat = catalog::load_catalog(fixture_path()).expect("catalog load");
    let children: Vec<_> = cat.marking_children("CUI//BOGUS").collect();
    assert!(
        children.is_empty(),
        "expected no children for unknown parent"
    );
}

// ---------------------------------------------------------------------------
// Label lookup — fixture
// ---------------------------------------------------------------------------

#[test]
fn label_lookup_cui_exists() {
    let cat = catalog::load_catalog(fixture_path()).expect("catalog load");
    let label = cat.label("CUI");
    assert!(label.is_some(), "expected CUI label to exist");
    let l = label.expect("label is Some");
    assert!(!l.name.is_empty(), "CUI label name should not be empty");
}

#[test]
fn label_lookup_missing_returns_none() {
    let cat = catalog::load_catalog(fixture_path()).expect("catalog load");
    assert!(cat.label("DOES_NOT_EXIST").is_none());
}

#[test]
fn iter_labels_is_nonempty() {
    let cat = catalog::load_catalog(fixture_path()).expect("catalog load");
    let count = cat.iter_labels().count();
    assert!(count > 0, "expected at least one label");
}

// ---------------------------------------------------------------------------
// Marking field presence predicates — fixture
// ---------------------------------------------------------------------------

#[test]
fn has_description_true_for_lei() {
    let cat = catalog::load_catalog(fixture_path()).expect("catalog load");
    let m = cat.marking("CUI//LEI").expect("CUI//LEI should exist");
    assert!(
        m.has_description(),
        "CUI//LEI should have a non-empty description"
    );
}

#[test]
fn has_handling_reflects_field_content() {
    let cat = catalog::load_catalog(fixture_path()).expect("catalog load");
    // Verify the predicate is consistent with what the field contains.
    // handling is now serde_json::Value — use has_handling() which understands
    // both string and object variants.
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

#[test]
fn has_description_reflects_field_content() {
    let cat = catalog::load_catalog(fixture_path()).expect("catalog load");
    for (key, m) in cat.iter_markings() {
        let expected = !m.description.trim().is_empty();
        assert_eq!(
            m.has_description(),
            expected,
            "has_description() mismatch for {key}"
        );
    }
}

#[test]
fn handling_as_str_returns_string_for_string_handling() {
    let cat = catalog::load_catalog(fixture_path()).expect("catalog load");
    let m = cat.marking("CUI//LEI").expect("CUI//LEI should exist");
    // LEI has a string handling field in the fixture.
    assert!(
        m.handling_as_str().is_some(),
        "CUI//LEI handling should be accessible as a string"
    );
    assert!(
        m.handling_as_object().is_none(),
        "CUI//LEI handling should not be an object"
    );
}

// ---------------------------------------------------------------------------
// US catalog — real data
// ---------------------------------------------------------------------------

#[test]
fn us_catalog_loads_without_error() {
    let result = catalog::load_catalog(us_catalog_path());
    assert!(result.is_ok(), "US catalog load failed: {:?}", result.err());
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
fn us_catalog_has_72_or_more_markings() {
    let cat = catalog::load_catalog(us_catalog_path()).expect("US catalog load");
    let count = cat.iter_markings().count();
    assert!(count >= 72, "US catalog should have at least 72 markings, got {count}");
}

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
fn us_catalog_markings_have_string_handling() {
    let cat = catalog::load_catalog(us_catalog_path()).expect("US catalog load");
    // US markings with handling guidance use plain strings.
    for (key, m) in cat.iter_markings() {
        if m.has_handling() {
            assert!(
                m.handling_as_str().is_some(),
                "US marking {key} with handling should be a string"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Canadian catalog — real data
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

#[test]
fn ca_catalog_has_three_labels() {
    let cat = catalog::load_catalog(ca_catalog_path()).expect("CA catalog load");
    let count = cat.iter_labels().count();
    assert_eq!(count, 3, "CA catalog should have exactly 3 Protected labels");
}

#[test]
fn ca_catalog_labels_pa_pb_pc_present() {
    let cat = catalog::load_catalog(ca_catalog_path()).expect("CA catalog load");
    assert!(cat.label("PROTECTED-A").is_some(), "PROTECTED-A should exist");
    assert!(cat.label("PROTECTED-B").is_some(), "PROTECTED-B should exist");
    assert!(cat.label("PROTECTED-C").is_some(), "PROTECTED-C should exist");
}

#[test]
fn ca_catalog_pa_has_level_s1() {
    let cat = catalog::load_catalog(ca_catalog_path()).expect("CA catalog load");
    let pa = cat.label("PROTECTED-A").expect("PROTECTED-A should exist");
    assert_eq!(pa.level.as_deref(), Some("s1"), "Protected A should be s1");
}

#[test]
fn ca_catalog_pb_has_level_s2() {
    let cat = catalog::load_catalog(ca_catalog_path()).expect("CA catalog load");
    let pb = cat.label("PROTECTED-B").expect("PROTECTED-B should exist");
    assert_eq!(pb.level.as_deref(), Some("s2"), "Protected B should be s2");
}

#[test]
fn ca_catalog_pc_has_level_s3() {
    let cat = catalog::load_catalog(ca_catalog_path()).expect("CA catalog load");
    let pc = cat.label("PROTECTED-C").expect("PROTECTED-C should exist");
    assert_eq!(pc.level.as_deref(), Some("s3"), "Protected C should be s3");
}

#[test]
fn ca_catalog_labels_have_bilingual_names() {
    let cat = catalog::load_catalog(ca_catalog_path()).expect("CA catalog load");
    for (key, label) in cat.iter_labels() {
        assert!(
            label.name_fr.is_some(),
            "CA label {key} should have a French name"
        );
    }
}

#[test]
fn ca_catalog_pa_has_abbrv_name() {
    let cat = catalog::load_catalog(ca_catalog_path()).expect("CA catalog load");
    let pa = cat.label("PROTECTED-A").expect("PROTECTED-A should exist");
    assert_eq!(
        pa.abbrv_name.as_deref(),
        Some("PA"),
        "Protected A abbrv_name should be PA"
    );
}

#[test]
fn ca_catalog_labels_handling_is_object() {
    let cat = catalog::load_catalog(ca_catalog_path()).expect("CA catalog load");
    for (key, label) in cat.iter_labels() {
        if label.has_handling() {
            assert!(
                label.handling_as_object().is_some(),
                "CA label {key} handling should be a structured object"
            );
        }
    }
}

#[test]
fn ca_catalog_no_markings_key() {
    // The Canadian catalog uses `labels` for its entries; `markings` should be empty.
    let cat = catalog::load_catalog(ca_catalog_path()).expect("CA catalog load");
    assert!(
        cat.markings.is_empty(),
        "CA catalog should have no markings (uses labels instead)"
    );
}

// ---------------------------------------------------------------------------
// Cross-catalog compatibility
// ---------------------------------------------------------------------------

#[test]
fn us_and_ca_load_with_same_catalog_type() {
    // Demonstrates that a single Catalog type deserializes both nations' files.
    let us = catalog::load_catalog(us_catalog_path()).expect("US catalog load");
    let ca = catalog::load_catalog(ca_catalog_path()).expect("CA catalog load");

    assert_eq!(us.country_code(), Some("US"));
    assert_eq!(ca.country_code(), Some("CA"));
    // US uses markings; CA uses labels.
    assert!(!us.markings.is_empty(), "US should have markings");
    assert!(!ca.labels.is_empty(), "CA should have labels");
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
    assert_eq!(count, 4, "LEVELS.json should define s0-s3 (4 levels), got {count}");
}

#[test]
fn levels_s0_through_s3_all_present() {
    let reg = catalog::load_levels(levels_path()).expect("levels load");
    for key in ["s0", "s1", "s2", "s3"] {
        assert!(reg.level(key).is_some(), "LEVELS.json should contain level {key}");
    }
}

#[test]
fn levels_s1_name_is_nonempty() {
    let reg = catalog::load_levels(levels_path()).expect("levels load");
    let s1 = reg.level("s1").expect("s1 should exist");
    assert!(!s1.name.is_empty(), "s1 name should not be empty");
    assert!(!s1.description.is_empty(), "s1 description should not be empty");
}

#[test]
fn levels_s1_nations_includes_us_cui() {
    let reg = catalog::load_levels(levels_path()).expect("levels load");
    let s1 = reg.level("s1").expect("s1 should exist");
    let nations = s1.nations.as_ref().expect("s1 should have nations");
    assert!(
        nations.iter().any(|n| n.contains("US CUI")),
        "s1 nations should reference US CUI"
    );
}

#[test]
fn levels_iter_yields_four_items() {
    let reg = catalog::load_levels(levels_path()).expect("levels load");
    assert_eq!(reg.iter_levels().count(), 4, "iter_levels should yield 4 items");
}

#[test]
fn levels_load_bad_path_returns_err() {
    let result = catalog::load_levels("/nonexistent/path/LEVELS.json");
    assert!(result.is_err(), "expected error for missing file");
}
