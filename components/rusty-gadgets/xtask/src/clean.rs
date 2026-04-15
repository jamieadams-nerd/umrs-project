//! Clean command — removes the `staging/` directory tree.
//!
//! This is the inverse of the staging pipeline: it deletes everything in
//! `staging/` so that the next `cargo xtask stage` run starts from a known
//! empty state.
//!
//! ## Compliance
//!
//! - `NIST SP 800-53 CM-2` — Baseline Configuration: the clean command
//!   provides a reproducible starting point before re-staging.
//! - `NIST SP 800-53 CM-7` — Least Functionality: removing stale artifacts
//!   prevents leftover binaries from prior builds from polluting a fresh
//!   staging pass.

use anyhow::{Context, Result};
use std::fs;

use crate::stage::workspace_root;

/// Removes the `staging/` directory and all its contents.
///
/// If `staging/` does not exist, the operation succeeds silently — the
/// postcondition (no staging directory) is already satisfied.
///
/// ## Compliance
///
/// `NIST SP 800-53 CM-2`, `NIST SP 800-53 CM-7`.
pub fn run() -> Result<()> {
    let root = workspace_root();
    let staging = root.join("staging");

    if !staging.exists() {
        eprintln!("[clean] staging/ does not exist — nothing to remove");
        return Ok(());
    }

    fs::remove_dir_all(&staging).with_context(|| format!("removing {}", staging.display()))?;

    eprintln!("[clean] removed {}", staging.display());
    Ok(())
}
