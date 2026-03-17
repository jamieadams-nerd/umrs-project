# Compliance Audit: UMRS Tool Initialization API (`umrs-core::init`)

```
Audit date: 2026-03-17
Depth: in-depth
Scope: .claude/plans/umrs-tool-init.md — environment scrubbing, validated accessors,
       logging initialization, i18n setup; including post-review decisions (Decisions 9–18).
       Cross-referenced against .claude/reports/2026-03-17-umrs-tool-init-security-review.md.
```

---

## Audit Overview

This audit reviews the compliance annotation coverage, citation accuracy, and
code-to-documentation consistency of the UMRS Tool Initialization API plan. It covers seven
areas: (1) control citation completeness and accuracy; (2) compliance claim substantiation;
(3) ScrubReport evidence trail sufficiency; (4) denylist justification traceability;
(5) validation class rigor; (6) gap analysis from NIST 800-53 assessor, CMMC L2 assessor, and
NSA RTB reviewer perspectives; (7) documentation artifact requirements for audit readiness.

The security engineer's prior review (2026-03-17) produced 11 findings (2 HIGH, 4 MEDIUM,
5 LOW). That review is treated as ground truth for resolved issues. This audit examines what
remains unaddressed and what new observations the compliance lens surfaces.

**Prior review status at plan freeze:**
- Finding 1 (HIGH — denylist incomplete): RESOLVED in plan — missing vars added to Tier 3 table
- Finding 2 (HIGH — unsafe/forbid incompatibility): RESOLVED — Decision 9 adopts Option A
  (read-only snapshot, no `set_var`/`remove_var`, `#![forbid(unsafe_code)]` preserved)
- Finding 3 (LOW — IFS classification): RESOLVED — Decision 13 moves IFS to Tier 3
- Finding 4 (MEDIUM — TOCTOU symlink chain): RESOLVED — Decision 18 adds per-component check
- Finding 5 (MEDIUM — USER/LOGNAME UID cross-check): RESOLVED — Decision 17
- Finding 6 (LOW — syslog tag rationale / stderr strip): OPEN — plan does not address
- Finding 7 (LOW — AU-8/AU-9/SI-11 missing from logging): PARTIALLY RESOLVED — Implementation
  Notes line 701 adds "Logging module needs AU-8, AU-9, SI-11" but the `init_logging` signature
  doc comment (line 554–556) still cites only AU-3 and AU-12
- Finding 8 (LOW — init_i18n provenance note / return type): PARTIALLY RESOLVED — Decision 14
  adds warn on fallback, but return type remains `()` and no provenance note is in the signature
- Finding 9 (MEDIUM — TCP D-Bus): RESOLVED — Decision 11 rejects tcp: and nonce-tcp:
- Finding 10 (MEDIUM — SSH_AUTH_SOCK Tier 2): RESOLVED — Decision 10 moves to Tier 3
- Finding 11 (LOW — test coverage): OPEN — test list not updated; no GLIBC_TUNABLES test added

---

## File: `.claude/plans/umrs-tool-init.md`

---

### Finding CA-1

**Location:** Phase 2, `init_logging` function signature doc comment (line 554–556)

**Finding:** The `init_logging` signature explicitly cites only AU-3 and AU-12. Finding 7 of
the security engineer's review identified AU-8 (Time Stamps) and AU-9 (Protection of Audit
Information) as required additional citations, and SI-11 (Error Handling) as relevant to the
debug information discipline constraint. Implementation Notes (line 701) records that these
controls need to be added, but the plan's authoritative specification — the function's doc
comment block — has not been updated. The doc comment as written is the specification that
the coder will implement against. If the implementer reads only the function signature section,
they will produce a logging module annotated with AU-3 and AU-12 only, with no SI-11 citation
governing the error information discipline requirement and no AU-9 rationale for journal forward
sealing guidance.

This is a documentation gap with direct compliance consequence: the coder specification is
incomplete, and a code review against the stated spec would incorrectly pass.

**Severity:** MEDIUM

**Recommended citation:**
- NIST SP 800-53 AU-8 (Time Stamps — journald `_SOURCE_REALTIME_TIMESTAMP`, monotonic correlation)
- NIST SP 800-53 AU-9 (Protection of Audit Information — journal forward sealing)
- NIST SP 800-53 AU-12 (Audit Record Generation — already present)
- NIST SP 800-53 SI-11 (Error Handling — debug information discipline; no variable data from
  env vars in log output)

**Remediation owner:** tech-writer

---

### Finding CA-2

**Location:** Phase 3, `init_i18n` function signature doc comment (line 578–581)

