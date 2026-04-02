//! =============================================================================
//! UMRS SELinux Modeling Library — Category / CategorySet Tests
//! =============================================================================
//!
//! Integration tests covering:
//!   • Category validation
//!   • Parsing / formatting
//!   • CategorySet operations
//!   • Dominance semantics
//!   • Set algebra
//!
//! Run with:
//!   cargo test
//! =============================================================================

use std::str::FromStr;

// Adjust crate path if needed
use umrs_selinux::category::{Category, CategoryError, CategorySet, MAX_CATEGORY};

//
// =============================================================================
// Category Tests
// =============================================================================
//

#[test]
fn category_valid_construction() {
    let c0 = Category::new(0).unwrap();
    let c1023 = Category::new(MAX_CATEGORY).unwrap();

    assert_eq!(c0.id(), 0);
    assert_eq!(c1023.id(), MAX_CATEGORY);
}

#[test]
fn category_out_of_range_fails() {
    let result = Category::new(MAX_CATEGORY + 1);
    assert!(result.is_err());

    match result {
        Err(CategoryError::OutOfRange(id)) => {
            assert_eq!(id, MAX_CATEGORY + 1);
        }
        _ => panic!("Expected OutOfRange error"),
    }
}

#[test]
fn category_display_format() {
    let c = Category::new(40).unwrap();
    assert_eq!(c.to_string(), "c40");
}

#[test]
fn category_parse_valid() {
    let c = Category::from_str("c7").unwrap();
    assert_eq!(c.id(), 7);
}

#[test]
fn category_parse_invalid_prefix() {
    let result = Category::from_str("x7");
    assert!(result.is_err());
}

#[test]
fn category_parse_invalid_number() {
    let result = Category::from_str("cXYZ");
    assert!(result.is_err());
}

#[test]
fn category_round_trip() {
    let original = Category::new(512).unwrap();
    let parsed = Category::from_str(&original.to_string()).unwrap();

    assert_eq!(original, parsed);
}

//
// =============================================================================
// CategorySet Tests
// =============================================================================
//

#[test]
fn categoryset_insert_and_contains() {
    let mut set = CategorySet::new();

    let c0 = Category::new(0).unwrap();
    let c40 = Category::new(40).unwrap();

    set.insert(c0);
    set.insert(c40);

    assert!(set.contains(c0));
    assert!(set.contains(c40));
}

#[test]
fn categoryset_remove() {
    let mut set = CategorySet::new();
    let c = Category::new(7).unwrap();

    set.insert(c);
    assert!(set.contains(c));

    set.remove(c);
    assert!(!set.contains(c));
}

#[test]
fn categoryset_is_empty() {
    let mut set = CategorySet::new();
    assert!(set.is_empty());

    set.insert(Category::new(1).unwrap());
    assert!(!set.is_empty());
}

#[test]
fn categoryset_display_format() {
    let mut set = CategorySet::new();

    set.insert(Category::new(0).unwrap());
    set.insert(Category::new(3).unwrap());
    set.insert(Category::new(40).unwrap());

    assert_eq!(set.to_string(), "c0,c3,c40");
}

#[test]
fn categoryset_parse_valid() {
    let set: CategorySet = "c0,c3,c40".parse().unwrap();

    assert!(set.contains(Category::new(0).unwrap()));
    assert!(set.contains(Category::new(3).unwrap()));
    assert!(set.contains(Category::new(40).unwrap()));
}

#[test]
fn categoryset_round_trip() {
    let original: CategorySet = "c1,c2,c3".parse().unwrap();
    let parsed: CategorySet = original.to_string().parse().unwrap();

    assert_eq!(original, parsed);
}

//
// =============================================================================
// Dominance Tests
// =============================================================================
//

#[test]
fn dominance_superset_true() {
    let a: CategorySet = "c0,c1,c2".parse().unwrap();
    let b: CategorySet = "c0,c1".parse().unwrap();

    assert!(a.dominates(&b));
}

#[test]
fn dominance_equal_true() {
    let a: CategorySet = "c5,c6".parse().unwrap();
    let b: CategorySet = "c5,c6".parse().unwrap();

    assert!(a.dominates(&b));
}

#[test]
fn dominance_subset_false() {
    let a: CategorySet = "c0,c1".parse().unwrap();
    let b: CategorySet = "c0,c1,c2".parse().unwrap();

    assert!(!a.dominates(&b));
}

//
// =============================================================================
// Set Algebra Tests
// =============================================================================
//

#[test]
fn union_operation() {
    let a: CategorySet = "c0,c1".parse().unwrap();
    let b: CategorySet = "c1,c2".parse().unwrap();

    let union = a.union(&b);

    assert!(union.contains(Category::new(0).unwrap()));
    assert!(union.contains(Category::new(1).unwrap()));
    assert!(union.contains(Category::new(2).unwrap()));
}

#[test]
fn intersection_operation() {
    let a: CategorySet = "c0,c1,c2".parse().unwrap();
    let b: CategorySet = "c1,c2,c3".parse().unwrap();

    let intersection = a.intersection(&b);

    assert!(intersection.contains(Category::new(1).unwrap()));
    assert!(intersection.contains(Category::new(2).unwrap()));
    assert!(!intersection.contains(Category::new(0).unwrap()));
    assert!(!intersection.contains(Category::new(3).unwrap()));
}

//
// =============================================================================
// Full Set Tests
// =============================================================================
//

#[test]
fn full_set_dominates_all() {
    let full = CategorySet::full();
    let subset: CategorySet = "c0,c100,c500".parse().unwrap();

    assert!(full.dominates(&subset));
}

#[test]
fn full_set_not_empty() {
    let full = CategorySet::full();
    assert!(!full.is_empty());
}
