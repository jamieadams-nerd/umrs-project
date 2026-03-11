# The /tmp and /var/tmp Fiesystems

This document describes the historical background, purpose, security risks, and modern security controls associated with the `/tmp` and /var`/tmp` filesystems on UNIX and Linux systems. The intent is to explain why these directories exist, how their use has evolved over time, and why additional controls are required in modern operating environments.

## History of the Filesystems
The `/tmp` directory first appeared in early UNIX systems, including First Edition UNIX released by Bell Labs in 1971. Its original purpose was to provide a shared location for temporary files created during program execution. Programs used this directory to store intermediate results and short-lived data. At the time, UNIX systems typically operated in trusted environments with few users, and security threats were not a primary concern. The design therefore favored simplicity and shared access rather than isolation.

As UNIX evolved into a true multi-user operating system, the assumptions underlying `/tmp` began to break down. The directory was world-writable, which allowed any user to create, delete, or modify files created by others. This led to reliability and security issues, including accidental data loss, intentional disruption of other users’ processes, and early forms of denial-of-service. Programs could interfere with one another simply by manipulating temporary files.

To address these problems, the sticky bit was introduced. The sticky bit originally had a performance-related purpose when applied to executable files, but its meaning changed when applied to directories. In BSD UNIX systems, the sticky bit on a directory restricted file deletion so that only the file owner, the directory owner, or the superuser could remove files. This change was directly motivated by security problems observed in shared directories. Modern systems apply the sticky bit to `/tmp` by default, using permission mode 1777. This preserves shared write access while preventing deletion abuse.

As systems grew more complex, additional classes of security issues emerged. Symbolic link attacks became common once symbolic links were introduced. An attacker could create a link inside `/tmp` that pointed to a sensitive file, and a privileged program might follow that link unintentionally. Race conditions also became prevalent, especially where programs checked temporary files and then used them later, allowing attackers to alter those files between operations. Over time, information persistence became a serious concern as sensitive data, such as credentials and cryptographic intermediates, could remain in temporary files longer than intended. Denial-of-service risks increased as well, since users could fill `/tmp` or `/var/tmp` with large files and exhaust system resources.

The `/var/tmp` directory was introduced to address persistence concerns. Its purpose was to separate temporary data that should survive system reboots from truly short-lived data. Files in `/tmp` were expected to be removed frequently, while files in `/var/tmp` could exist longer if necessary. This distinction was later formalized by the Filesystem Hierarchy Standard and remains valid today.

## Modern Uses 
Modern operating systems continue to rely on both `/tmp` and `/var/tmp`. They are used by the operating system itself, by system services, and by applications such as package managers, compilers, web servers, and cryptographic utilities. Temporary storage remains necessary for correct operation. However, modern systems operate in environments that are fundamentally different from early UNIX systems. They are network-connected, multi-user, and capable of executing untrusted code. As a result, `/tmp` and `/var/tmp` are now treated as high-risk locations that require layered security controls.

Several modern security controls are applied to reduce these risks. The sticky bit remains a baseline protection against deletion abuse. Filesystem mount options are commonly used to further restrict behavior. The noexec option prevents execution of programs from temporary storage. The nosuid option prevents privilege escalation through setuid or setgid files. The nodev option prevents device files from being created or used. Together, these options significantly reduce attack surface without breaking most legitimate software.

Availability protections are also required. systemd-tmpfiles enforces lifecycle rules but cannot limit disk usage. Size limits must therefore be enforced by the filesystem. Modern systems often mount `/tmp` as `tmpfs`, which stores data in memory and supports explicit size limits. When the limit is reached, write operations fail, preventing large files from exhausting system resources. The `/var/tmp` directory is often placed on a dedicated filesystem or controlled with filesystem quotas to achieve similar protection while still allowing persistence across reboots.

Automatic cleanup is another essential control. Temporary data should not persist indefinitely, as lingering files increase the risk of information exposure. `systemd-tmpfiles` is commonly used to remove temporary files based on age. Retention periods should be defined deliberately, with more sensitive data retained for shorter periods. This supports data minimization and reduces residual risk. See my [system-tmpfiles cleanup](security.conf) example configuration to assist with this. 

