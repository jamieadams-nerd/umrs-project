# /opt/umrs/ Deployment Layout Proposal (Revised)

Audit date: 2026-04-13 (revised same day per Jamie's decisions)
Depth: in-depth (design proposal — no artifacts written)
Scope: `/opt/umrs/` target layout, DAC plan, binary type declaration, confined-execution requirement, install sequencing for a **demonstration-grade** first release.

Status: **draft, awaiting Jamie confirmation on remaining open questions**. No `.te`/`.fc`/`.if` edits written yet.

**Phase A status:** awaiting Jamie's approval — 6 open questions. **Phase B status:** deferred, design only.

---

## Section 0 — Phased Delivery (governing work plan)

Per Jamie (2026-04-13), this proposal is split into two clearly scoped phases so Phase A can be approved and built while Phase B remains in design.

### Phase A — Installation skeleton + binary/config types

Goal: stand up `/opt/umrs/` with the documented layout, get binaries and configuration files labeled, and prove operators can execute UMRS tools inside a confined SELinux domain.

Phase A includes:

- `/opt/umrs/` tree: `bin/`, `etc/`, `share/man/`, `share/templates/`, `share/umrs/`, `var/lib/`, `var/log/`.
- DAC setup: `umrs-admin` user + `umrs-admin` group preconditions; directory ownership and modes.
- SELinux types for binaries (`umrs_bin_t`) and configuration (`umrs_config_ro_t`, optional `umrs_config_rw_t`, optional `umrs_trust_list_t`).
- `semanage login -m -s user_u <operator>` so operators execute in a confined domain.
- `neverallow unconfined_t umrs_exec_type:file *;` extension.
- Install mechanism: `cargo xtask install` (developer) and a future shipped `umrs-install` binary (release) that share a single core library.
- Proof that binaries load and run, configuration reads work, and labels are correct.

### Phase B — Vault + data types + tool access + access-control testing (deferred, design only)

Goal: introduce the vault, give UMRS tools labeled data to operate on, define actor-layer domains, and prove the MAC boundary holds under positive and negative test cases. This is where Phase 2 enforcement language begins to apply (per `cui_phase1_language.md`).

Phase B includes:

- Data types for vault contents (intake/staging/archive).
- File-type transitions and allow rules permitting UMRS tools to read/write vault data.
- Actor layer: per-tool or shared UMRS domains (the open Q3 from earlier design work).
- Category / MCS integration once the CUI catalog stabilizes.
- Access control test suite: positive tests proving authorized reads succeed; negative tests proving unauthorized reads are denied.
- Per-tool exec types (`umrs_ls_exec_t`, etc.) if warranted by the actor design.
- Complex fcontext rules for vault subtrees.

**Do not** add new Phase B content in this revision. Only move existing implied content under the Phase B subsection below.

### Phase A acceptance criteria

Phase A is "done" when all of the following are verifiable on a fresh RHEL 10 target:

1. **Clean install.** `cargo xtask install` (or `umrs-install`) completes with exit 0 on a fresh RHEL 10 VM where the only precondition is that the site administrator has created the `umrs-admin` user and `umrs-admin` group.
2. **Layout correctness.** `/opt/umrs/` exists with the directory tree from §4; `stat -c '%U %G %a'` on every directory and installed file matches the DAC table in §7.
3. **Binary labeling.** `ls -Z /opt/umrs/bin/umrs-ls` (and every other shipped UMRS binary) reports `umrs_bin_t`. `matchpathcon /opt/umrs/bin/umrs-ls` agrees.
4. **Confined execution.** A normal operator account that has been re-mapped via `semanage login -m -s user_u <operator>` can execute `/opt/umrs/bin/umrs-ls`, and the resulting process, observed via `ps -eZ`, runs in a domain that is **not** `unconfined_t`.
5. **Neverallow holds.** A shell started from `unconfined_t` is denied `execute` on `/opt/umrs/bin/umrs-ls`; the denial is observable in the audit log as an AVC against `umrs_exec_type`.
6. **Configuration reads succeed in the confined domain.** A UMRS tool launched from the confined operator domain can read files labeled `umrs_config_ro_t` under `/opt/umrs/etc/` without AVC denials.
7. **`verify-install` passes.** `umrs-install --verify` (or `cargo xtask verify-install`) reports all checks green on the installed tree.

No vault, no tool-to-vault allow rules, no MAC-enforcement test suite is required for Phase A. Those belong to Phase B.

---

## Section 1 — Design axioms  [Phase A]

- **[AXIOM] False-root prefix.** `/opt/umrs/` is designed as a *false root*: its internal layout mirrors a normal Linux root filesystem (`bin/`, `etc/`, `share/`, `var/`). The installer accepts a `--prefix` argument. In the demonstration release the prefix is hardcoded to `/opt/umrs`, but the architecture assumes the prefix is variable: it could be `/`, `/usr/local/umrs`, a chroot, or a container rootfs. No binary bakes in an absolute path. Runtime code resolves configuration paths from `$UMRS_PREFIX` or from `/proc/self/exe`, never from a compiled-in `/opt/umrs/...` literal.
  - **Implication for SELinux:** fcontext rules must reference the prefix. For the demonstration release this is resolved by shipping a `umrs.fc.in` template containing `@PREFIX@` placeholders that the installer substitutes before invoking `semanage fcontext` / `restorecon`. In the demo release the substitution result is static (`/opt/umrs/...`), but the generation step is real so that prefix-moves are a configuration change, not a source edit.
- **[AXIOM] Confined execution only.** UMRS-executing principals **MUST** run in a confined SELinux domain. Running UMRS tools as `unconfined_t` is **not a supported configuration.** The existing `neverallow unconfined_t umrs_*_type:*` stanzas extend to `umrs_exec_type`. Demonstration sites map operator logins away from `unconfined_u` (see §3).
- **[AXIOM] Phase 1 is labeling + awareness.** Targeted policy, single sensitivity `s0`. No enforcement claims (see `cui_phase1_language.md`).
- **[AXIOM] Label-based, not path-based.** SELinux decisions are label-based (`selinux.md`). Paths are the vehicle for labels via `.fc`; they are not themselves the access control surface.

Controls: NIST SP 800-53 CM-2 (Baseline Configuration), CM-6, AC-3 (Access Enforcement), AC-6 (Least Privilege), SI-7 (Integrity).

---

## Section 2 — Demonstration scope  [Phase A]

This proposal is explicitly scoped to a **demonstration-grade first release**. The goal is to stand up a working `/opt/umrs/` layout that demonstrates confined execution, label-based file separation, and a clean install/uninstall story. Everything optional is deferred.

**In scope for the demonstration release:**

- Single-prefix install at `/opt/umrs/` with the layout in §4.
- A single `umrs_bin_t` type for all UMRS executables.
- Mixed file types under a single `/opt/umrs/etc/` directory (no ro/rw directory split).
- Site-administrator precondition: `umrs-admin` user and `umrs-admin` group pre-created via the site's normal account provisioning.
- Installer that **verifies** (never creates) the `umrs-admin` user and group, lays down the tree, applies DAC, loads SELinux policy, and runs `restorecon`.
- `semanage login -m -s user_u <operator>` demonstration step so the operator is confined.
- Hardcoded `/opt/umrs/` prefix inside generated `.fc`; `@PREFIX@` substitution mechanism exists, but the generation output is static for this release.
- French and English man pages in `share/man/`.
- `share/templates/` with skeleton examples operators copy and customize.

**Deferred:**

- Per-tool exec types (`umrs_ls_exec_t`, etc.).
- Actor-domain transitions (`umrs_t` and `domain_auto_trans_pattern`).
- Dedicated `umrs_admin_u` SELinux user (custodian role).
- systemd units (no UMRS daemons planned for Phase 1).
- `umrs_httpd_t` (orphaned today; recommend removal for the demo).
- Prefix-variable install paths beyond `/opt/umrs/`.
- Local log spool under `var/log/` (recommendation: journald-only — pending confirmation).

---

## Section 3 — Confined execution and `semanage login`  [Phase A]

### Requirement

Normal operator accounts execute `umrs-*` tools. They **must not** execute them from `unconfined_t`. The targeted-policy default on RHEL 10 maps local human logins to `unconfined_u` out of the box. For the demonstration, the site administrator explicitly re-maps each UMRS operator login to a confined SELinux user that can enter UMRS-relevant domains via type transition.

### Demonstration command

For each operator who will run UMRS tools:

```
sudo semanage login -m -s user_u <login>
```

`user_u` is recommended for the demonstration. `staff_u` is also acceptable if the site needs role-transition flexibility. Neither `unconfined_u` nor `sysadm_u` is acceptable for running UMRS tools.

### Enforcement

The `neverallow unconfined_t umrs_*_type:*` stanzas already in `umrs.te` extend naturally to the new `umrs_exec_type` attribute:

```
neverallow unconfined_t umrs_exec_type:file *;
```

Any attempt to run a UMRS binary from `unconfined_t` fails at read/execute, not at the tool's own logic. This is the demonstration posture.

### Deferred

A dedicated `umrs_admin_u` SELinux user for the custodian role is out of scope. Revisit when actor-domain design begins.

Controls: NIST SP 800-53 AC-2, AC-3, AC-6, IA-2.

---

## Section 4 — `/opt/umrs/` directory tree  [Phase A]

```
/opt/umrs/                                          (prefix; variable in architecture, fixed for demo)
├── bin/                  root:root:0755            umrs_bin_t
├── etc/                  root:umrs-admin:2775 SGID (mixed types per file — see below)
├── share/
│   ├── man/
│   │   ├── man1/         root:root:0755            man_t (system default)
│   │   └── fr_CA/man1/   root:root:0755            man_t
│   ├── templates/        root:root:0755            umrs_config_type (RO skeletons)
│   └── umrs/             root:root:0755            umrs_data_type (RO assets)
├── var/
│   ├── lib/              root:umrs-admin:2775 SGID umrs_data_rw_t
│   └── log/              root:umrs-admin:2770 SGID umrs_log_rw_t  (pending Q1)
```

Notes on collapsing rules (per Jamie, 2026-04-13):

- `/opt/umrs/var/lib/umrs/` and `/opt/umrs/var/log/umrs/` are **redundant**. The `/opt/umrs/` prefix already namespaces everything inside. Use `var/lib/` and `var/log/` with no trailing `umrs/` component. Apply this principle everywhere: no component inside the prefix repeats the `umrs` token as a directory name.
- No `etc/ro/` vs `etc/rw/` directory split. A single `/opt/umrs/etc/` holds configuration. File-level SELinux types differentiate purpose (see §5).

### Path-to-label map

| Path | Owner | Group | Mode | SELinux type | R/W |
|---|---|---|---|---|---|
| `/opt/umrs/` | root | root | 0755 | `usr_t` (system default) | RO |
| `/opt/umrs/bin/` | root | root | 0755 | `umrs_bin_t` | RO |
| `/opt/umrs/bin/umrs-*` | root | root | 0755 | `umrs_bin_t` | RO |
| `/opt/umrs/etc/` | root | umrs-admin | 2775 SGID | (dir traversable; file types vary) | mixed |
| `/opt/umrs/etc/<site-config>.toml` | root | umrs-admin | 0664 | `umrs_config_rw_t` | RW (admin group) |
| `/opt/umrs/etc/<baseline>.json` | root | root | 0644 | `umrs_config_ro_t` | RO |
| `/opt/umrs/etc/<trust-list>` | root | root | 0640 | `umrs_trust_list_t` *(proposed)* | RO (tools only) |
| `/opt/umrs/etc/<secret-material>` | root | root | 0600 | `umrs_secret_t` *(proposed, deferred)* | RO (tools only) |
| `/opt/umrs/share/man/**` | root | root | 0755/0644 | `man_t` | RO |
| `/opt/umrs/share/templates/` | root | root | 0755 | `umrs_config_type` | RO |
| `/opt/umrs/share/templates/*` | root | root | 0644 | `umrs_config_type` | RO |
| `/opt/umrs/share/umrs/*` | root | root | 0644 | `umrs_data_type` | RO |
| `/opt/umrs/var/lib/` | root | umrs-admin | 2775 SGID | `umrs_data_rw_t` | RW |
| `/opt/umrs/var/log/` | root | umrs-admin | 2770 SGID | `umrs_log_rw_t` | RW (append) |

Mixed file types under a single `/opt/umrs/etc/` are acceptable because SELinux decisions are label-based. The installer assigns per-file labels via `.fc` rules keyed on filename pattern, not on a subdirectory split. Example `.fc` stanzas (to be generated from `umrs.fc.in`):

```
@PREFIX@/bin(/.*)?                              gen_context(system_u:object_r:umrs_bin_t,s0)
@PREFIX@/etc/site-.*\.toml                      gen_context(system_u:object_r:umrs_config_rw_t,s0)
@PREFIX@/etc/.*\.json                           gen_context(system_u:object_r:umrs_config_ro_t,s0)
@PREFIX@/etc/trust-list\.d(/.*)?                gen_context(system_u:object_r:umrs_trust_list_t,s0)
@PREFIX@/share/templates(/.*)?                  gen_context(system_u:object_r:umrs_config_type,s0)
@PREFIX@/share/umrs(/.*)?                       gen_context(system_u:object_r:umrs_data_type,s0)
@PREFIX@/var/lib(/.*)?                          gen_context(system_u:object_r:umrs_data_rw_t,s0)
@PREFIX@/var/log(/.*)?                          gen_context(system_u:object_r:umrs_log_rw_t,s0)
```

The installer substitutes `@PREFIX@` with the actual `--prefix` before `semanage fcontext -a` calls (demonstration default: `/opt/umrs`).

Controls: NIST SP 800-53 CM-2, CM-5 (Access Restrictions for Change), CM-6, AC-3, AC-6, SI-7.

---

## Section 5 — Configuration file types inside `/opt/umrs/etc/`  [Phase A]

Per Jamie: no directory-level ro/rw split. Purpose-specific SELinux types still exist at the file level. Proposed demonstration set:

- **`umrs_config_ro_t`** — baseline site configuration shipped in the package. Owned `root:root`, mode `0644`. Never written at runtime. Read granted via `umrs_read_config`.
- **`umrs_config_rw_t`** — operator-editable site overrides. Owned `root:umrs-admin`, mode `0664`. Written by `umrs-admin` group members; read by UMRS tools. Write granted (in a future actor-domain policy) via `umrs_manage_config`.
- **`umrs_trust_list_t`** (proposed) — trust lists (e.g., signer allow-lists). `root:root:0640`. UMRS tools read; operators do not edit directly, they replace via a controlled update procedure.
- **`umrs_secret_t`** (proposed, **deferred** past the demo) — anything cryptographically sensitive. `root:root:0600`. Access strictly through a future actor domain.

For the demonstration release, `umrs_config_ro_t` and `umrs_config_rw_t` are required. `umrs_trust_list_t` is declared only if the demonstration payload actually ships a trust list; otherwise defer. `umrs_secret_t` is out of scope.

Controls: NIST SP 800-53 CM-5, CM-6, AC-6.

---

## Section 6 — Binary type `umrs_bin_t`  [Phase A]

**Confirmation:** No `umrs_bin_t` exists today. `umrs.te` declares four attributes and seven file types; zero executable types, zero domain declarations.

### Decision: one shared `umrs_bin_t`, bound to a new `umrs_exec_type` attribute.

Justification:

1. **Demonstration scope.** Phase 1 is labeling + awareness. Per-tool exec types are only meaningful with domain transitions, which are out of scope.
2. **Caller-domain execution.** Operators mapped to `user_u` execute in `user_t`. A single `umrs_bin_t` with execute permission granted to confined user domains suffices.
3. **Future-proof.** Declaring `umrs_exec_type` as an attribute means per-tool specialization later is a one-line change per binary with no `.fc` churn.
4. **Neverallow consistency.** `neverallow unconfined_t umrs_exec_type:file *;` extends the existing pattern.

### Proposed declarations (for future `umrs.te` edit)

```
attribute umrs_exec_type;

type umrs_bin_t;
files_type(umrs_bin_t)
corecmd_executable_file(umrs_bin_t)
typeattribute umrs_bin_t umrs_exec_type;

neverallow unconfined_t umrs_exec_type:file *;
```

### Proposed fcontext (generated from `umrs.fc.in`)

```
@PREFIX@/bin(/.*)?   gen_context(system_u:object_r:umrs_bin_t,s0)
```

Controls: NIST SP 800-53 AC-3, AC-6, CM-5, SI-7.

---

## Section 7 — DAC plan  [Phase A]

### Precondition (site administrator, BEFORE running the installer)

The site administrator pre-creates, via the site's normal account-provisioning process:

- Linux group **`umrs-admin`** (note hyphen; valid in Linux group names).
- Linux user **`umrs-admin`** (or appropriate human operators added to the `umrs-admin` group). The user `umrs-admin` itself is optional; what matters is that at least one human operator is a member of the group.

The installer **does not** create users or groups. If `umrs-admin` group does not exist, the installer fails fast with a diagnostic instructing the administrator to create it.

Documented precondition (for installation notes):

> Before running the installer, create the `umrs-admin` user and `umrs-admin` group via your site's account provisioning procedure. The UMRS installer verifies these exist; it does not create them.

### Ownership and mode table (canonical)

Directories owned by the admin group use `root:umrs-admin` with setgid (`2xxx`) so files created by any member inherit the group.

Binaries and read-only material use `root:root`.

| Path | Owner | Group | Mode | Notes |
|---|---|---|---|---|
| `/opt/umrs/` | root | root | 0755 | anchor |
| `/opt/umrs/bin/` | root | root | 0755 | not group-writable |
| `/opt/umrs/bin/umrs-*` | root | root | 0755 | execute for all |
| `/opt/umrs/etc/` | root | umrs-admin | 2775 SGID | admin-writable dir |
| `/opt/umrs/etc/<rw config>` | root | umrs-admin | 0664 | group-writable |
| `/opt/umrs/etc/<ro config>` | root | root | 0644 | not group-writable; lives in same dir, different DAC + different SELinux type |
| `/opt/umrs/etc/<trust list>` | root | root | 0640 | not world-readable |
| `/opt/umrs/share/` | root | root | 0755 | |
| `/opt/umrs/share/man/**` | root | root | 0755 / 0644 | |
| `/opt/umrs/share/templates/` | root | root | 0755 | operators copy OUT of this tree |
| `/opt/umrs/share/templates/*` | root | root | 0644 | |
| `/opt/umrs/share/umrs/*` | root | root | 0644 | |
| `/opt/umrs/var/lib/` | root | umrs-admin | 2775 SGID | UMRS tools + admins write |
| `/opt/umrs/var/log/` | root | umrs-admin | 2770 SGID | group-readable for audit review |

### DAC/MAC independence

- A user not in `umrs-admin` cannot write `etc/<rw>` files **by DAC** regardless of SELinux state.
- A user in `umrs-admin` running in a domain without `umrs_manage_config` (future actor-layer interface) cannot write **by MAC** regardless of group membership.
- Both gates must pass. This layered posture survives misconfiguration at either layer.

Controls: NIST SP 800-53 AC-3, AC-6, CM-5, AU-9 (Protection of Audit Information).

---

## Section 8 — Templates directory  [Phase A]

`/opt/umrs/share/templates/` holds skeleton material that operators copy and customize:

- Example site-config TOML (operator copies to `/opt/umrs/etc/site-<name>.toml` and edits).
- Example CUI label JSON fragments (reference only; authoritative catalogs live under `share/umrs/`).
- Example policy fragments (`.te`/`.cil`) for operators extending UMRS in a local policy module.

Label: `umrs_config_type` (read-only from the tools' perspective; operators `cp` out of it, they do not edit in place). DAC: `root:root:0755` dirs, `root:root:0644` files.

Intent: operators discover what a valid configuration looks like by reading the template, never by modifying it in place. This keeps the shipped baseline recoverable.

Controls: NIST SP 800-53 CM-2, CM-6.

---

## Section 9 — Install mechanism  [Phase A]

### Recommendation: `cargo xtask install` during development; a shipped `umrs-install` binary at release time; shared code path.

Rationale:

- `cargo xtask install` is the right developer workflow — parity with existing `xtask stage` / `xtask clean`.
- End users do not have a Rust toolchain. They need a shipped `umrs-install` binary.
- Both must invoke the **same install logic.** Proposal: put the install logic in a library module (e.g., `xtask/src/install.rs` factored to share with a new `umrs-install` bin crate, or more cleanly a small `umrs-install-core` library). `xtask install` is a thin wrapper for developers; `umrs-install` is the shipped binary for operators. No duplicate code paths.

### Install flow (demonstration release)

1. **Verify preconditions.**
   a. `getent group umrs-admin` — fail fast with clear diagnostic if missing.
   b. `getent passwd umrs-admin` — optional; warn if missing.
   c. Target prefix is empty or a prior UMRS install (refuse to overwrite foreign content).
   d. SELinux is enabled and policy type is `targeted` (demonstration assumes targeted).
2. **Generate `umrs.fc`** from `umrs.fc.in` by substituting `@PREFIX@` with the actual prefix.
3. **Lay down the tree** per §4 (directories created with correct owner/group/mode as they are created — no trailing `chmod -R`, because setgid on directories and per-file modes require distinct calls).
4. **Copy artifacts:** `staging/bin/` → `bin/`, `staging/config/` → `etc/` (per manifest that names each file's RO vs RW classification and its target SELinux type), `staging/share/` → `share/`.
5. **Create empty** `var/lib/` and `var/log/`.
6. **Load SELinux policy:** `semodule -i umrs.pp`.
7. **Apply labels:** `restorecon -RF /opt/umrs/`.
8. **Post-install confined-execution step (documented, not automated):** site admin runs `semanage login -m -s user_u <operator>` for each UMRS operator.
9. **Verify:** `ls -lZ`, `matchpathcon`, `stat -c '%U %G %a'` spot checks. A `cargo xtask verify-install` / `umrs-install --verify` mode automates this (Q6).

Controls: NIST SP 800-53 CM-3 (Configuration Change Control), CM-5, SA-10, SI-7.

---

## Section 9.5 — Phase B deferred material

The following items are mentioned elsewhere in this proposal but belong to Phase B. They are collected here so Phase A implementation is not cluttered by them. No new Phase B content is added in this revision — the next session after Phase A approval will be a Phase B design pass.

### 9.5.1 Vault data types  [Phase B]

Data types for vault contents (intake / staging / archive) are out of scope for Phase A. Phase A ships `/opt/umrs/var/lib/` labeled `umrs_data_rw_t` as a placeholder working directory only — it is not the vault.

### 9.5.2 Actor-domain transitions  [Phase B]

Per-tool or shared UMRS actor domains (`umrs_t`, `umrs_admin_u`, `domain_auto_trans_pattern`, per-tool `umrs_<tool>_t`) are deferred. In Phase A, operators execute UMRS binaries from their confined login domain (`user_t`), not from a UMRS-specific process domain. Section 6 declares `umrs_exec_type` so that future per-tool exec types are a one-line change.

### 9.5.3 Per-tool exec types  [Phase B]

`umrs_ls_exec_t`, `umrs_label_exec_t`, and similar per-tool exec types are deferred. Phase A uses a single `umrs_bin_t`. The `umrs_exec_type` attribute is Phase A scaffolding that makes the Phase B split mechanical.

### 9.5.4 Enforcement claims  [Phase B]

Any language describing UMRS as *enforcing* CUI handling, vault access control, or mandatory access control belongs to Phase B (per `cui_phase1_language.md`). Phase A language is strictly *labeling, awareness, confined execution* — not enforcement of information-handling policy.

### 9.5.5 Complex fcontext for vault subtrees  [Phase B]

fcontext rules for vault intake/staging/archive subtrees, file-type transitions, and per-subtree allow rules are deferred. The `umrs.fc.in` template in Phase A covers only `bin/`, `etc/`, `share/`, `var/lib/`, `var/log/` at the directory-prefix granularity.

### 9.5.6 `umrs_secret_t`  [Phase B]

Declared as *proposed, deferred* in §5. Anything cryptographically sensitive waits for the Phase B actor layer so read access can be scoped to a specific tool domain.

### 9.5.7 Access control test suite  [Phase B]

Positive and negative MAC tests that prove tool-to-vault and tool-to-secret access boundaries are Phase B. Phase A acceptance includes only the confined-execution verification in §0.

### 9.5.8 Dedicated `umrs_admin_u` SELinux user  [Phase B]

The custodian SELinux user is Phase B. Phase A uses `user_u` for operators. The DAC group `umrs-admin` is Phase A; the SELinux user `umrs_admin_u` is not.

---

## Section 10 — Remaining open questions  [mixed — see per-question tags]

Reduced list; items Jamie already decided on 2026-04-13 are removed. All six are **Phase A gating** — Phase A approval depends on resolving them. No Phase B questions are open in this revision (Phase B is design-only, next session).

1. **[Phase A gating]** **Local log spool — journald only, or `/opt/umrs/var/log/`?** Recommendation: **journald only** for the demonstration. No UMRS tool in Phase 1 generates high-volume operational logs; journald + auditd cover structured events. If confirmed, drop `/opt/umrs/var/log/` and `umrs_log_rw_t` from the demo layout; keep the type declared in `umrs.te` for future re-activation. Confirm.

2. **[Phase A gating]** **Install mechanism.** Recommendation: `cargo xtask install` (dev) and a shipped `umrs-install` binary (release) that share a common core library. Single source of truth for layout constants. Confirm.

3. **[Phase A gating]** **Binary type shape.** Recommendation: single `umrs_bin_t` bound to `umrs_exec_type` attribute (§6). Confirm.

4. **[Phase A gating]** **Systemd units in Phase 1?** Recommendation: **none.** No UMRS daemons planned for demonstration. Confirm.

5. **[Phase A gating]** **Remove `umrs_httpd_t`?** Recommendation: **yes, remove** for the demo. Dead types invite drift. Re-declare when an HTTP-surfaced UMRS component is actually designed. Confirm.

6. **[Phase A gating]** **`verify-install` subcommand in initial scope?** Recommendation: **yes.** Read-only, no risk, high value for demonstrations (operator gets visible evidence the install is correct). Confirm.

---

## Summary

**Changes from the prior draft (2026-04-13, pre-revision):**

- Scoped the entire document to a demonstration-grade first release (new §2).
- Promoted the false-root `--prefix` principle to the top as a design axiom (§1).
- Added a confined-execution axiom: UMRS tools must not run as `unconfined_t`; operators are mapped to `user_u` via `semanage login -m` (§1, §3).
- Collapsed `etc/ro/` and `etc/rw/` into a single `/opt/umrs/etc/` with per-file SELinux types (§4, §5).
- Collapsed `/opt/umrs/var/lib/umrs/` → `/opt/umrs/var/lib/` and `/opt/umrs/var/log/umrs/` → `/opt/umrs/var/log/` (no redundant `umrs` component inside the prefix) (§4).
- Added `/opt/umrs/share/templates/` with `umrs_config_type` for skeleton material operators copy and customize (§8).
- Corrected group name: `umrs-admin` with hyphen (not `umrs_admin`) throughout.
- Documented site-administrator precondition: create `umrs-admin` user and group before running the installer; installer verifies, never creates (§7, §9).
- Recommended `cargo xtask install` (dev) + shipped `umrs-install` binary (release) sharing one core library (§9).
- Added `umrs.fc.in` → `umrs.fc` prefix-substitution mechanism (§1, §4, §9).
- Removed old Q7 (group name, answered), Q8 (custodian SELinux user, deferred), Q9 (setrans.conf, covered by templates).

**Final layout:**

```
/opt/umrs/
├── bin/                  root:root:0755             umrs_bin_t
├── etc/                  root:umrs-admin:2775 SGID  (mixed types per file)
├── share/
│   ├── man/              root:root:0755             man_t
│   │   └── fr_CA/man1/
│   ├── templates/        root:root:0755             umrs_config_type (RO examples)
│   └── umrs/             root:root:0755             umrs_data_type
├── var/
│   ├── lib/              root:umrs-admin:2775 SGID  umrs_data_rw_t
│   └── log/              root:umrs-admin:2770 SGID  umrs_log_rw_t  (pending Q1)
```

**Remaining open questions for Jamie:** 6 (see §10).

**Policy artifacts deferred pending approval:**
- `umrs.te` edit: declare `umrs_exec_type` attribute, `umrs_bin_t` type, proposed `umrs_config_rw_t`/`umrs_config_ro_t` split, optional `umrs_trust_list_t`, remove `umrs_httpd_t`.
- `umrs.fc.in` (new) + `umrs.fc` generator step — prefix-aware file context rules.
- `umrs.if` additions: `umrs_exec_all` interface; `umrs_read_config`/`umrs_manage_config` refinements.

**Code artifacts deferred pending approval:**
- `xtask/src/install.rs` and/or `umrs-install` bin crate sharing a common install-core library (coder).
- Staging manifest declaring per-file RO vs RW classification and target SELinux type (coder + security-engineer).
- `verify-install` mode (coder).

**Documentation artifacts deferred:**
- `/opt/umrs/` layout reference page under `docs/modules/operations/` (tech-writer).
- Installation procedure documenting the precondition, installer invocation, and confined-execution mapping step (tech-writer + Simone for fr_CA).
