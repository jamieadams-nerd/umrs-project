// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
use std::str::FromStr;

use umrs_selinux::category::{Category, CategorySet};
use umrs_selinux::mls::level::MlsLevel;
use umrs_selinux::sensitivity::SensitivityLevel;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // -------------------------------------------------------------------------
    // Example 1 — Parse sensitivity-only level
    // -------------------------------------------------------------------------

    let level = MlsLevel::from_str("s0")?;

    println!("Level: {}", level);
    println!("Sensitivity: {}", level.sensitivity());
    println!("Has categories: {}\n", level.has_categories());

    // -------------------------------------------------------------------------
    // Example 2 — Parse with categories
    // -------------------------------------------------------------------------

    let level = MlsLevel::from_str("s2:c1,c7,c42")?;

    println!("Level: {}", level);
    println!("Sensitivity: {}", level.sensitivity());
    println!("Categories: {}", level.categories());
    println!("Has categories: {}\n", level.has_categories());

    // -------------------------------------------------------------------------
    // Example 3 — Construct programmatically
    // -------------------------------------------------------------------------

    let sensitivity = SensitivityLevel::from_str("s3")?;

    let mut categories = CategorySet::default();
    categories.insert(Category::from_str("c0")?);
    categories.insert(Category::from_str("c5")?);
    categories.insert(Category::from_str("c9")?);

    let level = MlsLevel::new(sensitivity, categories);

    println!("Constructed Level: {}", level);
    println!("Sensitivity: {}", level.sensitivity());
    println!("Categories: {}\n", level.categories());

    // -------------------------------------------------------------------------
    // Example 4 — Equality comparison
    // -------------------------------------------------------------------------

    let a = MlsLevel::from_str("s1:c0,c1")?;
    let b = MlsLevel::from_str("s1:c0,c1")?;
    let c = MlsLevel::from_str("s1:c0,c2")?;

    println!("A == B → {}", a == b);
    println!("A == C → {}\n", a == c);

    // -------------------------------------------------------------------------
    // Example 5 — Display round-trip
    // -------------------------------------------------------------------------

    let input = "s4:c3,c8,c12";
    let parsed = MlsLevel::from_str(input)?;

    println!("Original: {}", input);
    println!("Round-trip: {}", parsed);

    Ok(())
}
