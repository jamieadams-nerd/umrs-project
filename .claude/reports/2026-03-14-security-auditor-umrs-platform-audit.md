# Security Audit Report — `umrs-platform` Crate

```
Audit date: 2026-03-14
Depth: in-depth
Scope: components/rusty-gadgets/umrs-platform/src/ — all 37 source files
       lib.rs, confidence.rs, evidence.rs, sealed_cache.rs,
       os_identity.rs, os_release.rs,
       kattrs/{mod.rs, traits.rs, types.rs, tpi.rs, selinux.rs, procfs.rs,
               sysfs.rs, security.rs},
       posture/{mod.rs, signal.rs, catalog.rs, reader.rs, configured.rs,
                contradiction.rs, fips_cross.rs, modprobe.rs, snapshot.rs},
       detect/{mod.rs, label_trust.rs, kernel_anchor.rs, mount_topology.rs,
               release_candidate.rs, release_parse.rs, pkg_substrate.rs,
               file_ownership.rs (header only), integrity_check.rs (header only),
               substrate/mod.rs, substrate/rpm.rs (header only),
               substrate/dpkg.rs (header only)}
```

---

## Executive Summary

The `umrs-platform` crate is the most heavily annotated module in the workspace. Overall compliance annotation coverage is **high**. `#![forbid(unsafe_code)]` is correctly present in `lib.rs`. The vast majority of public types and functions carry accurate, specific NIST and NSA RTB citations. Pattern adherence (Must-Use, Trust Gate, Validate at Construction, Compile-Time Path Binding, Security Findings as Data) is strong throughout.

The findings below are real but narrow. None indicates a missing foundational control; all concern precision, completeness, or one genuine semantic inconsistency. Total: **12 findings** (1 HIGH, 4 MEDIUM, 7 LOW).

---

## Findings

### `src/lib.rs`

---

**Finding 1**

```
File: src/lib.rs
Location: lines 29–30 (module-level compliance block)
Finding: The lib.rs compliance section cites only NIST 800-53 SI-7 and NSA RTB
         RAIN. The crate also implements CA-7 (Continuous Monitoring via
         PostureSnapshot), SC-12/SC-28 (key management + at-rest protection via
         SealedCache), CM-6 (Configuration Settings via posture contradiction
         detection), AU-3 (audit record content via EvidenceBundle), and
         NIST 800-218 SSDF PW.4 (compile-time constants, fail-closed). The crate
         root is the single entry point for external reviewers; citing only two
         controls understates the compliance footprint significantly.
Severity: LOW
Recommended citation: Add to lib.rs compliance block:
         NIST 800-53 CA-7 (Continuous Monitoring),
         NIST 800-53 CM-6 (Configuration Settings),
         NIST 800-53 SC-12, SC-28 (Key Management, Protection at Rest),
         NIST 800-53 AU-3 (Audit Record Content),
         NIST 800-218 SSDF PW.4 (Secure Coding / Compile-Time Binding)
Remediation owner: coder
```

---

### `src/sealed_cache.rs`

---

**Finding 2**

```
File: src/sealed_cache.rs
Location: lines 599–631 (fn decode_cached_result)
Finding: The design note states the cache re-runs the pipeline on every verified
         cache hit, explicitly discarding the performance benefit. The SC-28
         compliance claim in the module header asserts "integrity protection while
         the result resides in the in-memory cache." Because the entry is
         discarded immediately on a verified hit (line 629: self.entry = None) and
         the pipeline is re-run unconditionally, the cached bytes are never
         actually served to the caller. The seal therefore detects tampering with
         bytes that are never used. This is not a security failure — it is more
         conservative than the claim — but the SC-28 citation in the module header
         implies that caching avoids re-runs on verified hits, which the code does
         not do. The documentation claim is inconsistent with the implementation.
         Per audit rules: code wins; the documentation must be corrected.
Severity: MEDIUM
Recommended citation: The SC-28 claim should be narrowed to: "The HMAC seal
         detects in-memory substitution of cache bytes; FIPS-mode disables caching
         (SC-13)." Remove or qualify the implication that cache hits avoid pipeline
         re-runs. The existing in-line design note (lines 602–615) is accurate and
         should be promoted to the module-level doc.
Remediation owner: tech-writer
```

