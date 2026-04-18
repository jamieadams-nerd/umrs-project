// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a. Imodium Operator)
//! # POSIX Identity Primitives
//!
//! Strong types for POSIX Linux identity — user/group names, numeric UIDs/GIDs,
//! file mode, inode numbers, device IDs, file sizes, and hard link counts.
//!
//! Key re-exports:
//!
//! - `LinuxUser`, `LinuxGroup`, `LinuxUsername`, `LinuxGroupName` — validated
//!   name types with construct-time character-set enforcement.
//! - `LinuxOwnership`, `UserIdentity` — composite identity types.
//! - `Uid`, `Gid`, `Inode`, `FileMode`, `FileSize`, `DevId`, `HardLinkCount` —
//!   numeric primitives with newtype guarantees.
//!
//! ## Compliance
//!
//! - **NIST SP 800-53 AC-2**: Account Management — typed UID/GID and username
//!   representations prevent identity confusion during access control decisions.
//! - **NIST SP 800-53 AU-3**: Audit Record Content — owner identity is a
//!   required field in every `SecureDirent` audit record.
//! - **NSA RTB RAIN**: Validate at construction; downstream code never receives
//!   an invalid identity value.

pub mod identity;
pub mod primitives;

// Public API — everything a consumer needs from one import path
pub use identity::{
    LinuxGroup, LinuxGroupName, LinuxOwnership, LinuxUser, LinuxUsername, PosixNameError,
    UserIdentity, current_username,
};
pub use primitives::{DevId, FileMode, FileSize, Gid, HardLinkCount, Inode, Uid};
