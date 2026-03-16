# Performance Corpus — Term Glossary

| Term | Definition | Context |
|---|---|---|
| DHAT | Dynamic Heap Analysis Tool — Valgrind-based profiler for allocation sites, rates, sizes, lifetimes | Use to find hot allocation points |
| LTO | Link-Time Optimization — whole-program optimization across crate boundaries | fat LTO gives 10-20% speedup |
| PGO | Profile-Guided Optimization — compile, run with profiling, recompile with data | 10%+ improvement for binaries |
| SmallVec | Inline-storage vector that spills to heap above N elements | Reduces allocation rate for short vectors |
| ArrayVec | Fixed-capacity vector, never heap allocates, panics on overflow | When exact max length is known |
| ThinVec | Single-word vector storing len/cap in the heap allocation | Good for often-empty vectors in structs |
| Cow | Clone-on-Write — holds borrowed or owned data, clones only on mutation | Avoids unnecessary allocations |
| USE method | Utilization, Saturation, Errors — systematic performance analysis | Brendan Gregg methodology |
| Flame graph | Visualization of stack traces weighted by sample count | CPU hotspot identification |
| codegen-units | Number of parallel compilation units per crate | 1 = best optimization, slowest compile |
| SipHash | Default Rust hasher — DoS-resistant, moderate speed | Correct for security-relevant maps |
| FxHash | Fast non-cryptographic hasher from rustc — NOT DoS-resistant | Internal-only, non-adversarial data |
| cargo-asm | Tool to inspect generated assembly for Rust functions | Verify compiler optimization |
| Criterion | Statistical microbenchmarking framework for Rust | Stable Rust alternative to #[bench] |
| size_hint | Iterator method that reports expected remaining length | Helps collect/extend pre-allocate |
| chunks_exact | Slice iterator that guarantees exact chunk size | Faster than chunks when divisible |
| BufReader | Buffered wrapper for Read — reduces syscalls | Mandatory for repeated small reads |
| BufWriter | Buffered wrapper for Write — reduces syscalls | Mandatory for repeated small writes |
| memcpy threshold | Types >128 bytes copied via memcpy instead of inline | Shrink hot types below this |
| NUMA | Non-Uniform Memory Access — memory latency varies by socket | Drepper: locality matters |
