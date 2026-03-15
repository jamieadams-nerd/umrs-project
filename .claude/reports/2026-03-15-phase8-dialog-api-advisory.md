# Phase 8 Dialog API — Pre-Implementation Security Advisory

Audit date: 2026-03-15
Depth: surface
Scope: `.claude/plans/tui-enhancement-plan.md` Phase 8 design; Phases 1–7 implementation in working tree

---

## Phase 1–7 Implementation Health (brief)

Phases 1 through 7 are complete in the working tree and were examined as context for this review.
The implementation is clean: `IndicatorValue` is fail-closed, `SecureReader` is the only read path
for kernel attributes, `DialogState` does not yet exist. No annotation gaps were found in the
completed phases that are relevant to this Phase 8 advisory.

---

## Phase 8 Advisory — Dialog API

### A-1 — Dialog State Lifecycle: APPROVED WITH ONE CONDITION

The design correctly externalizes `DialogState` to the calling binary rather than embedding it
inside `AuditCardState`. This is the right call. It means:

- There is no implicit global modal state.
- The library has zero authority over when a dialog appears or disappears.
- The caller owns the `Option<DialogState>` and must explicitly construct and destroy it.

**Condition:** The `visible` field on `DialogState` creates a subtle dual-state risk. A caller
could construct `DialogState { visible: false, ... }` and accidentally pass it to `render_dialog()`
without it being displayed — and the `response` field would be `None` indefinitely. The design
should eliminate this by having `render_dialog()` simply not render when `state` is `None`.
The caller should pass `Option<&DialogState>` (or `Option<DialogState>`) to `render_dialog()`;
when it is `None`, the function is a no-op. Remove the `visible` field entirely — presence in
the `Option` is the authoritative visibility signal. This removes a redundant bit of state that
could diverge.

**Recommended API change:**
```rust
pub fn render_dialog(
    frame: &mut Frame,
    area: Rect,
    state: Option<&DialogState>,  // None = not visible; no separate `visible` field
    theme: &Theme,
)
```

This is a cleaner, more auditable pattern: the type system enforces that there is exactly one
way to show or hide a dialog.

---

### A-2 — SecurityWarning Default Focus: CHANGE REQUIRED

The design defines `DialogFocus::Primary` as "OK" / "Yes" and `DialogFocus::Secondary` as
"Cancel" / "No". The design does not state what the *default* focus is when a `SecurityWarning`
dialog is constructed.

For a DoD operator under time pressure, a reflexive Enter keypress must not result in an
irreversible action. **The default focus for `SecurityWarning` must be `DialogFocus::Secondary`
(Cancel).** The operator must make an affirmative hand movement (Tab, Right, or Left arrow) to
move focus to OK before confirming.

This is not merely a UX preference — it is a fail-safe requirement. NIST SP 800-53 SC-5 (denial
of service protection) and SI-10 (input validation) both support the principle that a safe default
is the conservative one.

**For `Confirm` mode**, the default focus of `Primary` ("Yes") is acceptable when the action
being confirmed is reversible. However, if `Confirm` is ever used for a destructive or
irreversible action, the same rule applies. Document this explicitly in the `DialogMode::Confirm`
doc comment so future callers know to assess their use case.

**Recommended type-level enforcement:**

```rust
impl DialogState {
    /// Construct a SecurityWarning dialog. Default focus is Cancel (Secondary)
    /// per fail-safe UI requirements — a reflexive Enter keypress must not
    /// confirm a security-sensitive action.
    ///
    /// NIST SP 800-53 SI-10, SC-5 — conservative default prevents accidental
    /// confirmation of security-affecting operations.
    #[must_use = "SecurityWarning dialog state must be stored and passed to render_dialog()"]
    pub fn security_warning(message: impl Into<String>) -> Self {
        Self {
            response: None,
            message: message.into(),
            mode: DialogMode::SecurityWarning,
            focused: DialogFocus::Secondary,  // Cancel is the safe default
        }
    }
}
```

Provide distinct constructors for each mode rather than a bare struct literal. This ensures
the correct default focus is always set — callers cannot accidentally construct a
`SecurityWarning` with `Primary` focus.

---

### A-3 — Journald Logging: REQUIRED FOR SecurityWarning

The current design has no mechanism for logging dialog interactions. For `SecurityWarning` and
`Confirm` modes, an operator acknowledgement is a security-relevant event that belongs in the
audit trail.

**Required:** When a `SecurityWarning` dialog is dismissed (either direction), the calling binary
must emit a structured journald log entry. The library cannot do this itself (it has no logging
dependency and should not acquire one), so the contract must be documented.

The plan should add a note to the Phase 8 design — and to the `render_dialog()` doc comment —
stating explicitly:

> Callers are responsible for emitting a journald audit record when a `SecurityWarning` or
> `Confirm` dialog produces a `Some(...)` response. The record must include: the dialog message
> (or a stable identifier for it), the outcome (`true`/`false`), the tool name, and the
> `boot_id` for session correlation.
> NIST SP 800-53 AU-2, AU-3 — operator acknowledgement of a security warning is an auditable event.

The tech-writer should add this to the Phase 8 design spec. The rust-developer must add it to the
`render_dialog()` and `DialogState` doc comments when implementing.

