---
name: umrs-c2pa evaluation findings
description: First-time consumer assessment of umrs-c2pa library and CLI (2026-04-01)
type: project
---

# UMRS C2PA Evaluation Summary

**Date:** 2026-04-01
**Status:** Complete report written to `.claude/api-reports/2026-04-01-umrs-c2pa-developer-eval.md`

## Key Patterns Worth Preserving

### Error Type Design
- `InspectError` is an enum with variants matching failure classes, NOT a generic string wrapper
- Each variant (Io, C2pa, Config, Signing, UnsafeAlgorithm, AlreadySigned) is matchable
- Manual `impl Display` allows future i18n integration via gettext
- This is the high-assurance error pattern to replicate in other crates

### High-Assurance Function Contracts
- All public functions returning `Result`, `Option`, or security-relevant types carry `#[must_use]`
- Messages explain the specific risk of discarding (e.g., "TrustStatus carries the trust evaluation... discarding it means trust decisions are silently bypassed")
- This is more precise than generic `#[must_use]` without explanation

### JSON Output Strategy
- Provide two modes: raw C2PA SDK output (`--json`, detailed-json`) + UMRS-processed (`--chain-json`)
- Processed output is flat, minimal, suitable for piping to tools
- Raw output is comprehensive but requires schema knowledge
- ChainEntry struct maps directly to JSON fields for consistency

### Trust Validation Non-Bypassability
- All manifest reads route through `build_c2pa_settings()` + `c2pa::Context`
- This function is non-bypassable; it's the sole entry point for trust validation
- Pattern name: NSA RTB RAIN (non-bypassable security checks)

### Ephemeral Mode Discipline
- When credentials are missing, the tool generates self-signed ephemeral certs at runtime
- Output is explicitly marked as UNTRUSTED/UNVERIFIED
- Blocks production use but enables testing — clear guard rails

## Documentation Gaps to Flag in Similar Crates

1. **JSON schema is undocumented** — when a tool outputs JSON, provide field reference or example with all possible fields
2. **Library API not called out separately** — if a crate is both a library and has a binary, the Antora page should link to rustdoc and show minimal integration example
3. **Verbose entry point not mentioned** — if `enable_verbose()` is exported, it should be listed in module docs

## Friction Points for First-Time Users

- `--detailed-json` flag is documented but broken (clap integration issue) — this creates docs/code mismatch
- TOML config is required for production; ephemeral mode is test-only — doc should make this clear upfront
- Trust list paths are configurable but complex (two separate files, combine logic in code) — well-documented, no issue here

## API Fitness Assessment

| Layer | Readiness | Notes |
|-------|-----------|-------|
| Library | 95% | Clear API, strong types, non-bypassable validation. Add: builder for config, rustdoc link in Antora. |
| CLI | 70% | Works for common ops. Fix: --detailed-json flag, JSON schema doc, verbose on subcommands. |
| Tests | 100% | 40/40 tests pass, clippy clean, all high-assurance patterns covered. |
| Docs | 80% | Man page + Antora detailed. Gap: library guide, JSON reference, --detailed-json regression. |

## Why: Controlling Trust at the API Boundary

The pattern of gating all manifest reads through a single settings builder (`build_c2pa_settings`) is a specific instance of **compile-time path binding** (high_assurance_pattern_rules.md). The caller cannot bypass trust validation because:

1. `read_chain()` requires `&UmrsConfig`
2. Config is loaded/validated upfront
3. Trust settings are derived from config
4. Reader is always created with those settings
5. No parallel path exists to read a manifest without settings

This is NSA RTB RAIN: the trust decision is non-bypassable because the architecture makes it impossible to skip.

## Recommendation for Next Session

When the `--detailed-json` flag is fixed (or removed from docs), that's a good time to add:
- JSON field reference in Antora
- Library integration guide in Antora (with rustdoc link)
- Note in man page about why trust lists are critical (trust_status depends on them)
