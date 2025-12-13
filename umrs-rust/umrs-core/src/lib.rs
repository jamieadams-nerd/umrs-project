// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Jamie Adams
//
// Unclassified MLS Reference System Project
//
// Core library with lots of tasty functions to be used by
// opter tools
//
use std::path::Path;
use std::fs;
use std::io::{self, Read, Write};

use serde::{Deserialize, Serialize};

pub mod console;

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
        let data = serde_json::to_string_pretty(state)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        file.write_all(data.as_bytes())?;
        file.sync_all()?;
    }
    fs::rename(tmp_path, path)?;
    Ok(())
}

