//! Staging pipeline — builds the workspace, assembles compiled binaries,
//! end-user scripts, configuration files, and man pages into a flat
//! `staging/` directory tree for installer consumption.
//!
//! ## Build-first guarantee
//!
//! `run()` invokes `cargo build --workspace` (with `--release` when requested)
//! before copying any artifacts. This ensures the staging bundle is never
//! assembled from a stale or partial `target/` directory. The build step cannot
//! be skipped; if it fails the entire pipeline aborts with a non-zero exit code.
//!
//! ## Expected binary set
//!
//! After copying binaries from `target/{profile}/`, the pipeline verifies that
//! every name in [`EXPECTED_BINARIES`] is present in `staging/bin/`. If any
//! are missing the pipeline bails with a precise error message naming them.
//! The declared set is:
//! `umrs-c2pa`, `umrs-label`, `umrs-ls`, `umrs-stat`, `umrs-uname`.
//!
//! ## Staging layout (FHS 2.3 compliant)
//!
//! The staging tree mirrors the final on-disk layout under `/opt/umrs/`,
//! which is the package's FHS `/opt/<package>` root (FHS 2.3 §3.12, §5.15).
//! Host-editable configuration (`/etc/opt/umrs/`) and variable data
//! (`/var/opt/umrs/`) are NOT part of the staging bundle: per FHS 2.3 §3.7.4
//! and §4.9.2 those hierarchies live outside `/opt/<package>` and are
//! provisioned by the installer, not shipped inside the artifact set.
//!
//! ```text
//! staging/
//!   bin/            ← compiled workspace binaries (expected set verified) +
//!                     end-user scripts
//!   share/
//!     umrs/         ← static package reference databases
//!                     (CUI labels, Canadian Protected catalog, MCS levels,
//!                      palette). FHS 2.3 §4.11 — package-specific static
//!                      architecture-independent data, analogous to
//!                      /usr/share/zoneinfo or /usr/share/mime.
//!       templates/  ← configuration templates operators copy and customize
//!                     into /etc/ (e.g., setrans.conf templates destined for
//!                     /etc/selinux/<policy>/setrans.conf).
//!     man/
//!       man1/       ← English man pages copied from <crate>/docs/*.1
//!       fr/
//!         man1/     ← Canadian French man pages from <crate>/docs/fr/*.1
//!     locale/
//!       <locale>/
//!         LC_MESSAGES/
//!                   ← compiled gettext catalogs copied from
//!                     <crate>/locale/<locale>/LC_MESSAGES/*.mo
//! ```
//!
//! ## Config classification rule
//!
//! Files under each crate's `config/` directory are sorted into two staging
//! destinations based on filename suffix:
//!
//! - `*.json` → `staging/share/umrs/`
//! - `*setrans*template*` (covers both `.conf.template` and `.conf-template`
//!   historical suffix variants) → `staging/share/umrs/templates/`
//!
//! This split reflects the FHS distinction between immutable reference data
//! (zoneinfo-style databases) and editable admin configuration. The JSON
//! catalogs are package-owned reference data; the `.template` files become
//! configuration only after an operator customizes and places them.
//!
//! ## Man page phase
//!
//! [`stage_man_pages`] runs after config staging. It sweeps each binary crate
//! for pre-built troff sources at `<crate>/docs/*.1` (English) and
//! `<crate>/docs/fr/*.1` (Canadian French) and copies them into the staging
//! layout. Man pages are authored and checked in as troff; no compilation
//! step runs at stage time. Crates without docs emit no output and no error.
//!
//! ## Locale phase
//!
//! [`stage_locales`] sweeps each binary crate for compiled gettext catalogs
//! at `<crate>/locale/<locale>/LC_MESSAGES/*.mo` and copies each into the
//! matching staging path. The locale set is discovered at stage time from
//! the filesystem — adding a new locale requires no code change.
//!
//! Man pages and `.mo` catalogs are independent artifact classes with
//! different owners (documentation vs. translation). Staging does not
//! require both to be present for a given crate; each is staged if it
//! exists and skipped if it does not.
//!
//! ## Compliance
//!
//! - `NIST SP 800-53 AC-3` — Access Enforcement: execute permission is
//!   verified before any binary or script is copied to `staging/bin/`.
//! - `NIST SP 800-53 CM-2` — Baseline Configuration: reference-data and
//!   template staging captures the authoritative package baseline per crate.
//! - `NIST SP 800-53 CM-7` — Least Functionality: only well-typed artifact
//!   classes (executables, reference data, templates, man pages, compiled
//!   catalogs) are accepted; all other artifacts are excluded by the filter.
//! - `FHS 2.3 §3.12, §5.15` — `/opt/<package>` layout. Static files live
//!   inside the package tree; host-specific config and variable data live
//!   outside it.
//! - `FHS 2.3 §4.11` — `/usr/share/<package>` pattern for static reference
//!   databases. The `share/umrs/` subtree mirrors this convention inside
//!   the `/opt/umrs/` root.
//! - `NIST SP 800-53 CM-8` — Component Inventory: the expected binary
//!   manifest ([`EXPECTED_BINARIES`]) declares the complete artifact set.
//!   Staging fails if any declared component is absent, preventing an
//!   incomplete deployment bundle from reaching the IMA signing step.
//! - `NIST SP 800-53 SA-12` — Supply Chain Protection: staging is the
//!   pre-installation checkpoint at which IMA signing will be applied before
//!   deployment. A missing binary would corrupt the component inventory and
//!   produce a signed bundle that does not match the declared artifact set.
//! - `NIST SP 800-53 SA-22` — Unsupported System Components: man pages are
//!   the operator-facing documentation artifact for each installed binary.
//!   Staging them alongside binaries ensures the documentation artifact set
//!   is present at the IMA signing checkpoint.

