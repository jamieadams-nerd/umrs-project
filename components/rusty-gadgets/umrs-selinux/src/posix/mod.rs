pub mod identity;
pub mod primitives;

// Public API — everything a consumer needs from one import path
pub use identity::{
    LinuxGroup, LinuxGroupName, LinuxOwnership, LinuxUser, LinuxUsername,
    PosixNameError, UserIdentity,
};
pub use primitives::{
    DevId, FileMode, FileSize, Gid, HardLinkCount, Inode, Uid,
};