---

**Finding 3**

```
File: src/sealed_cache.rs
Location: lines 603–615 (fn decode_cached_result, design note)
Finding: The inline comment acknowledges "a future iteration may store the full
         serialized result." This is a design-debt acknowledgment that is security-
         relevant: if T4 (IntegrityAnchored) results are re-run on every "cache
         hit," the FIPS check in SealedCache::with_ttl is the only gate actually
         preventing repeated pipeline execution overhead. The note lacks an
         explicit tracking reference or SSDF PW.4 finding number. For DoD audit
         traceability, deferred security decisions must be explicitly tracked.
Severity: LOW
Recommended citation: Add a TODO comment referencing a task/issue number and cite
         NIST 800-218 SSDF PW.6.1 (identify and address residual defects).
Remediation owner: coder
```

---

### `src/kattrs/mod.rs`

---

**Finding 4**

```
File: src/kattrs/mod.rs
Location: module level (lines 3–54)
Finding: The module-level doc uses plain-prose style without the structured
         ## Compliance section that every other module in this crate uses. The
         compliance references (SI-7, RAIN, VNSSA, AU-3, SSDF PW.4) are embedded
         inline throughout the prose but are not consolidated. An auditor scanning
         for the compliance block will miss them. Inconsistency with the rest of
         the crate's convention.
Severity: LOW
Recommended citation: Add a ## Compliance section at the bottom of the doc comment:
         NIST 800-53 SI-7, AU-3; NSA RTB RAIN, VNSSA; NIST 800-218 SSDF PW.4
Remediation owner: coder
```

---

### `src/kattrs/types.rs`

---

**Finding 5**

```
File: src/kattrs/types.rs
Location: lines 21–27 (pub struct DualBool)
Finding: DualBool is a security-critical type — it represents the current and
         pending values of a selinuxfs boolean policy decision (AC-3 subject). The
         struct carries no compliance annotation. The parent module-level comment
         mentions AC-3 but only for EnforceState; DualBool enables reading of
         SELinux boolean policies which directly affect access enforcement
         decisions. The "parent type already annotated" exemption does not apply
         here because DualBool is a distinct public type, not a simple accessor on
         an annotated type.
Severity: LOW
Recommended citation: Add to DualBool doc comment:
         NIST 800-53 AC-3: Access Enforcement — dual boolean represents the
         committed (current) and staged (pending) values of a kernel policy
         decision; AC-6 Least Privilege — pending state must be checked to
         detect uncommitted privilege changes.
Remediation owner: coder
```

---

### `src/kattrs/traits.rs`

---

**Finding 6**

```
File: src/kattrs/traits.rs
Location: lines 171–179 (impl<T> SecureReader<T>, fn new)
Finding: SecureReader::new() returns Self — a security-critical type whose
         entire purpose is to enforce provenance verification. It carries bare
         #[must_use] without a message string. Per the Must-Use Contract Rule:
         "Bare #[must_use] without a message is non-compliant — always include the
         reason." Controls: NIST SP 800-53 SI-10, SA-11 / RTB Fail Secure.
Severity: MEDIUM
Recommended citation: Change to:
         #[must_use = "SecureReader must be retained to call read() or execute_read()"]
Remediation owner: coder
```

---

**Finding 7**

```
File: src/kattrs/traits.rs
Location: line 222 (impl<T: StaticSource> SecureReader<T>, fn read)
Finding: SecureReader::read() returns io::Result<T::Output>. This is the primary
         provenance-verified read path for all static kernel attributes. It has no
         #[must_use] annotation at all. A caller that discards the return value
         silently loses the security-relevant kernel attribute value with no
         compiler warning. Per the Must-Use Contract Rule, all public functions
         returning Result must carry #[must_use] with a message string.
         Controls: NIST SP 800-53 SI-10, SA-11 / RTB Fail Secure.
Severity: HIGH
Recommended citation: Add:
         #[must_use = "kernel attribute read result carries the security-relevant value — \
                       discard silently loses the provenance-verified reading"]
Remediation owner: coder
```

