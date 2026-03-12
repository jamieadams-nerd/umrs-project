# Doc Team Shared Feedback Log

Shared between `tech-writer` and `senior-tech-writer`. Both agents read this file at the
start of any documentation session and append entries when leaving feedback for each other.

## Format

```
## [YYYY-MM-DD] [from-agent] → [to-agent]: [topic or document path]

**Status**: open | resolved

[Feedback content]
```

- Mark `resolved` when the feedback has been acted on; do not delete entries.
- Keep entries focused: one concern per entry. Open a new entry for unrelated feedback.
- Reference file paths relative to the repo root where applicable.

---

<!-- Entries below, newest first -->

---

## [2026-03-11] senior-tech-writer → senior-tech-writer: Phase 2 content restructure complete

**Status**: resolved

Seven changes executed:

1. **truth-concepts merged into reference-monitor** (`security-concepts/pages/reference-monitor.adoc`):
   New section "Ground Truth, Trust Anchors, and the Limits of Names" added before See Also.
   Covers: inodes as ground truth vs paths/labels; IMA/statfs/key-custody trust anchors;
   custody vs content trust (with cross-ref to integrity-and-provenance.adoc for full treatment);
   enforcement-by-design vs logging-only systems.
   `truth-concepts.adoc` replaced with redirect notice (file retained, not deleted).
   `security-concepts/nav.adoc` — truth-concepts entry removed; security-model.adoc entry added.
   `security-concepts/pages/index.adoc` — truth-concepts bullet removed; security-model bullet added.
   `architecture/nav.adoc` — "Case Studies" renamed "Justification and Case Studies";
   truth-concepts cross-ref removed.

2. **security-model.adoc moved** from `ROOT/pages/` to `security-concepts/pages/`.
   Broken xrefs fixed: `architecture:reference-monitor.adoc` → `reference-monitor.adoc` (local);
   `architecture:integrity-and-provenance.adoc` → `integrity-and-provenance.adoc` (local);
   `architecture:rationale.adoc` → `architecture:rationale-strongly-typed.adoc` (correct filename).
   `security-concepts/pages/index.adoc` updated with security-model entry.

3. **case-studies.adoc and mls-classified-talk.adoc moved** to `architecture/pages/history/`.
   See Also xrefs in case-studies.adoc updated to use module-qualified refs (now in subdirectory).
   xref in mls-classified-talk.adoc updated to use full module path for five-eyes-interop.
   `architecture/nav.adoc` updated with history/ paths.

4. **TW0-NETIF-JUSTIFICATION.adoc renamed** to `deployment/pages/dual-network-interface.adoc`.
   Added to `deployment/nav.adoc` under Assurance Enhancements.

5. **structured-logging.adoc and how-to-structure-log.adoc moved** from `operations/pages/`
   to `deployment/pages/`. Both added to `deployment/nav.adoc` under Assurance Enhancements.

6. **auditing-noise.adoc moved** from `operations/pages/` to `logging-audit/pages/`.
   Added to `logging-audit/nav.adoc`.

7. **operations/nav.adoc** — entire Logging section removed (all entries either moved to
   deployment or logging-audit, or were already cross-module refs now local to logging-audit).

---

## [2026-03-11] senior-tech-writer → tech-writer: Nav and antora.yml restructure complete — review xrefs in page bodies

**Status**: open

Phase 1 nav fixes are done. All nav files now point to real files. Summary of what changed:

- `docs/antora.yml` — now registers all 10 module nav files (was only ROOT)
- `docs/modules/ROOT/nav.adoc` — broken include syntax removed; top-level structure is now flat
  (modules as siblings, not nested under Architecture)
- `docs/modules/architecture/nav.adoc` — all xrefs corrected: history files use `history/` prefix,
  security-concepts files use `xref:security-concepts:filename.adoc[...]` cross-module syntax,
  openssl-no-vendoring moved to reference/cryptography/
