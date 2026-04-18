//! ## Compliance
//!
//! - **NIST SP 800-53 SI-7**: Software and Information Integrity — provenance
//!   verification via fd-anchored `fstatfs` before every kernel attribute read.
//! - **NIST SP 800-53 CA-7**: Continuous Monitoring — `PostureSnapshot` provides
//!   point-in-time kernel security posture with typed contradiction detection.
//! - **NIST SP 800-53 CM-6**: Configuration Settings — posture probe detects
//!   live-vs-configured divergence; Trust Gate prevents reads when subsystem
//!   is inactive.
//! - **NIST SP 800-53 SC-12, SC-28**: Key Management and Protection at Rest —
//!   `SealedCache` uses HMAC-SHA-256 with an ephemeral, boot-session-bound key
//!   that is zeroized on drop.
//! - **NIST SP 800-53 AU-3**: Audit Record Content — `EvidenceBundle` records
//!   what was read, from where, and under what filesystem magic; `ContradictionKind`
//!   is a typed enum enabling machine-readable audit classification.
//! - **NIST SP 800-218 SSDF PW.4**: Secure Coding — compile-time path and magic
//!   binding prevents runtime parameterization of security-critical constants.
//! - **NSA RTB RAIN**: Non-Bypassable — all kernel attribute reads route through
//!   `SecureReader`; the magic check cannot be skipped.
