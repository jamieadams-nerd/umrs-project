// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Jamie Adams
//
// --
// SPDX-FileContributor: Name <email> (useful if you want contributors 
// credited in-file; not required, and many projects avoid it to reduce churn)
// --
//
// Unclassified MLS Reference System Project (UMRS)
// MIT licensed—use, modify, and redistribute per LICENSE.
//
// umrs-core: 
// Purpose: <one-sentence purpose of this file/module>
// Notes:
//   - <key design constraint or security property>
//   - <any invariants / assumptions / non-goals>
//   - <where to look next: related modules or docs>
//

use std::path::Path;
use std::fs;
use std::io::{self, Read, Write};

use serde::{Deserialize, Serialize};

// Local modules
pub mod console;
pub mod ansi;
pub mod spinner;
pub mod i18n;


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

