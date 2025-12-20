# Cryptographic Key Management

In the context of information security and compliance frameworks like NIST Special Publication 800-53, **cryptographic key management** refers to the comprehensive set of practices, processes, and systems that oversee the full lifecycle of cryptographic keys. This lifecycle spans all stages from key generation, distribution, storage, and rotation, to archival and eventual destruction (cryptoperiod management). 

Effective key management ensures that cryptographic mechanisms, which protect the confidentiality, integrity, and authenticity of sensitive data, remain robust and trustworthy throughout their operational lives. It is the foundational security control that underpins all other uses of encryption within an organization.

The importance of robust key management cannot be overstated, as the security of encrypted data is entirely dependent on the security of the keys used to protect it. A weak or compromised key management framework can render even the strongest encryption algorithms useless. Poorly managed keys expose an organization to significant risks, including data breaches, loss of data integrity, and compliance failures with regulatory standards (such as FISMA, HIPAA, or GDPR). 

By implementing strong controls outlined in guidance like NIST 800-53 controls (e.g., CM-5, SC-12, SC-13), organizations establish a trusted system for safeguarding these critical assets, thereby maintaining secure operations and building verifiable trust in their information systems.

Many people get tripped up in audits: not the crypto itself, but how well the key lifecycle is documented and justified. In this document, it will be addressed in three layers:

* What standards actually require (authoritative controls)
* What auditors typically expect to see
* A practical, accreditation-safe documentation checklist you can actually use


## Authoritative security controls that govern keys & lifetimes

Across NIST, STIG, RTB, and MLS environments,key documentation requirements consistently come from the same control families.  Almost everything maps to NIST 800-53 Rev. 5.

### Key management (primary)

KM-1 — Cryptographic Key Management Policy
You must document:
	•	What kinds of keys exist
	•	What each key is used for
	•	Who is responsible for lifecycle decisions

KM-2 — Key Generation
You must document:
	•	How keys are generated
	•	Algorithms used
	•	Key strength
	•	Whether generation is automated or manual

KM-3 — Key Protection
You must document:
	•	Where keys are stored
	•	How access is restricted
	•	Why passphrases are or are not used
	•	Protection mechanisms (permissions, SELinux, HSM, TPM)

KM-4 — Key Lifetime and Rotation
You must document:
	•	Key validity period
	•	Rotation policy (scheduled, event-driven, or “until decommission”)
	•	Revocation / replacement procedure
---

### Audit and accountability

AU-2 — Audit Events
You must generate audit records for:
	•	Key creation
	•	Key rotation
	•	Key revocation
	•	Key deletion

AU-3 — Audit Record Contents
Audit records must include:
	•	What key (identifier)
	•	Action taken (created, rotated, retired)
	•	When it happened
	•	Who or what initiated it

AU-6 — Audit Review
You must be able to demonstrate:
	•	How key-related events are reviewed
	•	That failures or anomalies are detectable

---

### Configuration management

CM-2 — Baseline Configuration
You must document:
	•	Expected presence of keys
	•	Expected file locations
	•	Expected permissions and labels
	•	What constitutes “configuration drift”

CM-6 — Configuration Settings
You must justify:
	•	Why key settings are what they are
	•	Why defaults were changed (or not)

---

### System integrity & cryptography

SI-7 — Integrity Checks
You must show:
	•	What integrity guarantees the keys provide
	•	What happens if validation fails

SC-12 / SC-13 — Cryptographic Key Establishment & Protection
You must:
	•	Demonstrate FIPS-approved algorithms
	•	Explain system trust boundaries

---

## What auditors and reviewers actually expect to see

This is critical: auditors do not expect perfection, but they do expect explicit intent.

They want answers to these exact questions:

⸻

A. Key inventory

For each key:
	•	Key name / identifier
	•	Key purpose
	•	Algorithm and size
	•	Storage location
	•	Which system component uses it

Example (simple):

“Log archive signing key used to sign rotated application logs prior to off-system transfer.”

⸻

B. Lifetime definition

Auditors want clarity, not dogma.

Acceptable lifetime explanations include:
	•	“Key is rotated every 365 days”
	•	“Key is rotated at system rebuild”
	•	“Key is rotated on compromise, role change, or cryptographic deprecation”

What is not acceptable:
	•	“Key exists until it doesn’t”
	•	“We haven’t thought about it”

⸻

C. Rotation and revocation procedure

You must explain:
	•	How a new key is generated
	•	How old keys are retired
	•	What happens to previously signed artifacts
	•	How verification continues with historical keys

This can be procedural — it does not have to be automated.

⸻

D. Protection rationale (especially no passphrases)

This is where your earlier question lands.

You must explicitly explain:
	•	Why automated keys have no passphrase
	•	What compensating controls are in place
	•	Why that is sufficient for this threat model

This explanation is normal, expected, and allowed.

⸻

E. Event visibility

They will expect:
	•	Creation events logged
	•	Rotation events logged
	•	Failure to sign logged
	•	Verification failures logged

They do NOT expect:
	•	Secrets in logs
	•	Private key material logged

==================================================
3. Practical documentation checklist (use this)

You can literally drop this into a design doc or SSP.

⸻

Key documentation template