- `docs/modules/deployment/nav.adoc` — ubuntu xref corrected to `ubuntu/ubuntu.adoc`
- `docs/modules/deployment/pages/index.adoc` — internal xrefs corrected to `rhel/` and `ubuntu/` subdirs
- `docs/modules/reference/nav.adoc` — all xrefs updated with selinux/, cryptography/, cui/ subdirs
- `docs/modules/operations/nav.adoc` — removed dead entries (umrs-tool-*, rhel10-*, git-commit-signing);
  logging entries now use cross-module xrefs to logging-audit; tools section links to umrs-tools:index.adoc
- `docs/modules/devel/nav.adoc` — umrs-prog-lang cross-references architecture module;
  git-commit-signing added; nom-parser uses correct hyphenated filename
- `docs/modules/security-concepts/nav.adoc` — created (new)
- `docs/modules/security-concepts/pages/index.adoc` — created (new, minimal index)
- `docs/modules/logging-audit/nav.adoc` — created (new)
- `docs/modules/logging-audit/pages/index.adoc` — created (new, minimal index)
- `docs/modules/reference/pages/index.adoc` — created (new, was missing)

**Action for tech-writer**: When writing or updating any page in operations/, check that body-level
xrefs to logging (logging-capacity, log-lifecycle-model, log-tuning) use the cross-module syntax
`xref:logging-audit:filename.adoc[...]`. These pages are no longer in operations/.

---

## [2026-03-11] tech-writer → senior-tech-writer: pattern-sec.adoc updated for implementation — review requested

**Status**: open

SEC is now implemented in `umrs-platform/src/sealed_cache.rs`. Updated three documents:

1. `docs/modules/patterns/pages/pattern-sec.adoc`:
   - Removed WARNING block ("required, not yet implemented")
   - Replaced with NOTE confirming implementation location and zeroize crate presence
   - Rewrote "In the UMRS Codebase" section — removed conceptual pseudocode,
     replaced with actual `SealedCache` / `SealingKey` / `SealedEntry` / `CacheStatus` types
   - Updated Summary to remove the "hard dependency on zeroize" framing (now resolved)

2. `docs/modules/patterns/pages/index.adoc`:
   - SEC row in Pattern Reference Table: ⚠ → ✓ Implemented
   - Removed SEC block from "Open — Awaiting Implementation" section
   - Removed "⚠ awaiting implementation" annotation in Patterns by Group

3. `docs/modules/devel/pages/os-detection-deep-dive.adoc`:
   - Replaced "Future: Sealed Memory Cache" section with "Sealed Evidence Cache" section
   - Covers: `SealedCache` usage, seal payload layout, sealing key derivation,
     seal failure behavior, FIPS gate behavior, conservative `decode_cached_result()`
     design note, `CacheStatus` enum, compliance table
   - Updated See Also reference from "future" to "implemented"
   - Updated SC-28 row in Security Controls Summary

Requesting review pass for:
1. Technical accuracy of the seal payload layout description against the source
2. Whether the `decode_cached_result()` design note is clear to an engineer reading
   the doc without access to the source
3. Whether the FIPS fail-closed rationale is precise enough for audit purposes

---

## [2026-03-11] tech-writer → senior-tech-writer: Pattern library migrated to patterns/ module — review requested

**Status**: open

Created 12 individual pattern pages in `docs/modules/devel/pages/`, one per pattern from
CLAUDE.md. Two patterns are documented as "required but not yet implemented":

- `pattern-zeroize.adoc` — Zeroize Sensitive Data: `vaultmgr` currently has no `zeroize` dep.
  The pattern page explains the requirement and what needs to happen before the crate handles
  real secrets.
- `pattern-constant-time.adoc` — Constant-Time Comparison: no `subtle` dep in the workspace.
  Pattern page is marked with a WARNING admonition.

Requesting a senior review pass on:
1. Whether the security rationale in each file is technically accurate
2. Whether the NIST/RTB control citations are appropriately matched to each pattern
3. Whether the Mermaid diagrams in TPI, TOCTOU, Provenance, and Constant-Time are correct
4. Whether the two "not yet implemented" patterns should be flagged somewhere in the
   architecture docs or as an open ticket

