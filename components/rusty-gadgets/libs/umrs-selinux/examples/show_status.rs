// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a, Imodium Operator)
//! Example: Exercise SELinux status inspection.

use umrs_selinux::{SelinuxStatus, is_selinux_enabled, is_selinux_mls_enabled};

fn main() {
    println!("--- SELINUX RUNTIME STATUS ---");

    // 1. Test the individual legacy wrappers
    let enabled = is_selinux_enabled();
    let mls_enabled = is_selinux_mls_enabled();

    println!("{:<20} : {}", "SELinux Enabled", enabled);
    println!("{:<20} : {}", "MLS Policy Active", mls_enabled);

    // 2. Test the SelinuxStatus struct (Snapshot)
    let status = SelinuxStatus::current();

    println!("{:<20} : {}", "Enforcing Mode", status.enforcing());

    // 3. Simple logic check
    if status.enabled() {
        let mode = if status.enforcing() {
            "Enforcing"
        } else {
            "Permissive"
        };
        println!("\nSystem is currently in {} mode.", mode);
    } else {
        println!("\nSELinux is disabled or not mounted at /sys/fs/selinux.");
    }
}
