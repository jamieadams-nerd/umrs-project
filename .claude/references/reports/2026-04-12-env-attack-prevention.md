# Environment-Variable Attack Prevention Reference
**Prepared by:** The Librarian (researcher agent)
**Date:** 2026-04-12
**For:** umrs-core::init environment audit and sanitization work
**Plan reference:** .claude/plans/umrs-tool-init.md Phase 3 env corpus research
**Background source:** .claude/jamies_brain/archive/env-scrubbing.txt

---

## Executive Summary

The Unix process environment is one of the oldest and most persistently exploited attack
surfaces in systems security. This report covers six categories of guidance: historical
attacks and CVEs, authoritative coding standards, NIST/government controls, OS/runtime
hardening mechanisms, language-specific notes, and secrets-in-env critique. A prioritized
reading list for the rust-developer precedes Phase 1a, and a RAG ingestion shortlist with
rationale closes the report.

---

## Category 1 -- Historical Attacks and CVEs

### 1.1 CVE-2023-4911 "Looney Tunables" (GLIBC_TUNABLES)

CVE: CVE-2023-4911
CVSS v3.1: 7.8 HIGH (AV:L/AC:L/PR:L/UI:N/S:U/C:H/I:H/A:H)
CWE: CWE-787 (Out-of-bounds Write), CWE-122 (Heap-based Buffer Overflow)
Discovered: Qualys Threat Research Unit, disclosed 2023-10-03
Added to CISA KEV: Yes
Affected glibc: 2.34 through 2.38
Affected distributions: Fedora 37/38, Ubuntu 22.04/23.04, Debian 12/13, RHEL 8/9
Immune: Alpine Linux (uses musl libc, not glibc)

Technical mechanism: GLIBC_TUNABLES was introduced in glibc circa April 2021 to allow
runtime tuning of glibc behavior without recompilation. The dynamic loader's
parse_tunables() function contained a buffer overflow: if the tunable string was in the
form "tunable1=tunable2=value", the parser treated tunable1 as having value "tunable2=value"
and continued processing, copying more data into the allocated buffer than was permitted.
An attacker could overwrite the pointer to the library search path, causing ld.so to load
a malicious libc.so from an attacker-controlled location. When the exploit runs against any
SUID binary, the malicious library executes with elevated privileges.

UMRS applicability: RHEL 10 ships glibc patched for this. However, GLIBC_TUNABLES is a
high-risk variable that UMRS tools must classify as Tier 1 -- Never Expected in the scrub
report. Its mere presence in the environment of a security tool at runtime is a finding.

Sources:
- https://nvd.nist.gov/vuln/detail/CVE-2023-4911
- https://blog.qualys.com/vulnerabilities-threat-research/2023/10/03/cve-2023-4911-looney-tunables-local-privilege-escalation-in-the-glibcs-ld-so
- https://www.qualys.com/2023/10/03/cve-2023-4911/looney-tunables-local-privilege-escalation-glibc-ld-so.txt

---

### 1.2 CVE-2024-48990 / CVE-2024-48992 (needrestart -- PYTHONPATH / RUBYLIB cluster)

CVE cluster: CVE-2024-48990 (PYTHONPATH), CVE-2024-48992 (RUBYLIB),
             CVE-2024-10224 (PERL5OPT), CVE-2024-11003 (Perl regex), CVE-2024-48991 (race)
CVSS v3.1: 7.8 HIGH (AV:L/AC:L/PR:L/UI:N/S:U/C:H/I:H/A:H)
CWE: CWE-427 (Uncontrolled Search Path Element)
Discovered: Qualys, disclosed 2024-11-19
Affected: Ubuntu Server 21.04 through 24.04 LTS (needrestart installed by default)

Technical mechanism: needrestart is a utility that determines whether services need
restarting after package upgrades. It scans running processes by reading
/proc/pid/environ and extracting environment variables from those processes. It then
spawns interpreter processes (Python, Ruby, Perl) with environment variables sourced
from unprivileged processes -- including PYTHONPATH, RUBYLIB, and PERL5OPT. Since
needrestart runs as root, the interpreter inherits a root-level execution context with
attacker-controlled library search paths. An attacker plants a fake importlib/__init__.so
in a directory they control, sets PYTHONPATH accordingly in a long-running bait process,
and when needrestart runs it executes the malicious module as root.

Core lesson: Reading /proc/pid/environ to inherit environment from untrusted processes
and passing those values to privileged execution is a serious vulnerability. Any privileged
component that absorbs environment variables from lower-privilege processes must validate
or discard all *PATH* variables.

Sources:
- https://nvd.nist.gov/vuln/detail/CVE-2024-48990
- https://ubuntu.com/blog/needrestart-local-privilege-escalation
- https://www.qualys.com/needrestart

---

### 1.3 CVE-2014-6271 "Shellshock" (Bash function export via env)

CVE cluster: CVE-2014-6271, CVE-2014-7169, CVE-2014-7186, CVE-2014-7187,
             CVE-2014-6277, CVE-2014-6278
CVSS: Critical (remote code execution; exploited within hours of disclosure)
Discovered: Stephane Chazelas, 2014-09-12; public 2014-09-24

Technical mechanism: Bash exported function definitions to child processes via environment
variables named after the function (e.g., "x=() { : ; }; echo vulnerable"). The flaw was
that Bash continued executing commands appended after the closing brace. Any daemon, CGI
handler, or SUID program that passed environment variables to a child Bash process was
exploitable -- including Apache mod_cgi, OpenSSH ForceCommand, and Git shell restrictions.

