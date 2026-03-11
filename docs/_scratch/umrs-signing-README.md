Signing project

Got it, we’re firmly back in “signing-keys project” land — no more interfaces.

Below is a self-contained project description you can paste straight into design docs, Confluence, or a README. I’ll keep it plain text, but structured, and explicitly call out how functions map to controls so an auditor (or security reviewer) can follow the story.

⸻

	1.	Project overview

⸻

Title: High-Assurance Signing Key Orchestrator (HA-Sign)

Purpose:
HA-Sign is a Bash-based orchestration utility that standardizes the lifecycle management of host-local signing keys used for:
	•	Audit log signing (e.g., files produced by auditd / auditd log rotation)
	•	System and application logging (e.g., rsyslog, journald export, /var/log trees)
	•	Archival package signing (e.g., tarballs and backups created by logrotate and other retention jobs)
	•	Optional auxiliary signatures (e.g., configuration snapshots, integrity manifests)

The goal is to provide a deterministic, auditable, and policy-aligned mechanism for creating, rotating, inventorying, and documenting signing keys across a hardened Linux environment (e.g., RHEL 10 in FIPS mode). The utility is designed to support NIST 800-53 control families (especially AU, SC, SI, and CM), FIPS 140-3 cryptographic requirements, and common STIG-style expectations around key management, logging, and traceability.

The core design principles are:
	•	Deterministic structure: keys live in well-defined locations with predictable names.
	•	Strong defaults: FIPS-approved algorithms and strong key lengths by default.
	•	No passphrases for automation: signing keys intended for unattended jobs are created without passphrases, with compensating controls (file permissions, separation, rotation).
	•	Full lifecycle coverage: init, rotate, list, report, and import of existing keys.
	•	Auditor-friendly reporting: explicit mapping between keys, their purpose, locations, algorithms, and lifetimes.

⸻

	2.	Scope and responsibilities

⸻

HA-Sign focuses on local signing keys that protect the integrity and provenance of log and archive artifacts. It does NOT attempt to be:
	•	A general PKI or CA.
	•	A replacement for system-level crypto libraries.
	•	A TLS certificate manager.

