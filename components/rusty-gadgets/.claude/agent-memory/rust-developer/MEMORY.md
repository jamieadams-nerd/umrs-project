# Persistent Agent Memory — rust-developer

- [Project structure](project_structure.md) — Workspace layout, crate list, key conventions, i18n pattern
- [umrs-c2pa API surface](umrs_c2pa_api.md) — Chain report functions, IngestResult fields, public API exports
- [Performance baselines](perf_baselines.md) — Criterion results for umrs-core, umrs-selinux, umrs-platform; key ns/µs figures for regression detection
- [Debug instrumentation](feedback_debug_instrumentation.md) — `start` not `_start` in cfg(debug_assertions) blocks; SI-11 log discipline; const fn rule; pre-existing setrans_tests failures
