use chrono::{DateTime, Local};
use nix::unistd::{Gid, Group, Uid, User};
use rustix::fs::{IFlags, ioctl_getflags}; // Standardizing on the bitflags API
use std::fs;
use std::io;
use std::os::unix::fs::MetadataExt;
use std::path::Path;
use umrs_selinux::xattrs::SecureXattrReader;

/// High-Assurance Directory Entry (NIST 800-53 AC-3)
pub struct DirectoryEntry {
    pub name: String,
    pub selinux_type: String,
    pub mls_level: String,
    pub mtime: String,
    pub username: String,
    pub groupname: String,
    pub mode_string: String,
    pub immutable: bool,
}

/// Helper to format raw Unix mode to "drwxr-xr-x" string
fn format_mode(mode: u32) -> String {
    let mut s = String::with_capacity(10);
    let file_type = match mode & 0o170000 {
        0o040000 => 'd',
        0o120000 => 'l',
        0o100000 => '-',
        0o060000 => 'b',
        0o020000 => 'c',
        0o010000 => 'p',
        0o140000 => 's',
        _ => '?',
    };
    s.push(file_type);
    let chars = ['r', 'w', 'x'];
    for i in (0..3).rev() {
        let bits = (mode >> (i * 3)) & 0o7;
        s.push(if bits & 4 != 0 {
            chars[0]
        } else {
            '-'
        });
        s.push(if bits & 2 != 0 {
            chars[1]
        } else {
            '-'
        });
        s.push(if bits & 1 != 0 {
            chars[2]
        } else {
            '-'
        });
    }
    s
}

pub fn list_directory_ha(dir_path: &Path) -> io::Result<Vec<DirectoryEntry>> {
    let mut entries = Vec::new();
    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();
        let metadata = entry.metadata()?;
        let file = std::fs::File::open(&path)?;

        // 1. Label Provenance (TPI Verified)
        let context = SecureXattrReader::read_context(&file).ok();
        let (s_type, s_level) = match context {
            Some(ctx) => (
                ctx.security_type().to_string(),
                // Using .raw() to show the original string (SystemLow)
                ctx.level()
                    .map(|l| l.raw().to_string())
                    .unwrap_or_else(|| "s0".to_string()),
            ),
            None => ("<unlabeled>".to_string(), "N/A".to_string()),
        };

        // 2. FS Integrity Flags (NIST 800-53 SI-7)
        let is_immutable = match ioctl_getflags(&file) {
            Ok(f) => f.contains(IFlags::IMMUTABLE),
            Err(_) => false,
        };

        let mtime: DateTime<Local> = metadata.modified()?.into();
        let mode = metadata.mode();
        let mut name = entry.file_name().to_string_lossy().into_owned();
        if (mode & 0o170000) == 0o040000 {
            name.push('/');
        }

        // NIST 800-53 AC-3: Safe Name Resolution
        let username = match User::from_uid(Uid::from_raw(metadata.uid())) {
            Ok(Some(user)) => user.name,
            _ => metadata.uid().to_string(),
        };

        let groupname = match Group::from_gid(Gid::from_raw(metadata.gid())) {
            Ok(Some(group)) => group.name,
            _ => metadata.gid().to_string(),
        };

        entries.push(DirectoryEntry {
            name,
            selinux_type: s_type,
            mls_level: s_level,
            mtime: mtime.format("%Y-%m-%d %H:%M").to_string(),
            username,
            groupname,
            mode_string: format_mode(mode),
            immutable: is_immutable,
        });
    }
    Ok(entries)
}

fn main() -> io::Result<()> {
    //env_logger::init();

    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("debug"),
    )
    .format_timestamp(None) // This removes the timestamp globally
    .init();

    let target = std::env::args().nth(1).unwrap_or_else(|| ".".to_string());
    let entries = list_directory_ha(Path::new(&target))?;

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

    println!(
        "{:<w_mode$} {:<2} {:<w_type$} {:<w_level$} {:<w_user$} {:<18} {}",
        "MODE",
        "I",
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
        let i = if e.immutable {
            "i"
        } else {
            "-"
        };
        println!(
            "{:<w_mode$} {:<2} {:<w_type$} {:<w_level$} {}:{:<w_user_sub$} {:<18} {}",
            e.mode_string,
            i,
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
    Ok(())
}
