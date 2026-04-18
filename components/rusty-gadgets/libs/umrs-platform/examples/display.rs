// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
use umrs_platform::detect::OsDetector;

fn main() {
    env_logger::init(); // Must be called before any log macros
    //
    let result = match OsDetector::default().detect() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Detection error: {e}");
            return;
        }
    };

    if let Some(release) = &result.os_release {
        println!("OS ID:      {}", release.id.as_str());
        if let Some(version) = &release.version_id {
            println!("Version ID: {}", version.as_str());
        }
        println!("Name:       {}", release.name.as_str());
    } else {
        println!("OS release could not be determined.");
    }

    println!("Trust:      {:?}", result.label_trust);
    println!("Confidence: {:?}", result.confidence.level());
}