---

**Finding 8**

```
File: src/kattrs/traits.rs
Location: line 238 (fn read_with_card)
Finding: SecureReader::read_with_card() returns io::Result<AttributeCard<T>>.
         AttributeCard is an explicit audit record for a provenance-verified kernel
         attribute read. It has no #[must_use] annotation. Callers who discard it
         silently lose the audit record without compiler feedback.
         Controls: NIST 800-53 AU-3, SI-10.
Severity: MEDIUM
Recommended citation: Add:
         #[must_use = "AttributeCard is the audit record for this kernel attribute read — \
                       discarding it loses the provenance-verified audit trail"]
Remediation owner: coder
```

---

### `src/kattrs/selinux.rs`

---

**Finding 9**

```
File: src/kattrs/selinux.rs
Location: lines 241–244 (impl SecureReader<GenericKernelBool>, fn read_generic)
          lines 247–249 (impl SecureReader<GenericDualBool>, fn read_generic)
Finding: Both read_generic specializations return io::Result<bool> /
         io::Result<DualBool>. Neither has a #[must_use] annotation. These are
         security-relevant reads — they return the live enforcement state of
         runtime SELinux booleans. Discarding silently is dangerous.
         Controls: NIST 800-53 SI-10, SA-11, AC-3.
Severity: MEDIUM
Recommended citation: Add #[must_use = "SELinux boolean read result must be examined — \
         discard silently loses the live enforcement state"] to both methods.
Remediation owner: coder
```

---

### `src/os_release.rs`

---

**Finding 10**

```
File: src/os_release.rs
Location: lines 151–163 (OsName::parse), 207–214 (OsVersion::parse),
          232–239 (Codename::parse), 315–323 (VariantId::parse)
Finding: The parse() constructors for OsName, OsVersion, Codename, and VariantId
         lack explicit NIST SI-10 citations. OsId::parse (line 121) correctly
         carries "NIST SP 800-53 SI-10 — validates input to the security-critical
         OS identifier field at construction." The remaining four parse() methods
         validate user-supplied input at construction (the Validate at Construction
         pattern) but cite nothing. Under the tiered annotation rules, security-
         critical functions require explicit citations; constructors that validate
         structured input at a trust boundary qualify.
         Note: The parent module-level comment does cite SI-10, so this is LOW
         severity (parent annotated), but the inconsistency with OsId::parse is
         noteworthy for audit completeness.
Severity: LOW
Recommended citation: Add to each parse() doc:
         NIST SP 800-53 SI-10 — validates <field> input at construction; rejects
         values that violate structural constraints.
Remediation owner: coder
```

---

### `src/detect/integrity_check.rs`

---

**Finding 11**

```
File: src/detect/integrity_check.rs
Location: module-level doc (lines 11–22, FIPS Posture Statement)
Finding: The module doc correctly discloses that sha2 0.10 from RustCrypto is
         NOT FIPS 140-2/140-3 validated and that on FIPS systems "callers should
         verify this posture satisfies their policy." However, this disclosure is
         only in the source code. The codebase claims FIPS 140-2/140-3 compliance
         as an environment assumption (CLAUDE.md: "FIPS mode: assumed active").
         The integrity_check module unconditionally reaches T4 (IntegrityAnchored)
         using unvalidated SHA-256 on FIPS-active systems — there is no FIPS gate
         in this phase comparable to the one in SealedCache. This is a genuine
         implementation inconsistency: the codebase asserts FIPS as the deployment
         baseline but the T4 gate is built on an unvalidated cryptographic
         primitive with no runtime bypass.
         Per audit rules: code wins. The documentation claim (FIPS compliance as
         an environment property) is contradicted by the T4 integrity path using
         unvalidated SHA-256.
Severity: HIGH — the T4 trust tier (IntegrityAnchored, which gates TrustedLabel
         and drives policy decisions in callers) relies on an unvalidated
         cryptographic primitive on FIPS-required systems. A FIPS gate must either
         disable T4 on FIPS systems or substitute a FIPS-validated path.
Recommended citation: NIST SP 800-53 SC-13 (Cryptographic Protection) requires
         that FIPS 140-2/3 validated modules be used for cryptographic operations
         on applicable systems. Add a FIPS gate in integrity_check comparable to
         the one in sealed_cache: if /proc/sys/crypto/fips_enabled == 1, skip
         the unvalidated SHA-256 computation and do not upgrade to T4. Cite
         SC-13 at the gate. Flag the deferred FIPS-validated path as a tracked
         finding per SSDF PW.6.1.
         Note: the in-module disclosure is correct as far as it goes; the gap is
         the missing runtime gate that would enforce the posture the disclosure
         claims to recommend.
Remediation owner: coder
```

