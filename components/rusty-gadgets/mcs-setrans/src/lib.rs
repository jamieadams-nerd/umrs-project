// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
// ============================================================================
//! # High-Assurance Vernacular Translation (setrans)
//!
//! This crate implements a deterministic, audit-focused translation engine for
//! converting SELinux MCS/MLS kernel security ranges into human-readable
//! regulatory markings (e.g., NARA CUI labels).
//!
//! UMRS SELINUX: High-Assurance Vernacular Translation
//! NIST 800-53 AC-4, AU-3
//! NSA Raise-the-Bar (Determinism, Loud Failure, Audit Fidelity)
//!
//! ---
//!
//! ## Architectural Guarantees
//!
//! ### 1. Strict Deterministic Parsing
//! The `/etc/selinux/.../setrans.conf` file is parsed using a single,
//! explicit, imperative parser with:
//!
//! - Strict prefix validation (`s`, `c` only; uppercase rejected loudly)
//! - Explicit range expansion (`c0.c1023` → discrete bit insertion)
//! - Order-style auditing (non-fatal debug alerts)
//! - Duplicate range detection (first-match wins)
//! - Loud failure on malformed sensitivity/category syntax
//!
//! No heuristic fallback paths are used. Any ambiguity results in explicit
//! error reporting during ingestion.
//!
//! ---
//!
//! ### 2. Lattice-Based Authorization Model
//!
//! All lookups operate on structured `SecurityRange` and `SecurityLevel`
//! types, not raw strings.
//!
//! - Sensitivity comparison is numeric.
//! - Category comparison is performed via `CategorySet` dominance checks.
//! - Read authorization follows lattice dominance semantics:
//!   `(proc.low dominates file.low)`
//!
//! This eliminates string-ordering vulnerabilities and ensures mathematical
//! determinism for authorization decisions.
//!
//! ---
//!
//! ### 3. Translation Table Semantics
//!
//! The in-memory translation engine stores:
//!
//! - `rules: BTreeMap<SecurityRange, String>`
//! - `details: BTreeMap<SecurityRange, String>`
//!
//! Characteristics:
//!
//! - First-match wins on duplicate definitions
//! - Forward lookup: `Range -> Label`
//! - Reverse lookup: `Label -> Vec<(KernelString, Detail)>`
//! - Dual-map design preserves regulatory comments without polluting the
//!   primary lookup path
//!
//! ---
//!
//! ### 4. Controlled Global Access
//!
//! The translation table is stored as:
//!
//!     LazyLock<RwLock<Translator>>
//!
//! This ensures:
//!
//! - Single initialization
//! - Thread-safe concurrent reads
//! - Explicit write locking during ingestion
//! - Deterministic lifetime semantics
//!
//! ---
//!
//! ### 5. Fail-Safe Audit Posture
//!
//! If no translation exists for a given `SecurityRange`, the engine does not
//! fabricate a marking. The caller is expected to surface the raw kernel
//! range string, preserving provenance integrity.
//!
//! No security state is hidden from the auditor.
//!
// ===========================================================================
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
// Selective suppressions
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::missing_errors_doc)]
// ===========================================================================
use log::{debug, info, warn};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::sync::LazyLock;
use std::sync::RwLock;

