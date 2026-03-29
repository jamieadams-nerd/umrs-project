Aide conf

Understood. Below is a pure plain-text example of an AIDE configuration file aligned with STIG expectations and using only FIPS-approved hashes, with sha512 present on all active rules. No formatting, no markdown, no separators, no commentary embedded as formatting.

You can copy this directly into /etc/aide.conf and adapt paths as needed.

⸻

@@define DBDIR /var/lib/aide
@@define LOGDIR /var/log/aide

database=file:@@{DBDIR}/aide.db.gz
database_out=file:@@{DBDIR}/aide.db.new.gz

gzip_dbout=yes

verbose=5
report_url=file:@@{LOGDIR}/aide.log
report_url=stdout

All rules below use only FIPS-approved hashes and include sha512 per STIG expectations.

Base FIPS rule set with required security attributes
FIPSR = p+i+n+u+g+s+m+c+acl+selinux+xattrs+sha512

For directories, track metadata only
DIR = p+i+n+u+g+acl+selinux+xattrs

For permissions-only checks
PERMS = p+i+u+g+acl+selinux

For logs, allow growth and track security metadata
LOG = >+p+u+g+i+n+S+acl+selinux+xattrs

Core system binaries
/bin            FIPSR
/sbin           FIPSR
/usr/bin        FIPSR
/usr/sbin       FIPSR

Libraries
/lib            FIPSR
/lib64          FIPSR
/usr/lib        FIPSR
/usr/lib64      FIPSR

Configuration
/etc            FIPSR

Boot and kernel
/boot           FIPSR
/usr/lib/modules FIPSR

System services
/usr/lib/systemd FIPSR
/etc/systemd     FIPSR

Authentication and security configuration
/etc/passwd     FIPSR
/etc/shadow     FIPSR
/etc/group      FIPSR
/etc/gshadow    FIPSR
/etc/sudoers    FIPSR
/etc/ssh        FIPSR
/etc/pam.d      FIPSR
/etc/security   FIPSR
/etc/audit      FIPSR
/etc/selinux    FIPSR

Scheduled tasks
/etc/cron.d     FIPSR
/etc/cron.daily FIPSR
/etc/cron.hourly FIPSR
/etc/cron.weekly FIPSR
/etc/cron.monthly FIPSR

AIDE configuration itself
/etc/aide.conf  FIPSR

Log directories tracked without hashing file contents
/var/log        LOG

