# Performance Corpus — Concept Index

## Heap Allocation Strategies
- Vec::with_capacity for known sizes — avoids 4→8→16→32 realloc chain
- SmallVec<[T; N]> for short, known-max-length vectors (inline storage)
- ArrayVec when exact max is known (no heap fallback)
- BufRead::read_line with reusable String buffer vs lines() per-line alloc
- Cow<'a, str> for mixed borrowed/owned data — avoids unnecessary clones
- clone_from reuses existing allocations: a.clone_from(&b)
- Workhorse collections: declare outside loop, clear() at end of iteration

## I/O Performance
- BufReader/BufWriter for many small reads/writes — reduces syscalls
- Lock stdout manually for repeated println! calls
- BufRead::read_until for raw bytes — avoids UTF-8 validation overhead
- Combine manual locking AND buffering for high-throughput stdout

## Iterator Optimization
- Avoid collect() if result is only iterated again — return impl Iterator
- Implement size_hint/ExactSizeIterator::len for fewer allocations in collect/extend
- chunks_exact over chunks when divisible — faster
- iter().copied() for small types may generate better code
- filter_map over filter+map — single pass

## Type Size Optimization
- Types >128 bytes use memcpy — shrink hot types below this threshold
- Box outsized enum variants to shrink the whole enum
- Use u32/u16/u8 indices, coerce to usize at use points
- Box<[T]> is 2 words vs Vec<T>'s 3 words — use for frozen collections
- ThinVec: 1-word size, stores len/cap in allocation — good for often-empty vecs
- static_assertions::assert_eq_size! to prevent regression

## Hashing
- Default SipHash 1-3: DoS-resistant but slow for short keys
- FxHashMap (rustc-hash): fast but NOT DoS-resistant — internal data only
- For UMRS: SipHash is correct for security-relevant maps (policy caches, label maps)
- nohash_hasher for random-integer newtypes that don't need hashing

## Build Configuration
- codegen-units = 1 for max optimization
- lto = "fat" for 10-20% runtime improvement
- -C target-cpu=native for AVX/SIMD on known hardware
- Alternative allocators: jemalloc (tikv-jemallocator), mimalloc
- PGO viable for tool binaries (cargo-pgo), not library crates
- panic = "abort" — smaller binary, no unwinding overhead

## Profiling
- perf + flamegraph for CPU hotspots
- DHAT for allocation profiling (hot allocation sites, peak memory)
- Cachegrind for instruction counts and cache simulation
- samply for sampling profiles viewable in Firefox Profiler
- cargo-asm / godbolt for inspecting generated machine code
- debug = "line-tables-only" in release profile for profiling

## Systems Performance (Drepper/Gregg/Agner)
- Cache-friendly data layout: sequential access >> random access
- NUMA awareness for multi-socket systems
- USE method: Utilization, Saturation, Errors — systematic bottleneck identification
- Branch prediction: keep hot paths straight-line, rare paths in else
- Instruction-level parallelism: independent operations can overlap in pipeline
