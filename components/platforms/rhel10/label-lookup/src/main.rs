use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::env;

/// Parse setrans.conf and build marking → numeric mapping.
fn build_translation_map<P: AsRef<Path>>(
    path: P,
) -> std::io::Result<std::collections::HashMap<String, String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut map = std::collections::HashMap::new();

    for line in reader.lines() {
        let line = line?;

        // Only process translation lines
        if !line.starts_with("s0:c") {
            continue;
        }

        // Split on '='
        let parts: Vec<&str> = line.splitn(2, '=').collect();
        if parts.len() != 2 {
            continue;
        }

        let numeric = parts[0].trim();

        // Strip comment
        let rhs = parts[1];
        let marking = match rhs.find('#') {
            Some(pos) => &rhs[..pos],
            None => rhs,
        };

        let marking = marking.trim();

        map.insert(marking.to_string(), numeric.to_string());
    }

    Ok(map)
}

/// Lookup numeric label from marking string.
pub fn lookup_numeric_label(marking: &str) -> Option<String> {
    let path = "/etc/selinux/targeted/setrans.conf";

    match build_translation_map(path) {
        Ok(map) => map.get(marking).cloned(),
        Err(_) => None,
    }
}



fn main() {
    // Expect marking string as first argument
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!(
            "Usage: {} <MARKING>\nExample: {} CUI//GOVT/GOVTD",
            args[0], args[0]
        );
        std::process::exit(1);
    }

    let marking = &args[1];

    match lookup_numeric_label(marking) {
        Some(numeric) => {
            println!("{}", numeric);
        }
        None => {
            eprintln!("[FAIL] Marking not found: {}", marking);
            std::process::exit(2);
        }
    }
}
