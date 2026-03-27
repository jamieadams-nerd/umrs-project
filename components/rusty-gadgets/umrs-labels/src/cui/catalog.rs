// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Jamie Adams
//! # CUI Catalog
//!
//! Deserializes CUI (Controlled Unclassified Information) and Five Eyes partner
//! catalogs from JSON files, providing lookup and iteration over labels and markings.
//!
//! The catalog design supports both the US CUI taxonomy (72+ named markings under a
//! single sensitivity level, accessed via `markings`) and national partner catalogs
//! such as Canadian Protected (impact-based, accessed via `labels`). Both load into
//! the same [`Catalog`] type; the caller determines which accessor is appropriate
//! for the catalog's country.
//!
//! ## Key Exported Types
//!
//! - [`CatalogMetadata`] — `_metadata` block present in all catalog files
//! - [`Catalog`] — top-level container; loaded from JSON via [`load_catalog`]
//! - [`Label`] — a top-level classification label (e.g., `CUI`, `Protected A`)
//! - [`Marking`] — a regulatory marking with handling guidance
//! - [`LevelDefinition`] — one MCS sensitivity level definition
//! - [`LevelRegistry`] — container for all level definitions; loaded via [`load_levels`]
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AC-16**: Security Attributes — catalog entries define the
//!   canonical mapping between MCS labels and regulatory markings.
//! - **NIST SP 800-53 AU-3**: Audit Record Content — markings loaded here appear
//!   in all operator-visible security output.
//! - **CMMC AC.L2-3.1.3**: Control CUI flow in accordance with approved
//!   authorizations.

use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

// ===========================================================================
// CatalogMetadata
// ===========================================================================

/// Metadata block present at the top of every catalog file (`_metadata` key).
///
/// Contains provenance, version, and MCS allocation information that applies
/// to the entire catalog. Nation-specific fields (e.g., `catalog_name_fr`,
/// `authority_date`, `structural_differences_from_us_cui`) are captured by
/// `extra` via `#[serde(flatten)]`.
///
/// ## Compliance
///
/// - **NIST SP 800-53 AC-16**: Provenance tracking for security attribute definitions.
#[derive(Debug, Deserialize)]
pub struct CatalogMetadata {
    /// Human-readable catalog name. Absent in `LEVELS.json` which uses `description` instead.
    #[serde(default)]
    pub catalog_name: String,
    /// Semantic version string (e.g., `"0.2.0"`).
    pub version: String,
    /// Governing authority (e.g., `"NARA CUI Registry / 32 CFR Part 2002"`).
    /// Absent in `LEVELS.json`.
    #[serde(default)]
    pub authority: String,
    /// ISO 3166-1 alpha-2 country code (e.g., `"US"`, `"CA"`). Optional
    /// because the LEVELS.json file has no country association.
    pub country_code: Option<String>,
    /// SELinux MCS category range allocated to this catalog (e.g., `"c0-c199"`).
    pub mcs_category_range: Option<String>,
    /// Nation-specific metadata fields not enumerated above.
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

// ===========================================================================
// Catalog
// ===========================================================================

/// Top-level catalog container, loaded from a JSON file.
///
/// Holds optional metadata, a label registry, and a marking registry. Both
/// `labels` and `markings` default to empty if absent from the JSON file,
/// allowing a single type to represent catalogs from different nations:
///
/// - US CUI: populated `markings`, sparse `labels`
/// - Canadian Protected: populated `labels`, no `markings` key
///
/// ## Compliance
///
/// - **NIST SP 800-53 AC-16**: Security Attributes — the catalog is the
///   authoritative source of CUI marking definitions used in security output.
#[derive(Debug, Deserialize)]
pub struct Catalog {
    /// Catalog provenance and MCS allocation metadata.
    #[serde(rename = "_metadata")]
    pub metadata: Option<CatalogMetadata>,
    /// Top-level label registry (e.g., `"CUI"`, `"GENERAL"`, `"PROTECTED-A"`).
    #[serde(default)]
    pub labels: HashMap<String, Label>,
    /// Regulatory marking registry (e.g., `"CUI//LEI"`, `"CUI//LEI/JUV"`).
    #[serde(default)]
    pub markings: HashMap<String, Marking>,
}

/// Load and deserialize a catalog from a JSON file at `path`.
///
/// Returns `Err(String)` with a human-readable diagnostic if the file cannot
/// be opened or the JSON cannot be parsed.
#[must_use = "the loaded catalog is required; discard intentionally with let _ ="]
pub fn load_catalog<P: AsRef<Path>>(path: P) -> Result<Catalog, String> {
    let path_ref = path.as_ref();

    let file = File::open(path_ref)
        .map_err(|e| format!("Failed to open {}: {}", path_ref.display(), e))?;

    let reader = BufReader::new(file);

    serde_json::from_reader(reader)
        .map_err(|e| format!("Failed to parse JSON {}: {}", path_ref.display(), e))
}

impl Catalog {
    /// Returns the ISO 3166-1 alpha-2 country code from metadata, if present.
    #[must_use = "returns None if metadata or country_code is absent; check before use"]
    pub fn country_code(&self) -> Option<&str> {
        self.metadata
            .as_ref()
            .and_then(|m| m.country_code.as_deref())
    }

