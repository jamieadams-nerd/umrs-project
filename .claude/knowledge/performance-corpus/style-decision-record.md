# Performance Corpus — Style Decision Record

## Decisions informed by the performance corpus

### D1: SipHash remains the default hasher for all UMRS maps
**Rationale:** The corpus confirms FxHashMap is NOT DoS-resistant. UMRS maps
may hold data derived from SELinux policy files (untrusted input boundary).
Switching to a fast but non-resistant hasher would create an attack surface.
**Source:** perf-book/hashing — "HashDoS attacks are not a concern" is a
precondition for using alternative hashers. UMRS cannot make this claim.

### D2: Allocation profiling before optimization, not intuition
**Rationale:** The corpus consistently emphasizes measurement over intuition.
DHAT identifies hot allocation sites with precision. Do not optimize
allocations without profiler evidence.
**Source:** perf-book/heap-allocations, perf-book/benchmarking

### D3: forbid(unsafe_code) constrains optimization paths — and that's correct
**Rationale:** The corpus identifies get_unchecked as the bounds-check escape
hatch. UMRS cannot use it. Instead, structure iterator chains so the compiler
can prove bounds (zip, enumerate, chunks_exact). This is slower in some cases
but maintains the security guarantee.
**Source:** perf-book/bounds-checks, perf-book/iterators

### D4: BufReader/BufWriter are mandatory for file I/O
**Rationale:** Rust file I/O is unbuffered by default. Every small read/write
is a syscall. For directory walking and kernel attribute reads, this is
directly applicable.
**Source:** perf-book/io

### D5: PGO is a medium-term tool binary optimization
**Rationale:** cargo-pgo automates the workflow but requires representative
workloads. Not actionable until hot paths are known through profiling.
**Source:** perf-book/build-configuration, rustc-pgo
