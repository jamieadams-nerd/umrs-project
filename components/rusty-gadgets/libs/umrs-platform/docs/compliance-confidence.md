//! ## Compliance
//!
//! - **NIST SP 800-53 SA-9**: External Information System Services — trust in
//!   any external data source must be explicit, graded, and auditable. Every
//!   claim about the platform's identity is bound to a `TrustLevel` that
//!   reflects how many independent verification steps backed it.
//! - **NIST SP 800-53 CM-6**: Configuration Settings — the system's security
//!   posture depends on knowing the configuration accurately; the trust tier
//!   communicates the confidence in that knowledge.
//! - **NSA RTB**: Trust assertions must be traceable to a kernel-anchored
//!   evidence source. `TrustLevel::Untrusted` is the safe default; anything
//!   higher must be earned by passing the corresponding verification gate.
//!
