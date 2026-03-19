# Blog Post Accuracy Audit — `blog-cui-sign-lock.adoc`

Audit date: 2026-03-19
Depth: surface
Scope: `docs/sage/blogs/blog-cui-sign-lock.adoc`, `components/platforms/rhel10/CUI-LABELS.json`,
       `components/rusty-gadgets/` crate roots, `refs/reports/umrs-capabilities-800-171r3-mapping.md`,
       `refs/manifest.md`

---

## Summary of Findings

| Category | Count |
|---|---|
| ACCURATE | 14 |
| CONCERN | 6 |
| ERROR | 1 |

---

## ACCURATE

### A-1: EO 13556 — CUI program establishment date
Accurate. EO 13556 signed November 4, 2010, Federal Register 75 FR 68675.

### A-2: NIST SP 800-171 — 110 controls
Accurate for both Rev 2 and Rev 3.

### A-3: DFARS 252.204-7012 and SPRS
Accurate. Self-attestation characterization is historically and technically correct.

### A-4: Bell-LaPadula description
Accurate as a public-audience summary. Simple Security Property and Star Property correctly stated.

### A-5: CMMC 2.0 Final Rule effective date
Accurate. 32 CFR Part 170 effective December 16, 2024.

### A-6: Memory-safe languages as national priority
Accurate. NSA/CISA 2023 Cybersecurity Information Sheet confirmed.

### A-7: CUI catalog statistics — 72 entries, 23 groups, 48 subcategories
Verified against `CUI-LABELS.json`. Arithmetic correct: 1 + 23 + 48 = 72.

### A-8: Kernel vs. config file — SELinux enforce path
Accurate. `/sys/fs/selinux/enforce` is the authoritative runtime indicator.

### A-9: TPI dual-parser architecture
Verified. `umrs-selinux/src/context.rs` confirms both paths and fail-closed behavior.

### A-10: `#![forbid(unsafe_code)]` mechanical proof claim
Accurate for crates that carry the directive. See E-1 for scope qualification.

### A-11: Phase 1 / Phase 2 distinction
Clearly stated. Phase 2 items explicitly labeled in-progress. The statement "I will not tell you Phase 2 is done when it is not" is the post's strongest credibility signal.

### A-12: RHEL 4 first commercial SELinux — 2005
Accurate. RHEL 4 shipped with SELinux enabled-by-default in February 2005.

### A-13: Ada range constraint analogy
Technically coherent editorial/rhetorical comparison. No finding.

### A-14: References section completeness
All 10 in-text citations have corresponding reference entries. No orphaned citations.

---

## CONCERN

### C-1: DFARS acquisition rule — "clearing August 2025"
**Requires pre-publication verification.** The exact date and final form of the DFARS CMMC rule should be verified against the Federal Register. If the rule slipped or was modified, the claim will be wrong.
**Recommendation:** Add qualifying language or verify actual effective date.

### C-2: FAR CUI rule — "in 2025"
**Requires pre-publication verification.** FAR Case 2017-016 may still be in proposed rulemaking. Describing it as accomplished may be premature.
**Recommendation:** Qualify with "anticipated" or "proposed" and verify status.

### C-3: France / Five Eyes categorization
**Categorization error.** France is not a Five Eyes partner. Five Eyes = US, UK, Canada, Australia, New Zealand. France's SGDSN IGI 1300 should be listed separately or the label changed to "allied nation frameworks."
**Recommendation:** Change to "partner nations and allied frameworks" or split France out.

### C-4: `#![forbid(unsafe_code)]` scope — absorbed into E-1

### C-5: LEI/INV regulatory citation — 28 CFR Part 23
**Plausible but imprecise.** 28 CFR Part 23 governs criminal intelligence system data specifically, not all LEI/INV CUI. Primary authority is 5 U.S.C. 552(b)(7).
**Recommendation:** Rephrase or drop the specific citation for a blog post context.

### C-6: "UMRS supports loadable country label profiles" — Phase status unclear
The capability is described in present tense but not listed in the Phase 1 capability section. If Phase 2, it should be labeled accordingly.
**Recommendation:** Verify Phase 1 or Phase 2 and label accordingly.

---

## ERROR

### E-1: "#![forbid(unsafe_code)] in every crate root" — literal falsity

**Claim (line 134):** "The codebase uses `#![forbid(unsafe_code)]` in every crate root."

`umrs-hw/src/lib.rs` line 37 explicitly states: "NOTE: #![forbid(unsafe_code)] is intentionally ABSENT from this crate root. This is the workspace's designated unsafe isolation boundary."

The blog describes an auditor verification procedure ("An auditor opens one file per crate") that, when followed, immediately demonstrates the claim is false.

The actual security posture — one bounded unsafe block for RDTSCP timing, confined and documented — is strong. The problem is the blog makes a stronger and false universal claim.

**Severity:** HIGH — must fix before publication.

**Recommended replacement:**

> Every crate root that touches security-relevant data carries `#![forbid(unsafe_code)]`.
> The workspace contains one designated unsafe isolation boundary: `umrs-hw`, which wraps
> a single RDTSCP inline assembly instruction for hardware timestamps. That block is confined,
> documented, and separated from all security decision paths.

---

## Remediation Owner Summary

| Finding | Priority |
|---|---|
| E-1: `#![forbid]` scope — false universal claim | MUST FIX before publish |
| C-3: France / Five Eyes categorization | MUST FIX before publish |
| C-1: DFARS August 2025 date verification | Verify or qualify |
| C-2: FAR CUI 2025 rule verification | Verify or qualify |
| C-6: Country label profiles Phase status | Verify and label |
| C-5: 28 CFR Part 23 precision | Low — acceptable as-is |

---

## Strengths Worth Preserving

- Phase 1 / Phase 2 distinction is handled with exemplary honesty
- CategorySet set-theory explanation is accurate and accessible
- Targeted policy escape hatch candor is appropriately transparent
- All citations have corresponding references
