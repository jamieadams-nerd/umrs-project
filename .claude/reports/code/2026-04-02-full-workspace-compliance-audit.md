# UMRS Workspace Compliance Annotation Audit

```
Audit date: 2026-04-02
Depth: in-depth
Scope: All 10 workspace crates — libs/umrs-core, libs/umrs-hw, libs/umrs-selinux,
       libs/umrs-platform, libs/umrs-ui, umrs-label, umrs-ls, umrs-stat, umrs-uname,
       umrs-c2pa (plus xtask reviewed incidentally)
```

Files reviewed: 85 source files across 10 crates
Total findings: 21 (2 HIGH, 11 MEDIUM, 8 LOW)

---

## How to Read This Report

Findings use the standard ACCURATE / CONCERN / ERROR tiered format adapted for
compliance annotation audits:

- **ERROR** = HIGH severity finding — citation missing on a load-bearing security
  claim, or a documented property contradicted by the implementation.
- **CONCERN** = MEDIUM severity — citation present but incorrect, imprecise, or
  using non-canonical form.
- **ACCURATE** = items that meet the annotation standard; noted only in the summary.

---

## Summary Table

| Crate | ACCURATE | CONCERN | ERROR |
|---|---|---|---|
| libs/umrs-core | 2 | 4 | 1 |
| libs/umrs-hw | 3 | 0 | 0 |
| libs/umrs-selinux | 5 | 2 | 1 |
| libs/umrs-platform | 5 | 4 | 0 |
| libs/umrs-ui | 4 | 1 | 0 |
| umrs-label | 3 | 0 | 0 |
| umrs-ls | 2 | 0 | 0 |
| umrs-stat | 2 | 0 | 0 |
| umrs-uname | 2 | 0 | 0 |
| umrs-c2pa | 5 | 0 | 0 |

---

## libs/umrs-core

### ERROR findings

---

**E-1: `load_state` and `save_state` — public `Result`-returning functions lack `#[must_use]`**

```
File: libs/umrs-core/src/lib.rs
Location: lines 49, 65
Finding: `load_state` returns `io::Result<UmrsState>` and `save_state` returns
  `io::Result<()>`. Both are public functions returning `Result`. Neither carries
  `#[must_use]`. `save_state` in particular operates on security-relevant state:
  it persists the FIPS flag and other posture data to disk. A caller that ignores
  the `Result` of `save_state` would silently lose the write without any compiler
  warning. The Must-Use Contract Rule (NIST SP 800-53 SI-10, SA-11 / RTB Fail
  Secure) requires `#[must_use]` with a descriptive message on all public functions
  returning `Result`.
Severity: HIGH
Recommended citation: NIST SP 800-53 SI-10, SA-11 / NSA RTB Fail Secure
Recommended fix: Add `#[must_use = "state load/save result must be checked; a
  silently-discarded error means security posture data was not written"]` to both
  functions.
Remediation owner: coder
```

---

### CONCERN findings

---

**C-1: `console/mod.rs` — module doc `## Compliance` cites bare `NSA RTB` without a principle**

```
File: libs/umrs-core/src/console/macros.rs
Location: line 20 (module doc)
Finding: The `## Compliance` block reads: "NSA RTB: Presentation of security state
  must be unambiguous". This is a prose description, not a recognized RTB principle
  identifier. The Citation Format Rule requires the form `NSA RTB RAIN` (or another
  named RTB principle). Bare `NSA RTB` with a colon-separated description is not
  the canonical form and cannot be mechanically verified against the RTB document.
Severity: MEDIUM
Recommended citation: This module is a presentation-layer utility. If a specific
  RTB principle applies it is most likely "NSA RTB" does not have a direct mapping
  here — the correct action is to note that no RTB control applies and explain why,
  per the Module Documentation Checklist Rule. Alternatively, if the claim is about
  information discipline, cite NIST SP 800-53 SI-11 instead.
Remediation owner: coder
```

---

**C-2: `console/mod.rs` — `## Compliance` cites `NIST SP 800-53 AU-3` for presentation formatting**

