//! ## Compliance
//!
//! - **NIST SP 800-53 SC-28**: Protection of information at rest — the HMAC seal
//!   provides integrity protection while the result resides in the in-memory cache.
//! - **NIST SP 800-53 SC-12**: Cryptographic key management — the sealing key is
//!   ephemeral, boot-session-bound, never persisted, and zeroized on drop.
//! - **NIST SP 800-53 SI-7**: Software and information integrity — seal verification
//!   detects substitution; seal failure triggers re-verification.
//! - **NIST SP 800-53 AU-3**: Audit record content — seal failures are logged with
//!   `log::warn!` producing auditable anomaly records.
//! - **NIST SP 800-218 SSDF PW.4**: Secure coding — fail-closed behavior (PW.4.1),
//!   ephemeral key management to limit exposure.
//! - **FIPS 180-4**: Secure Hash Standard — SHA-256 used for HMAC-based
//!   cache integrity verification.