IFS connection: The IFS (Internal Field Separator) variable was historically abused in
system() calls -- setting IFS="/" could split "/bin/sh" into "bin sh" as separate tokens,
causing argument injection even when the command string appeared clean.

UMRS applicability: UMRS tools do not spawn bash child processes, but the lesson is
general: any variable that influences a child shell or interpreter must be explicitly
allowed or stripped. BASH_ENV (sourced by bash in non-interactive mode) and ENV (POSIX
sh equivalent) are direct execution vectors -- classify as Tier 1 Never Expected.

Sources:
- https://nvd.nist.gov/vuln/detail/cve-2014-6271
- https://access.redhat.com/articles/1200223
- https://www.cisa.gov/news-events/alerts/2014/09/25/gnu-bourne-again-shell-bash-shellshock-vulnerability-cve-2014-6271-cve-2014-7169-cve-2014-7186-cve

---

### 1.4 CVE-2017-17562 (GoAhead -- LD_PRELOAD via CGI environment)

CVE: CVE-2017-17562
CVSS: 9.8 CRITICAL (remote)

GoAhead web server (before 3.6.5) initialized CGI script environments from untrusted HTTP
request parameters. Attackers sent HTTP requests with LD_PRELOAD parameter names pointing
to /proc/self/fd/0 (the request body containing a malicious shared object). Demonstrates
remote LD_PRELOAD injection when environment initialization fails to allowlist variable names.

Source: https://www.elttam.com/blog/goahead/

---

### 1.5 CVE-2005-4158 (sudo Perl -- PERLLIB / PERL5LIB / PERL5OPT not cleared)

CVE: CVE-2005-4158
Affected: sudo before 1.6.8 p12

When sudo ran Perl scripts with the Perl taint flag off, it failed to clear PERLLIB,
PERL5LIB, and PERL5OPT. An attacker could inject a fake module in /tmp/root.pm and set
PERL5OPT=-Mroot PERL5LIB=/tmp to execute arbitrary code with sudo privileges.

Source: https://www.sudo.ws/security/advisories/perl_env/

---

### 1.6 CVE-2025-6018 / CVE-2025-6019 (pam_env -- ~/.pam_environment injection)

CVE cluster: CVE-2025-6018, CVE-2025-6019
Affected: PAM 1.3.0 through 1.6.0

The pam_env PAM module allowed environment variable injection via ~/.pam_environment,
leading to privilege escalation through systemd session manipulation. User-supplied
environment configuration in ~/.pam_environment was deprecated in pam_env 1.5.0 and will
be removed entirely. Illustrates why even configuration-file-sourced environment variables
from user-controlled paths are privilege escalation vectors.

Source: https://www.exploit-db.com/exploits/52386

---

### 1.7 NLSPATH buffer overflows (historical -- HP Tru64, multiple Unix)

NLSPATH tells catopen() where to find localized message catalogs. On HP Tru64 and similar
systems, numerous setuid binaries trusted NLSPATH without bounds-checking the buffer into
which the catalog path was copied, producing local root exploits. glibc secure-execution
mode now strips NLSPATH for SUID processes, but the historical record explains why
locale-related variables deserve explicit Tier 1 classification.

Source: https://www.exploit-db.com/exploits/21772

---

### 1.8 LD_PRELOAD / LD_LIBRARY_PATH -- Threat Actor Usage (ATT&CK T1574.006)

MITRE ATT&CK T1574.006 "Dynamic Linker Hijacking" catalogs observed threat actor usage:

- APT41: configured payloads to load via LD_PRELOAD
- Ebury: hooked libc system(), popen(), execve() in SSH sessions via LD_PRELOAD
- HiddenWasp: added itself to LD_PRELOAD
- Hildegard: modified /etc/ld.so.preload for container persistence
- Rocke: hooked libc to hide malware from process lists
- COATHANGER: copied malicious files to /lib/preload.so, injecting into PID 1
- XCSSET: set DYLD_FRAMEWORK_PATH and DYLD_LIBRARY_PATH (macOS equivalent)
- Aquatic Panda: modified ld.so preload file for Winnti malware persistence

Mitigations in ATT&CK: M1038 (application control blocking malicious libs), M1028 (OS
configuration: SELinux AT_SECURE stripping on domain transitions).

Detection: AN1209 -- monitor for unexpected LD_PRELOAD in shell scripts, anomalous .so
creation, execve events.

Source: https://attack.mitre.org/techniques/T1574/006/

---

### 1.9 PATH Trojan Horses (ATT&CK T1574.007)

Classic attack: a setuid binary calls system("ls") or execlp("ls") without a full path.
An attacker who controls PATH (or has placed "." early in PATH) puts a malicious "ls" in
an earlier directory. A 2001 Oracle case involved ORACLE_HOME controlling the path to
changepw, executed as root by the setuid dbsnmp binary.

MITRE ATT&CK T1574.007 covers this class.
Source: https://attack.mitre.org/techniques/T1574/007/

---

### 1.10 JAVA_TOOL_OPTIONS / _JAVA_OPTIONS (Java agent injection)

Java's JAVA_TOOL_OPTIONS and _JAVA_OPTIONS allow injecting JVM arguments including
-javaagent:<path> and OnOutOfMemoryError handlers that execute arbitrary code. Any Java
process in an environment where these variables are attacker-controlled is vulnerable to
arbitrary agent injection without modifying the JVM command line.

