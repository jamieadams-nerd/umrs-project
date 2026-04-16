// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Jamie Adams
//! # CUI Catalog
//!
//! Deserializes CUI (Controlled Unclassified Information) and Five Eyes partner
//! catalogs from JSON files, providing lookup and iteration over markings.
//!
//! Both US and Canadian catalogs use a unified `"markings"` top-level key.
//! Text fields (`name`, `description`, `marking_banner`, `injury_examples`)
//! are represented as [`LocaleText`], which transparently handles both the
//! legacy flat-string format (US catalogs) and the locale-keyed object format
//! (bilingual catalogs). Use `.en()` for English display, `.fr()` for Canadian
//! French.
//!
//! ## Key Exported Types
//!
//! - [`LocaleText`] — bilingual text container for locale-keyed catalog fields
//! - [`CatalogMetadata`] — `_metadata` block present in all catalog files
//! - [`Catalog`] — top-level container; loaded from JSON via [`load_catalog`]
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
//! - **NIST SP 800-53 SI-10**: Information Input Validation — `LocaleText`
//!   validates locale values at the deserialization boundary.
//! - **CMMC AC.L2-3.1.3**: Control CUI flow in accordance with approved
//!   authorizations.

use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use umrs_selinux::status::{SelinuxPolicy, selinux_policy};

pub use super::locale_text::LocaleText;

// ===========================================================================
// CatalogMetadata
// ===========================================================================

/// Metadata block present at the top of every catalog file (`_metadata` key).
///
/// Contains provenance, version, and MCS allocation information that applies
/// to the entire catalog. Nation-specific fields (e.g., `authority_date`,
/// `structural_differences_from_us_cui`) are captured by `extra` via
/// `#[serde(flatten)]`.
///
/// `catalog_name` is a [`LocaleText`] that handles both the US flat-string
/// form (`"United States CUI"`) and the Canadian locale-object form
/// (`{"en_US": "...", "fr_CA": "..."}`).
///
/// ## Compliance
///
/// - **NIST SP 800-53 AC-16**: Provenance tracking for security attribute definitions.
#[derive(Debug, Deserialize)]
pub struct CatalogMetadata {
    /// Human-readable catalog name. Absent in `LEVELS.json` which uses `description` instead.
    /// Handles both flat-string (US) and locale-object (CA) JSON representations.
    #[serde(default)]
    pub catalog_name: LocaleText,
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

// ===========================================================================
// DisseminationControl
// ===========================================================================

/// A Limited Dissemination Control (LDC) entry from the `dissemination_controls`
/// section of a US CUI catalog.
///
/// LDCs constrain who may receive CUI — e.g., `NOFORN` (no foreign dissemination),
/// `FED ONLY` (federal employees only). They appear at the end of a CUI banner
/// after a double slash: `CUI//SP-CTI//NOFORN`.
///
/// ## Compliance
///
/// - **NIST SP 800-53 AC-16**: LDCs are dissemination-control security attributes
///   that restrict which principals may receive CUI outside the originating agency.
/// - **CMMC AC.L2-3.1.3**: Control CUI flow in accordance with approved authorizations.
#[derive(Debug, Deserialize, Clone)]
pub struct DisseminationControl {
    /// Full human-readable name (may be bilingual).
    pub name: LocaleText,
    /// Description of when this LDC applies and what it means.
    pub description: LocaleText,
    /// The token that appears in the banner (e.g., `"NOFORN"`, `"Attorney-Client"`).
    pub banner_marking: Option<String>,
    /// Short portion-marking abbreviation (e.g., `"NF"`, `"AC"`).
    pub portion_marking: Option<String>,
    /// Whether this LDC requires a parameter (e.g., country codes for `REL TO`).
    #[serde(default)]
    pub parameterized: bool,
    /// If present, this LDC may only be used with the named CUI category.
    pub category_restriction: Option<String>,
    /// LDC abbreviations that are mutually exclusive with this one.
    #[serde(default)]
    pub mutually_exclusive_with: Vec<String>,
}

// ===========================================================================
// Catalog
// ===========================================================================

/// Top-level catalog container, loaded from a JSON file.
///
/// Both US and Canadian catalogs use the `markings` key. US entries contain
/// the core CUI taxonomy. Canadian entries additionally carry bilingual names,
/// impact-tier examples, and structured handling objects — all represented as
/// `Option` fields in the shared [`Marking`] type.
///
/// US catalogs also include a `dissemination_controls` section (LDCs) that is
/// separate from `markings`. Canadian catalogs omit this section.
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
    /// Regulatory marking registry (e.g., `"CUI//LEI"`,
    /// `"PROTECTED-A"`). Both US and Canadian catalogs populate this key.
    #[serde(default)]
    pub markings: HashMap<String, Marking>,
    /// Limited Dissemination Controls (LDCs) — US catalogs only.
    ///
    /// Keyed by the LDC name (e.g., `"NOFORN"`, `"Attorney-Client"`).
    /// Empty in Canadian catalogs.
    #[serde(default)]
    pub dissemination_controls: HashMap<String, DisseminationControl>,
}

