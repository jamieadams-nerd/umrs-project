// tests/primitives.rs
//
// Tests for posix/primitives.rs — Inode, DevId, HardLinkCount,
// FileMode, FileSize.
//
// Convention: all tests in tests/ — no inline test blocks in source.

use umrs_selinux::posix::primitives::{DevId, FileMode, FileSize, HardLinkCount, Inode};

// ── Inode ─────────────────────────────────────────────────────────────────────

#[test]
fn inode_roundtrip() {
    let i = Inode::new(12345);
    assert_eq!(i.as_u64(), 12345);
}

#[test]
fn inode_from_u64() {
    let i: Inode = 99u64.into();
    assert_eq!(i.as_u64(), 99);
}

#[test]
fn inode_zero_is_valid() {
    // Kernel returns 0 for some pseudo-filesystem entries.
    assert_eq!(Inode::new(0).as_u64(), 0);
}

#[test]
fn inode_display() {
    assert_eq!(Inode::new(42).to_string(), "42");
}

// ── DevId ─────────────────────────────────────────────────────────────────────

#[test]
fn devid_same_device() {
    let a = DevId::new(65024);
    let b = DevId::new(65024);
    assert!(a.same_device_as(b));
}

#[test]
fn devid_different_device() {
    let a = DevId::new(65024);
    let b = DevId::new(65025);
    assert!(!a.same_device_as(b));
}

#[test]
fn devid_display_is_hex() {
    let d = DevId::new(0xfd00);
    assert!(d.to_string().contains("fd00"));
}

// ── HardLinkCount ─────────────────────────────────────────────────────────────

#[test]
fn nlink_one_is_not_multiply_linked() {
    assert!(!HardLinkCount::new(1).is_multiply_linked());
}

#[test]
fn nlink_two_is_multiply_linked() {
    assert!(HardLinkCount::new(2).is_multiply_linked());
}

#[test]
fn nlink_from_u64_saturates() {
    // u64::MAX saturates to u32::MAX without panic
    let h = HardLinkCount::from_u64(u64::MAX);
    assert_eq!(h.as_u32(), u32::MAX);
}

#[test]
fn nlink_display() {
    assert_eq!(HardLinkCount::new(3).to_string(), "3");
}

// ── FileMode ──────────────────────────────────────────────────────────────────

// Construction masks to lower 12 bits
#[test]
fn filemode_masks_file_type_bits() {
    // 0o100644 = regular file (0o100000) + permissions (0o644)
    // Only the lower 12 bits (0o644) should be stored.
    let m = FileMode::from_mode(0o100644);
    assert_eq!(m.as_u32(), 0o644);
}

#[test]
fn filemode_from_u32_trait() {
    let m: FileMode = 0o100755u32.into();
    assert_eq!(m.as_u32(), 0o755);
}

// Setuid
#[test]
fn filemode_setuid_detected() {
    assert!(FileMode::from_mode(0o104755).is_setuid());
}

#[test]
fn filemode_no_setuid() {
    assert!(!FileMode::from_mode(0o100755).is_setuid());
}

// Setgid
#[test]
fn filemode_setgid_detected() {
    assert!(FileMode::from_mode(0o102755).is_setgid());
}

// Sticky
#[test]
fn filemode_sticky_detected() {
    assert!(FileMode::from_mode(0o101755).is_sticky());
}

// World-writable
#[test]
fn filemode_world_writable_detected() {
    assert!(FileMode::from_mode(0o100777).is_world_writable());
}

#[test]
fn filemode_not_world_writable() {
    assert!(!FileMode::from_mode(0o100644).is_world_writable());
}

// Privilege bits
#[test]
fn filemode_has_privilege_bits_setuid() {
    assert!(FileMode::from_mode(0o104755).has_privilege_bits());
}

#[test]
fn filemode_has_privilege_bits_setgid() {
    assert!(FileMode::from_mode(0o102755).has_privilege_bits());
}

#[test]
fn filemode_no_privilege_bits() {
    assert!(!FileMode::from_mode(0o100755).has_privilege_bits());
}

// mode string formatting
#[test]
fn filemode_str_plain_755() {
    assert_eq!(FileMode::from_mode(0o100755).as_mode_str(), "rwxr-xr-x");
}

#[test]
fn filemode_str_plain_644() {
    assert_eq!(FileMode::from_mode(0o100644).as_mode_str(), "rw-r--r--");
}

#[test]
fn filemode_str_setuid_with_execute() {
    // setuid + owner execute → 's' in owner execute position
    assert_eq!(FileMode::from_mode(0o104755).as_mode_str(), "rwsr-xr-x");
}

#[test]
fn filemode_str_setuid_no_execute() {
    // setuid but no owner execute → 'S'
    assert_eq!(FileMode::from_mode(0o104644).as_mode_str(), "rwSr--r--");
}

#[test]
fn filemode_str_setgid_with_execute() {
    assert_eq!(FileMode::from_mode(0o102755).as_mode_str(), "rwxr-sr-x");
}

#[test]
fn filemode_str_setgid_no_execute() {
    assert_eq!(FileMode::from_mode(0o102644).as_mode_str(), "rw-r-Sr--");
}

#[test]
fn filemode_str_sticky_with_execute() {
    assert_eq!(FileMode::from_mode(0o101777).as_mode_str(), "rwxrwxrwt");
}

#[test]
fn filemode_str_sticky_no_execute() {
    // sticky + no other execute → 'T'
    assert_eq!(FileMode::from_mode(0o101776).as_mode_str(), "rwxrwxrwT");
}

#[test]
fn filemode_display_matches_mode_str() {
    let m = FileMode::from_mode(0o100644);
    assert_eq!(m.to_string(), m.as_mode_str());
}

// ── FileSize ──────────────────────────────────────────────────────────────────

#[test]
fn filesize_empty() {
    assert!(FileSize::new(0).is_empty());
}

#[test]
fn filesize_nonzero() {
    assert!(!FileSize::new(1024).is_empty());
}

#[test]
fn filesize_roundtrip() {
    let s = FileSize::new(4096);
    assert_eq!(s.as_u64(), 4096);
}

#[test]
fn filesize_display() {
    assert_eq!(FileSize::new(1024).to_string(), "1024");
}

// ── Type safety — these must not compile ────────────────────────────────────
//
// Uncomment to verify compile-time type safety:
//
// #[test]
// fn inode_cannot_be_compared_to_devid() {
//     let i = Inode::new(42);
//     let d = DevId::new(42);
//     assert_eq!(i, d); // ← compile error: type mismatch
// }
//
// #[test]
// fn filemode_cannot_be_used_as_inode() {
//     let m = FileMode::from_mode(0o644);
//     let _: Inode = m.into(); // ← compile error: no From<FileMode> for Inode
// }