**Finding:** The `init_i18n` signature cites `NIST SP 800-53 SC-28 (locale-appropriate output)`.
SC-28 is "Protection of Information at Rest" — it governs encryption and handling of stored
data at rest, not locale display or internationalization. This is an incorrect control citation.
Locale-appropriate output for a security tool is not a data-at-rest protection requirement.

The correct control family for output formatting and information content concerns is SI-11
(Error Handling / output content), or if the concern is that locale-appropriate display prevents
misinterpretation of security-relevant output by operators (e.g., a misrendered classification
label), the applicable control is more precisely AU-3 (Content of Audit Records — records must
be intelligible) or SA-8 (Security and Privacy Engineering Principles — human-readable output
is a security usability principle).

For i18n initialization specifically, the substantive security claim is that the textdomain is
derived from a provenance-safe kernel path (`/proc/self/exe`) rather than from the environment,
making it resistant to injection — this maps to NSA RTB RAIN (non-bypassability: the
initialization cannot be subverted by inherited state). That citation is absent.

SC-28 as cited here is **not** a coding error that the compiler can catch — it is a plausible-
looking but incorrect citation that would mislead a controls assessor into expecting data-at-rest
encryption somewhere in the i18n initialization.

**Severity:** MEDIUM

**Recommended citation:**
- Remove: `NIST SP 800-53 SC-28` (incorrect; SC-28 is Protection of Information at Rest)
- Add: `NSA RTB RAIN` (non-bypassability — domain derived from `/proc/self/exe`, not env)
- Add: `NIST SP 800-53 AU-3` (locale-appropriate output contributes to intelligible audit records)
- Optional: `NIST SP 800-218 SSDF PW.4` (provenance-safe construction)

**Remediation owner:** tech-writer

---

### Finding CA-3

**Location:** Phase 1, Controls block (line 512–518); `scrub_env_with` doc comment (line 400–403)

**Finding:** Both the Phase 1 controls block and the `scrub_env_with` doc comment cite
`NIST SP 800-53 IA-5` for the claim "no secrets in env" / "authenticator management."
IA-5 governs *authenticator management* — it applies to how passwords, tokens, certificates,
and other authenticators are created, stored, distributed, and revoked. It does not govern
what information is placed in process environment variables.

The actual claim being made is that process environment variables must not contain secrets
(e.g., API keys, tokens, passwords) because the environment is observable. The correct control
for this claim is:

- **NIST SP 800-53 SC-28** (Protection of Information at Rest) — if the claim is about persistent
  storage of credentials in the environment
- **NIST SP 800-53 CM-7** (Least Functionality) — if the claim is that the environment should
  not carry unnecessary sensitive data (deny what is not required)
- **NIST SP 800-53 SI-12** (Information Management and Retention) — if the claim is about
  minimizing the footprint of sensitive data

More precisely, for the CWE-526 concern (environment variable exposure to unauthorized actors
via `/proc/<pid>/environ`), the applicable NIST control is **NIST SP 800-53 AC-3** (Access
Enforcement — controlling read access to `/proc/<pid>/environ`) or **SI-10** (Information Input
Validation — validating that sensitive values are not inadvertently passed via environment).

IA-5 is a weak citation for this claim and would not satisfy a CMMC L2 or NIST 800-53 assessor
asking which control governs "secrets not stored in environment variables."

Note: SC-28 cited in the `scrub_env_with` doc comment is similarly misapplied if the intent
is the same "no secrets in env" claim. See also Finding CA-2 where SC-28 appears incorrectly
on `init_i18n`.

**Severity:** MEDIUM

**Recommended citation:**
- Remove: `NIST SP 800-53 IA-5` from the "no secrets in env" claim
- Remove: `NIST SP 800-53 SC-28` from the same claim (incorrect control family)
- Add: `NIST SP 800-53 CM-7` (Least Functionality — environment should carry only what is needed)
- Add: `NIST SP 800-53 AC-3` (Access Enforcement — `/proc/<pid>/environ` access control)
- Retain: CWE-526, CERT ENV03-C (these are correct external references for this claim)

**Remediation owner:** tech-writer

---

### Finding CA-4

**Location:** Phase 3, `init_i18n` signature (line 581) and Decision 14 (line 114)