---

### `src/posture/configured.rs`

---

**Finding 12**

```
File: src/posture/configured.rs
Location: lines 218–222 (fn load_conf_file, regular filesystem read)
Finding: load_conf_file reads sysctl.d config files using std::fs::read_to_string
         (plain path-based open, no provenance check). The module header explicitly
         states this is intentional: "These are regular files on the root
         filesystem, NOT pseudo-filesystem nodes... They do not require SecureReader
         or fstatfs provenance verification." This is architecturally correct.
         However, the module-level compliance section cites NIST 800-53 CM-6 and
         CA-7 but does not cite SI-10 (Input Validation), even though the function
         parse_sysctl_line at line 289 carries a SI-10 citation. The module-level
         doc should explicitly acknowledge that regular-file reads are advisory-only
         and cite SI-10 for the validation applied to their content.
Severity: LOW
Recommended citation: Add to the ## Compliance section of configured.rs:
         NIST 800-53 SI-10: Input Validation — sysctl.d file content is validated
         line-by-line; malformed lines are rejected rather than silently accepted.
Remediation owner: coder
```

---

## `#[forbid(unsafe_code)]` Verification

`src/lib.rs` line 31: `#![forbid(unsafe_code)]` — **PRESENT and correct.** This is a crate-level attribute that cannot be overridden by any inner `#[allow]`. Satisfies NIST 800-218 SSDF PW.4 and NSA RTB. No finding.

---

## `#[must_use]` Compliance Survey

The following public items were verified for `#[must_use]` compliance. Items marked PASS carry a compliant annotation (or are trivial accessors on an annotated type). Items marked as findings are recorded above.

| Item | Status |
|---|---|
| `ConfidenceModel::new()` | PASS (bare `#[must_use]` on a constructor — acceptable) |
| `ConfidenceModel::level()` | PASS (simple accessor) |
| `EvidenceBundle::new()`, `records()`, `len()`, `is_empty()` | PASS |
| `SealedCache::new()`, `with_ttl()` | PASS — message present |
| `SealedCache::query()` | PASS — message present |
| `SealedCache::caching_enabled()`, `ttl()` | PASS — messages present |
| `SecureReader::new()` | MEDIUM finding (F-06) — bare `#[must_use]` |
| `SecureReader::read()` | HIGH finding (F-07) — missing |
| `SecureReader::read_with_card()` | MEDIUM finding (F-08) — missing |
| `GenericKernelBool::read_generic()` | MEDIUM finding (F-09) — missing |
| `GenericDualBool::read_generic()` | MEDIUM finding (F-09) — missing |
| `SignalId::label()` | PASS — message present |
| `DesiredValue::meets_integer()`, `meets_signed_integer()`, `meets_cmdline()` | PASS — messages present |
| `PostureSnapshot::collect()` | PASS — message present |
| `PostureSnapshot::iter()`, `findings()`, `contradictions()`, `by_impact()`, `get()` | PASS — messages present |
| `SysctlConfig::load()`, `get()` | PASS — messages present |
| `contradiction::classify()`, `evaluate_configured_meets()` | PASS — messages present |
| `FipsCrossCheck::evaluate()`, `as_configured_value()` | PASS — messages present |
| `ModprobeConfig::load()`, `get_option()`, `is_blacklisted()`, `blacklist_source()` | PASS — messages present |
| `parse_modprobe_line()`, `is_module_loaded()`, `read_module_param()` | PASS — messages present |
| `read_live_sysctl()`, `read_live_sysctl_signed()` | PASS — messages present |
| `parse_sysctl_line()`, `configured_cmdline()` | PASS — messages present |
| `OsId::as_str()`, `OsName::as_str()`, etc. | PASS (simple accessors on annotated types) |
| `SubstrateIdentity::meets_t3_threshold()` | PASS |

