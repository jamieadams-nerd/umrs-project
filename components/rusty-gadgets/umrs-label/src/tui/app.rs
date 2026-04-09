// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams

//! # app — Label Registry Application State and Tree Builder
//!
//! Provides [`LabelRegistryApp`], the top-level data holder for the
//! `umrs-label` TUI registry browser. The struct owns both loaded [`Catalog`]
//! instances (US and Canadian) and exposes methods for building the tree
//! model and constructing [`MarkingDetailData`] for the detail panel.
//!
//! ## Key Exported Types
//!
//! - [`LabelRegistryApp`] — owns both catalogs, exposes tree building and
//!   detail construction.
//! - [`Panel`] — identifies which panel (Tree or Detail) is focused.
//! - [`DetailContent`] — discriminates what the detail panel is showing.
//!
//! ## Tree Structure
//!
//! The tree has one root per loaded catalog. Within each root:
//! - Ungrouped markings (where `index_group` is `None`) appear directly
//!   under the catalog root.
//! - Grouped markings appear under `"Group: <name>"` branch nodes, sorted
//!   alphabetically by group name.
//! - Markings within each group are sorted alphabetically by key.
//! - US catalogs gain a `"Dissemination Controls"` branch when the
//!   `dissemination_controls` map is non-empty.
//!
//! ## Node Metadata Convention
//!
//! Leaf nodes carry their catalog key in `metadata["key"]`. Group branch
//! nodes carry `metadata["kind"] = "group"`. The detail panel builder
//! checks `metadata["kind"]` to decide what to render.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AC-16**: Security Attributes — tree accurately
//!   represents all catalog entries with no omissions.
//! - **NIST SP 800-53 AU-3**: Audit Record Content — every node in the
//!   tree carries its catalog key for traceability.
//! - **NIST SP 800-53 AC-3**: The app is read-only; catalogs are loaded
//!   at startup and never mutated through the viewer interface.
//! - **NSA RTB RAIN**: Non-bypassable read-only contract — no mutation
//!   methods exist on `LabelRegistryApp`.

use std::collections::BTreeMap;

use umrs_ui::marking_detail::MarkingDetailData;
use umrs_ui::viewer::tree::{TreeModel, TreeNode};

use crate::cui::catalog::{Catalog, DisseminationControl, Marking, policy_aware_description};

// ---------------------------------------------------------------------------
// Panel focus
// ---------------------------------------------------------------------------

/// Which panel currently has keyboard focus.
///
/// NIST SP 800-53 AU-3 — panel focus is surfaced in the status bar so
/// the operator always knows which panel is accepting input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Panel {
    /// The tree browser panel (left side).
    #[default]
    Tree,
    /// The detail content panel (right side).
    Detail,
}

// ---------------------------------------------------------------------------
// Catalog provenance rows
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// DetailContent — what the detail panel is showing
// ---------------------------------------------------------------------------

/// Discriminates the content currently displayed in the detail panel.
///
/// The detail panel shows different content depending on what tree node is
/// selected: a full marking detail, catalog metadata rows, or nothing.
///
/// The `Marking` and `DisseminationControl` variants carry a second field
/// containing catalog provenance rows `(label, value)` rendered at the bottom
/// of the detail in subdued `DarkGray` style for attribution without visual
/// competition with the marking content itself.
#[derive(Debug, Clone, Default)]
pub enum DetailContent {
    /// No node selected yet — show a placeholder prompt.
    #[default]
    None,
    /// A marking leaf is selected — show full marking detail + provenance.
    Marking(MarkingDetailData, Vec<(String, String)>),
    /// A dissemination control leaf is selected — show detail + provenance.
    DisseminationControl(MarkingDetailData, Vec<(String, String)>),
    /// A catalog root node is selected — show `_metadata` rows.
    CatalogMetadata(Vec<(String, String)>),
    /// A group branch is selected — show a brief group summary.
    Group {
        name: String,
        count: usize,
    },
}