use anyhow::{Context, Result, bail};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Returns the workspace root by walking one level above `CARGO_MANIFEST_DIR`.
///
/// `xtask` always lives at `<workspace_root>/xtask/`. Using the compile-time
/// manifest constant avoids any dependency on the process working directory.
///
/// # Panics
///
/// Panics at startup if the xtask crate has been moved to the filesystem root
/// (i.e., `CARGO_MANIFEST_DIR` has no parent). This cannot happen in a normal
/// Cargo workspace layout.
pub fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask has no parent directory")
        .to_owned()
}

/// The complete set of binary names that must be present in `staging/bin/`
/// after the copy step.
///
/// If any name in this list is absent after [`stage_binaries`] completes,
/// the pipeline aborts with a precise error message. This prevents an
/// incomplete deployment bundle from reaching the IMA signing checkpoint.
///
/// ## Compliance
///
/// - `NIST SP 800-53 CM-8` — Component Inventory: this constant is the
///   declared artifact inventory. Verification against it enforces that
///   the staged bundle matches the declared set exactly.
/// - `NIST SP 800-53 SA-12` — Supply Chain Protection: a partial bundle
///   would corrupt the inventory at the signing boundary.
pub const EXPECTED_BINARIES: &[&str] =
    &["umrs-c2pa", "umrs-label", "umrs-ls", "umrs-stat", "umrs-uname"];

/// File extensions that are never staged, even if they lack an extension in
/// the stem sense (e.g. a file named literally `.rlib` would still match).
///
/// These are Rust build artifacts that are not executable products.
///
/// ## Compliance
///
/// `NIST SP 800-53 CM-7` — only known, intentional artifact types proceed.
const EXCLUDED_EXTENSIONS: &[&str] = &["d", "so", "rlib", "rmeta", "dylib", "a"];

/// Binary names that are workspace dev tooling and must never be staged for
/// deployment.
///
/// `xtask` is the staging/build orchestrator itself — it runs at build time on
/// developer workstations and CI, not on target hosts. Shipping it would
/// violate least-functionality.
///
/// ## Compliance
///
/// `NIST SP 800-53 CM-7` — Least Functionality.
const EXCLUDED_BINARY_NAMES: &[&str] = &["xtask"];

/// Returns `true` if `path` is a compiled binary that should be staged.
///
/// A file is stageable when ALL of the following hold:
/// - It is a regular file (not a symlink, not a directory).
/// - It has no file extension (i.e. `.extension()` is `None`).
/// - Its Unix permission bits include at least one execute bit (`& 0o111 != 0`).
///
/// Files whose extension matches [`EXCLUDED_EXTENSIONS`] are rejected
/// regardless of other attributes. This guard is belt-and-suspenders: a file
/// named `foo.so` has an extension and would already fail the "no extension"
/// check, but the explicit list makes the intent clear to readers and provides
/// defence against future filter changes.
///
/// ## Compliance
///
/// `NIST SP 800-53 AC-3`, `NIST SP 800-53 CM-7`.
pub fn is_stageable_binary(path: &Path) -> Result<bool> {
    let meta = match path.metadata() {
        Ok(m) => m,
        Err(_) => return Ok(false),
    };

    if !meta.is_file() {
        return Ok(false);
    }

    // Reject known non-binary extensions explicitly.
    if let Some(ext) = path.extension() {
        let ext_str = ext.to_string_lossy();
        if EXCLUDED_EXTENSIONS.iter().any(|&e| e == ext_str.as_ref()) {
            return Ok(false);
        }
        // Any other extension also disqualifies (compiled binaries have none).
        return Ok(false);
    }

    // Must have at least one execute bit set.
    let mode = meta.permissions().mode();
    Ok(mode & 0o111 != 0)
}