**Controls: NIST SP 800-53 AU-2 (Event Logging), AU-3 (Content of Audit Records).**

---

### A-4 — NIST Control Alignment: SUPPLEMENT AC-2

The design cites `NIST SP 800-53 AC-2` for `DialogState` (explicit lifecycle, no implicit global
state). That citation is defensible — AC-2 covers account management lifecycle and is being
applied by analogy to session/state lifecycle. However it is a stretch. More precise controls are:

| Item | Recommended citation | Rationale |
|---|---|---|
| `DialogState` — explicit lifecycle, no hidden state | `NIST SP 800-53 SI-10` | Input validation — dialog state validates operator intent before acting |
| `DialogMode::SecurityWarning` — distinct mode for security actions | `NIST SP 800-53 AC-3` | Access enforcement — security-affecting actions require distinct confirmation path |
| `DialogFocus` — default to Cancel for security dialogs | `NIST SP 800-53 SC-5`, `SI-10` | Fail-safe default; deny-by-default posture |
| Operator acknowledgement logging | `NIST SP 800-53 AU-2`, `AU-3` | Event definition and record content |
| `render_dialog()` — no auto-dismiss | `NIST SP 800-53 AU-2` | Auditable event requires explicit operator action |

The existing AC-2 citation may remain as a secondary annotation. It should not be the sole or
primary citation.

---

### A-5 — Timeout: NOT RECOMMENDED

The plan's `render_dialog()` doc comment explicitly states "no timeout or auto-dismiss behavior."
This is cited as NIST SP 800-53 AC-2. The design is correct. **Do not add a timeout.**

A timeout that auto-dismisses a `SecurityWarning` with a "yes" outcome is a security defect.
A timeout that auto-dismisses with "no" (Cancel) would be safer but introduces a race condition
where an operator walking away from a terminal could silently cancel a legitimate confirmation.
Neither is acceptable. The explicit dismissal requirement is the right design. No change needed here.

---

### A-6 — Type-It-To-Confirm: NOT REQUIRED, BUT DOCUMENT THE DECISION

The question of whether `SecurityWarning` should require the operator to type "CONFIRM" (or
similar) rather than just pressing Enter is a policy question, not a pure implementation question.

**Assessment:** The two-button design with fail-safe default focus (Cancel) is sufficient for
Phase 8. A typed confirmation adds friction that could cause operators to copy-paste the string
or, worse, to wire up a macro — defeating the safety property entirely. The tab-to-OK-then-Enter
gesture is deliberate enough for a TUI environment where operators are already heads-down in a
terminal.

**However:** The typed-confirmation pattern is appropriate for destructive, irreversible, or
classified-data-affecting actions (e.g., "delete all audit records," "export CUI data"). Document
this in the `DialogMode` doc comment:

> For actions that are irreversible or affect classified data, consider whether a text-entry
> confirmation (`type "CONFIRM" to proceed`) is more appropriate than this two-button dialog.
> Phase 10 (Control Text Pop-Up) may need this pattern.

This is a tech-writer task for the design doc and a coder task for the `DialogMode` enum comment.

---

## Summary

| Item | Status | Action required |
|---|---|---|
| Dialog state lifecycle — no global modal state | APPROVED | Remove `visible` field; use `Option<&DialogState>` at call site |
| SecurityWarning default focus | CHANGE REQUIRED | Default focus must be `Secondary` (Cancel); use named constructors |
| Journald logging for acknowledgements | REQUIRED | Document the caller obligation in `render_dialog()` doc comment |
| NIST control citations | SUPPLEMENT | Replace or supplement AC-2 with SI-10, AC-3, SC-5, AU-2, AU-3 |
| Auto-dismiss timeout | NOT RECOMMENDED | Current design (no timeout) is correct — no change |
| Type-to-confirm | NOT REQUIRED IN PHASE 8 | Document the decision boundary in `DialogMode` doc comment |

**Sign-off status: CONDITIONAL.** Implementation may proceed after:

1. `visible` field removed from `DialogState`; `render_dialog()` takes `Option<&DialogState>`
2. Named constructors created for each `DialogMode`; `SecurityWarning` constructor sets focus to `Secondary`
3. Caller logging obligation documented in `render_dialog()` doc comment (AU-2, AU-3)
4. Control citations updated per the table in A-4

Items 3 and 4 are doc-comment work that the rust-developer can do inline during implementation.
Items 1 and 2 require a design decision acknowledgement from Jamie before the developer starts.

---

## Gap Analysis Summary

Files reviewed: 3 (tui-enhancement-plan.md, app.rs, main.rs) + lib.rs
Total findings: 4 (0 HIGH, 2 MEDIUM, 2 LOW)

HIGH findings: none

MEDIUM findings:
- A-1: `visible` field creates dual-state risk; API should use `Option<&DialogState>` at call site
- A-2: Default focus for SecurityWarning is unspecified; must be Secondary (Cancel) per fail-safe requirements

LOW findings:
- A-3: Caller logging obligation for dialog acknowledgements not documented in design (AU-2, AU-3)
- A-4: AC-2 is the sole citation; more precise controls (SI-10, AC-3, SC-5, AU-2, AU-3) should be added

Uncited security claims: "no implicit global modal state" — no control citation backing this
property in the current design text.

Inconsistencies (code vs. docs): none — Phase 8 is not yet implemented.
