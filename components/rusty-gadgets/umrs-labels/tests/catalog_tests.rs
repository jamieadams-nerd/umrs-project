// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams
//
// Integration tests for umrs-labels::cui::catalog.
//
// These tests load the bundled cui-labels.json fixture via CARGO_MANIFEST_DIR
// and exercise the Catalog API.

use std::path::PathBuf;
use umrs_labels::cui::catalog;

/// Returns the absolute path to the cui-labels.json fixture shipped with this
/// crate. Using CARGO_MANIFEST_DIR ensures the path is resolved correctly
/// regardless of where `cargo test` is invoked from.
fn fixture_path() -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(manifest_dir).join("cui-labels.json")
}

// ---------------------------------------------------------------------------
// Catalog loading
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
// Marking lookup
// ---------------------------------------------------------------------------

#[test]
fn marking_lookup_existing_key() {
    let cat = catalog::load_catalog(fixture_path()).expect("catalog load");
    let marking = cat.marking("CUI//LEI");
    assert!(marking.is_some(), "expected CUI//LEI to exist in catalog");
    let m = marking.unwrap();
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
// Marking iteration
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

// ---------------------------------------------------------------------------
// Marking children
// ---------------------------------------------------------------------------

#[test]
fn marking_children_lei_returns_subcategories() {
    let cat = catalog::load_catalog(fixture_path()).expect("catalog load");
    // CUI//LEI should have at least one child (e.g., CUI//LEI/AIV)
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
// Label lookup
// ---------------------------------------------------------------------------

#[test]
fn label_lookup_cui_exists() {
    let cat = catalog::load_catalog(fixture_path()).expect("catalog load");
    let label = cat.label("CUI");
    assert!(label.is_some(), "expected CUI label to exist");
    let l = label.unwrap();
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
// Marking field presence predicates
// ---------------------------------------------------------------------------

#[test]
fn has_description_true_for_lei() {
    let cat = catalog::load_catalog(fixture_path()).expect("catalog load");
    let m = cat.marking("CUI//LEI").expect("CUI//LEI should exist");
    // LEI is a well-populated marking; it should have a description.
    assert!(
        m.has_description(),
        "CUI//LEI should have a non-empty description"
    );
}

#[test]
fn has_handling_reflects_field_content() {
    let cat = catalog::load_catalog(fixture_path()).expect("catalog load");
    // Iterate all markings and verify the predicate is consistent with the field.
    for (key, m) in cat.iter_markings() {
        let expected = !m.handling.trim().is_empty();
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