/// Copies compiled workspace binaries from `target/{profile}/` into
/// `staging/bin/`.
///
/// If the target directory does not exist, prints a clear diagnostic and
/// returns an error so the caller can exit with a non-zero code.
///
/// ## Compliance
///
/// `NIST SP 800-53 AC-3` — execute permission verified per file.
/// `NIST SP 800-53 CM-7` — only executables without extensions are copied.
fn stage_binaries(workspace_root: &Path, release: bool) -> Result<()> {
    let profile = if release {
        "release"
    } else {
        "debug"
    };
    let target_dir = workspace_root.join("target").join(profile);

    if !target_dir.exists() {
        // build_workspace() should have created this; reaching here means the
        // build succeeded but produced no target directory — treat as fatal.
        bail!("target/{profile}/ not found after build — workspace may have no binary targets");
    }

    let staging_bin = workspace_root.join("staging").join("bin");
    fs::create_dir_all(&staging_bin)
        .with_context(|| format!("creating {}", staging_bin.display()))?;

    let entries =
        fs::read_dir(&target_dir).with_context(|| format!("reading {}", target_dir.display()))?;

    let mut count = 0usize;
    for entry in entries {
        let entry = entry.with_context(|| format!("reading entry in {}", target_dir.display()))?;
        let path = entry.path();

        let stageable = is_stageable_binary(&path)?;
        if !stageable {
            continue;
        }

        let filename = match path.file_name() {
            Some(n) => n,
            None => continue,
        };

        // Skip dev tooling that must not ship (e.g., xtask).
        let name_str = filename.to_string_lossy();
        if EXCLUDED_BINARY_NAMES.iter().any(|&n| n == name_str.as_ref()) {
            eprintln!("[stage] skip dev tool: {name_str}");
            continue;
        }

        let dest = staging_bin.join(filename);
        fs::copy(&path, &dest)
            .with_context(|| format!("copying {} to {}", path.display(), dest.display()))?;
        count = count.saturating_add(1);
        eprintln!("[stage] bin: {}", filename.to_string_lossy());
    }

    eprintln!("[stage] staged {count} binary/binaries from target/{profile}/");
    Ok(())
}

/// Copies end-user scripts from `scripts/` into `staging/bin/`, recursing
/// exactly one level deep.
///
/// ## Discovery rules
///
/// Scripts are discovered from two locations:
/// - `scripts/*.sh` — flat files directly under `scripts/`
/// - `scripts/*/*.sh` — files one level below a subdirectory (e.g. `scripts/signing/foo.sh`)
///
/// The depth bound is intentional and hard-coded. Files at `scripts/a/b/c.sh`
/// (depth ≥ 2) are silently skipped. This prevents accidental ingestion of
/// deeply nested drafts, archived material, or work-in-progress trees.
///
/// ## Target name derivation
///
/// The `.sh` suffix is stripped on copy. The target name in `staging/bin/`
/// is the filename stem only:
/// ```text
/// scripts/umrs-signing/umrs-sign-mgr.sh  →  staging/bin/umrs-sign-mgr
/// scripts/umrs-shred.sh                  →  staging/bin/umrs-shred
/// ```
///
/// This matches the naming convention for compiled workspace binaries (e.g.
/// `umrs-ls`, `umrs-stat`). The installer places all `staging/bin/` artifacts
/// at `/opt/umrs/bin/` without modification.
///
/// ## Provenance note (`NSA RTB`)
///
/// The `.sh` suffix is present in the repository and absent in the staging
/// bundle. This is an intentional provenance boundary: an operator running
/// `umrs-sign-mgr` is executing the staged (suffix-stripped) copy, not the
/// repository source. The IMA signing checkpoint signs the staged artifact.
/// Any post-strip substitution would produce a signing failure. This design
/// is consistent with `NSA RTB` non-bypassability — the staging copy is the
/// object of trust, not the source path.
///
/// ## Duplicate detection
///
/// If a flat script (`scripts/foo.sh`) and a nested script
/// (`scripts/bar/foo.sh`) would produce the same target name, staging aborts
/// with a hard error listing both conflicting paths. Ambiguous target names
/// are a supply-chain integrity violation and cannot be resolved silently.
///
/// ## Execute-bit guard
///
/// Files without the execute bit emit a warning and are NOT copied. Scripts
/// MUST be committed to git with the execute bit set:
///
/// ```text
/// git add --chmod=+x scripts/<name>.sh
/// ```
///
/// ## Compliance
///
/// `NIST SP 800-53 AC-3` — execute permission verified before copy.
/// `NIST SP 800-53 SA-12` — scripts enter the staging checkpoint alongside
/// compiled binaries, enabling uniform IMA-signing of the full `bin/` tree.
/// `NSA RTB` — suffix-strip provenance boundary; staged artifact is the
/// signed object, preventing source-path substitution attacks.
fn stage_scripts(workspace_root: &Path) -> Result<()> {
    let scripts_dir = workspace_root.join("scripts");

    if !scripts_dir.exists() {
        // Silent skip — scripts/ is optional.
        return Ok(());
    }

    let staging_bin = workspace_root.join("staging").join("bin");
    fs::create_dir_all(&staging_bin)
        .with_context(|| format!("creating {}", staging_bin.display()))?;

    // Collect (target_stem → source_path) pairs. Duplicate stems are a hard error.
    let mut candidates: std::collections::HashMap<String, PathBuf> =
        std::collections::HashMap::new();

    // Pass 1: flat scripts/*.sh
    collect_scripts(&scripts_dir, &scripts_dir, false, &mut candidates)?;

    // Pass 2: one-level subdirectories scripts/*/*.sh
    let top_entries =
        fs::read_dir(&scripts_dir).with_context(|| format!("reading {}", scripts_dir.display()))?;

    for entry in top_entries {
        let entry = entry.with_context(|| format!("reading entry in {}", scripts_dir.display()))?;
        let path = entry.path();
        let meta = path.metadata().with_context(|| format!("stat {}", path.display()))?;

        if meta.is_dir() {
            collect_scripts(&scripts_dir, &path, true, &mut candidates)?;
        }
    }

    // Stage each validated candidate.
    let mut count = 0usize;
    for (stem, src) in &candidates {
        let meta = src.metadata().with_context(|| format!("stat {}", src.display()))?;
        let mode = meta.permissions().mode();

        // NIST SP 800-53 AC-3: enforce execute permission at staging boundary.
        if mode & 0o111 == 0 {
            let rel = src.strip_prefix(&scripts_dir).unwrap_or(src);
            eprintln!(
                "WARNING: scripts/{} is not executable — skipping",
                rel.display()
            );
            continue;
        }

        let dest = staging_bin.join(stem);
        fs::copy(src, &dest)
            .with_context(|| format!("copying {} to {}", src.display(), dest.display()))?;
        count = count.saturating_add(1);
        eprintln!("[stage] script: {stem}");
    }

    eprintln!("[stage] staged {count} script(s) from scripts/");
    Ok(())
}

