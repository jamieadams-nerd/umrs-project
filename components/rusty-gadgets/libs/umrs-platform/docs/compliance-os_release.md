//! ## Compliance
//!
//! - **NIST SP 800-53 CM-8**: Information System Component Inventory —
//!   component identity fields must be accurately typed. Untyped string soup
//!   allows silent conflation of unrelated fields.
//! - **NIST SP 800-53 SI-10**: Information Input Validation — all field values
//!   are validated at construction. Callers cannot obtain an `OsId`, `VersionId`,
//!   or `CpeName` from an input that fails the field's structural rules.
//! - **NIST SP 800-53 SI-12**: Information Management and Retention — error
//!   payloads truncated to 64 characters at log call sites (not here) to
//!   prevent log flooding with user-controlled content.