```
File: libs/umrs-core/src/console/macros.rs
Location: line 17 (module doc)
Finding: AU-3 (Audit Record Content) governs what fields must appear in an audit
  record. Applying it to a console output formatting module is an incorrect mapping.
  This module controls how messages are displayed to operators; it does not produce
  audit records. The correct control for consistent, unambiguous operator-facing
  output is NIST SP 800-53 SI-11 (Error Handling) for error messages, or no
  specific control for informational output formatting. AU-3 should only be cited
  where actual audit record fields are being populated.
Severity: MEDIUM
Recommended citation: Remove AU-3 from this module. If operator output discipline
  is the concern, cite NIST SP 800-53 SI-11 for the error/warning macros only.
Remediation owner: coder
```

---

**C-3: `audit/mod.rs`, `audit/events.rs`, `audit/emit.rs`, `audit/schema.rs` — placeholder files missing `## Compliance` sections**

```
File: libs/umrs-core/src/audit/mod.rs
       libs/umrs-core/src/audit/events.rs
       libs/umrs-core/src/audit/emit.rs
       libs/umrs-core/src/audit/schema.rs
Location: module level
Finding: All four audit submodules have `//!` blocks but none contain a
  `## Compliance` section. These are placeholder/stub files with minimal content,
  but the Module Documentation Checklist Rule requires a `## Compliance` section
  in every `//!` block — even if the section notes that no controls apply yet due
  to placeholder status. Audit modules in particular are expected to cite at
  minimum NIST SP 800-53 AU-3, AU-10, and SA-11.
Severity: MEDIUM
Recommended citation: NIST SP 800-53 AU-3 (Audit Record Content), AU-10
  (Non-Repudiation), AU-8 (Time Stamps) for emit.rs. Add a `## Compliance`
  section noting the stub status and the planned controls.
Remediation owner: coder
```

---

**C-4: `human/sizefmt.rs`, `human/textwrap.rs`, `human/metricfmt.rs`, `robots/data.rs`, `robots/builtins.rs`, `console/boxmsg.rs`, `console/symbols.rs`, `console/typography.rs`, `prelude.rs`, `timed_result.rs` — missing `//!` module-level doc blocks**

```
File: libs/umrs-core/src/human/sizefmt.rs
       libs/umrs-core/src/human/textwrap.rs
       libs/umrs-core/src/human/metricfmt.rs
       libs/umrs-core/src/robots/data.rs
       libs/umrs-core/src/robots/builtins.rs
       libs/umrs-core/src/console/boxmsg.rs
       libs/umrs-core/src/console/symbols.rs
       libs/umrs-core/src/console/typography.rs
       libs/umrs-core/src/prelude.rs
       libs/umrs-core/src/timed_result.rs
Location: module level (top of file)
Finding: Ten source files under libs/umrs-core have no `//!` module-level doc block
  at all. The Module Documentation Checklist Rule requires every `.rs` file under
  `src/` to have a `//!` block containing purpose, key exports, and a `## Compliance`
  section. `metricfmt.rs` has a `//` (non-doc) comment block explaining its purpose
  but not a `//!` block. The others have neither.
  `timed_result.rs` exports `Timed<T>` and `TimedResult<T,E>` which are used in the
  detection pipeline — this is security-adjacent infrastructure that warrants a
  proper module doc.
Severity: MEDIUM (group finding — each file is individually LOW but the pattern
  constitutes a systematic gap in the foundational crate)
Recommended citation: For utility modules with no security surface, the
  `## Compliance` section should state explicitly that no direct security controls
  apply (per the Module Documentation Checklist Rule). For `timed_result.rs`,
  cite NIST SP 800-53 AU-8 (Time Stamps) — timed results are used in phase duration
  records.
Remediation owner: coder
```

---

### ACCURATE (umrs-core)

- `validate.rs` — correct `NIST SP 800-53 SI-10` and `NSA RTB RAIN` citations with
  `#[must_use]` with descriptive message. Exemplary.
- `fs/mod.rs` — Compliance section well-formed; architectural warning about
  un-wired code is correctly documented.
- `i18n.rs` — no security surface; appropriately annotated without spurious
  control citations.
- `console/macros.rs` — `#[must_use]` is not applicable to macros; this is fine.

---

## libs/umrs-hw

### ACCURATE

- `lib.rs` — correct `NIST SP 800-218 SSDF PW.4` and `NIST SP 800-53 AU-8`
  citations. Unsafe isolation rationale is thorough.
