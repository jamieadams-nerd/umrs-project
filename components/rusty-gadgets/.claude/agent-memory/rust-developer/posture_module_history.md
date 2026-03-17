---
name: posture module history
description: Terminology changes and key design decisions in umrs-platform posture module
type: project
---

## Signal → Indicator rename (2026-03-17, Phase 2b.0)

All "Signal" terminology in the posture module was renamed to "Indicator".

**Mapping:**
- `SignalId` → `IndicatorId`
- `SignalReport` → `IndicatorReport`
- `SignalDescriptor` → `IndicatorDescriptor`
- `SignalClass` → `IndicatorClass`
- `signal.rs` → `indicator.rs`
- `SIGNALS` const → `INDICATORS` const

**Files changed (14):**
- `umrs-platform/src/posture/indicator.rs` (was signal.rs)
- `umrs-platform/src/posture/mod.rs`
- `umrs-platform/src/posture/catalog.rs`
- `umrs-platform/src/posture/snapshot.rs`
- `umrs-platform/src/posture/reader.rs`
- `umrs-platform/src/posture/contradiction.rs`
- `umrs-platform/src/posture/fips_cross.rs`
- `umrs-platform/src/posture/configured.rs` (import only)
- `umrs-platform/src/posture/modprobe.rs` (import only)
- `umrs-platform/src/lib.rs`
- `umrs-platform/examples/posture_demo.rs`
- `umrs-platform/tests/posture_tests.rs`
- `umrs-platform/tests/posture_modprobe_tests.rs`
- `umrs-platform/tests/posture_bootcmdline_tests.rs`
- `umrs-platform/tests/posture_fips_tests.rs`
- `umrs-tui/src/main.rs`

**Why:** Jamie approved — "Indicator" is more precise terminology for posture assessment.

**How to apply:** When adding new posture module types or signals, use `IndicatorId`, `IndicatorDescriptor`, `IndicatorReport`, `IndicatorClass`, and `INDICATORS` — never the old Signal names.

## Posture module architecture (stable as of Phase 2b)

- `indicator.rs`: core types (IndicatorId enum, IndicatorClass, AssuranceImpact, DesiredValue, LiveValue, ConfiguredValue)
- `catalog.rs`: static INDICATORS array of IndicatorDescriptor — compile-time, no runtime I/O
- `reader.rs`: live-value readers via SecureReader; `read_live_sysctl(id: IndicatorId)` dispatches by enum
- `snapshot.rs`: PostureSnapshot::collect() → Vec<IndicatorReport>; get(id: IndicatorId) lookup
- `contradiction.rs`: classify(live_meets, configured_meets) → ContradictionKind
- `fips_cross.rs`: FIPS cross-check (Trust Gate pattern, multi-source)
- `configured.rs`: sysctl.d merge tree configured-value reading
- `bootcmdline.rs`: BLS entry reader for configured kernel cmdline
- `modprobe.rs`: modprobe.d parsing, module-load detection

## Key invariants
- `PostureSnapshot` is the public-facing type; consumers call `collect()` and iterate reports
- `IndicatorId` is `Copy + Hash` — usable as map key
- Blacklist signals: `DesiredValue::Exact(1)` with `"blacklisted"` sentinel in ConfiguredValue.raw
- KernelCmdline contradiction detection uses `meets_cmdline()` on BLS options string, NOT `evaluate_configured_meets()`
- `CorePattern` uses `DesiredValue::Custom` and TPI validation in reader.rs