For each signing key:
	1.	Key Identifier
	•	Logical name
	•	Filesystem path
	2.	Purpose
	•	What is signed
	•	Why signing is required
	3.	Algorithm & Parameters
	•	Algorithm (e.g., RSA-3072, ECDSA-P384, Ed25519 — if FIPS-approved where required)
	•	Hash function
	4.	Key Generation
	•	Method (manual provisioning / automated first-boot / build pipeline)
	•	Entropy source
	•	FIPS mode considerations
	5.	Key Storage & Protection
	•	Location on disk
	•	Ownership & permissions
	•	SELinux type/domain restrictions
	•	Explanation for passphrase or no passphrase
	6.	Key Lifetime
	•	Defined validity period or policy
	•	Rotation triggers
	7.	Key Rotation & Revocation
	•	How new keys are introduced
	•	How old keys are retired
	•	How historic signatures are verified
	8.	Auditing & Logging
	•	What key events are logged
	•	Log location
	•	Review process
	9.	Failure Handling
	•	What happens if signing fails
	•	What happens if verification fails
	10.	Residual Risk Statement

	•	What risks remain
	•	Why they are acceptable

⸻

Example lifetime policy language (you can reuse)

“System-generated signing keys are valid for a maximum of 12 months and are rotated during scheduled maintenance or system rebuilds. Keys may also be replaced immediately following suspected compromise, cryptographic deprecation, or role change. Historical public keys are retained to support verification of previously signed artifacts.”

==================================================
4. Why this satisfies MLS / RTB thinking

This approach aligns with high-assurance expectations because:
	•	Keys are purpose-bound
	•	Lifetimes are explicit
	•	Protection mechanisms are documented and layered
	•	Automation is intentional, not implicit
	•	Residual risk is acknowledged, not ignored

RTB does not require:
	•	Online CAs
	•	Mandatory HSMs
	•	Perfect secrecy

It requires clarity, control, and traceability.

==================================================
Bottom line

Security controls require you to:
	•	Inventory keys
	•	Document purpose
	•	Define lifetime
	•	Justify protection mechanisms
	•	Log lifecycle events
	•	Explain failures and residual risk

They do not require:
	•	Passphrases for unattended services
	•	Infinite rotation
	•	Magical automation

If you want, next I can:
	•	Map all this explicitly to an SSP section
	•	Write a one-page “Key Management Summary” suitable for auditors
	•	Align this with your HA log signer and CDS-style audit model


Absolutely. This is the right level of precision, and it’s exactly what most designs and SSPs fail to do. I’ll be very concrete and explicit: what data, which files, which subsystems, which key, and why that distinction matters.

I’ll walk this as a canonical inventory, written the way an auditor or accreditation reviewer would mentally parse it.

⸻

	1.	Audit log signing key
(THIS IS STRICTLY auditd OUTPUT)

⸻

What data is being signed
	•	Binary audit records generated by the Linux Audit Framework (auditd)
	•	These are security event records, not general logs