Source: https://docs.oracle.com/en/java/javase/26/troubleshoot/environment-variables-system-properties.html

---

### 1.11 Novel Cross-Interpreter Attack Chains (elttam, 2020)

Source: https://www.elttam.com/blog/env

The elttam research demonstrates cross-interpreter attack chains using environment variables
that go beyond the well-known LD_PRELOAD class:

- PERL5OPT=-Mbase;print(`id`);exit -- executes arbitrary code during Perl module load
- NODE_OPTIONS=--require /proc/self/environ -- injects JavaScript via proc filesystem
- PHPRC=/proc/self/environ combined with PHP directives in HOSTNAME -- PHP config injection
- RUBYOPT=-r... combined with BASH_FUNC_declare%%=() { id; exit; } -- shell function override
- PYTHONWARNINGS chained with BROWSER=perlthanks -- cross-interpreter payload chain

These attack chains establish that any variable beginning with LD_, PYTHON, RUBY, PERL,
NODE_, PHP, JAVA_, _JAVA should be treated as suspicious by default.

---

## Category 2 -- Authoritative Coding Guidance

### 2.1 SEI CERT C Coding Standard -- ENV Rules

Source: https://wiki.sei.cmu.edu/confluence/spaces/c/pages/87152421/Rule+10.+Environment+ENV
GitHub Pages mirror: https://cmu-sei.github.io/secure-coding-standards/sei-cert-c-coding-standard/

ENV30-C -- Do not modify the object referenced by the return value of getenv().
The string returned by getenv() must be treated as read-only. Modifying it produces
undefined behavior and can corrupt the environment list.

ENV31-C -- Do not rely on an environment pointer following an operation that may
invalidate it. After setenv(), putenv(), or unsetenv(), the envp pointer passed to main()
may no longer point to the current environment.

ENV32-C -- All exit handlers must return normally.
Exit handlers registered with atexit() must not call exit() or longjmp(). Affects
cleanup code in security-sensitive teardown paths.

ENV33-C -- Do not call system().
system() passes the command to "/bin/sh -c", which processes IFS, PATH, and shell
metacharacters. Even a sanitized command string can be reinterpreted if environment
variables are attacker-controlled. Use execve() family with an explicit environment array.
Risk: HIGH, Likelihood: Likely, Priority: P9 L2.
Clang-tidy check: cert-env33-c.
Related CWEs: CWE-78 (OS command injection), CWE-88 (argument injection), CWE-426 (untrusted
search path).

ENV34-C -- Do not store pointers returned by getenv(), setlocale(), strerror(), strsignal().
These return pointers to internal static buffers that may be overwritten by subsequent calls.
Store a copy via strdup() if the value must persist.

ENV03-C (Recommendation) -- Sanitize the environment when invoking external programs.
The canonical recommendation: call clearenv() to remove all variables, then use
confstr(_CS_PATH, ...) to obtain the certified system PATH, then setenv("PATH", ...),
setenv("IFS", " \t\n", 1), and only then call system() or exec(). The non-compliant
example shows how attacker-controlled IFS can break a shell command even when the command
string itself is clean.

ENV01-C (Recommendation) -- Do not make assumptions about the size of an environment
variable. Values have no defined maximum length.

ENV02-C (Recommendation) -- Beware of multiple environment variables with the same
effective name. On some platforms, getenv() returns the first match. If an attacker can
inject a duplicate name, getenv() may return the attacker's value.

---

### 2.2 CWE Entries Directly Relevant to Environment Variables

CWE-526 -- Cleartext Storage of Sensitive Information in an Environment Variable
https://cwe.mitre.org/data/definitions/526.html
"The product uses an environment variable to store unencrypted sensitive information."
Exposure paths: /proc/pid/environ, process listing tools (ps), crash dumps, inherited
environments, container orchestration metadata, debug tooling.
Parent: CWE-312 (Cleartext Storage). Related: CWE-214 (Invocation Using Visible Sensitive
Information).

CWE-454 -- External Initialization of Trusted Variables or Data Stores
https://cwe.mitre.org/data/definitions/454.html
"The product initializes critical internal variables or data stores using inputs that can
be modified by untrusted actors." The environment is an external, untrusted initialization
vector. Treating HOME, PATH, or LANG as trusted without validation violates this principle.

CWE-807 -- Reliance on Untrusted Inputs in a Security Decision
https://cwe.mitre.org/data/definitions/807
Using an environment variable to gate a security decision (e.g., if DEBUG_MODE=1 skip auth)
is a CWE-807 violation.

CWE-427 -- Uncontrolled Search Path Element
https://cwe.mitre.org/data/definitions/427.html
Directly covers PATH, LD_LIBRARY_PATH, PYTHONPATH, PERL5LIB, RUBYLIB. Allows attackers to
substitute malicious code for legitimate libraries or executables. Used as the CWE for
CVE-2024-48990.

CWE-15 -- External Control of System or Configuration Setting
Addresses the broader class of environment variables that control system behavior (locale
settings, timezone, debug modes, temp directories).

---

### 2.3 OWASP ASVS V14 -- Configuration Requirements

Source: https://github.com/OWASP/ASVS/blob/master/4.0/en/0x22-V14-Config.md

14.1.1: Secure, repeatable build processes (CI/CD, config management). Production
environments must not inherit developer or CI environment variables.

14.1.2: Compiler hardening -- stack randomization, data execution prevention, fail on
unsafe operations. Directly relevant to the buffer-overflow class of env var attacks.