Files:
- `docs/modules/devel/pages/pattern-tpi.adoc`
- `docs/modules/devel/pages/pattern-toctou.adoc`
- `docs/modules/devel/pages/pattern-fail-closed.adoc`
- `docs/modules/devel/pages/pattern-provenance.adoc`
- `docs/modules/devel/pages/pattern-loud-failure.adoc`
- `docs/modules/devel/pages/pattern-non-bypassability.adoc`
- `docs/modules/devel/pages/pattern-secure-arithmetic.adoc`
- `docs/modules/devel/pages/pattern-zeroize.adoc`
- `docs/modules/devel/pages/pattern-constant-time.adoc`
- `docs/modules/devel/pages/pattern-error-discipline.adoc`
- `docs/modules/devel/pages/pattern-bounds-safe.adoc`
- `docs/modules/devel/pages/pattern-supply-chain.adoc`

Also updated:
- `docs/modules/devel/pages/high-assurance-patterns.adoc` — Pattern Reference table inserted
- `docs/modules/devel/nav.adoc` — four new pattern subsections added

---

---

## [2026-03-10] tech-writer → Jamie: security-model.adoc is a redirect stub — action required

**Status**: open

`docs/modules/ROOT/pages/security-model.adoc` contains only a 7-line redirect note pointing
to `architecture:index.adoc`. It has no original content of its own. Two options:

1. Remove the nav entry and quarantine the file to `docs/_scratch/`. The architecture index
   (STW-2, now complete) covers the same ground.
2. Replace it with a brief summary page that introduces the security model and links to the
   full architecture overview — useful as a stable cross-reference target.

Do not delete or modify this file without explicit instruction. Flag your choice back here.

---

## [2026-03-10] tech-writer → Jamie: i18n.md quarantine — confirm safe to delete

**Status**: open

`docs/modules/devel/pages/i18n.md` was a raw AI conversation transcript.
`docs/modules/devel/pages/i18n.adoc` is a complete, structured replacement covering all the
same material plus proper formatting.

The .md has been quarantined to `docs/_scratch/i18n.md`. Confirm that it can be permanently
deleted, or instruct otherwise.

---

## [2026-03-10] tech-writer → Jamie: rhel10-install.adoc potential duplicate — review required

**Status**: open

`docs/modules/operations/pages/rhel10-install.adoc` (copied from admin/) may duplicate
`docs/modules/deployment/pages/rhel10-installation.adoc`.

These files have not been compared in detail. Do not merge or delete either until you have
reviewed both and decided which is authoritative, whether one supersedes the other, or
whether they cover distinct scopes.

---

## [2026-03-10] tech-writer → Jamie: admin/ module originals still present — cleanup needed

**Status**: open

All pages from `docs/modules/admin/pages/` have been copied to `docs/modules/operations/pages/`
(as .adoc versions where conversion was needed). The originals in admin/pages/ were not removed
because the task required not deleting content without instruction.

When you are satisfied that operations/ has everything, the following cleanup remains:
1. Delete or quarantine the originals in `docs/modules/admin/pages/`
2. Remove the `docs/modules/admin/` module from the Antora component descriptor
   (`docs/antora.yml` or equivalent)
3. Confirm the admin/ nav.adoc is no longer referenced

Holding on this until you sign off.

---

## [2026-03-10] tech-writer → tech-writer: Task 0 resolved — rhel10-packages.adoc Post-Install section

**Status**: resolved

Added "Post-Install Packages" section to `docs/modules/deployment/pages/rhel10-packages.adoc`
before the existing "All Systems" section. Includes:
- `dnf install -y` command for: ima-evm-utils, mcstrans, jq, gnupg2, tuned, aide
- Per-package definition list (name + purpose)
- NOTE admonition for `aide --init` timing requirement

---

## [2026-03-10] tech-writer → tech-writer: TW-1 resolved — umrs-tools/ wired into main nav

**Status**: resolved

Added `* xref:umrs-tools:index.adoc[UMRS Tools]` to `docs/modules/ROOT/nav.adoc`.
Added cross-reference to `docs/modules/operations/pages/index.adoc` pointing to umrs-tools index.

---

## [2026-03-10] tech-writer → tech-writer: TW-2 resolved — admin/ merged into operations/

**Status**: resolved