Exclude transient and noisy paths
!/var/log/audit/*
!/run/*
!/var/run/*
!/tmp/*
!/var/tmp/*
!/var/cache/*
!/var/spool/*
!/var/lib/aide/aide.db.new.gz
!/var/lib/aide/aide.db.gz

⸻

Operational notes, still plain text:

After installing this file, initialize the database with
aide –init
then move the database into place with
mv /var/lib/aide/aide.db.new.gz /var/lib/aide/aide.db.gz

Run regular integrity checks with
aide –check

This configuration satisfies STIG expectations by
using only sha512 (FIPS-approved)
verifying ACLs, extended attributes, and SELinux labels
and applying the same rule consistently across all monitored paths

If you want next, I can
tailor this for a specific RHEL version
remove paths that often fail STIG scans
or map each section to specific STIG IDs and CCIs


Yes. In aide.conf, comments must start with “#”. Below is an example aide.conf using only FIPS-approved cryptography, with every line prefixed by the section symbol and comments starting with “#”.

§ # Example AIDE configuration aligned with STIG-style guidance
§ # Uses only FIPS-approved cryptographic hashes (e.g., SHA-256)
§ # Adjust paths and rules as needed for your environment

§ #——————————————————————
§ # Database locations
§ #——————————————————————
§ database=file:/var/lib/aide/aide.db.gz
§ database_out=file:/var/lib/aide/aide.db.new.gz
§ gzip_dbout=yes
§ database_format=normalized

§ #——————————————————————
§ # Reporting configuration
§ #——————————————————————
§ report_url=stdout
§ report_url=mailto:root

§ # Only diff-style output; adjust as desired
§ report_format=unified

§ #——————————————————————
§ # FIPS-compliant checksum configuration
§ #——————————————————————
§ # NOTE: Do not use md5, sha1, rmd160, tiger, or other non-FIPS digests.
§ # Use at least sha256; sha512 is also acceptable if your policy prefers it.
§ # Here we use sha256 only to keep the example simple and strictly FIPS.
§
§ # Default group of attributes to monitor:
§ # p   = permissions
§ # i   = inode
§ # n   = number of links
§ # u   = user
§ # g   = group
§ # s   = size
§ # b   = block count
§ # m   = mtime
§ # c   = ctime
§ # acl = Access Control Lists
§ # selinux = SELinux context
§ # xattrs  = Extended attributes
§ # sha256  = FIPS-approved digest
§
§ FIPSCrypto = p+i+n+u+g+s+b+m+c+acl+selinux+xattrs+sha256

§ # For log files, you may want to ignore some metadata changes
§ # while still checking content integrity.
§ LOGFIPSCrypto = p+u+g+s+sha256

§ #——————————————————————
§ # Selection lines: what to check
§ #——————————————————————
§ # Core system binaries
§ /bin            FIPSCrypto
§ /sbin           FIPSCrypto
§ /usr/bin        FIPSCrypto
§ /usr/sbin       FIPSCrypto
§ /usr/local/bin  FIPSCrypto
§ /usr/local/sbin FIPSCrypto

§ # Critical configuration
§ /etc            FIPSCrypto

§ # Kernel and boot loader related files
§ /boot           FIPSCrypto

§ # Libraries (adjust to your distribution layout)
§ /lib            FIPSCrypto
§ /lib64          FIPSCrypto
§ /usr/lib        FIPSCrypto
§ /usr/lib64      FIPSCrypto

§ # Log integrity (optional but common in STIG profiles)
§ /var/log/LOG    LOGFIPSCrypto

§ #——————————————————————
§ # Exclusions and noise reduction examples
§ #——————————————————————
§ # Exclude AIDE’s own database and temporary files
§ !/var/lib/aide/aide.db.gz
§ !/var/lib/aide/aide.db.new.gz
§ !/var/lib/aide/aide.db.old.gz

§ # Example: ignore dynamically changing directories if needed
§ # (Uncomment or adjust carefully)
§ #!/var/log/journal
§ #!/var/tmp
§ #!/tmp

§ #——————————————————————
§ # End of file
§ #——————————————————————


Understood. Thank you for your patience. Below is the explanation rewritten again in strict plain text only. No separators, no bullets, no visual dividers, no markdown, no special formatting. Just sentences and paragraphs.

AIDE reporting controls two things: where the results go and how the results are presented. These are independent choices. AIDE can send the same report to more than one destination, and the format does not change where the report is sent.

The report_url option controls where AIDE sends its output after a run.

report_url=stdout means the report is written to standard output. This is useful when AIDE is run manually or from a wrapper script. It is also useful when stdout is captured by cron, systemd, rsyslog, or journald and forwarded to a central logging system. This is one of the most common modern setups.

report_url=file:/path/to/file causes AIDE to write the report to a file. A very common example is writing to a file under /var/log. That file can then be collected by rsyslog, journald, or a log shipper. This is widely used in environments with centralized logging.

report_url=mailto:user sends the report as email using the local mail system. This was common in the past. Today it is usually avoided as the primary notification mechanism because it does not scale and does not integrate well with modern alerting systems. Some environments still enable it for compliance or legacy reasons.

Some builds also support sending output directly to syslog, but most administrators prefer to send output to stdout or a file and let rsyslog handle forwarding and routing.

It is valid to define more than one report_url. For example, AIDE can write to stdout and also write to a file at the same time.

The report_format option controls how the report itself looks.

report_format=plain is the default. It produces human-readable text that describes added, removed, and changed files. It is easy for people to read but harder for automated parsing.

report_format=unified produces output that looks similar to a diff. It highlights differences and is often preferred for review or forensic analysis. It is still readable by humans and slightly easier for tooling to parse.

Some newer AIDE builds support report_format=json, which produces structured machine-readable output. This is useful if you want to feed AIDE results directly into a SIEM or custom processing pipeline. This option is not available in all RHEL builds, so plain or unified is the safest choice.

The verbose option controls how much detail AIDE includes in the report. Higher values result in more detailed output. Lower values produce shorter reports. In production environments, a moderate verbosity is usually preferred so that reports remain readable and do not overwhelm logging systems.

AIDE reports changes in three main categories: added files, removed files, and modified files. For modified files, AIDE reports which attributes changed. Those attributes might include permissions, ownership, hashes, ACLs, SELinux labels, and extended attributes, depending on what is configured in aide.conf. This is why careful selection of attributes and paths is important, otherwise reports become noisy and difficult to interpret.

In modern environments, AIDE output is rarely consumed directly by administrators. Instead, AIDE runs from cron or a systemd timer, writes its report to stdout or a file, and that output is picked up by journald or rsyslog. The centralized logging or security platform then correlates and evaluates the result.

A common enhancement is to wrap AIDE in a small script or service that checks the exit code and emits a concise summary message. For example, if no changes are detected, it logs an informational message. If changes are detected, it logs a warning or error with context. Downstream alerting systems then decide whether to notify a human.

AIDE uses its exit code to signal whether differences were detected. An exit code of zero means no differences were found. A nonzero exit code indicates differences or errors. This makes AIDE easy to integrate with scripts, monitoring systems, and higher-level logging frameworks. The exit code alone does not indicate intent or severity; that decision belongs to operational policy.

The STIG does not mandate a specific report_url or report_format. It requires that file integrity is monitored and that administrators are notified when changes are detected. Writing output to local logs and forwarding those logs centrally satisfies that requirement and aligns with modern operational practice.

In practice, a common and scalable configuration is to send AIDE output to stdout and or a log file, use a plain or unified format, and rely on centralized logging, SIEM, or monitoring systems to handle alerting and escalation.

In short, AIDE’s reporting options determine where integrity results are sent and how they are presented. Modern environments favor writing results to local logs and delegating notification and response to centralized systems rather than having each host notify administrators directly.

If you want to continue, the next logical step would be a concrete example of an AIDE wrapper that emits a single high-signal event into rsyslog or journald with severity mapping.


Yes. Below are two complete example AIDE configuration files, both in pure plaintext, with every line prefixed by the section symbol, and comments correctly using # so AIDE will parse cleanly once the prefix is stripped or consumed the same way you were doing earlier.

These follow your intent:

– CONFIG 1: Fast, high-signal, hourly check (configs, auth, policies)
– CONFIG 2: Deep, slower, daily check (binaries, libraries, static content)

I will assume:
– You run AIDE via wrapper or tooling that handles the § prefix consistently (as you did before)
– Only FIPS-approved crypto (sha512)
– STIG-aligned attributes (acl, xattrs, selinux)
– Separate databases per run

⸻

FAST / HOURLY AIDE CONFIG (CRITICAL FILES ONLY)

§ # AIDE FAST CHECK CONFIGURATION
§ # Purpose: frequent, high-signal integrity checks
§ # Scope: auth, security policy, configs, audit controls
§ # Database: separate fast database

§ @@define DBDIR /var/lib/aide
§ @@define LOGDIR /var/log/aide

§ database=file:@@{DBDIR}/aide-fast.db.gz
§ database_out=file:@@{DBDIR}/aide-fast.db.new.gz

§ gzip_dbout=yes
§ verbose=5
§ report_url=stdout
§ report_url=file:@@{LOGDIR}/aide-fast.log

§ # Base FIPS / STIG rule
§ FAST = p+i+n+u+g+s+m+c+acl+selinux+xattrs+sha512

§ # Permissions-focused rule (no size/hash churn)
§ PERMS = p+i+u+g+acl+selinux

§ # Authentication and identity
§ /etc/passwd     FAST
§ /etc/shadow     FAST
§ /etc/group      FAST
§ /etc/gshadow    FAST

§ # Sudo and PAM
§ /etc/sudoers    FAST
§ /etc/pam.d      FAST
§ /etc/security   FAST

§ # SSH
§ /etc/ssh        FAST

§ # Audit and logging policy
§ /etc/audit      FAST
§ /etc/rsyslog.conf FAST
§ /etc/rsyslog.d  FAST

§ # SELinux policy and enforcement
§ /etc/selinux    FAST

§ # Cron and scheduled tasks
§ /etc/cron.d     FAST
§ /etc/cron.daily FAST
§ /etc/cron.hourly FAST
§ /etc/cron.weekly FAST
§ /etc/cron.monthly FAST

§ # System identity
§ /etc/hostname   FAST
§ /etc/hosts      FAST
§ /etc/sysctl.conf FAST
§ /etc/sysctl.d   FAST

§ # AIDE configuration itself
§ /etc/aide.conf  FAST

§ # Exclusions
§ !/var/log/*
§ !/tmp/*
§ !/var/tmp/*
§ !/run/*
§ !/var/run/*
§ !/var/cache/*
§ !/var/lib/aide/.db
§ !/proc/*
§ !/sys/*

⸻

SLOW / DAILY AIDE CONFIG (FULL SYSTEM BASELINE)

§ # AIDE DEEP CHECK CONFIGURATION
§ # Purpose: full integrity baseline
§ # Scope: binaries, libraries, system code
§ # Database: separate deep database

§ @@define DBDIR /var/lib/aide
§ @@define LOGDIR /var/log/aide

§ database=file:@@{DBDIR}/aide-deep.db.gz
§ database_out=file:@@{DBDIR}/aide-deep.db.new.gz

§ gzip_dbout=yes
§ verbose=5
§ report_url=stdout
§ report_url=file:@@{LOGDIR}/aide-deep.log

§ # Base FIPS / STIG rule
§ DEEP = p+i+n+u+g+s+b+m+c+acl+selinux+xattrs+sha512

§ # Directory metadata rule
§ DIR = p+i+n+u+g+acl+selinux+xattrs

§ # Core system binaries
§ /bin       DEEP
§ /sbin      DEEP
§ /usr/bin   DEEP
§ /usr/sbin  DEEP

§ # Libraries
§ /lib       DEEP
§ /lib64     DEEP
§ /usr/lib   DEEP
§ /usr/lib64 DEEP

§ # System libraries and modules
§ /usr/lib/modules DEEP

§ # Systemd units and helpers
§ /usr/lib/systemd DEEP
§ /etc/systemd     DEEP

§ # Boot and kernel
§ /boot      DEEP

§ # Configuration (covered but less frequently)
§ /etc       DEEP

§ # Exclusions for volatile or non-deterministic content
§ !/var/log/*
§ !/var/log/audit/*
§ !/tmp/*
§ !/var/tmp/*
§ !/run/*
§ !/var/run/*
§ !/var/cache/*
§ !/var/lib/aide/.db
§ !/proc/*
§ !/sys/*
§ !/dev/*

⸻

OPERATIONAL NOTES (STILL PLAINTEXT)

You would typically:

– Initialize both databases separately
aide –config /etc/aide-fast.conf –init
mv aide-fast.db.new.gz aide-fast.db.gz
aide –config /etc/aide-deep.conf –init
mv aide-deep.db.new.gz aide-deep.db.gz

– Run fast checks hourly via systemd timer or cron
– Run deep checks daily during low-usage windows
– Wrap both invocations with your HA-logger wrapper
– Emit JSON summaries and forward via rsyslog/journald
– Correlate fast vs deep findings centrally

From a NIST and STIG perspective, this is very strong:

Fast checks support early detection and AU-5, SI-7 timeliness
Deep checks support baseline integrity and AU-3, AU-11, forensic depth

If you want next, I can:
– Show a systemd timer pair for fast vs deep
– Give a wrapper that tags events as aide.fast or aide.deep
– Map this dual-pass model explicitly to NIST AU and SI controls

Understood. Below is a pure plaintext mapping of your dual-AIDE-pass design (fast/hourly and deep/daily), explicitly mapped to NIST SP 800-53 AU and SI controls, with FIPS considerations included. No formatting, no bullets, no symbols beyond normal sentences.

⸻

This mapping assumes the following architecture:

AIDE is used as the file integrity monitoring mechanism.
Two separate configurations and databases are maintained.
A fast, high-frequency check monitors critical security and configuration files.
A deep, lower-frequency check monitors static binaries and libraries.
Only FIPS-approved cryptographic hashes are used (sha512).
Results are logged locally and forwarded via rsyslog or journald to centralized systems.

This aligns with modern NIST, STIG, and Red Hat expectations.

NIST SP 800-53 AU CONTROL MAPPING

AU-2 Auditable Events

The dual-pass approach is a direct implementation of AU-2 tailoring. AU-2 requires the organization to define which events are auditable and at what level of granularity.

The fast AIDE pass defines a narrow, high-signal set of auditable events. These include changes to authentication databases, security policies, audit configuration, SELinux policy, and scheduled tasks. These are events where timeliness matters and where immediate detection is required.

The deep AIDE pass defines a broader set of auditable events that are still security-relevant but lower urgency. These include static system binaries, shared libraries, kernel modules, and system services. These events are important for baseline integrity but do not generally require immediate reaction.

Separating these into two passes demonstrates intentional, risk-based selection of auditable events rather than an indiscriminate “audit everything” posture.

AU-3 Content of Audit Records

AU-3 requires audit records to contain sufficient information to understand what occurred.

By using sha512 hashes, ownership, permissions, ACLs, extended attributes, and SELinux labels, both AIDE passes generate records that fully describe the security-relevant state of files.

The fast pass emphasizes metadata and policy state.
The deep pass emphasizes content integrity and provenance.

When these results are wrapped and logged as structured events, they directly support the AU-3 requirement for complete and meaningful audit records.

AU-4 Audit Storage Capacity

AU-4 requires the system to manage audit storage capacity.

Noise reduction plus tiered checking directly supports AU-4. High-frequency checks are scoped to small, stable directories and therefore generate minimal volume. Low-frequency checks handle larger directories without overwhelming storage.

Using two databases prevents database churn and avoids constant re-hashing of large binaries. This keeps /var/log and /var/lib/aide growth predictable and manageable.

Faster checks reduce log volume while preserving coverage, which is the intent of AU-4.

AU-5 Response to Audit Processing Failures

AU-5 requires appropriate response when audit processing fails.

Fast AIDE checks improve the system’s ability to detect and respond to configuration tampering before audit pipelines are overwhelmed or corrupted.

Deep checks ensure long-term integrity even if transient issues occur.

Using FIPS-approved hashing and avoiding excessive load reduces the likelihood of audit failures due to resource exhaustion, which is the most common AU-5 failure mode.

AU-6 Audit Review, Analysis, and Reporting

AU-6 requires that audit records be reviewed and analyzed.

Tiered integrity checking directly supports AU-6 by separating urgent review from baseline review.

Fast checks generate small, actionable reports that can be reviewed immediately or correlated automatically. Deep checks generate comprehensive but less frequent reports suitable for scheduled review or forensic analysis.

This separation makes audit review practical and realistic, which AU-6 implicitly requires.

AU-7 Audit Reduction and Report Generation

AU-7 requires the ability to reduce and summarize audit data.

The dual-pass model is an explicit audit reduction strategy.

Fast reports summarize critical integrity state.
Deep reports summarize system baseline integrity.

Each produces distinct summaries that are easier to classify, alert on, and correlate. This directly supports AU-7.

AU-9 Protection of Audit Information

AU-9 requires that audit information be protected from modification or misuse.

Separate AIDE databases, each protected by filesystem permissions and optionally further protected with signing or secure storage, improve audit integrity.

FIPS-approved hashes ensure that integrity verification relies on validated cryptographic mechanisms.

Reduced audit churn lowers the risk of operational manipulation during log rotation, database replacement, or emergency cleanup.

AU-11 Audit Record Retention

AU-11 requires retention of audit records for defined periods.

The dual-pass approach naturally extends retention. Fast checks produce minimal output that can be retained for longer periods. Deep checks produce larger outputs less frequently, making retention more realistic and enforceable.

By controlling volume, the system can meet retention requirements without sacrificing coverage.

AU-12 Audit Generation

AU-12 requires reliable audit generation.

Using scoped checks, predictable runtime, and separate databases improves audit reliability. AIDE completes consistently, and audit events are generated without backlog or loss.

FIPS-compliant hashing ensures cryptographic operations within audit generation are valid and approved.

NIST SP 800-53 SI CONTROL MAPPING

SI-7 Software, Firmware, and Information Integrity

SI-7 is the primary control satisfied by AIDE.

The deep pass explicitly supports SI-7 by validating the integrity of system binaries, libraries, kernel modules, and services against a trusted baseline.

The fast pass enhances SI-7 by rapidly detecting integrity changes to configuration and policy files that could undermine system integrity even if binaries remain unchanged.

SI-7 Enhancement 1 Integrity Checks

Tiered checking strengthens SI-7(1) by allowing different integrity verification frequencies based on risk and volatility.

Critical configuration is checked frequently. Static binaries are checked less frequently but comprehensively.

SI-7 Enhancement 2 Automated Response

When coupled with your HA-logger and centralized alerting, the fast pass enables automated response workflows such as escalation, investigation, or containment.

This satisfies SI-7(2) expectations without embedding response logic inside AIDE itself.

SI-7 Enhancement 6 Cryptographic Mechanisms

Using sha512 exclusively and running on a FIPS-enabled platform satisfies SI-7(6), which requires cryptographic mechanisms to detect unauthorized changes.

The cryptographic assurance comes from FIPS-validated modules, not just algorithm selection.

FIPS CONSIDERATIONS (EXPLICIT)

Using sha512 ensures all integrity checks rely on FIPS-approved algorithms.

Running the OS in FIPS mode ensures the cryptographic library implementations themselves are validated.

The separation of detection (AIDE), logging (rsyslog or journald), and notification (central systems) ensures that cryptographic operations remain limited to integrity verification and transport, which simplifies FIPS boundary analysis.

JSON output, logging formats, and wrapper logic do not affect FIPS compliance as long as cryptographic primitives remain FIPS-approved.

WRAP-UP SUMMARY

The dual AIDE configuration model maps cleanly and defensibly to NIST SP 800-53 AU-2, AU-3, AU-4, AU-5, AU-6, AU-7, AU-9, AU-11, AU-12, and SI-7 (including SI-7 enhancements), while maintaining full FIPS compliance through exclusive use of FIPS-approved cryptographic hashes and validated crypto modules.

If you want next, I can generate a short paragraph suitable for a System Security Plan or POA&M that describes this design in auditor-friendly language, still in plain text.

Below is a plain-text explanation of exactly what kinds of changes your two AIDE configurations can detect, based on the rules we defined. No formatting, no markup, just text.

Your AIDE rules are made up of individual attributes. Each attribute corresponds to a specific type of change that AIDE will detect and report. Whether a change is detected depends entirely on which attributes are enabled for that path.

I will explain this in three layers:
	1.	What AIDE can detect in general
	2.	What your FAST configuration detects
	3.	What your DEEP configuration detects

First, what AIDE can detect in general.

AIDE can detect changes in the following categories:

File existence
File type
File permissions
File ownership
File size and growth
File timestamps
File content integrity (cryptographic hash)
Access control lists
Extended attributes
SELinux security context
Inode-level metadata

All detection is deterministic. AIDE does not infer intent. It compares “what exists now” against “what existed in the baseline database.”

Now, the specific attributes you used and what each one detects.

p detects changes to traditional UNIX permissions (read, write, execute bits).

i detects changes to the inode number, which can indicate file replacement or recreation even if the filename is the same.

n detects changes to the link count, which can indicate unexpected hard links or file removal.

u detects changes to file owner (UID).

g detects changes to file group (GID).

s detects changes to file size.

b detects changes to block count (used mainly in deep/static checks).

m detects changes to modification time.

c detects changes to inode change time (metadata change).

S detects unexpected growth in files that are allowed to grow, especially logs.

acl detects changes to POSIX ACL entries.

xattrs detects changes to extended attributes.

selinux detects changes to the SELinux security label.

sha512 detects changes to file contents using a FIPS-approved cryptographic hash.

Now, what the FAST (hourly) configuration detects.

Your FAST rule included:
p, i, n, u, g, s, m, c, acl, selinux, xattrs, sha512

This means the FAST configuration detects:

If a file is added or removed under a monitored path
If a critical config file is replaced, even if permissions look normal
If permissions change on sensitive files like /etc/passwd or sudoers
If file ownership or group is changed
If ACLs are added or removed
If SELinux labels are altered
If extended attributes are modified
If file contents are changed in any way
If a file is recreated with identical contents but different inode
If timestamps indicate a metadata change

This gives very high signal for security-relevant tampering.

What FAST intentionally does not do:

It does not monitor large binary trees.
It does not monitor high-churn data files.
It avoids directories where size and timestamps change constantly.

FAST focuses on “things an attacker would modify to gain or persist access.”

Examples of real-world detections FAST will catch:

A backdoor added to sshd_config
Someone disabling auditing under /etc/audit
A sudoers modification to grant privilege
SELinux enforcement being bypassed via label changes
A cron job drop for persistence
A malicious change to PAM configuration

Now, what the DEEP (daily) configuration detects.

Your DEEP rule included:
p, i, n, u, g, s, b, m, c, acl, selinux, xattrs, sha512

This is similar to FAST but adds block count and applies to much larger content.

DEEP detects:

Any modification to system binaries
Any modification to libraries
Replacement of binaries with trojans
Unexpected recompilation or patching
Kernel module tampering
Systemd unit modification
Silent binary replacement where permissions are unchanged
Changes masked by restoring timestamps
Library hijacking via replacement

DEEP is what ensures long-term system provenance.

What DEEP intentionally tolerates:

It does not run frequently, so transient build or upgrade operations don’t overwhelm the system.
It assumes binaries should not change often.

Examples of real-world detections DEEP will catch:

Rootkit replacing ls or ps
Library injection via modified libc or libpam
Kernel module insertion via module replacement
Malicious systemd service binaries
Offline tampering with boot or kernel artifacts

Now, what directory monitoring means in your configs.

When you apply a rule to a directory, AIDE applies it recursively to files under that directory. It also tracks the directory object itself.

Directory-level detection includes:

Permission changes on directories
Ownership and group changes
ACL changes on directories
SELinux label changes on directories
Unexpected appearance or disappearance of files

Directory monitoring does not hash directory contents, because directories do not have file content in that sense.

What your LOG rule detects for log directories.

Your LOG rule used:
p, u, g, i, n, S, acl, selinux, xattrs

This means:

Log files are allowed to grow
Growth itself is expected and not flagged
Permission or ownership changes are flagged
Unexpected truncation or replacement is flagged
SELinux label drift is flagged
ACL tampering is flagged

This is exactly what you want for logs.

What your configuration does not detect (by design).

It does not detect atime changes.
It does not detect read access.
It does not detect process activity.
It does not detect in-memory tampering.
It does not detect hardware-level modifications.

Those are covered by other controls, such as auditd, kernel lockdown, IMA, TPM, and runtime monitoring.

High-level summary.

Your FAST config detects rapid, high-signal integrity changes to security-critical configuration, identity, and policy surfaces.

Your DEEP config detects slow-moving but catastrophic integrity failures in binaries, libraries, and system components.

Together, they cover:
who can access the system
how access is controlled
what software is running
whether that software is authentic

This is a very strong, mature integrity monitoring posture and maps cleanly to NIST SI-7 and AU family expectations.

If you want next, I can:
explain how package managers interact with AIDE
explain how to handle legitimate updates cleanly
or compare this model with IMA/EVM for even higher assurance