14.1.3: Harden server config per vendor recommendations -- for glibc/systemd this
includes environment isolation for service units.

14.1.5: Verify integrity of security-relevant configurations to detect tampering (Level 3).
An audit of the process environment at startup partially satisfies this.

14.3.2: Disable debug modes in production -- directly relevant to suppressing LD_DEBUG,
LD_DEBUG_OUTPUT, and MALLOC_TRACE in production UMRS deployments.

ASVS V14 does not have a dedicated environment variable section, but the requirements
collectively imply: production processes must not carry developer-environment variables
(LD_PRELOAD, LD_DEBUG, MALLOC_TRACE), debug flags, or secrets.

---

## Category 3 -- NIST and Government Guidance

### 3.1 NIST SP 800-53 Rev 5 -- Directly Applicable Controls

CM-7 -- Least Functionality
Prohibit or restrict functions, ports, protocols not required. Environment variables
enabling developer-only features (LD_DEBUG, MALLOC_TRACE, GLIBC_TUNABLES) must be treated
as prohibited functionality in production execution contexts.
Source: https://csf.tools/reference/nist-sp-800-53/r5/cm/cm-7/

AC-3 -- Access Enforcement
Enforce approved authorizations for access to system resources. A validated SanitizedEnv
that refuses to expose raw, unvalidated env var values is an access enforcement mechanism
for process configuration state.

SI-7 -- Software, Firmware, and Information Integrity
Prevent unauthorized modification of runtime configuration. Environment variables are
runtime configuration injection points; detecting unexpected variables at startup satisfies
SI-7's intent for runtime integrity verification.

SI-10 -- Information Input Validation
Check validity of information inputs. The validate_lang(), validate_safe_path(), and all
other validators in the plan are direct SI-10 implementations against environment variable
input values.

SC-3 -- Security Function Isolation
Isolate security functions from nonsecurity functions. UMRS security tools must not
execute in an environment that could allow adversarial configuration to influence security
decisions.
Source: https://csf.tools/reference/nist-sp-800-53/r5/sc/sc-3/

SC-39 -- Process Isolation
Each process in a distinct address space. Environment scrubbing at process start is the
application-level boundary for this guarantee.

IA-5 -- Authenticator Management
Authentication credentials must be protected from unauthorized disclosure. Storing
credentials in environment variables violates IA-5.

SC-28 -- Protection of Information at Rest
Sensitive information in areas not designed for confidentiality protections (including
process environments) violates SC-28.

---

### 3.2 NIST SP 800-218 SSDF -- Produce Well-Secured Software

PW.4.1 -- Use established, vetted code for security functionality. For environment
handling, this means using secure_getenv() or equivalent rather than ad-hoc getenv()
calls in security-sensitive paths. Validated accessor types implement PW.4.1.

PW.6 / PW.7 -- Code review and security testing. The scrub_env() implementation must be
reviewed against the complete dangerous variables list as part of PW.7 security testing.

---

### 3.3 NSA RTB Principles

RAIN -- Non-Bypassability: Security checks on environment variables must be non-bypassable.
The plan correctly places the environment audit at the top of init_tool(), before any other
work. No code path can use an unvalidated env var.

Fail Closed: If validate_safe_path() cannot verify that PATH entries are root-owned and
non-world-writable, it must fail (return an error) rather than pass the value through.
The scrub report records the failure as a security finding.

Least Privilege: Any subprocess spawned by a UMRS tool must receive Command::env_clear()
plus an explicitly-constructed minimal environment. The privileged environment of the
parent must not be inherited.

---

### 3.4 DISA STIG / SCAP-Security-Guide -- Environment-Adjacent Items

The RHEL 10 STIG profile does not have a dedicated CCE for "process environment
sanitization at startup" because that is application-level behavior. Several STIG rules
create the system conditions that make environment attacks harder:

- FIPS mode (fips=1 kernel cmdline): Forces glibc secure-execution mode for
  capability-elevated processes.
- noexec on /tmp: Prevents attacker from placing executable malicious .so files in /tmp
  for LD_PRELOAD attacks.
- systemd PrivateTmp=true: Each service gets a private /tmp, blocking the "plant a
  malicious library in /tmp" attack vector.
- SELinux enforcing mode: Constrains which processes can modify /etc/ld.so.preload.

The absence of a specific STIG CCE means that UMRS's scrub_env() provides a control that
no STIG rule covers at the application level -- a genuine gap that UMRS fills.

---

### 3.5 CIS Benchmarks

CIS Linux benchmarks (Level 1) require:
- /etc/environment and /etc/profile.d/ not to inject dangerous variables
- PATH in login profiles not to include "." (current directory)
- Auditing of changes to /etc/profile, /etc/profile.d/, /etc/environment

CIS guidance supports the environmental baseline but does not address application-level
env audit, which UMRS fills.

---

## Category 4 -- OS / glibc / systemd / Kernel Hardening

### 4.1 glibc Secure-Execution Mode (AT_SECURE)

Source: https://man7.org/linux/man-pages/man8/ld.so.8.html

A binary enters secure-execution mode when the kernel sets AT_SECURE = 1 in the ELF
auxiliary vector. Conditions:
1. The process's real UID/GID differ from effective UID/GID (setuid/setgid binary).
2. A non-root process executes a binary that grants capabilities.
3. A Linux Security Module explicitly sets AT_SECURE = 1 (SELinux domain transitions
   can trigger this).

