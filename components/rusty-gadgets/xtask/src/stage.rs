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
//! ## Staging layout
//!
//! ```text
//! staging/
//!   bin/            ← compiled workspace binaries (expected set verified) +
//!                     end-user scripts
//!   config/         ← merged config files from all crates (subdirs preserved)
//!   share/
//!     man/
//!       man1/       ← English man pages compiled from <crate>/man/*.1.adoc
//!       fr_CA/
//!         man1/     ← Canadian French man pages from <crate>/man/fr_CA/*.1.adoc
//! ```
//!
//! ## Man page phase
//!
//! [`stage_man_pages`] runs as the final phase, after config staging.  It
//! delegates to `make man-pages` in the workspace root, which discovers all
//! `*/man/*.1.adoc` and `*/man/fr_CA/*.1.adoc` sources across crates and
//! compiles them with `asciidoctor -b manpage`.  If `asciidoctor` is absent
//! from `PATH`, the phase emits a warning and skips rather than aborting the
//! pipeline — man pages are a documentation artifact, not a build product.
//!
//! ## Compliance
//!
//! - `NIST SP 800-53 AC-3` — Access Enforcement: execute permission is
//!   verified before any binary or script is copied to `staging/bin/`.
//! - `NIST SP 800-53 CM-2` — Baseline Configuration: config file staging
//!   captures the authoritative configuration baseline for each crate.
//! - `NIST SP 800-53 CM-7` — Least Functionality: only well-typed artifact
//!   classes (executables, config files, man pages) are accepted; all other
//!   output artifacts are excluded by the filter.
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
        bail!(
            "target/{profile}/ not found after build — workspace may have no binary targets"
        );
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