// ---------------------------------------------------------------------------
// LabelRegistryApp
// ---------------------------------------------------------------------------

/// Application state for the Security Label Registry TUI browser.
///
/// Owns both catalog datasets (US CUI and Canadian Protected) and exposes
/// tree building, detail construction, and marking count queries. The struct
/// is constructed once at startup from the loaded catalog files; all methods
/// are read-only.
///
/// ## Construction
///
/// ```rust,ignore
/// let app = LabelRegistryApp::new(us_catalog, Some(ca_catalog));
/// let tree = app.build_tree();
/// ```
///
/// ## Compliance
///
/// - **NIST SP 800-53 AC-16**: Security Attributes — the app holds the
///   canonical catalog data used in every operator-visible display path.
/// - **NIST SP 800-53 AU-3**: The total marking count exposed by
///   [`total_markings`](LabelRegistryApp::total_markings) appears in the
///   status bar on every rendered frame.
/// - **NIST SP 800-53 AC-3**: No mutation methods — catalogs are read once
///   at startup; the TUI is unconditionally read-only.
pub struct LabelRegistryApp {
    /// US CUI catalog.
    us_catalog: Catalog,
    /// Canadian Protected catalog (optional — may not be configured).
    ca_catalog: Option<Catalog>,
    /// Total marking count across both catalogs (computed once at construction).
    total: usize,
}

impl LabelRegistryApp {
    /// Construct a new `LabelRegistryApp` from the loaded catalog data.
    ///
    /// `ca_catalog` is optional — the registry launches successfully with only
    /// the US catalog, showing a single root node in the tree.
    ///
    /// The total marking count is computed immediately so `total_markings()`
    /// is a cheap O(1) accessor with no allocation.
    ///
    /// NIST SP 800-53 AC-16 — marking count is computed from authoritative
    /// catalog data, not estimated.
    #[must_use = "LabelRegistryApp must be used in the event loop; discarding it has no effect"]
    pub fn new(us_catalog: Catalog, ca_catalog: Option<Catalog>) -> Self {
        let us_count =
            us_catalog.markings.len().saturating_add(us_catalog.dissemination_controls.len());
        let ca_count = ca_catalog.as_ref().map_or(0, |c| c.markings.len());
        let total = us_count.saturating_add(ca_count);
        Self {
            us_catalog,
            ca_catalog,
            total,
        }
    }

    /// Total marking and dissemination control count across all loaded catalogs.
    ///
    /// Displayed in the status bar on every rendered frame.
    #[must_use = "total marking count is required for the status bar"]
    pub const fn total_markings(&self) -> usize {
        self.total
    }

    /// Return the US catalog.
    #[must_use = "US catalog reference is needed for detail construction"]
    pub const fn us_catalog(&self) -> &Catalog {
        &self.us_catalog
    }

    /// Return the Canadian catalog, if loaded.
    #[must_use = "CA catalog reference is needed for detail construction"]
    pub const fn ca_catalog(&self) -> Option<&Catalog> {
        self.ca_catalog.as_ref()
    }

    /// Build the full two-catalog [`TreeModel`] for the viewer state.
    ///
    /// The tree has one root per catalog. Returns a ready-to-use model;
    /// callers should call `state.load_tree(model)` immediately after.
    ///
    /// NIST SP 800-53 AC-16 — the tree faithfully represents every entry
    /// in both catalogs, sorted deterministically by group name and key.
    #[must_use = "TreeModel must be loaded into ViewerState; discarding it leaves the display empty"]
    pub fn build_tree(&self) -> TreeModel {
        let mut model = TreeModel::new();

        // US catalog root
        let us_root = build_catalog_root(&self.us_catalog, true);
        model.roots.push(us_root);

        // Canadian catalog root (if present)
        if let Some(ca) = &self.ca_catalog {
            let ca_root = build_catalog_root(ca, false);
            model.roots.push(ca_root);
        }

        model.rebuild_display();
        model
    }