/// Scans `search_dir` for `*.sh` files and inserts them into `candidates`.
///
/// `scripts_root` is used only for diagnostic messages (relative path display).
/// `nested` controls whether this is a flat (`scripts/*.sh`) or nested
/// (`scripts/*/*.sh`) scan — used only for diagnostic labels.
///
/// Duplicate stems across flat and nested locations are a hard error: the
/// function calls `bail!` with both conflicting paths.
///
/// ## Compliance
///
/// `NIST SP 800-53 SA-12` — duplicate detection prevents ambiguous supply-chain
/// artifacts from silently shadowing each other at the staging boundary.
fn collect_scripts(
    scripts_root: &Path,
    search_dir: &Path,
    nested: bool,
    candidates: &mut std::collections::HashMap<String, PathBuf>,
) -> Result<()> {
    let entries =
        fs::read_dir(search_dir).with_context(|| format!("reading {}", search_dir.display()))?;

    for entry in entries {
        let entry = entry.with_context(|| format!("reading entry in {}", search_dir.display()))?;
        let path = entry.path();
        let meta = path.metadata().with_context(|| format!("stat {}", path.display()))?;

        if !meta.is_file() {
            if nested && meta.is_dir() {
                // Depth ≥ 2 — skip silently. Bounding recursion is intentional.
            }
            continue;
        }

        // Only `.sh` files are script candidates.
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        if ext != "sh" {
            continue;
        }

        // Derive target stem by stripping the `.sh` suffix.
        let stem = match path.file_stem().and_then(|s| s.to_str()) {
            Some(s) => s.to_owned(),
            None => continue,
        };

        // Duplicate detection: same stem from different source paths is fatal.
        if let Some(existing) = candidates.get(&stem) {
            let rel_existing = existing.strip_prefix(scripts_root).unwrap_or(existing);
            let rel_new = path.strip_prefix(scripts_root).unwrap_or(&path);
            bail!(
                "duplicate script target name '{stem}': two source scripts would \
                 produce the same staging/bin/{stem} artifact.\n  \
                 existing: scripts/{}\n  \
                 conflict: scripts/{}",
                rel_existing.display(),
                rel_new.display()
            );
        }

        candidates.insert(stem, path);
    }

    // Suppress the `nested` unused-variable lint for the depth-guard comment above.
    let _ = nested;
    Ok(())
}

