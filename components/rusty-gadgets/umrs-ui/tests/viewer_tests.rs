// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams

//! Integration tests for the ViewerApp pattern.
//!
//! Covers: tree model construction, expand/collapse, filter/search,
//! ViewerState action handling, and breadcrumb display.

use umrs_ui::viewer::tree::{TreeModel, TreeNode};
use umrs_ui::viewer::ViewerState;
use umrs_ui::Action;

// ---------------------------------------------------------------------------
// Tree construction helpers
// ---------------------------------------------------------------------------

/// Build a small test tree:
/// ```text
/// Root A
///   ├── Child A1
///   └── Child A2
///         └── Grandchild A2a
/// Root B (leaf)
/// ```
fn build_test_tree() -> TreeModel {
    let mut model = TreeModel::new();

    let mut root_a = TreeNode::branch("Root A", "branch");
    let child_a1 = TreeNode::leaf("Child A1", "leaf");
    let mut child_a2 = TreeNode::branch("Child A2", "branch");
    let grandchild = TreeNode::leaf("Grandchild A2a", "leaf");
    child_a2.children.push(grandchild);
    root_a.children.push(child_a1);
    root_a.children.push(child_a2);

    let root_b = TreeNode::leaf("Root B", "leaf");

    model.roots.push(root_a);
    model.roots.push(root_b);
    model
}

// ---------------------------------------------------------------------------
// TreeNode tests
// ---------------------------------------------------------------------------

#[test]
fn tree_node_leaf_has_no_children() {
    let node = TreeNode::leaf("label", "detail");
    assert!(node.is_leaf(), "leaf node must report is_leaf() = true");
    assert!(node.children.is_empty());
}

#[test]
fn tree_node_branch_is_not_leaf() {
    let mut branch = TreeNode::branch("label", "");
    branch.children.push(TreeNode::leaf("child", ""));
    assert!(!branch.is_leaf());
}

#[test]
fn tree_node_leaf_indent_prefix_contains_dot() {
    let leaf = TreeNode::leaf("l", "");
    let prefix = leaf.indent_prefix(0);
    assert!(
        prefix.contains('\u{00B7}'),
        "leaf indent prefix should contain '·', got: {prefix:?}"
    );
}

#[test]
fn tree_node_branch_collapsed_prefix_contains_arrow() {
    // A branch must have at least one child for is_leaf() to return false.
    let mut branch = TreeNode::branch("b", "");
    branch.children.push(TreeNode::leaf("child", ""));
    let prefix = branch.indent_prefix(0);
    assert!(
        prefix.contains('\u{25B6}'),
        "collapsed branch prefix should contain '▶', got: {prefix:?}"
    );
}

#[test]
fn tree_node_branch_expanded_prefix_contains_down_arrow() {
    // A branch must have at least one child for is_leaf() to return false.
    let mut branch = TreeNode::branch("b", "");
    branch.children.push(TreeNode::leaf("child", ""));
    branch.expanded = true;
    let prefix = branch.indent_prefix(0);
    assert!(
        prefix.contains('\u{25BC}'),
        "expanded branch prefix should contain '▼', got: {prefix:?}"
    );
}

#[test]
fn tree_node_indent_increases_with_depth() {
    let leaf = TreeNode::leaf("l", "");
    let p0 = leaf.indent_prefix(0);
    let p1 = leaf.indent_prefix(1);
    let p2 = leaf.indent_prefix(2);
    assert!(p1.len() > p0.len(), "depth 1 should be wider than depth 0");
    assert!(p2.len() > p1.len(), "depth 2 should be wider than depth 1");
}

// ---------------------------------------------------------------------------
// TreeModel — rebuild_display (collapsed)
// ---------------------------------------------------------------------------

#[test]
fn rebuild_display_collapsed_shows_only_roots() {
    let mut model = build_test_tree();
    model.rebuild_display();

    // Both roots are visible; children are hidden (not expanded).
    assert_eq!(
        model.display_list.len(),
        2,
        "collapsed tree should show exactly 2 root entries"
    );
    assert_eq!(model.display_list[0].label, "Root A");
    assert_eq!(model.display_list[1].label, "Root B");
}

#[test]
fn rebuild_display_depth_is_zero_for_roots() {
    let mut model = build_test_tree();
    model.rebuild_display();
    for entry in &model.display_list {
        assert_eq!(
            entry.depth, 0,
            "root entries should have depth 0 (all collapsed)"
        );
    }
}

// ---------------------------------------------------------------------------
// TreeModel — expand and collapse
// ---------------------------------------------------------------------------

