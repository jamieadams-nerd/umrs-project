# Rust Developer Agent Memory

## Key Files
- See `project_structure.md` for workspace layout and module map.
- See `patterns.md` for confirmed high-assurance pattern application notes.

## User Preferences
- Jamie Adams (Jamie). Concise communication.
- End of session: brief summary, flag anything unusual, "your turn to review, commit, and push."

## Active Work
- Header redesign (umrs-tui) COMPLETE — 4-row layout (2026-03-15)
  - HeaderContext: has system_uuid, architecture (uname.machine()), os_name (caller-supplied)
  - build_header_context() reads system_uuid internally; 3rd param is os_name
  - header.rs: 4-row layout:
    - Row 1: full-width "Assessment : {report_name} / {report_subject}"
    - Row 2: Host|Tool (two-column)
    - Row 3: OS "{os_name} ({architecture})"|Assessed (two-column)
    - Row 4: SELinux|FIPS (indicator line)
  - Removed from rendering: Boot ID, System ID, Scope, Kernel, LSM, Lockdown
    (fields still exist in HeaderContext/SecurityIndicators — do NOT remove them)
  - Single-column fallback: 4 rows (Assessment, Host, OS, SELinux)
  - build_two_column_lines / build_single_column_lines are pub #[doc(hidden)] for tests
  - tests/header_tests.rs: 39 tests; line_text() helper collects span content
- Phase 2a of Kernel Security Posture Probe: REVIEWED AND FIXED (2026-03-14)
  - Plan: `.claude/plans/kernel-security-posture-probe.md`
  - 180 tests pass (40 modprobe, 60 posture, 26 kattrs, 14 fips, 16 rpm_header, 7 rpm_db, 16 sealed_cache)
  - Security-engineer F1/F2, auditor F-06/F-07/F-08/F-09 + others, tech-writer sweep all resolved.
  - Doc-sync task #1 still pending for tech-writer.
  - Phase 2b is next.
- TUI Enhancement Plan phases 1–8 ALL COMPLETE (2026-03-15)
  - Plan: `.claude/plans/tui-enhancement-plan.md`
  - Phase 3: DataRow is enum with 6 variants; all constructors #[must_use]; data_panel.rs matches all
  - DataRow::new() kept as backward-compatible alias for key_value() — do not remove
  - Phase 5: theme has indicator_active/inactive/unavailable/group_title fields; Theme::indicator_style() method
  - Phase 5: indicator_unavailable uses Color::Yellow (auditor O-5 — distinct from inactive DarkGray)
  - Phase 6: TableRow/TableHeader variants, TABLE_COL_GAP (2 spaces) between columns to prevent jamming
  - Phase 7: 3rd tab "Kernel Security"; ctx built before app in main(); indicator_to_display() helper
  - 28 trait_impl tests pass (added MockAppThreeTabs with 6 new three_tab_mock_* tests)
  - Phase 8: dialog.rs (DialogState/Mode/Focus/render_dialog); no visible field; Option<&DialogState>
  - Phase 8: SecurityWarning and Confirm default focus = Secondary (Cancel/No); fail-safe per SC-5
  - Phase 8: 3 Action variants (DialogConfirm/Cancel/ToggleFocus); 7 theme fields; 6 dialog tests
  - Phase 8: Refresh + Dialog action variants merged into one no-op arm in handle_action (clippy)

## Known Patterns — Blacklist Sentinel
- `evaluate_configured_meets("blacklisted", desired)` now handles the sentinel.
- Blacklist signals use `DesiredValue::Exact(1)` — "blacklisted" maps to integer 1.
- Without this fix, `BootDrift` was silently suppressed for all four DMA-surface blacklist signals.

## Known Patterns — KernelCmdline Contradiction Path (S-01 fix)
- `evaluate_configured_meets()` returns None for BLS options strings (not integer, not "blacklisted").
- For `SignalClass::KernelCmdline` signals, `collect_one()` bypasses `evaluate_configured_meets()`.
- Instead uses `desc.desired.meets_cmdline(configured_boot_cmdline)` directly.
- This is the ONLY way to produce BootDrift/EphemeralHotfix for cmdline signals.
- `read_configured_boot_cmdline()` stores the full raw BLS options string for operator display.
- The dedicated path is in `collect_one()` — not in `evaluate_configured_meets()`.
- Tests in `posture_tests.rs` section "S-01: KernelCmdline configured-value contradiction detection".

## Confirmed Patterns (umrs-platform posture module)
- All `/proc/` reads: `ProcfsText` + `SecureReader::read_generic_text`
- All `/sys/` reads: `SysfsText` + `SecureReader::read_generic_text` with SYSFS_MAGIC
- Merge-tree pattern: load dirs in precedence order, last-writer-wins, lexicographic sort within dir
- Debug timing: `#[cfg(debug_assertions)]` block with `std::time::Instant`, log µs at completion
- Non-debug suppression: `#[cfg(not(debug_assertions))] let _ = var;`
- SignalId `Ok(None)` arm in `read_live_sysctl`: must include ALL non-sysctl variants explicitly
- `ParsedDirective` and `parse_modprobe_line` are `pub` (not pub(crate)) — integration tests need visibility

## Clippy Suppressions Active (lib.rs)
- `option_if_let_else`, `redundant_closure`, `module_name_repetitions`
- `missing_errors_doc`, `missing_panics_doc`, `unreadable_literal`, `doc_markdown`
- Never add suppressions without asking Jamie.