Typical artifact paths
	•	/var/log/audit/audit.log (if using text dispatcher)
	•	/var/log/audit/audit.log.* (rotated)
	•	/var/log/audit/*.log (all audit trails)
	•	If using au-remote / dispatcher:
	•	data streams produced by audisp modules before off-host transfer

What is explicitly not included
	•	journald logs
	•	syslog logs
	•	application logs
	•	kernel messages outside audit framework

Purpose of this key
	•	Ensure audit trail integrity
	•	Detect tampering after generation
	•	Preserve forensic reliability

Why it must be a dedicated key
	•	AU-9 / AU-10 require audit data to be independently protected
	•	Audit logs cannot share keys with application or system logs

Summary line you can reuse
“Audit signing keys are used exclusively to sign files produced by auditd under /var/log/audit and are not shared with any other logging or system function.”

⸻

	2.	System / application log signing key
(NON-AUDIT LOGS – INTERNAL SYSTEM RECORDS)

⸻

What data is being signed
This key covers non-audit operational logs, including:
	•	Application logs
	•	Service logs
	•	Daemon logs
	•	Custom application logging

Typical artifact paths
Depends on logging architecture, but commonly:
	•	/var/log/messages (syslog legacy)
	•	/var/log/syslog (if enabled)
	•	/var/log/secure
	•	/var/log/maillog
	•	/var/log/httpd/*
	•	/var/log/nginx/*
	•	/var/log/<application_name>/*
	•	/var/log/journal/* (only if exporting to files)

Important distinction
	•	journald internally seals its own journal files
	•	journald sealing ≠ external cryptographic signing
	•	If journal files are exported, archived, or moved, this key applies

What is not included
	•	auditd output (/var/log/audit)
	•	Archived bundles (see next key)

Purpose of this key
	•	Detect tampering with operational logs
	•	Provide integrity prior to archival or forwarding
	•	Support incident response and troubleshooting trust

Summary line
“System and application log signing keys protect non-audit operational logs generated under /var/log, excluding files owned by auditd.”

⸻

	3.	Log archive signing key
(POST-ROTATION / BUNDLED ARTIFACTS)

⸻

What data is being signed
	•	Bundles or compressed sets of logs
	•	Immutable artifacts prepared for long-term storage or transfer

Typical artifact paths
Examples (depends on implementation):
	•	/var/log/archive/*
	•	/var/log/archives/*
	•	/var/backups/logs/*
	•	/srv/log-archive/*
	•	Temporary artifacts:
	•	/var/tmp/log-archive-YYYYMMDD.tar.gz
	•	*.tar.gz, *.xz, *.zip produced by logrotate hooks

These artifacts often contain:
	•	audit logs
	•	system logs
	•	application logs

Important rule
	•	Archive signing happens after rotation
	•	It signs the container, not individual log files
	•	Prior signatures must remain verifiable

Purpose of this key
	•	Preserve chain of custody
	•	Prove archive integrity after leaving the host
	•	Support long-term forensic retention

Why this is separate
	•	Rotation cadence differs
	•	Archive lifetime is often much longer
	•	Private key exposure risk differs

Summary line
“Archive signing keys protect packaged log artifacts prepared for off-system storage or retention and are distinct from per-log signing keys.”

⸻

	4.	Integrity verification / baseline signing key
(FILE-INTEGRITY / MLS SYSTEM TRUST)

⸻

What data is being signed
	•	Integrity baselines
	•	File hashes
	•	Expected-good state definitions

Typical artifact paths
Depending on mechanism:

IMA/EVM:
	•	/etc/ima/keys/*
	•	Measurement data under:
	•	/sys/kernel/security/ima/
	•	Hashes of:
	•	/bin
	•	/sbin
	•	/usr/bin
	•	/usr/sbin
	•	/etc (config files)

Custom baseline tools:
	•	/var/lib/integrity-baseline/*
	•	/etc/integrity/*
	•	/var/lib/aide/*

What is not included
	•	Logs
	•	Dynamic runtime data

Purpose of this key
	•	Detect unauthorized modification of binaries or configuration
	•	Support SI-7 and CM integrity requirements
	•	Anchor MLS trust decisions

Summary line
“Integrity signing keys protect cryptographic baselines used to validate the system’s executable and configuration state and are not used for logging or archival purposes.”

⸻

	5.	Secure Boot / platform trust keys
(ONLY IF IN SCOPE)

⸻

What data is being signed / verified
	•	Bootloaders
	•	Kernel images
	•	Kernel modules

Typical artifact locations
	•	Firmware databases (not typical file paths):
	•	UEFI db
	•	UEFI dbx
	•	On disk:
	•	/boot/efi/*
	•	/boot/vmlinuz-*
	•	/boot/initramfs-*

Notes
	•	Usually manufacturer- or org-managed
	•	Often out of scope for application-level key tooling

Summary line
“Platform trust keys establish a chain of trust from firmware through kernel boot and are managed separately from OS-level signing keys.”

⸻

	6.	TLS / service authentication keys
(INVENTORIED, NOT LOG-SIGNING)

⸻

What data is being signed
	•	TLS sessions
	•	Authentication handshakes
	•	Service identity assertions

Typical artifact paths
	•	/etc/pki/tls/private/*.key
	•	/etc/pki/tls/certs/*.crt
	•	/etc/ssl/private/*

What is not covered
	•	Log integrity
	•	Audit integrity

Purpose
	•	Confidentiality and authentication
	•	NOT forensic record integrity

Summary line
“TLS private keys are inventoried as cryptographic material but are not used for log or audit signing purposes.”

⸻

	7.	Package / artifact verification keys
(CONSUMER TRUST)

⸻

What data is verified (not signed locally)
	•	RPM packages
	•	Updates
	•	External artifacts

Typical artifact paths
	•	/etc/pki/rpm-gpg/*
	•	/etc/yum.repos.d/*.repo

Purpose
	•	Supply-chain integrity
	•	Installation trust

Summary line
“Package verification keys validate external software artifacts and are distinct from system-originated signing keys.”

⸻

	8.	Minimal system-required signing keys (explicit)

⸻

For a serious MLS / high-assurance system, a defensible minimum is:
	1.	Audit log signing key
	•	/var/log/audit/*
	2.	System/application log signing key
	•	/var/log/* (non-audit)
	3.	Log archive signing key
	•	/var/log/archive/*
	•	/srv/log-archive/*
	4.	Integrity baseline signing key (if integrity monitoring is enabled)

Everything else is inventoried, not necessarily generated.

⸻

	9.	Why this granularity matters (documentation gold)

⸻

With this breakdown, you can:
	•	Generate reports that say:
“This key signs these files.”
	•	Detect misuse:
“This key was used outside its scope.”
	•	Justify separation of duty
	•	Avoid the “everything is signed by one magic key” anti-pattern

This is exactly the clarity auditors want.

⸻

If you want next steps

I can:
	•	Turn this into a machine-readable schema (JSON/YAML)
	•	Design the directory layout under /etc/pki for each key class
	•	Sketch the CLI UX (“inventory”, “generate”, “rotate”, “verify”)
	•	Map each key/artifact pair directly to NIST controls in output

You’re thinking about this at the right level.

Got it, we’re off network interfaces and firmly back in signing-key land.

I’ll answer your repeated question in one coherent pass, pulling everything together:
	1.	What keys, and what should we name them?

Let’s assume this HA/FIPS-ish RHEL system with your envisioned utility is going to sign at least:
	•	System audit logs (auditd output files)
	•	System/application logs (rsyslog / journald / app logs under /var/log)
	•	Rotated log archives (e.g., compressed tarballs from logrotate)
	•	Configuration baselines (e.g., files under /etc)
	•	Software or script releases you ship internally
	•	Backups (optional but nice)

You want names that clearly express:

a) Scope/purpose
b) Algorithm family or size, if relevant
c) Rotation version or date

A good generic pattern:

-[vNN].key  for private keys
-[vNN].pub  for raw public keys
-[_vNN].pem  for X.509 or PEM-bundled public material

Where:
	•	scope = host or environment domain, e.g. host1, cdslogger, ha, corp
	•	use   = audit, syslog, log, archive, config, release, backup, etc.
	•	algo/size = rsa-3072, rsa-4096, ecdsa-p256, etc.
	•	vNN = version if you don’t want dates in names, e.g. v01, v02.

Examples for a host named “ha-logger01”:

ha-logger01_audit_sign_rsa-3072_v01.key
ha-logger01_audit_sign_rsa-3072_v01.pub

ha-logger01_log_sign_rsa-3072_v01.key
ha-logger01_log_sign_rsa-3072_v01.pub

ha-logger01_archive_sign_rsa-3072_v01.key
ha-logger01_archive_sign_rsa-3072_v01.pub

ha-logger01_config_sign_rsa-3072_v01.key
ha-logger01_config_sign_rsa-3072_v01.pub

If you prefer dates instead of vNN:

ha-logger01_audit_sign_rsa-3072_2025-12-08.key
ha-logger01_audit_sign_rsa-3072_2025-12-08.pub
	2.	Exactly what is being signed by each key?

You wanted this more concrete, so let’s pin paths and artifacts.

Audit signing key
	•	Purpose: Sign files produced by auditd (or your SQLite audit store if you export them as flat logs).
	•	Artifacts:
	•	/var/log/audit/audit.log
	•	/var/log/audit/audit.log.* (rotated)
	•	Any exported audit archives, e.g. /var/log/audit/archive/*.gz
	•	Example key names:
	•	Private: ha-logger01_audit_sign_rsa-3072_v01.key
	•	Public:  ha-logger01_audit_sign_rsa-3072_v01.pub

Logging (non-audit) signing key
You can go narrow or broad. I’d aim for “everything that logrotate touches except auditd,” and keep audit separate.
	•	Purpose: Sign rotated log files for:
	•	/var/log/messages, /var/log/secure, /var/log/maillog, etc.
	•	Application logs under /var/log//
	•	Optionally /var/log/httpd/, /var/log/nginx/ if you want them under the same key.
	•	Artifacts:
	•	/var/log/*.1, *.2.gz, etc.
	•	/var/log//.log.* (depending on logrotate config)
	•	Example key names:
	•	Private: ha-logger01_log_sign_rsa-3072_v01.key
	•	Public:  ha-logger01_log_sign_rsa-3072_v01.pub

If you want to split web server logs (because of different retention or external sharing), you could also have:

ha-logger01_web_log_sign_rsa-3072_v01.key
ha-logger01_web_log_sign_rsa-3072_v01.pub

Archive signing key
	•	Purpose: Sign higher-level archives, like:
	•	Tarballs created from log snapshots: /var/archives/logs/*.tar.gz
	•	Forensic bundles: /var/archives/forensics/*.tar.gz
	•	Artifacts:
	•	/var/archives/**/*.tar.gz (or whatever directory you choose)
	•	Example keys:
	•	Private: ha-logger01_archive_sign_rsa-3072_v01.key
	•	Public:  ha-logger01_archive_sign_rsa-3072_v01.pub

