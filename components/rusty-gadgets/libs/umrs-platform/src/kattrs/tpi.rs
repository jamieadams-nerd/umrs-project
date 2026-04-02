// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jamie Adams (a.k.a, Imodium Operator)
//! Two-Path Independent (TPI) validation for SELinux security context strings.
//!
//! Provides `validate_type_redundant`, which extracts the SELinux type field
//! using two independent strategies and fails closed if they disagree.
//!
//! NIST SP 800-53 SI-7: Software and Information Integrity.
//! NSA RTB VNSSA: deterministic, bounded parsing.
//! NSA RTB RAIN: fail-closed on any ambiguity.

use nom::{
    IResult,
    bytes::complete::{tag, take_until},
};
use std::io;

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
    let _u = parts.next().ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "No User"))?;
    let _r = parts.next().ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "No Role"))?;
    let t = parts.next().ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "No Type"))?;
    Ok(t)
}

/// Two-Path Independent (TPI) parser for SELinux security context strings.
///
/// Extracts the SELinux type field using two independent parse strategies
/// (nom declarative parser and imperative split). If they disagree, the
/// function fails closed — guaranteeing that a single parser bug cannot
/// produce a trusted result.
///
/// # Errors
///
/// Returns `io::ErrorKind::InvalidData` if either parse path fails to extract
/// the type field from `context`. Returns `io::ErrorKind::PermissionDenied` if
/// both paths succeed but disagree on the extracted type value.
///
/// NIST SP 800-53 SI-7: Software and Information Integrity.
/// NSA RTB VNSSA: deterministic, bounded parsing.
/// NSA RTB RAIN: fail-closed on any ambiguity.
pub fn validate_type_redundant(context: &str) -> io::Result<&str> {
    let (_, type_a) = parse_type_path_a(context)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Path A Logic Failure"))?;

    let type_b = parse_type_path_b(context)?;

    if type_a != type_b {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "RTB Redundancy Failure: Logic Mismatch Detected",
        ));
    }

    Ok(type_a)
}
