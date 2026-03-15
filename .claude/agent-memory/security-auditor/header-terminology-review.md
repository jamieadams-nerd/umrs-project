---
name: TUI Report Header Terminology Review
description: Advisory analysis of header field labels and content for the umrs-tui audit card template — terminology aligned to NIST/OSCAL/DoD assessment frameworks
type: project
---

# TUI Report Header Terminology Review

**Date:** 2026-03-15
**Requested by:** Jamie (security-auditor advisory, not an audit finding)
**Scope:** Phase 1 header mock-up in `.claude/plans/tui-enhancement-plan.md`
**Audience:** rust-developer implementing the header; tech-writer documenting it

---

## Current Mock-Up (baseline)

```
Report   : OS Detection                              Boot ID  : a3f7c2d1-...
Subject  : Platform Identity and Integrity            Checked  : 2026-03-15 14:32
Host     : goldeneye                                  Kernel   : 6.12.0-211.el10
SELinux  : Enabled (Enforcing) / Targeted             LSM      : selinux
FIPS     : Active                                     Lockdown : integrity
```

---

## Question 1 — Terminology: What should the field labels be?

### "Report" vs "Assessment" vs "Audit"

**Recommendation: "Assessment"**

In NIST SP 800-53A Rev 5, the formal term is **security control assessment**. SP 800-53A
defines three assessment methods (Examine, Interview, Test). The UMRS tool performs the
"Examine" method programmatically — it reads and analyzes mechanisms. The output document
is a **Security Assessment Report (SAR)** in RMF terminology (SP 800-37 Rev 2, Task A-4).

"Audit" has a different meaning in NIST/DoD contexts: AU (Audit and Accountability) refers
to event logging (AU-2, AU-12). An "audit" in NIST terms is a review of log records, not
the examination of system configuration. Using "Audit" as the header label for a tool that
reads kernel state and SELinux posture would create a false mapping to AU-family controls.

"Report" is informal and has no normative meaning in SP 800-53 or SP 800-53A. It does not
signal to an assessor what kind of artifact this is.

**Conclusion:** Use "Assessment" for the header label. In JSON export, the key should be
`assessment_type` (OSCAL aligns on this term for the finding/result container).

### "Subject" — is this the right term?

**Recommendation: "Scope" or "Assessment Object"**

In SP 800-53A, the term **assessment object** is the specific thing being examined. For the
Examine method, assessment objects are specifications, mechanisms, or activities. The UMRS
tool examines mechanisms — specifically, kernel-sourced platform state.

However, "Assessment Object" is verbose for a header label. SP 800-53A also uses
**scope** when describing the boundary of what is being assessed (SAP Section 3 is titled
"Scope"). FedRAMP SAR Section 2.3 is titled "Scope (Controls Assessed)."

Two options, in descending preference:

1. **"Scope"** — short, immediately understood by assessors, maps to SAP/SAR Section 3.
   Answers "what was examined." Example value: `"Platform Identity and Integrity"`.

2. **"Assessment Object"** — more precise to SP 800-53A vocabulary but reads awkwardly
   on a TUI header row. Better as a JSON key (`assessment_object`) than a display label.

**Recommendation for display label:** `Scope`
**Recommendation for JSON key:** `assessment_object`

This separation of display label from JSON key is important: the tool can use
operator-friendly short labels while the JSON export uses normative SP 800-53A terminology
that an assessor will recognize when parsing the file.

### Are there standard field names from NIST SP 800-53A, OSCAL, or assessment methodology?

Yes. OSCAL (Open Security Controls Assessment Language) defines a JSON/XML schema for
assessment results. The OSCAL Assessment Results model includes:

| OSCAL field | Maps to header field | Display label recommendation |
|---|---|---|
| `assessment-results.metadata.title` | Assessment type | `Assessment` |
| `assessment-results.metadata.last-modified` | Checked timestamp | `Assessed` |
| `finding.target.target-id` | System/component being examined | `System` |
| `assessment-results.result.local-definitions.components` | Host, kernel version | split into `Host` + `Kernel` |
| `assessment-results.result.observations[].description` | Individual findings | (data panel, not header) |