In secure-execution mode, ld.so voids or strips these environment variables before the
binary's code runs:

  GCONV_PATH, GETCONF_DIR, HOSTALIASES, LOCALDOMAIN,
  LD_AUDIT, LD_DEBUG, LD_DEBUG_OUTPUT, LD_DYNAMIC_WEAK, LD_HWCAP_MASK,
  LD_LIBRARY_PATH, LD_ORIGIN_PATH, LD_PRELOAD, LD_PROFILE, LD_SHOW_AUXV,
  LOCPATH, MALLOC_TRACE, NIS_PATH, NLSPATH, RESOLV_HOST_CONF, RES_OPTIONS,
  TMPDIR, TZDIR

IMPORTANT: Secure-execution mode only activates for setuid/setgid programs or
capability-granted execution. A UMRS tool running as a normal user process (uid == euid)
does NOT get AT_SECURE = 1. These variables are NOT automatically stripped.
Application-level scrubbing is therefore necessary for non-setuid security tools.

secure_getenv(3) is the glibc API that returns NULL when AT_SECURE is set, allowing
libraries to self-protect without the application knowing whether it is setuid. In Rust
this requires "unsafe { libc::secure_getenv(...) }". UMRS tools follow
#![forbid(unsafe_code)] so this path is not directly available; the application-level
scrubber provides the equivalent defense.

Sources:
- https://man7.org/linux/man-pages/man8/ld.so.8.html
- https://linux.die.net/man/3/secure_getenv
- https://man.archlinux.org/man/secure_getenv.3.en

---

### 4.2 Linux no_new_privs (prctl PR_SET_NO_NEW_PRIVS)

Source: https://docs.kernel.org/userspace-api/no_new_privs.html

prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0) sets a bit inherited across fork(), clone(), and
execve() that cannot be unset. With this flag:
- setuid/setgid bits on executed binaries no longer change effective UID/GID
- File capabilities cease to expand the permitted capability set
- LSMs refrain from relaxing restrictions post-execution

This is the kernel's generalization of the ad-hoc LD_PRELOAD stripping for setuid programs.
It provides a process-level guarantee that child processes cannot gain new privileges even if
the parent placed LD_PRELOAD in their environment.

UMRS tools should consider calling prctl(PR_SET_NO_NEW_PRIVS, 1) early in init_tool() as
an additional defense layer.

Caveat: no_new_privs does not prevent privilege changes via setuid(2) directly (only
applies to execve). It may also interfere with SELinux domain transitions via exec in
edge cases.

Rust access: libc::prctl(libc::PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0) requires unsafe. The
nix crate provides a safer ergonomic wrapper.

---

### 4.3 systemd Service Environment Hardening

Sources:
- https://docs.rockylinux.org/10/guides/security/systemd_hardening/
- https://wiki.archlinux.org/title/Systemd/Sandboxing
- https://linux-audit.com/systemd/settings/units/protectproc/

systemd unit file directives relevant to environment isolation:

Environment=VAR=value
  Sets specific environment variables for the service. Acts as a controlled allowlist.

PassEnvironment=VAR
  Passes named variables from the system environment. Without this, service processes do
  not inherit any system environment variables by default.

UnsetEnvironment=VAR
  Explicitly unsets specific variables even if PassEnvironment would otherwise pass them.

PrivateTmp=true
  Service gets a private /tmp namespace, eliminating the "plant .so in /tmp" attack vector.

ProtectProc=invisible
  Processes owned by other users are hidden from /proc/. Prevents an attacker process
  from reading /proc/service-pid/environ.

NoNewPrivileges=true
  Sets PR_SET_NO_NEW_PRIVS on the service process (see 4.2).

For UMRS tools packaged as systemd services, the unit file should include at minimum:
  Environment=LANG=en_US.UTF-8
  UnsetEnvironment=LD_PRELOAD LD_LIBRARY_PATH GLIBC_TUNABLES LD_AUDIT LD_DEBUG
  PrivateTmp=true
  ProtectProc=invisible
  NoNewPrivileges=true

---

### 4.4 pam_env and /etc/environment

pam_env sets environment variables during PAM authentication based on
/etc/security/pam_env.conf and /etc/environment. The user-specific ~/.pam_environment
was deprecated in pam_env 1.5.0 (CVE-2025-6018, CVE-2025-6019 illustrate the attack).

For UMRS tools, /etc/environment is the OS-level baseline environment. Its contents should
be treated as a potential source of attacker-controlled variables in threat models where an
attacker has file-write capability in /etc/. UMRS's environment audit at startup provides
the detection layer.

---

### 4.5 SELinux and LD_PRELOAD

SELinux addresses LD_PRELOAD attacks through two complementary mechanisms:

1. AT_SECURE on domain transitions: When SELinux performs a type enforcement domain
   transition (exec of a process labeled differently from the parent), it sets AT_SECURE = 1
   in the aux vector. This causes glibc to strip the 22 dangerous variables listed in 4.1.

2. File context protection of /etc/ld.so.preload: A custom SELinux policy module can assign
   a restrictive type (etc_nowrite_t) to /etc/ld.so.preload and deny write access to all
   domains except the policy management domain. Combined with the secure_mode_policyload
   boolean (which prevents policy unloading in permissive mode), this makes persistent
   LD_PRELOAD modification much harder.

AppArmor note: lowercase exec modes (px, cx) do NOT scrub the environment. Only uppercase
modes (Px, Cx, Ux) invoke the kernel's unsafe_exec routines, scrubbing environment variables
analogously to the setuid path.

