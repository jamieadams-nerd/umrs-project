//! CUI Category Access Check — Simulated Process Clearance vs File Label
//!
//! Demonstrates: reading a file's SELinux MCS categories from its security
//! context via `SecureDirent`, constructing a simulated process clearance as a
//! `CategorySet`, and applying `CategorySet::dominates()` to decide AUTHORIZED
//! or DENIED for each file in a directory.
//!
//! Scenario: a contractor's process holds CUI//PROCURE authorization (c10
//! only). Files labeled with additional categories (e.g., c30,c31 for
//! CUI//CTI/EXPT) must be refused. After the software says DENIED for
//! specs.txt, we attempt to read the file anyway and explain what happens.
//!
//! Run:
//!   `cargo run -p umrs-selinux --example cui_access_check`
//!   `cargo run -p umrs-selinux --example cui_access_check -- /path/to/dir`
#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(clippy::option_if_let_else)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::result_large_err)]
#![allow(clippy::too_many_lines)]

use std::env;
use std::fs;
use std::io;
use std::io::Read as _;
use std::path::PathBuf;

use umrs_selinux::category::{Category, CategorySet};
use umrs_selinux::secure_dirent::{SecDirError, SecureDirent, SelinuxCtxState};

// ---------------------------------------------------------------------------
// Access decision type — security findings as typed data, not log strings.
// (NIST SP 800-53 AU-3: audit record content)
// ---------------------------------------------------------------------------

enum AccessDecision {
    Authorized,
    Denied {
        reason: DenialReason,
    },
}

enum DenialReason {
    Unlabeled,
    UnverifiableLabel,
    CategoryExceedsProcess {
        file_cats: CategorySet,
    },
}

// ---------------------------------------------------------------------------
// Helper: extract the CategorySet from a file's SELinux label state.
// ---------------------------------------------------------------------------

fn file_categories(
    label: &SelinuxCtxState,
) -> Result<CategorySet, DenialReason> {
    match label {
        SelinuxCtxState::Labeled(ctx) => {
            // context::MlsLevel has public field `categories`.
            // SecurityContext::level() returns Option<&context::MlsLevel>.
            match ctx.level() {
                Some(level) => Ok(level.categories.clone()),
                // No MLS level in the label — treat as SystemLow (no categories).
                None => Ok(CategorySet::new()),
            }
        }
        SelinuxCtxState::Unlabeled => Err(DenialReason::Unlabeled),
        SelinuxCtxState::ParseFailure | SelinuxCtxState::TpiDisagreement => {
            Err(DenialReason::UnverifiableLabel)
        }
    }
}

// ---------------------------------------------------------------------------
// Core check: does process clearance dominate the file's category set?
//
// MLS dominance rule (NIST SP 800-53 AC-4):
//   process dominates file  ⟺  (process ∩ file) == file
//   i.e., the process holds ALL categories the file requires.
// ---------------------------------------------------------------------------

fn check_access(
    process_clearance: &CategorySet,
    label: &SelinuxCtxState,
) -> AccessDecision {
    match file_categories(label) {
        Ok(file_cats) => {
            if process_clearance.dominates(&file_cats) {
                AccessDecision::Authorized
            } else {
                AccessDecision::Denied {
                    reason: DenialReason::CategoryExceedsProcess {
                        file_cats,
                    },
                }
            }
        }
        Err(reason) => AccessDecision::Denied {
            reason,
        },
    }
}

// ---------------------------------------------------------------------------
// Print helpers
// ---------------------------------------------------------------------------