    /// Iterate all markings (US convention — `markings` key in JSON).
    ///
    /// For catalogs that use the `labels` key (e.g., Canadian Protected),
    /// use [`Catalog::iter_labels`] instead.
    pub fn all_markings(&self) -> impl Iterator<Item = (&String, &Marking)> {
        self.markings.iter()
    }

    /// Lookup a top-level label by key (e.g., `"CUI"`, `"PROTECTED-A"`).
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
// Label
// ===========================================================================

/// A top-level classification label entry.
///
/// Represents broad classification tiers such as `"CUI"`, `"GENERAL"`, or
/// `"PROTECTED-A"`. The `handling` field is `serde_json::Value` to accommodate
/// both US string handling instructions and Canadian structured handling objects.
///
/// ## Compliance
///
/// - **NIST SP 800-53 AC-16**: Labels are the top tier of the security attribute
///   taxonomy defined by this catalog.
#[derive(Debug, Deserialize)]
pub struct Label {
    /// Human-readable label name.
    pub name: String,
    /// Human-readable label name in French (bilingual catalogs).
    pub name_fr: Option<String>,
    /// MCS sensitivity level identifier (e.g., `"s1"`, `"s2"`).
    pub level: Option<String>,
    /// Optional description.
    #[serde(default)]
    pub description: String,
    /// Optional description in French.
    pub description_fr: Option<String>,
    /// Abbreviated name (e.g., `"PA"` for Protected A).
    pub abbrv_name: Option<String>,
    /// MCS category base (e.g., `"c200"`).
    pub category_base: Option<String>,
    /// Handling guidance — string (US) or structured object (Canadian).
    #[serde(default)]
    pub handling: serde_json::Value,
}

impl Label {
    /// Returns handling guidance as a plain string, if it is one.
    #[must_use = "returns None if handling is absent or is not a string value"]
    pub fn handling_as_str(&self) -> Option<&str> {
        self.handling.as_str()
    }

    /// Returns handling guidance as a JSON object, if it is one.
    #[must_use = "returns None if handling is absent or is not an object value"]
    pub fn handling_as_object(&self) -> Option<&serde_json::Map<String, serde_json::Value>> {
        self.handling.as_object()
    }

    /// Returns `true` if this label has non-empty handling guidance.
    #[must_use = "check return value to determine if handling is present"]
    pub fn has_handling(&self) -> bool {
        match &self.handling {
            serde_json::Value::String(s) => !s.trim().is_empty(),
            serde_json::Value::Object(map) => !map.is_empty(),
            serde_json::Value::Null => false,
            _ => true,
        }
    }
}

// ===========================================================================
// Marking
// ===========================================================================

/// A specific regulatory marking with associated metadata.
///
/// The `handling` field is `serde_json::Value` to accommodate both US string
/// handling instructions and Canadian structured handling objects. Use
/// `handling_as_str()` or `handling_as_object()` for typed access.
///
/// ## Compliance
///
/// - **NIST SP 800-53 AC-16**: Markings are the leaf-level security attributes
///   used in MCS category assignments and operator-visible output.
/// - **NIST SP 800-53 AU-3**: Audit Record Content — markings appear in
///   audit-visible directory listings and security reports.
/// - **CMMC AC.L2-3.1.3**: Markings define the dissemination boundaries for
///   CUI and partner-nation controlled information.
#[derive(Debug, Deserialize)]
pub struct Marking {
    /// Full human-readable name (e.g., `"Law Enforcement Information"`).
    pub name: String,
    /// Full name in French (bilingual catalogs).
    pub name_fr: Option<String>,
    /// Abbreviated name (e.g., `"LEI"`).
    pub abbrv_name: String,
    /// Parent group identifier (e.g., `"LEI"` for a child of `CUI//LEI`).
    pub parent_group: String,
    /// Optional description of the marking.
    #[serde(default)]
    pub description: String,
    /// Optional description in French.
    pub description_fr: Option<String>,
    /// MCS sensitivity level identifier (e.g., `"s1"`).
    pub level: Option<String>,
    /// Handling guidance — string (US) or structured object (Canadian).
    #[serde(default)]
    pub handling: serde_json::Value,
    /// Detailed handling restrictions (US catalogs).
    pub handling_restrictions: Option<String>,
    /// Detailed handling restrictions in French.
    pub handling_restrictions_fr: Option<String>,
    /// Optional handling group identifier for grouping related markings.
    #[serde(default)]
    pub handling_group_id: String,
    /// MCS category base (e.g., `"c200"`).
    pub category_base: Option<String>,
    /// Reserved MCS category range for future use (e.g., `"c201-c299"`).
    pub category_range_reserved: Option<String>,
    /// Color palette reference key (links to UMRS-PALETTE.json).
    pub palette_ref: Option<String>,
    /// Risk domain identifiers associated with this marking.
    pub risk_domains: Option<Vec<String>>,
    /// Dissemination controls — varies by nation (string, object, or null).
    pub dissemination_controls: Option<serde_json::Value>,
    /// Cross-reference to approximately equivalent US CUI categories.
    pub us_cui_approximate_correspondence: Option<String>,
    /// Display banner in English (e.g., `"PROTECTED A"`).
    pub marking_banner_en: Option<String>,
    /// Display banner in French (e.g., `"PROTÉGÉ A"`).
    pub marking_banner_fr: Option<String>,
    /// Governing authority section reference (e.g., `"J.2.4.2.3"`).
    pub authority_section: Option<String>,
    /// Implementation phase notes for UMRS-specific constraints.
    pub phase_note: Option<String>,
    /// Injury examples in English (Canadian catalogs).
    pub injury_examples: Option<String>,
    /// Injury examples in French (Canadian catalogs).
    pub injury_examples_fr: Option<String>,
    /// Optional auxiliary metadata for forward compatibility.
    #[serde(default)]
    pub other: serde_json::Value,
}

impl Marking {
    /// Returns `true` if this marking has a non-empty description.
    #[must_use = "check return value to determine if description is present"]
    pub fn has_description(&self) -> bool {
        !self.description.trim().is_empty()
    }

