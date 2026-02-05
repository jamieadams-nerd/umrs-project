// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Jamie Adams
//
// Unclassified MLS Reference System Project
//
// A simple tool maintain a global state file for the system.
// Thie state file has things like FIPS mode, operating state,
// and such. It's intended to easily report on system posture.
//
// Probes will run at some schedules and then update keys in
// the state file. For example, after AIDE runs, a wrapper script
// will update the state file to let us know how it went.
//
// The tools has the ability to set/get keys. You can also list-keys
//
use std::path::{PathBuf, Path};

use clap::{Parser, Subcommand};
use umrs_core::{UmrsState, load_state, save_state, console};

#[derive(Parser, Debug)]
#[command(name = "umrs-state")]
struct Cli {
    #[arg(long = "state-file")]
    state_file: Option<PathBuf>,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Get { key: String },
    Set { key: String, value: String },
    ListKeys,
}

// Enum representing all available keys
#[derive(Debug, Clone, Copy)]
enum UmrsKey {
    Purpose,
    SystemType,
    Virtualization,
    FipsEnabled, // example boolean key
}

impl UmrsKey {
    fn as_str(&self) -> &'static str {
        match self {
            UmrsKey::Purpose => "purpose",
            UmrsKey::SystemType => "system_type",
            UmrsKey::Virtualization => "virtualization",
            UmrsKey::FipsEnabled => "fips_enabled",
        }
    }

    fn all() -> &'static [UmrsKey] {
        &[UmrsKey::Purpose, UmrsKey::SystemType, UmrsKey::Virtualization, UmrsKey::FipsEnabled]
    }

    fn parse(key: &str) -> Option<UmrsKey> {
        Self::all().iter().copied().find(|k| k.as_str() == key)
    }
}

fn main() -> std::io::Result<()> {
    init_logging();
    console::init();
    console::info("Starting umrs-state");
    // Status is simple to give a message, and then true or false. The console
    // will print "Ok" or "Fail" next to it. This is not tied to keys.
    console::status("FIPS is Enabled", true);
    console::status("System Purpose defined in state file", false);

    let cli = Cli::parse();

    let state_path = cli
        .state_file
        .unwrap_or_else(|| PathBuf::from("/var/lib/umrs/umrs-state.json"));

    // Load or default state based on command
    let mut state = match &cli.command {
        Command::ListKeys => UmrsState::default(), // no warnings
        Command::Get { .. } => {
            check_state_file(&state_path, false);
            load_state(&state_path)?
        }
        Command::Set { .. } => {
            check_state_file(&state_path, true);
            load_state(&state_path)?
        }
    };

    match cli.command {
        Command::Get { key } => handle_get(&state, &key),
        Command::Set { key, value } => {
            handle_set(&mut state, &key, &value)?;
            save_state(&state_path, &state)?;
            log_manual_set(&key, &value);
        }
        Command::ListKeys => list_keys(),
    }

    Ok(())
}

// Initialize syslog logging
fn init_logging() {
    if let Err(e) = syslog::init_unix(syslog::Facility::LOG_AUTH, log::LevelFilter::Info) {
        eprintln!("Failed to init syslog: {e}");
    }
}

// Print all available keys
fn list_keys() {
    for key in UmrsKey::all() {
        println!("{}", key.as_str());
    }
}

fn check_state_file(path: &Path, creating: bool) {
    if !path.exists() {
        if creating {
            eprintln!(
                "State file '{}' does not exist. It will be created.",
                path.display()
            );
        } else {
            eprintln!(
                "Warning: state file '{}' does not exist. Using default state.",
                path.display()
            );
        }
    }
}

// Handle `get` command
fn handle_get(state: &UmrsState, key: &str) {
    match UmrsKey::parse(key) {
        Some(UmrsKey::Purpose) => {
            match &state.purpose {
                Some(p) => println!("{p}"),
                None => println!("(not set)"),
            }
        }
        Some(UmrsKey::SystemType) => {
            match &state.system_type {
                Some(t) => println!("{t}"),
                None => println!("(not set)"),
            }
        }
        Some(UmrsKey::Virtualization) => {
            match &state.virtualization {
                Some(v) => println!("{v}"),
                None => println!("(not set)"),
            }
        }
        Some(UmrsKey::FipsEnabled) => {
            match state.fips_enabled {
                Some(b) => println!("{b}"),
                None => println!("(not set)"),
            }
        }
        None => eprintln!("Unknown key: {key}"),
    }
}

// Handle `set` command
fn handle_set(state: &mut UmrsState, key: &str, value: &str) -> std::io::Result<()> {
    match UmrsKey::parse(key) {
        Some(UmrsKey::Purpose) => state.purpose = Some(value.to_string()),
        Some(UmrsKey::SystemType) => state.system_type = Some(value.to_string()),
        Some(UmrsKey::Virtualization) => state.virtualization = Some(value.to_string()),
        Some(UmrsKey::FipsEnabled) => {
            let b = match value.to_lowercase().as_str() {
                "true" | "1" => true,
                "false" | "0" => false,
                _ => {
                    eprintln!("Invalid boolean value '{value}', using false.");
                    false
                }
            };
            state.fips_enabled = Some(b);
        }
        None => eprintln!("Unknown key: {key}"),
    }
    Ok(())
}

// Log manual set operations
fn log_manual_set(key: &str, value: &str) {
    let msg = format!("Manual umrs-state set: {key}={value}");
    log::info!("{}", msg);
}