/// Crate directories to scan for a `config/` subdirectory.
///
/// Paths are relative to the workspace root.
const CONFIG_CRATES: &[&str] = &[
    "libs/umrs-core",
    "libs/umrs-hw",
    "libs/umrs-platform",
    "libs/umrs-selinux",
    "libs/umrs-ui",
    "umrs-label",
    "umrs-ls",
    "umrs-stat",
    "umrs-uname",
];

/// Classification of a single source file found under a crate's `config/`
/// directory, used to decide which staging subtree it belongs in.
///
/// The classification is filename-driven and deliberately coarse. The rules
/// are defined in [`classify_config_file`].
enum ConfigKind {
    /// Static package reference database — goes under `staging/share/umrs/`.
    /// Per FHS 2.3 §4.11 this is the `/usr/share/<package>` analogue.
    Database,
    /// Admin-editable configuration template — goes under
    /// `staging/share/umrs/templates/`. Operators copy and customize these
    /// into `/etc/` (for UMRS: `/etc/selinux/<policy>/setrans.conf`).
    Template,
    /// Unclassified file. Currently ignored with a diagnostic so that a
    /// stray README or backup file does not silently land in a reference
    /// database directory and get labeled as trusted package data.
    Unknown,
}

/// Classifies a file under `config/` by its filename.
///
/// The rules, in order:
///
/// 1. `*.json` → `ConfigKind::Database`
/// 2. `*setrans*template*`, case-sensitive — matches both
///    `MLS-setrans.conf.template` and `TARGETED-setrans.conf-template`
///    (historical inconsistent suffix) → `ConfigKind::Template`
/// 3. Everything else → `ConfigKind::Unknown`
///
/// The classifier is a pure function on filename. It does not read file
/// contents and does not follow symlinks.
fn classify_config_file(name: &str) -> ConfigKind {
    if name.ends_with(".json") {
        return ConfigKind::Database;
    }
    if name.contains("setrans") && name.contains("template") {
        return ConfigKind::Template;
    }
    ConfigKind::Unknown
}

/// Sweeps each crate's `config/` directory (non-recursive at top level —
/// `_scratch/` subdirectories and other nested dirs are skipped) and copies
/// each file into the staging location chosen by [`classify_config_file`].
///
/// Scratch directories (`_scratch/`) and other subdirectories under `config/`
/// are never staged. The flat layout is enforced because under the new
/// FHS-compliant scheme there is no `us/` or `ca/` subdir — all JSON
/// databases live at the same level of `share/umrs/`.
///
/// Files with unknown classification are logged as a warning and skipped
/// rather than copied into an unclear location. This is a fail-safe choice:
/// a misfiled artifact landing in `share/umrs/` would be labeled as trusted
/// package reference data by the SELinux policy.
///
/// ## Collision policy
///
/// No filename collisions are expected across crates. If two crates ship a
/// file with the same name and classification, the second write will
/// overwrite the first. This matches the prior behaviour and is noted so
/// that a future collision is diagnosed as a data bug, not a tool bug.
///
/// ## Compliance
///
/// - `NIST SP 800-53 CM-2` — Baseline Configuration: reference data and
///   templates are part of the auditable deployment artifact set.
/// - `NIST SP 800-53 CM-7` — Least Functionality: unclassified files are
///   rejected rather than admitted to the staging bundle.
/// - `FHS 2.3 §4.11` — static package-specific data layout.
fn stage_configs(workspace_root: &Path) -> Result<()> {
    let data_dst = workspace_root.join("staging").join("share").join("umrs");
    let tmpl_dst = data_dst.join("templates");

    let mut data_count = 0usize;
    let mut tmpl_count = 0usize;
    let mut skipped = 0usize;
    let mut any_crate = false;

    for &crate_rel in CONFIG_CRATES {
        let config_dir = workspace_root.join(crate_rel).join("config");
        if !config_dir.exists() {
            continue;
        }
        any_crate = true;

        for entry in fs::read_dir(&config_dir)
            .with_context(|| format!("reading {}", config_dir.display()))?
        {
            let entry =
                entry.with_context(|| format!("reading entry in {}", config_dir.display()))?;
            let src_path = entry.path();
            let meta = entry.metadata().with_context(|| format!("stat {}", src_path.display()))?;

            // Non-recursive at top of config/: skip subdirectories silently.
            // Under the FHS-compliant scheme there is no nested layout to
            // preserve — every shipped file is a top-level file.
            if !meta.is_file() {
                continue;
            }

            let file_name = entry.file_name();
            let name_cow = file_name.to_string_lossy();
            let name: &str = name_cow.as_ref();

            match classify_config_file(name) {
                ConfigKind::Database => {
                    fs::create_dir_all(&data_dst)
                        .with_context(|| format!("creating {}", data_dst.display()))?;
                    let dst = data_dst.join(&file_name);
                    fs::copy(&src_path, &dst).with_context(|| {
                        format!("copying {} to {}", src_path.display(), dst.display())
                    })?;
                    data_count = data_count.saturating_add(1);
                    eprintln!("[stage] share/umrs: {name}");
                }
                ConfigKind::Template => {
                    fs::create_dir_all(&tmpl_dst)
                        .with_context(|| format!("creating {}", tmpl_dst.display()))?;
                    let dst = tmpl_dst.join(&file_name);
                    fs::copy(&src_path, &dst).with_context(|| {
                        format!("copying {} to {}", src_path.display(), dst.display())
                    })?;
                    tmpl_count = tmpl_count.saturating_add(1);
                    eprintln!("[stage] share/umrs/templates: {name}");
                }
                ConfigKind::Unknown => {
                    skipped = skipped.saturating_add(1);
                    eprintln!(
                        "WARNING: {crate_rel}/config/{name} is not a classified \
                         database or template — skipping"
                    );
                }
            }
        }

        eprintln!("[stage] scanned {crate_rel}/config/");
    }

    if any_crate {
        eprintln!(
            "[stage] share staging complete → {} database(s), {} template(s){}",
            data_count,
            tmpl_count,
            if skipped > 0 {
                format!(", {skipped} file(s) skipped — see warnings above")
            } else {
                String::new()
            }
        );
    } else {
        eprintln!("[stage] no config/ directories found in workspace crates");
    }

    Ok(())
}

