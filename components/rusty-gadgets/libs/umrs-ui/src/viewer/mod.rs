// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams

//! # Viewer — Read-Only Hierarchical Data Browser Pattern
//!
//! Provides the [`ViewerApp`] trait and [`ViewerState`] for building
//! read-only hierarchical data browser TUI tools within the UMRS platform.
//!
//! ## Intended Consumers
//!
//! - `umrs-labels` — browse CUI/classification JSON catalogs in a tree view
//! - `umrs-ls` TUI mode — directory hierarchy browser with SELinux metadata
//!
//! ## Usage Pattern
//!
//! 1. Implement [`ViewerApp`] on your data struct. Provide `tabs()`,
//!    `status()`, `card_title()`, `viewer_header()`, and optionally
//!    `initial_tree()`.
//! 2. Create a [`ViewerState`] with `ViewerState::new(tab_count)`.
//! 3. Populate `state.tree` with your root nodes, then call
//!    `state.tree.rebuild_display()`.
//! 4. Call [`layout::render_viewer`] inside `terminal.draw(...)`.
//! 5. Feed [`crate::keymap::KeyMap`] events into `state.handle_action(...)`.
//!
//! ## Header Context
//!
//! The viewer header is **tool-contextual**, not security-posture. It shows:
//! - Tool name and report title
//! - Data source (catalog name/version or directory path)
//! - Record count / summary statistics
//! - Breadcrumb trail (current position in the hierarchy)
//!
//! Security posture indicators from the kernel are intentionally absent —
//! the viewer pattern is for data browsing, not system assessment.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AU-3**: The viewer header carries tool identification,
//!   data source provenance, and record count on every rendered frame.
//! - **NIST SP 800-53 AC-3**: Tree navigation is read-only; no mutation of
//!   the underlying data is possible through the viewer interface.
//! - **NSA RTB RAIN**: Non-bypassable read-only contract — `ViewerApp` has
//!   no mutation methods; only `ViewerState` mutates (UI state only).

pub mod detail;
pub mod layout;
pub mod tree;

pub use layout::render_viewer;

use crate::app::{StatusMessage, TabDef};
use crate::keymap::Action;
use tree::TreeModel;

// ---------------------------------------------------------------------------
// ViewerHeaderContext
// ---------------------------------------------------------------------------

/// Context for the viewer header panel.
///
/// Tool-contextual: identifies the tool, data source, and record count.
/// Does not include kernel security posture indicators.
///
/// NIST SP 800-53 AU-3 — identification fields ensure every rendered frame
/// carries sufficient context for independent review.
#[derive(Debug, Clone)]
pub struct ViewerHeaderContext {
    /// Name of the tool or report (e.g., `"umrs-labels"` or `"CUI Catalog Browser"`).
    pub tool_name: String,

    /// Data source description (e.g., `"US CUI Catalog v0.1.0"` or `/srv/labels`).
    pub data_source: String,

    /// Total number of top-level records in the catalog or dataset.
    pub record_count: usize,

    /// Optional one-line summary description (e.g., `"15 categories, 127 subcategories"`).
    pub summary_description: Option<String>,
}

impl ViewerHeaderContext {
    /// Construct a `ViewerHeaderContext` with a tool name, data source, and count.
    ///
    /// `summary_description` defaults to `None`.
    #[must_use = "ViewerHeaderContext must be returned from viewer_header(); discarding it leaves the header empty"]
    pub fn new(
        tool_name: impl Into<String>,
        data_source: impl Into<String>,
        record_count: usize,
    ) -> Self {
        Self {
            tool_name: tool_name.into(),
            data_source: data_source.into(),
            record_count,
            summary_description: None,
        }
    }

    /// Set the optional summary description.
    #[must_use = "returns the modified context; the original is consumed"]
    pub fn with_summary(mut self, description: impl Into<String>) -> Self {
        self.summary_description = Some(description.into());
        self
    }
}

// ---------------------------------------------------------------------------
// ViewerApp trait
// ---------------------------------------------------------------------------

/// Trait for read-only hierarchical data browser TUI tools.
///
/// Implement this trait on your data struct to plug into the viewer layout.
/// The trait is object-safe — `render_viewer` accepts `&dyn ViewerApp`.
///
/// ## Invariants
///
/// - All returned data must be display-ready (already formatted for terminal output).
/// - `tabs()` must return at least one entry. An empty tab list is invalid.
/// - `viewer_header()` must return a `ViewerHeaderContext` with a non-empty `tool_name`.
///
/// NIST SP 800-53 AU-3 — identification fields are mandatory.
/// NIST SP 800-53 AC-3 — the trait provides no mutation operations;
/// the viewer is unconditionally read-only.
/// NSA RTB RAIN — the read-only contract is enforced at the trait level;
/// callers cannot bypass it through the trait interface.
pub trait ViewerApp {
    /// The title displayed in the header border.
    ///
    /// Used as the block title string (e.g., `"UMRS Label Viewer"`).
    fn card_title(&self) -> &str;

