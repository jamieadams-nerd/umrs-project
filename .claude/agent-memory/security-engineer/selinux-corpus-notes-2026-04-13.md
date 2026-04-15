# SELinux Corpus Familiarization Notes — 2026-04-13

Read-only pass. No writes to policy, no fixes, no recommendations delivered to Jamie yet.
Purpose: structured notes to support an upcoming discussion.

---

## 1. Layout Map — Where Policy Lives

### Current (canonical) location
`/components/rusty-gadgets/selinux/`
- `umrs.te` — **current base custody policy** (byte-identical to `/components/rusty-gadgets/selinux/umrs.te`)
- `umrs.fc` — file context assignments
- `umrs.if` — interfaces (read/manage for config/data/logs)
- `Makefile` — build driver, references `umrs`, `umrs_cui`, `cui_lei`
- `README.md` — 9-line stub, says three files per module (te/fc/if)

### Workspace copy (likely stale duplicate)
`/components/rusty-gadgets/selinux/`
- `umrs.te` — identical to the rhel10 one
- `README.md` — scratch/brainstorm notes: "Multiple Linux Accounts for Seperation", intake/staging/archive vault flow, proposed `umrs_vault_{intake,stage,archive}_data_t` types (NOT in current .te), mention of `umrs-us` / `umrs-ca` accounts and semanage login setup

### Per-crate policy stub
`/components/rusty-gadgets/libs/umrs-core/selinux/`
- `umrs-core.te` — minimal app domain skeleton (`umrs_core_t` + `umrs_core_exec_t`) with basic process/lib/etc allow rules. New-crate template; not integrated with base policy; not in Makefile `POLICY_MODULES`.
- `README.md` — 5-line stub, typo "Conextsl"

### C stubs README
`/components/rusty-gadgets/libs/umrs-selinux/examples/c-stubs/README.md`
- Documents intent: C programs linked against libselinux serve as oracles to verify Rust impls agree with libselinux output (with enrichment). Not policy.

### Archived
`/components/rusty-gadgets/selinux/archive/`
- `umrs_cui.te` (63 lines) — declares `policy_module(umrs_vault, 1.0)` (**module name mismatch with filename**) + CUI base types (cui_data_ro/rw_t, cui_config_ro/rw_t, cui_log_rw_t) + attribute `umrs_vault_data_t` declared but never used
- `umrs_cui.if` — CUI read/manage/append interfaces
- `umrs_cui.fc` — 1-line stub only
- `cui_lei.te` (468 lines) — LEI with 15 subcompartments (AIV, CHRI, CMPRS, SUB, LCOMM, LDNA, FUND, INF, INV, JUV, LFNC, LNSL, LSCRN, TRACE, RWRD, SCV, WHISTL), `lei_officer_t` domain with `domain_type()`, `role lei_officer_r`, `user lei_officer_u`, `read_files_pattern` on cui_lei_*_type
- `cui_lei.if`, `cui_lei.fc`, `backup-cui_lei.te` (412 lines), `backup-cui_lei.if`
- Git status confirms these were deleted from the main policy/ and moved to archive/ recently

### Junk (explicitly deprecated)
`/components/rusty-gadgets/selinux/junk/`
- `good_lei.te` — earlier simpler LEI module (no subcompartments, no domain; types/attributes/neverallows only)
- `good_lei.if` — literally `## <summary>junk</summary>`
- `NOTES.md` — **very valuable**: tutorial write-up (written TO Jamie, likely a prior session response) on why to use file/dir permission macros (`read_file_perms`, `read_files_pattern`, `rw_files_pattern`, etc.) instead of hand-rolled `{ read getattr open }`. Lists layering: Platform/Compartments → pattern macros; Tools/daemons → interface macros; Surgical → raw perms. Ends with proposed three-tier plan: `umrs.te/.if` (base) + `umrs_cui.te/.if` (classification) + future `cui_lei.te`/`cui_agr.te`/`cui_cti.te` compartments.
- `README.md` — stub