/// Binary crates that may carry man pages and compiled locale catalogs.
///
/// Both artifact classes are optional per-crate: a crate with no `docs/*.1`
/// files contributes no man pages; a crate with no populated
/// `locale/<locale>/LC_MESSAGES/*.mo` contributes no catalogs.
///
/// Paths are relative to the workspace root.
const DOC_CRATES: &[&str] = &["umrs-c2pa", "umrs-label", "umrs-ls", "umrs-stat", "umrs-uname"];

/// Copies pre-built troff man pages from each binary crate into
/// `staging/share/man/`.
///
/// ## Source discovery
///
/// For each crate listed in [`DOC_CRATES`]:
///
/// - English sources:  `<crate>/docs/*.1`
/// - Canadian French:  `<crate>/docs/fr/*.1`
///
/// Files matching these globs are assumed to be finished groff/troff man
/// pages (as authored by Simone and committed to the tree). They are copied
/// verbatim into `staging/share/man/man1/` and `staging/share/man/fr/man1/`
/// respectively. No compilation runs at stage time.
///
/// ## Soft-failure policy
///
/// A crate without a `docs/` directory or without any `*.1` file is not an
/// error. Man pages are authored asynchronously by the documentation team
/// and not every crate will have them on every release.
///
/// ## Coupling with locale catalogs
///
/// Man pages and compiled gettext catalogs are independent artifact classes
/// produced by different owners. Staging does NOT require a crate with a
/// man page to also ship a `.mo` for each declared locale, and vice versa.
/// Decoupling prevents the documentation pipeline from stalling whenever a
/// translator is mid-pass.
///
/// ## Compliance
///
/// - `NIST SP 800-53 SA-22` — Unsupported System Components: man pages are
///   the operator-facing documentation artifact for each installed binary.
/// - `NIST SP 800-53 CM-2` — Baseline Configuration: man pages are part of
///   the auditable deployment artifact set produced before IMA signing.
/// - `NIST SP 800-53 CM-8` — Component Inventory: [`DOC_CRATES`] declares
///   which crates contribute documentation artifacts.
fn stage_man_pages(workspace_root: &Path) -> Result<()> {
    let en_dst = workspace_root.join("staging").join("share").join("man").join("man1");
    let fr_dst = workspace_root.join("staging").join("share").join("man").join("fr").join("man1");

    let mut total = 0usize;

    for &crate_rel in DOC_CRATES {
        let docs_dir = workspace_root.join(crate_rel).join("docs");
        if !docs_dir.is_dir() {
            continue;
        }

        total = total.saturating_add(copy_troff_pages(&docs_dir, &en_dst, "man1")?);

        let fr_dir = docs_dir.join("fr");
        if fr_dir.is_dir() {
            total = total.saturating_add(copy_troff_pages(&fr_dir, &fr_dst, "fr/man1")?);
        }
    }

    if total == 0 {
        eprintln!("[stage] no man pages found in any crate — skipping");
    } else {
        eprintln!("[stage] man pages staged → staging/share/man/ ({total} page(s))");
    }

    Ok(())
}