    /// The tab definitions for this viewer.
    ///
    /// Must return at least one entry. Each tab corresponds to a different
    /// view or level of the data hierarchy.
    ///
    /// NIST SP 800-53 AU-3 — the active tab label communicates current
    /// view context in every rendered frame.
    fn tabs(&self) -> &[TabDef];

    /// The current status message for the status bar.
    ///
    /// Callers update this to reflect filter results, loading state, or
    /// navigation feedback. The level controls the status bar background color.
    ///
    /// NIST SP 800-53 AU-3 — status is always visible and typed.
    fn status(&self) -> StatusMessage;

    /// The header context for the viewer header panel.
    ///
    /// Called once per frame. Keep the implementation cheap — avoid I/O here.
    ///
    /// NIST SP 800-53 AU-3 — header context carries tool identity and
    /// data source provenance.
    fn viewer_header(&self) -> ViewerHeaderContext;

    /// Optional: provide an initial tree model to pre-populate `ViewerState`.
    ///
    /// If `Some`, the viewer runtime will load this tree into `state.tree`
    /// after calling `rebuild_display()`. If `None`, the caller is responsible
    /// for populating `state.tree` before the first render.
    ///
    /// Returning `Some` is the preferred pattern for tools with static data
    /// sources (catalog files loaded at startup).
    fn initial_tree(&self) -> Option<TreeModel> {
        None
    }
}

// ---------------------------------------------------------------------------
// ViewerState
// ---------------------------------------------------------------------------

/// Mutable UI state for a [`ViewerApp`] session.
///
/// Owns the tree model, scroll/selection state, and search state. Separate
/// from the app data struct so the event loop can mutate state while holding
/// an immutable reference to the app.
///
/// NIST SP 800-53 AU-3 — breadcrumb and selection state provide navigation
/// context that is rendered in every frame.
/// NSA RTB — state mutations are gated by action variants; no direct field
/// writes are possible through the public API (fields are accessible for
/// initialization only).
pub struct ViewerState {
    /// The tree data model (owned by state, populated by the caller).
    pub tree: TreeModel,

    /// Currently selected index in `tree.display_list`.
    pub selected_index: usize,

    /// Vertical scroll offset (for future use — ratatui `List` manages its
    /// own scroll via `ListState`; this is retained for custom scroll logic).
    pub scroll_offset: usize,

    /// Active tab index.
    pub active_tab: usize,

    /// Total number of tabs (used for wrap-around navigation).
    tab_count: usize,

    /// Breadcrumb trail — labels of ancestor nodes from root to the
    /// currently selected node, in order.
    breadcrumb: Vec<String>,

    /// Whether the search bar is currently active.
    pub search_active: bool,

    /// Current search query string (accumulated character by character).
    pub search_query: String,

    /// Signal to the event loop that the application should terminate.
    pub should_quit: bool,
}

impl ViewerState {
    /// Construct a new `ViewerState` with an empty tree and `tab_count` tabs.
    ///
    /// `selected_index` starts at 0. `search_active` starts `false`.
    #[must_use = "ViewerState must be used in the event loop; constructing and discarding it has no effect"]
    pub fn new(tab_count: usize) -> Self {
        Self {
            tree: TreeModel::new(),
            selected_index: 0,
            scroll_offset: 0,
            active_tab: 0,
            tab_count: tab_count.max(1),
            breadcrumb: Vec::new(),
            search_active: false,
            search_query: String::new(),
            should_quit: false,
        }
    }

    /// Load a tree model into state, rebuilding the display list.
    ///
    /// Call this once after the initial data load.
    pub fn load_tree(&mut self, model: TreeModel) {
        self.tree = model;
        self.tree.rebuild_display();
        self.selected_index = 0;
        self.breadcrumb.clear();
    }

    /// Return the breadcrumb trail as a displayable string.
    ///
    /// If the breadcrumb is empty, returns `"/"` (root).
    #[must_use = "breadcrumb string is used for rendering; discarding it has no visual effect"]
    pub fn breadcrumb_display(&self) -> String {
        if self.breadcrumb.is_empty() {
            "/".to_owned()
        } else {
            format!("/ {}", self.breadcrumb.join(" > "))
        }
    }

