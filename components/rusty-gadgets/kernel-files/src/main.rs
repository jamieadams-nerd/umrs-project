use kernel_files::{
    SecureReader, SelinuxEnforce, SelinuxMls, SelinuxPolicyVers, 
    ProcFips, GenericDualBool, EnforceState, validate_type_redundant
};
use std::path::PathBuf;

fn main() {
    println!("--- UMRS Platform Integrity Check (High-Assurance) ---");

    // 1. Hardware/Kernel FIPS Status (Verified via ProcFS Magic)
    let fips_hardware_reader = SecureReader::<ProcFips>::new();
    match fips_hardware_reader.read() {
        Ok(enabled) => println!("Hardware FIPS Enabled:  {}", enabled),
        Err(e) => eprintln!("FIPS Hardware Error:   {}", e),
    }

    // 2. SELinux Mode (Verified via SelinuxFS Magic)
    let enforce_reader = SecureReader::<SelinuxEnforce>::new();
    match enforce_reader.read() {
        Ok(EnforceState::Enforcing) => println!("SELinux Enforcement:    Enforcing"),
        Ok(EnforceState::Permissive) => println!("SELinux Enforcement:    Permissive"),
        Err(e) => eprintln!("Enforcement Error:     {}", e),
    }

    // 3. MLS Capability Check (Required for UMRS CUI Labels)
    let mls_reader = SecureReader::<SelinuxMls>::new();
    match mls_reader.read() {
        Ok(active) => println!("MLS Kernel Support:    {}", active),
        Err(e) => eprintln!("MLS Status Error:      {}", e),
    }

    // 4. Policy Versioning
    let vers_reader = SecureReader::<SelinuxPolicyVers>::new();
    match vers_reader.read() {
        Ok(version) => println!("SELinux Policy Ver:    {}", version),
        Err(e) => eprintln!("Policy Version Error:  {}", e),
    }

    println!("\n--- Dual-Boolean Testing (fips_mode) ---");

    // 5. SELinux Boolean with Dual-Value (Current/Pending)
    let dual_reader = SecureReader::<GenericDualBool>::new();
    let fips_mode_node = GenericDualBool {
        path: PathBuf::from("/sys/fs/selinux/booleans/fips_mode"),
    };

    match dual_reader.read_generic(&fips_mode_node) {
        Ok(state) => {
            println!("SELinux fips_mode:     Current={}, Pending={}", state.current, state.pending);
        }
        Err(e) => {
            eprintln!("SELinux fips_mode Err: {} (Boolean may not exist in current policy)", e);
        }
    }

    println!("\n--- NSA RTB Redundancy Verification (TPI) ---");

    // 6. Test 1: Valid high-assurance context
    // Satisfies RAIN (Redundant): Both Path A and Path B must agree.
    let valid_context = "unconfined_u:unconfined_r:umrs_data_t:s0";
    
    println!("[Test A] Valid Context: {}", valid_context);
    match validate_type_redundant(valid_context) {
        Ok(t) => println!("  Result: SUCCESS (Type identified as: {})", t),
        Err(e) => eprintln!("  Result: FAILED ({})", e),
    }

    // 7. Test 2: Mismatch Simulation (Fail-Closed)
    // Satisfies RAIN (Non-bypassable): If one parser fails or disagrees, access is denied.
    let malformed_context = "unconfined_u:unconfined_r:umrs_data_t"; // Missing final colon

    println!("[Test B] Malformed (No trailing colon): {}", malformed_context);
    match validate_type_redundant(malformed_context) {
        Ok(t) => println!("  Result: SUCCESS (Type: {})", t),
        Err(e) => eprintln!("  Result: DENIED (Reason: {})", e),
    }
}

