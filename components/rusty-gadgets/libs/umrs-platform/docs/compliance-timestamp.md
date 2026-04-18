//! ## Compliance
//!
//! - **NIST SP 800-53 AU-3**: Audit Record Content — timestamps provide the
//!   temporal ordering and precision required for complete, non-repudiable audit
//!   records in MLS environments.
//! - **NIST SP 800-53 AU-8**: Time Stamps — high-resolution monotonic source
//!   ensures audit record sequence is preserved without NTP correction interference.
//! - **NIST SP 800-53 AU-12**: Audit Record Generation — each call to
//!   [`BootSessionTimestamp::now`] generates a unique, ordered reference point
//!   suitable for event sequencing.
//! - **NSA RTB**: Deterministic Execution — fixed-size stack-allocated types
//!   with no heap allocation; arithmetic uses checked operations to prevent
//!   silent overflow.