    /// Build a [`MarkingDetailData`] for the marking identified by `key` in
    /// the US catalog.
    ///
    /// Returns `None` if the key is not present.
    #[must_use = "returns None if the marking key is absent; check before use"]
    pub fn marking_detail_us(&self, key: &str) -> Option<MarkingDetailData> {
        let flag = self.us_catalog.country_flag().unwrap_or_default();
        self.us_catalog.marking(key).map(|m| marking_to_detail(key, m, &flag))
    }

    /// Build a [`MarkingDetailData`] for the marking identified by `key` in
    /// the Canadian catalog.
    ///
    /// Returns `None` if the Canadian catalog is not loaded or the key is absent.
    #[must_use = "returns None if the key is absent or CA catalog not loaded"]
    pub fn marking_detail_ca(&self, key: &str) -> Option<MarkingDetailData> {
        self.ca_catalog.as_ref().and_then(|c| {
            let flag = c.country_flag().unwrap_or_default();
            c.marking(key).map(|m| marking_to_detail(key, m, &flag))
        })
    }

    /// Build a [`MarkingDetailData`] for a dissemination control entry.
    ///
    /// Returns `None` if the key is not present in the US catalog.
    #[must_use = "returns None if the dissemination control key is absent"]
    pub fn dissemination_detail(&self, key: &str) -> Option<MarkingDetailData> {
        let flag = self.us_catalog.country_flag().unwrap_or_default();
        self.us_catalog.dissemination_controls.get(key).map(|dc| dc_to_detail(key, dc, &flag))
    }

    /// Build compact catalog provenance rows for the bottom of the detail panel.
    ///
    /// Returns a `(label, value)` list drawn from the catalog `_metadata` block:
    /// catalog name, version, authority, authority date, author, and last-updated
    /// date. Rendered in subdued style below the marking fields to provide
    /// attribution context without competing with marking content.
    ///
    /// Pass `is_us = true` for the US CUI catalog, `false` for the Canadian
    /// Protected catalog. Returns an empty `Vec` if the requested catalog is not
    /// loaded or has no `_metadata` block.
    ///
    /// NIST SP 800-53 AC-16 — provenance identifies the authoritative source for
    /// each marking definition.
    #[must_use = "provenance rows must be passed to the detail panel renderer"]
    pub fn catalog_provenance(&self, is_us: bool) -> Vec<(String, String)> {
        let cat = if is_us {
            Some(&self.us_catalog)
        } else {
            self.ca_catalog.as_ref()
        };
        let Some(cat) = cat else {
            return Vec::new();
        };
        let Some(meta) = &cat.metadata else {
            return Vec::new();
        };

        let mut rows: Vec<(String, String)> = Vec::with_capacity(6);

        let name = meta.catalog_name.en();
        if !name.is_empty() {
            rows.push(("Catalog".to_owned(), name.to_owned()));
        }
        rows.push(("Version".to_owned(), meta.version.clone()));
        if !meta.authority.is_empty() {
            rows.push(("Authority".to_owned(), meta.authority.clone()));
        }
        if let Some(v) = meta.extra.get("authority_date").and_then(|v| v.as_str()) {
            rows.push(("Authority Date".to_owned(), v.to_owned()));
        }
        if let Some(v) = meta.extra.get("author").and_then(|v| v.as_str()) {
            rows.push(("Author".to_owned(), v.to_owned()));
        }
        if let Some(v) = meta.extra.get("updated").and_then(|v| v.as_str()) {
            rows.push(("Updated".to_owned(), v.to_owned()));
        }

        rows
    }

    /// Build catalog metadata rows for the US catalog root node.
    ///
    /// Returns a list of `(label, value)` pairs suitable for the metadata
    /// display in the detail panel when the catalog root is selected.
    #[must_use = "metadata rows must be passed to the detail panel renderer"]
    pub fn us_catalog_metadata(&self) -> Vec<(String, String)> {
        catalog_metadata_rows(&self.us_catalog)
    }

