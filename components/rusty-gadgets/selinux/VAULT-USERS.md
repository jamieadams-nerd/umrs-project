# Adding a Vault User Account

This guide describes the procedure for creating a new Linux account that
accesses the UMRS vault with a bounded clearance expressed as an MCS
category range. The reference account is `umrs-agent` (unrestricted
within `staff_u`). New accounts follow the same four-step recipe but
vary in step 2 — the MCS range.

Scope: Phase 1 (targeted policy). The mapping and relabel steps land
today; file-level enforcement activates in Phase 2 when vault data
types carry MCS categories.

Prerequisites:

- The `umrs` policy module is loaded (`sudo semodule -l | grep '^umrs$'`).
- `/opt/umrs/` is installed and labeled (see `INSTRUCTIONS.md`).
- The caller has sudo NOPASSWD on `useradd`, `semanage login`, and
  `restorecon` for host paths under `/home/` and `/opt/umrs/`.

---

## Axioms

Two rules govern every decision in this procedure:

1. **No unconfined user ever touches the vault.** New accounts must be
   mapped to `staff_u` (or another confined SELinux user). The
   `unconfined_t` neverallow guardrail in `umrs.te` is non-negotiable.

2. **The MCS range on the login mapping is the clearance.** Step 2 is
   where access scope is decided. Getting it wrong creates a silent
   over-grant once Phase 2 enforcement is live.

---

## The four steps

### 1. Create the Linux account

```bash
sudo useradd -m -s /bin/bash -G umrs-agent <username>
```

- `-m` creates the home directory.
- `-G umrs-agent` adds the account to the shared group used by vault
  service-style users. Add additional groups if the account also needs
  `umrs-admin` (config write) access — most vault users should NOT.

Verify:

```bash
getent passwd <username>
id <username>
```

### 2. Map to a confined SELinux user with a bounded MCS range

```bash
sudo semanage login -a -s staff_u -r s0:<category-set> <username>
```

The `-r s0:<category-set>` argument is the clearance. Syntax:

| Form | Meaning | Example |
|---|---|---|
| `s0:cN` | exactly one category | `s0:c310` — Canadian Protected B only |
| `s0:cN,cM` | exact discrete set | `s0:c90,c91` — CUI//LEI + CUI//AIV |
| `s0:cN.cM` | contiguous range | `s0:c300.c399` — all Canadian Protected categories |
| `s0` | no categories | unusable for vault work under Phase 2 |

Category numbers come from `.claude/rules/labeling_mcs.md`:

- US CUI categories: `c0`–`c249`
- US LDCs / Distribution Statements: `c250`–`c279`
- US reserved: `c280`–`c299`
- Canadian Protected: `c300`–`c399`

Verify the mapping was recorded:

```bash
sudo semanage login -l | grep <username>
```

### 3. Relabel the new home directory

New home directories inherit `user_home_dir_t` on creation. The SSH
login will not transition cleanly into `staff_r` unless the home is
labeled consistently with the mapping.

```bash
sudo restorecon -RFv /home/<username>
```

### 4. Verify the effective clearance

Use `ssh` — never `sudo -u` or `su -`. Only SSH performs the full PAM
login transition that applies the SELinux login mapping.

```bash
ssh <username> id -Z
```

Expected format:

```
staff_u:staff_r:staff_t:s0:<category-set>
```

The `<category-set>` in the output must match the `-r` argument from
step 2 exactly.

---

## Worked examples

### Example A — US CUI, limited to Law Enforcement categories

Account that may read CUI//LEI (`c90`) and CUI//AIV (`c91`) material,
nothing else:

```bash
sudo useradd -m -s /bin/bash -G umrs-agent umrs-lei-agent
sudo semanage login -a -s staff_u -r s0:c90,c91 umrs-lei-agent
sudo restorecon -RFv /home/umrs-lei-agent
ssh umrs-lei-agent id -Z
```

Expected verification:

```
staff_u:staff_r:staff_t:s0:c90,c91
```

