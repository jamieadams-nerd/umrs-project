# Module-Level Doc Comment Review — `umrs-platform`

**Reviewer**: tech-writer
**Date**: 2026-03-14
**Scope**: All `//!` doc comments in `components/rusty-gadgets/umrs-platform/src/`
**Mode**: Architecture Mode (explanatory review) / review-only, no source edits

---

## Executive Summary

The `umrs-platform` crate has strong, detailed module documentation overall. Most modules clearly state their purpose, include compliance citations, and document security rationale. Three structural issues recur across the codebase: (1) inconsistent compliance citation format — some modules use `NIST SP 800-53` and some use `NIST 800-53`; (2) the `kattrs/mod.rs` comment uses an older all-caps prose style that is inconsistent with every other module in the crate; (3) several modules are missing a top-level `#` heading in their `//!` doc, which affects rendered output from `cargo doc`.

No modules are missing documentation entirely. The weakest coverage is in `kattrs/tpi.rs`, `kattrs/types.rs`, and the internal detect phase modules (`file_ownership`, `integrity_check`), but these are all internal-only (`pub(super)`) modules and the deficiencies are acceptable at current scope.

---

## Per-File Findings

### `lib.rs`

**Overall**: Good. Clear two-paragraph summary. Module layout table is well-formed.

**Issues**:

1. **Citation format inconsistency** — compliance block uses bare `NIST 800-53 SI-7` and `NSA RTB RAIN` without the `SP` prefix or bullet formatting used by most other modules. Recommend reformatting to match the majority pattern:
   ```
   - **NIST SP 800-53 SI-7**: Software and Information Integrity.
   - **NSA RTB RAIN**: Non-Bypassable security checks.
   ```

2. **Module layout table** — `sealed_cache` is listed in the module declarations and re-exports but absent from the `## Module Layout` table. Add a row:
   ```
   | `sealed_cache` | `SealedCache`, `CacheStatus`, `DEFAULT_TTL_SECS`, `MAX_TTL_SECS` — SEC pattern |
   ```

3. **Missing `posture` sub-module content detail** — the table row for `posture` lists four types but does not mention the `SignalDescriptor`, `ContradictionKind`, `FipsCrossCheck`, or `ModprobeConfig` types that are also re-exported. The row is not wrong, but a reader coming to the crate root first will not find these. Consider expanding to: `PostureSnapshot`, `SignalReport`, `SignalId`, `AssuranceImpact`, and notes.

---

### `evidence.rs`

**Overall**: Excellent. Clear purpose statement, strong AU-10 rationale, correct doc-link usage in the `EvidenceBundle` comment.

**Issues**:

1. **`push()` doc comment** — the comment says "Records are never reordered or removed after being pushed" but does not link to `EvidenceBundle` for context. Add `[`EvidenceBundle`]` cross-reference.

2. **`EvidenceRecord::notes` field** — the 64-character truncation requirement is stated here ("Strings longer than 64 characters should be truncated at the call site") but it is not consistent with the `OsReleaseParseError` doc, which says truncation happens "at log and display call sites". These two statements describe the same policy but use different language. Standardize: pick "at call sites before logging or display" across both.

3. **Minor grammar** — `evidence.rs` line 14: "The design follows the principle that no fact should be trusted without provenance:" — the colon introduces a sentence, which is fine, but the very long sentence that follows (five comma-separated clauses) should be broken into two. Suggested split after "from which source kind":
   > The design follows the principle that no fact should be trusted without provenance. Each record captures how the data was obtained (fd-anchored or path-based), the source kind, what filesystem magic was observed, file metadata at the time of access, and whether parsing succeeded.

---

### `confidence.rs`

**Overall**: Excellent. The trust tier model is explained clearly and the monotonic ordering invariant is made explicit.

**Issues**:

1. **Citation format** — uses `NIST SP 800-53` (correct form) consistently. No issues here.

2. **`ConfidenceModel::downgrade()` doc** — states "If `to` is greater than or equal to the current level this is a no-op". Technically this should say "greater than" only — equal would also be a no-op under the current implementation (`to < self.level`), but the wording "greater than or equal" is accurate. No change required; this is a note.

3. **Cross-reference gap** — the `TrustLevel` enum describes `IntegrityAnchored` as "os-release ownership + installed digest verified" but does not cross-reference `sealed_cache::SealedCache`, which depends on `TrustLevel`. This gap does not affect correctness, but a developer reading the confidence model in isolation will not immediately see the cache relationship. Worth linking in a future pass.

---

### `os_identity.rs`

**Overall**: Good. Clear separation between substrate-derived identity and `os-release` self-report is explained well.

**Issues**:

1. **`SubstrateIdentity::facts_count` field doc** — references "ANSSI Rust Guide, Finding 1" which is a project-internal security review reference. External developers will not know what this refers to. Either expand to "ANSSI Secure Rust Coding Guide" and drop the "Finding 1" label, or add a brief parenthetical: "(the ANSSI secure Rust coding guide requirement for checked arithmetic)".

2. **`CpuArch` comment** — the doc says "cross-checked against the ELF header" but there is no ELF reading code visible in this file. The struct has fields for raw `e_machine` values, which suggests this is a plan or a description of what callers are expected to do. The comment creates an expectation that the type itself performs cross-checking; clarify that the cross-check is the caller's responsibility:
   > `Unknown(u16)` preserves the raw ELF `e_machine` value for audit records when the architecture is not in this enumeration. The caller is responsible for cross-checking `uname(2)` against an ELF header before constructing this value.