#[test]
fn expand_root_shows_children() {
    let mut model = build_test_tree();
    model.rebuild_display();

    // Expand root A (path [0]).
    model.expand(&[0]);
    model.rebuild_display();

    // Root A + Child A1 + Child A2 + Root B = 4 entries.
    assert_eq!(
        model.display_list.len(),
        4,
        "expanding root A should add 2 children"
    );
}

#[test]
fn collapse_reverses_expand() {
    let mut model = build_test_tree();
    model.rebuild_display();

    model.expand(&[0]);
    model.rebuild_display();
    model.collapse(&[0]);
    model.rebuild_display();

    assert_eq!(
        model.display_list.len(),
        2,
        "collapsing root A should return to 2 root entries"
    );
}

#[test]
fn expand_leaf_is_noop() {
    let mut model = build_test_tree();
    model.rebuild_display();

    // Root B is a leaf (path [1]).
    model.expand(&[1]);
    model.rebuild_display();

    assert_eq!(
        model.display_list.len(),
        2,
        "expanding a leaf must not change the display list"
    );
}

#[test]
fn toggle_expansion_works() {
    let mut model = build_test_tree();
    model.rebuild_display();

    model.toggle_expansion(&[0]);
    model.rebuild_display();
    assert_eq!(model.display_list.len(), 4);

    model.toggle_expansion(&[0]);
    model.rebuild_display();
    assert_eq!(model.display_list.len(), 2);
}

#[test]
fn expand_nested_child_shows_grandchild() {
    let mut model = build_test_tree();

    // Expand root A, then Child A2.
    model.expand(&[0]);
    model.rebuild_display();
    model.expand(&[0, 1]);
    model.rebuild_display();

    // Root A + A1 + A2 + Grandchild A2a + Root B = 5.
    assert_eq!(model.display_list.len(), 5);
    let grandchild = &model.display_list[3];
    assert_eq!(grandchild.label, "Grandchild A2a");
    assert_eq!(grandchild.depth, 2);
}

// ---------------------------------------------------------------------------
// TreeModel — node_ref / node_mut
// ---------------------------------------------------------------------------

#[test]
fn node_ref_valid_path_returns_node() {
    let model = build_test_tree();
    let node = model.node_ref(&[0]).expect("node at path [0] must exist");
    assert_eq!(node.label, "Root A");
}

#[test]
fn node_ref_invalid_path_returns_none() {
    let model = build_test_tree();
    let node = model.node_ref(&[99]);
    assert!(node.is_none(), "out-of-bounds path must return None (fail-closed)");
}

#[test]
fn node_ref_nested_path() {
    let model = build_test_tree();
    let node = model
        .node_ref(&[0, 1, 0])
        .expect("node at path [0,1,0] must exist");
    assert_eq!(node.label, "Grandchild A2a");
}

// ---------------------------------------------------------------------------
// TreeModel — filter
// ---------------------------------------------------------------------------

#[test]
fn apply_filter_hides_non_matching_nodes() {
    let mut model = build_test_tree();
    model.apply_filter("grandchild");
    model.rebuild_display();

    let visible: Vec<&str> = model
        .display_list
        .iter()
        .map(|e| e.label.as_str())
        .collect();
    assert!(
        visible.contains(&"Root A"),
        "Root A must be visible as ancestor of match"
    );
    assert!(
        !visible.contains(&"Root B"),
        "Root B must be hidden (no match)"
    );
}

#[test]
fn apply_filter_empty_string_restores_all() {
    let mut model = build_test_tree();
    model.apply_filter("nothing_matches_this_string_xyz");
    model.rebuild_display();

    model.apply_filter("");
    model.rebuild_display();
    assert_eq!(
        model.display_list.len(),
        2,
        "clearing filter must restore all roots"
    );
}

#[test]
fn clear_filter_restores_all_visible() {
    let mut model = build_test_tree();
    model.apply_filter("xyz_no_match");
    model.rebuild_display();
    model.clear_filter();
    model.rebuild_display();

    assert_eq!(model.display_list.len(), 2);
}

// ---------------------------------------------------------------------------
// TreeModel — metadata on nodes
// ---------------------------------------------------------------------------

#[test]
fn node_metadata_is_stored_and_retrievable() {
    let mut model = TreeModel::new();
    let mut leaf = TreeNode::leaf("label", "detail");
    leaf.metadata.insert("key1".to_owned(), "value1".to_owned());
    leaf.metadata.insert("key2".to_owned(), "value2".to_owned());
    model.roots.push(leaf);
    model.rebuild_display();

    let node = model.node_ref(&[0]).expect("leaf node at path [0] must exist");
    assert_eq!(node.metadata.get("key1").map(String::as_str), Some("value1"));
    assert_eq!(node.metadata.get("key2").map(String::as_str), Some("value2"));
}

