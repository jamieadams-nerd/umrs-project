// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a, Imodium Operator)
//
// ============================================================================
//!
//! UMRS KERNEL-FILES: High-Assurance Kernel Attribute Modeling
//!
//! This library provides a High-Assurance (HA) substrate for interacting with
//! kernel-resident security attributes on Red Hat Enterprise Linux 10.
//!
//! RATIONALE FOR SPECIALIZED READERS:
//! Standard userspace I/O primitives are insufficient for systems operating
//! under NSA "Raise the Bar" (RTB) mandates or NIST 800-53 High-Impact controls.
//! Traditional file access ignores the "provenance" of the data source and
//! relies on weak, string-based typing that is prone to logic flaws.
//!
//! DESIGN PRINCIPLES:
//!
//! 1. PROVENANCE VERIFICATION (NIST 800-53 SI-7):
//!    Before any byte is read, the SecureReader verifies the backing filesystem
//!    integrity using FsMagic (statfs). This ensures that critical security
//!    attributes (e.g., /sys/fs/selinux/enforce) originate from the authentic
//!    Kernel Security Server (selinuxfs) and have not been spoofed via bind-mounts
//!    or malicious overlays.
//!
//! 2. STRONG DATA MODELING (VERNACULAR FIDELITY):
//!    This library models kernel 'attributes' (kobjects) as unique Rust types.
//!    By encoding kernel-doc metadata (DESCRIPTION, ATTRIBUTE_NAME) directly
//!    into the type, we eliminate "Type Confusion" and ensure that the software
//!    vernacular is 1:1 with the Linux Kernel Source (selinuxfs.rst).
//!
//! 3. DETERMINISTIC PARSING (NSA RTB - MINIMIZED TCB):
//!    Parsing logic is strictly bounded and heap-free where possible. The use
//!    of "Redundant (TPI) Parsing" ensures that security decisions are not
//!    dependent on a single logic path. If two independent parsing strategies
//!    (Declarative vs. Imperative) disagree, the system Fails-Closed.
//!
//! 4. NON-BYPASSABILITY (NSA RTB - RAIN):
//!    By leveraging Rust's type system and private execution engines, we ensure
//!    that security checks are Always Invoked. The use of PhantomData and
//!    StaticSource traits prevents the developer from accidentally querying
//!    security nodes with incorrect parameters or unverified paths.
// ============================================================================
use nix::sys::statfs::{FsType, PROC_SUPER_MAGIC, SELINUX_MAGIC, statfs};
use nom::{
    IResult,
    bytes::complete::{tag, take_until},
};
use std::fmt;
use std::fs::File;
use std::io::{self, Read};
use std::marker::PhantomData;
use std::path::{Path, PathBuf};

// ===========================================================================
// Reading Kernel Object Attribute Files
//
// These are /proc and /sys files which contain useful informatoon to
// userland from the kernel.
// ===========================================================================
/// NIST 800-53 SI-7: Software and Information Integrity
/// Core contract for any file originating from a trusted Kernel Pseudo-FS.
pub trait KernelFileSource {
    type Output;

    /// NIST 800-53 AU-3: Attribute Identifier
    /// The formal attribute name as defined in kernel kobject/sysfs vernacular.
    const ATTRIBUTE_NAME: &'static str;

    /// NIST 800-53 AU-3: Event Content/Description
    /// Documentation or Format string derived from kernel-parameters.txt or rst docs.
    const DESCRIPTION: &'static str;

    /// /// NIST 800-53 AU-3: Audit Context
    /// Additional context regarding deprecation, defaults, or kernel version specifics.
    const KERNEL_NOTE: &'static str = "";

    /// NIST 800-53 AU-3: Location/Provenance Identifier
    /// The parent kobject in the kernel hierarchy (e.g., "selinuxfs" or "crypto")
    const KOBJECT: &'static str;

    fn parse(data: &[u8]) -> io::Result<Self::Output>;
}

/// Specifically for nodes with fixed, immutable paths.
/// Defaults to SELINUX_MAGIC, but can be overridden for ProcFS or others.
pub trait StaticSource: KernelFileSource {
    const PATH: &'static str;
    const EXPECTED_MAGIC: FsType = SELINUX_MAGIC;
}

// --- Domain Specific Data Types ---

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnforceState {
    Permissive = 0,
    Enforcing = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DualBool {
    pub current: bool,
    pub pending: bool,
}

// ---------------------------------------------------------------------------
// Audit Card for kernel attributes
// ---------------------------------------------------------------------------
/// A helper struct to provide a High-Assurance "Audit Card" for a kernel attribute.
pub struct AttributeCard<'a, T: KernelFileSource> {
    pub value: &'a T::Output,
    pub path: &'static str,
}

impl<'a, T: KernelFileSource + StaticSource> fmt::Display
    for AttributeCard<'a, T>
where
    T::Output: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r"--- [ UMRS KERNEL ATTRIBUTE CARD ] ---
KernelObj : {}
Attribute : {}
Path      : {}
Value     : {:?}

Description: 
{}

Note: 
{}
---------------------------------------",
            T::KOBJECT,
            T::ATTRIBUTE_NAME,
            T::PATH,
            self.value,
            T::DESCRIPTION,
            T::KERNEL_NOTE
        )
    }
}

// --- Static Path Markers (Known Kernel Nodes) ---

pub struct SelinuxEnforce;
impl KernelFileSource for SelinuxEnforce {
    type Output = EnforceState;
    const KOBJECT: &'static str = "selinuxfs";
    const ATTRIBUTE_NAME: &'static str = "enforce";
    const DESCRIPTION: &'static str = r"0 -- permissive
1 -- enforcing
Set the security-enforcing mode of SELinux.";
    const KERNEL_NOTE: &'static str =
        "Can be toggled at runtime via /sys/fs/selinux/enforce.";

