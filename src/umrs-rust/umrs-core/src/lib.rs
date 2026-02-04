// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Jamie Adams
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
pub mod i18n;
pub mod metricfmt;
pub mod prelude;
pub mod sizefmt;
pub mod timed_result;
pub mod robots;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UmrsState {
    pub purpose: Option<String>,
    pub system_type: Option<String>,
    pub virtualization: Option<String>,
    pub fips_enabled: Option<bool>, // Example boolean key
                                    // Add more flat keys here as needed
}

// Load state from a file. Returns default if file is missing.
pub fn load_state(path: &Path) -> io::Result<UmrsState> {
    if !path.exists() {
        //let msg = format!("state file '{}' not found, using default state.", path.display());
        //eprintln!("Warning: {}", msg);
        return Ok(UmrsState::default());
    }

    let mut file = fs::File::open(path)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    let state: UmrsState = serde_json::from_str(&buf)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    Ok(state)
}

// Save state to a file atomically
pub fn save_state(path: &Path, state: &UmrsState) -> io::Result<()> {
    let tmp_path = path.with_extension("json.tmp");
    {
        let mut file = fs::File::create(&tmp_path)?;
        let data =
            serde_json::to_string_pretty(state).map_err(io::Error::other)?;
        file.write_all(data.as_bytes())?;
        file.sync_all()?;
    }
    fs::rename(tmp_path, path)?;
    Ok(())
}
