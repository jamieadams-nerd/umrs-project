# umrs-platform Source Comment Review
**Date:** 2026-03-14
**Reviewer:** tech-writer
**Scope:** All `.rs` files under `components/rusty-gadgets/umrs-platform/src/`

---

## Summary

The comment quality across `umrs-platform` is generally strong. Module-level `//!` blocks are well-structured, compliance citations are present at the right levels, and the high-assurance patterns are clearly explained. The issues found fall into three categories:

1. **Citation Format Rule violations** — The most widespread finding. `catalog.rs` has pervasive bare `NIST 800-53` citations (missing `SP`). Several other files use bare `NIST 800-218` (missing `SP`). Two inline comments use `SSDF PW 4.1` (missing dot separator).
2. **Source Comment Discipline Rule violations** — A handful of trait-level associated-constant items have individual `NIST SP 800-53 AU-3` lines that add annotation noise without value.
3. **Minor precision issues** — One duplicate citation line, one `#[must_use]` without a message on a security-status accessor, and one vague comment on a private helper.

---

## Findings by File

---

### `posture/catalog.rs`

**Lines 20, 24, 26, 28 — module-level `//!` block: bare `NIST 800-` citations (missing `SP`)**

All four citations in the module doc omit `SP`.

Current (lines 20, 24, 26, 28):
```rust
//! under NIST 800-218 SSDF PW.4.
//! NIST 800-53 CM-6: Configuration Settings — ...
//! NIST 800-53 CA-7: Continuous Monitoring — ...
//! NIST 800-218 SSDF PW.4: Compile-time binding ...
```

Required:
```rust
//! under NIST SP 800-218 SSDF PW.4.
//! NIST SP 800-53 CM-6: Configuration Settings — ...
//! NIST SP 800-53 CA-7: Continuous Monitoring — ...
//! NIST SP 800-218 SSDF PW.4: Compile-time binding ...
```

---

**Lines 43–45 — `SignalDescriptor` doc: bare `NIST 800-` citations**

Current:
```rust
/// NIST 800-53 CM-6: each descriptor captures the security baseline
/// (desired value) alongside its rationale and NIST control citation.
/// NIST 800-53 AU-3: `nist_controls` provides the audit control mapping
```

Required:
```rust
/// NIST SP 800-53 CM-6: each descriptor captures the security baseline
/// (desired value) alongside its rationale and NIST control citation.
/// NIST SP 800-53 AU-3: `nist_controls` provides the audit control mapping
```

---

**Lines 64, 77, 78 — `SignalDescriptor` field doc and `SIGNALS` doc: bare `NIST 800-` citations**

Current (line 64):
```rust
    /// Applicable NIST 800-53 and NSA RTB control references.
```
Required:
```rust
    /// Applicable NIST SP 800-53 and NSA RTB control references.
```

Current (lines 77–78):
```rust
/// NIST 800-53 CA-7: the catalog is the enumerated monitoring scope.
/// NIST 800-53 CM-6: each entry encodes the security baseline.
```
Required:
```rust
/// NIST SP 800-53 CA-7: the catalog is the enumerated monitoring scope.
/// NIST SP 800-53 CM-6: each entry encodes the security baseline.
```

---

**Lines 90–403 (all `nist_controls` field values) — runtime strings: bare `NIST 800-53`**

Every `nist_controls: "..."` string in the `SIGNALS` array uses bare `NIST 800-53`. These are runtime display strings, not doc comments. Per the Citation Format Rule: "Runtime output strings (e.g., `nist_controls` fields in catalog entries) may use abbreviated forms for display compactness."

**No change required.** The bare form is intentional and within the rule's explicit exception.

---

### `sealed_cache.rs`

**Line 378 — inline `//` comment: bare `NIST 800-218`**

Context: a doc comment on a private helper function.

Current:
```rust
/// NIST 800-218 SSDF PW.4.
```

Required:
```rust
/// NIST SP 800-218 SSDF PW.4.
```

---

**Lines 189–193 — `detect()` doc: duplicate `NIST SP 800-53 SA-8` citation**

The function has two citation lines where the first subsumes the second.

Current (lines 189–190):
```rust
    /// NIST SP 800-53 SA-8, CM-6, SI-7.
    /// NIST SP 800-53 SA-8 — orchestrates layered, fail-closed platform verification pipeline.
```

The second line repeats SA-8 and its summary already belongs in the first line or the prose above. The first line is a citation; the second should either be removed or its rationale merged.

Recommended:
```rust
    /// NIST SP 800-53 SA-8, CM-6, SI-7 — orchestrates a layered, fail-closed platform
    /// verification pipeline; hard gates abort on kernel channel compromise.
```