All admin/ pages moved to operations/pages/:
- git-commit-signing.md → git-commit-signing.adoc (converted)
- install.adoc → admin-install.adoc (verbatim copy)
- key-management.md → key-management.adoc (converted)
- key-manager-tool.md → key-manager-tool.adoc (converted)
- rhel10-install.adoc → rhel10-install.adoc (verbatim copy)
- index.adoc → admin-index.adoc (verbatim copy)

Administration section added to operations/nav.adoc. Admin/ originals not yet removed —
see open entry above for cleanup steps.

Potential duplicate flagged: rhel10-install.adoc vs deployment/rhel10-installation.adoc —
see separate open entry.

---

## [2026-03-10] tech-writer → tech-writer: TW-3 resolved — architecture/ .md files converted

**Status**: resolved

Converted to .adoc and nav entries activated in architecture/nav.adoc:
- HACAMS.md → HACAMS.adoc
- nom_parser.md → nom_parser.adoc
- RTB.md → RTB.adoc
- reference_monitor.md → reference_monitor.adoc
- kernel-files-TPI.md → kernel-files-TPI.adoc
- TW0-NETIF-JUSTIFICATION.md → TW0-NETIF-JUSTIFICATION.adoc

All six originals quarantined to docs/_scratch/.

---

## [2026-03-10] tech-writer → tech-writer: TW-4 resolved — operations/ .txt and .md files converted

**Status**: resolved

Converted to .adoc and nav entries added to operations/nav.adoc:
- chain-intro.txt → chain-intro.adoc (Tools section)
- chain-verify-sign.txt → chain-verify-sign.adoc (Tools section)
- logging-capacity.txt → logging-capacity.adoc (Logging section)
- log-lifecycle-model.txt → log-lifecycle-model.adoc (Logging section)
- log-tuning.txt → log-tuning.adoc (Logging section)
- rhel10-openscap.txt → rhel10-openscap.adoc (Audit section)
- aide-README.md → aide-README.adoc (Integrity section)
- umrs-signing-README.md → umrs-signing-README.adoc (Tools section)

All eight originals quarantined to docs/_scratch/.

---

## [2026-03-10] tech-writer → tech-writer: TW-6 resolved — prototype crate stubs created

**Status**: resolved

Created stub pages:
- `docs/modules/umrs-tools/pages/umrs-logspace.adoc`
- `docs/modules/umrs-tools/pages/umrs-state.adoc`

Both follow the required header pattern. Nav entries added to umrs-tools/nav.adoc under
"Prototype Crates" section.

---

## [2026-03-10] tech-writer → tech-writer: TW-7 resolved — release-notes.adoc structure added

**Status**: resolved

Replaced the bare stub in `docs/modules/ROOT/pages/release-notes.adoc` with a structured
template: Current Milestone, Planned Features, Known Limitations sections with comment
placeholders. Section structure only — content left for Jamie to fill in.

---

## [2026-03-10] tech-writer → tech-writer: TW-9 resolved — devel/ .md files converted

**Status**: resolved

- i18n.md: i18n.adoc confirmed complete. i18n.md quarantined to docs/_scratch/.
  See separate open entry for Jamie confirmation.
- rust-must-use-contract.md → rust-must-use-contract.adoc (converted). Nav entry added
  to devel/nav.adoc under High-Assurance Patterns.

---

## [2026-03-10] tech-writer → tech-writer: TW-10 resolved — security-model.adoc assessed

**Status**: resolved

Read `docs/modules/ROOT/pages/security-model.adoc`. Confirmed it is a 7-line redirect stub
with no original content. Assessment and recommendation written to Jamie in separate open
entry above.

---

## [2026-03-10] senior-tech-writer → senior-tech-writer: STW-7c — trusted-path-orange.adoc reviewer corrections

**Status**: resolved

New Collaborator pass:
- Removed orphaned intro sentence with typo ("ntrusted path with the conext of")
- Defined "Trusted Computing Base (TCB)" before its first use — previously TCB appeared
  in the Trusted Path definition box before being defined anywhere
- Replaced informal opening with a factual orientation paragraph

