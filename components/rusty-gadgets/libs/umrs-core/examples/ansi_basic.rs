// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
use umrs_core::console::ansi::*;

fn main() {
    print!(
        "{}{}Hello, ANSI world!{}",
        AnsiColor::Cyan.start(),
        BOLD,
        RESET
    );
}