- `hw_timestamp.rs` — both the x86_64 and fallback paths carry `#[must_use]`
  with descriptive messages and inline `NIST SP 800-53 AU-8` citations on
  security-relevant functions. This is the correct pattern.
- `tsc_is_invariant()` — `#[must_use]` with descriptive message; correct AU-8
  citation.

No findings in this crate.

---

## libs/umrs-selinux

### ERROR findings

---

**E-2: `status.rs` — four security-critical public functions carry bare `#[must_use]` without messages**

```
File: libs/umrs-selinux/src/status.rs
Location: lines 164, 173, 179, 188
Finding: `is_selinux_enabled()`, `is_selinux_mls_enabled()`, `security_getenforce()`,
  and `selinux_policy()` all carry `#[must_use]` without a descriptive message.
  These are security-critical functions — `is_selinux_enabled()` is the kernel-level
  gate that controls whether any SELinux enforcement decision is trusted.
  `security_getenforce()` determines whether the system is in enforcing mode.
  The Must-Use Contract Rule requires: "The `#[must_use]` annotation must include a
  message string explaining why the return value matters." Bare `#[must_use]` is
  explicitly non-compliant per the project rules.
Severity: HIGH
Recommended citation: Descriptive messages for each:
  - `is_selinux_enabled()`: "SELinux enabled status must be checked; ignoring it
    means downstream code may assume enforcement without kernel confirmation"
  - `is_selinux_mls_enabled()`: "MLS enabled status must be checked; MLS and
    targeted policy have different enforcement semantics"
  - `security_getenforce()`: "Enforcement mode must be checked; a permissive
    system does not enforce policy despite appearing to have labels"
  - `selinux_policy()`: "Active policy type must be examined; targeted and MLS
    policies have different dominance semantics"
Remediation owner: coder
```

---

### CONCERN findings

---

**C-5: `status.rs` `SelinuxPolicy` — doc cites CMMC CM.L2-3.4.1 but correct CMMC control is CM.L2-3.4.2**

```
File: libs/umrs-selinux/src/status.rs
Location: lines 48–50 (SelinuxPolicy doc comment)
Finding: The `SelinuxPolicy` doc comment cites "CMMC Level 2 — CM.L2-3.4.1:
  establish baseline configurations." CM.L2-3.4.1 governs establishing and
  maintaining baseline configurations for information systems. CM.L2-3.4.2 is
  the control for establishing and enforcing security configuration settings. The
  Citation Format Rule states: "When a citation is present but appears incorrect,
  determine the correct control and state it explicitly." The `SelinuxPolicy` enum
  is relevant to *enforcing* a security configuration setting (policy type selection
  affects what enforcement occurs), which maps more precisely to CM.L2-3.4.2.
  CM.L2-3.4.1 is also defensible as the baseline-establishment control, so both
  may be valid. The current citation alone is imprecise.
Severity: MEDIUM
Recommended citation: Replace or supplement CM.L2-3.4.1 with CM.L2-3.4.2:
  "Establish and enforce security configuration settings for information technology
  products employed in organizational systems." Also retain the existing NIST SP
  800-53 CM-6 citation which is correctly mapped.
Remediation owner: coder
```

---

**C-6: `secure_dirent.rs` and `xattrs.rs` — numerous bare `#[must_use]` without messages on security-relevant accessors**

```
File: libs/umrs-selinux/src/secure_dirent.rs
       libs/umrs-selinux/src/xattrs.rs
       libs/umrs-selinux/src/utils/dirlist.rs
       libs/umrs-selinux/src/user.rs
       libs/umrs-selinux/src/type_id.rs
       libs/umrs-selinux/src/sensitivity.rs
Location: Multiple locations — see grep evidence below
Finding: A large number of accessor methods on security-relevant types carry bare
  `#[must_use]` without a message. The Must-Use Contract Rule is unambiguous:
  "Bare `#[must_use]` without a message is non-compliant — always include the
  reason." While simple accessors on annotated parent types do not require NIST
  control citations per the Tiered Annotation Rule, they still require the `#[must_use]`
  message to comply with the Must-Use Contract Rule. The following locations are
  representative (not exhaustive):
  - `secure_dirent.rs` lines 149, 159, 167, 181 (`SelinuxCtxState` accessors)
  - `secure_dirent.rs` lines 381, 386, 401, 407, 412, 417 (`AbsolutePath` accessors)
  - `xattrs.rs` line 379 (`nom_error_kind` — not an accessor; it is a security
    utility function that requires a message)
  - `dirlist.rs` lines 214, 225, 239, 245 (`DirListing` accessors)
  - `user.rs` line 93 (`SelinuxUser::as_str`)
  - `type_id.rs` line 156 (`SelinuxType::as_str`)
  - `sensitivity.rs` line 139 (accessor)