### Current vs deprecated
- **Working policy today:** only `umrs.te` + `umrs.fc` + `umrs.if` in `platforms/rhel10/.../policy/`
- **Recently retired:** `cui_lei.*` and `umrs_cui.*` (moved to archive/)
- **Build system drift:** `Makefile` still declares `POLICY_MODULES := umrs umrs_cui cui_lei` — `make` will fail

---

## 2. Type Enforcement Model (current `umrs.te`)

### Module
`policy_module(umrs, 1.0)`

### External deps
- `type unconfined_t;` (gen_require)

### Attributes
- `umrs_data_type`, `umrs_config_type`, `umrs_log_type`, `umrs_httpd_type`

### Types (all `files_type()`)
- `umrs_config_ro_t`, `umrs_config_rw_t`
- `umrs_data_ro_t`, `umrs_data_rw_t`
- `umrs_log_ro_t`, `umrs_log_rw_t`
- `umrs_httpd_t`

### Attribute bindings
RO and RW variants both bound to the same attribute (so `umrs_data_type` contains both ro_t and rw_t). Same for config, log. `umrs_httpd_t` → `umrs_httpd_type`.

### No domain, no role, no user
- No `application_domain()`, no `domain_type()`.
- No role or SELinux user declarations.
- No process/transition rules.

### No MLS/MCS constructs
- No `mlsconstrain`, no `mlsvalidatetrans`, no category references.
- All .fc contexts at `s0` (consistent with targeted policy per `selinux.md`).

### Rules
**No allow rules.** Only 8 `neverallow` rules blocking `unconfined_t` from any :dir/:file operation on the four umrs_*_type attributes (wildcard `*` perm).

---

## 3. Permissions Granted Summary

### In `umrs.te` itself
**None** — policy is declarations + neverallow guardrails only.

### In `umrs.if` (interfaces consumers call)
- `umrs_read_config($1)` — $1 gets read/search/open/getattr on dir, read/open/getattr on file, for BOTH `config_ro_t` AND `config_rw_t` (read side of RW included).
- `umrs_manage_config($1)` — full management on `config_rw_t` only: dir {create,read,write,add_name,remove_name,search,open,getattr,setattr}; file {create,read,write,append,open,getattr,setattr,unlink}.
- `umrs_read_data($1)` — mirrors read_config on data_ro_t + data_rw_t.
- `umrs_manage_data($1)` — mirrors manage_config on data_rw_t.
- `umrs_append_logs($1)` — append/open/getattr on `log_rw_t` file.
- `umrs_manage_logs($1)` — full management of `log_rw_t` dir + file.

**Implicit denials / gaps:**
- No `umrs_read_logs` interface — audit-reviewer tool cannot get log read access via existing interfaces.
- `umrs_httpd_t` has NO interface at all — declared but unreachable.
- No process/capability/fd/socket/transition rules anywhere.

### No `dontaudit` rules anywhere in corpus.

---

## 4. File Context Assignments (`umrs.fc`)

All `s0`, all `system_u:object_r`:
- `/usr/share/umrs(/.*)?` → `umrs_config_ro_t`
- `/etc/umrs(/.*)?` → `umrs_config_ro_t`
- `/var/lib/umrs(/.*)?` → `umrs_data_ro_t`
- `/var/log/umrs(/.*)?` → `umrs_log_ro_t`
- `/var/www/umrs(/.*)?` → `umrs_httpd_t`

**Noteworthy:**
- No file path in any .fc labels a `*_rw_t` type. RW types exist but no persistent filesystem path adopts them.
- `/var/log/umrs` is `log_ro_t`, yet the only log interfaces operate on `log_rw_t`.
- No fc entries for the intake/staging/archive vault paths discussed in the READMEs.

---

## 5. Booleans, Tunables, Conditionals

**None.** No `bool`, `gen_tunable`, or `if` blocks anywhere in current policy or archives.

---

