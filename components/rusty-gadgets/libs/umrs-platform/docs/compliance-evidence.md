//! ## Compliance
//!
//! - **NIST SP 800-53 AU-3**: Audit Record Content — records capture what was
//!   read, when, from where, and with what outcome.
//! - **NIST SP 800-53 AU-10**: Non-Repudiation — the evidence bundle is the
//!   authoritative record of every artifact the detection pipeline consumed.
//!   Callers cannot remove or modify records once pushed.
//! - **NSA RTB**: provenance must be traceable. Every `EvidenceRecord` carries
//!   the path requested, the resolved path (if different), and the filesystem
//!   magic observed — enough to reconstruct the decision chain post-incident.
