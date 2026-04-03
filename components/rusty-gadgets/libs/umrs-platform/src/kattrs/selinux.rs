// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a, Imodium Operator)
//! SELinux kernel attribute types (selinuxfs — `/sys/fs/selinux/`).
//!
//! All types in this module read from selinuxfs and are verified against
//! `SELINUX_MAGIC` before any bytes are parsed.
//!
//! NIST SP 800-53 AC-3, AC-16, AC-25, AU-3: Access enforcement, security attributes,
//! reference monitor, and audit record content.
//! NSA RTB RAIN: Non-bypassable — all reads route through `SecureReader`.

use nix::sys::statfs::SELINUX_MAGIC;
use std::io;
use std::path::{Path, PathBuf};

use super::traits::{KernelFileSource, SecureReader, StaticSource};
use super::types::DualBool;

// ===========================================================================
// Static-path selinuxfs attributes
// ===========================================================================

/// SELinux enforcement mode attribute node (`/sys/fs/selinux/enforce`).
///
/// NIST SP 800-53 AC-3: Access Enforcement — this attribute directly determines
/// whether the kernel LSM enforces or merely audits policy decisions.
/// NIST SP 800-53 AC-25: Reference Monitor — the enforcement bit is the kernel's
/// reference monitor enable switch.
pub struct SelinuxEnforce;
impl KernelFileSource for SelinuxEnforce {
    type Output = super::types::EnforceState;
    const KOBJECT: &'static str = "selinuxfs";
    const ATTRIBUTE_NAME: &'static str = "enforce";
    const DESCRIPTION: &'static str = r"0 -- permissive
1 -- enforcing
Set the security-enforcing mode of SELinux.";
    const KERNEL_NOTE: &'static str = "Can be toggled at runtime via /sys/fs/selinux/enforce.";

    fn parse(data: &[u8]) -> io::Result<Self::Output> {
        use super::types::EnforceState;
        match data.first() {
            Some(b'1') => Ok(EnforceState::Enforcing),
            Some(b'0') => Ok(EnforceState::Permissive),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid enforce bit",
            )),
        }
    }
}
impl StaticSource for SelinuxEnforce {
    const PATH: &'static str = "/sys/fs/selinux/enforce";
}

/// SELinux MLS capability attribute node (`/sys/fs/selinux/mls`).
///
/// NIST SP 800-53 AC-16: Security and Privacy Attributes — confirms that the
/// kernel policy supports Multi-Level Security labeling required for CUI
/// compartment enforcement.
pub struct SelinuxMls;
impl KernelFileSource for SelinuxMls {
    type Output = bool;
    const KOBJECT: &'static str = "selinuxfs";
    const ATTRIBUTE_NAME: &'static str = "mls";
    const DESCRIPTION: &'static str = "0 -- MLS disabled\n1 -- MLS enabled";
    const KERNEL_NOTE: &'static str =
        "Indicates if the Multi-Level Security (MLS) policy is active.";

    fn parse(data: &[u8]) -> io::Result<Self::Output> {
        match data.first() {
            Some(b'1') => Ok(true),
            Some(b'0') => Ok(false),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid MLS state",
            )),
        }
    }
}
impl StaticSource for SelinuxMls {
    const PATH: &'static str = "/sys/fs/selinux/mls";
}

/// SELinux policy version attribute node (`/sys/fs/selinux/policyvers`).
///
/// NIST SP 800-53 AU-3: Audit Record Content — the policy version is a required
/// component of any audit record involving SELinux access decisions, enabling
/// post-incident reconstruction against the correct policy baseline.
pub struct SelinuxPolicyVers;
impl KernelFileSource for SelinuxPolicyVers {
    type Output = u32;
    const KOBJECT: &'static str = "selinuxfs";
    const ATTRIBUTE_NAME: &'static str = "policyvers";
    const DESCRIPTION: &'static str =
        "The version of the SELinux policy language supported by the kernel.";

    fn parse(data: &[u8]) -> io::Result<Self::Output> {
        let s = std::str::from_utf8(data)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Non-UTF8 data"))?;
        // SI-12: use a fixed error string — do not propagate ParseIntError detail to callers
        s.trim().parse::<u32>().map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "Policy version is not a valid integer",
            )
        })
    }
}
impl StaticSource for SelinuxPolicyVers {
    const PATH: &'static str = "/sys/fs/selinux/policyvers";
}

// ===========================================================================
// Dynamic-path selinuxfs attributes
// ===========================================================================

/// Dynamic boolean kernel attribute node for paths known only at runtime
/// (e.g., `/sys/fs/selinux/booleans/<name>`).
///
/// Constructed via `GenericKernelBool::new_selinux()`, which validates the
/// path prefix and binds `SELINUX_MAGIC` as the expected filesystem type at
/// construction time. Must be read via `SecureReader::read_generic()`.
///
/// NIST SP 800-53 SI-7: path prefix validation + fs magic binding prevents
/// use of this reader on non-selinuxfs paths.
pub struct GenericKernelBool {
    pub(super) path: PathBuf,
    pub(super) expected_magic: nix::sys::statfs::FsType,
}