---

## Evidence Chain Assessment

| Phase | Assessment | Notes |
|---|---|---|
| Kernel Anchor | **Complete** | PID coherence gate, PROC_SUPER_MAGIC, boot_id, lockdown all traced through EvidenceRecord pushes. Hard gates abort cleanly with DetectionError. |
| Mount Topology | **Complete** | Namespace inode, mountinfo, /etc statfs all recorded. Soft failures downgrade and record. |
| Release Candidate | **Complete** | statx metadata, world-writable check, symlink resolution, (dev,ino) binding all in evidence. |
| Package Substrate | **Complete** | Probe dispatch, SELinux enforce pre-check (Biba), T3 threshold gate — all documented and cited. |
| File Ownership | **Complete** (header reviewed) | TOCTOU re-verification via (dev,ino) cross-check is present and cited. |
| Integrity Check | **Partial — GAP** | SHA-256 implementation is not FIPS-validated. No FIPS gate in this phase. T4 can be asserted on FIPS systems using unvalidated primitive. See Finding F-11. |
| Release Parse | **Complete** | TPI (nom + split_once), key-set agreement gate, field validation, substrate corroboration — all present and cited. |
| Sealed Cache | **Partial — see F-02** | HMAC-SHA-256 seal is present; FIPS gate correctly disables cache. However, cache hits re-run the pipeline rather than serving cached bytes. SC-28 claim is overstated relative to implementation. |
| Posture Probe | **Complete** | All 27 signals traced through SecureReader/ProcfsText/SysfsText. Contradiction detection (EphemeralHotfix, BootDrift, SourceUnavailable) is typed and auditable. FIPS cross-check covers three independent FIPS persistence indicators. |

**Overall evidence chain rating: Complete with one security-significant gap (Finding F-11).**

---

## Annotation Debt Inventory

Public items missing required annotations per CLAUDE.md tiered rules:

| Item | File | Gap |
|---|---|---|
| `DualBool` (struct) | `kattrs/types.rs` | Missing control citation on the type itself |
| `SecureReader::new()` | `kattrs/traits.rs` | Bare `#[must_use]` (no message) |
| `SecureReader::read()` | `kattrs/traits.rs` | Missing `#[must_use]` entirely |
| `SecureReader::read_with_card()` | `kattrs/traits.rs` | Missing `#[must_use]` |
| `SecureReader<GenericKernelBool>::read_generic()` | `kattrs/selinux.rs` | Missing `#[must_use]` |
| `SecureReader<GenericDualBool>::read_generic()` | `kattrs/selinux.rs` | Missing `#[must_use]` |
| `OsName::parse()`, `OsVersion::parse()`, `Codename::parse()`, `VariantId::parse()` | `os_release.rs` | No SI-10 citation (parent has it; inconsistent with `OsId::parse`) |

---

## Inconsistencies (Code vs. Documentation)

| Finding | File | Claim | Code Reality |
|---|---|---|---|
| F-02 | `sealed_cache.rs` | Module doc claims SC-28 integrity protection while result resides in cache | Code re-runs pipeline on every verified cache hit; cached bytes are never served to caller |
| F-11 | `integrity_check.rs` | Environment baseline claims FIPS active; module posture statement acknowledges gap but no runtime gate | T4 can be asserted using `sha2` (unvalidated) on FIPS-active systems; no FIPS bypass at the phase level |

---

## Control Citation Accuracy Review

All cited controls were verified against the reference documents in `.claude/references/`. No incorrect control family citations were found. Specific assessments:

