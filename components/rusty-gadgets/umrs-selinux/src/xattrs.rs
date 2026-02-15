// ============================================================================
// UMRS SELINUX: Extended Attribute (xattr) Logic
// NIST 800-53 AC-3 / NSA RTB (Non-Bypassability & Redundancy)
// ============================================================================
use nom::{
    bytes::complete::{tag, take_until},
    IResult,
};
use rustix::fs::fgetxattr;
use std::fs::File;
use std::io;
use std::time::Instant;

use crate::context::{ContextParseError, MlsLevel, SecurityContext};

/// The standard SELinux xattr name (NIST 800-53 AU-3: Source Identifier)
pub const XATTR_NAME_SELINUX: &str = "security.selinux";

pub struct SecureXattrReader;

impl SecureXattrReader {
    /// NIST 800-53 SI-7: High-Assurance xattr retrieval via raw syscalls.
    pub fn read_raw(file: &File, attr: &str) -> io::Result<Vec<u8>> {
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
    pub fn read_context(file: &File) -> io::Result<SecurityContext> {
        let start_time = Instant::now();
        let raw_bytes = Self::read_raw(file, XATTR_NAME_SELINUX)?;
        let context_str = std::str::from_utf8(&raw_bytes).map_err(|_| {
            io::Error::new(io::ErrorKind::InvalidData, "Non-UTF8 Context")
        })?;

        // --- PATH A: nom Parser (Declarative) ---
        let (_, context_a) = parse_context_nom(context_str).map_err(|e| {
            log::error!("TPI Path A (nom) failed: {:?}", e);
            io::Error::new(io::ErrorKind::InvalidData, "TPI Path A Failed")
        })?;

        // --- PATH B: FromStr (Imperative) ---
        let context_b: SecurityContext =
            context_str.parse().map_err(|e: ContextParseError| {
                log::error!("TPI Path B (FromStr) failed: {}", e);
                io::Error::new(io::ErrorKind::InvalidData, e.to_string())
            })?;

        // --- TPI GATE: Integrity Check (Loud Mismatch) ---
        if context_a != context_b {
            log::error!("CRITICAL: RTB Redundancy Failure!");
            log::error!("PATH A OBJECT: {:?}", context_a);
            log::error!("PATH B OBJECT: {:?}", context_b);
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "RTB Redundancy Failure: Security Ctx logic mismatch",
            ));
        }

        let duration = start_time.elapsed();
        log::debug!(
            "RTB Redundancy Success: Security Ctx logic match in {:?}",
            duration
        );

        Ok(context_a)
    }
}

// ===========================================================================
// TPI Path A Helper Logic
//
// NOM Parser - Next Generation Object Manipulators
//
// In high-assurance engineering, NOM is a Parser Combinator library.
// To an architect, it is the difference between "string-splitting"
// (brute force) and "formal grammar" (surgical precision).
//
// NIST 800-53 AC-4: Redundant (Path A) MLS Parser
// ===========================================================================
fn parse_context_nom(input: &str) -> IResult<&str, SecurityContext> {
    use crate::category::CategorySet;
    use crate::role::SelinuxRole;
    use crate::sensitivity::SensitivityLevel;
    use crate::type_id::SelinuxType;
    use crate::user::SelinuxUser;
    use std::str::FromStr;

    // 1. Parse user:role
    let (input, user_raw) = take_until(":")(input)?;
    let (input, _) = tag(":")(input)?;

    let (input, role_raw) = take_until(":")(input)?;
    let (input, _) = tag(":")(input)?;

    // 2. Greedy Decompose for the Level
    // Handles multi-part contexts found in RHEL 10 MCS
    let (remaining_after_type, type_raw) =
        match take_until::<&str, &str, nom::error::Error<&str>>(":")(input) {
            Ok((rem, t)) => (rem.strip_prefix(":").unwrap_or(rem), t),
            Err(_) => ("", input),
        };

    log::debug!("[PATH A] Raw Type: '{}', Level remainder: '{}'", type_raw, remaining_after_type);

    // 3. Level Parsing (Sensitivity + Categories)
    let level = if !remaining_after_type.is_empty() {
        let (sens_raw, cats_str) = remaining_after_type
            .split_once(':')
            .unwrap_or((remaining_after_type, ""));

        let sens = match SensitivityLevel::from_str(sens_raw) {
            Ok(s) => s,
            Err(_) => SensitivityLevel::new(0).unwrap(),
        };

        let cats = if !cats_str.is_empty() {
            parse_mcs_categories(cats_str).map_err(|_| {
                nom::Err::Failure(nom::error::Error::new(
                    cats_str,
                    nom::error::ErrorKind::Tag,
                ))
            })?
        } else {
            CategorySet::new()
        };

        Some(MlsLevel {
            sensitivity: sens,
            categories: cats,
            raw_level: remaining_after_type.to_string(),
        })
    } else {
        None
    };

    // 4. Map to Strong Types
    let user = SelinuxUser::from_str(user_raw).map_err(|_| {
        nom::Err::Failure(nom::error::Error::new(user_raw, nom::error::ErrorKind::Tag))
    })?;

    let role = SelinuxRole::from_str(role_raw).map_err(|_| {
        nom::Err::Failure(nom::error::Error::new(role_raw, nom::error::ErrorKind::Tag))
    })?;

    let security_type = SelinuxType::from_str(type_raw).map_err(|_| {
        nom::Err::Failure(nom::error::Error::new(type_raw, nom::error::ErrorKind::Tag))
    })?;

    Ok(("", SecurityContext::new(user, role, security_type, level)))
}

// ===========================================================================
// Shared Category Parser
// NIST 800-53 AC-4: Helper to parse MCS Category strings (e.g., "c0.c3,c90")
// ===========================================================================
pub(crate) fn parse_mcs_categories(
    input: &str,
) -> io::Result<crate::category::CategorySet> {
    let mut set = crate::category::CategorySet::new();

    if !input.contains('c') {
        return Ok(set);
    }

    for part in input.split(',') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }

        if part.contains('.') {
            let range: Vec<&str> = part.split('.').collect();
            if range.len() == 2 {
                let start = parse_cat_id(range[0])?;
                let end = parse_cat_id(range[1])?;
                for i in start..=end {
                    let cat = crate::category::Category::new(i).map_err(|e| {
                        io::Error::new(io::ErrorKind::InvalidData, e.to_string())
                    })?;
                    set.insert(cat);
                }
            }
        } else {
            let id = parse_cat_id(part)?;
            let cat = crate::category::Category::new(id).map_err(|e| {
                io::Error::new(io::ErrorKind::InvalidData, e.to_string())
            })?;
            set.insert(cat);
        }
    }
    Ok(set)
}

fn parse_cat_id(s: &str) -> io::Result<u16> {
    s.trim_start_matches('c')
        .parse::<u16>()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))
}