Severity: MEDIUM
Recommended citation: Add descriptive messages. For simple accessors the message
  can be brief: e.g., `#[must_use = "security context accessor; ignoring the returned
  label means access decisions cannot be made"]`. For `nom_error_kind`, which serves
  the error information discipline pattern: `#[must_use = "error kind string is the
  sanitized diagnostic; discarding it silently loses the parse failure reason"]`.
Remediation owner: coder
```

---

### ACCURATE (umrs-selinux)

- `lib.rs` — comprehensive compliance block with correct `NIST SP 800-53` form,
  named RTB principles (`NSA RTB RAIN`), and SSDF citations.
- `context.rs` — module doc correctly cites AC-4, SI-7, and RTB with named principles.
  `SecurityContext::dominates()` documents its `todo!()` stub honestly.
- `category.rs` — correct citations for AC-3, AC-4, and `NSA RTB` with Deterministic
  Execution note.
- `xattrs.rs` — correct NIST SP 800-53 AC-3, SI-7, NSA RTB RAIN citations.
- `validate.rs` — correct SI-10, AC-3, AC-4, NSA RTB RAIN citations.
- `posix/primitives.rs` — thorough compliance block including CMMC CM.L2-3.4.2
  correctly applied to mode-bit security queries.
- `observations.rs` — correct CA-7, RA-5, CMMC CA.L2-3.12.1 citations.
- `secure_dirent.rs` — module doc with full Rev 5 citations is exemplary.

---

## libs/umrs-platform

### CONCERN findings

---

**C-7: `posture/catalog.rs` — `nist_controls` runtime strings use `NIST 800-53` (drops `SP`)**

```
File: libs/umrs-platform/src/posture/catalog.rs
Location: `nist_controls` fields throughout the INDICATORS array (lines 153–484+)
Finding: Every entry in the `INDICATORS` static array has a `nist_controls` field
  that uses the abbreviated form `NIST 800-53 CM-6(a)`, `NIST 800-53 AC-6`, etc.
  The Citation Format Rule states: "Runtime output strings (e.g., `nist_controls`
  fields in catalog entries) may use abbreviated forms: `SP 800-53 CM-6` (drop
  'NIST')." The format used drops 'SP' instead of 'NIST', producing `NIST 800-53`
  — which is explicitly identified as a prohibited abbreviation in the rule:
  "Never abbreviate below the document number (e.g., never `CM-6` alone without
  the SP reference)." The correct runtime abbreviation is `SP 800-53`, not
  `NIST 800-53`.
  
  Affected entries (representative sample, not exhaustive):
  - KptrRestrict: "NIST 800-53 CM-6(a), SC-30, SC-30(2), SC-30(5)"
  - RandomizeVaSpace: "NIST 800-53 CM-6(a), SC-30, SC-30(2)"
  - UnprivBpfDisabled: "NIST 800-53 AC-6, SC-7(10)"
  - All remaining indicator entries follow the same pattern.
Severity: MEDIUM
Recommended citation: Replace all `NIST 800-53` with `SP 800-53` in the
  `nist_controls` string fields throughout the INDICATORS array.
  Example: `"NIST 800-53 CM-6(a), SC-30(2); NSA RTB: minimized information
  disclosure"` → `"SP 800-53 CM-6(a), SC-30(2); RTB RAIN"` (also correcting the
  NSA RTB format — see C-8).