**Finding:** The `init_i18n` signature specifies return type `()`. Decision 14 states that a
fallback to the `"umrs"` domain must be logged at `log::warn!` — silent fallback is unacceptable.
However, a `warn!` log is not a machine-detectable signal: the caller cannot distinguish a
successful `init_i18n("umrs-ls")` from a failed auto-detection that silently fell back to
`"umrs"`. For a tool where correct i18n domain is operationally required (e.g., to load
CUI-handling messages in the operator's language), a warning log may be missed, especially
when logging is initialized *after* i18n in the recommended order.

The initialization order in Phase 4 (line 614) is:
1. `scrub_env()` — first
2. `init_i18n(None)` — second
3. `init_logging(verbose)` — third

This means when `init_i18n` emits a `log::warn!` on fallback, the logging subsystem has not
yet been initialized. The warning goes to the pre-initialized backend (likely a no-op), and
the operator never sees it.

This is a substantive correctness problem that affects the claim in Decision 14 that "silent
fallback is unacceptable." With the current call order and return type, the fallback is
effectively silent — the warn is never delivered.

**Severity:** HIGH — a security property (Decision 14) is claimed but contradicted by the
implementation design (initialization order means the warn is never delivered).

**Recommended citation:** NIST SP 800-53 AU-3 (Content of Audit Records — initialization
failures must be observable), NIST SP 800-53 SI-11 (Error Handling — errors must not be
silently suppressed)

**Remediation:** One of:
(a) Change `init_i18n` to return `Result<(), InitError>` (preferred — callers can detect and
    act on the failure) and propagate via `init_tool`'s return value or a distinct initialization
    error type; or
(b) Swap initialization order — initialize logging before i18n so the warn can be delivered; or
(c) Emit the fallback warning to `eprintln!` (stderr) explicitly, documenting that this is an
    intentional pre-logging fallback path. This is the minimum acceptable fix.

**Remediation owner:** tech-writer (specification; coder implements the fix)

---

### Finding CA-5

**Location:** Phase 5, Tests (lines 628–686)

**Finding:** The test list was not updated after the post-review Decisions 9–18 were added.
Specifically:

1. **No test for D-Bus TCP rejection.** Decision 11 rejects `tcp:` and `nonce-tcp:` transports
   and logs at `warn`. The test list (tests 42–43) covers `unix:path=` acceptance and
   `unixexec:` rejection only. TCP rejection is untested.

2. **No test for USER/LOGNAME UID cross-check.** Decision 17 adds UID cross-check for
   `USER` and `LOGNAME`. Tests 37–38 cover syntax validation only. No test verifies that
   `USER=root` from a non-root process is rejected.

3. **No test for the GLIBC_TUNABLES denylist entry.** This was called out explicitly in
   Finding 11 of the security engineer's review and remains absent. `GLIBC_TUNABLES` is the
   highest-priority denylist entry given CVE-2023-4911 (RHEL 10 / glibc 2.39 affected).

4. **Test 9 ("init_tool runs without panic") is still under-specified.** Finding 11 of the
   security engineer's review identified this as insufficient. No update was made.

5. **No test for init_i18n fallback warning delivery.** Given Finding CA-4 above, the
   absence of a test that verifies the fallback warning is observable is a traceability gap.

The test suite as written would not satisfy a NIST SP 800-218 SSDF PW.8 assessor asking for
evidence that the denylist entries are tested or that security-critical validation decisions
are covered.

**Severity:** MEDIUM

**Recommended citations:** NIST SP 800-53 CA-7 (Continuous Monitoring — tests are the
verification mechanism), NIST SP 800-218 SSDF PW.8 (Test for Security)

**Remediation owner:** tech-writer (spec update); coder (test implementation)

---

### Finding CA-6

**Location:** Phase 2, `init_logging` signature (line 556)

**Finding:** `init_logging` returns `()`. The function initializes the journald backend and
falls back to stderr if journald is unavailable. Neither the availability of journald nor the
fallback condition is surfaced to the caller. If journald initialization fails and the fallback
is silently used, the tool continues in degraded audit posture (all audit output goes to stderr,
which may not be captured, forwarded, or stored with integrity).

For a tool suite producing compliance-relevant audit output, a silent degradation of the
logging backend from journald to stderr is an AU-9 (Protection of Audit Information) gap:
audit records may be lost without any signal to the operator or to monitoring systems.

The return type `()` does not satisfy the must-use contract for security-relevant return values
(as required by the Must-Use Contract Rule). The function either produces audit output or it
doesn't — the caller must be able to detect which.

**Severity:** HIGH — a security property (AU-9: audit information must be protected) is
contradicted by a design that allows silent degradation of the audit backend.

**Recommended citation:** NIST SP 800-53 AU-9 (Protection of Audit Information — degradation
of audit backend must be observable), NIST SP 800-53 SI-11 (Error Handling — errors must not
be suppressed)

**Recommended fix:** `init_logging` should return `Result<LogBackend, InitError>` or at minimum
`LoggingBackend` (an enum: `Journald | Stderr`) so that callers and the `init_tool` wrapper
can log a warning when the degraded path is taken. The `init_tool` wrapper can then emit
"logging degraded: journald unavailable, falling back to stderr" before returning.

**Remediation owner:** tech-writer (specification); coder (implementation)

---

### Finding CA-7

**Location:** Phase 1, `ScrubReport.ScrubEntry.original_value` (lines 427–433)

**Finding:** `original_value` is typed as `Option<String>` with the comment "None if suppressed
for security." The plan states that `original_value` for denylist vars is never included in
log output (line 440). However, `original_value` is a `pub` field on a `pub struct`, meaning
it is fully accessible to any caller. There is no type-level mechanism preventing a caller
from logging `original_value` directly, passing it to a string formatter, or persisting it in
an audit record.

For denylist entries like `LD_PRELOAD`, `GLIBC_TUNABLES`, or `PYTHONSTARTUP`, the original
value may contain attacker-controlled content. If a downstream caller (e.g., a future
posture-integration path mentioned in Future Considerations) naively logs or persists the
`ScrubReport`, attacker-controlled content from these fields enters the audit log. This is
a log injection surface.

The plan's constraint ("None if suppressed for security") relies entirely on the `scrub_env`
implementation setting `original_value = None` for denylist entries. This is a runtime
behavioral contract with no type-level enforcement.

A stronger design would either:
(a) Use a newtype wrapper `RedactedValue(Option<String>)` with a `Display` impl that always
    prints `"[REDACTED]"`, making it safe to include in logs; or
(b) Document the field with `#[doc(hidden)]` and provide a typed accessor that enforces the
    suppression policy; or
(c) At minimum, add a `#[must_use]` message on `ScrubEntry` warning callers about the
    suppression contract.

**Severity:** MEDIUM

**Recommended citation:** NIST SP 800-53 SI-11 (Error Handling / output information discipline —
attacker-controlled content must not flow into audit records), NIST SP 800-53 AU-9 (Protection
of Audit Information — audit logs must not be injectable)

**Remediation owner:** tech-writer (specification); coder (type design)

---

### Finding CA-8

**Location:** Problem statement, line 17 — cited citations at module entry point

**Finding:** The problem statement cites `NIST IA-5/SC-28/SI-7` (abbreviated form). Per the
Citation Format Rule, all citations in Rust doc comments and design documentation must use the
canonical form `NIST SP 800-53` (not `NIST 800-53`). The abbreviated form `NIST IA-5` will
appear in generated documentation and would not satisfy an assessor searching for the full
control family reference. Additionally, SC-28 in this context is incorrect (see Finding CA-3).

**Severity:** LOW (citation format violation; does not affect implementation correctness)

**Recommended citation:** Use `NIST SP 800-53 SI-7` (correct), replace `NIST IA-5` with
`NIST SP 800-53 CM-7` and `NIST SP 800-53 AC-3` (see Finding CA-3), remove `SC-28`.

**Remediation owner:** tech-writer

---

### Finding CA-9

**Location:** Finding 6 of the security engineer's review (syslog tag / stderr fallback strip)

**Finding:** Decision 6 of the security engineer's review is OPEN — the plan was not updated
to address it. Specifically:

(a) The plan does not state the rationale for the single `"umrs"` syslog identifier. Without
    this, a future implementer may "fix" it by adding per-tool tags, inadvertently breaking
    the unified monitoring approach.

(b) The stderr fallback does not specify that ANSI codes and structured journal fields
    (`CODE_FILE=`, `CODE_LINE=`, etc.) must be stripped from stderr output. If a SIEM or
    log aggregator ingests stderr from a container environment, journald protocol fields
    passed through as plain text create parsing artifacts and potential injection vectors.

Both are documentation gaps in the plan's logging specification. Neither has a compliance
remediation ticket.

**Severity:** LOW

**Recommended citation:** NIST SP 800-53 AU-3 (Content of Audit Records — log records must
be well-formed and non-injectable), NIST SP 800-53 SI-11 (Error Handling — output must not
include structured control data that could be misinterpreted)

**Remediation owner:** tech-writer

---

### Finding CA-10

**Location:** Future Considerations — `detect_secure_execution()` (line 737)

**Finding:** The security engineer's architectural comment (not a severity finding, but
substantive) recommended escalating `detect_secure_execution()` from Future Considerations
to near-term scope for Phase 1. The plan retains it in Future Considerations unchanged.