OSCAL uses "finding" and "observation" rather than "report" or "subject." The JSON export
layer should align to OSCAL field names where they exist.

**Citation:** NIST SP 800-53A Rev 5, Section 2.1 (assessment methods and objects);
OSCAL Assessment Results schema (NIST GitHub, oscal.io).

---

## Question 2 — Header Content: What fields SHOULD be in a common report header?

### Mandatory Fields (every audit card tool, every run)

These fields are required to make the report a valid SP 800-53A "Examine" object and to
satisfy CA-7 (continuous monitoring) correlation requirements.

| Display label | JSON key | Source | Why mandatory |
|---|---|---|---|
| `Assessment` | `assessment_type` | caller-supplied | Names what was assessed. Without this, the report cannot be classified by type. |
| `Scope` | `assessment_object` | caller-supplied | Names what was examined (SP 800-53A assessment object). |
| `Assessed` | `assessed_at` | system clock at run start | ISO-8601 timestamp. CA-7 requires that each check be time-stamped to support frequency verification. |
| `Tool` | `tool_name` | binary constant | Ties the result to the producing tool. Required for SP 800-53A SA-11 (tool evidence). |
| `Tool Version` | `tool_version` | Cargo package version | Required for the artifact to serve as an Examine object — assessors must know the tool version. This was called out in prior TUI audit card review. |
| `Host` | `hostname` | `gethostname()` | Identifies the system assessed. Without this, the report is unattributable. |
| `Boot ID` | `boot_id` | `/proc/sys/kernel/random/boot_id` | Uniquely identifies the specific boot instance. Correlates with journald. Essential for CA-7 ongoing monitoring — the same hostname across different boot instances is a different system state. |

### Strongly Recommended Fields (support correlation and trending)

| Display label | JSON key | Source | Why recommended |
|---|---|---|---|
| `System UUID` | `system_uuid` | `/sys/class/dmi/id/product_uuid` | Uniquely identifies the physical or virtual machine even across hostname changes. Enables cross-run correlation when hostname or network config changes. |
| `Kernel` | `kernel_version` | `uname -r` or `/proc/version` | Version of the kernel under which the assessment ran. Different kernel versions may produce different posture results. Required for meaningful trending. |
| `SELinux` | `selinux_status` | kernel attribute | Primary MAC control — any assessor receiving this report needs immediate visibility. |
| `FIPS` | `fips_mode` | kernel attribute | FIPS active/inactive is a go/no-go signal for cryptographic compliance. Must be in the header, not buried in a tab. |
| `LSM` | `active_lsm` | kernel attribute | Which LSM is active matters for interpreting SELinux status. |
| `Lockdown` | `lockdown_mode` | kernel attribute | Kernel lockdown constrains what privileged processes can do. Relevant to tool execution context. |

### Optional / Tool-Specific Fields (Phase 2 HeaderField)

These fields vary by tool and are best supplied via the Phase 2 `HeaderField` extensibility
mechanism rather than being hardcoded into the common header:

| Display label | JSON key | Applies to |
|---|---|---|
| `FQDN` | `fqdn` | Tools running in networked environments; overkill for standalone assessments |
| `Operator` | `operator` | Could be `$USER` or UID; useful for multi-user systems; privacy-sensitive |
| `Report UUID` | `report_uuid` | For tools that export structured JSON; generated at run time; enables report deduplication |
| `Policy` | `selinux_policy_type` | OS detect tool specifically (targeted vs. mls vs. mcs) |

### What SHOULD NOT be in the header

- Raw security labels or MLS categories (SC-28 — could expose classification)
- Operator password or credentials
- Network addresses or routing information (isolated systems; also SI-12)
- Internal path names (SI-12)

---

## Revised Header Mock-Up

Based on the above analysis, the revised two-column header would be:

