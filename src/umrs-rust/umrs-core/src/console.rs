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
// umrs-core 
//
// Purpose: 
//   Provide pretty, colorized output to standard out and
//   error to really get the operator's attention.
//
// Notes:
//   - <key design constraint or security property>
//   - <any invariants / assumptions / non-goals>
//   - <where to look next: related modules or docs>
//

 use colored::Colorize;

 /// Initialize console handling.
 /// Currently a no-op, but left here intentionally so
 /// all UMRS tools have a consistent entry point.
 pub fn init() {
     // Future use: global quiet mode, no-color mode, etc.
 }

 /// Informational message to STDOUT.
 pub fn info(message: &str) {
     println!(
         "{} {}",
         "[INFO]".bold(),
         message
     );
 }

 /// Success message to STDOUT.
 pub fn success(message: &str) {
     println!(
         "{} {}",
         "[ OK ]".green().bold(),
         message
     );
 }

 /// Warning message to STDERR.
 pub fn warn(message: &str) {
     eprintln!(
         "{} {}",
         "[WARN]".yellow().bold(),
         message
     );
 }

 /// Error message to STDERR.
 pub fn error(message: &str) {
     eprintln!(
         "{} {}",
         "[ERR ]".red().bold(),
         message
     );
 }

 /// Diagnostic / developer message to STDERR.
 /// Intended for verbose or troubleshooting modes.
 pub fn debug(message: &str) {
     eprintln!(
         "{} {}",
         "[DBG ]".cyan(),
         message
     );
 }

 /// Standardized boolean status line.
 /// Useful for summary views.
 pub fn status(label: &str, ok: bool) {
     if ok {
         println!(
             "{} {}",
             "[ OK ]".green().bold(),
             label
         );
     } else {
         println!(
             "{} {}",
             "[FAIL]".red().bold(),
             label
         );
     }
 }
