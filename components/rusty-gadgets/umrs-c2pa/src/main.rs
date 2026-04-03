// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Jamie Adams
#![forbid(unsafe_code)]

//! # UMRS C2PA — Binary Entry Point
//!
//! CLI entry point for the `umrs-c2pa` tool. Initializes structured audit
//! logging via journald (with a graceful fallback to stderr if journald is
//! unavailable), loads configuration from TOML, and dispatches to the
//! appropriate subcommand handler.
//!
//! ## Key Behaviors
//!
//! - Journald logging initialized before any security-relevant operation.
//! - Config loaded from `--config` path; defaults used if absent.
//! - All signing operations delegated to `c2pa::ingest_file`.
//! - Private key files created with mode 0600 from the start.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AU-3**: Audit Record Content — journald initialization
//!   ensures all security-relevant events are captured with structured context.
//! - **NIST SP 800-53 AU-5**: Response to Audit Processing Failures — if
//!   journald is unavailable, the binary falls back to stderr logging rather
//!   than aborting, so audit output is never silently lost.
//! - **NIST SP 800-53 CM-6**: Configuration Settings — config is loaded and
//!   validated before any operation proceeds.
//! - **NIST SP 800-53 SC-12**: Cryptographic Key Management — private key
//!   files are created with mode 0600 atomically at file creation time, not
//!   as a post-write chmod, eliminating the race window.
//! - **NIST SP 800-53 SC-13**: Cryptographic Protection — all signing
//!   operations are delegated through the FIPS-gated signing path.

use umrs_c2pa::c2pa;
use umrs_c2pa::verbose;

use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use gettextrs::ngettext;
use umrs_core::i18n;

use c2pa::{
    UmrsConfig,
    ingest::{sha256_hex, sha384_hex},
    manifest::{chain_report_json, read_chain},
    manifest_json,
    report::{print_chain, print_chain_readonly, print_validation_report},
    validate::validate_config,
};

/// UMRS media inspection and C2PA signing tool.
///
/// Inspect a file:   umrs-c2pa photo.jpg
/// Sign a file:      umrs-c2pa --sign photo.jpg
/// Manage creds:     umrs-c2pa creds generate --output ./certs
/// Manage config:    umrs-c2pa config validate
#[derive(Parser)]
#[command(name = "umrs-c2pa", version, about, long_about = None)]
#[expect(clippy::struct_excessive_bools, reason = "CLI flags — not a state machine")]
struct Cli {
    /// Path to UMRS configuration file.
    #[arg(long, global = true, default_value = "umrs-c2pa.toml")]
    config: PathBuf,

    /// Show step-by-step progress on stderr.
    #[arg(long, short, global = true)]
    verbose: bool,

    // ── default action: inspect/sign a file ──────────────────────────────
    /// Media file to inspect or sign.
    #[arg(value_name = "FILE")]
    file: Option<PathBuf>,

    /// Sign (ingest) the file and record a UMRS chain-of-custody entry.
    #[arg(long)]
    sign: bool,

    /// Emit the full manifest store as JSON instead of the formatted report.
    #[arg(long)]
    json: bool,

    /// Emit the UMRS-parsed evidence chain as JSON.
    #[arg(long)]
    chain_json: bool,

    /// Security marking to embed in the manifest (e.g. "CUI" or "CUI//SP-CTI//NOFORN").
    /// Only applies when --sign is used.
    #[arg(long)]
    marking: Option<String>,

    /// Write the signed output to this path (default: <file>_`umrs_signed`.<ext>).
    #[arg(long)]
    output: Option<PathBuf>,

    // ── subcommands for non-file operations ──────────────────────────────
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Configuration management.
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },

    /// Signing credential management (certificates and keys).
    Creds {
        #[command(subcommand)]
        action: CredsAction,
    },
}

#[derive(Subcommand)]
enum ConfigAction {
    /// Validate the configuration file and signing credentials.
    Validate,

    /// Generate a commented starter configuration file.
    Generate {
        /// Write the generated config to this path (default: stdout).
        #[arg(long)]
        output: Option<PathBuf>,
    },
}