use umrs_selinux::category::{Category, CategorySet};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SecurityLevel {
    pub sensitivity: u32,
    pub categories: CategorySet,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SecurityRange {
    pub low: SecurityLevel,
    pub high: SecurityLevel,
}

// ===========================================================================
///
/// Translator Engine
///
/// This is already more feature-complete than typical setrans parsers.
/// Key characteristics:
/// - First-match wins
/// - Detail sidecar preserved
/// - Reverse lookup supported
/// - Range dominance used for authorization queries
///
pub struct Translator {
    pub rules: BTreeMap<SecurityRange, String>,
    /// Sidecar map for the # detail comments (your notes)
    pub details: BTreeMap<SecurityRange, String>,
}

///
/// The Global Singleton (Initialized with both maps)
///
/// Correct for: Read-heavy workload, One-time load, Runtime lookup
///
pub static GLOBAL_TRANSLATOR: LazyLock<RwLock<Translator>> =
    LazyLock::new(|| {
        RwLock::new(Translator {
            rules: BTreeMap::new(),
            details: BTreeMap::new(),
        })
    });

impl Translator {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            rules: BTreeMap::new(),
            details: BTreeMap::new(),
        }
    }

    /// Adds a rule and its optional detail sidecar.
    ///
    /// Sidecar detail map
    /// -------------------
    /// This is not avaiable in the main distribution of libselinux, but feel it is
    /// useful. If time is taken to detail the setrans.conf, that informaton should
    /// be available later during runtime.
    ///
    /// Keeping comments seperate from labels is good:
    /// -  Avoids polutting label string
    /// - Preserves audit metadata
    /// - Enables reverse lookup enrichment.
    ///
    pub fn add_rule(
        &mut self,
        range: SecurityRange,
        label: String,
        detail: String,
    ) {
        if !detail.is_empty() {
            self.details.insert(range.clone(), detail);
        }
        self.rules.insert(range, label);
    }

    ///
    /// FORWARD: 's0:c0' -> 'CUI'
    ///
    #[must_use]
    pub fn lookup(&self, range: &SecurityRange) -> Option<String> {
        self.rules.get(range).cloned()
    }

    ///
    /// DETAIL: Get the comment for a range
    ///
    /// In the setrans.conf, if comments are inline and to the right side
    /// of a definition, it keeps it as a detail. For example, the following line
    /// in setrans.conf:
    ///
    /// s0:c10 = CUI//PROCURE # GENERAL PROCUREMENT AND ACQUISITION
    ///
    /// details would be trimmed and include everything to the right of "#"
    ///   "GENERAL PROCUREMENT AND ACQUISITION"
    ///
    #[must_use]
    pub fn get_detail(&self, range: &SecurityRange) -> String {
        self.details.get(range).cloned().unwrap_or_default()
    }

    ///
    /// REVERSE: 'CUI' -> ('s0:c0', 'Controlled Unclassified Info')
    ///
    #[must_use]
    pub fn lookup_by_marking(&self, marking: &str) -> Vec<(String, String)> {
        let marking = marking.trim();
        self.rules
            .iter()
            .filter(|(_, label)| label.as_str() == marking)
            .map(|(range, _label)| {
                let kernel_str = if range.low == range.high {
                    format!(
                        "s{}:{}",
                        range.low.sensitivity, range.low.categories
                    )
                } else {
                    format!(
                        "s{}:{}-s{}:{}",
                        range.low.sensitivity,
                        range.low.categories,
                        range.high.sensitivity,
                        range.high.categories
                    )
                };
                let detail = self.get_detail(range);
                (kernel_str, detail)
            })
            .collect()
    }

    ///
    /// List of all markings authorized to read
    ///
    /// Returns a list of all markings (Label, Detail) that the given context is authorized to
    /// read.
    ///
    #[must_use]
    pub fn list_readable_markings(
        &self,
        proc_ctx: &SecurityRange,
    ) -> Vec<(SecurityRange, String, String)> {
        self.rules
            .iter()
            .filter(|(rule_range, _)| proc_ctx.can_read(rule_range))
            .map(|(range, label)| {
                let detail = self.get_detail(range);
                (range.clone(), label.clone(), detail)
            })
            .collect()
    }
}

impl Default for Translator {
    fn default() -> Self {
        Self::new()
    }
}


// ===========================================================================
// STRICT PARSER LOGIC
// ===========================================================================
impl SecurityLevel {
    ////
    /// Dominates
    ///
    /// This the correct layer -- dominance is a level property, not a range property.
    /// Math: (Self Sensitivity >= Other Sensitivity) AND (Self Categories are a
    /// SUPERSET of Other Categories)
    ///
    /// Returns true if 'self' dominates 'other'
    ///
    #[must_use]
    pub fn dominates(&self, other: &Self) -> bool {
        // Sensitivity check: Self must be at least as high as Other
        if self.sensitivity < other.sensitivity {
            return false;
        }

        // Category check: Self must contain EVERY category that Other has
        // We iterate through every category in 'other'
        //
        // Triple-check: 
        // - Ensure your CategorySet has a 'contains' method
        // - If it doesn't, we can use other.categories.iter().all(|c| ...)
        //
        for cat in other.categories.iter() {
            if !self.categories.contains(cat) {
                return false;
            }
        }

        true
    }
}

impl FromStr for SecurityLevel {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let (sens_part, cat_part) = if let Some((s_p, c_p)) = s.split_once(':')
        {
            (s_p, Some(c_p))
        } else {
            (s, None)
        };

        // Strict Sensitivity Check: Alert on 'S' (we don't like uppercase).
        if let Some(stripped) = sens_part.strip_prefix('S') {
            return Err(format!(
                "Syntax Error: Uppercase 'S' is invalid. Did you mean 's{stripped}'?")
                );
        }

        if !sens_part.starts_with('s') {
            return Err(format!(
                "Syntax Error: Sensitivity must start with 's' (found '{sens_part}')"
            ));
        }

        let sensitivity = sens_part
            .trim_start_matches('s')
            .parse::<u32>()
            .map_err(|_| format!("Invalid sensitivity format: {sens_part}"))?;

