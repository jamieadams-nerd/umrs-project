
## Vision of UMRS Documentation

- Organized from the sense of introduction, to implementation, to reference for continued use.
- Invidiual sections tie together but could also be used as a stand alone if needed.
  - For example, Development Guide and subsections like high-assurance patterns, or Secure Rust




## General Ideas and Purposes of Sections
Looking the web interface, I am using the link names from the left side. My general 
suggestion is follows. Titles don't have to exact but groupings of information is key.
- Start Here
  - Introduction [new] What is UMRS basic goals
  - Scope and audience
  - Release notes
  - Legal notices
- Historical Background
  - Evolution of high-assurance
  - Previous tecnologies and systems
  - Military and other people used it more adopting it now.
  - A long history of "locking" down systems but not applying high-assurance techniques
    in operations or coding.
    - Mention things like HACAMS and Ada SPARK
  - The person who started this project had yeas of experience in classified systems
    - want to apply techniques and knowledge to unclassified.

## UMRS Architecture
- Base operating system
- CUI Policy
  - 5 eyes security
- SELinux Context MOdel
- MLS and Security Model
  - MCS Configuration

## Getting involved in UMRS
  - Try out the reference system
    - Deployment Guide:
      - Generic Linux Basline
      - RHEL 10
      - Ubuntu
  - Try out the programming techniques
      - Developer Guide
        - Language Guides
        - Buid & Workspace
        - Explain UMRS packages: platform, selinux, and core
        - Use Cases -> which UMRS code can we use (e.g., read /sysfs - use our secure reader)
      - Programming Resources
        - High-assurance patterns 
        - Other security techniques and available code we may have

## Operations (& Administration Combined?)
- Tools
  - umrs-ls
  - umrs-logspace
  - umrs-audit
  - umrs-state
- Auditing and Logging
- Integrity

## Security Controls
- Always cite security controls throughut the document
- This section should list each security document, version and it's role it played in the project:
  - NIST 800-53, CMMS, RTB, etc, etc.


## Initial reorganization
- Not everything is going to fit in the right place.
- Identify those items and move them to the _scratch area and try to organize/group the newly
  placed items. I will review them and give you advice.
- try to cross-reference items as much as we can with regard to coding technique, tool, 
  and even security controls.