#[derive(Subcommand)]
enum CredsAction {
    /// Generate a new signing certificate and private key.
    ///
    /// Creates a self-signed certificate by default.
    /// Use --csr to generate a Certificate Signing Request for your CA.
    ///
    /// After generating, set the paths in umrs-c2pa.toml:
    ///   [identity]
    ///   `cert_chain`  = "/path/to/signing.pem"
    ///   `private_key` = "/path/to/signing.key"
    Generate {
        /// Directory to write signing.pem and signing.key.
        #[arg(long, default_value = ".")]
        output: PathBuf,

        /// Generate a CSR instead of a self-signed certificate.
        #[arg(long)]
        csr: bool,

        /// Certificate validity in days (ignored with --csr). Default: 365.
        #[arg(long, default_value = "365")]
        days: u32,
    },

    /// Validate the configured signing credentials.
    ///
    /// Checks that cert and key files exist, are valid PEM, match each other,
    /// and reports certificate details (subject, issuer, validity, algorithm).
    Validate,
}

fn main() -> Result<()> {
    // Initialize the i18n subsystem before any user-facing output.
    // This binds the "umrs-c2pa" gettext domain so that tr() calls resolve
    // against the correct message catalog for the active system locale.
    i18n::init("umrs-c2pa");

    let cli = Cli::parse();

    // Enable verbose console output if requested.
    if cli.verbose {
        c2pa::enable_verbose();
    }

    // Load config — fall back to defaults if the file doesn't exist.
    let config = if cli.config.exists() {
        verbose!("Loading config from {}", cli.config.display());
        UmrsConfig::load(&cli.config).with_context(|| {
            format!(
                "{} {}",
                i18n::tr("Failed to load config:"),
                cli.config.display()
            )
        })?
    } else {
        verbose!(
            "{} {}",
            i18n::tr("No config file at"),
            format!("{} — {}", cli.config.display(), i18n::tr("using defaults"))
        );
        UmrsConfig::default()
    };

    // Initialize logging — journald with graceful fallback to stderr.
    // If journald is unavailable (container, chroot, minimal environment),
    // we fall back to env_logger rather than panicking. This satisfies
    // NIST SP 800-53 AU-5 — response to audit processing failures must not
    // silently abort the tool; instead, degrade gracefully and log a warning.
    let level_filter = config.log_level_filter();
    match systemd_journal_logger::JournalLog::new() {
        Ok(journal_log) => {
            match journal_log.with_syslog_identifier("umrs".to_string()).install() {
                Ok(()) => {}
                Err(e) => {
                    // journald connected but logger install failed — use stderr.
                    eprintln!("umrs-c2pa: journald logger install failed ({e}), using stderr");
                    env_logger::Builder::new().filter_level(level_filter).init();
                    log::warn!(target: "umrs", "journald unavailable — logging to stderr");
                }
            }
        }
        Err(e) => {
            // journald not available — fall back to stderr logging.
            eprintln!("umrs-c2pa: journald unavailable ({e}), falling back to stderr logging");
            env_logger::Builder::new().filter_level(level_filter).init();
        }
    }
    log::set_max_level(level_filter);

    // Dispatch: subcommand takes priority, otherwise inspect a file.
    match cli.command {
        Some(Commands::Config {
            action,
        }) => match action {
            ConfigAction::Validate => {
                cmd_config_validate(&config);
            }
            ConfigAction::Generate {
                output,
            } => {
                cmd_config_generate(output.as_deref())?;
            }
        },
        Some(Commands::Creds {
            action,
        }) => match action {
            CredsAction::Generate {
                output,
                csr,
                days,
            } => {
                cmd_creds_generate(&config, &output, csr, days)?;
            }
            CredsAction::Validate => {
                cmd_creds_validate(&config);
            }
        },
        None => {
            // Default action: inspect or sign a file.
            let Some(file) = cli.file else {
                // No file and no subcommand — show help.
                use clap::CommandFactory;
                Cli::command().print_help()?;
                println!();
                return Ok(());
            };
            cmd_c2pa(
                &file,
                cli.sign,
                cli.json,
                cli.chain_json,
                cli.marking.as_deref(),
                cli.output.as_deref(),
                &config,
            )?;
        }
    }

    Ok(())
}

