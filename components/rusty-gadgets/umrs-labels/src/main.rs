use anyhow::{Context, Result};
use clap::Parser;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
struct SensitivityLevel {
    name: String,
    description: String,
    handling: String,
}

#[derive(Debug, Deserialize)]
struct Category {
    abbreviation: String,
    description: String,
    handling: String,
}

#[derive(Debug, Deserialize)]
struct UmrsMetadata {
    sensitivity_levels: Vec<SensitivityLevel>,
    categories: Vec<Category>,
}

#[derive(Parser, Debug)]
#[command(version, about = "UMRS Sensitivity & Category Lookup Utility")]
struct Args {
    /// Path to the JSON metadata file
    #[arg(short, long)]
    metadata: PathBuf,

    /// Query a sensitivity level (example: S4)
    #[arg(short = 's', long)]
    sensitivity: Option<String>,

    /// Query a category abbreviation (example: PRIV_GEN or EXPORT)
    #[arg(short = 'c', long)]
    category: Option<String>,

    /// List all known metadata
    #[arg(short = 'l', long)]
    list: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Load JSON file
    let data = fs::read_to_string(&args.metadata)
        .with_context(|| format!("Failed to read {:?}", args.metadata))?;

    let meta: UmrsMetadata = serde_json::from_str(&data)
        .with_context(|| format!("Failed to parse JSON from {:?}", args.metadata))?;

    // List mode
    if args.list {
        println!("Known Sensitivity Levels:");
        for s in &meta.sensitivity_levels {
            println!("  {} - {}", s.name, s.description);
        }

        println!();
        println!("Known Categories:");
        for c in &meta.categories {
            println!("  {} - {}", c.abbreviation, c.description);
        }
        return Ok(());
    }

    // Sensitivity lookup
    if let Some(squery) = args.sensitivity.as_deref() {
        for s in &meta.sensitivity_levels {
            if s.name.eq_ignore_ascii_case(squery) {
                println!("Sensitivity Level: {}", s.name);
                println!("Description: {}", s.description);
                println!("Handling: {}", s.handling);
                return Ok(());
            }
        }
        println!("No sensitivity level '{}' found.", squery);
    }

    // Category lookup
    if let Some(cquery) = args.category.as_deref() {
        for c in &meta.categories {
            if c.abbreviation.eq_ignore_ascii_case(cquery) {
                println!("Category: {}", c.abbreviation);
                println!("Description: {}", c.description);
                println!("Handling: {}", c.handling);
                return Ok(());
            }
        }
        println!("No category '{}' found.", cquery);
    }

    Ok(())
}
