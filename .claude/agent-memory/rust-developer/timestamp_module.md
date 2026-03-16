---
name: timestamp_module
description: BootSessionTimestamp and BootSessionDuration — nanosecond audit ordering in umrs-platform
type: project
---

## BootSessionTimestamp module (implemented 2026-03-16)

Module: `umrs-platform/src/timestamp.rs`
Tests: `umrs-platform/tests/timestamp_tests.rs` (17 tests, all passing)
Example: `umrs-platform/examples/timestamp_demo.rs`
Public re-exports from `lib.rs`: `BootSessionTimestamp`, `BootSessionDuration`, `TimestampError`

**Clock source**: `CLOCK_MONOTONIC_RAW` via `rustix::time::clock_gettime(ClockId::MonotonicRaw)`
- Hardware clocksource rate, no NTP slewing
- Architecture-independent (x86_64 + AArch64 RHEL 10)
- Returns `Timespec` directly (not `io::Result`) in rustix 0.38

**New rustix feature**: `time` added to umrs-platform's rustix dep (`["fs", "process", "system", "time"]`)
- No new transitive dependencies — feature has empty dep list in rustix 0.38
- Documented in Cargo.toml comment

**Why not RDTSC**: requires `unsafe` and is x86-only — both prohibited.
**Why not `std::time::Instant`**: backed by CLOCK_MONOTONIC (NTP-slewed); insufficient for sub-nanosecond ordering.

**TimestampError variants**: `NegativeSecs(i64)`, `NegativeNsecs(i64)`, `Overflow`
- `clock_gettime` in rustix 0.38 returns `Timespec` (not Result), so no ClockGettime variant needed
- Fail-closed: reject negative tv_sec or tv_nsec

**Arithmetic**: `checked_sub` in `elapsed_since`, `checked_mul`+`checked_add` in `now()`, `checked_add` on Duration
**Ordering**: implements `Ord`/`PartialOrd` on both types via derive — compares nanosecond `u64` values
**Display**: `BootSessionTimestamp` → `T+{secs}.{nanos_rem:09}s`; `BootSessionDuration` → `{micros}.{nanos_rem:03} µs`

**Compliance**: NIST SP 800-53 AU-3, AU-8, AU-12; NSA RTB Deterministic Execution

**Validity scope**: single boot session only. Do not compare across reboots.
