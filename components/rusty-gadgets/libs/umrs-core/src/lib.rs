// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Jamie Adams
//
// NIST SP 800-218 SSDF PW.4 / NSA RTB: Provable safe-code guarantee.
// #![forbid] cannot be overridden by any inner #[allow] — this is a
// compile-time proof, not a policy. An auditor can verify it mechanically.
#![forbid(unsafe_code)]
//
//! UMRS Core Library
//!
//! Foundational primitives and shared infrastructure for all UMRS tools.
//!
//! Guarantees:
//! - Stable, versioned APIs for cross-tool reuse
//! - Deterministic behavior across supported platforms
//! - Security-first design aligned with MLS and FIPS constraints
//!
//! Non-goals:
//! - End-user CLI interfaces
//! - Tool-specific business logic
//

use std::fs;
use std::io::{self, Read, Write};
use std::path::Path;

use serde::{Deserialize, Serialize};

// Local modules
pub mod audit;
pub mod console;
pub mod human;
pub mod i18n;
pub mod prelude;
pub mod robots;
pub mod timed_result;
pub mod validate;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UmrsState {
    pub purpose: Option<String>,
    pub system_type: Option<String>,
    pub virtualization: Option<String>,
    pub fips_enabled: Option<bool>, // Example boolean key
                                    // Add more flat keys here as needed
}

/// Load state from a file. Returns default if file is missing.
#[must_use = "state load result must be checked — a silently ignored error means security posture data was not loaded"]
pub fn load_state(path: &Path) -> io::Result<UmrsState> {
    let mut file = match fs::File::open(path) {
        Ok(f) => f,
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            return Ok(UmrsState::default());
        }
        Err(e) => return Err(e),
    };
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    let state: UmrsState =
        serde_json::from_str(&buf).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    Ok(state)
}

/// Save state to a file atomically
#[must_use = "state save result must be checked — a silently discarded error means security posture data was not persisted"]
pub fn save_state(path: &Path, state: &UmrsState) -> io::Result<()> {
    let tmp_path = path.with_extension("json.tmp");
    {
        let mut file = fs::File::create(&tmp_path)?;
        let data = serde_json::to_string_pretty(state).map_err(io::Error::other)?;
        file.write_all(data.as_bytes())?;
        file.sync_all()?;
    }
    fs::rename(tmp_path, path)?;
    Ok(())
}
