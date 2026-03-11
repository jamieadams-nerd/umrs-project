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