3. **`OsFamily::PacmanBased`** — listed in the enum but not mentioned in the `os_identity.rs` compliance block, which only discusses RHEL and Fedora examples. The compliance text is still accurate as written; this is cosmetic.

---

### `os_release.rs`

**Overall**: Strong. The "no raw strings" invariant and two-path parsing cross-reference are well stated.

**Issues**:

1. **Module doc cross-reference** — says "Two-path independent parsing of the `os-release` file itself occurs in `detect/release_parse.rs`." This is an informal module path reference. It should use a doc link or at minimum backtick the path. `detect/release_parse.rs` is a `pub(super)` module so a doc link would be `[`crate::detect`]` — point to the `detect` module rather than the private submodule. Suggested wording:
   > Two-path independent parsing of the `os-release` file itself occurs in the `detect` module (`release_parse`). This module provides only the types and their per-field validation logic.

2. **`OsReleaseParseError` payload truncation** — the doc says "truncated to 64 characters at log and display call sites". The `EvidenceRecord::notes` field says 64 characters too. These are consistent but the number is not defined as a named constant. If this is a policy limit, it should be a `const MAX_LOG_PAYLOAD_LEN: usize = 64` somewhere. Flag for developer attention.

3. **`ValidatedUrl`** — permits `http://` in addition to `https://`. The compliance doc for `SI-10` says "prevents data URI or file URI injection" but does not flag that plain HTTP is permitted. On a FIPS/CUI system this is worth noting explicitly:
   > **Note**: HTTP URLs are accepted for compatibility with some distribution configurations. Callers must not use `http://` URLs for outbound connections from deployed binaries (network isolation policy).

4. **`OsRelease::ansi_color`** — the "MUST NOT" warning is correct and important. However, the field is `Option<String>` with no validation. Consider adding a note that the value is intentionally unvalidated because it is display-only and validation would add no security value. This prevents a future developer from adding "helpful" validation that changes the semantics.

---

### `sealed_cache.rs`

**Overall**: Excellent — the strongest module doc in the crate. The threat model section is clear, the FIPS posture section is thorough, and the sealing key lifetime is fully explained.

**Issues**:

1. **Citation format** — uses `NIST SP 800-53` (correct form with `SP`) and `NIST 800-218` (without `SP`). The NIST 800-218 citation should be `NIST SP 800-218` for consistency. Affected lines in the compliance block.

2. **"Seal failure → discard, re-run, log anomaly, return fresh result"** — this is an accurate summary, but it is written as a sequence of imperative fragments separated by arrows. The doc style elsewhere uses full sentences. Suggested rewrite:
   > On seal verification failure, the cache discards the stored result, re-runs the detection pipeline, logs an anomaly, and returns the fresh result.

3. **Sealing key derivation formula** — `Key = SHA-256(boot_id_bytes ‖ 0x00 separator ‖ starttime_ticks_le)` — the `‖` symbol may not render in all `cargo doc` themes. Use standard Markdown code or explicit prose:
   > `Key = SHA-256(boot_id || 0x00 || starttime_le)` where `||` denotes concatenation and `starttime_le` is the start time as a little-endian `u64`.

---

### `kattrs/mod.rs`

**Overall**: Functional but uses an older style that is visually inconsistent with every other module in the crate. The all-caps `RATIONALE FOR SPECIALIZED READERS:` and `DESIGN PRINCIPLES:` headings read as a design note from an early draft rather than rendered documentation.

**Issues**:

1. **Style inconsistency** — all other modules use standard Rust `//!` Markdown doc comment conventions with `##` section headings. `kattrs/mod.rs` uses all-caps headings and prose sections without Markdown formatting. The content is good but it will not render as intended with `cargo doc`. The numbered principles should be `##` sections or a description list.

2. **Missing top-level `#` heading** — the module doc starts with "UMRS PLATFORM: High-Assurance Kernel Attribute Modeling" inside a `//!` comment block but without a `#` heading prefix. Every other module uses `# ModuleName — subtitle` as the first line. Add `# UMRS Platform kattrs — High-Assurance Kernel Attribute Access`.

3. **Recommendation** — rewrite the doc to match the style of `posture/mod.rs` or `detect/mod.rs`: concise opening paragraph, `## Architecture` section replacing the all-caps principles, `## Submodule Layout` table (already present), `## Compliance` section. The content can stay; only the formatting needs updating.

4. **"Typo** — `// Copyright (c) 2026 Jamie Adams (a.k.a, Imodium Operator)` — the comma placement in `(a.k.a, Imodium Operator)` should be `(a.k.a. Imodium Operator)`. This appears in `kattrs/mod.rs`, `kattrs/traits.rs`, `kattrs/types.rs`, `kattrs/selinux.rs`, `kattrs/tpi.rs`, and `kattrs/procfs.rs`. All other modules use the correct form `(a.k.a. Imodium Operator)`. This is a copyright-block punctuation error, not a doc comment issue, but worth noting as a sweep item.

---

### `kattrs/traits.rs`

**Overall**: Good. The `SecureReader`, `KernelFileSource`, `StaticSource`, and `AttributeCard` types are well documented at the type level.

**Issues**:

1. **Module `//!` heading missing `#`** — starts with a description paragraph with no `#` heading. Add `# kattrs::traits — Kernel Attribute Read Engine`.

