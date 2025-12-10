§use std::path::PathBuf;
§
§use clap::{Parser, Subcommand};
§use umrs-core::{UmrsState, SystemMetadata, load_state, save_state};
§
§#[derive(Parser, Debug)]
§#[command(name = “umrs-state”)]
§struct Cli {
§    #[arg(long = “state-file”)]
§    state_file: Option,
§
§    #[command(subcommand)]
§    command: Command,
§}
§
§#[derive(Subcommand, Debug)]
§enum Command {
§    Get {
§        key: String,
§    },
§    Set {
§        key: String,
§        value: String,
§    },
§}
§
§fn main() -> std::io::Result<()> {
§    let cli = Cli::parse();
§
§    let state_path = cli
§        .state_file
§        .unwrap_or_else(|| PathBuf::from(”/var/lib/umrs/umrs-state.json”));
§
§    let mut state = load_state(&state_path)?;
§
§    match cli.command {
§        Command::Get { key } => {
§            handle_get(&state, &key);
§        }
§        Command::Set { key, value } => {
§            handle_set(&mut state, &key, &value)?;
§            save_state(&state_path, &state)?;
§            log_manual_set(&key, &value);
§        }
§    }
§
§    Ok(())
§}
§
§fn handle_get(state: &UmrsState, key: &str) {
§    match key {
§        “system_metadata.purpose” => {
§            if let Some(p) = &state.system_metadata.purpose {
§                println!(”{p}”);
§            }
§        }
§        “system_metadata.system_type” => {
§            if let Some(t) = &state.system_metadata.system_type {
§                println!(”{t}”);
§            }
§        }
§        “system_metadata.virtualization” => {
§            if let Some(v) = &state.system_metadata.virtualization {
§                println!(”{v}”);
§            }
§        }
§        _ => {
§            eprintln!(“Unknown key: {key}”);
§        }
§    }
§}
§
§fn handle_set(state: &mut UmrsState, key: &str, value: &str) -> std::io::Result<()> {
§    match key {
§        “system_metadata.purpose” => {
§            state.system_metadata.purpose = Some(value.to_string());
§        }
§        “system_metadata.system_type” => {
§            state.system_metadata.system_type = Some(value.to_string());
§        }
§        “system_metadata.virtualization” => {
§            state.system_metadata.virtualization = Some(value.to_string());
§        }
§        _ => {
§            eprintln!(“Unknown key: {key}”);
§        }
§    }
§    Ok(())
§}
§
§fn log_manual_set(key: &str, value: &str) {
§    if let Err(e) = syslog::init_unix(
§        syslog::Facility::LOG_AUTH,
§        log::LevelFilter::Info,
§        Some(“umrs-state”),
§    ) {
§        eprintln!(“Failed to init syslog: {e}”);
§        return;
§    }
§
§    let msg = format!(“Manual umrs-state set: {key}={value}”);
§    if let Err(e) = syslog::log(syslog::Severity::LOG_INFO, &msg) {
§        eprintln!(“Failed to write syslog: {e}”);
§    }
§}