// ── inspect / sign a file ────────────────────────────────────────────────────

fn cmd_c2pa(
    file: &std::path::Path,
    sign: bool,
    json: bool,
    chain_json: bool,
    marking: Option<&str>,
    output: Option<&std::path::Path>,
    config: &UmrsConfig,
) -> Result<()> {
    if !file.exists() {
        anyhow::bail!("{} {}", i18n::tr("File not found:"), file.display());
    }

    // Marking without signing is an error — markings are embedded during signing.
    if marking.is_some() && !sign {
        anyhow::bail!(
            "--marking requires --sign. Markings are embedded during signing, not during inspection."
        );
    }

    verbose!("Found file: {}", file.display());

    // JSON mode — emit raw manifest store and exit.
    if json {
        verbose!("{}", i18n::tr("Reading raw manifest store as JSON..."));
        match manifest_json(file, config) {
            Ok(j) => println!("{j}"),
            Err(e) => eprintln!("{} {e}", i18n::tr("No manifest or read error:")),
        }
        return Ok(());
    }

    // Chain JSON mode — emit parsed evidence chain with integrity hashes as JSON and exit.
    if chain_json {
        verbose!("{}", i18n::tr("Computing SHA-256 and SHA-384 digests..."));
        let sha256 = sha256_hex(file)
            .with_context(|| format!("{} {}", i18n::tr("Failed to hash file:"), file.display()))?;
        let sha384 = sha384_hex(file)
            .with_context(|| format!("{} {}", i18n::tr("Failed to hash file:"), file.display()))?;
        verbose!("{}", i18n::tr("Reading chain of custody as JSON..."));
        match chain_report_json(file, &sha256, &sha384, config) {
            Ok(j) => println!("{j}"),
            Err(e) => eprintln!("{} {e}", i18n::tr("No manifest or read error:")),
        }
        return Ok(());
    }

    verbose!("{}", i18n::tr("Computing SHA-256 and SHA-384 digests..."));
    let sha256 = sha256_hex(file)
        .with_context(|| format!("{} {}", i18n::tr("Failed to hash file:"), file.display()))?;
    let sha384 = sha384_hex(file)
        .with_context(|| format!("{} {}", i18n::tr("Failed to hash file:"), file.display()))?;
    verbose!("{}: {}", i18n::tr("SHA-256"), sha256);
    verbose!("{}: {}", i18n::tr("SHA-384"), sha384);

    if sign {
        verbose!(
            "{}",
            i18n::tr("Signing mode — ingesting file into UMRS chain of custody...")
        );
        if let Some(m) = marking {
            verbose!("{} {}", i18n::tr("Security marking:"), m);
        }

        // Ingest mode: sign the file, display the resulting chain.
        let result = c2pa::ingest_file(file, output, marking, config)
            .with_context(|| format!("{} {}", i18n::tr("Ingest failed for:"), file.display()))?;
        verbose!(
            "{} {}",
            i18n::tr("Signed output written to:"),
            result.output_path.display()
        );

        verbose!(
            "{}",
            i18n::tr("Reading chain of custody from signed output...")
        );
        let chain = read_chain(&result.output_path, config)
            .with_context(|| i18n::tr("Failed to read chain from signed output"))?;
        let n = chain.len();
        verbose!(
            "{}",
            ngettext(
                "Chain contains {} entry",
                "Chain contains {} entries",
                u32::try_from(n).unwrap_or(u32::MAX)
            )
            .replace("{}", &n.to_string())
        );

        print_chain(
            &file.display().to_string(),
            &sha256,
            &sha384,
            &chain,
            Some(&result),
        );
    } else {
        verbose!(
            "{}",
            i18n::tr("Read-only mode — inspecting existing chain of custody...")
        );

        // Read-only mode: display the chain as-is.
        let chain = read_chain(file, config).with_context(|| {
            format!(
                "{} {}",
                i18n::tr("Failed to read chain from:"),
                file.display()
            )
        })?;
        let n = chain.len();
        verbose!(
            "{}",
            ngettext(
                "Chain contains {} entry",
                "Chain contains {} entries",
                u32::try_from(n).unwrap_or(u32::MAX)
            )
            .replace("{}", &n.to_string())
        );

        print_chain_readonly(&file.display().to_string(), &sha256, &sha384, &chain);
    }

    Ok(())
}

