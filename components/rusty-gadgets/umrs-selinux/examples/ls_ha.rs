use std::io;
use std::path::Path;
use std::time::Instant;
use umrs_selinux::utils::dirlist;
use umrs_selinux::utils::dirlist::DirectoryEntry;

fn main() -> io::Result<()> {
    //env_logger::init();

    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("debug"),
    )
    .format_timestamp(None) // This removes the timestamp globally
    .init();

    let start_time = Instant::now();
    let target = std::env::args().nth(1).unwrap_or_else(|| ".".to_string());
    let mut entries = dirlist::list_directory_ha(Path::new(&target))?;

    // sort_by() uses zero-copy logic - happens in-place on the vector.
    // Show directories fist is a standard ls behavior
    //entries.sort_by(|a, b| a.name.cmp(&b.name));
    entries.sort_by(|a, b| {
        // Check if it's a directory by looking at the mode_string prefix 'd'
        let a_is_dir = a.mode_string.starts_with('d');
        let b_is_dir = b.mode_string.starts_with('d');

        b_is_dir
            .cmp(&a_is_dir) // Directories (true) come before files (false)
            .then(a.name.cmp(&b.name))
    });

    let duration = start_time.elapsed();
    let stack_bytes = entries.len() * std::mem::size_of::<DirectoryEntry>();
    log::debug!(
        "Retrieved and sorted {} entries ({} bytes) in {:?}",
        entries.len(),
        stack_bytes,
        duration
    );

    let mut w_type = 12;
    let mut w_level = 9;
    let mut w_user = 9;
    let w_mode = 10;
    for e in &entries {
        w_type = w_type.max(e.selinux_type.len());
        w_level = w_level.max(e.mls_level.len());
        w_user = w_user.max(e.username.len() + e.groupname.len() + 1);
    }
    w_type += 2;
    w_level += 2;
    w_user += 2;

    println!("\n");
    println!(
        "{:<w_mode$} {:3} {:<w_type$} {:<w_level$} {:<w_user$} {:<18} {}",
        "MODE",
        "IAV",
        "SELINUX TYPE",
        "MLS LEVEL",
        "UID:GID",
        "MODIFIED",
        "NAME",
        w_mode = w_mode,
        w_type = w_type,
        w_level = w_level,
        w_user = w_user
    );
    println!("{}", "-".repeat(110));

    for e in entries {
        // Constants for the "Reference Monitor" look
        const RED: &str = "\x1b[31m";
        const YELLOW: &str = "\x1b[33m";
        const GREEN: &str = "\x1b[32m";
        const RESET: &str = "\x1b[0m";
        const DIM: &str = "\x1b[2m";

        // Build the marker string
        // Immutable, POSIX ACLs, and IMVA
        let i_bit = if e.immutable {
            format!("{}I{}", RED, RESET)
        } else {
            format!("{}-{}", DIM, RESET)
        };
        let a_bit = if e.has_acl {
            format!("{}A{}", YELLOW, RESET)
        } else {
            format!("{}-{}", DIM, RESET)
        };
        let v_bit = if e.has_ima {
            format!("{}V{}", GREEN, RESET)
        } else {
            format!("{}-{}", DIM, RESET)
        };
        let integrity_marker = format!("{}{}{}", i_bit, a_bit, v_bit);

        // ===================================================================
        // Now, print a row (entry) to stdout.
        println!(
            "{:<w_mode$} {:<2} {:<w_type$} {:<w_level$} {}:{:<w_user_sub$} {:<18} {}",
            e.mode_string,
            integrity_marker,
            e.selinux_type,
            e.mls_level,
            e.username,
            e.groupname,
            e.mtime,
            e.name,
            w_mode = w_mode,
            w_type = w_type,
            w_level = w_level,
            w_user_sub = w_user - e.username.len() - 1
        );
    }
    println!("\n");
    Ok(())
}