// ---------------------------------------------------------------------------
// ViewerState — construction
// ---------------------------------------------------------------------------

#[test]
fn viewer_state_new_defaults() {
    let state = ViewerState::new(3);
    assert_eq!(state.selected_index, 0);
    assert_eq!(state.active_tab, 0);
    assert!(!state.search_active);
    assert!(state.search_query.is_empty());
    assert!(!state.should_quit);
}

#[test]
fn viewer_state_tab_count_minimum_one() {
    let mut state = ViewerState::new(0);
    // tab_count of 0 is coerced to 1; NextTab wraps correctly.
    let changed = state.handle_action(Action::NextTab);
    assert!(changed);
    assert_eq!(state.active_tab, 0, "wrapping tab_count=1 must stay at 0");
}

// ---------------------------------------------------------------------------
// ViewerState — navigation actions
// ---------------------------------------------------------------------------

#[test]
fn scroll_down_increments_selected_index() {
    let mut state = ViewerState::new(1);
    state.load_tree({
        let mut m = build_test_tree();
        m.expand(&[0]);
        m
    });
    let initial = state.selected_index;
    let changed = state.handle_action(Action::ScrollDown);
    assert!(changed);
    assert_eq!(state.selected_index, initial + 1);
}

#[test]
fn scroll_up_at_zero_is_noop() {
    let mut state = ViewerState::new(1);
    state.load_tree(build_test_tree());
    assert_eq!(state.selected_index, 0);
    let changed = state.handle_action(Action::ScrollUp);
    assert!(!changed, "ScrollUp at index 0 must report no change");
    assert_eq!(state.selected_index, 0);
}

#[test]
fn scroll_down_at_end_is_noop() {
    let mut state = ViewerState::new(1);
    state.load_tree(build_test_tree()); // 2 entries (collapsed)
    let _ = state.handle_action(Action::ScrollDown);
    // Now at index 1 (last entry).
    let changed = state.handle_action(Action::ScrollDown);
    assert!(!changed, "ScrollDown at last entry must report no change");
    assert_eq!(state.selected_index, 1);
}

#[test]
fn page_up_saturates_at_zero() {
    let mut state = ViewerState::new(1);
    state.load_tree(build_test_tree());
    let _ = state.handle_action(Action::ScrollDown); // move to index 1
    let _ = state.handle_action(Action::PageUp); // jump -10, should saturate at 0
    assert_eq!(state.selected_index, 0);
}

#[test]
fn page_down_saturates_at_end() {
    let mut state = ViewerState::new(1);
    state.load_tree(build_test_tree()); // 2 entries
    let _ = state.handle_action(Action::PageDown); // jump +10, should saturate at 1
    assert_eq!(state.selected_index, 1);
}

// ---------------------------------------------------------------------------
// ViewerState — tab navigation
// ---------------------------------------------------------------------------

#[test]
fn next_tab_wraps() {
    let mut state = ViewerState::new(3);
    let _ = state.handle_action(Action::NextTab);
    let _ = state.handle_action(Action::NextTab);
    let _ = state.handle_action(Action::NextTab);
    assert_eq!(state.active_tab, 0, "NextTab must wrap at tab_count");
}

#[test]
fn prev_tab_wraps() {
    let mut state = ViewerState::new(3);
    let _ = state.handle_action(Action::PrevTab);
    assert_eq!(state.active_tab, 2, "PrevTab at 0 must wrap to tab_count - 1");
}

// ---------------------------------------------------------------------------
// ViewerState — expand/collapse via action
// ---------------------------------------------------------------------------

#[test]
fn expand_action_on_branch_expands_it() {
    let mut state = ViewerState::new(1);
    state.load_tree(build_test_tree());
    // Select index 0 (Root A, a branch).
    assert_eq!(state.selected_index, 0);

    let _ = state.handle_action(Action::Expand);
    // Root A should now be expanded; display list grows.
    assert!(
        state.tree.display_list.len() > 2,
        "Expand on root branch must increase display list"
    );
}

#[test]
fn collapse_action_collapses_expanded_node() {
    let mut state = ViewerState::new(1);
    state.load_tree(build_test_tree());
    let _ = state.handle_action(Action::Expand); // Expand Root A
    let expanded_count = state.tree.display_list.len();

    let _ = state.handle_action(Action::Collapse);
    assert!(
        state.tree.display_list.len() < expanded_count,
        "Collapse must reduce the display list"
    );
}

