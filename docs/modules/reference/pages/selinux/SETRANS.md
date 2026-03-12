Technical Brief: UMRS High-Assurance Vernacular Translation
Module: umrs_selinux::mcs::setrans
Standard Compliance: NIST 800-53 AC-4, AU-3; NSA RTB (Redundancy & Determinism)
1. The Problem: The Vernacular vs. Ground Truth
In SELinux, the "Ground Truth" stored on disk is often a raw bitmask (e.g., s0:c90,c99). While mathematically sound for the kernel, it is illegible to human operators and auditors who require regulatory markings (e.g., CUI//LEI/INV). Traditional systems rely on the mcstransd daemon, which introduces a non-anchored userspace dependency.
2. The UMRS Solution: TPI-Vetted Translation
The setrans module implements a high-performance, redundant translation engine that ingests the system's setrans.conf and maps kernel bitmasks to human-readable markings with Two-Path Integrity (TPI).
A. Redundant Ingestion (The TPI Gate)
To ensure the translation table hasn't been tampered with or misinterpreted, every line in /etc/selinux/targeted/setrans.conf is parsed by two independent logic paths:
Path A (Declarative): A formal nom parser combinator that enforces the strict grammar of the configuration file.
Path B (Imperative): A robust string-splitting and joining algorithm.
The Invariant: Both paths must agree bit-for-bit on the mapping before an entry is accepted into the trusted map. A single character mismatch fails the ingestion of that line.
B. Lattice-Based Indexing
Unlike standard tools that use string-matching (which is fragile and order-dependent), UMRS indexes the translation table using the CategorySet bitmask.
Order Independence: Because the key is the bitmask, c90,c99 and c99,c90 resolve to the exact same entry.
Deterministic Lookup: The table uses a BTreeMap of bitmasks, ensuring that lookups are 
 and mathematically deterministic.
3. Performance & Caching
The module is designed for massive directory audits (e.g., 100,000+ files) where performance is a security requirement (Availability).
Lazy Loading: Using std::sync::OnceLock, the translation table is parsed exactly once upon first access.
Zero-Copy Lookups: The engine performs 
 bitmask comparisons rather than expensive string comparisons or regex matching.
Static Residence: Once loaded, the map remains in read-only memory, providing thread-safe access to all audit tools (CLI, TUI, or GUI) without lock contention.
4. Audit Fidelity (NIST 800-53 AU-3)
Fail-Safe Fallback: If a file contains a category bitmask not defined in the configuration, the engine returns the Raw Provenance String (e.g., s0:c1023). This ensures the auditor sees the "Unknown" state rather than a silent failure or an empty field.
Traceability: Every successful and failed translation can be traced via the log facade, identifying exactly which bitmask triggered a vernacular match.
5. Future Trajectory
The substrate is architected to bridge into the UMRS CUI Master Catalog. Future iterations will use the translated markings (e.g., CUI//AGR) as a key to pull extended metadata from JSON registries, including:
Visual Fidelity: Dynamic color palettes (e.g., agriculture_green) for UI rendering.
Handling Instructions: Automated display of NARA-mandated safeguarding requirements.
Next Step for the UMRS:
Now that the Vernacular Bridge is secure, we can finalize the 
 Lattice Dominance math to enforce information flow policy (NIST 800-53 AC-4).

