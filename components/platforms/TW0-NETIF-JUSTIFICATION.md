# Two Netowrk Interface Justification

In nearly every major security framework, guidance for network interface (NIC) **separation** is implied if not explicitly mandated. While none of the mainstream security publications say, in one sentence, _a system must have at least two NICs_, the controls strongly push you in that direction, especially in high-assurance or high-impact systems like the UMRS.

Two network interfaces offers seperation. This separation enables mandatory isolation, reduces attack surface, and prevents user-space or application-space compromise from immediately affecting administrative control channels. Therefore, at least two network interfaces (NICs) is the recommended minimum:
* [x] One interface is dedicated to user or application data flows (customer data, mission data, normal service traffic).
* [x] A second, separate interface is dedicated to administrative access (SSH administration, backup channels, configuration management systems, monitoring, orchestration).

Two network interfaces provides isolation and why this matters
- Prevents operational compromise from granting admin access.
- Allows firewalling, routing, and SELinux labeling to be stricter and more enforceable.
- Supports high-assurance evidence: you can prove that management traffic cannot traverse the same pathway as user-facing traffic.
- Makes monitoring more effective: the management network becomes a high-signal channel.
- Enables better compliance posture because you can demonstrate architectural separation (a key theme in many accreditations).

## Security Guidelines
Existing security guidance justifies two network interfaces. While the documents do not dictate **two NICs**, they all drive you directly to this architectural separation.

* Zero Trust Architecture (NIST SP 800-207):
  - Encourages segmentation of control planes and data planes. The cleanest implementation is a distinct interface for the administrative/control plane.
    
* NIST SP 800-171 (CUI environments):
  - Highlights segmentation, separation of duties, and isolating administrative mechanisms so they are not reachable through the same pathways that handle user data.
  
* DoD STIGs:
  - Many system STIGs (RHEL, networking, application servers) explicitly require administrative services to be bound only to trusted interfaces or isolated management networks. This strongly implies at least one dedicated management interface.
    
* High-Assurance & CDS / MLS environments (which you already know deeply):
  - These systems always physically and logically separate administrative networks from operational networks. Raise-the-Bar guidance emphasizes minimizing attack paths into administrative subsystems; the simplest enforcement is separate NICs and separate routing rules.

* NIST SP 800-53:&nbsp;
  * SC-7 (Boundary Protection) pushes systems to enforce strong separation of different traffic types.
  * AC-6 (Least Privilege) and AC-17 (Remote Access) both imply that administrative access paths must be tightly constrained and isolated.
  * CM-6 and CM-7 discuss reducing exposure and controlling where administrative functionality resides.
  

## Consider Several Network Interfaces

Two network interfaces remains the baseline for ordinary hardened systems. However, ff the infrastrucutre supports and the system warrants it, configure more than two when appropriate. Additional interfaces may be used for:
- out-of-band hardware management (IPMI, iDRAC, iLO)
- storage networks or replication
- separation of different mission enclaves
- cross-domain or MLS channeling
