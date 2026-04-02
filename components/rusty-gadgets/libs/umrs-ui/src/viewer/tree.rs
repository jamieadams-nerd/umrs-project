// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams

//! # Tree — Hierarchical Data Model for ViewerApp
//!
//! Provides the core tree data model used by [`super::ViewerApp`]. The model
//! supports arbitrarily nested nodes, lazy expansion state, filtering, and
//! a flat display list that the layout renderer can iterate without
//! recursion.
//!
//! ## Design
//!
//! `TreeNode` owns its children. `TreeModel` owns the root nodes and
//! maintains a flat `display_list` (a `Vec` of node path references) that is
//! rebuilt whenever the expansion state or filter changes. The renderer
//! iterates the display list — it never traverses the tree itself.
//!
//! Paths are `Vec<usize>` index sequences from the root, used as stable node
//! identifiers. A path of `[1, 0, 3]` means "root node at index 1, its
//! first child, then that child's fourth child."
//!
//! ## Filtering
//!
//! When a search query is active, `TreeModel::apply_filter` marks nodes as
//! hidden/visible and re-builds the display list showing only the match
//! subtree. Ancestors of matches are always shown (so the operator can see
//! where the match lives in the hierarchy). The filter is case-insensitive
//! on the node label.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AU-3**: Each node carries a label and optional
//!   metadata fields; the tree preserves provenance of every item displayed.
//! - **NSA RTB**: Fail-closed design — an out-of-range path lookup returns
//!   `None`, never panics or produces a misleading result.

use std::collections::BTreeMap;

// ---------------------------------------------------------------------------
// NodeId — stable path-based identifier
// ---------------------------------------------------------------------------

/// A path-based node identifier: an ordered sequence of child indices from
/// the tree root to the target node.
///
/// Paths are stable as long as the tree topology does not change. Mutations
/// (add/remove node) invalidate all existing paths that overlap with the
/// changed subtree. For read-only viewer use, paths are stable for the
/// lifetime of the data source.
///
/// NIST SP 800-53 AU-3 — nodes are identified by a deterministic path
/// rather than a runtime-assigned opaque ID, making audit trails reproducible.
pub type NodeId = Vec<usize>;

// ---------------------------------------------------------------------------
// TreeNode
// ---------------------------------------------------------------------------

/// A single node in the hierarchical data tree.
///
/// Nodes may have zero or more children. Leaf nodes (`children.is_empty()`)
/// cannot be expanded. Branch nodes can be expanded (showing children) or
/// collapsed (hiding children). The `expanded` flag persists across filter
/// changes.
///
/// NIST SP 800-53 AU-3 — the `label`, `detail`, and `metadata` fields carry
/// audit-relevant identification for each record in the browsed catalog.
#[derive(Debug, Clone)]
pub struct TreeNode {
    /// Display label shown in the tree column.
    pub label: String,

    /// Short detail string shown in the tree row to the right of the label
    /// (e.g., a code, count, or value summary). Empty string if unused.
    pub detail: String,

    /// Whether this node is currently expanded (children visible).
    /// Leaf nodes ignore this field — it is kept at `false` for leaves.
    pub expanded: bool,

    /// Child nodes (empty for leaves).
    pub children: Vec<Self>,

    /// Arbitrary key-value metadata shown in the detail panel when this
    /// node is selected. Keys are ordered for stable display.
    pub metadata: BTreeMap<String, String>,

    /// Whether this node is currently visible after filter application.
    /// `true` by default (no filter active). Set to `false` by
    /// [`TreeModel::apply_filter`] for nodes that do not match the query
    /// and whose subtree contains no matches.
    pub visible: bool,
}