    /// Build catalog metadata rows for the Canadian catalog root node.
    ///
    /// Returns `None` if the Canadian catalog is not loaded.
    #[must_use = "returns None if CA catalog not loaded"]
    pub fn ca_catalog_metadata(&self) -> Option<Vec<(String, String)>> {
        self.ca_catalog.as_ref().map(catalog_metadata_rows)
    }
}

// ---------------------------------------------------------------------------
// Tree builders (private)
// ---------------------------------------------------------------------------

/// Catalog-root node label from metadata.
fn catalog_root_label(cat: &Catalog) -> String {
    // NOTE: country flag prepending was tested here but felt cluttered since
    // the country name already serves as the organizing header. The flag is
    // shown in the detail panel instead. To re-enable, bind the result below
    // to `name` and use: `cat.country_flag().map_or(name, |f| format!("{f} {name}"))`
    cat.metadata
        .as_ref()
        .map(|m| m.catalog_name.en().to_owned())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "Catalog".to_owned())
}

/// Build the root `TreeNode` for one catalog.
///
/// Top-level layout:
/// 1. Ungrouped markings (index_group is None) directly under root.
/// 2. `"Group: <name>"` branch nodes for each distinct index_group, sorted.
/// 3. `"Dissemination Controls"` branch when the catalog has LDC entries.
fn build_catalog_root(cat: &Catalog, is_us: bool) -> TreeNode {
    let label = catalog_root_label(cat);
    let marking_count = cat.markings.len();
    let dc_count = cat.dissemination_controls.len();
    let detail = format!(
        "{} markings{}",
        marking_count,
        if dc_count > 0 {
            format!(", {dc_count} LDCs")
        } else {
            String::new()
        },
    );

    let mut root = TreeNode::branch(label, detail);
    root.metadata.insert("kind".to_owned(), "catalog_root".to_owned());
    root.metadata.insert(
        "is_us".to_owned(),
        if is_us {
            "1"
        } else {
            "0"
        }
        .to_owned(),
    );
    root.expanded = true;

    // ── Sort markings by index_group (None first), then by key ──────────────
    // Use BTreeMap<Option<String>, Vec<...>> sorted by group name.
    // `None` entries sort before any group name by using an empty-string proxy.
    let mut by_group: BTreeMap<String, Vec<(&String, &Marking)>> = BTreeMap::new();
    for (key, marking) in cat.iter_markings() {
        let group_key = marking.index_group.clone().unwrap_or_default(); // "" = ungrouped, sorts first alphabetically
        by_group.entry(group_key).or_default().push((key, marking));
    }
    for entries in by_group.values_mut() {
        entries.sort_by(|a, b| a.0.cmp(b.0));
    }

    for (group_name, entries) in &by_group {
        if group_name.is_empty() {
            // Ungrouped: add directly under root.
            for (key, marking) in entries {
                root.children.push(marking_leaf(key, marking));
            }
        } else {
            // Named group: a branch node containing sorted leaves.
            let branch_label = group_name.clone();
            let branch_detail = format!("{} markings", entries.len());
            let mut branch = TreeNode::branch(branch_label, branch_detail);
            branch.metadata.insert("kind".to_owned(), "group".to_owned());
            branch.metadata.insert("group_name".to_owned(), group_name.clone());
            branch.metadata.insert("count".to_owned(), entries.len().to_string());
            for (key, marking) in entries {
                branch.children.push(marking_leaf(key, marking));
            }
            root.children.push(branch);
        }
    }

    // ── Dissemination Controls branch (US only) ──────────────────────────────
    if cat.has_dissemination_controls() {
        let dc_branch_label = "Dissemination Controls".to_owned();
        let dc_branch_detail = format!("{dc_count} entries");
        let mut dc_branch = TreeNode::branch(dc_branch_label, dc_branch_detail);
        dc_branch.metadata.insert("kind".to_owned(), "dc_branch".to_owned());

        // Sort by key alphabetically.
        let mut dc_entries: Vec<(&String, &DisseminationControl)> =
            cat.iter_dissemination_controls().collect();
        dc_entries.sort_by(|a, b| a.0.cmp(b.0));

        for (key, dc) in dc_entries {
            let leaf_label = format!(
                "{} - {}",
                dc.portion_marking.as_deref().unwrap_or(key.as_str()),
                dc.name.en(),
            );
            let mut leaf = TreeNode::leaf(leaf_label, String::new());
            leaf.metadata.insert("kind".to_owned(), "dc_leaf".to_owned());
            leaf.metadata.insert("key".to_owned(), key.clone());
            dc_branch.children.push(leaf);
        }

        root.children.push(dc_branch);
    }

    root
}