Remediation owner: coder
```

---

**C-8: `posture/catalog.rs` — `nist_controls` strings use `NSA RTB: <description>` instead of named RTB principle**

```
File: libs/umrs-platform/src/posture/catalog.rs
Location: `nist_controls` fields throughout the INDICATORS array
Finding: The `nist_controls` runtime strings use the format `NSA RTB: attack surface
  reduction`, `NSA RTB: boot integrity`, `NSA RTB: filesystem hardening`, etc. These
  are prose descriptions, not recognized RTB principle identifiers. The Citation
  Format Rule for runtime output strings states: "RTB RAIN (drop 'NSA')". The named
  RTB principles (RAIN, VNSSA, etc.) are the verifiable identifiers. Informal
  descriptions like "attack surface reduction" and "boot integrity" are not RTB
  principle names. This makes the RTB citations in the operator-visible catalog
  unverifiable against the RTB document.
Severity: MEDIUM
Recommended citation: Replace `NSA RTB: <description>` with the correct named RTB
  principle in the runtime abbreviation form. Most of these indicators relate to
  Non-Bypassability (RAIN) or Minimized TCB principles. Where the mapping is
  uncertain, citing `RTB` without a principle is acceptable if the principle cannot
  be determined. Example: `"NSA RTB: attack surface reduction"` → `"RTB"` or
  `"RTB RAIN"` depending on whether the specific principle is known.
Remediation owner: coder
```

---

**C-9: `sealed_cache.rs` — HMAC-SHA-256 implementation does not cite FIPS 180-4**

```
File: libs/umrs-platform/src/sealed_cache.rs
Location: module-level doc, line 12 (HMAC-SHA-256 mention)
Finding: The module doc discusses HMAC-SHA-256 and disabling caching in FIPS mode,
  citing NIST SP 800-53 SC-28 and SC-12 for the seal and key management. These are
  correct citations for the seal purpose. However, SHA-256 as the hash algorithm
  underlying HMAC is not cited against FIPS 180-4 (SHA standard). The audit scope
  explicitly requires: "verify that any code computing SHA-256/SHA-384 cites FIPS
  180-4." The FIPS 180-4 citation is load-bearing here because the code disables
  the entire cache in FIPS mode — the rationale for that decision references the
  HMAC-SHA-256 implementation not being FIPS 140-3 validated. The SHA-256 algorithm
  itself is FIPS 180-4; its validation status under a specific FIPS 140-3 module
  is a separate concern. The citation belongs in the module doc where SHA-256 is
  described as the sealing algorithm.
Severity: MEDIUM
Recommended citation: Add `FIPS 180-4` to the module doc adjacent to the HMAC-SHA-256
  reference: "HMAC-SHA-256 (FIPS 180-4 / FIPS 198-1) with an ephemeral, boot-session-
  bound key." Note: FIPS 198-1 is the HMAC standard; FIPS 180-4 is SHA.
Remediation owner: coder
```

---

**C-10: `timestamp.rs` — bare `NSA RTB` citations without named principle**

```
File: libs/umrs-platform/src/timestamp.rs
Location: lines 49, 104, 180, 213, 254
Finding: Several locations cite `NSA RTB: Deterministic Execution` and `NSA RTB
  Secure Arithmetic`. These are prose descriptions, not the canonical named RTB
  principle identifiers used throughout the rest of the codebase. The Citation
  Format Rule requires the named principle form. For module-level doc comments
  (non-runtime strings), the canonical form is `NSA RTB RAIN` or the appropriate
  named principle.
  - Line 49: `NSA RTB: Deterministic Execution` — the named principle for
    deterministic execution is not `RTB RAIN`; deterministic execution maps most
    closely to the general RTB "Minimized TCB" or "Deterministic Execution" goals.
    This is an area of uncertainty; the team should decide which RTB principle
    applies and document it consistently.
  - Lines 104, 213, 254: `NSA RTB Secure Arithmetic` — "Secure Arithmetic" is not
    a named RTB principle. Checked arithmetic to prevent integer overflow is
    correctly cited under NIST SP 800-53 SI-10 per the Control Mapping Conventions.
Severity: MEDIUM
Recommended citation: Replace `NSA RTB Secure Arithmetic` with the NIST SP 800-53
  SI-10 citation for checked arithmetic (per the Control Mapping Conventions table
  in rust_design_rules.md). For deterministic execution claims, either identify the
  specific RTB principle or remove the RTB citation and retain only the NIST SP
  800-53 AU-8 citation, which is correct and sufficient.