Auditor pass:
- Added TCSEC formal name and year (1983) to the Orange Book reference
- Added NIST 800-53 SC-11 (Trusted Path) as the modern control equivalent
- Trusted path requirement at B2 was implied; now explicitly attributed to TCSEC

---

## [2026-03-10] senior-tech-writer → senior-tech-writer: STW-7b — selinux-history.adoc reviewer corrections

**Status**: resolved

New Collaborator pass:
- "security server" used without definition (second paragraph); added definition:
  a policy engine running in kernel space that evaluates access requests against
  a loaded policy without the policy being hard-coded into the kernel itself
- Added "hooks" definition for SELinux enforcement points; previously "enforcement points"
  was used without explaining what form those take

Auditor pass:
- Bell-LaPadula and Biba now explicitly named in the TCS section where MLS concepts
  are described ("clearance, dominance, trusted subjects")
- "accreditation" defined for readers unfamiliar with DoD RMF context:
  "the process by which a DoD authorizing official accepts residual risk and approves
  a system to operate at a given classification level"

---

## [2026-03-10] senior-tech-writer → senior-tech-writer: STW-7a — mls-history.adoc reviewer corrections

**Status**: resolved

New Collaborator pass:
- "Mandatory access control" (MAC) introduced immediately after DAC without a definition;
  added a definition paragraph: MAC means the system enforces access rules that neither
  the user nor the application can override
- DAC was defined ("users controlled permissions on files they owned") but MAC was not —
  fixed by adding a two-sentence MAC definition paragraph

Auditor pass:
- Bell-LaPadula confidentiality model named correctly; no gap there
- Biba integrity model implied by "confidentiality rather than integrity" but never named;
  added a paragraph naming Biba (1977, Kenneth Biba), its rules (no read down, no write up),
  and its complementary relationship to Bell-LaPadula

---

## [2026-03-10] senior-tech-writer → senior-tech-writer: STW-6 — compliance-annotations.adoc

**Status**: resolved

Created `docs/modules/devel/pages/compliance-annotations.adoc`.
Real examples used from: `umrs-selinux/src/category.rs` (CategorySet, dominates),
`umrs-selinux/src/context.rs` (SecurityContext), `umrs-platform/src/kattrs/mod.rs`
(module-level provenance verification pattern).
Nav entry added to `docs/modules/devel/nav.adoc` under High-Assurance Patterns.

---

## [2026-03-10] senior-tech-writer → senior-tech-writer: STW-5 — compliance-frameworks.adoc

**Status**: resolved

Created `docs/modules/reference/pages/compliance-frameworks.adoc`.
Covers: NIST 800-53 Rev 5, NIST 800-171 Rev 2, NIST 800-218 SSDF v1.1,
CMMC 2.0 (Level 1–3), NSA RTB VNSSA/RAIN, FIPS 140-2/140-3.
Includes table (Standard, Version, Role) and interaction narrative.
Nav entry added to `docs/modules/reference/nav.adoc` under CUI & Policy.

---

## [2026-03-10] senior-tech-writer → senior-tech-writer: STW-4 — getting-started.adoc

**Status**: resolved

Created `docs/modules/ROOT/pages/getting-started.adoc`.
Three paths: deploy, develop, audit. Items not yet written are called out
with NOTE admonitions. Does not link to non-existent pages.
Nav entry added to `docs/modules/ROOT/nav.adoc` under Start Here.

---

## [2026-03-10] senior-tech-writer → senior-tech-writer: STW-3 — five-eyes-interop.adoc

**Status**: resolved

Created `docs/modules/architecture/pages/five-eyes-interop.adoc`.
Covers US CUI baseline, UK GSC (Official/Official-Sensitive), Canada (Protected A/B/C),
Australia PSPF (OFFICIAL/OFFICIAL:Sensitive), NZ (IN-CONFIDENCE/SENSITIVE).
Current status section explicitly states CUI is the only implemented scheme.
NIST 800-171 Rev 2 (3.1.3, 3.13.1) and CMMC 2.0 AC.L2-3.1.3 cited.
Nav entry added to `docs/modules/architecture/nav.adoc` under Security Model.

---

## [2026-03-10] senior-tech-writer → senior-tech-writer: STW-2 — architecture/index.adoc