    /// Returns handling guidance as a plain string, if it is one.
    #[must_use = "returns None if handling is absent or is not a string value"]
    pub fn handling_as_str(&self) -> Option<&str> {
        self.handling.as_str()
    }

    /// Returns handling guidance as a JSON object, if it is one.
    #[must_use = "returns None if handling is absent or is not an object value"]
    pub fn handling_as_object(&self) -> Option<&serde_json::Map<String, serde_json::Value>> {
        self.handling.as_object()
    }

    /// Returns `true` if this marking has non-empty handling guidance.
    #[must_use = "check return value to determine if handling is present"]
    pub fn has_handling(&self) -> bool {
        match &self.handling {
            serde_json::Value::String(s) => !s.trim().is_empty(),
            serde_json::Value::Object(map) => !map.is_empty(),
            serde_json::Value::Null => false,
            _ => true,
        }
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
// LevelRegistry
// ===========================================================================

/// One MCS sensitivity level definition from `LEVELS.json`.
///
/// Maps a sensitivity label (e.g., `"s1"`) to its name, description, and the
/// nations that use it as their default controlled-information tier.
///
/// ## Compliance
///
/// - **NIST SP 800-53 AC-16**: Sensitivity levels define the Bell-LaPadula
///   dominance ordering for multi-level security enforcement.
#[derive(Debug, Deserialize)]
pub struct LevelDefinition {
    /// Human-readable level name (e.g., `"Controlled L1"`).
    pub name: String,
    /// Description of the sensitivity tier and its enforcement characteristics.
    pub description: String,
    /// Nations that use this level as their primary controlled-information tier.
    pub nations: Option<Vec<String>>,
}

/// Registry of all MCS sensitivity level definitions, loaded from `LEVELS.json`.
///
/// The `levels` map is keyed by sensitivity label string (e.g., `"s0"`, `"s1"`).
///
/// ## Compliance
///
/// - **NIST SP 800-53 AC-16**: Security Attributes — the level registry is the
///   authoritative source of MCS sensitivity semantics across all Five Eyes catalogs.
#[derive(Debug, Deserialize)]
pub struct LevelRegistry {
    /// Provenance metadata for the levels file.
    #[serde(rename = "_metadata")]
    pub metadata: Option<CatalogMetadata>,
    /// Map from sensitivity label (e.g., `"s1"`) to level definition.
    pub levels: HashMap<String, LevelDefinition>,
}

/// Load and deserialize the MCS level registry from a JSON file at `path`.
///
/// Returns `Err(String)` with a human-readable diagnostic if the file cannot
/// be opened or the JSON cannot be parsed.
#[must_use = "the loaded level registry is required; discard intentionally with let _ ="]
pub fn load_levels<P: AsRef<Path>>(path: P) -> Result<LevelRegistry, String> {
    let path_ref = path.as_ref();

    let file = File::open(path_ref)
        .map_err(|e| format!("Failed to open {}: {}", path_ref.display(), e))?;

    let reader = BufReader::new(file);

    serde_json::from_reader(reader)
        .map_err(|e| format!("Failed to parse JSON {}: {}", path_ref.display(), e))
}

impl LevelRegistry {
    /// Lookup a level definition by its sensitivity label (e.g., `"s1"`).
    #[must_use = "returns None if the level key is absent; check before use"]
    pub fn level(&self, key: &str) -> Option<&LevelDefinition> {
        self.levels.get(key)
    }

    /// Iterate all level definitions as `(key, definition)` pairs.
    pub fn iter_levels(&self) -> impl Iterator<Item = (&String, &LevelDefinition)> {
        self.levels.iter()
    }
}

// ===========================================================================