Sources:
- https://www.defensive-security.com/blog/preventing-modification-of-etcldsopreload-with-selinux
- https://forums.whonix.org/t/eliminate-ld-preload-and-other-dangerous-environment-variables/10594
- https://doc.opensuse.org/documentation/leap/security/html/book-security/cha-apparmor-profiles.html

---

### 4.6 Complete Reference -- Variables Stripped by glibc AT_SECURE

For the UMRS scrub implementation, this is the canonical list from ld.so(8). Any of these
appearing in the environment of a non-setuid UMRS tool warrants a WARN classification in
the scrub report.

Variable          | Risk vector
------------------|-------------------------------------------------------------------
GCONV_PATH        | Character set conversion library path injection
GETCONF_DIR       | getconf(1) configuration directory injection
GLIBC_TUNABLES    | glibc runtime parameter injection (CVE-2023-4911)
HOSTALIASES       | Hostname resolution table injection
LD_AUDIT          | Dynamic linker audit module injection
LD_DEBUG          | Dynamic linker debug output (information leak, race widening)
LD_DEBUG_OUTPUT   | LD_DEBUG output file path injection
LD_DYNAMIC_WEAK   | Changes library symbol resolution semantics
LD_HWCAP_MASK     | Modifies hardware capability mask used by linker
LD_LIBRARY_PATH   | Library search path injection
LD_ORIGIN_PATH    | $ORIGIN resolution override injection
LD_PRELOAD        | Library preload injection (APT41, Ebury, HiddenWasp, Rocke)
LD_PROFILE        | Dynamic linker profiling target injection
LD_SHOW_AUXV      | Dumps auxiliary vector (information leak)
LOCPATH           | Locale data directory injection
MALLOC_TRACE      | Memory allocation trace file path injection
NIS_PATH          | NIS database search path injection
NLSPATH           | Message catalog path injection (historical buffer overflows)
RESOLV_HOST_CONF  | DNS resolver hostname configuration injection
RES_OPTIONS       | DNS resolver options injection
TMPDIR            | Temporary directory path injection (symlink attacks)
TZDIR             | Timezone data directory injection

---

## Category 5 -- Language-Specific Guidance

### 5.1 Rust -- std::env and the Rust 2024 Edition Change

Source: https://doc.rust-lang.org/edition-guide/rust-2024/newly-unsafe-functions.html
Tracking issue: https://github.com/rust-lang/rust/issues/124866

std::env::set_var and std::env::remove_var became "unsafe" in Rust 2024 Edition because:
"It can be unsound to call std::env::set_var or std::env::remove_var in a multithreaded
program due to safety limitations of the way the process environment is handled on some
platforms."

Why this matters for UMRS:

1. POSIX setenv()/putenv() are not thread-safe. Any concurrent call to getenv() by a
   spawned thread or a library (including logger initialization) while the main thread
   mutates the environment via set_var() is a data race. The Rust stdlib uses an internal
   mutex, but only protects calls from std functions -- any libc::getenv() call from a C
   library bypasses the mutex.

2. ScrubReport and SanitizedEnv are read-only snapshots -- they never call set_var().
   This is the correct design. The scrub implementation must call std::env::vars() once at
   process start (before thread spawning) to take a snapshot.

3. Removing env vars from the parent process environment is explicitly NOT the UMRS design
   (the plan states "zero side effects -- the parent process is untouched"). This is the
   safe and correct choice for Rust 2024.

Command::env_clear() is the right API for child process isolation. It constructs a new
environment for the child without touching the parent. No unsafe required.

---

### 5.2 Rust -- secure_getenv Equivalent

secure_getenv(3) from glibc returns NULL when AT_SECURE is set (running setuid or with
elevated capabilities). Rust does not expose this directly.

UMRS tools are not setuid, so AT_SECURE will typically be 0 and secure_getenv would behave
identically to getenv. The application-level scrubber provides the equivalent protection.

If UMRS ever ships a setuid helper, glibc automatically strips the dangerous variables
before the program starts, providing defense without application code.

For programs needing runtime detection of secure execution mode (requires unsafe):
  unsafe {
      let val = libc::secure_getenv(b"SOME_VAR\0".as_ptr() as *const libc::c_char);
      // val is NULL if running setuid/setcap; otherwise returns normal value
  }

Given #![forbid(unsafe_code)], UMRS cannot use this directly. The read-only snapshot
approach from std::env::vars() at single-threaded startup is the idiomatic safe Rust
equivalent.

---

### 5.3 Python (-I isolated flag)

Python reads PYTHONPATH, PYTHONSTARTUP, and PYTHONDONTWRITEBYTECODE at startup before any
application code runs. The -I (isolated) flag disables all user-affecting environment
variables. For high-assurance Python child processes from Rust:

  Command::new("python3")
      .arg("-I")  // isolated mode -- ignores PYTHON* env vars and user site-packages
      .env_clear()
      .env("PATH", "/usr/bin:/usr/sbin")
      .spawn()?;

---

### 5.4 Java

JAVA_TOOL_OPTIONS and _JAVA_OPTIONS allow injecting JVM arguments including -javaagent
and OnOutOfMemoryError handlers. Any Java process in an attacker-controlled environment
is vulnerable to arbitrary agent injection without modifying the command line.

When spawning Java from Rust, env_clear() plus no JAVA_* variables in the reconstructed
environment is the only safe approach.

---

## Category 6 -- Secrets in Environment Variables

### 6.1 The Twelve-Factor Critique

Source: https://12factor.net/config

