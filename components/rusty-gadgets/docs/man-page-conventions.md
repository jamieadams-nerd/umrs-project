# UMRS Man Page Conventions

Source location, build pipeline, section layout, and verification procedure
for all UMRS tool man pages.

---

## Source Location

Man page sources are **per-crate**, not centralized.

Each binary crate owns its man page sources under a `man/` subdirectory:

```
<crate>/man/<tool>.1.adoc          ← English source (AsciiDoc, manpage backend)
<crate>/man/fr_CA/<tool>.1.adoc    ← Canadian French source
```

**Rationale:** Per-crate placement follows the existing `umrs-ls` pattern,
keeps man page sources adjacent to the code they document, and allows crate
owners to update man pages in the same commit as CLI changes without touching
a shared tree. Centralized placement was considered and rejected; it would
require cross-crate coordination for every option change.

### Current source inventory

| Crate       | English source                      | French source                            | Status              |
|-------------|-------------------------------------|------------------------------------------|---------------------|
| `umrs-ls`   | `umrs-ls/man/umrs-ls.1.adoc`        | `umrs-ls/man/fr_CA/umrs-ls.1.adoc`       | To be created       |
| `umrs-c2pa` | `umrs-c2pa/man/umrs-c2pa.1.adoc`    | `umrs-c2pa/man/fr_CA/umrs-c2pa.1.adoc`   | To be created       |
| `umrs-label`| `umrs-label/man/umrs-label.1.adoc`  | `umrs-label/man/fr_CA/umrs-label.1.adoc` | To be created       |
| `umrs-stat` | `umrs-stat/man/umrs-stat.1.adoc`    | `umrs-stat/man/fr_CA/umrs-stat.1.adoc`   | To be created       |
| `umrs-uname`| `umrs-uname/man/umrs-uname.1.adoc`  | `umrs-uname/man/fr_CA/umrs-uname.1.adoc` | To be created       |

**Note:** `umrs-c2pa/docs/umrs-c2pa.1` and `umrs-c2pa/docs/fr/umrs-c2pa.1` are
pre-existing groff-format man pages. They must be converted to AsciiDoc and
moved to `umrs-c2pa/man/` before the `make man-pages` target will pick them up.
The groff originals should be retained until the AsciiDoc versions are verified.
This conversion is tracked as a separate task.

---

## Section Layout

Every UMRS man page must contain the following sections, in this order.
See `docs/man-page-template.adoc` for the complete AsciiDoc template.

| Section        | Required | Notes                                              |
|----------------|----------|----------------------------------------------------|
| `NAME`         | Yes      | One-line description, identical to `--help` first line |
| `SYNOPSIS`     | Yes      | All command forms, one per line                    |
| `DESCRIPTION`  | Yes      | What the tool does; security posture context       |
| `OPTIONS`      | Yes      | All flags; group by function if > 8 options        |
| `EXIT CODES`   | Yes      | Full exit code table; follow `secure_bash_rules.md` codes |
| `FILES`        | Yes if applicable | Config paths, key files, output locations |
| `ENVIRONMENT`  | Yes if applicable | Env vars that affect behaviour                |
| `COMPLIANCE`   | Yes      | NIST SP 800-53 controls the tool satisfies         |
| `SEE ALSO`     | Yes      | Cross-references to related tools; man(1) format   |
| `AUTHOR`       | Yes      | Standard UMRS attribution line                     |

---

## Build Pipeline

Man pages are built by `make man-pages` from the workspace Makefile at
`components/rusty-gadgets/Makefile`. The target is also invoked automatically
by `cargo xtask stage`.

The build requires `asciidoctor` with the `asciidoctor-pdf` and manpage
backend support. On RHEL 10:

```
sudo dnf install rubygem-asciidoctor
```

On Ubuntu:

```
sudo apt install asciidoctor
```

Verify the installation:

```
asciidoctor --version
```

### Staging output layout

```
staging/
  share/
    man/
      man1/          ← English compiled .1 pages
      fr_CA/
        man1/        ← Canadian French compiled .1 pages
```

---

## Verification

After running `make man-pages` or `cargo xtask stage`, verify a rendered man
page as follows.

1. Run `make man-pages` from `components/rusty-gadgets/`.

   The build prints `[man] compiled: staging/share/man/man1/<tool>.1` for each
   page processed.

2. Open the compiled English man page for `umrs-ls`:

   ```
   man -l staging/share/man/man1/umrs-ls.1
   ```

   The page renders in the system pager. Verify that NAME, SYNOPSIS, and all
   required sections are present.

3. Open the French man page:

   ```
   man -l staging/share/man/fr_CA/man1/umrs-ls.1
   ```

   Verify French section headings and content.

4. Check that no AsciiDoc source warnings appear in the build output.
   Warnings about missing attributes or unresolved cross-references are build
   failures — fix them before committing.

---

## Compliance

- `NIST SP 800-53 SA-22` — Unsupported System Components: man pages are the
  operator-facing documentation artifact for each installed binary. Absence of
  a man page for an installed tool represents a documentation gap that an
  auditor may flag as incomplete operator guidance.
- `NIST SP 800-53 CM-2` — Baseline Configuration: the staged man pages are
  part of the auditable deployment artifact set produced before IMA signing.