The architectural concern is well-founded: a UMRS tool invoked via `sudo` or from a
setuid context runs `scrub_env()` with the same posture as a user-invoked instance. Without
AT_SECURE / real-vs-effective-UID detection, the scrubber cannot distinguish between:
- Normal user invocation (expected)
- Privilege elevation in progress (should trigger stricter posture)

Comparing `getuid()` vs `geteuid()` is pure safe Rust with no unsafe code required. The
risk of deferring this is that the first implementation ships without privilege-elevation
awareness. Post-shipping, adding stricter posture for elevated execution is a breaking change
to `ScrubReport` (new fields, new log output) that requires re-testing.

This is classified as LOW because it is an architectural gap in the plan, not a citation or
consistency failure.

**Severity:** LOW

**Recommended citation:** NIST SP 800-53 AC-6 (Least Privilege — tools operating with elevated
privileges must apply stricter posture), NIST SP 800-53 CA-7 (Continuous Monitoring)

**Remediation owner:** tech-writer (escalate to Phase 1 scope in the plan)

---

## Area-by-Area Assessment

### 1. Control Citation Completeness and Accuracy

| Location | Citation | Assessment |
|---|---|---|
| Phase 1 controls block | IA-5 "no secrets in env" | INCORRECT — see Finding CA-3; use CM-7, AC-3 |
| Phase 1 controls block | SC-28 | INCORRECT for this claim — see Finding CA-3 |
| Phase 1 controls block | SI-7, SI-10 | Correct |
| Phase 1 controls block | CWE-526, CERT ENV03-C | Correct |
| Phase 1 controls block | OWASP A2/A3 | Correct as supplementary references |
| `scrub_env_with` doc | IA-5, SC-28, SI-7 | IA-5 and SC-28 incorrect — see Finding CA-3 |
| `init_logging` doc | AU-3, AU-12 | Incomplete — missing AU-8, AU-9, SI-11 (Finding CA-1) |
| `init_i18n` doc | SC-28 | INCORRECT — see Finding CA-2; use NSA RTB RAIN, AU-3 |
| `init_tool` doc | IA-5, SC-28, SI-7, SI-10, AU-3, AU-8, AU-9, AU-12, SI-11 | IA-5 and SC-28 incorrect (carried from sub-functions) |
| `SanitizedEnv` doc | SI-10, SI-7, CWE-526 | Correct |
| `EnvValidationError` doc | SI-10, CWE-526 | Correct |
| Problem statement (line 17) | NIST IA-5/SC-28/SI-7 | Abbreviated form; SC-28 and IA-5 both incorrect |
| USER/LOGNAME validation | AU-3 | Correct |
| Decision 18 (symlink chain) | No citation | Missing; NIST SP 800-53 SI-7, NSA RTB RAIN apply |

