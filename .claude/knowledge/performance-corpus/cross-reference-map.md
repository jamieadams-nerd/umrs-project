# Performance Corpus — Cross-Reference Map

## UMRS Crate → Applicable Techniques

### umrs-ls (directory walking, xattr reading)
- Vec::with_capacity for directory entry buffers (known from readdir count)
- BufReader wrapping for any file content reads
- Cow for security context strings (mostly borrowed from xattr)
- Workhorse collection reuse across directory entries
- chunks_exact for batch processing of entries
- Profile with DHAT to find hot allocation sites in listing path

### umrs-selinux (security context parsing, MLS math)
- SipHash (default) is CORRECT for SecurityContext maps — DoS-resistant
- Type size assertions for CategorySet ([u64; 16] = 128 bytes — at threshold)
- iter().copied() for CategorySet bit operations on u64 values
- SmallVec potential for small MLS range collections

### umrs-platform (kernel attribute reading, posture probing)
- BufRead::read_line with reusable buffer for /proc/ and /sys/ reads
- String buffer reuse across multiple kernel attribute reads
- Avoid format!() in hot paths — allocates every time

### umrs-core (formatting, i18n, timing)
- Cow<'static, str> for i18n strings (static when untranslated, owned when translated)
- Pre-size format buffers for known-length output

### Tool binaries (umrs-ls CLI, future TUI)
- Lock stdout for batch output
- BufWriter wrapping for file/pipe output
- PGO candidate once hot paths are known
- Consider jemalloc for tools with many small allocations

## Technique → Source Document

| Technique | Primary Source | Chapter/Section |
|---|---|---|
| Vec growth strategy | perf-book | heap-allocations |
| SmallVec/ArrayVec | perf-book | heap-allocations |
| BufReader/BufWriter | perf-book | io |
| Cow clone-on-write | perf-book | heap-allocations |
| Type size optimization | perf-book | type-sizes |
| Hashing alternatives | perf-book | hashing |
| Build config (LTO, PGO) | perf-book | build-configuration |
| Profiling tools | perf-book | profiling |
| Cache behavior | Drepper | cpumemory |
| USE method | Brendan Gregg | linuxperf |
| Instruction scheduling | Agner Fog | optimizing-cpp |
| Criterion benchmarking | criterion-rs | README |
| Flame graphs | flamegraph-rs | README |