impl TreeNode {
    /// Construct a branch node with a label, optional detail, and children.
    ///
    /// `expanded` defaults to `false` (collapsed). `metadata` is empty.
    /// Call `node.metadata.insert(k, v)` to add detail panel entries.
    #[must_use = "TreeNode must be inserted into a tree; constructing and discarding it has no effect"]
    pub fn branch(label: impl Into<String>, detail: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            detail: detail.into(),
            expanded: false,
            children: Vec::new(),
            metadata: BTreeMap::new(),
            visible: true,
        }
    }

    /// Construct a leaf node with a label and optional detail string.
    ///
    /// `children` is always empty on leaf nodes.
    #[must_use = "TreeNode must be inserted into a tree; constructing and discarding it has no effect"]
    pub fn leaf(label: impl Into<String>, detail: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            detail: detail.into(),
            expanded: false,
            children: Vec::new(),
            metadata: BTreeMap::new(),
            visible: true,
        }
    }

    /// Return `true` if this node has no children (is a leaf).
    #[must_use = "leaf status determines whether Expand/Collapse actions are valid"]
    pub const fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    /// Return a depth-aware display indent prefix for rendering.
    ///
    /// Each level adds two spaces. The connecting character differs between
    /// expanded branches (`▼`), collapsed branches (`▶`), and leaves (`·`).
    #[must_use = "prefix is used for rendering; discarding it has no visual effect"]
    pub fn indent_prefix(&self, depth: usize) -> String {
        let pad = "  ".repeat(depth);
        if self.is_leaf() {
            format!("{pad}  · ")
        } else if self.expanded {
            format!("{pad}▼ ")
        } else {
            format!("{pad}▶ ")
        }
    }
}

// ---------------------------------------------------------------------------
// DisplayEntry — flat render list item
// ---------------------------------------------------------------------------

/// An entry in the flat display list produced by [`TreeModel::rebuild_display`].
///
/// The renderer iterates `TreeModel::display_list` to produce the tree column.
/// Each entry carries everything needed to render one row without traversing
/// the tree again.
#[derive(Debug, Clone)]
pub struct DisplayEntry {
    /// Stable path identifying this node.
    pub path: NodeId,

    /// Depth in the tree (root nodes are depth 0).
    pub depth: usize,

    /// Cached indent prefix (avoids recomputing during render).
    pub prefix: String,

    /// Display label for this node.
    pub label: String,

    /// Short detail string (may be empty).
    pub detail: String,

    /// Whether the node can be expanded (has children and is currently collapsed).
    pub expandable: bool,

    /// Whether the node is currently expanded.
    pub expanded: bool,
}

// ---------------------------------------------------------------------------
// TreeModel
// ---------------------------------------------------------------------------

/// Owns the root nodes and the flat display list used by the renderer.
///
/// Call [`TreeModel::rebuild_display`] after any topology or expansion
/// change. The renderer reads only `display_list` and `selected_index`.
///
/// NIST SP 800-53 AU-3 — the model carries all identification needed to
/// render a complete, self-describing catalog view.
/// NSA RTB — fail-closed: out-of-bounds path lookups return `None`.
pub struct TreeModel {
    /// Root-level nodes.
    pub roots: Vec<TreeNode>,

    /// Flat ordered list of visible nodes, rebuilt after state changes.
    pub display_list: Vec<DisplayEntry>,
}

impl TreeModel {
    /// Construct an empty model. Call `roots.push(...)` then `rebuild_display()`.
    #[must_use = "TreeModel must be populated and rebuilt before rendering"]
    pub const fn new() -> Self {
        Self {
            roots: Vec::new(),
            display_list: Vec::new(),
        }
    }

    /// Rebuild the flat display list from the current tree state.
    ///
    /// Call after any of: node added/removed, expansion state changed,
    /// filter applied/cleared. The renderer reads `display_list` directly —
    /// the caller is responsible for calling `rebuild_display` at the right
    /// moments.
    pub fn rebuild_display(&mut self) {
        self.display_list.clear();
        let roots = std::mem::take(&mut self.roots);
        for (i, node) in roots.iter().enumerate() {
            collect_entries(node, &vec![i], 0, &mut self.display_list);
        }
        self.roots = roots;
    }

    /// Resolve a node path to a mutable reference.
    ///
    /// Returns `None` if any index in the path is out of bounds (fail-closed).
    ///
    /// NIST SP 800-53 AU-3 / NSA RTB — bounds-safe lookup; panics are
    /// prohibited in the viewer data model.
    #[must_use = "the returned node reference is needed to mutate expansion state"]
    pub fn node_mut(&mut self, path: &[usize]) -> Option<&mut TreeNode> {
        let (first, rest) = path.split_first()?;
        let mut node = self.roots.get_mut(*first)?;
        for &idx in rest {
            node = node.children.get_mut(idx)?;
        }
        Some(node)
    }