### 2. Compliance Claim Substantiation

All three primary claims are substantiated:

- **"Does the tool know what's in its environment?"** — Substantiated by `ScrubReport.stripped`
  vector + `log::warn!` for denylist entries. Controls: SI-7, CWE-526.
- **"Does the tool trust its inputs?"** — Substantiated by `SanitizedEnv` validated accessor
  pattern replacing raw `std::env::var()`. Controls: SI-10.
- **"`#![forbid(unsafe_code)]` fully honored"** — Substantiated by Decision 9 adopting
  read-only snapshot architecture (no `set_var`/`remove_var`). Controls: NIST SP 800-218 SSDF PW.4.

One claim is NOT substantiated: **"silent fallback is unacceptable"** (Decision 14) — the
initialization order places i18n before logging, making the warn undeliverable (Finding CA-4).

### 3. ScrubReport Evidence Trail Sufficiency

The `ScrubReport` struct provides adequate audit trail structure for its intended use cases:
- `stripped` vector — denylist anomalies with name + reason
- `failed_validation` vector — value-level rejections with name + reason
- `reset` vector — Tier 1 resets
- `preserved` vector — allowlist survivors
- `unknown_removed` vector — catch-all unknowns

**Gap 1:** `original_value: Option<String>` is publicly accessible on denylist entries
despite the plan's intent to suppress it. Type-level enforcement is absent (Finding CA-7).

**Gap 2:** No `uid_mismatch` flag. Decision 17 adds UID cross-check for USER/LOGNAME but the
`ScrubReport` struct has no field for this finding. The UID mismatch would currently land in
`failed_validation` as a generic `ScrubEntry`, which is technically sufficient but loses the
semantic specificity needed for AU-3 audit record content (identity accuracy finding vs.
a syntax validation failure are different things to an assessor).

**Gap 3:** No `init_logging` backend indicator in `init_tool` return. If the logging backend
degrades to stderr, the `ScrubReport` does not contain evidence of this. The `ScrubReport`
returned by `init_tool` is the primary audit artifact; it should reflect the full initialization
posture including logging backend state.

### 4. Denylist Entry Justification Traceability

**Complete and correct** for the current Tier 3 table (as updated by post-review decisions):

