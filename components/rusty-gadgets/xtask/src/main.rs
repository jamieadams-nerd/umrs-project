use anyhow::{Context, Result, bail};
use std::env;
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};

/// Scan all `src/**/*.rs` files in workspace crates for missing `//!` module
/// doc blocks. Flags files that have no `//!` line at all — the most common
/// class of documentation debt.
///
/// NIST SP 800-53 SA-11 / NIST SP 800-218 SSDF PW.4.
fn doc_check() -> Result<()> {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let mut missing = Vec::new();

    // Walk each workspace member's src/ directory
    for entry in fs::read_dir(workspace_root)? {
        let entry = entry?;
        let src_dir = entry.path().join("src");
        if !src_dir.is_dir() {
            continue;
        }
        check_dir_for_module_docs(&src_dir, &mut missing)?;
    }

    if missing.is_empty() {
        eprintln!("[doc-check] All .rs files have //! module documentation.");
        Ok(())
    } else {
        eprintln!(
            "[doc-check] {} file(s) missing //! module documentation:",
            missing.len()
        );
        for path in &missing {
            eprintln!("  {}", path.display());
        }
        bail!(
            "doc-check failed: {} file(s) without //! block",
            missing.len()
        );
    }
}

fn check_dir_for_module_docs(dir: &Path, missing: &mut Vec<std::path::PathBuf>) -> Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            check_dir_for_module_docs(&path, missing)?;
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            let content =
                fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?;
            if !content.lines().any(|line| line.starts_with("//!")) {
                missing.push(path);
            }
        }
    }
    Ok(())
}

fn run(cmd: &mut Command) -> Result<()> {
    eprintln!("[xtask] {:?}", cmd);
    let status = cmd
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .context("failed to spawn command")?;

    if !status.success() {
        bail!("command failed: {:?}", cmd);
    }
    Ok(())
}

fn main() -> Result<()> {
    let mut args = env::args().skip(1);
    let sub = args.next().unwrap_or_else(|| "help".to_string());

    match sub.as_str() {
        "fmt" => {
            run(Command::new("cargo").arg("fmt").arg("--all"))?;
        }
        "clippy" => {
            run(Command::new("cargo")
                .arg("clippy")
                .arg("--workspace")
                .arg("--all-targets")
                .arg("--")
                .arg("-D")
                .arg("warnings"))?;
        }
        "test" => {
            run(Command::new("cargo").arg("test").arg("--workspace"))?;
        }
        "robots" => {
            // Example: run your robot generator crate (adjust args as needed)
            run(Command::new("cargo").args(["run", "-p", "robotgen", "--"]))?;
        }
        "doc-check" => {
            doc_check()?;
        }
        "help" | "-h" | "--help" => {
            eprintln!(
                "Usage: cargo xtask <cmd>\n\
                 \n\
                 Commands:\n\
                 \tfmt\n\
                 \tclippy\n\
                 \ttest\n\
                 \tdoc-check\n\
                 \trobots\n"
            );
        }
        other => bail!("unknown xtask command: {other}"),
    }

    Ok(())
}