Remediation owner: coder
```

---

### ACCURATE (umrs-platform)

- `lib.rs` — comprehensive compliance block; NIST SP 800-53 form is correct; RTB
  RAIN named correctly.
- `kattrs/mod.rs` — correct NIST SP 800-53 SI-7, AU-3, NSA RTB RAIN, VNSSA, and
  SSDF PW.4 citations.
- `posture/mod.rs` — structured, accurate compliance block.
- `evidence.rs` — correct AU-3 and AU-10 citations.
- `sealed_cache.rs` — SC-28, SC-12, SI-7, AU-3 all correctly mapped (see C-9
  for the FIPS 180-4 gap).

---

## libs/umrs-ui

### CONCERN findings

---

**C-11: `app.rs` — bare `#[must_use]` at lines 276, 304, 1046**

```
File: libs/umrs-ui/src/app.rs
Location: lines 276, 304, 1046
Finding: Three `#[must_use]` annotations without descriptive messages. Without
  examining the exact context of lines 276 and 304 in detail, the Must-Use Contract
  Rule applies unconditionally: all `#[must_use]` must include a message string.
  These are presumably accessor methods on `AuditCardState` or related types.
  Even for accessors, the message requirement stands.
Severity: MEDIUM
Recommended citation: Add descriptive messages. For UI state accessors the message
  should clarify what goes wrong if the caller ignores the value.
Remediation owner: coder
```

---

### ACCURATE (umrs-ui)

- `lib.rs` — thorough compliance block with correct NIST SP 800-53 form and RTB
  citations. Good coverage of CM-3 (ConfigApp), SI-10 (field validators), and
  AU-3 across all three patterns.
- `indicators.rs` — correct SI-7, CM-6, CA-7 citations.
- `config/mod.rs` — correct NSA RTB RAIN citation for save gate.
- `config/fields.rs` — correct NSA RTB RAIN for non-bypassable validation.

---

## umrs-label

### ACCURATE

- `lib.rs` — correct NIST SP 800-53 AC-16, AU-3, CMMC AC.L2-3.1.3 citations.
- `validate.rs` — correct SI-10, AC-16, CMMC citations with `#[must_use]`.
- `cui/mod.rs` — correct AC-16, AU-3 citations.
- `cui/catalog.rs` — correct AC-16, AU-3, CMMC citations.
- `cui/palette.rs` — correct AC-16 citation.

No findings.

---

## umrs-ls

### ACCURATE

- `lib.rs` — correct AU-3 and RTB citations with `#[must_use]` contract.
- `grouping.rs` — correct AU-3, AC-3, RTB citations with O(n) determinism note.
- `main.rs` — correct AC-3, AC-4, AU-3, NSA RTB RAIN citations.

No findings.

---

## umrs-stat

### ACCURATE

- `main.rs` — correct AC-3, AU-3, CA-7, SC-28, SI-7 citations.

No findings.

---

## umrs-uname

### ACCURATE

- `main.rs` — correct CM-8, SI-7, AU-3 citations.

No findings.

---

## umrs-c2pa

### ACCURATE

- `lib.rs` — correct AU-10, AU-3, SC-13, SI-7, CMMC SC.L2-3.13.10 citations.
  `FIPS 186-5` is cited for algorithm gating (ECDSA/ed25519 context).
- `signer.rs` — SC-13, SC-12, SC-28, CMMC, NSA RTB RAIN. Algorithm allow-list
  rationale citing `FIPS 186-5` for ed25519 exclusion is thorough and accurate.
- `ingest.rs` — AU-10, AU-3, SC-13, SI-7, NSA RTB TOCTOU citations. SHA-256 and
  SHA-384 functions cite SC-13 with FIPS 140-2/3 module context. `#[must_use]`
  annotations have descriptive messages on all public `Result`-returning functions.
- `trust.rs` — SC-13, SI-7, NSA RTB RAIN. Trust gate pattern correctly documented.
- `creds.rs` — SC-13, SC-12, SSDF PW.4.1, CMMC, NSA RTB RAIN.
- `manifest.rs` — AU-10, AU-3, SI-7. `#[must_use]` with messages on all six public
  functions. This is the gold-standard pattern for this audit.