| Entry | CVE/CWE traceability | Assessment |
|---|---|---|
| `GLIBC_TUNABLES` | CVE-2023-4911 cited explicitly | Complete |
| `LD_PRELOAD`, `LD_LIBRARY_PATH`, `LD_AUDIT` | "Library injection" — glibc AT_SECURE reference | Adequate |
| `LD_ORIGIN_PATH`, `LD_DYNAMIC_WEAK`, `LD_USE_LOAD_BIAS` | RPATH/ASLR manipulation | Adequate |
| `LD_SHOW_AUXV` | "Dumps load addresses and AT_SECURE value" | Adequate |
| `TZDIR` | "Overrides zoneinfo lookup directory — undermines validate_tz()" | Correct and complete; cross-reference to validate_tz() is present |
| `NLSPATH` | "glibc locale message catalog path — path injection" | Adequate |
| `GETCONF_DIR` | "glibc getconf directory override" | Adequate |
| `POSIXLY_CORRECT` | Present in table | NOTE: not present in current Tier 3 table (lines 316–343); `LD_DYNAMIC_WEAK` and `LD_USE_LOAD_BIAS` are also absent. The table includes entries from Finding 1 but does not include all ten entries listed in Finding 1. See note below. |

**NOTE:** Comparing the Tier 3 table (lines 316–343 of the plan) with Finding 1's recommended
additions: `LD_ORIGIN_PATH`, `LD_SHOW_AUXV`, `GLIBC_TUNABLES`, `TZDIR`, `NLSPATH`, `GETCONF_DIR`
are present in the updated table. However, `LD_DYNAMIC_WEAK`, `LD_USE_LOAD_BIAS`, and
`POSIXLY_CORRECT` are absent from the plan's Tier 3 table. These were listed in Finding 1 as
required additions and appear in the security engineer's reference list (lines 504–511 of the
security review) but were not added to the plan's canonical Tier 3 table.

**Severity:** MEDIUM (three denylist entries from Finding 1 not carried into plan)

**Recommended citation:** NIST SP 800-53 SI-7 (runtime configuration integrity), CM-7 (least
functionality)

**Remediation owner:** tech-writer

*(This finding is labeled CA-11 in the gap analysis summary below.)*

### 5. Validation Class Rigor

| Class | Specification completeness | Control citations | Gaps |
|---|---|---|---|
| Safe Path | Thorough: 9 rules, ownership, perms, traversal, NUL, metacharacters, PATH_MAX | None cited in the class spec | Missing SI-10, SI-7 citations on the class definition |
| Colon-Delimited Path List | Thorough: per-component Safe Path + empty/relative/duplicate strip | None cited | Same as Safe Path |
| POSIX Locale | Thorough: regex, known shorthands, codeset warning | None cited | Missing SI-10 |
| Terminal Identifier | Good: charset, length, known-value catalog, `dumb` warning | None cited | Missing SI-10 |
| Positive Integer | Adequate: range 1–9999, no leading zeros | None cited | Missing SI-10 |
| Username | Good: POSIX charset, length, option confusion guard, UID cross-check | AU-3 cited in the table | Correct |
| Boolean Presence | Minimal but correct for NO_COLOR spec | None cited | No citation needed; trivial |
| Timezone | Good: POSIX TZ vs Olson discrimination, traversal block, no absolute paths | None cited | Missing SI-10; TZDIR interaction noted but citation absent |
| D-Bus Address | Good: transport allowlist, path sub-validation, TCP rejection with rationale | None cited in the class spec (Decision 11 adds SC-7 rationale) | SC-7 and CM-7 should appear in the class spec |

The **validation class specifications themselves carry no control citations** — citations appear
only at the module level (Phase 1 controls block) and in the function-level doc comments. Per
the Tiered Annotation Expectations, security-critical functions require explicit control citations.
Each validator function (`validate_safe_path`, `validate_lang`, `validate_tz`, etc.) is a
security-critical function. The plan's function signature specs do not include per-function
citations for the validator functions (only `scrub_env_with` and `EnvValidationError` have them).

This is a systematic gap: the validator functions as specified will be implemented without
per-function control citations, requiring a documentation pass after implementation.

**Severity:** LOW (systematic but indirect; the parent module has citations)

**Recommended citation per validator:**
- `validate_safe_path`, `validate_path_list`: NIST SP 800-53 SI-7, SI-10, NSA RTB RAIN
- `validate_lang`: NIST SP 800-53 SI-10, CM-7
- `validate_term`: NIST SP 800-53 SI-10
- `validate_tz`: NIST SP 800-53 SI-7, SI-10 (with note on TZDIR Tier 3 dependency)
- `validate_dbus_address`: NIST SP 800-53 SC-7, CM-7
- `validate_username`: NIST SP 800-53 AU-3, SI-10
- `validate_positive_int`: NIST SP 800-53 SI-10

*(This is labeled CA-12 in the gap analysis summary below.)*

### 6. Gap Analysis by Assessor Perspective

