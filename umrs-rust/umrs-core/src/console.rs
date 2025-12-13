// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Jamie Adams
//
// Unclassified MLS Reference System Project
//
// Provide pretty, colorized output to standard out and
// error to really get the operator's attention.
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