/// Copies every `*.1` file directly under `src_dir` (non-recursive) into
/// `dst_dir`. Returns the number of pages copied.
///
/// Non-regular entries and files that do not end in `.1` are skipped
/// silently. The destination is created if it does not exist.
fn copy_troff_pages(src_dir: &Path, dst_dir: &Path, label: &str) -> Result<usize> {
    let mut entries: Vec<_> = fs::read_dir(src_dir)
        .with_context(|| format!("reading {}", src_dir.display()))?
        .filter_map(std::result::Result::ok)
        .collect();
    entries.sort_by_key(std::fs::DirEntry::file_name);

    let mut count = 0usize;
    let mut created_dst = false;

    for entry in entries {
        let path = entry.path();

        // Only regular files ending in .1 are staged as man pages.
        let meta = match path.metadata() {
            Ok(m) => m,
            Err(_) => continue,
        };
        if !meta.is_file() {
            continue;
        }
        if path.extension().and_then(|e| e.to_str()) != Some("1") {
            continue;
        }

        if !created_dst {
            fs::create_dir_all(dst_dir)
                .with_context(|| format!("creating {}", dst_dir.display()))?;
            created_dst = true;
        }

        let filename = match path.file_name() {
            Some(n) => n,
            None => continue,
        };
        let dst = dst_dir.join(filename);
        fs::copy(&path, &dst)
            .with_context(|| format!("copying {} to {}", path.display(), dst.display()))?;
        count = count.saturating_add(1);
        eprintln!("[stage] man ({label}): {}", filename.to_string_lossy());
    }

    Ok(count)
}

/// Copies compiled gettext catalogs from each binary crate into
/// `staging/share/locale/<locale>/LC_MESSAGES/`.
///
/// ## Source discovery
///
/// For each crate listed in [`DOC_CRATES`], the set of locales is discovered
/// at stage time by listing immediate subdirectories of `<crate>/locale/`.
/// For each such locale, every `*.mo` file under `LC_MESSAGES/` is copied
/// into `staging/share/locale/<locale>/LC_MESSAGES/`.
///
/// This means adding a new locale is purely a filesystem operation — no
/// change to this code is required to stage it.
///
/// ## Empty catalogs are not errors
///
/// An empty `LC_MESSAGES/` directory is skipped silently. A missing `locale/`
/// directory is skipped silently. The pipeline continues.
///
/// ## Filename collisions
///
/// Two crates compiling the same `<domain>.mo` filename into the same locale
/// would collide in the staging layout. Today each crate uses a distinct
/// domain name matching its binary name (e.g., `umrs-c2pa.mo`), so no
/// collision is possible. If that convention ever changes, this function
/// will overwrite silently — documented here so the symptom is understood.
///
/// ## Compliance
///
/// - `NIST SP 800-53 CM-2` — Baseline Configuration: compiled catalogs are
///   part of the auditable deployment artifact set.
/// - `NIST SP 800-53 CM-8` — Component Inventory: [`DOC_CRATES`] declares
///   which crates contribute localization artifacts.
fn stage_locales(workspace_root: &Path) -> Result<()> {
    let locale_root = workspace_root.join("staging").join("share").join("locale");
    let mut total = 0usize;

    for &crate_rel in DOC_CRATES {
        let crate_locale = workspace_root.join(crate_rel).join("locale");
        if !crate_locale.is_dir() {
            continue;
        }

        let locale_dirs = fs::read_dir(&crate_locale)
            .with_context(|| format!("reading {}", crate_locale.display()))?;

        for locale_entry in locale_dirs {
            let locale_entry = locale_entry
                .with_context(|| format!("reading entry in {}", crate_locale.display()))?;
            let locale_path = locale_entry.path();
            let meta = match locale_path.metadata() {
                Ok(m) => m,
                Err(_) => continue,
            };
            if !meta.is_dir() {
                continue;
            }

            let locale_name = match locale_entry.file_name().into_string() {
                Ok(s) => s,
                Err(_) => continue,
            };

            let src_lc = locale_path.join("LC_MESSAGES");
            if !src_lc.is_dir() {
                continue;
            }

            let dst_lc = locale_root.join(&locale_name).join("LC_MESSAGES");

            for mo_entry in
                fs::read_dir(&src_lc).with_context(|| format!("reading {}", src_lc.display()))?
            {
                let mo_entry =
                    mo_entry.with_context(|| format!("reading entry in {}", src_lc.display()))?;
                let mo_path = mo_entry.path();
                let mo_meta = match mo_path.metadata() {
                    Ok(m) => m,
                    Err(_) => continue,
                };
                if !mo_meta.is_file() {
                    continue;
                }
                if mo_path.extension().and_then(|e| e.to_str()) != Some("mo") {
                    continue;
                }

                fs::create_dir_all(&dst_lc)
                    .with_context(|| format!("creating {}", dst_lc.display()))?;

                let filename = match mo_path.file_name() {
                    Some(n) => n,
                    None => continue,
                };
                let dst = dst_lc.join(filename);
                fs::copy(&mo_path, &dst).with_context(|| {
                    format!("copying {} to {}", mo_path.display(), dst.display())
                })?;
                total = total.saturating_add(1);
                eprintln!(
                    "[stage] locale ({locale_name}): {}",
                    filename.to_string_lossy()
                );
            }
        }
    }

    if total == 0 {
        eprintln!("[stage] no compiled .mo catalogs found in any crate — skipping");
    } else {
        eprintln!("[stage] locale catalogs staged → staging/share/locale/ ({total} catalog(s))");
    }

    Ok(())
}