Isolation is now considered best practice. Sensitive services should not rely on shared temporary directories. Instead, each service should use a dedicated temporary directory with restrictive permissions, clear ownership, and defined cleanup rules. This prevents cross-service data exposure and reduces the impact of misbehaving applications.

Mandatory access control systems provide enforcement beyond traditional permissions. SELinux can restrict which processes may access specific temporary paths, and MLS systems typically treat `/tmp` as _system-low_. Higher-sensitivity processes are prevented from staging data there, which avoids accidental downgrades and policy violations. Auditing further supports detection and investigation by recording access patterns and identifying abnormal usage.

Together, these measures address both historical and modern risks associated with shared temporary storage. The sticky bit prevents deletion abuse. Mount options reduce execution and privilege escalation risk. Size limits protect availability. Cleanup prevents information persistence. Isolation and mandatory access control enforce separation and policy.

The original design of `/tmp` assumed a trusted environment. Modern systems assume errors, misuse, and hostile activity. Layered security controls allow `/tmp` and `/var/tmp` to remain functional while aligning their use with today’s security expectations.


## Relavent Security Controls
This section maps the design, use, and security controls applied to `/tmp` and `/var/tmp` to applicable security control families. The intent is to demonstrate that the handling of temporary storage is deliberate, risk-informed, and aligned with mandatory security requirements rather than ad hoc hardening.

For clarity, here are the NIST 800-53 security control abbreviations:
* AC – Access Control
* AT – Awareness and Training
* AU – Audit and Accountability
* CA – Assessment, Authorization, and Monitoring
* CM – Configuration Management
* CP – Contingency Planning
* IA – Identification and Authentication
* IR – Incident Response
* MA – Maintenance
* MP – Media Protection
* PE – Physical and Environmental Protection
* PL – Planning
* PM – Program Management
* PS – Personnel Security
* RA – Risk Assessment
* SA – System and Services Acquisition
* SC – System and Communications Protection
* SI – System and Information Integrity
* SR – Supply Chain Risk Management


### Background and design rationale
Relevant controls: PL, SA

* PL-8 Security and Privacy Architectures - The historical background and evolution of /tmp and /var/tmp demonstrate that their modern treatment is part of a defined security architecture. The deliberate distinction between short-lived and longer-lived temporary storage reflects architectural planning rather than default operating system behavior.

* SA-8 Security Engineering Principles - The use of layered controls on shared temporary storage reflects defense-in-depth and least privilege principles. The system design accounts for known classes of weaknesses, including race conditions, symbolic link attacks, and information persistence.


### Shared temporary storage risks
Relevant controls: RA, SI

* RA-3 Risk Assessment - The identified risks associated with /tmp and /var/tmp, including unauthorized access, denial-of-service, and data persistence, reflect documented threat scenarios. These conditions are well known and supported by historical vulnerability data and security advisories.

* RA-7 Risk Response - The application of filesystem hardening, isolation, and lifecycle management represents a defined response to these risks rather than acceptance.

* SI-7 Information Integrity - Controls applied to temporary storage reduce the likelihood of unauthorized modification of data staged during program execution. Why it applies:
- Prevents unauthorized modification paths via execution of untrusted code
- Reduces integrity risk from malicious temporary payloads
- Often cited in audit crosswalks even if CM-6 is the primary


### Sticky bit and basic filesystem protections
Relevant controls: AC

* AC-6 Least Privilege - The sticky bit enforces a minimal privilege model within a shared directory. Users retain the ability to create files while being restricted from deleting or modifying files owned by others.

* AC-3 Access Enforcement - Filesystem permissions and sticky bit semantics are enforced by the kernel and cannot be bypassed by normal users.


### Mount options and execution restrictions
Relevant controls: CM, SI, SC

* CM-6 Configuration Settings - This is the core control.. Mount options such as `noexec, nosuid`, and `nodev` represent explicit configuration settings that define allowed system behavior. These settings are documented, reviewable, and enforceable. Why it applies:
- Explicitly requires the system to enforce organization-defined secure configuration settings
- Mount options (noexec, nosuid, nodev) are configuration settings enforced at boot
- STIGs are DISA’s concretization of CM-6
- Auditor language you’ll hear: “_System enforces approved configuration baselines for temporary filesystems._”