```
Assessment  : OS Detection                            Boot ID   : a3f7c2d1-...
Scope       : Platform Identity and Integrity         Assessed  : 2026-03-15 14:32
Host        : goldeneye                               Kernel    : 6.12.0-211.el10
Tool        : umrs-os-detect 0.3.1                   System ID : <UUID>
SELinux     : Enabled (Enforcing) / Targeted          LSM       : selinux
FIPS        : Active                                  Lockdown  : integrity
```

Changes from current mock-up:
- `Report` → `Assessment` (SP 800-53A normative terminology)
- `Subject` → `Scope` (SAP/SAR Section 3 terminology; `assessment_object` in JSON)
- `Checked` → `Assessed` (consistent with SP 800-53A language for completed assessments)
- Added `Tool` + version (required for Examine object per SA-11)
- Added `System ID` / `system_uuid` (right column, replaces nothing — new field)
- All else unchanged

The `Tool` field can be a `HeaderField` (Phase 2) if the common header does not include
it — but it should be treated as mandatory, not optional. The plan already calls this out:
"Tool version as a HeaderField is required for audit card to serve as SP 800-53A Examine
object."

---

## JSON Export Field Mapping

When the `--json` flag exports this header, the JSON object should use:

```json
{
  "assessment_type": "OS Detection",
  "assessment_object": "Platform Identity and Integrity",
  "assessed_at": "2026-03-15T14:32:00Z",
  "tool": {
    "name": "umrs-os-detect",
    "version": "0.3.1"
  },
  "system": {
    "hostname": "goldeneye",
    "system_uuid": "...",
    "boot_id": "a3f7c2d1-...",
    "kernel_version": "6.12.0-211.el10"
  },
  "security_posture": {
    "selinux": "Enabled (Enforcing) / Targeted",
    "fips_mode": "Active",
    "active_lsm": "selinux",
    "lockdown_mode": "integrity"
  }
}
```

This structure maps directly to OSCAL Assessment Results metadata and component
observation patterns, enabling future OSCAL serialization without a schema redesign.

---

## Control Citations for the Header Design

| Field group | Control | Rationale |
|---|---|---|
| `assessed_at`, `boot_id` | NIST SP 800-53 AU-3 | Content of audit records — time, subject, outcome |
| `assessed_at` + CA-7 frequency | NIST SP 800-53 CA-7 | Ongoing monitoring — checks must be timestamped to verify frequency |
| `tool_version` | NIST SP 800-53 SA-11 | Developer security testing — tool identity required for Examine object traceability |
| `selinux_status`, `fips_mode`, `lockdown_mode` | NIST SP 800-53 SI-7, CM-6 | System integrity and security function verification |
| Header as Examine object | NIST SP 800-53A Rev 5 Section 2.1 | The header provides the metadata an assessor needs to qualify this output as an Examine artifact |

---

## Summary of Recommendations

1. **Change "Report" to "Assessment"** — normative SP 800-53A term; maps to `assessment_type`
   in JSON.

2. **Change "Subject" to "Scope"** — maps to SAP/SAR Scope sections; maps to
   `assessment_object` in JSON. This is the term assessors use.

3. **Change "Checked" to "Assessed"** — consistent with SP 800-53A; maps to `assessed_at`
   in JSON as ISO-8601.

4. **Add "Tool" (name + version) as a mandatory header field** — required for the artifact
   to serve as an SP 800-53A Examine object. Implement via Phase 2 `HeaderField` if not
   in the common fixed header, but treat as mandatory, not optional.

5. **Add "System ID" (system_uuid) as a recommended header field** — enables cross-run
   correlation and trending when hostname changes. Right-column addition.

6. **JSON keys must use OSCAL-aligned names** — `assessment_type`, `assessment_object`,
   `assessed_at`, `boot_id`, `system_uuid`. Display labels can be short; JSON keys should
   be normative.

7. **`boot_id` stays** — this is correct and important. No change recommended.

8. **Security posture fields (SELinux, FIPS, LSM, Lockdown) stay in the header** — these
   are the right fields. An assessor scanning a report stack needs these at a glance.
