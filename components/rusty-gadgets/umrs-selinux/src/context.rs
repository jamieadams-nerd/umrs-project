//!
//! `SELinux` Security Context 
//!
//!  Author: Jamie Adams
//!  License: MIT
//!
//!   Strongly-typed Rust primitives modeling `SELinux` MLS security context and 
//!   dominance semantics.
//!
//! ## Implementation Lineage & Design Note:
//! This module provides an independent, original implementation functionality conceptually 
//! comparable to traditional userland libraries. Behavioral interfaces and operational 
//! semantics were studied ensure familiarity for long-time `SELinux` developers.
//! No source code has been copied or translated, and line-by-line reimplementation 
//! was performed. Where appropriate, this implementation takes advantage of RUST language 
//! features such as strong typing, validation at and memory safety guarantees to improve
//! and assurance beyond legacy approaches.
//!