        let mut categories = CategorySet::new();
        let mut last_cat_id: Option<u32> = None;

        if let Some(c_str) = cat_part {
            for part in c_str.split(',') {
                let part = part.trim();

                // Strict Category Check: Alert on 'C' (we don't like uppercase)
                // Then, find the number to provide a helpful hint
                if part.contains('C') {
                    let hint = part.to_lowercase();
                    return Err(format!(
                        "Syntax Error: Uppercase 'C' is invalid in '{part}'. Use lowercase '{hint}'."
                    ));
                }

                let current_id =
                    if let Some((start_s, _)) = part.split_once('.') {
                        start_s.trim_start_matches('c').parse::<u32>().ok()
                    } else {
                        part.trim_start_matches('c').parse::<u32>().ok()
                    };

                // Style Auditor (Order) 
                // Read readability: we prefer the categories to be in ascending order.
                #[allow(clippy::collapsible_if)]
                if let Some(curr) = current_id {
                    if let Some(last) = last_cat_id {
                        if curr < last {
                            debug!(
                                "STYLE ALERT: Categories out of order ('{part}' follows 'c{last}')."
                            );
                        }
                    }

                    last_cat_id = if let Some((_, end_s)) = part.split_once('.')
                    {
                        end_s.trim_start_matches('c').parse::<u32>().ok()
                    } else {
                        Some(curr)
                    };
                }

                // Actual Parse Logic
                if let Some((start_str, end_str)) = part.split_once('.') {
                    let start = start_str
                        .trim_start_matches('c')
                        .parse::<u32>()
                        .map_err(|_| "Bad start")?;
                    let end = end_str
                        .trim_start_matches('c')
                        .parse::<u32>()
                        .map_err(|_| "Bad end")?;
                    for i in start..=end {
                        let name = format!("c{i}");
                        categories.insert(
                            Category::from_str(&name)
                                .map_err(|e| format!("{e:?}"))?,
                        );
                    }
                } else if !part.is_empty() {
                    categories.insert(
                        Category::from_str(part)
                            .map_err(|e| format!("{e:?}"))?,
                    );
                }
            }
        }
        Ok(Self {
            sensitivity,
            categories,
        })
    }
}

// ===========================================================================
impl SecurityRange {
    ///
    /// Range read authorization
    ///
    /// The "Read" Check: Returns true if the process (self) can read the target (`file_range`)
    /// This mirrors real SELinux read checks for simple policies.
    /// Later, we will expand to full range math, but this is valid.d
    ///
    /// In most SELinux policies, we compare the "low" levels for simple read access
    ///
    #[must_use]
    pub fn can_read(&self, file_range: &Self) -> bool {
        self.low.dominates(&file_range.low)
    }
}

impl FromStr for SecurityRange {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if let Some((low_s, high_s)) = s.split_once('-') {
            Ok(Self {
                low: SecurityLevel::from_str(low_s)?,
                high: SecurityLevel::from_str(high_s)?,
            })
        } else {
            let level = SecurityLevel::from_str(s)?;
            Ok(Self {
                low: level.clone(),
                high: level,
            })
        }
    }
}

// ===========================================================================
/// THE LOADER (First-Match Wins with Strict Error Handling)
///
pub fn load_setrans_file(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut translator =
        GLOBAL_TRANSLATOR.write().map_err(|_| "Lock poisoned")?;

    info!("Loading SELinux translations from: {path}");

    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        let line_num = index + 1;
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if let Some((raw_range, rest)) = line.split_once('=') {
            let raw_range = raw_range.trim();
            let (label, comment) = if let Some((l, c)) = rest.split_once('#') {
                (l.trim(), c.trim())
            } else {
                (rest.trim(), "")
            };

            match SecurityRange::from_str(raw_range) {
                Ok(range) => {
                    if let Some(old) = translator.rules.get(&range) {
                        warn!(
                            "Line {line_num}: IGNORED DUPLICATE! Range '{raw_range}' is already '{old}'."
                        );
                    } else {
                        if comment.is_empty() {
                            debug!(
                                "Line {line_num}: Loaded '{raw_range}' -> '{label}'"
                            );
                        } else {
                            debug!(
                                "Line {line_num}: Loaded '{raw_range}' -> '{label}' | Detail: {comment}"
                            );
                        }
                        translator.add_rule(
                            range,
                            label.to_string(),
                            comment.to_string(),
                        );
                    }
                }
                Err(e) => warn!(
                    "Line {line_num}: Syntax error on '{raw_range}' - {e}"
                ),
            }
        }
    }
    let rule_count = translator.rules.len();
    drop(translator); // - lock released.

    info!("Load complete. {rule_count} unique rules in memory.");

    Ok(())
}
