// =============================================================================
// UMRS SELinux Modeling Library
// =============================================================================
//
// Author: Jamie Adams
// License: MIT
//
// Description:
// Strongly-typed Rust primitives modeling SELinux MLS constructs,
// including categories, category sets, and dominance semantics.
//
// -----------------------------------------------------------------------------
// SELinux Lineage Reference
// -----------------------------------------------------------------------------
// Primitive: MLS Category Bitmap
//
// Kernel Sources Consulted:
//
//   security/selinux/ss/ebitmap.c
//   security/selinux/ss/ebitmap.h
//   security/selinux/ss/mls.c
//
// This implementation provides a strongly-typed Rust equivalent of the
// kernel ebitmap structure used to represent MLS category sets.
//
// Design deviations:
//
// • Dense bitmap instead of sparse linked nodes
// • Fixed 1024-bit width
// • Construct-time validation
//
// No SELinux source code has been copied or translated.
// -----------------------------------------------------------------------------

use std::fmt;
use std::str::FromStr;

//
// =============================================================================
// Category Primitive
// =============================================================================
//
// Category represents a single SELinux MLS category (c0–c1023).
//
// Categories are represented as bit positions within an ebitmap.
// This Rust type provides a strongly-typed, validated wrapper around
// that primitive representation.
//

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Category(u16);

pub const MAX_CATEGORY: u16 = 1023;

impl Category {
    pub fn new(id: u16) -> Result<Self, CategoryError> {
        if id > MAX_CATEGORY {
            return Err(CategoryError::OutOfRange(id));
        }
        Ok(Self(id))
    }

    pub fn id(self) -> u16 {
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

        Category::new(id)
    }
}

//
// =============================================================================
// CategorySet — ebitmap Equivalent
// =============================================================================
//
// Kernel MLS uses ebitmap — effectively a sparse bitmap.
//
// Userland can safely model this as a dense bitset for performance,
// determinism, and simplified memory management.
//
// Fixed bitmap covering 1024 categories.
//

#[derive(Clone, PartialEq, Eq)]
pub struct CategorySet {
    bits: [u64; 16], // 16 * 64 = 1024 bits
}

//
// Constructors
//
impl CategorySet {
    pub fn new() -> Self {
        Self { bits: [0; 16] }
    }

    pub fn full() -> Self {
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
    fn index(cat: Category) -> (usize, u64) {
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
    /// Kernel equivalent:
    ///   ebitmap_set_bit()
    pub fn insert(&mut self, cat: Category) {
        let (word, mask) = Self::index(cat);
        self.bits[word] |= mask;
    }

    pub fn remove(&mut self, cat: Category) {
        let (word, mask) = Self::index(cat);
        self.bits[word] &= !mask;
    }

    /// Tests category membership.
    ///
    /// Kernel equivalent:
    ///   ebitmap_get_bit()
    pub fn contains(&self, cat: Category) -> bool {
        let (word, mask) = Self::index(cat);
        (self.bits[word] & mask) != 0
    }

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
    ///   ebitmap_and() + comparison logic
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
    pub fn union(&self, other: &Self) -> Self {
        let mut out = Self::new();
        for i in 0..16 {
            out.bits[i] = self.bits[i] | other.bits[i];
        }
        out
    }

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
            write!(f, "{}", cat)?;
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
        write!(f, "CategorySet({})", self)
    }
}

//
// Parsing Category Sets
//
impl FromStr for CategorySet {
    type Err = CategoryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut set = CategorySet::new();

        for part in s.split(',') {
            let cat = part.trim().parse::<Category>()?;
            set.insert(cat);
        }

        Ok(set)
    }
}