**Status**: resolved

Expanded `docs/modules/architecture/pages/index.adoc` from 7-line stub to full
architecture overview. Covers: base OS layer, SELinux context model, MLS/MCS model,
CUI policy and Five Eyes planned support, Rust implementation layer (crate dependency
chain), high-assurance patterns summary, compliance baseline table.
Links to all existing detailed pages and the new Five Eyes and compliance registry pages.

---

## [2026-03-10] senior-tech-writer → senior-tech-writer: STW-1 — introduction.adoc

**Status**: resolved

Created `docs/modules/ROOT/pages/introduction.adoc`.
Third person throughout. Covers: what UMRS is, the problem it solves, target environment,
project origin narrative, three audiences, and what is stable vs. prototype.
Word count is within the 400–600 word target.
Nav entry added to `docs/modules/ROOT/nav.adoc` under Start Here.

---

## [2026-03-10] senior-tech-writer → tech-writer: TW-1 — Wire umrs-tools/ into main nav

**Status**: open

Wire `docs/modules/umrs-tools/` into `docs/modules/ROOT/nav.adoc` so the module appears in the
published site navigation. Add cross-reference links from `docs/modules/operations/pages/index.adoc`
pointing to `docs/modules/umrs-tools/pages/index.adoc`.

No content changes required — structural nav work only.

---

## [2026-03-10] senior-tech-writer → tech-writer: TW-2 — Merge admin/ into operations/

**Status**: open

Move all pages from `docs/modules/admin/pages/` into `docs/modules/operations/pages/`.
Files to move:
- `docs/modules/admin/pages/git-commit-signing.md` (convert to .adoc during move)
- `docs/modules/admin/pages/install.adoc`
- `docs/modules/admin/pages/key-management.md` (convert to .adoc during move)
- `docs/modules/admin/pages/key-manager-tool.md` (convert to .adoc during move)
- `docs/modules/admin/pages/rhel10-install.adoc`

After moving, merge `docs/modules/admin/nav.adoc` entries into `docs/modules/operations/nav.adoc`
under a new "Administration" section heading. Remove `docs/modules/admin/` module from the
Antora component descriptor once empty. Do NOT delete any content during this process —
flag any apparent duplicates (rhel10-install.adoc vs. deployment/rhel10-installation.adoc)
and ask Jamie before resolving.

---

## [2026-03-10] senior-tech-writer → tech-writer: TW-3 — Convert pending .md and .txt files in architecture/

**Status**: open

Convert the following files from `.md` to `.adoc` format. Do not change content — format conversion
only. Add the standard AsciiDoc header (title, author line omitted, `:toc: left`) and convert
Markdown syntax (headings, code blocks, bold, links) to AsciiDoc equivalents.

Files in `docs/modules/architecture/pages/`:
- `HACAMS.md`
- `nom_parser.md`
- `RTB.md`
- `reference_monitor.md`
- `kernel-files-TPI.md`
- `TW0-NETIF-JUSTIFICATION.md`

After conversion, uncomment the nav entries in `docs/modules/architecture/nav.adoc` that reference
these pages. The original `.md` files may be quarantined to `docs/_scratch/` — do not delete them.

---

## [2026-03-10] senior-tech-writer → tech-writer: TW-4 — Convert inert .txt and .md files in operations/

**Status**: open

Convert the following files to `.adoc` format (format only, no content changes):

Files in `docs/modules/operations/pages/`:
- `chain-intro.txt`
- `chain-verify-sign.txt`
- `logging-capacity.txt`
- `log-lifecycle-model.txt`
- `log-tuning.txt`
- `rhel10-openscap.txt`
- `aide-README.md`
- `umrs-signing-README.md`

After conversion, add nav entries in `docs/modules/operations/nav.adoc` for each converted page,
grouped under appropriate section headings (Auditing/Logging, Integrity). The original files may
be quarantined to `docs/_scratch/` — do not delete them.

---

## [2026-03-10] senior-tech-writer → tech-writer: TW-5 — Reorganize operations/ nav into coherent sections

**Status**: open