/// Builds the full workspace before staging.
///
/// Executes `cargo build --workspace` (with `--release` when `release` is
/// `true`) as a mandatory pre-staging step. If the build fails, the entire
/// pipeline aborts immediately — there is no way to stage without a fresh
/// build.
///
/// ## Compliance
///
/// - `NIST SP 800-53 CM-8` — Component Inventory: building before staging
///   guarantees that `target/` reflects the current source tree, so the
///   staged bundle matches the declared artifact set.
/// - `NIST SP 800-53 SA-12` — Supply Chain Protection: a stale or partial
///   `target/` would produce a corrupt deployment bundle.
fn build_workspace(release: bool) -> Result<()> {
    let mut cmd = Command::new("cargo");
    cmd.arg("build").arg("--workspace");
    if release {
        cmd.arg("--release");
    }
    eprintln!(
        "[stage] building workspace (profile: {})",
        if release {
            "release"
        } else {
            "debug"
        }
    );
    crate::run_cmd(&mut cmd)
}

/// Verifies that every name in [`EXPECTED_BINARIES`] is present in
/// `staging/bin/` after the copy step.
///
/// If any expected binary is absent, bails with a precise error message
/// listing only the missing names so the operator knows exactly what to fix.
///
/// ## Compliance
///
/// - `NIST SP 800-53 CM-8` — Component Inventory: verification enforces that
///   the staged bundle contains the complete declared artifact set.
/// - `NIST SP 800-53 SA-12` — Supply Chain Protection: an incomplete bundle
///   must never proceed to the IMA signing checkpoint.
fn verify_staged_binaries(workspace_root: &Path) -> Result<()> {
    let staging_bin = workspace_root.join("staging").join("bin");

    let missing: Vec<&str> = EXPECTED_BINARIES
        .iter()
        .copied()
        .filter(|&name| !staging_bin.join(name).exists())
        .collect();

    if missing.is_empty() {
        eprintln!(
            "[stage] binary manifest verified — all {} expected binaries present",
            EXPECTED_BINARIES.len()
        );
        return Ok(());
    }

    bail!(
        "staging incomplete — {} expected binary/binaries missing from staging/bin/: {}",
        missing.len(),
        missing.join(", ")
    );
}

/// Runs the full staging pipeline.
///
/// 1. Builds the workspace (`cargo build --workspace [--release]`).
/// 2. Copies compiled binaries from `target/{profile}/` to `staging/bin/`.
/// 3. Verifies every name in [`EXPECTED_BINARIES`] landed in `staging/bin/`.
/// 4. Copies scripts from `scripts/` (if present) to `staging/bin/`.
/// 5. Classifies each crate's `config/*` files and copies them to either
///    `staging/share/umrs/` (JSON reference databases) or
///    `staging/share/umrs/templates/` (setrans.conf templates). FHS 2.3 §4.11.
/// 6. Copies pre-built troff man pages from `<crate>/docs/*.1` and
///    `<crate>/docs/fr/*.1` into `staging/share/man/` (see [`stage_man_pages`]).
/// 7. Copies compiled gettext catalogs from `<crate>/locale/<locale>/LC_MESSAGES/*.mo`
///    into `staging/share/locale/` (see [`stage_locales`]).
///
/// On `--release`, reads from `target/release/`; otherwise from `target/debug/`.
/// The build step is mandatory and cannot be skipped.
///
/// ## Compliance
///
/// `NIST SP 800-53 AC-3`, `CM-2`, `CM-7`, `CM-8`, `SA-12`, `SA-22`.
pub fn run(release: bool) -> Result<()> {
    let root = workspace_root();
    eprintln!("[stage] workspace root: {}", root.display());

    build_workspace(release)?;
    stage_binaries(&root, release)?;
    verify_staged_binaries(&root)?;
    stage_scripts(&root)?;
    stage_configs(&root)?;
    stage_man_pages(&root)?;
    stage_locales(&root)?;

    eprintln!("[stage] staging complete → {}/staging/", root.display());
    Ok(())
}