fn print_decision(
    filename: &str,
    label: &SelinuxCtxState,
    decision: &AccessDecision,
) {
    let label_str = match label {
        SelinuxCtxState::Labeled(ctx) => ctx.to_string(),
        SelinuxCtxState::Unlabeled => "<unlabeled>".to_string(),
        SelinuxCtxState::ParseFailure => "<parse-error>".to_string(),
        SelinuxCtxState::TpiDisagreement => "<unverifiable>".to_string(),
    };

    match decision {
        AccessDecision::Authorized => {
            println!("  [AUTHORIZED]  {filename}");
            println!("                label:  {label_str}");
        }
        AccessDecision::Denied {
            reason,
        } => {
            println!("  [DENIED]      {filename}");
            println!("                label:  {label_str}");
            match reason {
                DenialReason::Unlabeled => {
                    println!(
                        "                reason: file carries no SELinux label — MAC cannot be evaluated"
                    );
                }
                DenialReason::UnverifiableLabel => {
                    println!(
                        "                reason: label integrity gate failed — label is unverifiable"
                    );
                }
                DenialReason::CategoryExceedsProcess {
                    file_cats,
                } => {
                    println!(
                        "                reason: file categories {file_cats} exceed process clearance"
                    );
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

fn main() -> io::Result<()> {
    // Step 1: determine the directory to scan.
    let scan_dir: PathBuf = env::args().nth(1).map_or_else(
        || {
            let mut home = dirs_home();
            home.push("cui-lab");
            home
        },
        PathBuf::from,
    );

    println!("CUI Access Check");
    println!("================");
    println!("Scan directory : {}", scan_dir.display());

    // Step 2: build the simulated process clearance.
    //
    // This contractor process is authorized for CUI//PROCURE only, which maps
    // to category c10 in the test environment.
    //
    // STUCK: there is no way to discover which numeric category maps to which
    // CUI marking from the API alone.  The mapping (c10 = PROCURE, c30/c31 =
    // CTI/EXPT) comes from the MCS translator / setrans.conf, not from any
    // public API type I could find in the docs.  I am hard-coding c10 here
    // because it matches the `chcon -l s0:c10` command in the scenario setup.

    let c10 = match Category::new(10) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to construct c10: {e:?}");
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "category out of range",
            ));
        }
    };

    let mut process_clearance = CategorySet::new();
    process_clearance.insert(c10);

    println!("Process clearance: {process_clearance}  (CUI//PROCURE)");
    println!();

    // Step 3: read directory entries.
    let entries = match fs::read_dir(&scan_dir) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Cannot read directory '{}': {e}", scan_dir.display());
            eprintln!(
                "Tip: run the setup commands from the exercise to create ~/cui-lab/"
            );
            return Err(e);
        }
    };

    let mut denied_path: Option<PathBuf> = None;

    for entry_result in entries {
        let entry = match entry_result {
            Ok(e) => e,
            Err(e) => {
                eprintln!("  [ERROR] reading directory entry: {e}");
                continue;
            }
        };

        let path = entry.path();

        // SecureDirent reads the SELinux xattr via fd-based fgetxattr with
        // dual-path TPI validation (nom + FromStr).
        let dirent = match SecureDirent::from_path(&path) {
            Ok(d) => d,
            Err(SecDirError::InvalidPath(e)) => {
                eprintln!(
                    "  [ERROR] path validation for '{}': {e:?}",
                    path.display()
                );
                continue;
            }
            Err(e) => {
                eprintln!(
                    "  [ERROR] SecureDirent::from_path for '{}': {e:?}",
                    path.display()
                );
                continue;
            }
        };

        let decision = check_access(&process_clearance, &dirent.selinux_label);
        let filename = dirent.name.to_string();

        // Remember the first denied file so we can attempt to read it below.
        if denied_path.is_none()
            && let AccessDecision::Denied {
                ..
            } = &decision
        {
            denied_path = Some(path.clone());
        }

        print_decision(&filename, &dirent.selinux_label, &decision);
        println!();
    }

    // Step 4: attempt to actually read a DENIED file.
    //
    // This is the key educational moment.  After our software says DENIED, we
    // try to open and read the file using ordinary std::fs — no SELinux policy
    // awareness, just a normal open().
    println!("---");
    println!("Post-denial read test");
    println!("=====================");

    match denied_path {
        None => {
            println!("No files were denied — nothing to test.");
        }
        Some(ref path) => {
            println!(
                "Attempting to read '{}' after DENIED decision ...",
                path.display()
            );

            match fs::File::open(path) {
                Err(e) => {
                    println!("  open() failed: {e}");
                    println!(
                        "  SELinux enforcement blocked the read at the kernel level."
                    );
                }
                Ok(mut file) => {
                    let mut buf = String::new();
                    match file.read_to_string(&mut buf) {
                        Err(e) => {
                            println!("  read() failed: {e}");
                            println!(
                                "  SELinux enforcement blocked the read at the kernel level."
                            );
                        }
                        Ok(n) => {
                            println!("  Read {n} bytes successfully: {buf:?}");
                            println!();
                            println!("  WHY DID THIS SUCCEED AFTER DENIED?");
                            println!("  -----------------------------------");
                            println!(
                                "  Our software said DENIED, but the kernel said OK."
                            );
                            println!();
                            println!(
                                "  The umrs-selinux library models SELinux MCS mathematics"
                            );
                            println!(
                                "  in pure Rust userland — it does NOT call into the kernel"
                            );
                            println!(
                                "  or libselinux to enforce access.  CategorySet::dominates()"
                            );
                            println!(
                                "  is a *simulation* of the Bell-LaPadula lattice check,"
                            );
                            println!("  not a kernel syscall.");
                            println!();
                            println!(
                                "  Kernel enforcement only triggers when the SELinux policy"
                            );
                            println!(
                                "  says the PROCESS LABEL does not dominate the FILE LABEL."
                            );
                            println!(
                                "  In targeted policy (the default on RHEL/Fedora), most user"
                            );
                            println!(
                                "  processes run as 'unconfined_t' or have type enforcement"
                            );
                            println!(
                                "  rules that ALLOW cross-category reads regardless of MCS."
                            );
                            println!();
                            println!(
                                "  MCS enforcement is strong in MLS policy.  In targeted"
                            );
                            println!(
                                "  policy it is present but unconfined processes bypass it."
                            );
                            println!(
                                "  The blog post on CUI sign-and-lock covers this in detail."
                            );
                            println!();
                            println!("  In a real CUI enforcement deployment:");
                            println!(
                                "    1. The application runs as a confined SELinux type."
                            );
                            println!(
                                "    2. The policy denies cross-category reads at the kernel."
                            );
                            println!(
                                "    3. Our DENIED decision and the kernel decision agree."
                            );
                            println!(
                                "    4. The userland check is an additional audit layer, not"
                            );
                            println!(
                                "       a substitute for kernel-level policy enforcement."
                            );
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Minimal home-directory lookup without depending on the `dirs` crate.
// ---------------------------------------------------------------------------

fn dirs_home() -> PathBuf {
    env::var_os("HOME").map_or_else(|| PathBuf::from("/root"), PathBuf::from)
}