// ── config validate ──────────────────────────────────────────────────────────

fn cmd_config_validate(config: &UmrsConfig) {
    verbose!("{}", i18n::tr("Running configuration preflight checks..."));
    let results = validate_config(config);
    let n = results.len();
    verbose!(
        "{}",
        ngettext(
            "{} check completed",
            "{} checks completed",
            u32::try_from(n).unwrap_or(u32::MAX)
        )
        .replace("{}", &n.to_string())
    );
    print_validation_report(&results);

    let failures = results.iter().filter(|r| r.status == c2pa::validate::CheckStatus::Fail).count();

    if failures > 0 {
        std::process::exit(1);
    }
}

// ── config generate ──────────────────────────────────────────────────────────

fn cmd_config_generate(output: Option<&std::path::Path>) -> Result<()> {
    let template = config_template();
    match output {
        Some(path) => {
            std::fs::write(path, template).with_context(|| {
                format!(
                    "{} {}",
                    i18n::tr("Failed to write config to:"),
                    path.display()
                )
            })?;
            println!(
                "{} {}",
                i18n::tr("Config template written to:"),
                path.display()
            );
        }
        None => print!("{template}"),
    }
    Ok(())
}

const fn config_template() -> &'static str {
    r#"# umrs-c2pa.toml — UMRS C2PA signing configuration
#
# Quick start:
#   1. umrs-c2pa creds generate --output ./certs    # create cert + key
#   2. Edit this file — set cert_chain and private_key paths
#   3. umrs-c2pa config validate                    # verify everything
#   4. umrs-c2pa --sign photo.jpg                   # sign your first file
#
# Run `umrs-c2pa config validate` to verify before use.
# Run `umrs-c2pa config generate --output <path>` to regenerate this template.

[identity]
# Human-readable name embedded in every manifest produced by this system.
claim_generator = "UMRS Reference System/1.0"

# Organization name for display in chain-of-custody reports.
organization = "Your Organization"

# Path to PEM-encoded signing certificate chain (leaf cert first, root last).
# Generate with: umrs-c2pa creds generate --output ./certs
# If omitted, an ephemeral self-signed cert is generated at runtime (test mode).
#cert_chain = "./certs/signing.pem"

# Path to PEM-encoded private key corresponding to the leaf certificate.
#private_key = "./certs/signing.key"

# Signing algorithm. Must be in the FIPS-safe set.
# Allowed : es256 | es384 | es512 | ps256 | ps384 | ps512
# Excluded: ed25519 — unreliable on FIPS-enabled RHEL
# Strongest FIPS+C2PA intersection: es512
algorithm = "es256"

[timestamp]
# Time Stamp Authority URL for trusted signing timestamps.
# Omit (or comment out) to sign without a TSA timestamp.
# tsa_url = "http://timestamp.digicert.com"

[policy]
# Action label and reason for files arriving WITHOUT an existing C2PA manifest.
# c2pa.acquired = "we received this; we are not the creator"
unsigned_action = "c2pa.acquired"
unsigned_reason = "Received at UMRS trusted ingest dropbox. Origin unknown. No modifications made."

# Action label and reason for files arriving WITH an existing C2PA manifest.
# c2pa.published = "we forwarded this as-is"
signed_action = "c2pa.published"
signed_reason = "Received at UMRS trusted ingest dropbox with existing provenance. No modifications made."

[trust]
# Trust list configuration for C2PA signature validation.
# All paths are configurable — no hardcoded default location.
# See docs/trust-maintenance.md for setup and update procedures.
#
# UMRS combines trust_anchors + user_anchors into a single PEM bundle.
# Use user_anchors for the TSA trust list so both files update independently.
#
# Download both from:
#   https://raw.githubusercontent.com/c2pa-org/conformance-public/refs/heads/main/trust-list/C2PA-TRUST-LIST.pem
#   https://raw.githubusercontent.com/c2pa-org/conformance-public/refs/heads/main/trust-list/C2PA-TSA-TRUST-LIST.pem

