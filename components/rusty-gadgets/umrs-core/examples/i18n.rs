// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Jamie Adams
//
// Unclassified MLS Reference System Project (UMRS)
// MIT licensed—use, modify, and redistribute per LICENSE.
//
// umrs-tester: a small scratch/test binary for exercising umrs-core APIs.
//

use umrs_core::i18n;

fn main() {
    // Pick a domain for this tester. You can keep it "umrs-tester" permanently.
    i18n::init("umrs-tester");

    println!("umrs-tester: starting up");

    // ---- i18n smoke test ----
    // With no translations installed, this prints the msgid.
    println!("tr(test.hello) = {}", i18n::tr("test.hello"));

    // ---- Add future umrs-core experiments below ----
    // Example pattern:
    //   - call umrs_core::<module>::<fn>()
    //   - print results
    //   - keep it simple and explicit

    println!("umrs-tester: done");
}
