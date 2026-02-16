use mcs_setrans::{GLOBAL_TRANSLATOR, SecurityRange, load_setrans_file};
use std::io::Write;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Corrected Builder for env_logger
    env_logger::Builder::from_default_env()
        .format(|buf, record| {
            // Explicitly using the writeln! macro with the io::Write trait in scope
            writeln!(buf, "[{}] - {}", record.level(), record.args())
        })
        .init();

    // 1. Load your test file
    let test_path = "setrans.conf";
    if let Err(e) = load_setrans_file(test_path) {
        log::error!("CRITICAL: Failed to load '{}': {}", test_path, e);
        std::process::exit(1);
    }

    println!("\n--- Testing setrans.conf Logic ---");

    let translator = GLOBAL_TRANSLATOR.read().unwrap();

    // 2. Define the "Hard" test cases from your file
    let test_cases =
        vec!["s0-s2", "s0-s2:c0", "s2:c0-s15:c0.c1023", "s2:c1-s2:c0,c1"];

    for raw_ctx in test_cases {
        match SecurityRange::from_str(raw_ctx) {
            Ok(range) => match translator.lookup(&range) {
                Some(label) => println!("MATCH: '{}' -> '{}'", raw_ctx, label),
                None => println!("FAIL:  '{}' not found in map", raw_ctx),
            },
            Err(e) => println!("ERROR: Could not parse '{}': {}", raw_ctx, e),
        }
    }

    let translator = GLOBAL_TRANSLATOR.read().unwrap();

    println!(
        "\n--- TRIPLE-CHECK: Forward Lookups (Range -> Label + Detail) ---"
    );
    let forward_tests =
        vec!["s0:c100,c90", "s0:c500", "s0:c90,c100", "s2:c0-s15:c0.c1023"];

    for raw in forward_tests {
        if let Ok(range) = SecurityRange::from_str(raw) {
            let label = translator
                .lookup(&range)
                .unwrap_or_else(|| "MISSING".to_string());
            let detail = translator.get_detail(&range);
            println!(
                "QUERY:  {}\nLABEL:  {}\nDETAIL: {}\n",
                raw, label, detail
            );
        }
    }

    println!(
        "--- TRIPLE-CHECK: Reverse Lookups (Marking -> Kernel String + Detail) ---"
    );
    let reverse_tests = vec!["CUI//LEI/JUV", "Secret:A-SystemHigh"];

    for marking in reverse_tests {
        let results = translator.lookup_by_marking(marking);
        if results.is_empty() {
            println!("QUERY:  {}\nRESULT: No match found\n", marking);
        } else {
            for (kernel_str, detail) in results {
                println!(
                    "QUERY:  {}\nKERNEL: {}\nDETAIL: {}\n",
                    marking, kernel_str, detail
                );
            }
        }
    }

    // 1. Simulate a process with a specific MCS range
    // Example: A user with access to categories c0 through c10
    let my_proc_raw = "s0:c0.c10";
    let my_proc_ctx = SecurityRange::from_str(my_proc_raw)?;

    println!("\n--- Test: What can I read? ---");
    println!("Current Process Context: {}", my_proc_raw);

    // Use the new backend method to find all authorized labels
    let authorized_markings = translator.list_readable_markings(&my_proc_ctx);

    if authorized_markings.is_empty() {
        println!("Result: No markings authorized for this context.");
    } else {
        println!("Authorized Markings ({} found), you have access to:", authorized_markings.len());
        //for (label, detail) in authorized_markings {
        //println!("  - {:<25} | {}", label, detail);
        //}

        for (range, label, detail) in authorized_markings {
            // Lazy helper to format the range back to s0:c#
            let kernel_fmt = if range.low == range.high {
                format!("s{}:{}", range.low.sensitivity, range.low.categories)
            } else {
                format!(
                    "s{}:{}-s{}:{}",
                    range.low.sensitivity,
                    range.low.categories,
                    range.high.sensitivity,
                    range.high.categories
                )
            };

            println!("  [{:<18}] {:<25} | {}", kernel_fmt, label, detail);
        }
    }
    println!("\n");

    Ok(())
}