impl GenericKernelBool {
    /// Construct a selinuxfs boolean node, validating the path prefix.
    ///
    /// # Errors
    ///
    /// Returns `io::ErrorKind::InvalidInput` if `path` does not start with
    /// `/sys/fs/selinux/`.
    pub fn new_selinux(path: PathBuf) -> io::Result<Self> {
        if !path.starts_with("/sys/fs/selinux/") {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "GenericKernelBool path must be under /sys/fs/selinux/",
            ));
        }
        Ok(Self {
            path,
            expected_magic: SELINUX_MAGIC,
        })
    }

    #[must_use = "pure accessor — returns the selinuxfs boolean attribute path bound at construction"]
    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl KernelFileSource for GenericKernelBool {
    type Output = bool;
    const KOBJECT: &'static str = "selinuxfs/booleans";
    const ATTRIBUTE_NAME: &'static str = "generic_bool";
    const DESCRIPTION: &'static str = "Dynamic selinuxfs boolean attribute";

    fn parse(data: &[u8]) -> io::Result<Self::Output> {
        match data.first() {
            Some(b'1') => Ok(true),
            Some(b'0') => Ok(false),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid bool bit",
            )),
        }
    }
}

/// Dynamic dual-boolean kernel attribute node for paths known only at runtime.
///
/// Returns both the current and pending policy values. Constructed via
/// `GenericDualBool::new_selinux()`, which validates the path prefix and
/// binds `SELINUX_MAGIC` as the expected filesystem type. Must be read via
/// `SecureReader::read_generic()`.
///
/// NIST SP 800-53 SI-7: path prefix validation + fs magic binding.
pub struct GenericDualBool {
    pub(super) path: PathBuf,
    pub(super) expected_magic: nix::sys::statfs::FsType,
}

impl GenericDualBool {
    /// Construct a selinuxfs dual-bool node, validating the path prefix.
    ///
    /// # Errors
    ///
    /// Returns `io::ErrorKind::InvalidInput` if `path` does not start with
    /// `/sys/fs/selinux/`.
    pub fn new_selinux(path: PathBuf) -> io::Result<Self> {
        if !path.starts_with("/sys/fs/selinux/") {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "GenericDualBool path must be under /sys/fs/selinux/",
            ));
        }
        Ok(Self {
            path,
            expected_magic: SELINUX_MAGIC,
        })
    }

    #[must_use = "pure accessor — returns the selinuxfs dual-boolean attribute path bound at construction"]
    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl KernelFileSource for GenericDualBool {
    type Output = DualBool;
    const KOBJECT: &'static str = "selinuxfs/booleans";
    const ATTRIBUTE_NAME: &'static str = "generic_dual_bool";
    const DESCRIPTION: &'static str =
        "Dynamic selinuxfs dual-boolean attribute (current / pending)";

    fn parse(data: &[u8]) -> io::Result<Self::Output> {
        let s = std::str::from_utf8(data)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Non-UTF8"))?;
        // NIST SP 800-218 SSDF PW.4.1: bounds-safe access — use .get() rather than direct indexing
        let parts: Vec<&str> = s.split_whitespace().collect();
        let current_str = parts
            .first()
            .copied()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Malformed dual bool"))?;
        let pending_str = parts
            .get(1)
            .copied()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Malformed dual bool"))?;
        Ok(DualBool {
            current: current_str == "1",
            pending: pending_str == "1",
        })
    }
}

// ===========================================================================
// SecureReader specializations for dynamic-path nodes
// ===========================================================================

impl SecureReader<GenericKernelBool> {
    /// Provenance-verified read of a dynamic selinuxfs boolean attribute.
    ///
    /// # Errors
    ///
    /// Returns `io::Error` if the attribute file cannot be opened, if the
    /// filesystem magic does not match `SELINUX_MAGIC` (integrity failure),
    /// or if the byte content is not a valid boolean (`0` or `1`).
    ///
    /// NIST SP 800-53 SI-10, SA-11, AC-3: the result is the live enforcement
    /// state of a runtime SELinux boolean and must be examined — discarding it
    /// silently loses the live enforcement state.
    #[must_use = "SELinux boolean read result must be examined — \
                  discard silently loses the live enforcement state"]
    pub fn read_generic(&self, node: &GenericKernelBool) -> io::Result<bool> {
        Self::execute_read(&node.path, node.expected_magic)
    }
}

impl SecureReader<GenericDualBool> {
    /// Provenance-verified read of a dynamic selinuxfs dual-boolean attribute.
    ///
    /// # Errors
    ///
    /// Returns `io::Error` if the attribute file cannot be opened, if the
    /// filesystem magic does not match `SELINUX_MAGIC` (integrity failure),
    /// or if the content cannot be parsed as two whitespace-separated boolean
    /// tokens.
    ///
    /// NIST SP 800-53 SI-10, SA-11, AC-3: the result carries both the committed
    /// and pending values of a kernel SELinux policy decision — must be examined.
    #[must_use = "SELinux boolean read result must be examined — \
                  discard silently loses the live enforcement state"]
    pub fn read_generic(&self, node: &GenericDualBool) -> io::Result<DualBool> {
        Self::execute_read(&node.path, node.expected_magic)
    }
}