/// Load and deserialize a catalog from a JSON file at `path`.
///
/// Returns `Err(String)` with a human-readable diagnostic if the file cannot
/// be opened or the JSON cannot be parsed.
#[must_use = "the loaded catalog is required; discard intentionally with let _ ="]
pub fn load_catalog<P: AsRef<Path>>(path: P) -> Result<Catalog, String> {
    let path_ref = path.as_ref();

    // ACCEPTED-RISK: Catalog JSON files are the authoritative source of CUI label
    // definitions (NIST SP 800-53 AC-16). A tampered catalog could show incorrect
    // marking information. Catalog integrity verification (SHA-256 against a
    // known-good manifest) is planned for a future phase. Until then, catalog
    // files are trusted based on filesystem permissions and SELinux type enforcement.
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
        self.metadata.as_ref().and_then(|m| m.country_code.as_deref())
    }

    /// Returns the Unicode flag emoji for this catalog's country, if the
    /// country code is present and valid.
    ///
    /// Delegates to [`country_flag`]. Returns `None` when the catalog has no
    /// `country_code` in its `_metadata` block, or when the stored code is not
    /// exactly two ASCII alphabetic characters.
    #[must_use = "flag emoji is computed but unused"]
    pub fn country_flag(&self) -> Option<String> {
        self.country_code().and_then(country_flag)
    }

    /// Iterate all markings as `(key, marking)` pairs.
    pub fn all_markings(&self) -> impl Iterator<Item = (&String, &Marking)> {
        self.markings.iter()
    }

    /// Lookup a marking by its JSON key (e.g., `"CUI//JUV"`, `"PROTECTED-A"`).
    #[must_use = "returns None if the key is absent; check before use"]
    pub fn marking(&self, key: &str) -> Option<&Marking> {
        self.markings.get(key)
    }

    /// Iterate all markings as `(key, marking)` pairs.
    pub fn iter_markings(&self) -> impl Iterator<Item = (&String, &Marking)> {
        self.markings.iter()
    }

    /// Look up a marking by a raw MCS level string (e.g., `"s1:c300"`, `"s2:c301"`).
    ///
    /// This is a fallback lookup for catalog entries whose JSON keys (e.g.,
    /// `"PROTECTED-A"`) do not appear as translated labels in `setrans.conf`.
    /// When `setrans.conf` has no entry for a given MCS category (c300–c399 for
    /// Canadian Protected designations), the group header carries the raw level
    /// string rather than a human-readable marking. This method finds the catalog
    /// entry whose `level` and `category_base` match the components of the raw
    /// level string, allowing the popup to resolve correctly.
    ///
    /// Parsing rules:
    /// - `raw_level` is split on `:` to extract `(sensitivity, categories)`.
    /// - `sensitivity` is compared against `Marking::level` (e.g., `"s1"`).
    /// - The first category token in `categories` is compared against
    ///   `Marking::category_base` (e.g., `"c300"`).
    /// - Returns the first matching `(key, marking)` pair, or `None`.
    ///
    /// This method performs a linear scan over the catalog. It is intended for
    /// small catalogs (e.g., three Canadian tiers) where a hash lookup by display
    /// string is insufficient. It is not intended for the US CUI catalog where
    /// setrans.conf provides complete translations.
    ///
    /// Fail-closed: returns `None` on any parse ambiguity rather than guessing.
    ///
    /// ## Compliance
    ///
    /// - **NIST SP 800-53 AC-16**: Security Attributes — enables correct popup
    ///   resolution for Canadian Protected markings when setrans translations
    ///   are absent.
    #[must_use = "returns None if no matching entry is found; check before use"]
    pub fn marking_by_mcs_level(&self, raw_level: &str) -> Option<(&String, &Marking)> {
        // Split on ':' to separate sensitivity from category list.
        // "s1:c300" → sensitivity = "s1", categories = "c300"
        // "s1" (no colon) → no categories, cannot match a category-based entry.
        let (sensitivity, categories) = raw_level.split_once(':')?;

        // Extract the first category token (before any comma or range separator).
        // "c300" → "c300"
        // "c300,c303" → "c300"  (first base category)
        let first_category = categories.split([',', '.']).next()?;

        // Trim whitespace — defensive, since raw level strings are kernel-sourced
        // and should not contain whitespace, but be explicit.
        let sensitivity = sensitivity.trim();
        let first_category = first_category.trim();

        if sensitivity.is_empty() || first_category.is_empty() {
            return None;
        }

        self.markings.iter().find(|(_, m)| {
            m.level.as_deref() == Some(sensitivity)
                && m.category_base.as_deref() == Some(first_category)
        })
    }

    /// Iterate all dissemination controls as `(key, control)` pairs.
    ///
    /// Returns an empty iterator for catalogs that have no dissemination controls
    /// section (e.g., Canadian catalogs).
    pub fn iter_dissemination_controls(
        &self,
    ) -> impl Iterator<Item = (&String, &DisseminationControl)> {
        self.dissemination_controls.iter()
    }

    /// Returns `true` if this catalog contains any dissemination control entries.
    #[must_use = "check return value to determine whether to build the LDC tree branch"]
    pub fn has_dissemination_controls(&self) -> bool {
        !self.dissemination_controls.is_empty()
    }
}

