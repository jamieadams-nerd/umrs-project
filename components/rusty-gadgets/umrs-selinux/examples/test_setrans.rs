use umrs_selinux::mcs::setrans;
use umrs_selinux::xattrs::parse_mcs_categories;

fn main() {
    // 1. Initialize the High-Assurance Translation Map (Lazy Load)
    // NIST 800-53 AU-3: Audit of the translation table itself
    let map = setrans::get_map();
    println!("--- [ UMRS SETRANS AUDIT ] ---");
    
    // 2. Test Case: "CUI//LEI/INV" (s0:c90,c99)
    // We simulate a raw bitmask found on a file like your TESTFILE
    let test_cats = "c90,c99";
    if let Ok(bits) = parse_mcs_categories(test_cats) {
        match map.get_text(&bits) {
            Some(marking) => println!("Bitmask [{}] -> Marking: {}", test_cats, marking),
            None => println!("Bitmask [{}] -> No translation found in setrans.conf", test_cats),
        }
    }

    // 3. Test Case: Reverse Lookup (Marking -> Bits)
    let search_term = "CUI//PRIVACY/CONTRACT";
    match map.get_bits(search_term) {
        Some(bits) => println!("Marking [{}] -> Bits: {}", search_term, bits),
        None => println!("Marking [{}] -> Not found in map", search_term),
    }

    // 4. Verify TPI Invariant: Non-contiguous ordering
    // In setrans.conf it is "c90,c91", but if the xattr is "c91,c90", 
    // the bitmask lookup MUST still work.
    let reordered = "c91,c90";
    if let Ok(bits) = parse_mcs_categories(reordered) {
        if let Some(marking) = map.get_text(&bits) {
            println!("Order Invariance Check: [{}] also maps to {}", reordered, marking);
        }
    }
}