/// Build a leaf `TreeNode` for a single marking entry.
fn marking_leaf(key: &str, marking: &Marking) -> TreeNode {
    // Leaf label: "<marking_key> - <english_name>" or just the key if no name.
    // Using the full marking key (e.g., "CUI//ADJ") rather than the bare
    // abbreviation makes the tree self-contained: operators can read the banner
    // directly without expanding the detail panel.
    let label = if marking.name.en().is_empty() {
        key.to_owned()
    } else {
        format!("{} - {}", key, marking.name.en())
    };
    let mut leaf = TreeNode::leaf(label, String::new());
    leaf.metadata.insert("kind".to_owned(), "marking_leaf".to_owned());
    leaf.metadata.insert("key".to_owned(), key.to_owned());
    leaf
}

// ---------------------------------------------------------------------------
// Detail builders (private)
// ---------------------------------------------------------------------------

/// Build a [`MarkingDetailData`] from a catalog [`Marking`] entry.
///
/// Maps all available `Marking` fields into the flat owned-string format
/// expected by [`render_marking_detail`].
pub fn marking_to_detail(key: &str, m: &Marking, flag: &str) -> MarkingDetailData {
    let handling = match m.handling_as_str() {
        Some(s) if !s.trim().is_empty() => s.to_owned(),
        _ => m
            .handling_as_object()
            .and_then(|obj| {
                // Try to extract a plain English handling string from
                // a locale-keyed handling object.
                obj.get("en_US")
                    .or_else(|| obj.get("en"))
                    .and_then(|v| v.as_str())
                    .map(str::to_owned)
            })
            .unwrap_or_default(),
    };

    let mut additional: Vec<(String, String)> = Vec::new();

    if let Some(cat_base) = &m.category_base {
        additional.push(("MCS Category Base".to_owned(), cat_base.clone()));
    }
    if let Some(range) = &m.category_range_reserved {
        additional.push(("MCS Range (Reserved)".to_owned(), range.clone()));
    }
    if let Some(ph) = &m.phase_note {
        additional.push(("Phase Note".to_owned(), ph.clone()));
    }
    if let Some(auth) = &m.authority_section {
        additional.push(("Authority Section".to_owned(), auth.clone()));
    }
    if let Some(corr) = &m.us_cui_approximate_correspondence {
        additional.push(("US CUI Approximation".to_owned(), corr.clone()));
    }
    if let Some(domains) = m.risk_domains.as_ref().filter(|d| !d.is_empty()) {
        additional.push(("Risk Domains".to_owned(), domains.join(", ")));
    }

    MarkingDetailData {
        key: key.to_owned(),
        name_en: m.name.en().to_owned(),
        name_fr: m.name.fr().to_owned(),
        abbreviation: m.abbrv_name.clone(),
        designation: m.designation.as_deref().unwrap_or("").to_owned(),
        index_group: m.index_group.as_deref().unwrap_or("").to_owned(),
        level: m.level.as_deref().unwrap_or("").to_owned(),
        description_en: policy_aware_description(m.description.en()),
        description_fr: policy_aware_description(m.description.fr()),
        handling,
        required_warning: m.required_warning_statement.as_deref().unwrap_or("").to_owned(),
        required_dissemination: m
            .required_dissemination_control
            .as_deref()
            .unwrap_or("")
            .to_owned(),
        marking_banner_en: m
            .marking_banner
            .as_ref()
            .map(|lt| lt.en().to_owned())
            .unwrap_or_default(),
        marking_banner_fr: m
            .marking_banner
            .as_ref()
            .map(|lt| lt.fr().to_owned())
            .unwrap_or_default(),
        injury_examples_en: m
            .injury_examples
            .as_ref()
            .map(|lt| lt.en().to_owned())
            .unwrap_or_default(),
        injury_examples_fr: m
            .injury_examples
            .as_ref()
            .map(|lt| lt.fr().to_owned())
            .unwrap_or_default(),
        additional,
        country_flag: flag.to_owned(),
    }
}