Instead, it standardizes host-level signing of:
	•	Audit logs (e.g., /var/log/audit/audit.log.*)
	•	System / application logs (e.g., /var/log/messages, /var/log/secure, /var/log/httpd/*.log, rotated logs)
	•	Archived data bundles (e.g., compressed rotated logs, backup tarballs)

The script provides:
	•	Key directory layout and access control.
	•	Key creation, rotation, and archival.
	•	Public key export and management.
	•	Inventory and formal reporting.

Integrations (e.g., telling logrotate, auditd, or rsyslog which key to use) are left to system configuration, but the script outputs consistent paths and names to plug into those tools.

⸻

	3.	Key classes and naming conventions

⸻

The script assumes a configurable base directory (default example shown):
	•	Base directory (configurable):
	•	Default: /etc/pki/ha-sign (but can be overridden via a config file or environment).
	•	Subdirectories by key purpose:
	•	/etc/pki/ha-sign/audit/       – keys for audit log file signatures.
	•	/etc/pki/ha-sign/logging/     – keys for non-audit application/system log signatures.
	•	/etc/pki/ha-sign/archive/     – keys for signing compressed/archived artifacts.
	•	/etc/pki/ha-sign/aux/         – optional keys for extra use-cases (config snapshots, manifests, etc.).
	•	/etc/pki/ha-sign/public/      – exported public keys, organized by purpose and key ID.

Each purpose directory follows a consistent naming scheme:
	•	Private keys:
	•	PURPOSE-hostname-YYYYMMDD.key
	•	Example: audit-guard01-20251208.key
	•	Public keys:
	•	PURPOSE-hostname-YYYYMMDD.pub
	•	Symlinks for “current”:
	•	PURPOSE-current.key → PURPOSE-hostname-YYYYMMDD.key
	•	PURPOSE-current.pub → PURPOSE-hostname-YYYYMMDD.pub

This allows:
	•	Simple human recognition of key type, host, and generation date.
	•	Easy rotation (update the current symlink).
	•	Straightforward archival of old keys (rename, move to an archive folder, but keep them for verification).

Permissions (defaults):
	•	Base directory:
	•	Owner: root
	•	Group: root (or a dedicated security group such as security-admin)
	•	Mode: 0750
	•	Private key files:
	•	Owner: root
	•	Group: root or security-admin
	•	Mode: 0400 (readable only by owner)
	•	Public key files:
	•	Owner: root
	•	Group: root or security-admin
	•	Mode: 0644 (readable by all; suitable for export)

No passphrases are applied to signing keys, by design, so that unattended services (auditd, logrotate, journald export jobs, etc.) can perform signing without interactive input. The security assumption is that private keys remain strictly protected by file permissions, SELinux, and system hardening.

⸻

	4.	Cryptographic parameters and compliance

⸻

The script is configurable, but defaults are chosen to align with:
	•	FIPS 140-3 validated primitives available via the system’s crypto stack.
	•	NIST SP 800-57 (Part 1) recommendations for key sizes and lifetimes.
	•	NIST SP 800-131A guidance on acceptable and deprecated algorithms.

Default settings (examples; actual choices tied to the platform’s FIPS-mode capabilities):
	•	Algorithm: Ed25519, ECDSA over P-256, or RSA with 3072-bit key length (whichever combination is best supported and validated in the environment; Ed25519 or P-256 preferred when appropriate).
	•	Key length (if RSA): minimum 3072 bits.
	•	Validity / lifetime: configurable, with a default that matches organization policy (e.g., 2–3 years for signing keys used for logs/archives, with shorter for high-sensitivity environments).

The script should:
	•	Allow administrator to select algorithm and key length via a config file or CLI options.
	•	Enforce a “no weaker than policy” rule: if an operator tries to select an algorithm/key size below the minimum policy, the script should refuse and log the reason.
	•	Store key metadata (algorithm, size, generated on, expires on, purpose) in a small companion metadata file or embedded in the report.

⸻

	5.	Script capabilities and subcommands

⸻

The script supports a discrete set of operations. Each is verbose and color-annotated (where terminals support color) so operators see exactly what is happening.

Core subcommands:
	1.	init
	•	Purpose:
	•	Initialize the HA-Sign key hierarchy.
	•	Actions:
	•	Create the base directory (default /etc/pki/ha-sign) and standard subdirectories.
	•	Enforce secure permissions and ownership.
	•	Optionally create initial signing keys for each supported purpose (audit, logging, archive, aux) using configured defaults.
	•	Emit a human-readable summary and a machine-readable JSON/YAML metadata file.
	2.	rotate
	•	Purpose:
	•	Generate a new key for a given purpose and archive the old one.
	•	Actions:
	•	Create a new private/public key pair using configured algorithm and key length.
	•	Move the previous “current” key pair into an archive subdirectory while leaving them available to verify historical data.
	•	Update the “current” symlinks to point to the new pair.
	•	Update or append key metadata, including the “retired on” date for old keys and “activated on” date for new keys.
	3.	list
	•	Purpose:
	•	Inventory all known keys and summarize their attributes.
	•	Actions:
	•	Walk the base directory and subdirectories, reading private and public keys.
	•	For each key, infer or parse:
	•	Purpose (audit / logging / archive / aux).
	•	Host identifier (from name).
	•	Algorithm and key length.
	•	Generation date (from name and/or metadata).
	•	Expiration date (from metadata or config).
	•	Current vs archived status.
	•	Output:
	•	Colorized console table for humans.
	•	Optionally, a structured JSON/YAML representation for integration with other tools.
	4.	report
	•	Purpose:
	•	Generate an auditor-ready report describing all signing keys and how they’re used.
	•	Actions:
	•	Use the same inventory as list, but produce:
	•	A formal header (system name, hostname, date, OS version, FIPS mode status).
	•	One section per key purpose, describing:
	•	Key naming convention.
	•	Directory paths.
	•	Number of active keys, number of archived keys.
	•	Cryptographic algorithms and sizes.
	•	Lifetimes (policy vs actual key data).
	•	Integration points (e.g., “audit log signatures are created by post-rotate hooks, using ‘audit-current.key’ to sign /var/log/audit/audit.log.*”).
	•	A compliance mapping section that ties key management and logging to the relevant security controls.
	•	Export formats:
	•	Plain text report.
	•	Optional machine-readable summary for other systems.
	5.	import
	•	Purpose:
	•	Discover and incorporate existing public keys the script did not originally create.
	•	Actions:
	•	Scan a configurable set of directories (e.g., /etc/pki, /etc/ssh, application-specific directories).
	•	For each public key found:
	•	Attempt to parse algorithm and key length.
	•	Use naming, path context, and optional config hints to guess purpose and usage.
	•	Include these keys in the report as “external” or “pre-existing” entries.
	•	Optionally copy them into /etc/pki/ha-sign/public/ for centralized tracking (without touching the original copy).

⸻

	6.	Use of existing keys and discovery

⸻

A key requirement is that HA-Sign must not “own the world” but must be able to inventory and report on keys it did not create, especially for:
	•	Legacy systems being migrated into a more controlled regime.
	•	Third-party tools that generate their own keys in fixed locations.

Discovery logic:
	•	Configurable search roots for public keys:
	•	Example: /etc/pki, /etc/ssh, /usr/local/etc/pki, application-specific paths.
	•	Whitelisting/blacklisting patterns to focus on relevant keys.
	•	For each found public key:
	•	Use tools like ssh-keygen -lf, openssl pkey -text -noout, or equivalent to read algorithm and size.
	•	Record path, parsed attributes, and any metadata.
	•	Present this clearly in the report, with a distinction between:
	•	HA-Sign managed keys (in base directory, following naming convention).
	•	External keys (elsewhere, discovered for documentation and review).

This gives auditors a consolidated view of the host’s signing keys, even if multiple tools are involved.

⸻

	7.	Security controls and requirements mapping

⸻

Below is a conceptual mapping between HA-Sign functions and typical control families. Exact control IDs will depend on your organization’s profile, but NIST SP 800-53 Rev. 5 is a useful anchor:
	1.	Key generation and strength (init, rotate)
	•	Relevant controls:
	•	SC-12 (Cryptographic Key Establishment and Management)
	•	SC-13 (Cryptographic Protection)
	•	SC-17 (Public Key Infrastructure Certificates)
	•	AU-10 (Non-repudiation)
	•	How HA-Sign helps:
	•	Enforces strong, FIPS-approved algorithms and key sizes.
	•	Centralizes signing key generation in a controlled script with deterministic output.
	•	Documents key parameters and lifetimes for review.
	2.	Key protection and access control (init, directory layout, permissions)
	•	Relevant controls:
	•	AC-3 (Access Enforcement)
	•	AC-6 (Least Privilege)
	•	SC-28 (Protection of Information at Rest)
	•	How HA-Sign helps:
	•	Places private keys in root-owned, 0400 files in a locked-down directory.
	•	Separates private and public material into different paths.
	•	Provides a simple, reviewable structure for file-based access checks and SELinux policies.
	3.	Key lifecycle, rotation, and archival (rotate, list)
	•	Relevant controls:
	•	SC-12 (Key management – specifically key lifetime and replacement)
	•	CM-6 (Configuration Settings)
	•	CM-9 (Configuration Management Plan)
	•	How HA-Sign helps:
	•	Enforces a repeatable rotation procedure with clear “current” vs archived keys.
	•	Records generation and retirement dates for each key.
	•	Makes it straightforward to prove that keys are not used beyond their intended lifetime, while still allowing verification of historical data.
	4.	Logging, audit, and integrity (audit signing, logging signing, report)
	•	Relevant controls:
	•	AU-2 (Event Logging)
	•	AU-3 (Content of Audit Records)
	•	AU-6 (Audit Review, Analysis, and Reporting)
	•	AU-8 (Time Stamps; when integrated with log infrastructure)
	•	AU-9 (Protection of Audit Information)
	•	AU-10 (Non-repudiation)
	•	SI-7 (Software, Firmware, and Information Integrity)
	•	How HA-Sign helps:
	•	Provides keys dedicated to signing audit logs and other critical log files.
	•	Supports configuration of post-rotate hooks to sign new log segments.
	•	Facilitates verification of log integrity over time using exported public keys.
	•	Produces reports that show auditors exactly which keys protect which artifacts.
	5.	Documentation, reporting, and assurance (report, import, list)
	•	Relevant controls:
	•	PL-2 (System and Communications Protection Policy and Procedures)
	•	CA-7 (Continuous Monitoring)
	•	RA-5 (Vulnerability Monitoring and Scanning – indirectly, via documented cryptographic posture)
	•	How HA-Sign helps:
	•	Generates a repeatable, timestamped report describing keys, their purpose, and their lifetimes.
	•	Enables continuous key posture review, especially when combined with scheduled runs.
	•	Clearly ties technical implementation to policy expectations (e.g., “audit logs must be integrity-protected”).
	6.	Integration with broader compliance frameworks
	•	FIPS 140-3:
	•	Ensures only validated algorithms and key sizes are used (to the extent supported by the platform).
	•	STIG / CIS Benchmark-style expectations:
	•	Centralized key directories.
	•	Strong file permissions.
	•	Documented key lifecycle and rotation processes.
	•	High-assurance / RTB-style environments:
	•	Provides deterministic behavior and a clear control narrative that can be reused in accreditation documentation.

⸻

	8.	Assurance story for auditors and reviewers

⸻

When an auditor asks, “How do you manage signing keys for your logs and archives?” the HA-Sign story is:
	•	We have a dedicated, documented utility that:
	•	Creates signing keys in one place, with FIPS-approved algorithms and minimum key sizes.
	•	Separates private and public keys, enforces strict file permissions, and prevents interactive passphrases for automated jobs.
	•	Rotates keys on a documented schedule, archives old keys for verification, and tracks lifetimes.
	•	Produces a formal report enumerating every key, its purpose, algorithm, length, locations, and status (active or archived).
	•	Encapsulates this in a repeatable, scriptable process that can be inspected and tested like any other component.
	•	This report is used as evidence for:
	•	Proper key management (SC-12, SC-13).
	•	Log integrity and non-repudiation (AU-9, AU-10, SI-7).
	•	Configuration management and continuous monitoring (CM, CA families).

⸻

	9.	Suggested introductory paragraph (for docs / README)

⸻

You can drop this straight into a README or design document:

“The High-Assurance Signing Key Orchestrator (HA-Sign) is a Bash-based utility that standardizes how this system creates, rotates, inventories, and documents signing keys used for log and archive integrity. It enforces FIPS-approved algorithms, strong key lengths, and strict file permissions, while deliberately avoiding passphrases for keys used by unattended services. HA-Sign centralizes key storage under /etc/pki, provides clear naming and directory conventions, and generates auditor-ready reports that show exactly which keys protect which artifacts, how long they’re valid for, and when they were rotated. By using this tool and its associated processes, we can confidently claim alignment with NIST 800-53 controls for key management (SC-12, SC-13), audit logging and integrity (AU-2, AU-9, AU-10, SI-7), and configuration management and monitoring (CM-6, CA-7), providing a high level of assurance in the integrity of our logs and archived data.”

If you’d like, next step we can draft a short “operator’s guide” section (usage examples, sample init/rotate/report runs) written in the same formal style, so you can paste that right after this description in your documentation.

Here’s a concrete example using an audit-signing private key to sign a freshly rotated audit log, plus how to verify it.

Assumptions (based on what we discussed earlier):
	•	Private key: /etc/pki/ha-signing/audit/private/audit_signing_rsa_3072.key
	•	Public key:  /etc/pki/ha-signing/audit/public/audit_signing_rsa_3072.pem
	•	Newly rotated audit log: /var/log/audit/audit.log.1
	•	Key has no passphrase (so it can be used non-interactively).
	•	FIPS-approved RSA-3072 with SHA-256.

	1.	Sign the newly rotated audit log with the audit signing key

openssl dgst -sha256 
-sign /etc/pki/ha-signing/audit/private/audit_signing_rsa_3072.key 
-out /var/log/audit/audit.log.1.sig 
/var/log/audit/audit.log.1

What this does:
	•	Computes a SHA-256 digest of /var/log/audit/audit.log.1
	•	Signs that digest with the private key
	•	Writes the detached signature to /var/log/audit/audit.log.1.sig

Because the key has no passphrase, this runs cleanly in a post-rotate hook or a cron job with no prompts.
	2.	Verify the signature later (for auditors or forensic checks)

openssl dgst -sha256 
-verify /etc/pki/ha-signing/audit/public/audit_signing_rsa_3072.pem 
-signature /var/log/audit/audit.log.1.sig 
/var/log/audit/audit.log.1

If the output is:

Verified OK

then:
	•	The file /var/log/audit/audit.log.1 has not been modified since it was signed.
	•	The signature was created with the private key corresponding to the published audit_signing_rsa_3072.pem.

	3.	How this would typically be called from automation

In logrotate’s config for auditd (for example /etc/logrotate.d/audit):

postrotate
/usr/sbin/service auditd reload >/dev/null 2>&1 || true
/usr/local/sbin/ha-signing.sh sign-audit /var/log/audit/audit.log.1
endscript

and inside ha-signing.sh the sign-audit subcommand would effectively run the openssl dgst command above, using the configured key path and algorithm.

If you’d like, next step I can sketch the exact sign-audit function inside the script so it’s drop-in for your logrotate postrotate.

Here’s a concrete example using an audit-signing private key to sign a freshly rotated audit log, plus how to verify it.

Assumptions (based on what we discussed earlier):
	•	Private key: /etc/pki/ha-signing/audit/private/audit_signing_rsa_3072.key
	•	Public key:  /etc/pki/ha-signing/audit/public/audit_signing_rsa_3072.pem
	•	Newly rotated audit log: /var/log/audit/audit.log.1
	•	Key has no passphrase (so it can be used non-interactively).
	•	FIPS-approved RSA-3072 with SHA-256.

	1.	Sign the newly rotated audit log with the audit signing key

openssl dgst -sha256 
-sign /etc/pki/ha-signing/audit/private/audit_signing_rsa_3072.key 
-out /var/log/audit/audit.log.1.sig 
/var/log/audit/audit.log.1

What this does:
	•	Computes a SHA-256 digest of /var/log/audit/audit.log.1
	•	Signs that digest with the private key
	•	Writes the detached signature to /var/log/audit/audit.log.1.sig

Because the key has no passphrase, this runs cleanly in a post-rotate hook or a cron job with no prompts.
	2.	Verify the signature later (for auditors or forensic checks)

openssl dgst -sha256 
-verify /etc/pki/ha-signing/audit/public/audit_signing_rsa_3072.pem 
-signature /var/log/audit/audit.log.1.sig 
/var/log/audit/audit.log.1

If the output is:

Verified OK

then:
	•	The file /var/log/audit/audit.log.1 has not been modified since it was signed.
	•	The signature was created with the private key corresponding to the published audit_signing_rsa_3072.pem.

	3.	How this would typically be called from automation

In logrotate’s config for auditd (for example /etc/logrotate.d/audit):

postrotate
/usr/sbin/service auditd reload >/dev/null 2>&1 || true
/usr/local/sbin/ha-signing.sh sign-audit /var/log/audit/audit.log.1
endscript

and inside ha-signing.sh the sign-audit subcommand would effectively run the openssl dgst command above, using the configured key path and algorithm.

If you’d like, next step I can sketch the exact sign-audit function inside the script so it’s drop-in for your logrotate postrotate.

Good catch — you’re right to call that out.
In the rewrite, configuration knobs like base directory, algorithm, key length, and expiration were assumed but not explicitly documented for operators. Auditors will notice that gap too.

Below is a clean addendum section you can insert into the Operator’s Guide. It is written at the same formality level and explicitly explains how an operator configures HA-Sign.

This does not contradict anything we’ve already done — it completes the picture.

⸻

Configuration and Policy Controls
	12.	Configuration model and operator-controlled options

HA-Sign is designed to be configurable without modifying source code. Configuration values establish cryptographic policy and filesystem layout and are intended to be reviewed and approved as part of system hardening.

Configuration can be provided using:
	•	Command-line options (for ad-hoc use)
	•	A configuration file (for standard, repeatable deployments)
	•	Environment variables (for automation, if explicitly allowed)

All configuration values are explicitly reflected in reports generated by HA-Sign for audit purposes.

⸻

12.1 Base directory configuration

The base directory defines where all signing keys, public keys, metadata, and archives are stored.

Default:
	•	/etc/pki/ha-signing

To override the base directory at execution time:

sudo ha-signing-manager.sh –base-dir /etc/pki/custom-signing init

Operational considerations:
	•	The directory must reside on a local filesystem with appropriate integrity protections.
	•	The directory must be writable only via sudo.
	•	SELinux policies must permit read access to private keys only for the signing processes.

Policy guidance:
	•	The base directory location should be fixed and documented.
	•	Changing the base directory after initial deployment should be treated as a configuration change and reviewed.

⸻

12.2 Cryptographic algorithm selection

HA-Sign allows the cryptographic signing algorithm to be selected at runtime or via configuration.

Supported examples (platform-dependent):
	•	RSA (minimum 3072 bits)
	•	ECDSA (P-256 / P-384 curves)
	•	EdDSA (where FIPS-validated support is available)

Example (override algorithm):

sudo ha-signing-manager.sh –alg rsa init

Policy guidance:
	•	The selected algorithm must be FIPS-approved in the operational environment.
	•	The algorithm must be documented in the system’s cryptographic policy.
	•	HA-Sign will refuse to use algorithms below the minimum security threshold.

⸻

12.3 Key length and cryptographic strength

HA-Sign allows explicit configuration of key length or curve.

Examples:
	•	RSA key length (bits): 3072, 4096
	•	Elliptic curve: prime256v1, secp384r1

Example:

sudo ha-signing-manager.sh –alg rsa –length 4096 init

Defaults:
	•	Strongest available algorithm and key length supported by the platform under FIPS mode.

Policy guidance:
	•	Key length must align with NIST SP 800-57 Part 1 recommendations.
	•	Stronger key lengths may be selected for higher-classification environments.
	•	Weak or deprecated sizes are explicitly rejected.

⸻

12.4 Key lifetime and expiration configuration

HA-Sign allows configuration of key lifetimes to align with organizational cryptoperiod policy.

Example:

sudo ha-signing-manager.sh –expiry-days 730 init

Operational behavior:
	•	Expiration metadata is recorded at key generation time.
	•	The tool can warn or report on keys nearing expiration.
	•	Rotation actions update metadata while retaining old public keys.

Policy guidance:
	•	Lifetimes must align with data retention and regulatory requirements.
	•	Shorter lifetimes may be required for audit signing keys.
	•	Expiration values must be documented and consistent across environments.

⸻

12.5 Configuration file usage (recommended for production)

For stable environments, HA-Sign supports a configuration file to avoid reliance on long command lines.

Example path:
	•	/etc/ha-signing/ha-sign.conf

Example contents (conceptual):

base_dir=/etc/pki/ha-signing
algorithm=rsa
key_length=4096
expiry_days=730
discover_public_keys=yes
discovery_paths=/etc/pki,/usr/local/etc/pki

Operational guidance:
	•	The configuration file must be root-owned and readable only via sudo.
	•	Changes to this file should follow change-management procedures.
	•	The active configuration values are echoed in reports for audit review.

⸻

12.6 Reporting configuration state for auditors

Every HA-Sign report includes:
	•	Effective base directory
	•	Selected algorithm and key length
	•	Configured expiration values
	•	Discovery settings for external keys

This ensures auditors can verify:
	•	The tool is operating as documented
	•	Cryptographic parameters match policy
	•	Configuration drift has not occurred

⸻

12.7 Change management considerations

Any change to:
	•	Base directory
	•	Algorithm
	•	Key length
	•	Expiration values

should be treated as a security-relevant configuration change and:
	•	Reviewed by a security authority
	•	Documented in system change records
	•	Reflected in the next HA-Sign report

⸻

Summary for operators

Operators configure HA-Sign through explicit, reviewable settings rather than implicit behavior. These settings directly control the cryptographic posture of the system and are automatically surfaced in reporting, providing transparency and audit traceability.

⸻
