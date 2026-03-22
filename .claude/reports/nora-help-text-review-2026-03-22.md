# Help Text & Dialog API Review — Nora (guest-coder)

**Date:** 2026-03-22
**Reviewer:** Nora (guest-coder — first-time user perspective)
**Files reviewed:**
- `umrs-uname/src/main.rs` — `help_text_for_tab()`, lines 1479–1646
- `umrs-ui/src/dialog.rs` — full file, focus on scroll API

---

## Summary

The help text is better than most TUI tools I have used. An operator who knows what SELinux is will get through it without friction. The weak spots are concentrated in Tab 2: the trust tier progression is not self-explanatory to someone without UMRS context, and the verification codes section — while individually accurate — explains *what* before explaining *why it matters*. The dialog API in `dialog.rs` is clean; the scroll writeback design is the one place requiring a pause, and the doc comment covers it adequately.

---

## Tab 0 — OS Information

**Verdict: Good. Minor issues only.**

What works: "These values identify the platform under assessment" immediately frames the tab's purpose. The dual-source corroboration note on PLATFORM IDENTITY ("two independent sources must agree") is exactly the right detail to surface — it explains why the tool does something, not just what it does. The BOOT ID explanation gives a concrete use case.

**Finding 1 — LOW**
Item: Tab 0 — OS NAME / VERSION bullet
Finding: "Used to confirm the platform matches the expected baseline for this environment" implies there is a configured expected baseline somewhere. There is not. A first-time operator may spend time looking for where to set one.
Suggestion: "Use this to verify the platform is the expected OS and version for this deployment."

**Finding 2 — LOW**
Item: Tab 0 — BOOT ID bullet
Finding: "journald" will be unfamiliar to operators from non-systemd backgrounds (Ubuntu users in particular). Minor — the surrounding sentence is clear enough.
Suggestion: Add "(the system's structured event log)" after "journald" in-line.

---

## Tab 1 — Kernel Security

**Verdict: Solid. One clarity gap in the contradiction block.**

What works: `✓ / ✗ / ?` legend is unambiguous. The six indicator groups are a useful map. The NOTE at the bottom correctly manages expectations.

**Finding 3 — MEDIUM**
Item: Tab 1 — "⚠ DRIFT" explanation
Finding: "Configuration is hardened but running kernel is not — intended hardening is not active." On a first read, "Configuration is hardened" sounds like a positive assertion. An operator unfamiliar with sysctl persistence may interpret this as "the system is hardened" before arriving at the contradiction. The causal direction is backwards from how most people read status messages.
Suggestion: "The configuration file calls for hardening, but the running kernel does not reflect it. The intended setting is not active right now."

**Finding 4 — LOW**
Item: Tab 1 — "⚠ NOT PERSISTED" explanation
Finding: "Running kernel is hardened now but configuration will not survive a reboot" — what will not survive is the hardened *state*, not the configuration file. The sentence reads oddly.
Suggestion: "Running kernel is hardened now, but the configuration file does not match — this hardening will be lost on reboot."

**Finding 5 — MEDIUM**
Item: Tab 1 — SUMMARY, baseline comparison paragraph
Finding: "If the kernel is a major version newer, accuracy cannot be guaranteed." This tells the operator something is wrong but gives no action to take. "Accuracy" is also vague.
Suggestion: Append: "If this appears, treat indicator readings as advisory and verify against kernel documentation for your specific version."

---

## Tab 2 — Trust / Evidence

**Verdict: Needs the most work. Trust tiers and verification codes require UMRS background to parse.**

What works: The TOP/BOTTOM section split is clearly labeled. The explicit callout distinguishing Tab 2 contradictions (OS detection) from Tab 1 contradictions (kernel/config) is important and rarely done this carefully.