The twelve-factor app methodology recommends storing configuration in environment variables.
This was widely adopted for containers. Security problems:

- docker inspect and kubectl describe expose env vars to anyone with API access
- /proc/pid/environ exposes env vars to same-UID processes
- Crash reporters and log aggregators routinely capture env var dumps
- Child processes inherit secrets (blast radius expansion)
- No audit trail, no version history, no rotation detection

Better alternatives:
- Secrets managers (HashiCorp Vault, AWS Secrets Manager, Azure Key Vault)
- Kubernetes Secrets mounted as files (not env vars) with automountServiceAccountToken: false
- SOPS for GitOps workflows
- Runtime injection via file descriptors

Source: https://blog.arcjet.com/storing-secrets-in-env-vars-considered-harmful/

---

### 6.2 NIST Guidance on Secrets in Environment

NIST SP 800-53 IA-5 prohibits storing authenticator information in areas without
confidentiality protections. Process environments visible via /proc/pid/environ do not
provide adequate confidentiality protections.

NIST SP 800-53 SC-28 requires protection of information at rest. Environment variables are
not protected at rest in any meaningful cryptographic sense.

NIST SP 800-57 Part 1 Rev 5 (Key Management) requires secret and private keys to have
protection against unauthorized disclosure. Storing keys in environment variables fails.

Implication for UMRS: The scrub report should flag any variable whose name pattern
suggests it contains a secret (*_KEY, *_SECRET, *_TOKEN, *_PASSWORD, *_PASS, *_API_KEY,
*_CREDENTIAL, AWS_*, GITHUB_TOKEN, etc.) -- not to read the value, but to record the
variable name as a CWE-526 finding.

---

## Appendix A -- Complete Dangerous Variables Quick Reference

### Tier 1 -- Never Expected in a UMRS Tool's Environment

These variables should trigger a WARN in the scrub report regardless of value.

  # glibc AT_SECURE strip list (22 variables from ld.so(8))
  GCONV_PATH, GETCONF_DIR, GLIBC_TUNABLES, HOSTALIASES, LOCALDOMAIN,
  LD_AUDIT, LD_DEBUG, LD_DEBUG_OUTPUT, LD_DYNAMIC_WEAK, LD_HWCAP_MASK,
  LD_LIBRARY_PATH, LD_ORIGIN_PATH, LD_PRELOAD, LD_PROFILE, LD_SHOW_AUXV,
  LOCPATH, MALLOC_TRACE, NIS_PATH, NLSPATH, RESOLV_HOST_CONF, RES_OPTIONS,
  TMPDIR, TZDIR,

  # Shell execution injection
  BASH_ENV, ENV, SHELLOPTS, GLOBIGNORE, IFS,

  # Interpreter module/library injection (see 1.11 for attack chains)
  PERL5LIB, PERLLIB, PERL5OPT,
  PYTHONPATH, PYTHONSTARTUP, PYTHONHASHSEED,
  RUBYLIB, RUBYOPT,
  NODE_OPTIONS, NODE_PATH,
  JAVA_TOOL_OPTIONS, _JAVA_OPTIONS, JVM_OPTS,
  PHPRC

### Tier 2 -- Validate Before Use

Variables that UMRS tools legitimately use but must validate before trusting.

  PATH          -- validate_path_list(): root-owned entries, no world-writable, no "."
  LANG          -- validate_lang(): POSIX locale syntax
  LC_ALL        -- validate_lang(): same
  LC_MESSAGES   -- validate_lang(): same
  TERM          -- validate_term(): known terminal identifiers only
  HOME          -- validate_safe_path(): exists, correct ownership, 0700+
  TZ            -- validate_tz(): valid timezone identifier
  LOGNAME       -- validate_username(): POSIX username syntax
  USER          -- validate_username(): same
  HOSTNAME      -- validate_hostname(): RFC 1123

### Tier 3 -- Pass Through (Low Risk)

  TERM_PROGRAM, COLORTERM, NO_COLOR, LINES, COLUMNS
  DBUS_SESSION_BUS_ADDRESS  -- validate_dbus_address() before use
  XDG_RUNTIME_DIR, XDG_SESSION_TYPE
  DISPLAY, WAYLAND_DISPLAY

---

## Appendix B -- Prioritized Reading List for Rust-Developer (Before Phase 1a)

Must-read before writing any validator:

1. ld.so(8) man page -- Secure-Execution Mode section
   https://man7.org/linux/man-pages/man8/ld.so.8.html
   Read: complete list of stripped variables and AT_SECURE conditions.
   This is the ground truth for Tier 1 classification.

2. CERT ENV03-C
   https://cmu-sei.github.io/secure-coding-standards/sei-cert-c-coding-standard/recommendations/environment-env/env03-c
   Read: the clearenv() + confstr() + setenv("IFS", ...) compliant solution pattern.
   This is the CERT-canonical version of what scrub_env() implements in Rust.

3. Rust 2024 Edition -- Newly Unsafe Functions
   https://doc.rust-lang.org/edition-guide/rust-2024/newly-unsafe-functions.html
   Read: why set_var/remove_var are unsafe, and what "only before threads start" means
   for the snapshot-at-startup design.

4. CVE-2023-4911 Qualys Analysis (full text)
   https://www.qualys.com/2023/10/03/cve-2023-4911/looney-tunables-local-privilege-escalation-glibc-ld-so.txt
   Read: the GLIBC_TUNABLES buffer overflow mechanism. Makes "why do we care about this
   variable?" concrete.