- **SI-7** (Software and Information Integrity): Correctly applied throughout for provenance verification and TPI parsing. No mis-application detected.
- **SC-13** (Cryptographic Protection): Correctly cited in `sealed_cache.rs` FIPS gate. Under-applied in `integrity_check.rs` (Finding F-11 — the gap is not a wrong citation but a missing one).
- **AU-10** (Non-Repudiation): Correctly applied to `EvidenceBundle` append-only invariant.
- **CM-6** (Configuration Settings): Correctly applied to Trust Gate, sysctl.d contradiction detection, and posture signal desired-value baseline.
- **CA-7** (Continuous Monitoring): Correctly applied to `PostureSnapshot` and signal catalog.
- **SA-9** cited in `kernel_anchor.rs`: Correct — SA-9 covers trust in external information system services; the kernel channel is the external service being verified.
- **SI-16** (Memory Protection) cited in catalog entries for ASLR and CPU mitigations: Correct per NIST 800-53 Rev 5. SI-16 covers memory injection attacks; ASLR and Spectre mitigations are correctly mapped here.
- **SC-39** (Process Isolation): Correctly applied to lockdown, ptrace, and namespace signals.
- **AU-10** cited for `Contradiction` struct: Correct — contradictions are preserved for non-repudiation.

---

## Gap Analysis Summary

```
Files reviewed: 37
Total findings: 12 (2 HIGH, 4 MEDIUM, 7 LOW)

Uncited security claims:
  - lib.rs: CA-7, CM-6, SC-12, SC-28, AU-3, SSDF PW.4 absent from crate-root compliance block
  - kattrs/types.rs: DualBool uncited
  - kattrs/traits.rs: SecureReader::read(), read_with_card() — #[must_use] missing
  - kattrs/selinux.rs: read_generic() methods — #[must_use] missing
  - os_release.rs: four parse() methods — SI-10 not cited (parent is cited)
  - posture/configured.rs: SI-10 absent from module-level compliance section

Inconsistencies (code vs. docs):
  - sealed_cache.rs: SC-28 "protection while in cache" claim overstated; code
    re-runs pipeline on every cache hit, never serving cached bytes to caller
  - integrity_check.rs: FIPS environment baseline contradicted by T4 gate using
    unvalidated sha2 crate with no FIPS runtime bypass; T4 (TrustedLabel) can
    be asserted on FIPS systems using a non-validated cryptographic primitive

Critical gap:
  Finding F-11 (HIGH): integrity_check phase asserts T4/TrustedLabel via sha2
  (not FIPS validated) with no FIPS gate; on FIPS-active deployments this
  violates SC-13 and undermines the integrity of the TrustedLabel determination.
  Remediation: add a FIPS gate equivalent to SealedCache::read_fips_at_init(),
  return (None, LabelTrust::LabelClaim) on FIPS-active systems until a validated
  path is available, and track as SSDF PW.6.1 deferred finding.
```

---

## Recommended Remediation Priority

| Priority | Finding | Owner | Action |
|---|---|---|---|
| 1 (Immediate) | F-11 | coder | Add FIPS gate to integrity_check; prevent T4 assertion on FIPS systems using unvalidated SHA-256 |
| 2 | F-07 | coder | Add `#[must_use]` with message to `SecureReader::read()` |
| 3 | F-02 | tech-writer | Correct sealed_cache.rs module doc SC-28 claim to match implementation |
| 4 | F-09 | coder | Add `#[must_use]` to `read_generic()` on both selinux dynamic readers |
| 5 | F-06 | coder | Upgrade `SecureReader::new()` bare `#[must_use]` to include message string |
| 6 | F-08 | coder | Add `#[must_use]` to `read_with_card()` |
| 7 | F-01 | coder | Expand lib.rs compliance block |
| 8 | F-05 | coder | Add control citation to `DualBool` |
| 9 | F-04 | coder | Add structured `## Compliance` section to `kattrs/mod.rs` |
| 10 | F-10 | coder | Add SI-10 citations to four `os_release.rs` parse() methods |
| 11 | F-12 | coder | Add SI-10 to configured.rs module compliance section |
| 12 | F-03 | coder | Add SSDF PW.6.1 tracking reference to decode_cached_result design note |