# Path to PEM bundle of C2PA signing root CA certificates.
#trust_anchors = "/path/to/trust/C2PA-TRUST-LIST.pem"

# Path to PEM bundle of TSA root CA certificates (combined with trust_anchors).
# Also accepts organization root CAs — any PEM bundle works here.
#user_anchors = "/path/to/trust/C2PA-TSA-TRUST-LIST.pem"

# Path to end-entity certificate allowlist (optional).
# Directly trust specific signer certs without chain validation.
#allowed_list = "/path/to/trust/allowed-signers.pem"

# Path to EKU OID filter file (optional). One OID per line, // comments.
#trust_config = "/path/to/trust/ekus.cfg"

# Enable trust validation (default: true).
verify_trust = true

# OCSP responder URL (skeleton — not fully implemented yet).
# Organizations can point this to their own OCSP server when ready.
#ocsp_responder = "http://ocsp.internal.example.com"

[logging]
# Enable or disable all logging output.
enabled = true

# Minimum log level: off | error | warn | info | debug | trace
# Set to "off" in production if journald volume is a concern.
level = "info"
"#
}

// ── creds generate ───────────────────────────────────────────────────────────

fn cmd_creds_generate(
    config: &UmrsConfig,
    output_dir: &std::path::Path,
    csr: bool,
    days: u32,
) -> Result<()> {
    verbose!(
        "{} {}",
        i18n::tr("Generating credentials in:"),
        output_dir.display()
    );

    // Create output directory if it doesn't exist.
    if !output_dir.exists() {
        std::fs::create_dir_all(output_dir).with_context(|| {
            format!(
                "{} {}",
                i18n::tr("Failed to create directory:"),
                output_dir.display()
            )
        })?;
    }

    let result = c2pa::creds::generate(config, csr, days)
        .with_context(|| i18n::tr("Credential generation failed"))?;

    let cert_name = if result.is_csr {
        "signing.csr"
    } else {
        "signing.pem"
    };
    let cert_path = output_dir.join(cert_name);
    let key_path = output_dir.join("signing.key");

    // Safety: refuse to overwrite existing files.
    if cert_path.exists() {
        anyhow::bail!(
            "{} {} {}. {} {}",
            cert_name,
            i18n::tr("already exists at"),
            cert_path.display(),
            i18n::tr("Remove it first or choose a different --output directory."),
            ""
        );
    }
    if key_path.exists() {
        anyhow::bail!(
            "signing.key {} {}. {}",
            i18n::tr("already exists at"),
            key_path.display(),
            i18n::tr("Remove it first or choose a different --output directory."),
        );
    }

    std::fs::write(&cert_path, &result.cert_or_csr_pem)
        .with_context(|| format!("{} {}", i18n::tr("Failed to write"), cert_path.display()))?;

    // Write private key with mode 0600 from the start, not as a post-write
    // chmod. Using OpenOptions with .mode(0o600) on Unix creates the file with
    // restrictive permissions atomically — there is no window during which the
    // key is world-readable.  On non-Unix systems, std::fs::write is used and
    // a warning is emitted.
    //
    // Controls: NIST SP 800-53 SC-12 (Key Management), NIST SP 800-53 AC-3.
    #[cfg(unix)]
    {
        use std::io::Write;
        use std::os::unix::fs::OpenOptionsExt;
        let mut key_file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(&key_path)
            .with_context(|| {
                format!(
                    "{} {}",
                    i18n::tr("Failed to create key file:"),
                    key_path.display()
                )
            })?;
        key_file.write_all(&result.key_pem).with_context(|| {
            format!(
                "{} {}",
                i18n::tr("Failed to write key to:"),
                key_path.display()
            )
        })?;
    }
    #[cfg(not(unix))]
    {
        std::fs::write(&key_path, &result.key_pem)
            .with_context(|| format!("{} {}", i18n::tr("Failed to write"), key_path.display()))?;
        eprintln!(
            "{} {}",
            i18n::tr(
                "Warning: key file permissions cannot be restricted on this platform. \
                 Manually restrict access to:"
            ),
            key_path.display()
        );
    }

    // Render the generation summary from structured fields — each line is
    // an independent translated string.
    if result.is_csr {
        println!("{}", i18n::tr("Generated CSR + private key"));
    } else {
        println!(
            "{}",
            i18n::tr("Generated self-signed certificate + private key")
        );
    }
    println!(
        "{}: {} (ECDSA {}, {}-bit)",
        i18n::tr("Algorithm"),
        result.algorithm,
        result.curve_name,
        result.key_bits
    );
    println!(
        "{}: O={org}, CN={org} (UMRS C2PA Signing{suffix})",
        i18n::tr("Subject"),
        org = result.organization,
        suffix = if result.is_csr {
            ""
        } else {
            " \u{2014} self-signed"
        },
    );
    if let Some(days) = result.validity_days {
        println!(
            "{}: {}",
            i18n::tr("Validity"),
            ngettext("{} day from now", "{} days from now", days).replace("{}", &days.to_string())
        );
    }
    println!();
    if result.is_csr {
        println!(
            "{}",
            i18n::tr("Submit the CSR to your Certificate Authority for signing.")
        );
        println!(
            "{}",
            i18n::tr("Keep the private key safe \u{2014} it cannot be regenerated.")
        );
    } else {
        println!(
            "{}",
            i18n::tr("Self-signed certificates will show as UNVERIFIED by external validators.")
        );
        println!(
            "{}",
            i18n::tr("For trusted status, submit a CSR to a recognized CA")
        );
        println!(
            "{}",
            i18n::tr("or add your org's root to the trust anchors.")
        );
    }
    println!();
    println!("{}:", i18n::tr("Files written"));
    println!("  {} : {}", cert_name, cert_path.display());
    println!("  signing.key : {}", key_path.display());
    println!();
    println!(
        "{} umrs-c2pa.toml:",
        i18n::tr("Next step — add these to your")
    );
    println!();
    println!("  [identity]");
    if result.is_csr {
        println!(
            "  # {}:",
            i18n::tr("After your CA signs the CSR, replace signing.csr with the signed cert")
        );
        println!(
            "  cert_chain  = \"{}\"",
            output_dir.join("signing.pem").display()
        );
    } else {
        println!("  cert_chain  = \"{}\"", cert_path.display());
    }
    println!("  private_key = \"{}\"", key_path.display());
    println!();
    println!("{} umrs-c2pa creds validate", i18n::tr("Then run:"));

    Ok(())
}