// ===========================================================================
// Marking
// ===========================================================================

/// A specific regulatory marking with associated metadata.
///
/// This type represents a unified schema shared by both US CUI and Canadian
/// Protected catalog entries. All human-readable text fields (`name`,
/// `description`, `marking_banner`, `injury_examples`) use [`LocaleText`],
/// which transparently handles both legacy flat-string and locale-keyed object
/// JSON representations. Use `.en()` for English and `.fr()` for Canadian French.
///
/// `index_group` is a display-only grouping hint for UI purposes (e.g.,
/// `"Law Enforcement"`) and carries no enforcement semantics.
///
/// The `handling` field is `serde_json::Value` to accommodate both US string
/// handling instructions and Canadian structured handling objects (which contain
/// locale-keyed sub-fields). Use `handling_as_str()` or `handling_as_object()`
/// for typed access.
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
    /// Use `.en()` for English, `.fr()` for Canadian French.
    pub name: LocaleText,
    /// Abbreviated name (e.g., `"LEI"`).
    pub abbrv_name: String,
    /// Display-only grouping hint (e.g., `"Law Enforcement"`). Carries no
    /// enforcement semantics.
    pub index_group: Option<String>,
    /// CUI designation: `"basic"`, `"specified"`, or `null`.
    pub designation: Option<String>,
    /// Description of the marking. Use `.en()` for English, `.fr()` for
    /// Canadian French. Empty `LocaleText` when absent.
    #[serde(default)]
    pub description: LocaleText,
    /// MCS sensitivity level identifier (e.g., `"s1"`).
    pub level: Option<String>,
    /// Handling guidance — string (US) or structured object (Canadian).
    /// Canadian objects contain locale-keyed sub-fields; use `handling_as_object()`
    /// and navigate the sub-keys directly as `serde_json::Value`.
    #[serde(default)]
    pub handling: serde_json::Value,
    /// Detailed handling restrictions (US catalogs).
    pub handling_restrictions: Option<String>,
    /// Optional handling group identifier for grouping related markings.
    /// `null` in Canadian entries.
    pub handling_group_id: Option<String>,
    /// Required warning statement associated with this marking, if any.
    pub required_warning_statement: Option<String>,
    /// Required dissemination control suffix, if mandated by policy.
    pub required_dissemination_control: Option<String>,
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
    /// Display banner text (e.g., English `"PROTECTED A"`, French `"PROTÉGÉ A"`).
    /// Use `.en()` for English, `.fr()` for Canadian French.
    /// `None` for US CUI entries that do not carry a separate banner field.
    pub marking_banner: Option<LocaleText>,
    /// Governing authority section reference (e.g., `"J.2.4.2.3"`).
    pub authority_section: Option<String>,
    /// Implementation phase notes for UMRS-specific constraints.
    pub phase_note: Option<String>,
    /// Injury examples (Canadian catalogs). Use `.en()` or `.fr()`.
    pub injury_examples: Option<LocaleText>,
    /// Optional auxiliary metadata for forward compatibility.
    #[serde(default)]
    pub other: serde_json::Value,
}

impl Marking {
    /// Returns the English display name for this marking.
    ///
    /// Convenience accessor equivalent to `self.name.en()`.
    #[must_use = "display name is consumed by label rendering and audit paths"]
    pub fn display_name(&self) -> &str {
        self.name.en()
    }