Under Phase 2 enforcement, this account will be denied access to a file
labeled `s0:c1` (CUI//PRVCY) because `c1` is not in the cleared set.

### Example B — Canadian, limited to Protected B

Account that may read Protected B material only — not A, not C:

```bash
sudo useradd -m -s /bin/bash -G umrs-agent umrs-ca-b-agent
sudo semanage login -a -s staff_u -r s0:c310 umrs-ca-b-agent
sudo restorecon -RFv /home/umrs-ca-b-agent
ssh umrs-ca-b-agent id -Z
```

Expected verification:

```
staff_u:staff_r:staff_t:s0:c310
```

Canadian Protected tiers are mutually exclusive on the same asset, so
the `-r` argument is a single category. To extend clearance to
Protected A (`c300`) as well, use `s0:c300,c310` — but that is a policy
decision that requires authorization separate from account creation.

### Example C — US CUI, broad clearance across all US categories

Account cleared for the full US CUI category range (e.g., an auditor):

```bash
sudo useradd -m -s /bin/bash -G umrs-agent umrs-us-auditor
sudo semanage login -a -s staff_u -r s0:c0.c249 umrs-us-auditor
sudo restorecon -RFv /home/umrs-us-auditor
ssh umrs-us-auditor id -Z
```

Note: this does NOT grant Canadian Protected access. The range
`c0.c249` stops before `c300`.

---

## Changing an existing account's clearance

Use `-m` (modify) in place of `-a` (add):

```bash
sudo semanage login -m -s staff_u -r s0:c90,c91,c92 umrs-lei-agent
```

The account must log out and back in via SSH for the new range to take
effect. No restorecon is needed for a range change alone.

## Removing an account

Reverse order: remove the mapping first, then the Linux account.

```bash
sudo semanage login -d <username>
sudo userdel -r <username>
```

The `-r` flag to `userdel` removes the home directory. If the account
accessed vault data that now lives under `/opt/umrs/var/lib/`, that
data is not removed — vault data has its own retention policy
independent of user accounts.

---

## Phase 1 vs Phase 2 — what enforces what today

| Check | Phase 1 (today) | Phase 2 (MLS types) |
|---|---|---|
| Account cannot exec UMRS binary | enforced via `unconfined_t` neverallow | same |
| Account can read `/opt/umrs/etc` | enforced via `umrs_read_config(staff_t)` | same |
| Account can read vault files | not yet — vault data types are Phase 2 | enforced via MCSConstrain on `umrs_data_*_t` |
| Account's MCS clearance limits what it can read | **not enforced** — advisory only | enforced: subject range must dominate object range |
| Account cannot write outside cleared range | **not enforced** | enforced |

The `-r` clearance recorded today is defensive-in-advance: every
account created now will silently pick up full enforcement the moment
vault types are labeled in Phase 2, with no account-side changes
required.

---

## Common mistakes

- **Using `user_u` instead of `staff_u`.** `user_u` has no `newrole`
  path and limited auditability. `staff_u` is the idiomatic confined
  admin role under RHEL 10 targeted policy.
- **Forgetting step 3 (restorecon on the home directory).** The
  account will still log in, but some PAM-managed session files will
  land with the wrong type and later confuse `staff_r` transitions.
- **Using `sudo -u` or `su -` to test the mapping.** Neither performs
  a real login transition. Always verify with `ssh <username> id -Z`.
- **Over-granting by using a range when a set was intended.**
  `s0:c0.c249` covers 250 categories; `s0:c90,c91` covers exactly two.
  A range is rarely what you want for a category-scoped account.
- **Mapping to an MCS range that includes reserved slots.** The
  `c250`–`c299` range holds LDCs, Distribution Statements, and
  reserved slots. Clearing an account for those without a distinct
  authorization is an over-grant.

---

## References

- `INSTRUCTIONS.md` — the underlying install pipeline and base account layout
- `.claude/rules/labeling_mcs.md` — MCS category number ranges per nation
- `.claude/rules/cui-taxonomy-and-rules.md` — US CUI category catalog and banner rules
- `.claude/rules/selinux.md` — SELinux axioms, trust gates, context format