#### NIST SP 800-53 Assessor

**Uncovered controls (not cited anywhere in the plan):**

| Control | Relevance | Gap |
|---|---|---|
| AC-3 (Access Enforcement) | `/proc/<pid>/environ` access; who can read the raw environment | Not cited; CWE-526 covers the symptom but AC-3 covers the countermeasure |
| AC-6 (Least Privilege) | Strip principle — environment should carry only what is needed | Not cited; CM-7 cited instead (CM-7 is system-scope least functionality; AC-6 is process/user scope) |
| SA-8 (Security and Privacy Engineering Principles) | Secure-by-default startup; composable design | Not cited; applicable to the overall init pattern |
| SC-4 (Information in Shared System Resources) | Process environment as a shared resource between parent and child processes | Not cited; relevant to the trust boundary between inherited and validated env |

**Incorrectly cited controls:** IA-5 (Finding CA-3), SC-28 on i18n (Finding CA-2), SC-28 in
Phase 1 controls block (Finding CA-3).

#### CMMC L2 Assessor

CMMC Level 2 maps to NIST SP 800-171, which in turn maps to NIST SP 800-53 moderate baseline.

**Relevant CMMC domains:**

| CMMC Practice | Mapping | Status |
|---|---|---|
| SI.L2-3.14.1 (Identify and manage info system flaws) | Maps to SI-7 | Present and correct |
| SI.L2-3.14.3 (Monitor security alerts) | Maps to SI-5, CA-7 | CA-7 cited in test coverage; SI-5 not cited |
| CM.L2-3.4.6 (Least functionality) | Maps to CM-7 | CM-7 not cited in plan (should be, per Finding CA-3) |
| AU.L2-3.3.1 (Create and retain audit records) | Maps to AU-3, AU-12 | AU-3 and AU-12 present |
| AU.L2-3.3.2 (Ensure user actions traceable) | Maps to AU-9, AU-12 | AU-9 missing from logging spec (Finding CA-1) |
| IA.L2-3.5.3 (Use multifactor authentication) | Maps to IA-5 | IA-5 incorrectly cited here; does not apply to env scrubbing |

**Overall CMMC L2 assessment:** The core claims are supportable against the relevant CMMC
practices, but the incorrect IA-5 citation would confuse an assessor into looking for
authenticator management evidence in the env scrubbing module. AU.L2-3.3.2 (user action
traceability) is partially covered but requires AU-9 citation fix.

#### NSA RTB Reviewer

NSA RTB principles relevant to this module:

| RTB Principle | Claim | Status |
|---|---|---|
| RAIN (Non-Bypassability) | `SanitizedEnv.get()` is the only sanctioned env access path | Well-stated in architecture; Decision 9 makes it enforceable. Citation absent from validator function specs (Finding CA-12). |
| RAIN (Non-Bypassability) | i18n domain derived from `/proc/self/exe`, not environment | Correct design (Decision 14 implicitly); but NSA RTB RAIN citation missing from `init_i18n` spec (Finding CA-2). |
| Fail Secure | `init_logging` falls back to stderr silently | NOT fail-secure — Finding CA-6. Silent degradation is not acceptable under Fail Secure. |
| Tamper Evidence | `ScrubReport` as audit artifact | Good structure. `original_value` public field weakens tamper evidence for log injection (Finding CA-7). |
| Least Privilege | Tier 3 strip removes what is not needed | Correct. Three entries missing: `LD_DYNAMIC_WEAK`, `LD_USE_LOAD_BIAS`, `POSIXLY_CORRECT` (Finding CA-4 denylist note above). |

### 7. Documentation Artifact Requirements for Audit Readiness

For a module of this security significance in a DoD/CUI environment, the following artifacts
are required at implementation time:

| Artifact | Required by | Status | Gap |
|---|---|---|---|
| System Security Plan (SSP) control implementation statements for AU-3, AU-9, AU-12 | NIST SP 800-53 CA-1 | Not present | The logging module's AU-3/AU-12 implementation must be documented as a control implementation statement in the SSP. Currently the plan is the only specification. |
| Developer guide section on tool initialization | Cited in File Changes (line 730) | Planned | No content yet; must be written before tools implement `init_tool()` or validation drift occurs. |
| Validator API reference | Cited in File Changes (line 730) | Planned | No content yet. |
| Denylist change log | Not specified | Missing | When new CVEs are discovered (next `LD_*` exploit, next glibc tunable), there must be a process for updating Tier 3 and verifying the test suite. No maintenance process is defined. |
| Threat model documenting trust boundary between inherited env and `SanitizedEnv` | Not specified | Missing | For audit review, the trust boundary must be explicitly stated. The plan describes it implicitly but no threat model document is referenced. |