    fn parse(data: &[u8]) -> io::Result<Self::Output> {
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

pub struct SelinuxPolicyVers;
impl KernelFileSource for SelinuxPolicyVers {
    type Output = u32;
    const KOBJECT: &'static str = "selinuxfs";
    const ATTRIBUTE_NAME: &'static str = "policyvers";
    const DESCRIPTION: &'static str =
        "The version of the SELinux policy language supported by the kernel.";

    fn parse(data: &[u8]) -> io::Result<Self::Output> {
        let s = std::str::from_utf8(data).map_err(|_| {
            io::Error::new(io::ErrorKind::InvalidData, "Non-UTF8 data")
        })?;
        s.trim()
            .parse::<u32>()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }
}
impl StaticSource for SelinuxPolicyVers {
    const PATH: &'static str = "/sys/fs/selinux/policyvers";
}

pub struct ProcFips;
impl KernelFileSource for ProcFips {
    type Output = bool;
    const KOBJECT: &'static str = "crypto";
    const ATTRIBUTE_NAME: &'static str = "fips_enabled";
    const DESCRIPTION: &'static str = "0 -- FIPS disabled\n1 -- FIPS enabled";
    const KERNEL_NOTE: &'static str =
        "Verified via /proc/sys/crypto/fips_enabled (PROC_SUPER_MAGIC).";

    fn parse(data: &[u8]) -> io::Result<Self::Output> {
        match data.first() {
            Some(b'1') => Ok(true),
            Some(b'0') => Ok(false),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid FIPS bit",
            )),
        }
    }
}
impl StaticSource for ProcFips {
    const PATH: &'static str = "/proc/sys/crypto/fips_enabled";
    const EXPECTED_MAGIC: FsType = PROC_SUPER_MAGIC;
}

// --- Dynamic Path Markers ---

pub struct GenericKernelBool {
    pub path: PathBuf,
}
impl KernelFileSource for GenericKernelBool {
    type Output = bool;
    const KOBJECT: &'static str = "dynamic/pending";
    const ATTRIBUTE_NAME: &'static str = "generic_bool";
    const DESCRIPTION: &'static str = "Dynamic boolean attribute";

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

pub struct GenericDualBool {
    pub path: PathBuf,
}
impl KernelFileSource for GenericDualBool {
    type Output = DualBool;
    const KOBJECT: &'static str = "dynamic/pending";
    const ATTRIBUTE_NAME: &'static str = "generic_dual_bool";
    const DESCRIPTION: &'static str =
        "Dynamic dual-boolean attribute (current pending)";

    fn parse(data: &[u8]) -> io::Result<Self::Output> {
        let s = std::str::from_utf8(data).map_err(|_| {
            io::Error::new(io::ErrorKind::InvalidData, "Non-UTF8")
        })?;
        let parts: Vec<&str> = s.split_whitespace().collect();
        if parts.len() < 2 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Malformed dual bool",
            ));
        }
        Ok(DualBool {
            current: parts[0] == "1",
            pending: parts[1] == "1",
        })
    }
}

// --- The High-Assurance Reader Engine ---

pub struct SecureReader<T> {
    _marker: PhantomData<T>,
}

impl<T> Default for SecureReader<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> SecureReader<T> {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<T: StaticSource> SecureReader<T> {
    pub fn read(&self) -> io::Result<T::Output> {
        SecureReader::<T>::execute_read(Path::new(T::PATH), T::EXPECTED_MAGIC)
    }
}

impl SecureReader<GenericKernelBool> {
    pub fn read_generic(&self, node: &GenericKernelBool) -> io::Result<bool> {
        SecureReader::<T>::execute_read(&node.path, SELINUX_MAGIC)
    }
}

impl SecureReader<GenericDualBool> {
    pub fn read_generic(&self, node: &GenericDualBool) -> io::Result<DualBool> {
        SecureReader::<T>::execute_read(&node.path, SELINUX_MAGIC)
    }
}

impl<T: KernelFileSource> SecureReader<T> {
    fn execute_read(
        path: &Path,
        expected_magic: FsType,
    ) -> io::Result<T::Output> {

        let stats = statfs(path).map_err(io::Error::other)?;

        if stats.filesystem_type() != expected_magic {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                format!(
                    "Integrity Failure: {} is not on the authorized filesystem",
                    path.display()
                ),
            ));
        }

        let mut file = File::open(path)?;
        let mut buffer = [0u8; 16];
        let bytes_read = file.read(&mut buffer)?;

        T::parse(&buffer[..bytes_read])
    }
}

// ===========================================================================
// Redundant (TPI) logic for parsing security contexts
// ===========================================================================
fn parse_type_path_a(input: &str) -> IResult<&str, &str> {
    let (input, _) = take_until(":")(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = take_until(":")(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, type_name) = take_until(":")(input)?;
    Ok((input, type_name))
}

fn parse_type_path_b(input: &str) -> io::Result<&str> {
    let mut parts = input.split(':');
    let _u = parts
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "No User"))?;
    let _r = parts
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "No Role"))?;
    let t = parts
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "No Type"))?;
    Ok(t)
}

pub fn validate_type_redundant(context: &str) -> io::Result<&str> {
    let (_, type_a) = parse_type_path_a(context).map_err(|_| {
        io::Error::new(io::ErrorKind::InvalidData, "Path A Logic Failure")
    })?;

    let type_b = parse_type_path_b(context)?;

    if type_a != type_b {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "RTB Redundancy Failure: Logic Mismatch Detected",
        ));
    }

    Ok(type_a)
}