2. **`KernelFileSource::parse()` warning** — the doc comment contains a "# Warning" pseudo-section using plain prose. Rust doc conventions use `# Safety`, `# Panics`, `# Errors` as standard section names. A `# Warning` section is non-standard but will render fine. However, the warning text is important and should use `> **Warning:**` blockquote format or the standard `# Warning` pattern consistently.

3. **`SecureReader::execute_read_text()` doc** — says "Callers **must** be `ProcfsText` or `SysfsText` wrapper types". This is a `pub(crate)` function so the constraint is enforced by visibility. The doc is accurate but slightly alarming phrasing for an internal function. Since it is `pub(crate)`, only module-aware callers can reach it; simplify: "Only `ProcfsText` and `SysfsText` wrapper types should call this function. Path prefix validation is the caller's responsibility."

4. **`SecureReader::read_with_card()`** — the doc says "Cards constructed via this method are proof that the value was obtained through the full provenance-verified path". This is a strong claim. The proof is by construction (the method calls `execute_read` which enforces the magic check), and the claim is accurate. However, `AttributeCard` also has a direct-field construction path that does NOT carry this proof. The type-level doc for `AttributeCard` already notes this ("Direct field construction is permitted but does not carry provenance proof"). The `read_with_card` doc should cross-reference the `AttributeCard` doc note: "See [`AttributeCard`] — only cards produced via this method carry the provenance guarantee."

---

### `kattrs/types.rs`

**Overall**: Minimal but adequate for the types it defines. `EnforceState` and `DualBool` are simple value types that derive meaning from their parent context.

**Issues**:

1. **Module `//!` heading missing `#`** — add `# kattrs::types — Domain Value Types`.

2. **`DualBool` doc** — says "Used for SELinux booleans where the kernel tracks a live state and an uncommitted pending state separately." The pending state concept is not explained further. A new developer may not know what "uncommitted pending state" means in the SELinux boolean context. Add one sentence: "SELinux boolean nodes expose both the current active value and a pending value that takes effect when `commit_pending_bools` is written to selinuxfs."

3. **Compliance citation** — `EnforceState` has `NIST 800-53 AC-3` (no `SP`). Should be `NIST SP 800-53 AC-3`.

---

### `kattrs/procfs.rs`

**Overall**: Good. `ProcFips`, `ModuleLoadLatch`, and `ProcfsText` are clearly documented.

**Issues**:

1. **Module `//!` heading missing `#`** — add `# kattrs::procfs — Procfs Kernel Attribute Types`.

2. **Citation format** — module-level citations use bare `NIST 800-53` without `SP`. Consistent within the module but inconsistent with `evidence.rs`, `confidence.rs`, `sealed_cache.rs`.

3. **`ProcFips` struct** — no module-level `#` heading; starts immediately with a struct doc. The pattern used in `kattrs/selinux.rs` (section dividers `// === ... ===`) is present here but there is no `//!` heading for the module. The module-level comment ends at line 11 with no heading.

4. **`ProcfsText::new()` return type** — the error is documented as "`InvalidInput` if `path` does not start with `/proc/`" but does not use doc-link syntax to the `io::ErrorKind::InvalidInput`. For a public API used by multiple callers, explicit linkage is helpful. This is a minor style issue.

---

### `kattrs/sysfs.rs`

**Overall**: Good. Closely mirrors `procfs.rs` structure.

**Issues**:

1. **Module `//!` heading missing `#`** — add `# kattrs::sysfs — Sysfs Kernel Attribute Types`.

2. **Citation format** — same bare `NIST 800-53` vs. `NIST SP 800-53` inconsistency.

3. **`SYSFS_MAGIC` value comment** — "Not exposed as a named constant in nix 0.27 on this target, so defined locally." This is accurate and useful. When `nix` adds this constant, the local definition should be removed. Flag for a developer to add a TODO comment with the nix issue tracker reference if one exists.

---

### `kattrs/selinux.rs`

**Overall**: Good. Static and dynamic selinuxfs types are clearly separated and documented.

**Issues**:

1. **Module `//!` heading missing `#`** — add `# kattrs::selinux — SELinux Kernel Attribute Types`.

2. **Citation format** — bare `NIST 800-53 AC-3` etc. without `SP`.

3. **`GenericKernelBool` and `GenericDualBool`** — both types have `KOBJECT = "selinuxfs/booleans"` and `ATTRIBUTE_NAME = "generic_bool"` / `"generic_dual_bool"`. These are synthetic names, not real kernel kobject names. The doc comments should note that these are placeholder identifiers used for `AttributeCard` display purposes only, not actual kernel attribute names.

4. **`SecureReader<GenericKernelBool>::read_generic()`** — no doc comment. Add a one-line doc: "Provenance-verified read of a dynamic selinuxfs boolean attribute." Same for `read_generic` on `GenericDualBool`.

---

### `kattrs/security.rs`

**Overall**: Strong. `KernelLockdown` and `LockdownMode` are thoroughly documented, including the TPI parsing rationale.

**Issues**:

1. **Module `//!` heading missing `#`** — add `# kattrs::security — Securityfs Kernel Attribute Types`.

2. **Citation format** — `NIST 800-53 CM-7` etc. without `SP`. Inconsistent with `sealed_cache.rs`.

3. **`parse_lockdown_path_a` and `parse_lockdown_path_b`** — these are private functions with doc comments, which is good practice. `parse_lockdown_path_a` calls them "Nom path" and "Imperative path" — consider using "Path A (nom declarative)" and "Path B (imperative split)" to match the terminology in the `KernelLockdown::parse()` doc and in the TPI pattern documentation.

