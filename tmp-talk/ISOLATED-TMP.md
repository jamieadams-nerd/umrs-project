# PER-USER and PER-PROCESS TEMP DIRECTORIES
Some systems implement **per-user** or **per-process** temporary directories instead of relying on a single shared `/tmp` location. The purpose of these approaches is to reduce the risks associated with shared temporary storage while preserving necessary functionality.

* Per-user temporary directories provide each user with a private temporary workspace. Files created by one user are not visible to other users by default. This reduces the risk of unintended data disclosure and prevents one user from interfering with another user’s temporary files. Per-user temporary storage also limits the impact of denial-of-service behavior, since a single user cannot easily consume all available temporary space. This approach is well suited for multi-user systems where users operate independently and do not require shared temporary state.
  
* Per-process temporary directories provide a private temporary workspace for a specific process or service. These directories are typically created with restrictive permissions and owned by the service account. This approach prevents cross-process data exposure and makes it harder for one service to interfere with another. Per-process temporary storage is especially useful for privileged services or services that handle sensitive data. It also reduces the risk of symbolic link and race-condition attacks by removing shared writable paths from the threat model.


The main benefit of per-user and per-process temporary storage is isolation. Isolation improves confidentiality by preventing unintended data access, improves integrity by reducing interference, and improves availability by limiting the scope of resource exhaustion. These approaches also simplify auditing and incident response because temporary data is clearly associated with a specific user or service.

However, per-user and per-process temporary directories are not always appropriate. Some applications expect a global /tmp directory for compatibility or interoperability reasons. Legacy software may fail if shared temporary paths are not available or are restricted too aggressively. Systems that rely heavily on inter-process coordination through shared temporary files may also require careful redesign if isolation is introduced.

* Per-user temporary storage should be implemented when users do not need to share temporary data and when user isolation is a security priority. It should be avoided when applications rely on shared temporary locations or when the operational overhead of managing per-user directories outweighs the security benefit.

* Per-process temporary storage should be implemented for privileged services, network-facing services, and services that handle sensitive or controlled data. It should also be used when strong separation between services is required. It may not be necessary for simple or low-risk services where shared temporary storage does not present significant risk.

In practice, many systems use a mixed approach. Shared /tmp exists with baseline protections, while higher-risk users or services use dedicated temporary directories. This approach balances compatibility, security, and operational stability and is commonly used in modern, high-assurance systems.

There are two distinct ways Linux systems provide “per-user temp space”:
1.	PAM-based per-user temp directories (older, explicit)
2.	systemd PrivateTmp isolation (modern, service-scoped)

They solve related but different problems.


## PER-USER /tmp

### 1.	PER-USER / PAM NAMESPACE /tmp
This is the classic mechanism you’re remembering.

Component:
pam_namespace.so

What it does:
• Creates private mount namespaces per login session
• Each user sees their own /tmp and /var/tmp
• Files are not visible across users
• Typically implemented using bind mounts or tmpfs overlays

This dates back a long time — Solaris did similar things even earlier, which may be why it feels “old but real.”

How it works conceptually:
User logs in
PAM creates:
/tmp/user_
/var/tmp/user_
Then bind-mounts them to:
/tmp
/var/tmp
But only inside that user’s mount namespace

Other users do not see the same contents.

Configuration files:
- /etc/security/namespace.conf
- /etc/security/namespace.d/

Example concept (simplified):
```fstab
/tmp     /tmp/user_%{UID}     level=root,root
/var/tmp /var/tmp/user_%{UID} level=root,root
```
pam_namespace is then enabled in PAM stack:
auth or session required pam_namespace.so

⸻

STIG / NIST POSITION ON THIS

Is it allowed?
Yes.

Is it required?
No.

Is it recommended?
It depends — and this is important.

Security benefits:
• Stops classic /tmp symlink attacks between users
• Strong containment in multi-user systems
• Very strong alignment with:
AC-6 Least Privilege
CM-7 Least Functionality
SI-7 Integrity

Operational downsides:
_ Breaks legacy software that expects shared /tmp
- Some installers and compilers behave badly
- Debugging gets harder
- Not intuitive for admins

DISA / STIG stance:
- Allowed but NOT mandated
- Considered “environment-specific hardening”
- Often discouraged on general-purpose systems unless well-tested

High-assurance environments?
More common — but still carefully controlled.


## PER-PROCESS / systemd PrivateTmp (MODERN, SERVICE-LEVEL ISOLATION)
This is newer and far more common today.

Component: `systemd`
Directive: `PrivateTmp=true`

What it does:
- Each daemon gets its own `/tmp` and `/var/tmp`
- Implemented via mount namespaces
- Affects services only — NOT interactive users

Example:
```
[Service]
PrivateTmp=true
```

This gives:
- SSHD has its own /tmp
- httpd has its own /tmp
- postfix has its own /tmp
- Users still share the normal /tmp

This is now widely recommended and frequently STIG-aligned. NIST alignment: CM-6, CM-7, AC-3, AC-6, and SI-7. It is also RTB-friendly, Low breakage, and Easy to audit.


### WHICH ONE SHOULD YOU USE?

Interactive per-user /tmp (pam_namespace):
* Real
* Powerful
* High breakage risk
* Generally NOT recommended unless:
  – Strong multi-user threat model
  – Heavy testing
  – Well-controlled software stack

Service-level PrivateTmp:
* Strongly recommended
* Low risk
* Excellent security return
* Increasingly common in STIG’d systems