Depends on: TW-2 (admin merge), TW-4 (txt/md conversions complete)

Restructure `docs/modules/operations/nav.adoc` so pages are grouped under these sections:
- Day-to-Day Operations
- Auditing and Logging
- Integrity (AIDE content)
- Administration (merged from admin/)
- Tooling (cross-reference to umrs-tools/)
- Troubleshooting

All pages must have valid nav entries before this task starts. Do not move files — nav
restructure only.

---

## [2026-03-10] senior-tech-writer → tech-writer: TW-6 — Add placeholder pages for prototype crates

**Status**: open

Create stub pages for prototype crates that are not yet stable. Each page should state the crate
name, its planned purpose, and a note that documentation will be added when the API stabilizes.
Use this header pattern:

  = [Crate Name]
  :description: Placeholder — documentation pending API stabilization.

  NOTE: This crate is under active development. Documentation will be added when the API stabilizes.

  == Planned Purpose
  [one paragraph from CLAUDE.md crate description]

Files to create:
- `docs/modules/umrs-tools/pages/umrs-logspace.adoc`
- `docs/modules/umrs-tools/pages/umrs-state.adoc`

Add nav entries for both in `docs/modules/umrs-tools/nav.adoc`.

---

## [2026-03-10] senior-tech-writer → tech-writer: TW-7 — Populate release-notes.adoc structure

**Status**: open

`docs/modules/ROOT/pages/release-notes.adoc` is a stub. Add a structured template with sections:
- Current Milestone (what is working now)
- Planned Features (what is coming)
- Known Limitations

Do not populate the content — set up the section structure so Jamie can fill it in. Keep the
milestone framing (not tied to version numbers).

---

## [2026-03-10] senior-tech-writer → tech-writer: TW-8 — Use case page: secure file security-info lookup [devel]

**Status**: open

Depends on: STW-2 (architecture overview) should exist first so this page can link to it.
Coordinate with senior-tech-writer before starting — confirm STW-2 is complete.

Create `docs/modules/devel/pages/use-case-file-security-info.adoc`.

User story: "As a developer, I want to get a file's security information — label, sensitivity,
categories, ownership — in a way that is TOCTOU-safe."

Page should cover:
1. Which crate and type to use (`umrs-selinux`, `SecureDirent`)
2. Why the inode-anchored approach is required (link to TOCTOU pattern in high-assurance-patterns.adoc)
3. A minimal code example showing `SecureDirent::from_path()` and what fields are available
4. What errors to handle and what they mean
5. Compliance note (NIST AC-3, SI-7)

The code example should come from the existing codebase examples — do not invent API signatures.
Check `components/rusty-gadgets/umrs-selinux/examples/` for source material.

Add a nav entry in `docs/modules/devel/nav.adoc` under a "Use Cases" section.

---

## [2026-03-10] senior-tech-writer → tech-writer: TW-9 — Convert devel/ .md files

**Status**: open

Convert the following files in `docs/modules/devel/pages/` to `.adoc`:
- `i18n.md` (duplicate of `i18n.adoc` — check for content differences before converting;
  if the .adoc is complete, quarantine the .md to `docs/_scratch/` and ask Jamie)
- `rust-must-use-contract.md` — convert to `.adoc`, add nav entry in `docs/modules/devel/nav.adoc`

---

## [2026-03-10] senior-tech-writer → tech-writer: TW-10 — Assess ROOT/security-model.adoc redirect stub

**Status**: open

Depends on: STW-2 (architecture overview) complete.

Read `docs/modules/ROOT/pages/security-model.adoc`. If it is purely a redirect stub with no
original content, flag it to Jamie with a recommendation to either remove it or replace it with
a genuine summary that links to the architecture overview. Do not delete it without explicit
instruction.

---

## [2026-03-10] senior-tech-writer → senior-tech-writer: STW backlog summary

**Status**: open

For senior-tech-writer reference — full backlog in MEMORY.md.

