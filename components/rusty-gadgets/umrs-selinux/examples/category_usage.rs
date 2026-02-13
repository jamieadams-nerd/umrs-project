//! =============================================================================
//! UMRS SELinux Modeling Library — Category / CategorySet Usage Examples
//! =============================================================================
//!
//! Demonstrates:
//!   • Category construction + validation
//!   • Parsing from strings
//!   • Error handling
//!   • CategorySet operations
//!   • Dominance checks
//!   • Union / intersection
//!   • Iteration + display formatting
//!
//! Run with:
//!   cargo run --example category_usage
//! =============================================================================

use std::str::FromStr;

// Adjust this path to match your crate layout
use umrs_selinux::category::{Category, CategorySet};

fn main() {
    println!("=== Category Construction ===");

    // -------------------------------------------------------------------------
    // Valid construction
    // -------------------------------------------------------------------------
    let c0 = Category::new(0).unwrap();
    let c40 = Category::new(40).unwrap();

    println!("Created categories: {}, {}", c0, c40);

    // -------------------------------------------------------------------------
    // Validation failure
    // -------------------------------------------------------------------------
    let invalid = Category::new(5000);

    match invalid {
        Ok(_) => println!("Unexpected success"),
        Err(e) => println!("Validation correctly failed: {:?}", e),
    }

    // -------------------------------------------------------------------------
    // Parsing from string
    // -------------------------------------------------------------------------
    let parsed = Category::from_str("c7").unwrap();
    println!("Parsed category: {}", parsed);

    let bad_parse = Category::from_str("x7");

    match bad_parse {
        Ok(_) => println!("Unexpected parse success"),
        Err(e) => println!("Parse correctly failed: {:?}", e),
    }

    println!("\n=== CategorySet Operations ===");

    // -------------------------------------------------------------------------
    // Create empty set
    // -------------------------------------------------------------------------
    let mut set_a = CategorySet::new();

    set_a.insert(c0);
    set_a.insert(c40);
    set_a.insert(parsed);

    println!("Set A: {}", set_a);

    // -------------------------------------------------------------------------
    // Remove category
    // -------------------------------------------------------------------------
    set_a.remove(parsed);
    println!("Set A after removal: {}", set_a);

    // -------------------------------------------------------------------------
    // Membership test
    // -------------------------------------------------------------------------
    println!("Contains c0? {}", set_a.contains(c0));
    println!("Contains c7? {}", set_a.contains(parsed));

    println!("\n=== Parsing CategorySet ===");

    let set_b: CategorySet = "c0,c3,c40".parse().unwrap();
    println!("Set B: {}", set_b);

    println!("\n=== Dominance Checks ===");

    // A dominates B?
    println!("A dominates B? {}", set_a.dominates(&set_b));
    println!("B dominates A? {}", set_b.dominates(&set_a));

    println!("\n=== Union / Intersection ===");

    let union = set_a.union(&set_b);
    let intersection = set_a.intersection(&set_b);

    println!("Union: {}", union);
    println!("Intersection: {}", intersection);

    println!("\n=== Iteration ===");

    for cat in union.iter() {
        println!(" - {}", cat);
    }

    println!("\n=== Full Set Example ===");

    let full = CategorySet::full();

    println!(
        "Full set dominates A? {}",
        full.dominates(&set_a)
    );

    println!("\n=== is_empty() Check ===");

    let empty = CategorySet::new();
    println!("Empty set? {}", empty.is_empty());
}
