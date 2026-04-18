//! ## Compliance
//!
//! - **NIST SP 800-53 CM-8**: Information System Component Inventory —
//!   component identity must be accurate, typed, and independently verifiable.
//!   These types represent the inventory-level facts derived from the package
//!   substrate, which is the most authoritative non-kernel identity source.
//! - **NIST SP 800-53 SA-12**: Supply Chain Risk Management — package
//!   substrate probes derive identity from the same database that tracks
//!   installed software provenance. A mismatch between substrate identity
//!   and `os-release` is a supply chain integrity signal.
//! - **NSA RTB**: identity must be derived from multiple independent facts.
//!   `SubstrateIdentity::facts_count` records how many independent facts
//!   were corroborated; T3 (`SubstrateAnchored`) requires ≥2.