4. **`SECURITYFS_MAGIC` comment** — "Value `0x73636673` is defined in the Linux kernel `include/linux/magic.h`." The hex representation is not the canonical notation. The nix crate definition at the top of the file uses `0x7363_6673`. The constant value in the comment should match the actual constant definition above it. Flag as a minor display inconsistency.

---

### `kattrs/tpi.rs`

**Overall**: Adequate. The module-level doc is correct but terse.

**Issues**:

1. **Module `//!` heading missing `#`** — add `# kattrs::tpi — Two-Path Independent Parsing`.

2. **Private functions `parse_type_path_a` and `parse_type_path_b`** — no doc comments. These are the TPI implementation functions. Even as private functions, a one-line doc on each would match the pattern in `kattrs/security.rs` (which docs its equivalent private helpers). Suggested:
   - `parse_type_path_a`: "Nom declarative parser: extracts the type field from a `user:role:type:range` context string."
   - `parse_type_path_b`: "Imperative split parser: splits on `:` and returns the third field."

3. **`validate_type_redundant` public function** — well documented. No changes needed.

4. **Scope note** — this module only handles type field extraction. The full TPI parsing for `SecurityContext` is in `umrs-selinux/src/context.rs`. The module doc could note: "Type-field extraction only. Full security context TPI parsing, including sensitivity and category fields, is implemented in `umrs-selinux::context`." This cross-crate reference may not be possible as a doc link, but the prose note is valuable.

---

### `posture/mod.rs`

**Overall**: Excellent. The Quick Start code block, Architecture table, and Compliance section are all well-formed and accurate.

**Issues**:

1. **Citation format** — uses bare `NIST 800-53` without `SP`. Four compliance entries at the bottom all use this form. Should be `NIST SP 800-53 CA-7` etc.

2. **Quick Start code block** — uses `rust,ignore` which is correct for code that requires a running system. Good.

3. **Architecture table** — `modprobe` is listed as `modprobe.d configured-value reading` but the actual module also handles live `/sys/module/` cross-check. More accurate description: `modprobe.d merge-tree reader and live /sys/module/ cross-check`.

4. **`fips_cross`** — listed in Architecture table as just a module name. Add description: `FipsCrossCheck — RHEL FIPS persistence layer cross-check`.

---

### `posture/signal.rs`

**Overall**: Strong. Every type is documented, compliance citations are present, and the `DesiredValue::meets_signed_integer` rationale (negative sysctl values) is clearly explained.

**Issues**:

1. **Citation format** — bare `NIST 800-53` without `SP` throughout.

2. **`DesiredValue::Custom` variant** — says "The `meets` method always returns `None` for this variant; callers must invoke the signal-specific validator." The doc references "the `meets` method" but the method is actually named `meets_integer`. This is a small precision issue. Change to "The `meets_integer` and `meets_signed_integer` methods always return `None` for this variant."

3. **`ConfiguredValue` struct** — no compliance citation. The parent module (`signal.rs`) is well-cited, but this struct carries data directly relevant to CM-6. A brief citation: "NIST SP 800-53 CM-6: configured value from the persistence layer for contradiction detection" would be consistent with the other struct docs.

4. **`SignalClass::DistroManaged` variant** — "Distro-managed: live value from a kernel interface, but the canonical configuration channel is a distro tool (e.g., `fips-mode-setup`, `mokutil`)." The mention of `mokutil` may confuse — that tool manages Secure Boot keys, not FIPS. Remove `mokutil` or clarify it is an example of a different distro-managed signal. Revised: "e.g., `fips-mode-setup` for FIPS, or `update-secureboot-policy` for Secure Boot state."

---

### `posture/catalog.rs`

**Overall**: Good. The compile-time binding rationale is well stated.

**Issues**:

1. **Citation format** — bare `NIST 800-53` without `SP`.

2. **`SignalDescriptor` struct** — no `#[non_exhaustive]` annotation and no doc comment noting whether fields may be added. Since this is a `pub` struct in a `pub` module, adding fields is a breaking change. Either add `#[non_exhaustive]` (for forward compatibility) or add a doc note: "Fields are stable; this struct is part of the public API." Flag for developer decision — this is a documentation gap that implies an architectural decision.

3. **`SIGNALS` static** — very well documented. The compliance citations in each `SignalDescriptor`'s `nist_controls` field are slightly inconsistent with the broader crate: some use `NIST 800-53` and some use `NSA RTB:` (with colon). These appear inside data values rather than doc comments, so they are out of scope for this review. Flagging for awareness.

---

### `posture/reader.rs`

**Overall**: Excellent. The "hand-written reference + declarative macro" pattern is clearly explained, the reuse table is helpful, and the `PerfEventParanoid` signed-integer rationale is thorough.

**Issues**:

1. **Citation format** — bare `NIST 800-53` without `SP`.

2. **Module `//!` heading** — starts with "Live-value readers for kernel security posture signals." — no `#` heading prefix. Add `# posture::reader — Live-Value Signal Readers`.

3. **`define_sysctl_signal!` macro doc** — uses `# Usage` as a section heading, which is non-standard for Rust doc comments (standard headings are `# Errors`, `# Panics`, `# Safety`, `# Examples`). The content is fine; `# Examples` would be more conventional but `# Usage` will render correctly.

4. **`BootIdReader`** — the doc comment says "Implemented independently of the `detect` module as specified in the plan." The phrase "as specified in the plan" is a project-management reference that has no meaning to a future developer reading the docs cold. Remove it. Keep only: "Reads the kernel boot ID from `/proc/sys/kernel/random/boot_id`. The boot ID is a UUID generated at boot time; it changes on every reboot."