Configuration baseline signing key
	•	Purpose: Sign configuration “snapshots” or baselines.
	•	Artifacts:
	•	Checksummed config manifests, e.g. /var/lib/config-baseline/etc-baseline.json
	•	Tarballs of /etc state: /var/lib/config-baseline/etc-YYYYMMDD.tar.gz
	•	Example keys:
	•	Private: ha-logger01_config_sign_rsa-3072_v01.key
	•	Public:  ha-logger01_config_sign_rsa-3072_v01.pub

Software / script release key
	•	Purpose: Sign internal tools, scripts, and packages you ship around.
	•	Artifacts:
	•	Tarballs of internal software: /opt/ha-tools/*.tar.gz
	•	Scripts under version control when exported: /opt/ha-tools/bin/*.sh
	•	RPMs, if you roll your own repo.
	•	Example keys:
	•	Private: ha-logger01_release_sign_rsa-3072_v01.key
	•	Public:  ha-logger01_release_sign_rsa-3072_v01.pub

Backup signing key (optional)
	•	Purpose: Sign backup manifests or the backup archives themselves.
	•	Artifacts:
	•	/var/backups/*.tar.gz or whatever your backup tool writes locally before shipping off.
	•	Example keys:
	•	Private: ha-logger01_backup_sign_rsa-3072_v01.key
	•	Public:  ha-logger01_backup_sign_rsa-3072_v01.pub

	3.	Where should these keys live?

A clean, central layout under /etc/pki is absolutely reasonable. For example:

/etc/pki/ha-keys/
audit/
private/
public/
archive/
logging/
private/
public/
archive/
archive/
private/
public/
archive/
config/
private/
public/
archive/
release/
private/
public/
archive/
backup/
private/
public/
archive/

Or, to be simpler:

/etc/pki/ha-keys/audit/private/
/etc/pki/ha-keys/audit/public/
/etc/pki/ha-keys/logging/private/
/etc/pki/ha-keys/logging/public/
…

So, for example, the audit signing key pair:

/etc/pki/ha-keys/audit/private/ha-logger01_audit_sign_rsa-3072_v01.key
/etc/pki/ha-keys/audit/public/ha-logger01_audit_sign_rsa-3072_v01.pub
	4.	What about permissions and ownership?

For private keys:
	•	Directory: 0700
	•	File: 0600
	•	Owner: ideally a dedicated service account used only by your signing daemon, e.g. user “logsignd”, group “logsignd”.
	•	Example:
chown -R logsignd:logsignd /etc/pki/ha-keys
chmod 700 /etc/pki/ha-keys
chmod 700 /etc/pki/ha-keys//private
chmod 600 /etc/pki/ha-keys//private/*.key

If you don’t have a dedicated service user yet and this is all run by a root-managed daemon, then root:root 0700/0600 is acceptable as a starting point, but the long-term direction should be a restricted account whose only role is signing.

For public keys:
	•	Directory: 0755 (or 0750 if you really want to limit who can see them).
	•	File: 0644.
	•	Owner: same user or root:root; public keys are okay to read by many, but I’d still keep them in a well-defined place.

	5.	Passphrases or no passphrases?

For automated, non-interactive signing (log rotation, journald export, cron jobs), you typically cannot use keys that require passphrase entry at each use, unless:
	•	You have an agent process that unlocks the key once at boot (still tricky in a strict, headless, MLS/FIPS environment), or
	•	You store the passphrase in a file on disk, which defeats the whole purpose.

So, in practice, for these “machine” signing keys:
	•	Private keys are usually stored unencrypted on disk but:
	•	Protected by strict file permissions (0700/0600).
	•	Ideally bound to a restricted service user account.
	•	Optionally protected by hardware (TPM, HSM, smartcard) if your environment supports that.

For high-assurance environments, the ideal is:
	•	Use a hardware module (HSM or TPM) such that the private key never leaves hardware.
	•	Your script or daemon just asks the hardware to perform the signature.

But if you’re strictly on commodity RHEL without HSMs, unencrypted private key files with tight permissions and good process isolation is the usual path.
	6.	Is there any system startup that “auto-creates” keys?

Some services do auto-generate keys when missing (SSH host keys, some TLS tools, etc.), but:
	•	There is no general RHEL startup service that auto-generates the sort of specialized “log signing” or “audit signing” keys we’re describing.
	•	auditd, rsyslog, journald, logrotate, etc., do not automatically create signing keys for you.

So for your design:
	•	Assume: “Our utility (or its installer) is responsible for generating all of these signing key pairs.”
	•	On first run, the utility checks:
	•	If required keys exist in /etc/pki/ha-keys/…, otherwise it:
	•	Creates the directories with the right ownership and perms.
	•	Generates new key pairs with the expected naming convention.
	•	Registers/updates a metadata file (more on that next).

	7.	Should the utility produce a list of all public keys to transport?

Yes. Absolutely.

I’d have the utility maintain both:
	•	Per-scope bundles:
	•	/etc/pki/ha-keys/audit/public/audit_sign_pubkeys.pem
	•	/etc/pki/ha-keys/logging/public/log_sign_pubkeys.pem
	•	And a global bundle:
	•	/etc/pki/ha-keys/all_public_keys.pem

These would contain:
	•	All current active public keys, plus
	•	Archived public keys that you may still need to verify older logs and archives.

This makes it trivial to:
	•	Copy all_public_keys.pem to a verification node, SOC, SIEM, or offline forensic workstation.
	•	Have verification scripts automatically know where to look.

From a security/compliance perspective, yes: public keys are exactly what you want to be able to transport and publish widely, because they allow independent verification.
	8.	Will public keys ever be needed on another system?

Yes, in multiple scenarios:
	•	On an offline validation/forensic machine where you verify the integrity of:
	•	Exported audit logs
	•	Exported rotated logs
	•	Exported archives and config baselines
	•	On central log collectors (e.g., your SIEM) if:
	•	Your RHEL box signs logs locally, ships them, and the SIEM verifies signatures on ingest.
	•	On any host or tool that needs to check the authenticity of internal software releases or configuration baselines.

So your utility should absolutely support:
	•	“Export verification bundle” action that:
	•	Writes a clean set of public keys (and maybe metadata) for transfer.
	•	Optionally signs that bundle itself using a higher-trust key (meta).

	9.	How do we handle key rotation and archiving old public keys?

For each key type (audit, logging, etc.), I’d design the utility like this:

a) Metadata file per key family:
Example for audit signing:

/etc/pki/ha-keys/audit/audit_signing_keys.json

It would contain an array of records like:

[
{
“key_name”: “ha-logger01_audit_sign_rsa-3072_v01.key”,
“public_name”: “ha-logger01_audit_sign_rsa-3072_v01.pub”,
“created”: “2025-12-08T00:00:00Z”,
“activated”: “2025-12-08T00:05:00Z”,
“deactivated”: null,
“retired”: null,
“status”: “active”,
“algorithm”: “RSA-3072”,
“scope”: “audit”
},
…
]

b) Directory structure with an “archive” subdir:

/etc/pki/ha-keys/audit/private/
/etc/pki/ha-keys/audit/public/
/etc/pki/ha-keys/audit/archive/private/
/etc/pki/ha-keys/audit/archive/public/

When rotating:
	•	The utility:
	1.	Marks the current key in the JSON as “retired” with a “deactivated” timestamp.
	2.	Moves:
current private key   -> /etc/pki/ha-keys/audit/archive/private/
current public key    -> /etc/pki/ha-keys/audit/archive/public/
	3.	Generates new vNN+1 key pair into the main private/public directories.
	4.	Updates the JSON to mark the new key as “active.”
	5.	Rebuilds:
	•	/etc/pki/ha-keys/audit/public/audit_sign_pubkeys.pem
	•	/etc/pki/ha-keys/all_public_keys.pem

c) How old public keys are used:
	•	Verification tools must:
	•	Read the metadata and/or the bundle that includes both active and archived public keys.
	•	Choose the right key based on the signature’s key identifier (fingerprint, key ID), or just try all if the list is small.
	•	You NEVER delete old public keys until:
	•	All data signed with them has reached the end of its retention period and no longer needs to be verified.
	•	A documented key-destruction procedure says it’s okay.

	10.	Should the utility also generate a “key inventory” report?

Yes, and this ties nicely into your question about documenting lifetimes and controls.

The utility can produce, for example:

/var/lib/ha-keys/reports/key_inventory_YYYYMMDD.json
/var/lib/ha-keys/reports/key_inventory_YYYYMMDD.txt

Containing for each key:
	•	Key role: audit_sign, log_sign, etc.
	•	Key name and path.
	•	Algorithm and size.
	•	Creation date.
	•	Activation date.
	•	Deactivation / retirement date (if any).
	•	Intended lifetime (policy-based: e.g., 1 year, 2 years).
	•	Current status: active, retired, compromised, destroyed.
	•	Associated artifacts: e.g., “used to sign logs in /var/log/audit/”.

This inventory is:
	•	A direct input to your compliance documentation.
	•	A control artifact for auditors and for internal key-management review.
	•	Something you can tie to NIST SP 800-57-style key management requirements (documented key lifetimes, rotations, etc.).

	11.	Summary of what you asked:

	•	Naming convention:
	•	host_scope_use_algo-size_vNN for key filenames.
	•	Exact names:
	•	For example:
	•	ha-logger01_audit_sign_rsa-3072_v01.key / .pub
	•	ha-logger01_log_sign_rsa-3072_v01.key / .pub
	•	ha-logger01_archive_sign_rsa-3072_v01.key / .pub
	•	ha-logger01_config_sign_rsa-3072_v01.key / .pub
	•	Storage:
	•	Under /etc/pki/ha-keys//{private,public,archive}
	•	Access controls:
	•	Private: 0700 directories, 0600 files, owned by a dedicated signer account or root.
	•	Public: 0755 (or 0750) directories, 0644 files.
	•	Should the utility produce a list of public keys?
	•	Yes. Per-family bundles plus a global bundle for easy export to verification systems.
	•	Will public keys be needed elsewhere?
	•	Yes, for verification on other systems (SIEM, offline forensics, config or software validation).
	•	How do we archive old public keys?
	•	Move them to an archive subdirectory, track them in a metadata JSON, and keep them included in verification bundles until retention periods have expired and policy allows destruction.

If you want, next step we can sketch the directory tree and a small “key inventory” JSON schema plus a simple command-line design for your utility (e.g., ha-keyctl init, ha-keyctl rotate audit, ha-keyctl report).

Got it, dropping network interfaces and staying 100% on the signing-key utility.

Let’s treat what we already agreed on (which keys exist, where they live under /etc/pki/…, permissions, etc.) as fixed, and now design the script around that with:
	•	configurable base directory
	•	configurable algorithms and key lengths
	•	ability to discover existing keys and report on them

I’ll describe behavior first, then config, then how it inspects unknown keys.

⸻

	1.	Crypto defaults (what “strongest recommended FIPS/STIG” means)

⸻

For a RHEL 10 FIPS box that you care about lasting a long time and aligning with NIST/NSA guidance, a safe default for new signing keys is:
	•	Asymmetric algorithm: RSA
	•	Key length: 3072 bits

NIST SP 800-131A Rev.2 and SP 800-57 Part 1 treat RSA-2048 as acceptable but only 112-bit strength; RSA-3072 gives about 128-bit security and is the better forward-looking default.  

NSA and DoD guidance and RHEL’s CNSA-aligned crypto profiles are also moving toward “at least RSA-3072, 4096 preferred” for new keys.  

So for your script:
	•	default_algo = rsa
	•	default_bits = 3072

But the config must let you override per key type to:
	•	rsa: 2048, 3072, 4096
	•	ecdsa: p256, p384 (still FIPS-approved) if you want, as long as the backend supports it.

⸻

	2.	Big picture of the script

⸻

One Python 3 tool (name anything you like, e.g. ha_signing_ctl) that:
	•	Reads a config file describing:
	•	base_dir (e.g. /etc/pki/ha-signing, but overridable)
	•	global crypto defaults
	•	key “profiles” (audit_signing, logging_signing, config_backup_signing, release_signing, etc.)
	•	For each profile knows:
	•	algorithm and key length
	•	where the keys live (relative to base_dir)
	•	what artifacts they sign (paths and/or patterns)
	•	cryptoperiod (how long the key is supposed to be “active”)
	•	Commands:
	•	init           – create any missing keys for all or selected profiles
	•	rotate         – generate a new key pair, archive the old, update “current”
	•	list           – show a summary of keys and metadata
	•	report         – emit a formal report (text and/or JSON) about all keys
	•	scan-existing  – walk the tree and parse keys it did not originally create

Everything should be driven off a config file so you are not hard-coding paths or algorithms in the script.

⸻

	3.	Config file design

⸻

Assume a simple YAML (or TOML/INI if you prefer) file. I’ll show YAML because it’s clear to read:

Example: /etc/ha-signing/ha-signing.yml

global:

Can be overridden per-profile

base_dir: /etc/pki/ha-signing

default_algo: rsa
default_bits: 3072

Allowed algorithms and sizes so we stay FIPS-/STIG-aligned

allowed_algorithms:
rsa: [2048, 3072, 4096]
ecdsa: [p256, p384]

profiles:

audit_signing:
description: “Signs files produced by auditd (audit.log, rotated audit logs).”
algo: rsa           # optional; if omitted, use global default_algo
bits: 3072          # optional; if omitted, use global default_bits
subdir: audit       # keys under ${base_dir}/audit/
private_name: audit_signing_key.pem
public_name:  audit_signing_key.pub
artifacts:
type: files
paths:
- /var/log/audit/audit.log
- /var/log/audit/audit.log.*
cryptoperiod_days: 365

logging_signing:
description: “Signs non-audit system logs (rsyslog/journald exports, app logs).”
algo: rsa
bits: 3072
subdir: logging
private_name: logging_signing_key.pem
public_name:  logging_signing_key.pub
artifacts:
type: files
paths:
- /var/log/messages
- /var/log/secure
- /var/log/httpd/*
cryptoperiod_days: 365

config_backup_signing:
description: “Signs configuration snapshots and backups.”
algo: rsa
bits: 3072
subdir: config
private_name: config_backup_signing_key.pem
public_name:  config_backup_signing_key.pub
artifacts:
type: files
paths:
- /var/backups/etc/*
cryptoperiod_days: 730

release_signing:
description: “Signs software release bundles or deployed tarballs.”
algo: rsa
bits: 4096
subdir: release
private_name: release_signing_key.pem
public_name:  release_signing_key.pub
artifacts:
type: files
paths:
- /srv/releases/*
cryptoperiod_days: 730

Observations:
	1.	base_dir is configurable:
	•	Globally in the config.
	•	Optionally overridden by a command-line flag like:
ha_signing_ctl –base-dir /opt/my-keys init
	2.	Algorithms and key size are configurable:
	•	Global defaults plus per-profile overrides.
	•	Script validates that requested algo/bits are in allowed_algorithms and otherwise refuses to generate keys (so nobody sneaks in 1024-bit RSA).
	3.	Artifacts used in your reports:
	•	The script doesn’t actually have to sign here; it just documents “this key is intended to sign these paths” in the report.
	•	Your signing daemons or log-rotate hooks can then consume the same config to know which key to use.

⸻

	4.	Directory layout and permissions

⸻

Given base_dir = /etc/pki/ha-signing, a clean layout is:

/etc/pki/ha-signing/
audit/
current/
private/
audit_signing_key.pem
public/
audit_signing_key.pub
meta.json
archive/
2025-12-08T00-00-00Z/
private.pem
public.pem
meta.json
logging/
current/…
archive/…
config/
…
release/
…

Permissions/ownership (enforced by the script):
	•	Directories: 0700, owned by root:root (or a dedicated “ha-sign” service account if you separate duties).
	•	Private key files: 0600, owner=root, group=root (or your dedicated account).
	•	Public key files: 0644, owner=root, group=root is fine; or 0640 if you want them non-world-readable.
	•	meta.json: 0640 or 0644, depending on your preference.

The script should always:
	•	create directories with the right mode (os.makedirs with mode=0o700 + chmod).
	•	create private keys with umask 077 and explicitly chmod 0600 after write.
	•	create public keys with 0644.

⸻

	5.	How the script uses the config (per command)

⸻

I’ll stick to behavior, not full code, so we stay within the design level.

Command: init
	•	Inputs:
	•	optional profile name(s) (e.g. audit_signing, logging_signing, or all)
	•	optional overrides: base_dir, algo, bits (mostly for testing)
	•	Behavior:
	1.	Read /etc/ha-signing/ha-signing.yml.
	2.	Resolve base_dir (CLI flag > config > default /etc/pki/ha-signing).
	3.	For each selected profile:
	•	Construct paths for:
	•	current/private/<private_name>
	•	current/public/<public_name>
	•	current/meta.json
	•	If keys already exist and meta.json exists:
	•	Skip or warn: “audit_signing already initialized”.
	•	If missing:
	•	Generate key using OpenSSL or GnuPG with the configured algo/bits.
For example (OpenSSL RSA):
openssl genpkey -algorithm RSA -pkeyopt rsa_keygen_bits:3072 -out private.pem
openssl pkey -in private.pem -pubout -out public.pem
	•	Write meta.json containing:
{
“profile”: “audit_signing”,
“purpose”: “Files produced by auditd”,
“algo”: “rsa”,
“bits”: 3072,
“created_at”: “…”,
“cryptoperiod_days”: 365,
“expires_at”: “…computed…”,
“fingerprint”: “”,
“created_by”: “ha_signing_ctl”,
“version”: 1
}

Command: rotate
	•	Inputs:
	•	required: profile
	•	optional: reason string (“scheduled rotation”, “algorithm upgrade”, etc.)
	•	Behavior:
	1.	Read profile config and meta.json (if present).
	2.	Move existing current/* into archive/timestamp/.
	3.	Generate new key pair with same algo/bits (unless overrides are given).
	4.	Update meta.json with:
	•	old_fingerprint
	•	rotation_reason
	•	previous_expires_at
	5.	Optionally, if configured, mark the old key as “retired but still valid for verification only” vs “compromised”.

Command: list
	•	Behavior:
	•	Walk each profile directory and read current/meta.json plus archive/*/meta.json.
	•	Print a concise table:
profile, algo/bits, created_at, expires_at, age, status (active/expired/compromised).

Command: report
	•	Behavior:
	•	Same as list, but:
	•	Adds information about artifacts from the config (paths, purpose).
	•	Includes whether the key is within its cryptoperiod.
	•	Optionally includes any external keys discovered by scan-existing.
	•	Output formats:
	•	Plain text (for docs, STIG evidence).
	•	JSON (for machine consumption, GRC tooling).

Command: scan-existing

This is where it reads keys it did not create.
	•	Behavior:
	1.	Walk base_dir recursively.
	2.	For any file that looks like a key or cert:
	•	*.pem, *.key, *.crt, *.cer, *.pub
	3.	For each:
	•	Try “openssl x509 -in file -noout -text”:
	•	If it’s a certificate:
	•	extract subject CN, issuer, algorithm, key size, NotBefore/NotAfter.
	•	If not a cert, try “openssl pkey -in file -noout -text”:
	•	determine key type (RSA vs EC), and modulus or curve.
	•	For public keys in OpenSSH format (*.pub under .ssh or similar), run:
	•	ssh-keygen -lf file
	4.	Compute a fingerprint (e.g. SHA-256 of the DER public key).
	5.	Try to map purpose heuristically:
	•	Use directory path:
	•	if /audit/ in the directory path, likely audit_signing;
	•	if /logging/, likely logging_signing; etc.
	•	Use file name (contains “audit”, “logging”, “config”, “release”).
	6.	For anything not already known in meta.json:
	•	add an “external_keys” section in the report like:
	•	location, filename, algo, bits, fingerprint, not_before/not_after (if cert), guessed_purpose, comment=“discovered, not managed by ha_signing_ctl”.