5. CVE-2024-48990 Ubuntu blog summary
   https://ubuntu.com/blog/needrestart-local-privilege-escalation
   Read: the PYTHONPATH/RUBYLIB exploit chain. Motivates the complete interpreter
   variable Tier 1 list.

Before implementing path validators:

6. MITRE ATT&CK T1574.006 -- Dynamic Linker Hijacking
   https://attack.mitre.org/techniques/T1574/006/
   Read: real-world threat actor procedures. Makes validate_safe_path() motivations concrete.

7. MITRE ATT&CK T1574.007 -- PATH Interception
   https://attack.mitre.org/techniques/T1574/007/
   Read: the PATH trojan horse attack. Motivates checking for "." and world-writable
   directories in PATH.

Before Phase 1c -- ScrubReport design:

8. CWE-526, CWE-454, CWE-427 (MITRE CWE entries)
   https://cwe.mitre.org/data/definitions/526.html
   https://cwe.mitre.org/data/definitions/454.html
   https://cwe.mitre.org/data/definitions/427.html
   These give the standardized weakness vocabulary the scrub report should use.

9. elttam "Hacking with Environment Variables"
   https://www.elttam.com/blog/env
   Read: cross-interpreter attack chains. Motivates why PERL5OPT, NODE_OPTIONS, and
   PHPRC are Tier 1 despite being less well-known than LD_PRELOAD.

---

## Appendix C -- RAG Ingestion Candidates

No ingestion recommended yet -- this is the inventory for Jamie to review.

Document               | Source URL                                          | Recommended action | Priority
-----------------------|-----------------------------------------------------|--------------------|---------
ld.so(8) man page      | https://man7.org/linux/man-pages/man8/ld.so.8.html | Fetch + ingest     | HIGH
secure_getenv(3)       | https://man7.org/linux/man-pages/man3/getenv.3.html | Fetch + ingest     | MEDIUM
CERT ENV03-C           | https://cmu-sei.github.io/secure-coding-standards/  | Fetch + ingest     | MEDIUM
elttam env blog        | https://www.elttam.com/blog/env                     | Fetch + ingest     | MEDIUM
CWE-526/454/427        | https://cwe.mitre.org/data/definitions/*.html       | Familiarize only   | LOW
ATT&CK T1574.006/007   | https://attack.mitre.org/techniques/T1574/006/      | Familiarize only   | LOW
no_new_privs kernel doc| https://docs.kernel.org/userspace-api/no_new_privs.html | ALREADY in kernel-docs -- verify chunk | LOW

NOT recommended for RAG ingestion:
- CVE detail pages (nvd.nist.gov) -- data changes; retrieve fresh when needed
- Qualys CVE blog posts -- narrative; better as bookmarks
- OWASP ASVS Chapter 14 -- already substantially covered by existing NIST content
- Twelve-factor config page -- brief, conceptual; familiarization sufficient

---

## Appendix D -- UMRS Implementation Notes

These notes are addressed to the rust-developer implementing Phases 1a--1c.

On the Tier 1 list in scrub.rs:
The variables in the "glibc AT_SECURE strip list" should be cited in validator source
comments as:
  // glibc ld.so(8) secure-execution stripped variables -- NIST SP 800-53 CM-7, SI-7

On validate_safe_path():
A path in PATH is safe only if all of the following hold:
- The directory exists
- It is not world-writable (mode bits do not include o+w)
- It is not "." or a relative path
- It is owned by root or the current user
- It does not contain symlinks to world-writable directories

World-writable directories in PATH are the mechanism behind T1574.007. Relative paths
in PATH are the mechanism behind the classic "." attack.

On GLIBC_TUNABLES classification:
This variable must be classified as Tier 1 regardless of value. Its presence is the
finding. The scrub report comment should reference CVE-2023-4911.

On PYTHONPATH / RUBYLIB / PERL5LIB / NODE_OPTIONS:
These are Tier 1. Their presence is a finding. Reference CVE-2024-48990 (PYTHONPATH)
and CVE-2024-48992 (RUBYLIB). The needrestart vulnerability shows that a privileged
process reading these variables from a low-privilege process's environment is exploitable
even without being the original source of those variables.

On IFS:
IFS is Tier 1. UMRS tools do not invoke shell interpreters, but the variable's presence
is anomalous. If a future UMRS component ever passes environment to a child process,
IFS must be explicitly overridden to " \t\n".

On BASH_ENV / ENV:
If any UMRS component ever invokes bash (even as "bash -c ..."), these variables would
cause arbitrary code execution before the command runs. They are Tier 1.

On the ScrubReport finding types and CWE mapping:

Finding class                          | CWE     | NIST control
---------------------------------------|---------|------------------
Dangerous loader variable present      | CWE-427 | CM-7, SI-7
Secret-pattern variable present        | CWE-526 | IA-5, SC-28
External initialization of trusted var | CWE-454 | SI-10, AC-3
Security decision from env var         | CWE-807 | SI-10
Invalid value in validated var         | CWE-20  | SI-10

On Command::env_clear() for child processes:
  Command::new("/usr/bin/helper")
      .env_clear()
      .env("PATH", "/usr/bin:/usr/sbin:/bin:/sbin")
      .env("LANG", "C")
      .env("TZ", "UTC")
      .spawn()?

This mirrors sudo env_reset and systemd's service unit default environment construction.

---

Report ends.
Retrieved: 2026-04-12. All URLs verified as accessible on retrieval date.
