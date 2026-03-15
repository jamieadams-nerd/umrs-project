# umrs-logspace Design Reference

**Created:** 2026-03-15
**Source:** `.claude/jamies_brain/logging_space.txt`
**ROADMAP Goals:** G5 (Security Tools)
**Status:** Existing crate, design notes preserved for future work

---

## Summary

umrs-logspace is a **log storage capacity monitoring tool** — like `df` for log tiers.
NOT an event analysis tool (that's a separate concept).

## Core Concept

Models log storage as **resource pools** with lifecycle stages:

```
active → inactive → archive
```

Each pool is a TOML-configured logical storage tier with capacity constraints.

## Configuration

TOML-driven. Example:

```toml
[[pool]]
name = "active"
path = "/var/log/umrs/active"
description = "Active event logs"

[[pool]]
name = "inactive"
path = "/var/log/umrs/inactive"
description = "Rotated logs awaiting archival"

[[pool]]
name = "archive"
path = "/vaults-lei/log-archive"
description = "Long term log archive"
```

## Output Model (mirrors `df`)

```
POOL        PATH                        SIZE   USED   AVAIL  USE%
active      /var/log/umrs/active        10G    4.2G   5.8G   42%
inactive    /var/log/umrs/inactive      20G    8.1G   11.9G  40%
archive     /vaults-lei/log-archive     200G   63G    137G   31%
```

## Current Status

- Crate exists at `components/rusty-gadgets/umrs-logspace/`
- Parses TOML config
- Enumerates pools
- Reports storage statistics

## Remaining Work

- Formatting improvements
- Error handling
- Threshold warnings (>80% = WARN)
- Colorized output (green/yellow/red by utilization)
- JSON output (`--json`)
- Trend tracking (growth rate)
- Retention forecasting ("archive fills in N days")
- CLI flags
- Configuration validation

## Open Design Question

Pool usage calculation method:
- A) Filesystem stats (statvfs/statfs) — whole-filesystem view
- B) Directory walking (du-style summation) — pool-specific usage
- Likely: support both

## Relationship to UMRS Logging

| Component | Role |
|-----------|------|
| journald | Primary system log ingress |
| UMRS mediator | Event normalization |
| event JSONL store | Structured event persistence |
| logspace utility | Storage capacity monitoring |