That satisfies your “read existing public keys it did not create and determine key type/length/possible purpose/expiration from its location and name and expiration” requirement.

⸻

	6.	Naming conventions and public-key export

⸻

You asked specifically about:
	•	naming conventions
	•	where keys live (we already answered conceptually)
	•	whether the utility should produce a distributable list of public keys
	•	how to archive old public keys for verification

Naming conventions (tying to what we already standardized earlier):

Per profile (example):
	•	Private key (current):
	•	audit_signing_key.pem
	•	logging_signing_key.pem
	•	config_backup_signing_key.pem
	•	release_signing_key.pem
	•	Public key (current):
	•	audit_signing_key.pub
	•	logging_signing_key.pub
	•	config_backup_signing_key.pub
	•	release_signing_key.pub

For archives:
	•	Use a timestamped directory under archive/:
archive/
2025-12-08T00-00-00Z/
private.pem
public.pem
meta.json

or carry the profile name into the filename:

archive/
audit_signing_2025-12-08T00-00-00Z_priv.pem
audit_signing_2025-12-08T00-00-00Z_pub.pem
audit_signing_2025-12-08T00-00-00Z_meta.json

I like the “directory per rotation” approach better; easier to reason about.

Public-key export

Yes, the script absolutely should export public keys for transport, because:
	•	any external verification tool or remote system that wants to verify signatures on logs, backups, or releases needs those public keys;
	•	you also want an offline “trust anchor” bundle you can stash in your evidence locker.

