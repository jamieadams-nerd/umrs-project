# Source Tracking: rust-performance Collection

**Collection:** performance-corpus/rust-performance
**Retrieved:** 2026-03-16
**Purpose:** Rust-specific performance engineering reference for rust-developer agent

## Documents

### 1. The Rust Performance Book
- **Source URL:** https://github.com/nnethercote/perf-book
- **Retrieved from:** https://raw.githubusercontent.com/nnethercote/perf-book/master/src/
- **Format:** Markdown (individual chapter files from GitHub raw source)
- **Chapters fetched:** introduction, benchmarking, build-configuration, type-sizes, heap-allocations, iterators, machine-code, profiling, inlining, hashing, io, parallelism, standard-library-types, bounds-checks, compile-times, general-tips, linting, logging-and-debugging, wrapper-types, SUMMARY
- **File naming:** perf-book-{chapter}.md
- **Update check URL:** https://github.com/nnethercote/perf-book/commits/master

### 2. Rust Compiler (rustc) Profiling Guide
- **Source URL:** https://github.com/rust-lang/rustc-dev-guide
- **Retrieved from:**
  - https://raw.githubusercontent.com/rust-lang/rustc-dev-guide/master/src/profiling.md
  - https://raw.githubusercontent.com/rust-lang/rustc-dev-guide/master/src/profiling/with-perf.md
  - https://raw.githubusercontent.com/rust-lang/rustc-dev-guide/master/src/profile-guided-optimization.md
- **Format:** Markdown
- **Files:** rustc-profiling-guide.md, rustc-profiling-with-perf.md, rustc-profile-guided-optimization.md
- **Update check URL:** https://github.com/rust-lang/rustc-dev-guide/commits/master/src/profiling.md

## Notes

- The Rust Performance Book is an mdbook (JS-rendered). Individual chapter source files were fetched from GitHub raw to avoid JS rendering.
- The original bheisler/criterion.rs repo has been abandoned; active development moved to criterion-rs/criterion.rs. The new README is in the profiling/ collection.
