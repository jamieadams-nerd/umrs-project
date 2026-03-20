# Security-Engineer Intern Exercises — SELinux CUI Labeling

**Date:** 2026-03-20
**Designed by:** security-engineer agent
**For:** guest-coder (summer intern)

---

## Exercise 1 — Label a Directory Tree and Audit It

**Scenario:** A compliance officer asks: "Show me which files in this project directory carry CUI markings and which categories they belong to."

### Setup

```bash
mkdir -p ~/cui-lab
echo "public document" > ~/cui-lab/readme.txt
echo "export controlled data" > ~/cui-lab/specs.txt
echo "whistleblower report" > ~/cui-lab/witness.txt

# Label files with CUI categories
chcon -l s0 ~/cui-lab/readme.txt                    # no CUI marking
chcon -l s0:c90,c91 ~/cui-lab/specs.txt             # CUI//LEI (Law Enforcement)
chcon -l s0:c90,c107 ~/cui-lab/witness.txt          # CUI//LEI/WHISTL

# Verify
ls -lZ ~/cui-lab/
```

### Write: `cui_label_audit.rs`

Using `SecureDirent::from_path()`:
- Read each file's `SelinuxCtxState`
- Iterate the `CategorySet`
- Surface all `SecurityObservation` results
- Produce a report: filename | MCS level | category count | observations

### Teaching Moment: `restorecon`

```bash
restorecon -v ~/cui-lab/*
ls -lZ ~/cui-lab/
```

The intern discovers `restorecon` **erased their custom MCS categories** because the file context policy for the path doesn't preserve them. This is the #1 practical mistake when deploying CUI labeling.

### Security-Engineer Concerns

- WARNING: Labeled does NOT mean enforced in targeted policy
- `chcon` does not validate category numbers against `setrans.conf` — you can assign meaningless categories
- `restorecon` behavior must be in the deployment guide — persistent labeling requires file context entries or `semanage fcontext`

### What's Different in MLS Policy (Phase 2)

- `chcon` would require the caller to have dominance over the target level
- `restorecon` would restore to the policy-defined default level, which WOULD include MLS ranges
- File context entries would specify MLS levels, not just types

---

## Exercise 2 — Cross-Label Access: Observe What Targeted Policy Does NOT Block

**Scenario:** A developer's software needs to decide: "Should my process handle this file?" The software checks the file's CUI category against the process's authorized categories.

### Setup

```bash
echo "compartment A data" > ~/cui-lab/compartment-a.txt
echo "compartment B data" > ~/cui-lab/compartment-b.txt

chcon -l s0:c30 ~/cui-lab/compartment-a.txt          # CUI//CTI
chcon -l s0:c60 ~/cui-lab/compartment-b.txt          # CUI//NNPI
```

### Write: `cui_access_check.rs`

- Build a simulated "process clearance" using `CategorySet` containing only `c30` (CTI)
- Read each file's security context
- Use `CategorySet::dominates()` to check access
- For `compartment-a.txt`: PASS (categories match)
- For `compartment-b.txt`: FAIL (disjoint categories)

### The Teaching Moment

```bash
# After the program says FAIL for compartment-b.txt:
cat ~/cui-lab/compartment-b.txt
# Output: "compartment B data"
# Wait... it worked?!
```

**YES.** In targeted policy, the `cat` succeeds because `unconfined_t` bypasses MCS checks. The userspace `dominates()` check is **awareness, not enforcement**. The kernel did not block the read.

This is the Phase 1 vs Phase 2 lesson: "Phase 1 tells you the access SHOULD be denied. Phase 2 actually denies it."

### Security-Engineer Concerns

- Do NOT ship `dominates()` as an access gate in Phase 1 — it's informational only
- The intern cannot observe their own process's real clearance in targeted policy
- API gap identified: no `getcon()` equivalent in umrs-selinux to read the calling process's security context

### What's Different in MLS Policy (Phase 2)

- The kernel would return EACCES on the `cat` command
- `dominates()` result would match kernel enforcement
- The process's real clearance would be visible via `/proc/self/attr/current`

---

## Exercise 3 — Observe `restorecon` and File Context Policy

**Scenario:** The operations team asks: "Are our CUI labels consistent with what the SELinux policy expects for these paths?"

### Setup

```bash
# Label a file with a custom context
chcon -l s0:c30,c31 ~/cui-lab/specs.txt

# Check what the policy says the label SHOULD be
matchpathcon ~/cui-lab/specs.txt

# Compare
ls -lZ ~/cui-lab/specs.txt
```

### Write: `policy_consistency_check.rs`

- Read the live label via `SecureDirent`
- Call `matchpathcon` via subprocess (NOTE: this violates the UMRS no-subprocess rule — the intern should file this as an API gap)
- Compare live label vs policy default
- Report: CONSISTENT or DRIFT with details

### Useful inspection commands for the intern

```bash
seinfo --sensitivity             # Show defined sensitivities
seinfo --category               # Show defined categories
sesearch --allow -s unconfined_t -t user_home_t -c file  # What can unconfined do to home files?
```

### Security-Engineer Concerns

- `matchpathcon` output is policy intent, not live state — the intern must understand the difference
- Do NOT write a relabeler — observation only
- `setools-console` may not be installed on minimal RHEL 10 — note this in prerequisites
- The subprocess call to `matchpathcon` should be filed as API gap: need `matchpathcon()` binding in umrs-selinux

---

## API Gaps the Intern Should File

After completing all three exercises:

1. **No `getcon()` equivalent** — cannot read the calling process's own security context
2. **No `matchpathcon()` binding** — requires subprocess workaround
3. **Cross-crate pointer** — umrs-platform rustdoc doesn't mention umrs-selinux (already identified)

---

## Documentation Work for Tech-Writer

- Cross-reference gap between umrs-platform and umrs-selinux in rustdoc
- Deployment guide needs a dedicated section on `restorecon` and persistent CUI labeling
- WARNING admonition in all labeling tutorials: targeted policy = discretionary, not mandatory