Add a command:

ha_signing_ctl export-public –dest /var/tmp/ha-signing-pub-bundle

Behavior:
	1.	Create a directory or tarball named something like:
ha-signing-pubkeys-2025-12-08T00-00-00Z/
	2.	For each profile:
	•	copy current public key into that directory as:
audit_signing_key.pub
logging_signing_key.pub
…
	•	optionally also include all archived public keys renamed with their timestamp.
	3.	Generate a manifest.json:
{
“generated_at”: “…”,
“base_dir”: “/etc/pki/ha-signing”,
“profiles”: {
“audit_signing”: {
“current”: {
“file”: “audit_signing_key.pub”,
“fingerprint”: “…”,
“algo”: “rsa”,
“bits”: 3072,
“created_at”: “…”,
“expires_at”: “…”
},
“archive”: [
{
“file”: “archive/audit/2024-12-01T00-00-00Z_public.pem”,
“fingerprint”: “…”,
“algo”: “rsa”,
“bits”: 2048,
“created_at”: “…”,
“expires_at”: “…”
}
]
},
“logging_signing”: { … }
}
}

This manifest is then what you hand to other hosts/tools so they can verify historical signatures.

Archiving old public keys

You keep them:
	•	in archive/, as described
	•	referenced in manifest.json and/or the export bundle