#### WHAT MOST HIGH-ASSURANCE SYSTEMS DO TODAY

Practical modern posture:
- /tmp mounted noexec,nosuid,nodev
- /var/tmp bind-mounted to /tmp
- PrivateTmp=true for most system services
- NO pam_namespace for interactive users unless explicitly justified

This balances: Security, Auditability, and Operational sanity






You need:
• A ROOT-OWNED template directory
• ROOT controls creation
• USERS do NOT own the template

We will use these paths (recommended):

/tmp/user_tmpl
/var/tmp/user_tmpl

⸻

A3. CREATE TEMPLATE DIRECTORIES

Commands:

mkdir -p /tmp/user_tmpl
mkdir -p /var/tmp/user_tmpl

Ownership (CRITICAL):

chown root:root /tmp/user_tmpl
chown root:root /var/tmp/user_tmpl

Permissions (CRITICAL):

chmod 0755 /tmp/user_tmpl
chmod 0755 /var/tmp/user_tmpl

Why 0755 and NOT 1777?
• These are NOT temp directories themselves
• They are containers for per-UID directories
• Users must NOT be able to create arbitrary siblings

This is one of the most common mistakes.

⸻

A4. PER-USER DIRECTORIES (CREATED AUTOMATICALLY)

pam_namespace will create:

/tmp/user_tmpl/
/var/tmp/user_tmpl/

These directories will be:

Ownership:
:

Permissions:
0700 (or 0700-equivalent)

You do NOT manually create them.

Verify after login:

ls -ld /tmp/user_tmpl/*
ls -ld /var/tmp/user_tmpl/*

⸻

A5. CONFIGURE /etc/security/namespace.conf

Edit:

/etc/security/namespace.conf

Minimal, correct configuration:

/tmp     /tmp/user_tmpl/%{UID}     level:level,root
/var/tmp /var/tmp/user_tmpl/%{UID} level:level,root

Explanation:
• Left column: what appears inside the namespace
• Middle: backing directory path
• %{UID}: numeric UID expansion
• level:level,root — required for SELinux awareness

DO NOT add mount options here — they inherit from the source FS.

⸻

A6. ENABLE pam_namespace IN PAM STACK

Edit BOTH files:

/etc/pam.d/system-auth
/etc/pam.d/password-auth

Add under the “session” section (near the end):

session required pam_namespace.so

Order matters:
• After pam_limits.so
• Before pam_systemd.so is usually safe

DO NOT place this in auth or account sections.

⸻

A7. VERIFICATION (DO NOT SKIP)
	1.	Login as a non-root user
	2.	Run:

mount | grep “ /tmp “
mount | grep “ /var/tmp “

You should see bind mounts specific to the session.
	3.	Create a temp file:

touch /tmp/testfile

Log in as a DIFFERENT user:
• File should NOT exist
	4.	Confirm permissions:

ls -ld /tmp
ls -ld /var/tmp

Inside namespace:
• /tmp shows normal path
• But content is private

⸻

A8. STIG / AUDIT NOTES (VERY IMPORTANT)

• SCAP may falsely flag /var/tmp
• Documentation may be required
• This is allowed but not required
• Often treated as “compensating control”

This is advanced hardening, not baseline STIG.

============================================================
PART B — SYSTEMD PrivateTmp (RECOMMENDED, LOW RISK)

This is service isolation, not user isolation.

⸻

B1. WHAT PrivateTmp DOES

• Creates a private mount namespace
• Gives the service its own /tmp AND /var/tmp
• Fully transparent to interactive users
• Minimal breakage risk

⸻

B2. ENABLE PrivateTmp FOR A SERVICE

Example: sshd

mkdir -p /etc/systemd/system/sshd.service.d

Create override:

/etc/systemd/system/sshd.service.d/10-privatetmp.conf

Contents:

[Service]
PrivateTmp=true

⸻

B3. APPLY CHANGES

systemctl daemon-reexec
systemctl restart sshd

⸻

B4. VERIFY

Find process:

pidof sshd

Then:

ls -l /proc//root/tmp
ls -l /proc//root/var/tmp

You will see isolated directories.

⸻

B5. OWNERSHIP DETAILS (SYSTEMD HANDLED)

systemd automatically creates:

/tmp/systemd-private--.tmp
/var/tmp/systemd-private--.tmp

Ownership:
root:root

Permissions:
0700

DO NOT TOUCH THESE DIRECTORIES.

⸻

CCE / STIG CROSS-REFERENCE (SUMMARY)

pam_namespace:
• No direct “must-use” CCE
• Used as compensating control for:
CCE-80844-1 (/tmp noexec)
CCE-80845-8 (/tmp nosuid)
CCE-80846-6 (/tmp nodev)
CCE-80847-4 (/var/tmp protected)

PrivateTmp:
• Contributes to same controls
• Often satisfies CM-7(2) expectations
• Rarely flagged by SCAP

============================================================
FINAL RECOMMENDATION (STRAIGHT TALK)

If you are building:
• General STIG-compliant systems → DO NOT enable pam_namespace
• Multi-user hostile systems → Consider pam_namespace carefully
• High-assurance modern Linux → Use PrivateTmp + hardened mounts

pam_namespace is a scalpel.
PrivateTmp is seatbelts and airbags.

If you want next:
• SCAP tailoring examples
• SSP-ready justification language
• MLS-specific notes (pam_namespace + MLS can bite)

