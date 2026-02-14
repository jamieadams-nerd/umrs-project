use std::fs::File;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::marker::PhantomData;
use nix::sys::statfs::{statfs, FsType, SELINUX_MAGIC, PROC_SUPER_MAGIC};
use nom::{
    bytes::complete::{tag, take_until},
    IResult,
};



/// NIST 800-53 SI-7: Software and Information Integrity
/// Core contract for any file originating from a trusted Kernel Pseudo-FS.
pub trait KernelFileSource {
    type Output;
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

// --- Static Path Markers (Known Kernel Nodes) ---

pub struct SelinuxEnforce;
impl KernelFileSource for SelinuxEnforce {
    type Output = EnforceState;
    fn parse(data: &[u8]) -> io::Result<Self::Output> {
        match data.first() {
            Some(b'1') => Ok(EnforceState::Enforcing),
            Some(b'0') => Ok(EnforceState::Permissive),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid enforce bit")),
        }
    }
}
impl StaticSource for SelinuxEnforce {
    const PATH: &'static str = "/sys/fs/selinux/enforce";
}

pub struct SelinuxMls;
impl KernelFileSource for SelinuxMls {
    type Output = bool;
    fn parse(data: &[u8]) -> io::Result<Self::Output> {
        match data.first() {
            Some(b'1') => Ok(true),
            Some(b'0') => Ok(false),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid MLS state")),
        }
    }
}
impl StaticSource for SelinuxMls {
    const PATH: &'static str = "/sys/fs/selinux/mls";
}

pub struct SelinuxPolicyVers;
impl KernelFileSource for SelinuxPolicyVers {
    type Output = u32;
    fn parse(data: &[u8]) -> io::Result<Self::Output> {
        let s = std::str::from_utf8(data)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Non-UTF8 data"))?;
        s.trim().parse::<u32>().map_err(|e| {
            io::Error::new(io::ErrorKind::InvalidData, e)
        })
    }
}
impl StaticSource for SelinuxPolicyVers {
    const PATH: &'static str = "/sys/fs/selinux/policyvers";
}

/// High-Assurance reader for FIPS status via ProcFS
pub struct ProcFips;
impl KernelFileSource for ProcFips {
    type Output = bool;
    fn parse(data: &[u8]) -> io::Result<Self::Output> {
        match data.first() {
            Some(b'1') => Ok(true),
            Some(b'0') => Ok(false),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid FIPS bit")),
        }
    }
}
impl StaticSource for ProcFips {
    const PATH: &'static str = "/proc/sys/crypto/fips_enabled";
    const EXPECTED_MAGIC: FsType = PROC_SUPER_MAGIC;
}

// --- Dynamic Path Markers ---

/// Standard single boolean (0 or 1)
pub struct GenericKernelBool {
    pub path: PathBuf,
}
impl KernelFileSource for GenericKernelBool {
    type Output = bool;
    fn parse(data: &[u8]) -> io::Result<Self::Output> {
        match data.first() {
            Some(b'1') => Ok(true),
            Some(b'0') => Ok(false),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid bool bit")),
        }
    }
}

/// Dual boolean parser for "1 1" or "0 0" style nodes
pub struct GenericDualBool {
    pub path: PathBuf,
}
impl KernelFileSource for GenericDualBool {
    type Output = DualBool;
    fn parse(data: &[u8]) -> io::Result<Self::Output> {
        let s = std::str::from_utf8(data)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Non-UTF8"))?;
        let parts: Vec<&str> = s.split_whitespace().collect();
        if parts.len() < 2 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Malformed dual bool"));
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

impl<T> SecureReader<T> {
    pub const fn new() -> Self {
        Self { _marker: PhantomData }
    }
}

impl<T: StaticSource> SecureReader<T> {
    #[must_use]
    pub fn read(&self) -> io::Result<T::Output> {
        self.execute_read(Path::new(T::PATH), T::EXPECTED_MAGIC)
    }
}

impl SecureReader<GenericKernelBool> {
    #[must_use]
    pub fn read_generic(&self, node: &GenericKernelBool) -> io::Result<bool> {
        self.execute_read(&node.path, SELINUX_MAGIC)
    }
}

impl SecureReader<GenericDualBool> {
    #[must_use]
    pub fn read_generic(&self, node: &GenericDualBool) -> io::Result<DualBool> {
        self.execute_read(&node.path, SELINUX_MAGIC)
    }
}

impl<T: KernelFileSource> SecureReader<T> {
    fn execute_read(&self, path: &Path, expected_magic: FsType) -> io::Result<T::Output> {
        // 1. Provenance Check: Verify backing FS (NIST 800-53 SI-7)
        let stats = statfs(path).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        if stats.filesystem_type() != expected_magic {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                format!("Integrity Failure: {:?} is not on the authorized filesystem", path),
            ));
        }

        // 2. Controlled Access
        let mut file = File::open(path)?;
        let mut buffer = [0u8; 16]; 
        let bytes_read = file.read(&mut buffer)?;

        // 3. Deterministic Parsing
        T::parse(&buffer[..bytes_read])
    }
}
// ===========================================================================
// These implement the Redundant (TPI) logic for the type field in a 
// context string (e.g., user:role:type:level):
// ===========================================================================
/// Path A: Combinator-based parser (nom). 
/// NIST 800-53 AC-4 / NSA RTB Redundancy Requirement.
fn parse_type_path_a(input: &str) -> IResult<&str, &str> {
    let (input, _) = take_until(":")(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = take_until(":")(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, type_name) = take_until(":")(input)?;
    Ok((input, type_name))
}

/// Path B: Manual iteration parser. 
/// NIST 800-53 AC-4 / NSA RTB Redundancy Requirement.
fn parse_type_path_b(input: &str) -> io::Result<&str> {
    let mut parts = input.split(':');
    let _u = parts.next().ok_or(io::Error::new(io::ErrorKind::InvalidData, "No User"))?;
    let _r = parts.next().ok_or(io::Error::new(io::ErrorKind::InvalidData, "No Role"))?;
    let t = parts.next().ok_or(io::Error::new(io::ErrorKind::InvalidData, "No Type"))?;
    Ok(t)
}

/// NSA RTB "RAIN" Redundancy Check.
/// Returns Ok(type) only if both Path A and Path B agree exactly.
#[must_use]
pub fn validate_type_redundant(context: &str) -> io::Result<&str> {
    let (_, type_a) = parse_type_path_a(context)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Path A Logic Failure"))?;

    let type_b = parse_type_path_b(context)?;

    // RTB Integrity Gate: Failure of agreement results in a Deny (Fail Closed).
    if type_a != type_b {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "RTB Redundancy Failure: Logic Mismatch Detected"
        ));
    }

    Ok(type_a)
}