Your verifying code needs to know which key to use. Easiest pattern:
	•	Whenever you sign something, also store:
	•	key fingerprint
	•	signing time
	•	When verifying, you:
	•	read the fingerprint from the signature metadata
	•	find that fingerprint in your current+archived public keys
	•	verify with that public key

Your ha_signing_ctl tool doesn’t have to be the verifier; it just needs to ensure fingerprints and archives are consistent and well-documented.

⸻

	7.	How this connects to key lifetimes and documentation controls

⸻

You asked earlier about “security controls and requirements for documenting and reporting on signing keys and lifetimes”. High level:
	•	NIST SP 800-57 Part 1 and SP 800-131A address appropriate key sizes and cryptoperiods (key lifetimes) for signing keys and emphasize documenting lifecycle events (generation, activation, suspension, compromise, destruction).  
	•	STIGs and similar overlays typically require:
	•	up-to-date inventory of cryptographic keys
	•	documented key purpose and usage
	•	defined cryptoperiods
	•	records of generation, rotation, revocation, and destruction.

Your script is basically a “key inventory and lifecycle evidence generator” tailored specifically to your local signing keys:
	•	meta.json files + reports == key inventory and lifecycle documentation.
	•	cryptoperiod_days + created_at + expires_at == explicit cryptoperiod.
	•	rotate + archive == lifecycle rotation evidence.

