// ============================================================================
//! ## Architectural Note: Global Translator State
//!
//! The current implementation exposes a process-wide singleton translator via:
//!
//! ```text
//! GLOBAL_TRANSLATOR: LazyLock<RwLock<Translator>>
//! ```
//!
//! This design reflects the operational reality that SELinux `setrans.conf`
//! mappings are system-wide, loaded once, and shared across all consumers.
//!
//! However, alternative architectures have been considered:
//!
//! ### 1. Dependency Injection Model
//! The translator could be instantiated explicitly and passed through call
//! chains rather than accessed globally. This would eliminate global mutable
//! state and simplify deterministic testing, at the cost of ergonomics.
//!
//! ### 2. Read-Only Freeze Model
//! The translator could be loaded once into an immutable structure
//! (e.g., `OnceLock<Translator>`) with no post-load mutation capability,
//! improving high-assurance posture and eliminating locking overhead.
//!
//! At present, the mutable global model provides the best balance of
//! simplicity, performance, and operational alignment. Future revisions
//! may revisit this decision based on deployment feedback or assurance
//! requirements.
//!
//! Architectural feedback is welcome.
// ============================================================================