---

### `posture/configured.rs`

**Overall**: Strong. The Trust Boundary section is particularly good — explicitly explaining why these files do not require `SecureReader` is important for auditors.

**Issues**:

1. **Citation format** — bare `NIST 800-53` without `SP`.

2. **Module `//!` heading missing `#`** — add `# posture::configured — Sysctl.d Configured-Value Reader`.

3. **`SYSCTL_SEARCH_DIRS` comment** — says "Phase 1 intentionally omits `/lib/sysctl.d/` (deprecated path alias for `/usr/lib/sysctl.d/` on modern RHEL) to avoid double-counting." This is a good note but uses "Phase 1" terminology that may be unclear to someone unfamiliar with the pipeline phasing. Suggestion: add a parenthetical pointing to where Phase 1 is defined: "see `detect/mod.rs` for the phase sequence."

4. **`parse_sysctl_line` slash-to-dot normalization comment** — the inline comment in `load_conf_file` references "Finding 3" from a security review. Like the ANSSI "Finding 1" reference in `os_identity.rs`, this is an internal reference that has no meaning outside the project. Replace with a self-contained explanation: "Normalize slash-style keys (e.g., `kernel/kptr_restrict`) to dot-style (e.g., `kernel.kptr_restrict`) to match the catalog's `sysctl_key` format. Without this, sysctl.d files using slash-style keys would produce `ConfiguredValue: None` for every signal, silently disabling contradiction detection."

---

### `posture/contradiction.rs`

**Overall**: Excellent. The taxonomy table is clear, the three `ContradictionKind` variants are documented with operator-actionable remediation text, and the `classify()` function doc is precise.

**Issues**:

1. **Citation format** — bare `NIST 800-53` without `SP`.

2. **Module `//!` heading missing `#`** — add `# posture::contradiction — Live vs. Configured Contradiction Classification`.

3. **`evaluate_configured_meets()` signed integer gap** — the function only parses the configured value as `u32`. For `PerfEventParanoid`, which can have a configured value of `-1`, this function would return `None` on a negative configured value, silently treating it as "no configured value". A comment should note this limitation: "Note: negative configured values (e.g., `perf_event_paranoid=-1`) parse as `Err` from `u32` and produce `None`. This is conservative — a negative configured value will not trigger a contradiction even if the live value also differs."

---

### `posture/snapshot.rs`

**Overall**: Good. The Usage code block and Compliance section are solid.

**Issues**:

1. **Citation format** — bare `NIST 800-53` without `SP`.

2. **Module `//!` heading** — starts with "Point-in-time kernel security posture snapshot." — no `#` heading prefix. Add `# posture::snapshot — PostureSnapshot and SignalReport`.

3. **`SignalReport` struct** — has `#[must_use]` with message, which is good. The `must_use` message says "signal reports carry security posture findings — do not discard". However the struct has no `collect()` or `run()` method that returns it — it is populated internally and exposed via `PostureSnapshot::reports()`. The `#[must_use]` on the struct is correct but could be more informative: "security findings — inspect `meets_desired` and `contradiction` before discarding."

4. **Internal phase references** — the module uses `read_live_sysctl`, `read_live_sysctl_signed`, `FipsCrossCheck::evaluate()`, etc. from sibling modules. None of these are doc-linked. Since all sibling modules are `pub` within `posture`, doc links would work. This is a "nice to have" for audit reviewers navigating the rendered docs.

---

### `posture/modprobe.rs`

**Overall**: Excellent. The "Applicable Patterns" section is the most complete pattern listing in the crate — each pattern is named, cited, and briefly explained. This is the model other modules should follow.

**Issues**:

1. **Citation format** — bare `NIST 800-53` without `SP` in the `## Compliance` section. The `## Applicable Patterns` section is consistent.

2. **Module `//!` heading** — starts with "modprobe.d merge-tree reader and live `/sys/module/` cross-check." — no `#` heading prefix. Add `# posture::modprobe — modprobe.d Reader and Live sysfs Cross-Check`.

3. **`is_module_loaded()` note** — "Returns `false` immediately for an empty `module_name` to prevent path construction anomalies where `join("")` would resolve to the base sysfs module directory itself." This is a good security note. It should be elevated from the function body comment into the function's `//` doc comment so it is visible in `cargo doc` output.

---

### `posture/fips_cross.rs`

**Overall**: Excellent. The Three Sources of Truth section, Trust Gate documentation, and Error Information Discipline section are all exemplary.

**Issues**:

1. **Citation format** — uses `NIST 800-53 SC-13` etc. without `SP` in the `## Compliance` section, but uses `NIST 800-53 SI-11` correctly in the Applicable Patterns list. Inconsistent within the same file.

2. **Module `//!` heading** — starts with "FIPS distro-managed configured-value cross-check." — no `#` heading prefix. Add `# posture::fips_cross — FIPS Configured-Value Cross-Check`.

3. **`check_system_fips_marker()` citation** — `NIST 800-53 SI-10: Fail Closed` — SI-10 is Input Validation, not specifically Fail Closed. The fail-closed property is better cited as `NSA RTB Fail Secure` or `NIST SP 800-218 SSDF PW.4.1`. The "Fail Closed" parenthetical is accurate; the control citation is imprecise.

---

### `detect/mod.rs`

**Overall**: Excellent. The phase sequence table is clear, hard vs. soft gate distinction is well explained, and the two abort conditions are called out explicitly.

