= FHS 2.3 Migration — Session Close
:date: 2026-04-16
:owner: Knox (security-engineer)
:status: session complete; follow-ups tracked

## One-line summary

UMRS on-disk layout is now FHS 2.3 compliant. Installer, staging, Rust
defaults, and deployment docs all agree on the new paths. SELinux policy,
CLI audit implementation, and cross-tool doc sweep remain as tracked
follow-ups.

## What changed

### Layout

Pre-FHS (wrong) → FHS 2.3 (correct):

[cols="2,2,1",options="header"]
|===
| Pre-FHS path | FHS path | FHS reference

| `/opt/umrs/etc/` | `/etc/opt/umrs/` | §3.7.4
| `/opt/umrs/var/lib/` | `/var/opt/umrs/lib/` | §4.9.2
| `/opt/umrs/var/log/` | `/var/opt/umrs/log/` | §4.9.2
| `/opt/umrs/etc/umrs/us/US-CUI-LABELS.json` | `/opt/umrs/share/umrs/US-CUI-LABELS.json` | §4.11, §5.15
| `/opt/umrs/etc/umrs/ca/CANADIAN-PROTECTED.json` | `/opt/umrs/share/umrs/CANADIAN-PROTECTED.json` | §4.11, §5.15
| (no defined home) | `/opt/umrs/share/umrs/templates/` | §4.11
|===

The `us/` and `ca/` subdirectories in the reference database tree were
also eliminated — the layout is now flat: four JSONs directly under
`/opt/umrs/share/umrs/`, two templates under `templates/`.

### Files modified

Code:

- `components/rusty-gadgets/xtask/src/stage.rs` — new `classify_config_file()`; FHS doc citations
- `components/rusty-gadgets/xtask/tests/stage_test.rs` — new classifier test
- `components/rusty-gadgets/scripts/umrs-install.sh` — full rewrite for three-root FHS layout
- `components/rusty-gadgets/umrs-label/src/main.rs` — new default paths
- `components/rusty-gadgets/umrs-ls/src/main.rs` — new default paths
- `components/rusty-gadgets/umrs-label/tests/{catalog,setrans}_tests.rs`, `umrs-ls/tests/marking_lookup_tests.rs`, `umrs-label/examples/{label_registry,labels}.rs` — flattened `config/` paths

Docs:

- `docs/modules/deployment/pages/rhel/rhel10-directory-structure.adoc` — full rewrite
- `docs/modules/umrs-label/pages/index.adoc` — Mermaid + tables + examples updated
- `components/rusty-gadgets/umrs-label/docs/umrs-label.1` (EN) + `docs/fr/umrs-label.1` (FR)
- `.claude/agent-memory/security-engineer/permissions.md` — install sequence

Data:

- `components/rusty-gadgets/umrs-label/config/US-CUI-HANDLING.json` → `_scratch/` (unused)

### Verification

- `cargo xtask fmt` — clean
- `cargo xtask clippy` — clean
- `cargo xtask stage` — produces correct FHS layout; 4 databases + 2 templates staged
- `cargo xtask test` — all suites pass EXCEPT pre-existing `umrs-selinux/tests/mcs_translator.rs` fixture issue (5 failures; not caused by FHS work)

## What did not change this session

- `components/rusty-gadgets/selinux/umrs.fc.in` — still uses `@PREFIX@/etc` and `@PREFIX@/var` (gone). Tracked as task #7.
- Clap audit findings (2026-04-15 report) — still unimplemented, and audit itself is now stale. Task #8.
- 4 other tools' man pages — c2pa/ls/stat/uname EN+FR unaudited for FHS + CLI. Task #9.
- Antora stale path references in `operations/ima-evm-ops.adoc` and `umrs-tools/umrs-c2pa.adoc`. Task #9.
- `rhel10-installation.adoc` end-to-end stitching. Task #10.

## Outstanding work (tracked in task board)

Execution order matters. Each blocks the next:

1. **#7 — Knox: FHS-update `umrs.fc.in`** (smallest; unblocks full `restorecon` coverage)
2. **#8 — Rusty: refresh + execute clap audit** (blocked by #7's SELinux stability)
3. **#9 — tech-writer + Simone: man + Antora sweep** (blocked by #8's CLI reality)
4. **#10 — stitch `rhel10-installation.adoc` end-to-end** (blocked by #9)

When all four complete, the RHEL 10 setup guide is delivery-ready.

### Parallel work (new flag, 2026-04-16 late)

5. **#11 — Move umrs-sign-mgr + umrs-shred to `/opt/umrs/bin`** (Jamie, 2026-04-16)
   - umrs-sign-mgr.sh has broken Unicode smart quotes — not executable until repaired
   - Requires `/etc/opt/umrs/pki/{private,public,meta,archive}` directory provisioning
   - Staging pipeline currently skips `scripts/*/*.sh` (nested) — needs recurse or flatten
   - Full analysis in memory: `project_next_bin_migration.md`

## Known pre-existing issues (flagged, not in scope)

- `libs/umrs-selinux/tests/mcs_translator.rs` references `data/setrans.conf`
  fixture that does not exist. 5 failing tests. Unrelated to FHS work.
- `libs/umrs-selinux/config/{secolor.conf,setrans.conf}` — classified as
  unknown by staging (not JSON, not template). Previously copied to
  `staging/config/` but never installed. Probably belong at
  `/etc/selinux/<policy>/` at deploy time. Policy call pending.

## Compliance citations (canonical)

- FHS 2.3 §3.7.4 — `/etc/opt` for add-on package configuration
- FHS 2.3 §3.12 — `/opt` add-on application software packages
- FHS 2.3 §4.9.2 — `/var/opt` variable data for add-on packages
- FHS 2.3 §4.11 — `/usr/share/<package>` pattern for static reference data
- FHS 2.3 §5.15 — "No other package files may exist outside /opt, /var/opt, /etc/opt"
- NIST SP 800-53 CM-2, CM-6, CM-7, CM-8, SA-12 (baseline, config, least functionality, inventory, supply chain)

## Files to read on resume

1. This report — the consolidated picture
2. `.claude/agent-memory/security-engineer/permissions.md` — install sequence
3. `docs/modules/deployment/pages/rhel/rhel10-directory-structure.adoc` — operator-facing layout doc
4. `TaskList` in session (tasks #7–#10) — ordered next steps