*(Note: this finding is in `detect/mod.rs`, not `sealed_cache.rs` — see below.)*

---

### `detect/mod.rs`

**Lines 189–190 — `OsDetector::detect()` doc: duplicate citation**

Current:
```rust
    /// NIST SP 800-53 SA-8, CM-6, SI-7.
    /// NIST SP 800-53 SA-8 — orchestrates layered, fail-closed platform verification pipeline.
```

`SA-8` appears twice. The second line's rationale should replace the first bare listing, or the two lines should be merged.

Recommended:
```rust
    /// NIST SP 800-53 SA-8, CM-6, SI-7 — orchestrates a layered, fail-closed platform
    /// verification pipeline; hard gates abort on kernel channel compromise (SA-8).
```

---

### `kattrs/selinux.rs`

**Line 221 — inline `//` comment: non-canonical `SSDF PW 4.1` citation**

Current:
```rust
        // SSDF PW 4.1: bounds-safe access — use .get() rather than direct indexing
```

The citation is missing the framework prefix and uses a space instead of a dot separator.

Required:
```rust
        // NIST SP 800-218 SSDF PW.4.1: bounds-safe access — use .get() rather than direct indexing
```

---

### `kattrs/security.rs`

**Line 94 — `parse_lockdown_path_b` doc: non-canonical `SSDF PW 4.1` citation**

Current:
```rust
/// SSDF PW 4.1: bounds-safe slice indexing via checked length guard.
```

Required:
```rust
/// NIST SP 800-218 SSDF PW.4.1: bounds-safe slice indexing via checked length guard.
```

---

### `kattrs/traits.rs`

**Lines 41–55 — `KernelFileSource` trait: per-constant `NIST SP 800-53 AU-3` annotations**

The three associated constants (`ATTRIBUTE_NAME`, `DESCRIPTION`, `KOBJECT`) each carry an individual `/// NIST SP 800-53 AU-3: ...` annotation. The trait-level doc already establishes the AU-3 mapping for the whole type. Per the Source Comment Discipline Rule, security control citations belong at the module, struct, or major component level — not on every field or associated constant.

Current (lines 41–55):
```rust
    /// NIST SP 800-53 AU-3: Attribute Identifier
    /// The formal attribute name ...
    const ATTRIBUTE_NAME: &'static str;

    /// NIST SP 800-53 AU-3: Event Content/Description
    /// Documentation or Format string ...
    const DESCRIPTION: &'static str;

    /// NIST SP 800-53 AU-3: Audit Context
    /// Additional context ...
    const KERNEL_NOTE: &'static str = "";

    /// NIST SP 800-53 AU-3: Location/Provenance Identifier
    /// The parent kobject ...
    const KOBJECT: &'static str;
```

Recommended — drop the per-constant `NIST SP 800-53 AU-3:` label; keep the descriptive text:
```rust
    /// The formal attribute name as defined in kernel kobject/sysfs vernacular.
    const ATTRIBUTE_NAME: &'static str;

    /// Documentation or format string derived from kernel-parameters.txt or rst docs.
    const DESCRIPTION: &'static str;

    /// Additional context regarding deprecation, defaults, or kernel version specifics.
    const KERNEL_NOTE: &'static str = "";

    /// The parent kobject in the kernel hierarchy (e.g., `"selinuxfs"` or `"crypto"`).
    const KOBJECT: &'static str;
```

The `KernelFileSource` trait-level doc comment already cites `NIST SP 800-53 SI-7` and the existing `AttributeCard` doc cites `NIST SP 800-53 AU-3`. No information is lost.

---

### `posture/fips_cross.rs`

**Line 48 — module-level `//!` block: bare `NIST 800-218`**

Current:
```rust
//! - **Pattern Execution Measurement** (NIST 800-218 SSDF PW.4): debug-mode
```

Required:
```rust
//! - **Pattern Execution Measurement** (NIST SP 800-218 SSDF PW.4): debug-mode
```

---

### `posture/modprobe.rs`

**Line 46 — module-level `//!` block: bare `NIST 800-218`**

Current:
```rust
//! - **Pattern Execution Measurement** (NIST 800-218 SSDF PW.4): timing logged
```

Required:
```rust
//! - **Pattern Execution Measurement** (NIST SP 800-218 SSDF PW.4): timing logged
```

---

### `posture/catalog.rs` — `FIPS SP 800-90B` in `nist_controls` strings

**Lines 317, 328 — runtime display strings**

These use `FIPS SP 800-90B`. The document is actually an NIST publication: `NIST SP 800-90B`. The `FIPS` prefix is incorrect.

Current (line 317):
```rust
nist_controls: "NIST 800-53 SC-12; FIPS SP 800-90B entropy requirements",
```
Current (line 328):
```rust
nist_controls: "NIST 800-53 SC-12, SI-7; FIPS SP 800-90B entropy requirements",
```