/// Build a [`MarkingDetailData`] from a [`DisseminationControl`] entry.
fn dc_to_detail(key: &str, dc: &DisseminationControl, flag: &str) -> MarkingDetailData {
    let mut additional: Vec<(String, String)> = Vec::new();

    if let Some(pm) = &dc.portion_marking {
        additional.push(("Portion Marking".to_owned(), pm.clone()));
    }
    if dc.parameterized {
        additional.push((
            "Parameterized".to_owned(),
            "yes — requires parameter".to_owned(),
        ));
    }
    if let Some(cr) = &dc.category_restriction {
        additional.push(("Category Restriction".to_owned(), cr.clone()));
    }
    if !dc.mutually_exclusive_with.is_empty() {
        additional.push((
            "Mutually Exclusive With".to_owned(),
            dc.mutually_exclusive_with.join(", "),
        ));
    }

    MarkingDetailData {
        key: key.to_owned(),
        name_en: dc.name.en().to_owned(),
        name_fr: dc.name.fr().to_owned(),
        abbreviation: dc.banner_marking.as_deref().unwrap_or(key).to_owned(),
        designation: "LDC".to_owned(),
        description_en: policy_aware_description(dc.description.en()),
        description_fr: policy_aware_description(dc.description.fr()),
        additional,
        country_flag: flag.to_owned(),
        ..MarkingDetailData::default()
    }
}

/// Build metadata display rows for the catalog root detail view.
///
/// Returns a `Vec` of `(label, value)` pairs for the `_metadata` block
/// present in each catalog JSON file.
fn catalog_metadata_rows(cat: &Catalog) -> Vec<(String, String)> {
    let mut rows: Vec<(String, String)> = Vec::with_capacity(8);
    if let Some(meta) = &cat.metadata {
        let name = meta.catalog_name.en();
        if !name.is_empty() {
            rows.push(("Catalog Name".to_owned(), name.to_owned()));
            let fr = meta.catalog_name.fr();
            if !fr.is_empty() {
                rows.push(("Nom du catalogue".to_owned(), fr.to_owned()));
            }
        }
        rows.push(("Version".to_owned(), meta.version.clone()));
        if !meta.authority.is_empty() {
            rows.push(("Authority".to_owned(), meta.authority.clone()));
        }
        if let Some(cc) = &meta.country_code {
            rows.push(("Country Code".to_owned(), cc.clone()));
        }
        if let Some(range) = &meta.mcs_category_range {
            rows.push(("MCS Category Range".to_owned(), range.clone()));
        }

        // Surface any extra string fields, skipping verbose or non-display keys.
        for (k, v) in &meta.extra {
            if k == "scope" || k == "notes" || k == "mcs_ranges" {
                continue;
            }
            if let Some(s) = v.as_str().filter(|s| !s.is_empty()) {
                rows.push((format_extra_key(k), s.to_owned()));
            }
        }
    }
    rows
}

/// Convert a snake_case JSON key to a Title Case display label.
fn format_extra_key(key: &str) -> String {
    key.split('_')
        .map(|word| {
            let mut chars = word.chars();
            chars.next().map_or_else(String::new, |first| {
                let upper: String = first.to_uppercase().collect();
                format!("{upper}{}", chars.as_str())
            })
        })
        .collect::<Vec<_>>()
        .join(" ")
}
