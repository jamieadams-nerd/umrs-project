// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Jamie Adams
//! # CUI Catalog
//!
//! Deserializes the CUI (Controlled Unclassified Information) label catalog
//! from a JSON file and provides lookup by label key and marking name.
//!
//! The catalog maps regulatory markings (e.g., `CUI//SP-CTI`) to structured
//! metadata as defined by the NARA CUI registry and applicable policy.
//!
//! ## Key Exported Types
//!
//! - [`Catalog`] — top-level container; loaded from a JSON file
//! - [`Label`] — top-level CUI label entry (e.g., `CUI`, `CUI//GENERAL`)
//! - [`Marking`] — a specific regulatory marking with metadata
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AC-16**: Security Attributes — catalog entries define
//!   the canonical mapping between MCS labels and regulatory CUI markings.
//! - **NIST SP 800-53 AU-3**: Audit Record Content — markings loaded here
//!   appear in all operator-visible security output.
//! - **CMMC AC.L2-3.1.3**: Control CUI flow in accordance with approved
//!   authorizations.

use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

/// Top-level CUI catalog container.
///
/// Loaded from a JSON file; holds label and marking registries.
///
/// ## Compliance
///
/// - **NIST SP 800-53 AC-16**: Security Attributes — the catalog is the
///   authoritative source of CUI marking definitions used in security output.
#[derive(Debug, Deserialize)]
pub struct Catalog {
    /// Top-level label registry (e.g., `"CUI"`, `"CUI//GENERAL"`).
    pub labels: HashMap<String, Label>,
    /// Regulatory marking registry (e.g., `"CUI//LEI"`, `"CUI//LEI/JUV"`).
    pub markings: HashMap<String, Marking>,
}

/// Load and deserialize a CUI catalog from a JSON file at `path`.
///
/// Returns `Err(String)` with a human-readable diagnostic if the file cannot
/// be opened or the JSON cannot be parsed.
#[must_use = "the loaded catalog is required; discard intentionally with let _ ="]
pub fn load_catalog<P: AsRef<Path>>(path: P) -> Result<Catalog, String> {
    let path_ref = path.as_ref();

    let file = File::open(path_ref)
        .map_err(|e| format!("Failed to open {}: {}", path_ref.display(), e))?;

    let reader = BufReader::new(file);

    serde_json::from_reader(reader).map_err(|e| {
        format!("Failed to parse JSON {}: {}", path_ref.display(), e)
    })
}

impl Catalog {
    /// Lookup a top-level label by key (e.g., `"CUI"`, `"GENERAL"`).
    #[must_use = "returns None if the key is absent; check before use"]
    pub fn label(&self, key: &str) -> Option<&Label> {
        self.labels.get(key)
    }

    /// Iterate all top-level labels.
    pub fn iter_labels(&self) -> impl Iterator<Item = (&String, &Label)> {
        self.labels.iter()
    }

    /// Lookup a marking by its JSON key (e.g., `"CUI//LEI/JUV"`).
    #[must_use = "returns None if the key is absent; check before use"]
    pub fn marking(&self, key: &str) -> Option<&Marking> {
        self.markings.get(key)
    }

    /// Iterate all markings as `(key, marking)` pairs.
    pub fn iter_markings(&self) -> impl Iterator<Item = (&String, &Marking)> {
        self.markings.iter()
    }

    /// Return all direct children of a marking key.
    ///
    /// A child is any marking whose `parent_group` matches the last
    /// slash-separated segment of `parent_key`.
    ///
    /// Example: `"CUI//LEI"` yields all LEI sub-category markings.
    pub fn marking_children<'a>(
        &'a self,
        parent_key: &str,
    ) -> impl Iterator<Item = (&'a String, &'a Marking)> {
        // Extract the last segment of the key (the group identifier).
        let parent_segment =
            parent_key.rsplit("//").next().unwrap_or(parent_key).to_string();

        self.markings
            .iter()
            .filter(move |(_, m)| m.parent_group == parent_segment)
    }
}

// ===========================================================================

/// A top-level CUI label entry.
#[derive(Debug, Deserialize)]
pub struct Label {
    /// Human-readable label name.
    pub name: String,
    /// Classification level string.
    pub level: String,
    /// Optional description.
    #[serde(default)]
    pub description: String,
    /// Optional handling guidance.
    #[serde(default)]
    pub handling: String,
}

// ===========================================================================

/// A specific CUI regulatory marking with associated metadata.
#[derive(Debug, Deserialize)]
pub struct Marking {
    /// Full human-readable name (e.g., `"Law Enforcement Information"`).
    pub name: String,
    /// Abbreviated name (e.g., `"LEI"`).
    pub abbrv_name: String,
    /// Parent group identifier (e.g., `"LEI"` for a child of `CUI//LEI`).
    pub parent_group: String,
    /// Optional description of the marking.
    #[serde(default)]
    pub description: String,
    /// Optional handling instructions.
    #[serde(default)]
    pub handling: String,
    /// Optional handling group identifier.
    #[serde(default)]
    pub handling_group_id: String,
    /// Optional auxiliary metadata.
    #[serde(default)]
    pub other: serde_json::Value,
}

impl Marking {
    /// Returns `true` if this marking has a non-empty description.
    #[must_use = "check return value to determine if description is present"]
    pub fn has_description(&self) -> bool {
        !self.description.trim().is_empty()
    }

    /// Returns `true` if this marking has non-empty handling guidance.
    #[must_use = "check return value to determine if handling is present"]
    pub fn has_handling(&self) -> bool {
        !self.handling.trim().is_empty()
    }

    /// Returns `true` if this marking has a non-empty handling group ID.
    #[must_use = "check return value to determine if handling group is present"]
    pub fn has_handling_group(&self) -> bool {
        !self.handling_group_id.trim().is_empty()
    }

    /// Returns `true` if this marking has non-empty auxiliary metadata.
    #[must_use = "check return value to determine if auxiliary metadata is present"]
    pub fn has_other(&self) -> bool {
        match &self.other {
            serde_json::Value::Object(map) => !map.is_empty(),
            serde_json::Value::Null => false,
            _ => true,
        }
    }
}

// ===========================================================================
