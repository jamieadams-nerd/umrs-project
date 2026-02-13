//!
//! SELinux Catagories, Category Sets, and dominance semantics
//!
//! - Author: Jamie Adams
//! - License: MIT
//!
//! Strongly-typed Rust primitives modeling SELinux MLS constructs,
//! including categories, category sets, and dominance semantics.
//!
//! ## Primitive Modeled: MLS Category Bitmap
//! This module provides a strongly-typed Rust equivalent of the
//! kernel ebitmap structure used to represent MLS category sets.
//!
//! Kernel Sources Consulted:
//! - security/selinux/ss/ebitmap.c
//! - security/selinux/ss/ebitmap.h
//! - security/selinux/ss/mls.c
//!
//! Design Deviations:
//! - Dense bitmap instead of sparse linked nodes
//! - Fixed 1024-bit width
//! - Construct-time validation
//!
//! These deviations are intentional and reflect userland performance,
//! determinism, and safety priorities rather than kernel memory
//! optimization constraints.
//!
//! ## Implementation Lineage & Design Note
//! This module provides an independent, original implementation of
//! functionality conceptually comparable to traditional SELinux
//! userland libraries.
//!
//! Behavioral interfaces and operational semantics were studied
//! to ensure familiarity for long-time SELinux developers.
//! However:
//!
//! - No SELinux source code has been copied.
//! - No code has been translated.
//! - No line-by-line reimplementation has been performed.
//!
//! Where appropriate, this implementation takes advantage of
//! Rust language features such as strong typing, validation at
//! construction, and memory safety guarantees to improve
//! correctness and assurance beyond legacy approaches.
//!

use std::fmt;
use std::str::FromStr;

///
/// Category Primitive
///
/// Category represents a single SELinux MLS category (c0–c1023).
///
/// Categories are represented as bit positions within an ebitmap.
/// This Rust type provides a strongly-typed, validated wrapper around
/// that primitive representation.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Category(u16);

pub const MAX_CATEGORY: u16 = 1023;

impl Category {
    /// Creates a new validated SELinux MLS category.
    ///
    /// Categories represent compartment identifiers within the SELinux
    /// Multi-Level Security (MLS) model. They are encoded as bit positions
    /// within an MLS category bitmap (ebitmap equivalent).
    ///
    /// Valid category identifiers range from `c0` through `c1023`,
    /// matching the standard SELinux category domain.
    ///
    /// # Errors
    ///
    /// Returns `CategoryError::OutOfRange` if the provided category
    /// identifier exceeds the maximum supported value (`MAX_CATEGORY`).
    ///
    /// # Examples
    ///
    /// ```
    /// let c0 = Category::new(0).unwrap();
    /// let c40 = Category::new(40).unwrap();
    /// println!("Created categories: {}, {}", c0, c40);
    ///
    /// let invalid = Category::new(5000);
    /// match invalid {
    ///    Ok(_) => println!("Unexpected success"),
    ///    Err(e) => println!("Validation correctly failed: {:?}", e),
    /// }
    /// ```
    ///
    /// Parsing from a string will also self-validate provided string. 
    /// ```
    /// let parsed = Category::from_str("c7").unwrap();
    /// println!("Parsed category: {}", parsed);
    /// ```
    ///
    pub const fn new(id: u16) -> Result<Self, CategoryError> {
        if id > MAX_CATEGORY {
            return Err(CategoryError::OutOfRange(id));
        }
        Ok(Self(id))
    }

    /// Return the numeric id of the category ("c7" would return 7).
    #[must_use]
    pub const fn id(self) -> u16 {
        self.0
    }
}

#[derive(Debug)]
pub enum CategoryError {
    OutOfRange(u16),
    InvalidFormat(String),
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "c{}", self.0)
    }
}

impl FromStr for Category {
    type Err = CategoryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with('c') {
            return Err(CategoryError::InvalidFormat(s.into()));
        }

        let num_part = &s[1..];

        let id: u16 = num_part
            .parse()
            .map_err(|_| CategoryError::InvalidFormat(s.into()))?;

        Self::new(id)
    }
}