// ---------------------------------------------------------------------------
// ViewerState — search mode
// ---------------------------------------------------------------------------

#[test]
fn search_action_activates_search_mode() {
    let mut state = ViewerState::new(1);
    assert!(!state.search_active);
    let _ = state.handle_action(Action::Search);
    assert!(state.search_active);
    assert!(state.search_query.is_empty());
}

#[test]
fn push_search_char_accumulates_query() {
    let mut state = ViewerState::new(1);
    state.load_tree(build_test_tree());
    state.search_active = true;
    state.push_search_char('A');
    assert_eq!(state.search_query, "A");
    state.push_search_char('2');
    assert_eq!(state.search_query, "A2");
}

#[test]
fn pop_search_char_removes_last_char() {
    let mut state = ViewerState::new(1);
    state.load_tree(build_test_tree());
    state.search_active = true;
    state.push_search_char('A');
    state.push_search_char('2');
    state.pop_search_char();
    assert_eq!(state.search_query, "A");
}

#[test]
fn dialog_cancel_clears_search_mode() {
    let mut state = ViewerState::new(1);
    state.load_tree(build_test_tree());
    state.search_active = true;
    state.search_query = "test".to_owned();

    let changed = state.handle_action(Action::DialogCancel);
    assert!(changed);
    assert!(!state.search_active);
    assert!(state.search_query.is_empty());
}

// ---------------------------------------------------------------------------
// ViewerState — quit
// ---------------------------------------------------------------------------

#[test]
fn quit_action_sets_should_quit() {
    let mut state = ViewerState::new(1);
    assert!(!state.should_quit);
    let _ = state.handle_action(Action::Quit);
    assert!(state.should_quit);
}

// ---------------------------------------------------------------------------
// ViewerState — breadcrumb
// ---------------------------------------------------------------------------

#[test]
fn breadcrumb_empty_returns_root() {
    let state = ViewerState::new(1);
    assert_eq!(state.breadcrumb_display(), "/");
}

#[test]
fn breadcrumb_after_expand_and_down_navigate() {
    let mut state = ViewerState::new(1);
    state.load_tree(build_test_tree());
    let _ = state.handle_action(Action::Expand); // Expand Root A
    let _ = state.handle_action(Action::ScrollDown); // Move to Child A1
    let breadcrumb = state.breadcrumb_display();
    assert!(
        breadcrumb.contains("Root A"),
        "breadcrumb must include Root A after navigating to its child; got: {breadcrumb}"
    );
}

// ---------------------------------------------------------------------------
// ViewerState — load_tree resets state
// ---------------------------------------------------------------------------

#[test]
fn load_tree_resets_selection_and_breadcrumb() {
    let mut state = ViewerState::new(1);
    state.load_tree(build_test_tree());
    let _ = state.handle_action(Action::ScrollDown);
    assert_eq!(state.selected_index, 1);

    // Reload — should reset.
    state.load_tree(build_test_tree());
    assert_eq!(state.selected_index, 0);
    assert_eq!(state.breadcrumb_display(), "/");
}

// ---------------------------------------------------------------------------
// ViewerHeaderContext
// ---------------------------------------------------------------------------

#[test]
fn viewer_header_context_summary_none_by_default() {
    use umrs_ui::ViewerHeaderContext;
    let ctx = ViewerHeaderContext::new("tool", "source", 42);
    assert!(ctx.summary_description.is_none());
    assert_eq!(ctx.record_count, 42);
}

#[test]
fn viewer_header_context_with_summary() {
    use umrs_ui::ViewerHeaderContext;
    let ctx =
        ViewerHeaderContext::new("tool", "source", 42).with_summary("15 categories");
    assert_eq!(
        ctx.summary_description.as_deref(),
        Some("15 categories")
    );
}

// ---------------------------------------------------------------------------
// BTreeMap metadata ordering
// ---------------------------------------------------------------------------

#[test]
fn node_metadata_is_sorted_by_key() {
    let mut node = TreeNode::leaf("l", "");
    node.metadata.insert("z_key".to_owned(), "last".to_owned());
    node.metadata.insert("a_key".to_owned(), "first".to_owned());
    node.metadata.insert("m_key".to_owned(), "middle".to_owned());

    let keys: Vec<&str> = node.metadata.keys().map(String::as_str).collect();
    assert_eq!(keys, vec!["a_key", "m_key", "z_key"]);
}