/// Copies end-user scripts from `scripts/` (flat, non-recursive) into
/// `staging/bin/`.
///
/// - If `scripts/` does not exist, skips silently.
/// - Subdirectories inside `scripts/` are skipped silently (non-recursive).
/// - Files without the execute bit emit a warning and are NOT copied.
///
/// ## Compliance
///
/// `NIST SP 800-53 AC-3` — execute permission verified before copy.
/// `NIST SP 800-53 SA-12` — scripts enter the staging checkpoint alongside
/// compiled binaries, enabling uniform IMA-signing of the full `bin/` tree.
///
/// ## Contributor Note
///
/// Scripts MUST be committed to git with the execute bit set. When adding a
/// new script to the repository, use:
///
/// ```text
/// git add --chmod=+x scripts/<name>.sh
/// ```
///
/// Without this, the file will not have the execute bit in the repository and
/// the staging pipeline will emit a warning and skip it.
fn stage_scripts(workspace_root: &Path) -> Result<()> {
    let scripts_dir = workspace_root.join("scripts");

    if !scripts_dir.exists() {
        // Silent skip — scripts/ is optional.
        return Ok(());
    }

    let staging_bin = workspace_root.join("staging").join("bin");
    fs::create_dir_all(&staging_bin)
        .with_context(|| format!("creating {}", staging_bin.display()))?;

    let entries =
        fs::read_dir(&scripts_dir).with_context(|| format!("reading {}", scripts_dir.display()))?;

    let mut count = 0usize;
    for entry in entries {
        let entry = entry.with_context(|| format!("reading entry in {}", scripts_dir.display()))?;
        let path = entry.path();
        let meta = path.metadata().with_context(|| format!("stat {}", path.display()))?;

        // Non-recursive: skip subdirectories silently.
        if !meta.is_file() {
            continue;
        }

        let filename = match path.file_name() {
            Some(n) => n,
            None => continue,
        };

        // Verify execute bit — warn and skip if absent.
        // NIST SP 800-53 AC-3: enforce execute permission at staging boundary.
        let mode = meta.permissions().mode();
        if mode & 0o111 == 0 {
            eprintln!(
                "WARNING: scripts/{} is not executable — skipping",
                filename.to_string_lossy()
            );
            continue;
        }

        let dest = staging_bin.join(filename);
        fs::copy(&path, &dest)
            .with_context(|| format!("copying {} to {}", path.display(), dest.display()))?;
        count = count.saturating_add(1);
        eprintln!("[stage] script: {}", filename.to_string_lossy());
    }

    eprintln!("[stage] staged {count} script(s) from scripts/");
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

/// Recursively copies a source directory tree into a destination directory,
/// preserving subdirectory structure.
fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<()> {
    fs::create_dir_all(dst).with_context(|| format!("creating {}", dst.display()))?;

    for entry in fs::read_dir(src).with_context(|| format!("reading {}", src.display()))? {
        let entry = entry.with_context(|| format!("reading entry in {}", src.display()))?;
        let src_path = entry.path();
        let meta = entry.metadata().with_context(|| format!("stat {}", src_path.display()))?;

        let file_name = entry.file_name();
        let dst_path = dst.join(&file_name);

        if meta.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else if meta.is_file() {
            fs::copy(&src_path, &dst_path).with_context(|| {
                format!("copying {} to {}", src_path.display(), dst_path.display())
            })?;
            eprintln!("[stage] config: {}", dst_path.display());
        }
    }

    Ok(())
}

/// Copies `config/` directories from each listed crate into `staging/config/`,
/// preserving subdirectory structure without crate-name prefixing.
///
/// If a crate has no `config/` directory, it is skipped silently.
///
/// ## Collision policy
///
/// No filename collisions are expected across crates. If two crates contain a
/// file at the same relative path, the second write will overwrite the first.
/// This is documented here so that if a collision is introduced in future, the
/// symptom (silent overwrite) is understood and not mistaken for a bug.
///
/// ## Compliance
///
/// `NIST SP 800-53 CM-2` — staging captures the full configuration baseline.
fn stage_configs(workspace_root: &Path) -> Result<()> {
    let staging_config = workspace_root.join("staging").join("config");
    let mut any = false;

    for &crate_rel in CONFIG_CRATES {
        let config_dir = workspace_root.join(crate_rel).join("config");
        if !config_dir.exists() {
            continue;
        }

        any = true;
        fs::create_dir_all(&staging_config)
            .with_context(|| format!("creating {}", staging_config.display()))?;

        copy_dir_recursive(&config_dir, &staging_config)?;
        eprintln!("[stage] config from {crate_rel}/config/");
    }

    if any {
        eprintln!(
            "[stage] config staging complete → {}",
            staging_config.display()
        );
    } else {
        eprintln!("[stage] no config/ directories found in workspace crates");
    }

    Ok(())
}

/// Compiles AsciiDoc man page sources across all crates into
/// `staging/share/man/`.
///
/// Delegates to `make man-pages` in the workspace root. That target discovers
/// all `*/man/*.1.adoc` (English) and `*/man/fr_CA/*.1.adoc` (Canadian French)
/// sources and compiles each with `asciidoctor -b manpage`.
///
/// ## Soft failure policy
///
/// If `asciidoctor` is not installed, `make man-pages` exits with code 3 and
/// emits a clear diagnostic. This function treats that outcome as a warning
/// rather than a hard failure: the binary and config artifacts are more
/// critical than documentation at the build boundary, and an absent
/// `asciidoctor` on a developer workstation should not block a staging run.
///
/// CI and pre-release builds must install `asciidoctor` before staging.
/// Missing man pages in a release bundle are a compliance gap under
/// `NIST SP 800-53 SA-22`.
///
/// ## Compliance
///
/// - `NIST SP 800-53 SA-22` — Unsupported System Components: man pages staged
///   here are the operator-facing documentation artifact for each installed
///   binary.
/// - `NIST SP 800-53 CM-2` — Baseline Configuration: man pages are part of
///   the auditable deployment artifact set produced before IMA signing.
fn stage_man_pages(workspace_root: &Path) -> Result<()> {
    let mut cmd = Command::new("make");
    cmd.arg("man-pages").current_dir(workspace_root);

    eprintln!("[stage] building man pages via make man-pages");

    // Use a spawned child so we can inspect the exit code and apply the
    // soft-failure policy without crate::run_cmd()'s hard bail!().
    let status = cmd
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .status();

    match status {
        Ok(s) if s.success() => {
            eprintln!("[stage] man pages staged → staging/share/man/");
            Ok(())
        }
        Ok(s) => {
            // Exit code 3 from make man-pages means asciidoctor is absent.
            // Any other non-zero code is a real build failure.
            let code = s.code().unwrap_or(1);
            if code == 3 {
                eprintln!(
                    "[stage] WARNING: asciidoctor not found — man pages were not compiled. \
                     Install rubygem-asciidoctor (RHEL 10) or asciidoctor (Ubuntu) \
                     before a release build. Missing man pages violate NIST SP 800-53 SA-22."
                );
                Ok(())
            } else {
                anyhow::bail!(
                    "make man-pages failed with exit code {code} — \
                     fix asciidoctor errors before staging"
                );
            }
        }
        Err(e) => {
            // make itself was not found — emit a warning and continue.
            eprintln!(
                "[stage] WARNING: could not invoke make ({e}) — \
                 man pages skipped. Ensure make is installed."
            );
            Ok(())
        }
    }
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
        if release { "release" } else { "debug" }
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
/// 5. Copies `config/` trees from all listed crates to `staging/config/`.
/// 6. Compiles AsciiDoc man pages to `staging/share/man/` (soft failure if
///    `asciidoctor` is absent — see [`stage_man_pages`]).
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

    eprintln!("[stage] staging complete → {}/staging/", root.display());
    Ok(())
}