**Issues**:

1. **Citation format** — uses `NIST SP 800-53` (correct form) consistently. No issues.

2. **Phase table** — the `label_trust` module is not in the phase table (it is not a phase itself, but it produces output). The `LabelTrust` type is the final verdict of the pipeline and is visible in `DetectionResult`. Consider a note: "The `label_trust` module defines `LabelTrust`, the pipeline's final verdict on the parsed `os-release` label."

3. **`OsDetector::detect()` doc** — has a duplicate compliance citation: `NIST SP 800-53 SA-8` appears twice on adjacent lines (lines 190-191 in the source). Remove the duplicate.

---

### `detect/kernel_anchor.rs`

**Overall**: Strong. Steps are numbered, hard gate conditions are explicit, and failure handling is documented.

**Issues**:

1. **Citation format** — uses `NIST SP 800-53` (correct form) consistently.

2. **Module `//!` heading** — uses `#` heading: `# Kernel Anchor Phase`. Good.

3. **`check_pid_coherence()` note** — "PIDs on Linux are always positive i32 values; the cast to u32 is safe." This comment is in the function body, not the doc comment. Elevate to the function doc.

4. **`read_lockdown()` function** — doc says the lockdown tier is "captured as a note rather than a separate confidence modifier — a system in `Confidentiality` lockdown provides stronger provenance guarantees". This statement implies that the caller could use the lockdown tier to qualify the result, but there is no API to retrieve it from the `EvidenceBundle` other than scanning the notes strings. Flag for developer: either expose the lockdown tier as a typed field in `DetectionResult` or note here that the current approach requires callers to scan `notes` fields.

---

### `detect/mount_topology.rs`

**Overall**: Good. Steps are clearly numbered and the confidence upgrade condition is documented.

**Issues**:

1. **Module `//!` heading** — uses `#` heading: `# Mount Topology Phase`. Good.

2. **`read_mnt_namespace()` note** — "readlinkat with AT_EMPTY_PATH on the procfs link" — `AT_EMPTY_PATH` is not what `readlinkat(CWD, path, ...)` does. The code uses `CWD` as the dirfd with a path argument. This note is inaccurate. `readlinkat(CWD, "/proc/self/ns/mnt", ...)` resolves the path relative to the current directory (or absolute, since the path starts with `/`). Remove the `AT_EMPTY_PATH` reference; it is incorrect.

3. **`read_etc_statfs()` doc** — says "cross-checks that the path where `os-release` will be sought is on a real, identifiable filesystem — not a tmpfs substitution." The check records the magic but does not reject any magic value. An adversary could bind-mount tmpfs at `/etc` and this check would record `TMPFS_MAGIC` without refusing to continue. The doc should reflect this limitation: "Records the filesystem magic — but does not reject any specific magic value. A non-standard magic (e.g., `TMPFS_MAGIC`) is an anomaly that callers may inspect in the evidence bundle."

---

### `detect/label_trust.rs`

**Overall**: Excellent. The distinction between `LabelTrust` and `TrustLevel` is explained clearly and the ordering rationale is present.

**Issues**:

1. **Module `//!` heading** — uses `#` heading. Good.

2. **Cross-reference to `TrustLevel`** — the doc says `Unlike [`crate::TrustLevel`]` — this uses a doc link, which is correct. Verify this renders properly since `TrustLevel` is re-exported from the crate root. It should, but test with `cargo doc`.

3. **`IntegrityVerifiedButContradictory::contradiction` field** — the `≤64 characters` constraint matches the policy stated in `EvidenceRecord::notes`, which is good. Cross-reference: "See `EvidenceRecord::notes` for the truncation policy."

---

### `detect/substrate/mod.rs` (`substrate/`)

**Overall**: Good. The trait design rationale ("intentionally narrow surface") is well explained.

**Issues**:

1. **Module `//!` heading** — uses `#` heading: `# Package Substrate Probes`. Good.

2. **`PackageProbe` trait** — the doc mentions "three operations the pipeline needs (probe identity, query ownership, fetch digest)" but the `pub(crate)` visibility means external developers will not see this. The comment is accurate for internal audiences.

3. **`ProbeResult::evidence_trail`** — doc says it "records the DB entries that proved ownership" but the field type is `Vec<EvidenceRecord>`. The relationship between `ProbeResult` and `EvidenceBundle` (the phase pushes the records from `evidence_trail` into the main bundle) is not explained in the module doc. A brief note would help: "`ProbeResult::evidence_trail` records are pushed into the pipeline's `EvidenceBundle` by `pkg_substrate::run()`."

---

### `detect/substrate/rpm.rs`

**Overall**: Strong. The feature-gate explanation, trust model note, and TOCTOU mention are all accurate.

**Issues**:

1. **"Trust model" section** — says the RPM database is "untrusted input" parsed by the "TPI parser in `rpm_header`". The TPI characterization in this context means two independent parse paths are used within `rpm_header`. Verify this is actually the case (it appears to be from the `rpm_header.rs` module doc). If so, this is correct; no change needed.

2. **Feature gate note** — "Without it, the probe reverts to stub behaviour: presence checks only, `can_query_ownership = false`, `can_verify_digest = false`." These appear to be fields or return values, but there is no `can_query_ownership` field visible in the struct. Ensure these field names are accurate before publish.

---

### `detect/substrate/dpkg.rs`

**Overall**: Good. The stub nature is clearly flagged and the limitation on `query_ownership` is stated.