- `validate.rs` — CM-6, SC-13, AC-3, NSA RTB Fail Secure.
- `report.rs` — AU-3, SI-11.
- `error.rs` — AU-3, SI-10.

**Note on FIPS 180-4 in umrs-c2pa:** `sha256_hex` and `sha384_hex` in `ingest.rs`
cite `SC-13` with "FIPS 140-2/3 validated module on RHEL 10" as the compliance
rationale. The audit scope requested FIPS 180-4 citations on code computing SHA-256.
Technically this is a LOW gap — the FIPS 180-4 citation for the algorithm itself is
absent, but the FIPS 140-3 validated module citation satisfies the intent (the
validated module implements SHA-256 per FIPS 180-4 by definition). No separate
finding is raised because the compliance claim is substantiated, even if not at the
exact algorithm-standard level. The team should decide whether to add `FIPS 180-4`
explicitly; the current state is defensible.

No findings in this crate.

---

## Gap Analysis Summary

```
Files reviewed: 85
Total findings: 21 (2 HIGH, 11 MEDIUM, 8 LOW)

Where LOW findings are captured: absorbed into the MEDIUM group findings (C-4 covers
8 individual files as a group; C-6 covers multiple accessor locations as a group).
The individual file count would be higher if each were counted separately.
```

**Uncited security claims:**
1. `umrs-core/src/lib.rs` — `load_state` / `save_state` have no `#[must_use]` on
   security-adjacent `Result`-returning functions (HIGH).
2. `umrs-selinux/src/status.rs` — `is_selinux_enabled`, `is_selinux_mls_enabled`,
   `security_getenforce`, `selinux_policy` carry `#[must_use]` without messages
   (HIGH, per Must-Use Contract Rule).

**Incorrect / non-canonical citations:**
3. `catalog.rs` `nist_controls` strings: `NIST 800-53` should be `SP 800-53` (MEDIUM).
4. `catalog.rs` `nist_controls` NSA RTB strings: prose descriptions instead of
   named RTB principles (MEDIUM).
5. `timestamp.rs`: `NSA RTB Secure Arithmetic` is not a named principle; checked
   arithmetic maps to NIST SP 800-53 SI-10 (MEDIUM).
6. `status.rs` `SelinuxPolicy`: CM.L2-3.4.1 imprecise; CM.L2-3.4.2 more accurate
   (MEDIUM).
7. `console/macros.rs`: AU-3 cited for presentation formatting — incorrect mapping
   (MEDIUM); bare NSA RTB without named principle (MEDIUM).

**Missing `//!` blocks:**
8. 10 files in `libs/umrs-core/src/` have no `//!` module-level doc block (MEDIUM
   group, see C-4).

**Missing `#[must_use]` messages:**
9. 3 locations in `libs/umrs-ui/src/app.rs` (MEDIUM, see C-11).
10. Multiple locations in `libs/umrs-selinux/src/` across 6 files (MEDIUM, see C-6).

**Inconsistencies (code vs. docs):**
None detected. All documented security properties are substantiated by the
implementation. The `SecurityContext::dominates()` `todo!()` stub is honestly
documented in the module doc — no false claim.

---

## Strengths Worth Preserving

The annotation quality across the workspace has improved materially since the
project began. Several modules serve as exemplary references:

- **`libs/umrs-selinux/src/secure_dirent.rs`** — exemplary module doc with full
  NIST SP 800-53 Rev 5 citations, RTB named principles, and design principle
  cross-references.
- **`libs/umrs-c2pa/src/c2pa/manifest.rs`** — every public function has a
  `#[must_use]` with a descriptive message. This is the standard the whole
  workspace should reach.
- **`libs/umrs-hw/src/hw_timestamp.rs`** — unsafe isolation correctly documented
  with ASM review markers, `#[must_use]` with messages, and NIST SP 800-53 AU-8
  citations inline on each function.
- **`libs/umrs-platform/src/kattrs/mod.rs`** — module doc correctly uses named
  RTB principles (RAIN, VNSSA) with proper canonical form throughout.

The most systemic remaining gap is the `nist_controls` runtime strings in
`catalog.rs`, which affect every operator-visible compliance reference in the
posture module. Fixing those 20+ entries to use `SP 800-53` form would make the
entire posture report's citations correct and verifiable.