**Finding 6 — HIGH**
Item: Tab 2 — TRUST TIERS table, T1 through T4
Finding: The tier names ("Kernel Anchored", "Env Anchored", "Platform Verified", "Integrity Anchored") are UMRS-internal vocabulary. The table says what each tier *is* but not what it *implies for the operator*. A security operator who sees T2 needs to know: "what can I rely on? What can I not?"
Suggestion: Add a one-line implication after each tier:
```
T0  Untrusted          No reliable sources found.
                       Do not rely on any displayed values.
T1  Kernel Anchored    At least one kernel source confirmed.
                       Core identity is reliable; environment data is not corroborated.
T2  Env Anchored       Environment sources corroborate kernel.
                       OS identity is consistent across kernel and config files.
T3  Platform Verified  All expected sources agree.
                       Suitable for assessment; detection pipeline fully corroborated.
T4  Integrity Anchored All sources agree + kernel filesystem provenance verified.
                       Highest confidence; verification codes confirm no path substitution.
```

**Finding 7 — MEDIUM**
Item: Tab 2 — VERIFICATION CODES section, missing preamble
Finding: There is no sentence explaining what verification codes *are for* before explaining what each one does. A first-time operator may assume these are informational annotations rather than security-relevant provenance markers.
Suggestion: Add before the first entry: "Verification codes appear alongside evidence entries read through provenance-checked paths. They confirm the tool read from the real kernel filesystem, not a substitute or overlay that an attacker could control."

**Finding 8 — MEDIUM**
Item: Tab 2 — TRUST TIERS, cumulative nature
Finding: The tiers are cumulative — T3 includes all guarantees of T0 through T2, and T4 includes all of T0 through T3. This is not stated anywhere.
Suggestion: Add after the tier table: "Each tier includes all guarantees of the tiers below it. T4 is the strongest claim and subsumes T0 through T3."

**Finding 9 — LOW**
Item: Tab 2 — EVIDENCE TYPES, "Filesystem identity" entry
Finding: "from statfs() syscall" will be opaque to operators who are not C programmers. "syscall" adds no value for this audience.
Suggestion: "from statfs() — the kernel reports the mounted filesystem type directly." Drop "syscall."

---

## Navigation Instructions

**Finding 10 — LOW**
Item: Fallback navigation block (tab index `_`)
Finding: The fallback block differs from the per-tab blocks: omits PgDn/PgUp, says "q or Esc quit" instead of "Enter, Esc, or q close this help."
Suggestion: Normalize to match the per-tab format.

---

## Dialog API — `umrs-ui/src/dialog.rs`

**Verdict: Clean. One design decision requiring careful reading; two doc gaps.**

What works: The `Option<&mut DialogState>` visibility model is elegant. The `DialogMode` table is the right artifact. The fail-safe Cancel/No default is well-motivated. `#[must_use]` messages on all four constructors are specific and useful.

**Finding 11 — MEDIUM**
Item: `DialogState` — `total_lines` and `visible_height` fields
Finding: Both fields are writable by callers, but callers must never write them — `render_dialog` owns these values. A caller who reads only the struct definition may attempt to set `total_lines` manually.
Suggestion: Add to both field doc comments: "Do not set this field — [`render_dialog`] owns this value and will overwrite any caller-set value on the next render frame."

**Finding 12 — LOW**
Item: `DialogState::scroll_down` — pre-first-render behavior
Finding: The struct-level doc says calling scroll before the first render is a no-op, but `scroll_down`'s own method doc does not say this.
Suggestion: Add: "Before the first `render_dialog` call, `total_lines` and `visible_height` are zero, making this a safe no-op."

**Finding 13 — MEDIUM**
Item: `render_dialog` — width computation uses `str::len` (byte count, not character count)
Finding: For ASCII help text this is correct. Future i18n messages with multibyte characters will get incorrect width estimation.
Suggestion: Add to the `message` field doc: "Width estimation uses byte length. Messages containing multibyte Unicode characters may produce a wider-than-expected dialog."

---

## Priority Order

1. **Finding 6 (HIGH)** — Trust tier implications missing
2. **Finding 3 (MEDIUM)** — DRIFT ambiguous causal direction
3. **Finding 7 (MEDIUM)** — Verification codes need purpose statement
4. **Finding 8 (MEDIUM)** — Trust tiers cumulative nature unstated
5. **Finding 5 (MEDIUM)** — Baseline caveat needs action
6. **Finding 11 (MEDIUM)** — Dialog scroll fields ownership note
7. **Finding 13 (MEDIUM)** — Dialog width byte-vs-char doc gap
8. **Findings 1, 2, 4, 9, 10, 12 (LOW)** — Minor phrasing and consistency