However, these are runtime display strings where abbreviated forms are permitted. The `FIPS SP 800-90B` citation is factually wrong regardless: NIST SP 800-90B is an SP document, not a FIPS document. The correct abbreviated form is `NIST SP 800-90B`.

Recommended (runtime strings, abbreviated form is acceptable):
```rust
nist_controls: "NIST 800-53 SC-12; NIST SP 800-90B entropy requirements",
```
```rust
nist_controls: "NIST 800-53 SC-12, SI-7; NIST SP 800-90B entropy requirements",
```

The `rationale` field on line 315 also references `FIPS SP 800-90B` — same fix applies:
```
rationale: "Trusting CPU RNG unconditionally may not satisfy NIST SP 800-90B; \
```

---

### `posture/snapshot.rs`

**Line 711 — bare `NIST 800-218`**

Current:
```rust
/// NIST 800-218 SSDF PW.4: pattern timing in debug builds.
```

Required:
```rust
/// NIST SP 800-218 SSDF PW.4: pattern timing in debug builds.
```

---

### `sealed_cache.rs`

**Line 818 — bare `#[must_use]` without message on security-status accessor**

Current:
```rust
    #[must_use]
    pub fn status(&self) -> CacheStatus {
```

`CacheStatus` is a security-relevant type (it communicates whether the FIPS gate blocked caching or a seal failure occurred). Per the Must-Use Contract Rule, `#[must_use]` on a security-relevant return type must include a message string.

Recommended:
```rust
    #[must_use = "CacheStatus indicates FIPS gate and seal health — discarding it loses the security posture signal"]
    pub fn status(&self) -> CacheStatus {
```

---

## Files with No Findings

The following files were reviewed and contain no comment issues:

- `lib.rs`
- `confidence.rs`
- `evidence.rs`
- `os_identity.rs`
- `os_release.rs`
- `kattrs/mod.rs`
- `kattrs/procfs.rs`
- `kattrs/sysfs.rs`
- `kattrs/tpi.rs`
- `kattrs/types.rs`
- `kattrs/selinux.rs` (one finding above; remainder clean)
- `posture/mod.rs`
- `posture/signal.rs`
- `posture/reader.rs`
- `posture/configured.rs`
- `posture/contradiction.rs`
- `posture/modprobe.rs` (one finding above; remainder clean)
- `posture/fips_cross.rs` (one finding above; remainder clean)
- `detect/mod.rs` (one finding above; remainder clean)
- `detect/file_ownership.rs`
- `detect/integrity_check.rs`
- `detect/kernel_anchor.rs`
- `detect/mount_topology.rs`
- `detect/pkg_substrate.rs`
- `detect/release_candidate.rs`
- `detect/release_parse.rs`
- `detect/label_trust.rs`
- `detect/substrate/mod.rs`
- `detect/substrate/rpm.rs`
- `detect/substrate/dpkg.rs`

---

## Priority Summary

| Priority | File | Lines | Issue |
|---|---|---|---|
| High | `posture/catalog.rs` | 20–78 | Pervasive `NIST 800-` in doc comments (8 occurrences) |
| High | `posture/catalog.rs` | 315–328 | `FIPS SP 800-90B` should be `NIST SP 800-90B` (factual error) |
| Medium | `kattrs/traits.rs` | 41–55 | Per-constant AU-3 annotations (Source Comment Discipline violation) |
| Medium | `detect/mod.rs` | 189–190 | Duplicate `SA-8` citation in `detect()` doc |
| Medium | `sealed_cache.rs` | 378 | Bare `NIST 800-218` in doc comment |
| Medium | `posture/fips_cross.rs` | 48 | Bare `NIST 800-218` in module doc |
| Medium | `posture/modprobe.rs` | 46 | Bare `NIST 800-218` in module doc |
| Medium | `posture/snapshot.rs` | 711 | Bare `NIST 800-218` in function doc |
| Low | `kattrs/selinux.rs` | 221 | Non-canonical `SSDF PW 4.1` inline comment |
| Low | `kattrs/security.rs` | 94 | Non-canonical `SSDF PW 4.1` function doc |
| Low | `sealed_cache.rs` | 818 | Bare `#[must_use]` without message on `status()` |

---

## Unresolved Items

None. All findings are actionable. The `nist_controls` runtime display strings using bare `NIST 800-53` are explicitly permitted by the Citation Format Rule and do not require changes. The ANSSI citations in `os_identity.rs` and `detect/kernel_anchor.rs` use a non-standard external document reference — this is a documentation choice, not a rule violation, and has been left to the developer's discretion.
