// ============================================================================
// UMRS SELINUX: Extended Attribute (xattr) Logic
// NIST 800-53 AC-3 / NSA RTB (Non-Bypassability & Redundancy)
// ============================================================================
use std::fs::File;
use std::io;
use rustix::fs::fgetxattr;
use nom::{bytes::complete::{tag, take_until}, IResult};

use crate::context::{SecurityContext, ContextParseError, MlsLevel};
use crate::sensitivity::SensitivityLevel;
use crate::category::CategorySet;

/// The standard SELinux xattr name (NIST 800-53 AU-3: Source Identifier)
pub const XATTR_NAME_SELINUX: &str = "security.selinux";

pub struct SecureXattrReader;

impl SecureXattrReader {
    /// NIST 800-53 SI-7: High-Assurance xattr retrieval via raw syscalls.
    pub fn read_raw(file: &File, attr: &str) -> io::Result<Vec<u8>> {
        // Size probe
        let size = fgetxattr(file, attr, &mut [])
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        if size == 0 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Empty xattr"));
        }

        let mut buffer = vec![0u8; size];
        let bytes_read = fgetxattr(file, attr, &mut buffer)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        buffer.truncate(bytes_read);

        if let Some(&0) = buffer.last() {
            buffer.pop();
        }

        Ok(buffer)
    }

    /// NSA RTB (Redundant/TPI): Validates the context using two independent paths.
    #[must_use]
    pub fn read_context(file: &File) -> io::Result<SecurityContext> {
        let raw_bytes = Self::read_raw(file, XATTR_NAME_SELINUX)?;
        let context_str = std::str::from_utf8(&raw_bytes)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Non-UTF8 Context"))?;

        // --- PATH A: nom Parser (Declarative) ---
        let (_, context_a) = parse_context_nom(context_str)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "TPI Path A Failed"))?;

        // --- PATH B: FromStr (Imperative) ---
        let context_b: SecurityContext = context_str.parse()
            .map_err(|e: ContextParseError| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;

        // --- TPI GATE: Integrity Check ---
        if context_a != context_b {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "RTB Redundancy Failure: Security Context logic mismatch"
            ));
        }

        Ok(context_a)
    }
}

// ===========================================================================
// TPI Path A Helper Logic
// ===========================================================================

fn parse_context_nom(input: &str) -> IResult<&str, SecurityContext> {
    use std::str::FromStr;

    // 1. Parse user:role:type
    let (input, user_raw) = take_until(":")(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, role_raw) = take_until(":")(input)?;
    let (input, _) = tag(":")(input)?;
    
    let (remaining, type_raw) = match take_until::<&str, &str, nom::error::Error<&str>>(":")(input) {
        Ok((rem, t)) => (rem.strip_prefix(":").unwrap_or(rem), t),
        Err(_) => ("", input), 
    };

    // 2. Parse MlsLevel
    let level = if !remaining.is_empty() {
        let (sens_raw, cats_str) = if remaining == "SystemLow" {
            ("s0", "")
        } else {
            remaining.split_once(':').unwrap_or((remaining, ""))
        };

        let sens = SensitivityLevel::from_str(sens_raw)
            .map_err(|_| nom::Err::Failure(nom::error::Error::new(remaining, nom::error::ErrorKind::Tag)))?;
        
        let cats = if !cats_str.is_empty() {
            parse_mcs_categories(cats_str).map_err(|_| nom::Err::Failure(nom::error::Error::new(cats_str, nom::error::ErrorKind::Tag)))?
        } else {
            CategorySet::new()
        };

        Some(MlsLevel { sensitivity: sens, categories: cats })
    } else {
        None
    };

    // 3. Map to strong types
    let user = crate::user::SelinuxUser::from_str(user_raw).map_err(|_| nom::Err::Failure(nom::error::Error::new(user_raw, nom::error::ErrorKind::Tag)))?;
    let role = crate::role::SelinuxRole::from_str(role_raw).map_err(|_| nom::Err::Failure(nom::error::Error::new(role_raw, nom::error::ErrorKind::Tag)))?;
    let security_type = crate::type_id::SelinuxType::from_str(type_raw).map_err(|_| nom::Err::Failure(nom::error::Error::new(type_raw, nom::error::ErrorKind::Tag)))?;

    Ok(("", SecurityContext::new(user, role, security_type, level)))
}

/// Helper to parse MCS Category strings (e.g., "c0.c3,c10")
/// NIST 800-53 AC-4: Helper to parse MCS Category strings (e.g., "c0.c3,c10")
/// Marked pub(crate) so Path B (context.rs) can use it for TPI agreement.
pub(crate) fn parse_mcs_categories(input: &str) -> io::Result<crate::category::CategorySet> {
    let mut set = crate::category::CategorySet::new();
    
    // If no categories are present (e.g., just "s0"), return empty set.
    if !input.contains('c') {
        return Ok(set);
    }

    for part in input.split(',') {
        if part.contains('.') {
            let range: Vec<&str> = part.split('.').collect();
            if range.len() == 2 {
                let start = parse_cat_id(range[0])?;
                let end = parse_cat_id(range[1])?;
                for i in start..=end {
                    // Extract Category from Result via map_err and ?
                    let cat = crate::category::Category::new(i)
                        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
                    set.insert(cat);
                }
            }
        } else {
            let id = parse_cat_id(part)?;
            let cat = crate::category::Category::new(id)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
            set.insert(cat);
        }
    }
    Ok(set)
}

fn parse_cat_id(s: &str) -> io::Result<u16> {
    s.strip_prefix('c')
     .and_then(|n| n.parse::<u16>().ok())
     .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Invalid Category Format"))
}

