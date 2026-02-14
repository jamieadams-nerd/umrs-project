use kernel_files::{
    SecureReader, SelinuxEnforce, SelinuxMls, SelinuxPolicyVers, 
    ProcFips, GenericDualBool, EnforceState
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
    // Note: 'fips_mode' returns "1 1" or "0 0"
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
}