///
/// CategorySet — ebitmap Equivalent
///
/// Kernel MLS uses ebitmap — effectively a sparse bitmap.
///
/// Fixed bitmap covering 1024 categories.
///
/// Userland can safely model this as a dense bitset for performance,
/// determinism, and simplified memory management.
///
#[derive(Clone, PartialEq, Eq)]
pub struct CategorySet {
    bits: [u64; 16], // 16 * 64 = 1024 bits
}

//
// Constructors
//
impl CategorySet {
    /// Creates an empty category set.
    /// Internal state: all bits = 0 which conceptually means no compartments.
    /// 
    /// # Example
    /// ```rust
    /// let mut myset = CategorySet::new();
    /// myset.insert(Category::new(4)?);
    /// ```
    #[must_use]
    pub const fn new() -> Self {
        Self { bits: [0; 16] }
    }

    /// Creates a full category set.
    /// Internal state: all bits = 1 which conceptually means all compartments.
    #[must_use]
    pub const fn full() -> Self {
        Self { bits: [u64::MAX; 16] }
    }
}

impl Default for CategorySet {
    fn default() -> Self {
        Self::new()
    }
}

//
// Bit position helpers
//
impl CategorySet {
    const fn index(cat: Category) -> (usize, u64) {
        let id = cat.id() as usize;
        let word = id / 64;
        let bit = id % 64;
        (word, 1u64 << bit)
    }
}

//
// Insert / Remove / Membership
//
impl CategorySet {
    /// Inserts a category into the set.
    ///
    /// Kernel equivalent: `ebitmap_set_bit()`
    ///
    pub const fn insert(&mut self, cat: Category) {
        let (word, mask) = Self::index(cat);
        self.bits[word] |= mask;
    }

    pub const fn remove(&mut self, cat: Category) {
        let (word, mask) = Self::index(cat);
        self.bits[word] &= !mask;
    }

    /// Tests category membership.
    ///
    /// Kernel equivalent:
    ///   `ebitmap_get_bit()`
    #[must_use]
    pub const fn contains(&self, cat: Category) -> bool {
        let (word, mask) = Self::index(cat);
        (self.bits[word] & mask) != 0
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.bits.iter().all(|w| *w == 0)
    }
}

//
// Dominance / Superset Checks
//
impl CategorySet {
    /// Determines MLS dominance.
    ///
    /// Kernel equivalent:
    ///   `ebitmap_and()` + comparison logic
    #[must_use]
    pub fn dominates(&self, other: &Self) -> bool {
        for i in 0..16 {
            if (self.bits[i] & other.bits[i]) != other.bits[i] {
                return false;
            }
        }
        true
    }
}

//
// Set Operations
//
impl CategorySet {
    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        let mut out = Self::new();
        for i in 0..16 {
            out.bits[i] = self.bits[i] | other.bits[i];
        }
        out
    }

    #[must_use]
    pub fn intersection(&self, other: &Self) -> Self {
        let mut out = Self::new();
        for i in 0..16 {
            out.bits[i] = self.bits[i] & other.bits[i];
        }
        out
    }
}

//
// Iteration Support
//
impl CategorySet {
    pub fn iter(&self) -> impl Iterator<Item = Category> + '_ {
        (0..=MAX_CATEGORY).filter_map(|id| {
            let cat = Category(id);
            if self.contains(cat) {
                Some(cat)
            } else {
                None
            }
        })
    }
}

//
// Display Formatting
//
impl fmt::Display for CategorySet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;

        for cat in self.iter() {
            if !first {
                write!(f, ",")?;
            }
            write!(f, "{cat}")?;
            first = false;
        }

        Ok(())
    }
}

//
// Debug Formatting (human-friendly)
//
impl fmt::Debug for CategorySet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CategorySet({self})")
    }
}

//
// Parsing Category Sets
//
impl FromStr for CategorySet {
    type Err = CategoryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut set = Self::new();

        for part in s.split(',') {
            let cat = part.trim().parse::<Category>()?;
            set.insert(cat);
        }

        Ok(set)
    }
}