## 6. Comments and TODOs (Verbatim)

### `/selinux/README.md` and `/platforms/rhel10/.../policy/README.md` (scratch/brainstorm, near-identical)
- "Accounts and groups should be prefixed with 'umrs'"
- Intake → staging → archive workflow described informally
- Typos throughout: "Seperation", "Acitivity", "accoun", "checksujs", "teh", "rusty-workstapce"
- **Explicit request left in file:** "I need an selinux .te and and .if file created to defined the selinux types. Let's work in rusty-workstapce/selinux. We will create umrs-us and umrs-ca linux accounts and an selinux login with no unconfined_t. This accounts will be used for testing access controls. The accounts mentioned above, what do I need to do with semanage to make all of this work."
- Proposed types (NOT in current .te): `umrs_vault_intake_data_t`, `umrs_vault_stage_data_t`, `umrs_vault_archive_data_t`
- Access sketch: "these users can not read/write from Staged but can read from the archive"

### `archive/cui_lei.te` module-ordering comment (verbatim, lines 3-13) — **Jamie's intended module structure template**
```
# 1. policy_module()
# 2. gen_require()
# 3. Attributes
# 4. Base custody types
# 5. Subcategory types
# 6. Domain / operational actors
# 7. Role declarations
# 8. SELinux user declarations
# 9. Allow rules
# 10. Neverallow guardrails
# 11. Transitions (if any)
```

### `archive/backup-cui_lei.te` quirks
- Line 419: `#ç≈role lei_officer_r types lei_officer_t;` — stray `ç≈` chars on a commented-out role decl
- Lines 246-247: `files_type(lei_inv_data_ro_t)` / `files_type(lei_inv_data_rw_t)` duplicated (paste error)

### `junk/NOTES.md`
Tutorial on SELinux macro usage (pattern macros vs raw perms). Ends with the three-tier layering plan the archived files were implementing.

### `umrs-core/selinux/umrs-core.te`
- Commented-out "Optional" logging and network sections
- Not registered with base policy or Makefile

---

## 7. Observations (discussion seeds — NOT findings, NOT recommendations)

### O-1: Scope reduction
Archived modules implemented a three-tier model (base / cui / lei) with fully-fleshed 15-subcompartment LEI policy and lei_officer_t domain. Current state is only base-tier `umrs.te` with no CUI, no LEI, no actor domains. Deliberate retreat to a minimal base pending firmer model, or different reason?

### O-2: Makefile drift
`POLICY_MODULES := umrs umrs_cui cui_lei` still listed; only `umrs.*` exists in build dir. `make` will fail on `umrs_cui.pp` and `cui_lei.pp`.

### O-3: Base `umrs.te` grants nothing
No allow rules and no actor domains. Every umrs_*_rw_t is declared but no process can reach it — everything is unconfined and blocked by neverallow. Intentional ("label now, grant later"), or expected to be reached via a separate per-tool module that calls the interfaces?

### O-4: RW types exist but nothing is labeled RW on disk
.fc labels everything RO. When is a `*_rw_t` label supposed to appear — subpath relabel, file_type_transition on create, or some other mechanism?

### O-5: Phase 1 doctrine vs archived content
`cui_phase1_language.md` constrains Phase 1 to labeling + awareness (no enforcement claims). Current `umrs.te` is consistent. Archived `cui_lei.te` declared `user lei_officer_u`, `role lei_officer_r`, `read_files_pattern` — that is enforcement. Was the archive move partly a Phase 1 scope-discipline decision (pull back until MLS)?

### O-6: Workspace `/selinux/umrs.te` duplicate
Byte-identical to the platforms copy. Editor working copy or leftover from before the `components/platforms/rhel10/` reorganization?

### O-7: `umrs-core/selinux/umrs-core.te` orphan
Not in Makefile. Per-crate policy template waiting to be integrated, or scaffolding residue?

### O-8: Interface asymmetries
- No `umrs_read_logs` (only append and manage) — log-reader tool has no legitimate path.
- No interface for `umrs_httpd_t` at all — type is declared but orphaned.