    /// Handle an [`Action`] and update state accordingly.
    ///
    /// Returns `true` if the caller should re-render (state changed).
    /// Returns `false` for no-op actions.
    ///
    /// Navigation actions (`ScrollUp`, `ScrollDown`, `Expand`, `Collapse`,
    /// `Back`, `NextTab`, `PrevTab`) update state. `Search` activates the
    /// search bar. `Quit` sets `should_quit`. Character input while in
    /// search mode must be handled by the calling event loop (not through
    /// this method).
    ///
    /// NIST SP 800-53 AC-3 — no action variant produces a data mutation;
    /// only UI state is updated.
    #[must_use = "return value indicates whether a re-render is needed"]
    pub fn handle_action(&mut self, action: Action) -> bool {
        match action {
            Action::Quit => {
                self.should_quit = true;
                true
            }
            Action::NextTab => {
                self.active_tab = (self.active_tab + 1) % self.tab_count;
                true
            }
            Action::PrevTab => {
                self.active_tab = self
                    .active_tab
                    .checked_sub(1)
                    .unwrap_or(self.tab_count - 1);
                true
            }
            Action::ScrollUp => {
                if self.search_active {
                    // In search mode, Up navigates results — not implemented
                    // yet; consume the action without updating.
                    false
                } else {
                    self.move_selection_up()
                }
            }
            Action::ScrollDown => {
                if self.search_active {
                    false
                } else {
                    self.move_selection_down()
                }
            }
            Action::PageUp => {
                self.selected_index = self.selected_index.saturating_sub(10);
                self.update_breadcrumb();
                true
            }
            Action::PageDown => {
                let max = self.tree.display_count().saturating_sub(1);
                self.selected_index =
                    self.selected_index.saturating_add(10).min(max);
                self.update_breadcrumb();
                true
            }
            Action::Expand => {
                self.expand_selected();
                true
            }
            Action::Collapse => {
                self.collapse_selected();
                true
            }
            Action::Search => {
                self.search_active = true;
                self.search_query.clear();
                true
            }
            Action::Back => {
                self.navigate_back();
                true
            }
            Action::DialogCancel => {
                // Escape dismisses search mode.
                if self.search_active {
                    self.search_active = false;
                    self.search_query.clear();
                    self.tree.clear_filter();
                    self.tree.rebuild_display();
                    self.selected_index = 0;
                    true
                } else {
                    false
                }
            }
            Action::DialogConfirm => {
                // Enter in search mode commits the query; in normal mode it expands.
                if self.search_active {
                    self.search_active = false;
                } else {
                    self.expand_selected();
                }
                true
            }
            // Actions not relevant to viewer mode.
            Action::Refresh
            | Action::DialogToggleFocus
            | Action::ShowHelp
            | Action::Save
            | Action::Discard
            | Action::ToggleEdit => false,
        }
    }

    /// Append a character to the search query and re-apply the filter.
    ///
    /// Call this from the event loop when `search_active` is `true` and a
    /// printable character event arrives.
    pub fn push_search_char(&mut self, ch: char) {
        self.search_query.push(ch);
        self.tree.apply_filter(&self.search_query);
        self.tree.rebuild_display();
        self.selected_index = 0;
    }

    /// Remove the last character from the search query and re-apply the filter.
    pub fn pop_search_char(&mut self) {
        self.search_query.pop();
        self.tree.apply_filter(&self.search_query);
        self.tree.rebuild_display();
        self.selected_index = 0;
    }

    // -----------------------------------------------------------------------
    // Private helpers
    // -----------------------------------------------------------------------

    fn move_selection_up(&mut self) -> bool {
        if self.selected_index > 0 {
            self.selected_index -= 1;
            self.update_breadcrumb();
            true
        } else {
            false
        }
    }

    fn move_selection_down(&mut self) -> bool {
        let max = self.tree.display_count().saturating_sub(1);
        if self.selected_index < max {
            self.selected_index += 1;
            self.update_breadcrumb();
            true
        } else {
            false
        }
    }

    fn expand_selected(&mut self) {
        if let Some(entry) = self.tree.display_list.get(self.selected_index) {
            let path = entry.path.clone();
            self.tree.expand(&path);
            self.tree.rebuild_display();
            self.update_breadcrumb();
        }
    }

    fn collapse_selected(&mut self) {
        if let Some(entry) = self.tree.display_list.get(self.selected_index) {
            let path = entry.path.clone();
            self.tree.collapse(&path);
            self.tree.rebuild_display();
            self.update_breadcrumb();
        }
    }

    fn navigate_back(&mut self) {
        // Navigate to the parent of the currently selected node.
        if let Some(entry) = self.tree.display_list.get(self.selected_index) {
            let path = entry.path.clone();
            if path.len() > 1 {
                let parent_path = path[..path.len() - 1].to_vec();
                // Find the display list index of the parent.
                if let Some(parent_idx) = self
                    .tree
                    .display_list
                    .iter()
                    .position(|e| e.path == parent_path)
                {
                    self.selected_index = parent_idx;
                    self.update_breadcrumb();
                }
            }
        }
    }

    /// Rebuild the breadcrumb from the currently selected display entry.
    fn update_breadcrumb(&mut self) {
        self.breadcrumb.clear();
        if let Some(entry) = self.tree.display_list.get(self.selected_index) {
            let path = entry.path.clone();
            // Walk the path, collecting labels.
            let mut current_children = &self.tree.roots;
            for &idx in &path {
                if let Some(node) = current_children.get(idx) {
                    self.breadcrumb.push(node.label.clone());
                    current_children = &node.children;
                } else {
                    break;
                }
            }
        }
    }
}