* CM-7 Least Functionality - Why it applies:
- This enhancement maps perfectly to noexec on /tmp and /var/tmp.
  - Removing executable capability from world-writable directories reduces available functionality
  - Prevents arbitrary code execution from /tmp or /var/tmp
- Bind-mounting keeps functionality minimal and consistent
- Especially relevant enhancement:
  - CM-7(2) – Prevent program execution in user-writable directories
 

* SI-16 Memory Protection - Preventing execution from temporary storage reduces exposure to injected or transient malicious code. (Indirect but commonly accepted in high-assurance environments). Why it applies:
- `tmpfs + noexec` limits executable attack surface in memory-backed storage
- Helps constrain runtime execution paths


* SC-7 Boundary Protection (local context) - Restricting executable behavior within temporary storage limits lateral movement and unintended execution paths inside the system.


### Size limits and availability protection
Relevant controls: SC, CP

* SC-5 Denial of Service Protection - Filesystem size limits and tmpfs enforcement prevent uncontrolled resource consumption that could degrade or deny system services.

* CP-10 System Recovery - By preventing disk exhaustion caused by temporary data, the system reduces the likelihood of conditions that impair recovery or require administrator intervention.


### Cleanup and retention controls
Relevant controls: SI, AU, DM

* SI-12 Information Management and Retention - `systemd-tmpfiles` enforces defined retention limits for temporary data. Files are removed based on age rather than relying on user behavior or reboot cycles. See my [system-tmpfiles cleanup](security.conf) example configuration to assist with this. Also, read the [official documentation](https://man7.org/linux/man-pages/man5/tmpfiles.d.5.html).

* AU-11 Audit Record Retention (indirect support) - Temporary directories are prevented from becoming unofficial audit or log repositories, preserving the integrity of defined audit retention policies.

* DM-2 Data Retention and Disposal - Automatic cleanup ensures temporary data is disposed of once it is no longer required for operational purposes.

### Isolation of service temporary storage
Relevant controls: AC, SC

* AC-4 Information Flow Enforcement - Dedicated service-specific temporary directories prevent unintended information flow between services through shared writable paths.

* SC-3 Security Function Isolation - Segregating temporary storage for sensitive services limits the impact of a compromised or misbehaving process.


### Mandatory access control and MLS enforcement
Relevant controls: AC, SC

* AC-2 Account Management (supporting) - SELinux contexts bind access decisions to system identities and roles rather than discretionary permissions alone.

* AC-6 Least Privilege - SELinux and MLS policy prevent higher-sensitivity processes from staging data in system-low temporary areas.

* SC-2 Application Partitioning - MLS enforcement ensures that temporary storage does not introduce unintended downgrade paths between sensitivity levels.

### Auditing and monitoring
Relevant controls: AU

* AU-12 Audit Generation - Access to temporary storage can be audited to detect abnormal usage patterns or misuse.

* AU-6 Audit Review, Analysis, and Reporting - Audit records associated with temporary storage support incident investigation and corrective action.


### Accreditation perspective summary
The treatment of `/tmp` and `/var/tmp` is not based on convenience or default operating system behavior.

It reflects:
- documented historical risk
- known vulnerability classes
- explicit architectural decisions
- enforceable technical controls
- alignment with recognized security standards

Temporary storage is managed as a controlled system resource. Its use is constrained by permissions, filesystem settings, lifecycle rules, mandatory access control, and auditing.

This approach demonstrates compliance with core security objectives related to confidentiality, integrity, availability, and controlled information flow, and is suitable for systems operating in regulated or high-assurance environments.

## HOW AUDITORS EXPECT THIS TO BE EXPLAINED
Accepted justification text (you can reuse this):

> “The system enforces secure configuration settings for temporary storage locations in accordance with NIST SP 800-53 CM-6 and CM-7. The /tmp filesystem is mounted with nodev, nosuid, and noexec options. The /var/tmp directory is bind-mounted to /tmp, inheriting the same security controls. This configuration prevents execution of untrusted code and unauthorized privilege escalation from world-writable locations.”

That language passes DISA, NSA RTB, and NCDSMO reviews.

---
_This document is licensed under the Creative Commons Attribution 4.0 International License (CC BY 4.0).
You may copy, redistribute, and adapt this material, provided that appropriate credit is given to the original author._

Author: Jamie L. Adams<br>
License text: https://creativecommons.org/licenses/by/4.0/