---

## Gap Analysis Summary

```
Files reviewed: 2
  - .claude/plans/umrs-tool-init.md
  - .claude/reports/2026-03-17-umrs-tool-init-security-review.md (prior review baseline)

Total findings: 12 (2 HIGH, 5 MEDIUM, 5 LOW)
  CA-1: MEDIUM — init_logging doc comment incomplete (AU-8, AU-9, SI-11 missing)
  CA-2: MEDIUM — init_i18n cites SC-28 (incorrect); should cite NSA RTB RAIN, AU-3
  CA-3: MEDIUM — IA-5 and SC-28 incorrectly cited for "no secrets in env" claim
  CA-4: HIGH   — init_i18n warn-on-fallback contradicted by initialization order
  CA-5: MEDIUM — test list not updated for Decisions 9–18 (3 test gaps + test 9 under-spec)
  CA-6: HIGH   — init_logging return type () hides journald degradation (AU-9 gap)
  CA-7: MEDIUM — ScrubReport.original_value publicly accessible; no type-level suppression
  CA-8: LOW    — Abbreviated NIST citation in problem statement (format violation)
  CA-9: LOW    — Security engineer Finding 6 (syslog tag rationale / stderr strip) open
  CA-10: LOW   — detect_secure_execution() should be Phase 1 scope, not Future
  CA-11: MEDIUM — LD_DYNAMIC_WEAK, LD_USE_LOAD_BIAS, POSIXLY_CORRECT absent from Tier 3 table
  CA-12: LOW   — Validator functions lack per-function control citations

Uncited security claims:
  - "silent fallback is unacceptable" (Decision 14) — contradicted by init order (CA-4)
  - "no secrets in env" — IA-5 incorrect; AC-3 and CM-7 more precise (CA-3)
  - journald-native logging provides audit integrity — no AU-9 citation (CA-1, CA-6)
  - i18n domain is provenance-safe — NSA RTB RAIN not cited (CA-2)
  - detect_secure_execution() is deferred — AC-6 not cited (CA-10)

Inconsistencies (plan vs. design):
  - init_i18n Decision 14 ("warn on fallback") vs. initialization order (logging initialized
    after i18n — warn never delivered) [CA-4]
  - init_logging return type () vs. AU-9 requirement that audit backend degradation is
    observable [CA-6]
  - Tier 3 table missing 3 of 10 entries from Finding 1 of security engineer review [CA-11]
  - "init_logging needs AU-8, AU-9, SI-11" in Implementation Notes vs. init_logging doc
    comment that still cites only AU-3 and AU-12 [CA-1]

Open from prior security engineer review (unresolved):
  - Finding 6: syslog tag rationale not documented; stderr fallback field-stripping not specified
  - Finding 11: test 9 under-specified; no GLIBC_TUNABLES test case

All findings are plan-level specifications. No source code exists yet.
```

---

## Recommendations Summary

**Priority order for remediation before implementation begins:**

1. **CA-4 (HIGH):** Resolve the init_i18n initialization order contradiction. Either change
   return type to `Result`, swap init order (logging before i18n), or document that the
   fallback warn uses `eprintln!` explicitly.

2. **CA-6 (HIGH):** Change `init_logging` to return `LoggingBackend` or `Result<LoggingBackend, InitError>`
   so journald degradation is visible to `init_tool` and to the caller.

3. **CA-11 (MEDIUM):** Add `LD_DYNAMIC_WEAK`, `LD_USE_LOAD_BIAS`, and `POSIXLY_CORRECT` to
   the Tier 3 table. These were in Finding 1 of the security engineer's review but are absent
   from the plan's canonical table.

4. **CA-2 and CA-3 (MEDIUM):** Correct IA-5 and SC-28 citations throughout. These incorrect
   citations are load-bearing in the sense that a controls assessor will look for the wrong
   evidence. Correct citations: CA-2 → NSA RTB RAIN + AU-3; CA-3 → CM-7 + AC-3.

5. **CA-1 (MEDIUM):** Update `init_logging` doc comment to include AU-8, AU-9, SI-11.

6. **CA-5 (MEDIUM):** Add missing tests for D-Bus TCP rejection, USER/LOGNAME UID cross-check,
   and GLIBC_TUNABLES denylist. Expand test 9 to snapshot and assert post-scrub environment.

7. **CA-7 (MEDIUM):** Add a `RedactedValue` newtype or access control to `original_value`
   to prevent accidental log injection via attacker-controlled denylist values.

8. **CA-8, CA-9, CA-10, CA-12 (LOW):** Fix citation format in problem statement; document
   syslog tag rationale and stderr stripping; elevate `detect_secure_execution()` to Phase 1;
   add per-function citations to validator function specs.