### O-9: `neverallow` is compile-time
`neverallow unconfined_t umrs_*_type:* *;` is a policy-compiler assertion, not a runtime deny. It means no future module can grant unconfined any access to umrs types. Worth confirming that's understood as a constraint on the policy lattice, not a runtime gate.

### O-10: `lei_officer_t` had no transitions
In archive/cui_lei.te the domain was declared with `domain_type()` but no `type_transition` or `filetrans_pattern` — files written by the officer would inherit the working directory's label, not a compartment label. Likely a planned gap.

### O-11: Archive hygiene (context only)
- `archive/umrs_cui.te` has module-name/filename mismatch (`umrs_vault` vs `umrs_cui`).
- `archive/umrs_cui.te` attribute `umrs_vault_data_t` is declared but never referenced.
- `archive/backup-cui_lei.te` has the duplicate `files_type(lei_inv_*)` pair.
- `archive/backup-cui_lei.te` has the `#ç≈role` typo.

### O-12: No process/fd/capability rules anywhere
Entire corpus touches only `:dir` and `:file`. Fine for custody-labeling scope; when actor domains land, `self:process` / `self:fd` / `self:capability` will be needed immediately.

---

## 8. UMRS Phase 1 Alignment

- Current `umrs.te` = labeling-only (types + attributes + fc + neverallow). **Consistent with Phase 1 doctrine.**
- No `mlsconstrain`, no category enforcement, nothing Phase 1 forbids.
- Neverallow guardrails are type-enforcement — appropriate under targeted policy.
- Documentation risk: `/selinux/README.md` uses enforcement language ("these users can not read/write") — should be checked against `cui_phase1_language.md` before any user-facing doc use.

---

## 9. Files Read (audit trail)

- `/components/rusty-gadgets/selinux/README.md`, `umrs.te`
- `/components/rusty-gadgets/libs/umrs-core/selinux/README.md`, `umrs-core.te`
- `/components/rusty-gadgets/libs/umrs-selinux/examples/c-stubs/README.md`
- `/components/rusty-gadgets/selinux/` — `README.md`, `umrs.te`, `umrs.fc`, `umrs.if`, `Makefile`
- `/components/rusty-gadgets/selinux/junk/` — `README.md`, `NOTES.md`, `good_lei.te`, `good_lei.if`
- `/components/rusty-gadgets/selinux/archive/` — `umrs_cui.te`, `umrs_cui.fc`, `umrs_cui.if`, `cui_lei.te`, `backup-cui_lei.te`

---

## Jamie's answers to the five discussion questions (2026-04-13)

**Q1 — Scope retreat:** Ignore items in `archive/`. Those are previous thoughts. Do not resume or critique the archived `cui_lei.*` / `umrs_cui.*` three-tier design.

**Q2 — RW types without RW fcontexts:** Jamie has not finished the `.fc` yet because the filesystem locations are not finalized. His intent is two kinds of configuration files — `rw` and `ro`. Beyond that distinction he doesn't have more specifics yet. Open until paths are decided.

**Q3 — Missing actor layer:** Not answered. Carry forward to next discussion.

**Q4 — Makefile + corpus cleanup:** Jamie doesn't know the ideal shape yet. His guess: one module dedicated to building the `.pp` policy package. Carry forward.

**Q5 — neverallow unconfined_t:** Jamie states he didn't know what he was doing when he wrote that rule. His actual design intent is the opposite of what the current rule expresses:

- **Design intent:** `unconfined_t` is NOT included in UMRS design. None of the users or accounts UMRS creates will be unconfined.
- **Implication:** The `neverallow unconfined_t` rule is consistent with that intent (it locks unconfined out of umrs types), so the rule STAYS — but for the right reason. The policy design does not rely on unconfined being reachable at all.
- Record this as an AXIOM for the policy: UMRS-managed principals never run in `unconfined_t`.