**Issues**:

1. **`facts_count = 1` vs `facts_count = 2`** — the module doc says "Returns `facts_count = 1` for the DB root alone, or `facts_count = 2` if the status file is also present." This means the dpkg stub can technically satisfy the T3 threshold (`facts_count >= 2`) with two weak filesystem-presence checks. The doc should note whether this is intentional or a known limitation of the stub.

2. **No compliance for `SI-7`** — the dpkg stub correctly declares `can_verify_digest = false`. The compliance block could add: "Declaring `can_verify_digest = false` ensures the pipeline does not attempt integrity verification and make false-positive claims."

---

### `detect/substrate/rpm_header.rs`

**Overall**: Good. The binary format documentation (nindex, hsize, 16-byte index entry layout) is clear and accurate.

**Issues**:

1. **TPI characterization** — this module is referenced from `rpm.rs` as containing a "TPI parser" but the module doc does not use the term "Two-Path Independent" or "TPI". If TPI parsing is actually implemented here (two independent parse strategies), the doc should state it explicitly with the standard TPI rationale. If it is a single-path parser with bounds checking, the `rpm.rs` reference is inaccurate and should be corrected.

2. **Format diagram** — the `text` code block for the binary format is clear. Good practice for binary protocol documentation.

---

### `detect/file_ownership.rs`

**Overall**: Good. Steps are numbered, TOCTOU note is present, and the phase boundary is clear.

**Issues**:

1. **Module `//!` heading** — uses `#` heading: `# File Ownership Phase`. Good.

2. **`## TOCTOU note`** — the heading is present but the section content (not visible in the preview) should state explicitly that `(dev, ino)` verification by the probe is the mitigation. This appears to be covered.

3. **Module is `pub(super)` (internal)** — the doc comment level of detail is appropriate given that external callers never interact with this module directly.

---

### `detect/integrity_check.rs` and `detect/pkg_substrate.rs`

These modules were reviewed from partial reads. Both have `#` headings, numbered steps, and compliance citations in the standard `NIST SP 800-53` format (pkg_substrate uses bare `NIST 800-53` without `SP` — same inconsistency as elsewhere).

`pkg_substrate.rs` — the Biba integrity pre-check for SELinux enforce is documented with a RAG Finding reference ("RAG Finding 5"). This has the same issue as the "Finding 1" and "Finding 3" references in other modules — internal review references with no meaning to future readers. Recommend replacing with the self-contained rationale.

---

## Consistency Recommendations

### 1. Compliance Citation Format (High Priority)

**Problem**: Two formats are in use across the codebase.

| Format used | Files |
|---|---|
| `NIST SP 800-53 SI-7` (correct) | `evidence.rs`, `confidence.rs`, `sealed_cache.rs`, `detect/mod.rs`, `detect/kernel_anchor.rs`, `detect/label_trust.rs` |
| `NIST 800-53 SI-7` (missing `SP`) | `lib.rs`, `kattrs/*.rs`, `posture/*.rs`, most others |

**Recommendation**: Standardize on `NIST SP 800-53` across all modules. The `SP` (Special Publication) designator is the correct abbreviated form. This is a sweep edit across approximately 30 citation occurrences.

### 2. Module-Level `//!` Headings (Medium Priority)

**Problem**: Many modules lack a `# Module Name — subtitle` opening line in their `//!` block, which means `cargo doc` renders these modules without a navigation-visible title.

Affected modules (from this review):
- `kattrs/mod.rs`, `kattrs/traits.rs`, `kattrs/types.rs`, `kattrs/procfs.rs`, `kattrs/sysfs.rs`, `kattrs/selinux.rs`, `kattrs/security.rs`, `kattrs/tpi.rs`
- `posture/signal.rs`, `posture/reader.rs`, `posture/configured.rs`, `posture/contradiction.rs`, `posture/snapshot.rs`, `posture/modprobe.rs`, `posture/fips_cross.rs`

**Recommendation**: Add `# ModuleName — Brief Description` as the first line of each `//!` block.

### 3. Internal Review References (Medium Priority)

**Problem**: Several doc comments reference internal security review findings by number ("Finding 1", "Finding 3", "Finding 5", "RAG Finding 5") that have no meaning to future developers or external auditors.

Affected locations:
- `os_identity.rs`: "ANSSI Rust Guide, Finding 1"
- `configured.rs` (inline comment): "Finding 3"
- `configured.rs` (inline comment): "Finding 4"
- `pkg_substrate.rs`: "RAG Finding 5"

**Recommendation**: Replace each reference with the self-contained technical rationale. The finding numbers can be preserved in a project CHANGELOG or audit record, not in source doc comments.

### 4. `kattrs/mod.rs` Style (High Priority)

**Problem**: This module's `//!` block uses all-caps plain-prose headings instead of Markdown, and lacks a `#` title. It is the only module in the crate with this style.

**Recommendation**: Rewrite to match `posture/mod.rs` or `detect/mod.rs` as the style reference. The content is accurate and should be preserved.

### 5. Copyright Block Punctuation (Low Priority)

**Problem**: `kattrs/mod.rs`, `kattrs/traits.rs`, `kattrs/types.rs`, `kattrs/selinux.rs`, `kattrs/tpi.rs`, and `kattrs/procfs.rs` all have `(a.k.a, Imodium Operator)` — misplaced comma after `a.k.a`. All other files have the correct form `(a.k.a. Imodium Operator)`.

**Recommendation**: Correct in the next editing pass.

---

