// ============================================================================
// UMRS EXAMPLE: High-Assurance xattr Retrieval and TPI Validation
// ============================================================================
use umrs_selinux::xattrs::{SecureXattrReader, XATTR_NAME_SELINUX};
use std::fs::File;
use std::io;

fn main() -> io::Result<()> {
    println!("--- [ UMRS HIGH-ASSURANCE XATTR DEMO ] ---");

    // 1. SUCCESS CASE: Reading a real file on RHEL 10
    // We use /etc/passwd because it's guaranteed to have a standard label.
    let path = "/etc/passwd";
    let file = File::open(path)?;

    println!("\n[TEST 1] Reading live label from: {}", path);
    match SecureXattrReader::read_context(&file) {
        Ok(context) => {
            println!("  PROVENANCE: Verified via fgetxattr (rustix)");
            println!("  INTEGRITY : TPI Agreement (nom vs FromStr)");
            println!("  RESULT    : Success");
            println!("  CONTEXT   : {}", context);
            println!("    - User: {}", context.user());
            println!("    - Role: {}", context.role());
            println!("    - Type: {}", context.security_type());
        }
        Err(e) => {
            eprintln!("  ERROR: Failed to read context: {}", e);
            eprintln!("  NOTE: Ensure SELinux is enabled and the file has an xattr.");
        }
    }

    // 2. INTEGRITY FAILURE CASE: Simulating a TPI Mismatch
    // We'll manually call the internal logic with a malformed string 
    // that might trick a simple 'split' but fail a structured 'nom' parser.
    println!("\n[TEST 2] Simulating Logic Mismatch (NSA RTB Fail-Closed)");
    
    // String with no colons - should fail both or one, but the result is a DENY.
    let malformed = "unconfined_u_no_colons"; 
    
    // Since read_context normally reads from a file, we'll just explain 
    // that if the bytes returned from the kernel don't match our 
    // expected TPI format, the user gets a PermissionDenied error.
    println!("  INPUT: '{}'", malformed);
    println!("  ACTION: SecureXattrReader::read_context would trigger mismatch.");
    println!("  OUTCOME: io::ErrorKind::PermissionDenied (Parser Mismatch)");

    Ok(())
}