Sequenced deliverables:
1. STW-1: `docs/modules/ROOT/pages/introduction.adoc` — new page, what UMRS is (third person)
2. STW-2: `docs/modules/architecture/pages/index.adoc` — expand stub into architecture overview
3. STW-3: `docs/modules/architecture/pages/five-eyes-interop.adoc` — new page, multi-national CUI interop
4. STW-4: `docs/modules/ROOT/pages/getting-started.adoc` — new page, onboarding path by audience
5. STW-5: `docs/modules/reference/pages/compliance-frameworks.adoc` — new page, framework registry
6. STW-6: `docs/modules/devel/pages/compliance-annotations.adoc` — how to write annotated rustdoc
7. STW-7: Reviewer pass on mls-history.adoc, selinux-history.adoc, trusted-path-orange.adoc

STW-1 and STW-2 are the highest-priority unlocking items. STW-3 and STW-4 depend on STW-2.
STW-6 depends on STW-5. STW-7 can run in parallel with anything.

---

## [2026-03-11] senior-tech-writer → Jamie: selinux-registry.txt is a duplicate — confirm safe to delete

**Status**: open

`docs/modules/reference/pages/selinux-registry.txt` and
`docs/modules/reference/pages/umrs-mls-registry.txt` are byte-for-byte identical.
Both contain the UMRS filesystem layout and registry location discussion.

`umrs-mls-registry.txt` is the correctly named version. It has been converted to
`umrs-mls-registry.adoc`.

`selinux-registry.txt` should be deleted — its name is misleading (it has nothing to do
with a SELinux registry), and it duplicates the converted content. Confirm before
deletion.

---

## [2026-03-11] senior-tech-writer → senior-tech-writer: Phase 1 and Phase 2 reorganization complete

**Status**: resolved

Phase 1 (structural moves):

- S-1: `security/` module retired. All 5 pages moved to `architecture/` or `devel/`.
  `security/nom_parser.adoc` → `devel/pages/nom-parser.adoc`.
  `architecture/nav.adoc` and `devel/nav.adoc` updated.
  `security/` directory removed.

- S-2: `historical/` module retired. All 11 pages copied to `architecture/pages/`.
  `historical/` directory removed. `antora.yml` required no changes (neither module
  was registered; `architecture` was already listed).

- S-3: All 9 non-.adoc files in `reference/pages/` converted to `.adoc`.
  Typo in `cui-descriiptions.txt` corrected to `cui-descriptions.adoc`.
  `selinux-registry.txt` confirmed as duplicate of `umrs-mls-registry.txt` — flagged
  to Jamie separately. `reference/nav.adoc` updated with all new pages under
  "MLS Display", "CUI & Policy", and "Cryptography" sections.

Approved deletions executed:
- `ROOT/pages/security-model.adoc` — deleted (confirmed redirect stub, no content)
- `docs/_scratch/pdf-security-model.adoc` — deleted (approved)

Phase 2 (new architecture content pages):

- A-1: `architecture/pages/mls-label-model.adoc` — MLS level set (s0–s15),
  CUI vs classification vs proprietary, dominance ordering, SELinux mapping,
  NIST control alignment. Synthesized from umrs-levels-cui.txt and umrs-concepts.txt.

- A-2: `architecture/pages/integrity-and-provenance.adoc` — Defines provenance,
  integrity, integrity assurance, attestation, authenticity, non-repudiation, chain
  of custody, custody vs content trust, auditability. Covers IMA/EVM enforcement
  architecture. Synthesized from terminology.txt and RATIONALE_for_HA.adoc.

- A-3: `architecture/pages/case-studies.adoc` — High-assurance failure cases by CUI
  category: OPM, Equifax, Twitter misuse, Flint water, water report falsification,
  rail inspection falsification, Boeing 737 MAX, pipeline fraud, Mars Orbiter,
  Deepwater Horizon, Vayner, Lorraine v. Markel. All presented as design lessons with
  UMRS control pattern mapping. IMPORTANT note added on statistical uncertainty.

- A-4: `architecture/pages/cui-structure.adoc` — CUI definition (EO 13556), CUI vs
  classification, CUI vs proprietary, CUI vs ITAR/EAR, NIST 800-171 mapping to UMRS.

- A-5: `architecture/pages/truth-concepts.adoc` — Placeholder stub.

All new pages added to `architecture/nav.adoc`.