## Future Documentation Requirements

The following gaps were identified from reading the source that are not addressed by source comment improvements alone. These require new documentation in the Antora docs (`docs/modules/`).

### Developer Guide Needs

1. **`detect` pipeline walkthrough** (`docs/modules/devel/pages/`) — The multi-phase pipeline has complex phase interactions, confidence propagation, and hard vs. soft gate semantics. A developer guide page explaining the full pipeline flow would be valuable. The `detect/mod.rs` module doc is a strong starting point but the per-phase docs are spread across seven files. A consolidated walkthrough is needed.

2. **Confidence model and trust tiers** — `confidence.rs` documents the tier system well, but there is no Antora page explaining the T0–T4 model, what gates earn each tier, and how callers should interpret the final `TrustLevel` in a `DetectionResult`. The `os-detection-deep-dive.adoc` mentioned in the `devel/index.adoc` may cover this; verify and fill gaps.

3. **`posture` module guide** — The posture probe has a Quick Start in `posture/mod.rs` but no Antora page. Given the number of types and the complexity of the contradiction detection system, a developer guide page is warranted.

### Pattern Documentation Needs

4. **Sealed Evidence Cache (SEC) pattern page** — `sealed_cache.rs` documents the HMAC-SHA-256 sealing pattern thoroughly in the module doc, but the high-assurance patterns library (`docs/modules/patterns/pages/`) should have a dedicated SEC page. The `sealed_cache.rs` doc is sufficient source material for that page.

5. **Provenance verification pattern** — The `SecureReader` / fd-anchored `fstatfs` pattern is described in `kattrs/mod.rs` and `kattrs/traits.rs` but there may not be a standalone pattern page for it. Verify coverage in `docs/modules/patterns/pages/`.

6. **Trust Gate pattern** — The trust gate rule (only read configuration when the kernel confirms the subsystem is active) is documented in `fips_cross.rs`, `modprobe.rs`, and `configured.rs`. A standalone pattern page would consolidate this.

### Architecture Documentation Needs

7. **`umrs-platform` architecture overview** — `lib.rs` gives a module map but there is no architecture page explaining how `kattrs`, `detect`, `posture`, `confidence`, and `evidence` fit together as a system. An architecture page in `docs/modules/architecture/` would serve security auditors and new developers.

8. **EvidenceBundle audit trail design** — The append-only AU-10 invariant is well documented in code but the rationale (why append-only? what does AU-10 require?) should be in the architecture or patterns docs.

---

## Summary Table

| File | Priority | Issues |
|---|---|---|
| `lib.rs` | Medium | Missing `sealed_cache` in module table; citation format |
| `evidence.rs` | Low | Grammar in opening paragraph; truncation policy wording |
| `confidence.rs` | Low | None significant |
| `os_identity.rs` | Medium | Internal finding references; `CpuArch` cross-check claim |
| `os_release.rs` | Medium | HTTP URL note; cross-reference to `release_parse` |
| `sealed_cache.rs` | Low | `SP` missing from `NIST 800-218`; arrow-notation style |
| `kattrs/mod.rs` | High | Style overhaul needed; missing `#` heading; punctuation |
| `kattrs/traits.rs` | Medium | Missing `#` heading; `execute_read_text` doc wording |
| `kattrs/types.rs` | Low | Missing `#` heading; `DualBool` pending state explanation |
| `kattrs/procfs.rs` | Low | Missing `#` heading; citation format |
| `kattrs/sysfs.rs` | Low | Missing `#` heading; citation format |
| `kattrs/selinux.rs` | Medium | Missing `#` heading; synthetic kobject name note |
| `kattrs/security.rs` | Medium | Missing `#` heading; citation format; path-function naming |
| `kattrs/tpi.rs` | Medium | Missing `#` heading; private function docs; cross-crate scope note |
| `posture/mod.rs` | Medium | Citation format; Architecture table detail |
| `posture/signal.rs` | Low | Citation format; `DesiredValue::Custom` method name |
| `posture/catalog.rs` | Low | Citation format; `SignalDescriptor` stability note |
| `posture/reader.rs` | Low | Missing `#` heading; "as specified in the plan" removal |
| `posture/configured.rs` | Medium | Missing `#` heading; internal finding references |
| `posture/contradiction.rs` | Medium | Missing `#` heading; signed integer gap in `evaluate_configured_meets` |
| `posture/snapshot.rs` | Low | Missing `#` heading; citation format |
| `posture/modprobe.rs` | Medium | Missing `#` heading; `is_module_loaded` empty-string guard elevation |
| `posture/fips_cross.rs` | Low | Missing `#` heading; citation format; `SI-10` citation for fail-closed |
| `detect/mod.rs` | Low | Duplicate `SA-8` citation; `LabelTrust` note |
| `detect/kernel_anchor.rs` | Low | Lockdown tier retrieval note |
| `detect/mount_topology.rs` | Medium | `AT_EMPTY_PATH` inaccuracy; `statfs` limitation note |
| `detect/label_trust.rs` | Low | None significant |
| `detect/substrate/mod.rs` | Low | `ProbeResult` → `EvidenceBundle` flow note |
| `detect/substrate/rpm.rs` | Low | Verify TPI claim |
| `detect/substrate/dpkg.rs` | Low | T3 threshold from two presence checks |
| `detect/substrate/rpm_header.rs` | Medium | TPI claim — verify or correct |

---

*This report covers review-only findings. No source files were modified. All corrections are editorial or clarification changes; no architectural changes are implied.*