    /// Returns `true` if this marking has non-empty description text.
    ///
    /// Uses the English value as the canonical presence indicator — a marking
    /// with only a French description is considered to have content.
    #[must_use = "check return value to determine if description is present"]
    pub fn has_description(&self) -> bool {
        self.description.has_content()
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
        self.handling_group_id.as_deref().is_some_and(|s| !s.trim().is_empty())
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
// Country flag emoji
// ===========================================================================

/// Convert an ISO 3166-1 alpha-2 country code to a Unicode flag emoji.
///
/// Each letter maps to a Regional Indicator Symbol Letter (U+1F1E6 through
/// U+1F1FF). Two such code points combined render as a flag emoji in
/// Unicode-conformant renderers.
///
/// Returns `None` if `iso_code` is not exactly two ASCII alphabetic characters.
/// The check is case-insensitive — `"us"`, `"US"`, and `"Us"` all yield `"🇺🇸"`.
///
/// # Examples
///
/// ```rust
/// use umrs_labels::cui::catalog::country_flag;
///
/// assert_eq!(country_flag("US"), Some("🇺🇸".to_string()));
/// assert_eq!(country_flag("CA"), Some("🇨🇦".to_string()));
/// assert_eq!(country_flag("GB"), Some("🇬🇧".to_string()));
/// assert_eq!(country_flag("us"), Some("🇺🇸".to_string()));
/// assert_eq!(country_flag("USA"), None);
/// assert_eq!(country_flag("12"), None);
/// assert_eq!(country_flag(""),   None);
/// ```
///
/// ## Compliance
///
/// - **NIST SP 800-53 AU-3**: Audit Record Content — flag emoji provides
///   operator-visible nation identification in security output.
#[must_use = "flag emoji is computed but unused"]
pub fn country_flag(iso_code: &str) -> Option<String> {
    // Regional Indicator Symbol Letter A starts at U+1F1E6.
    // Offset by (uppercase_letter - 'A') to reach the correct indicator letter.
    const REGIONAL_A: u32 = 0x1F1E6;

    // Consume exactly two characters; reject empty, single-char, and longer inputs.
    let mut chars = iso_code.chars();
    let a = chars.next()?;
    let b = chars.next()?;

    // Must be exactly 2 characters — reject a third.
    if chars.next().is_some() {
        return None;
    }

    // Both characters must be ASCII alphabetic.
    if !a.is_ascii_alphabetic() || !b.is_ascii_alphabetic() {
        return None;
    }

    let offset_a = u32::from(a.to_ascii_uppercase()) - u32::from('A');
    let offset_b = u32::from(b.to_ascii_uppercase()) - u32::from('A');

    // char::from_u32 returns Option<char>; propagate None on failure (fail-closed).
    // In practice this cannot fail for valid A-Z input, but we do not rely on that.
    let ri_a = char::from_u32(REGIONAL_A + offset_a)?;
    let ri_b = char::from_u32(REGIONAL_A + offset_b)?;

    let mut flag = String::with_capacity(8); // two 4-byte Unicode scalar values
    flag.push(ri_a);
    flag.push(ri_b);
    Some(flag)
}

// ===========================================================================
// Policy-aware description adjustment
// ===========================================================================

/// Adjust description text based on the active SELinux policy type.
///
/// Under targeted policy (Phase 1), enforcement language is replaced with
/// labeling language so that displayed descriptions accurately reflect UMRS
/// capability. Under MLS policy (Phase 2), enforcement language is accurate
/// and is preserved unchanged.
///
/// This function is called on the English and French description strings from
/// catalog entries before they reach operator-visible output. It ensures that
/// no UMRS display path overstates the enforcement capability of the active
/// policy.
///
/// ## Compliance
///
/// - **NIST SP 800-53 AC-3**: Access Enforcement — accurately describes the
///   level of enforcement the active policy provides.
/// - **NIST SP 800-53 PL-4**: Rules of Behavior — security documentation must
///   not overstate system capability.
#[must_use = "the adjusted description must be used in place of the original"]
pub fn policy_aware_description(description: &str) -> String {
    let is_mls = matches!(selinux_policy(), Some(SelinuxPolicy::Mls));
    if is_mls {
        // Under MLS, enforcement language is accurate — preserve as-is.
        description.to_owned()
    } else {
        // Under targeted policy, replace enforcement language with labeling
        // language to avoid overstating UMRS capability (Phase 1).
        description
            .replace("MAC enforcement", "MCS labeling")
            .replace(
                "mandatory access control enforcement",
                "MCS category labeling",
            )
            .replace("granular MAC enforcement", "granular MCS category labeling")
            // French equivalents
            .replace("contrôle d'accès obligatoire", "étiquetage MCS")
            .replace(
                "l'application granulaire du contrôle d'accès obligatoire",
                "l'étiquetage granulaire des catégories MCS",
            )
    }
}