// ── creds validate ───────────────────────────────────────────────────────────

fn cmd_creds_validate(config: &UmrsConfig) {
    verbose!(
        "{}",
        i18n::tr("Validating configured signing credentials...")
    );
    let checks = c2pa::creds::validate(config);

    let pass_mark = "\u{2714}"; // checkmark
    let fail_mark = "\u{2718}"; // x-mark

    println!();
    println!("{}", i18n::tr("Credential Validation"));
    println!("{}", "\u{2501}".repeat(56));

    for check in &checks {
        let (mark, label) = if check.ok {
            (pass_mark, "PASS")
        } else {
            (fail_mark, "FAIL")
        };
        println!("  {mark} [{label}] {}: {}", check.check, check.message);
    }

    println!("{}", "\u{2501}".repeat(56));

    let failures = checks.iter().filter(|c| !c.ok).count();
    if failures > 0 {
        println!(
            "{}",
            ngettext(
                "{} check failed.",
                "{} checks failed.",
                u32::try_from(failures).unwrap_or(u32::MAX)
            )
            .replace("{}", &failures.to_string())
        );
        println!();
        println!(
            "{} umrs-c2pa creds generate --output /path/to/certs/",
            i18n::tr("To generate new credentials:")
        );
        std::process::exit(1);
    } else {
        println!("{}", i18n::tr("All checks passed."));
    }
}
