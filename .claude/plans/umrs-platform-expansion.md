---
  name: Scope of umrs-platform to expand for new detection items
  path: components/rusty-gadgets/umrs-platform
  agent: rusty-developer
  status: reeady to review
---

## umrs-platform package/crate 
- Currently contains OS detection 
- has some kernel detection to identify capabilities
- PUblic interface acts most other interfaces to get basic information
  - Tell me if capable? is Selinux Capable?
  - Give me the operating system name and version
  - However, the detailed trust checks an evidence doesn't complicate usage
  - The trust checks and evicdence chain does give the public caller a level of assurance and
    evidence if needed.
  - For novice or intermediate programmers, if possible a single createion of an object and then be able
    to query it for stuff. They heed abstraction and ease of use. 
    - For example, they just want to know if a package is installed?
    - What's the version of this OS
    - Things like the OsDetector:: is **GREAT**! Love public facing detectors like this.
    - Simple for them and keep the detailed, advanced stuff we have for experienced programmers.

## New Capabilities
- We might want to consider refactoring to facilitate easy to read or manage.
- OS Detection is present
- Kernel attributes (kattrs) might want to be "kernel" contains reading/parsing
  - Also contains some existing stuctures and objects to query
  - The base structure will expand to include storage for evidence and level of trust
- CPU Extension detection will be in the future
  - Tools to query if security, high-assurance, or cryptographic extensions are avaialble.
  - These can be used to audit ELF binaries to see if things are linked using extensions

## Additional Kernel Flags to check
- These settings have an impact to high-assurance/security posture of system
- Read .claude/kernel-flags absorb information
  - Consider an external json for rich data for runtime and ifnroomation
  - Create a plan out of the discussion 
  - Save the plan in .claude/plans for later review and discussion.
- A public function should be present to give a list of settings with an iterator
- easy to use public interfaces
- Plenty of log::debug!() to show what is going on for developers and auditors.

## Future: DetectionResult Serialization Layer
- The SEC pattern's `decode_cached_result()` currently re-runs the pipeline on cache hit because `DetectionResult` has no serialization impl.
- A future iteration should add a serialization/deserialization layer for `DetectionResult` so that verified cache hits return the stored result directly — avoiding the pipeline re-run entirely.
- This is the key remaining step to realize the full performance benefit of SEC.
- Requires careful design: `DetectionResult` contains `OsRelease` (with validated newtypes), `EvidenceBundle` (append-only), `ConfidenceModel`, and `SubstrateIdentity`.
- Consider a custom binary format or `serde` with a compact binary codec (e.g., `postcard`).

## DO NOT CHANGE ANY CODE Right NOW.
- Just develop a plan
- And save it, ask questions, and we will keep it in the queue.




