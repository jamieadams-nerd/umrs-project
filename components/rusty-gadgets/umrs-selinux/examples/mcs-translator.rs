use std::str::FromStr;
use umrs_selinux::mcs::translator::{self, GLOBAL_TRANSLATOR, SecurityRange};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    println!("\n");
    println!("--- [ UMRS SETRANS AUDIT ] ---");

    // Initialize the High-Assurance Translation Map (Lazy Load)
    // NIST 800-53 AU-3: Audit of the translation table itself
    translator::load_setrans_file("setrans.conf")?;
    let guard = GLOBAL_TRANSLATOR.read().unwrap();

    // Forward lookups : SecurityRange -> Label
    // Reverse lookups : Label -> Vec<SecurityRange/strings>


    // FORWARD Lookup - from single context string.
    // --- Forward lookup: kernel range -> marking --------------------------------
    println!("[TEST - Valid FORWARD Lookup");
    let query_range = SecurityRange::from_str("s0:c90,c91")?;
    let marking = guard
        .lookup(&query_range)
        .unwrap_or_else(|| "MISSING".to_string());
    println!("FORWARD  {} -> {}", "s0:c90,c91", marking);

    //let marking = guard
    //.lookup(&query_range)
    //.unwrap_or_else(|| "MISSING".to_string());
    //
    //assert_ne!(
        //marking,
        //"MISSING",
        //"Translator returned MISSING for range {}",
        //"s0:c90,c91"
    //);

    // REVERSE lookup: marking -> kernel ranges 
    let search_marking = "CUI//PRIVACY/CONTRACT";
    let matches = guard.lookup_by_marking(search_marking);

    if matches.is_empty() {
        println!("REVERSE  {} -> Not found", search_marking);
    } else {
        println!("REVERSE  {} ->", search_marking);
        for (kernel_str, detail) in matches {
            if detail.is_empty() {
                println!("  - {}", kernel_str);
            } else {
                println!("  - {}  --> {}", kernel_str, detail);
            }
        }
    }

    // REVERSE Invalid formats just return missing
    let search_marking = "CUI\t//PRIVACY/CONTRACT";
    let matches = guard.lookup_by_marking(search_marking);

    if matches.is_empty() {
        println!("REVERSE  {} -> Not found", search_marking);
    } else {
        println!("REVERSE  {} ->", search_marking);
        for (kernel_str, detail) in matches {
            if detail.is_empty() {
                println!("  - {}", kernel_str);
            } else {
                println!("  - {}  --> {}", kernel_str, detail);
            }
        }
    }

    Ok(())
}