    /// Resolve a node path to an immutable reference.
    ///
    /// Returns `None` if any index in the path is out of bounds.
    #[must_use = "the returned node reference is needed for detail panel rendering"]
    pub fn node_ref(&self, path: &[usize]) -> Option<&TreeNode> {
        let (first, rest) = path.split_first()?;
        let mut node = self.roots.get(*first)?;
        for &idx in rest {
            node = node.children.get(idx)?;
        }
        Some(node)
    }

    /// Toggle the expansion state of the node at `path`.
    ///
    /// Leaf nodes are silently ignored (fail-closed). After toggling,
    /// callers must call `rebuild_display()` to update the display list.
    ///
    /// NIST SP 800-53 AU-3 — expansion state change is always operator-driven;
    /// no automatic expansion occurs without an explicit `Expand` action.
    pub fn toggle_expansion(&mut self, path: &[usize]) {
        if let Some(node) = self.node_mut(path) {
            if !node.is_leaf() {
                node.expanded = !node.expanded;
            }
        }
    }

    /// Expand the node at `path`.
    ///
    /// Leaf nodes are silently ignored.
    pub fn expand(&mut self, path: &[usize]) {
        if let Some(node) = self.node_mut(path) {
            if !node.is_leaf() {
                node.expanded = true;
            }
        }
    }

    /// Collapse the node at `path`.
    pub fn collapse(&mut self, path: &[usize]) {
        if let Some(node) = self.node_mut(path) {
            node.expanded = false;
        }
    }

    /// Apply a case-insensitive filter to the tree.
    ///
    /// Nodes whose `label` contains `query` (case-insensitive) are marked
    /// visible. Ancestors of matching nodes are also marked visible so the
    /// match can be located in context. Non-matching nodes with no matching
    /// descendants are marked hidden.
    ///
    /// After filtering, callers must call `rebuild_display()`.
    ///
    /// Passing an empty `query` clears the filter (all nodes become visible).
    ///
    /// NIST SP 800-53 AU-3 — filter operates on label text only (display
    /// data), never on classification or metadata values that may be CUI.
    pub fn apply_filter(&mut self, query: &str) {
        if query.is_empty() {
            reset_visibility(&mut self.roots);
            return;
        }
        let q = query.to_ascii_lowercase();
        mark_visible(&mut self.roots, &q);
    }

    /// Clear the active filter and restore all nodes to visible.
    pub fn clear_filter(&mut self) {
        reset_visibility(&mut self.roots);
    }

    /// Return the total number of entries in the current display list.
    #[must_use = "display count is used to bounds-check the selected index"]
    pub const fn display_count(&self) -> usize {
        self.display_list.len()
    }
}

impl Default for TreeModel {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Private helpers
// ---------------------------------------------------------------------------

/// Recursively collect visible nodes into the flat display list.
fn collect_entries(node: &TreeNode, path: &NodeId, depth: usize, out: &mut Vec<DisplayEntry>) {
    if !node.visible {
        return;
    }
    let prefix = node.indent_prefix(depth);
    out.push(DisplayEntry {
        path: path.clone(),
        depth,
        prefix,
        label: node.label.clone(),
        detail: node.detail.clone(),
        expandable: !node.is_leaf() && !node.expanded,
        expanded: node.expanded,
    });
    if node.expanded {
        for (i, child) in node.children.iter().enumerate() {
            let mut child_path = path.clone();
            child_path.push(i);
            collect_entries(child, &child_path, depth + 1, out);
        }
    }
}

/// Recursively reset all nodes to visible (no filter active).
fn reset_visibility(nodes: &mut [TreeNode]) {
    for node in nodes.iter_mut() {
        node.visible = true;
        reset_visibility(&mut node.children);
    }
}

/// Recursively mark nodes visible if they or any descendant matches `query`.
///
/// Returns `true` if this subtree contains any match.
fn mark_visible(nodes: &mut [TreeNode], query: &str) -> bool {
    let mut any_match = false;
    for node in nodes.iter_mut() {
        let self_match = node.label.to_ascii_lowercase().contains(query)
            || node.detail.to_ascii_lowercase().contains(query);
        let child_match = mark_visible(&mut node.children, query);
        let visible = self_match || child_match;
        node.visible = visible;
        if visible {
            any_match = true;
        }
    }
    any_match
}
