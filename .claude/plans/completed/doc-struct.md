## Unhappy
- Dpocument structure is a wreck. 
- Navigation is a wreck with duplciates.
- modules versus subdirectories 
  - I don't know which and why one over the other

## Reader's persecptive
The information should be rouglly orgaized as follows:
- Introduction to UMRS
  - What is the project?
  - Why you should be interested.
  - Quick start to maybe Deployment and Development
- Generic Architecture 
  - SELinux Flask
    - MCS for labeling
- Security Concepts
  - Sourch of Trush concepts
  - Reference monitor
  - Lots in directory already.
- Deployment - how to configure a platform to get started in UMRS
  - Generic stuff common to all platforms
    - Filesystem layout
    - Dual network interface recommendations
  - RHEL 10 
    - Configure SELinux to Support CUI
      - Explaining our use of level and categories
  - Ubuntu
  - Assurance Enhancements
    - IMA/EVM
    - Isoalted /tmp setup
    - Kernel Security Settings
    - Enhanced journald configurations
- Development
  - Languages of our choice Rust (pro-lang* document)
  - Secure Coding stanards (bash, python, and rust)
  - High-assurnace patterns
  - Using UMRS crates/packages
    - umrs-platform
    - umrs-selinux / umrs-apparmor (mutually exclusive)
    - umrs-core
    - umrs-crypto
  - Use Cases - progrtamming tasks( you have many already)
    - Read files (kernel or regular)
    - Parsing correctly (use our stuff)
- UMRS Tools
  - umrs-ls
  - umrs-state
- Operations
  - Logging rotations/signing
  - key management.
  - need more organizaition.
  - Find files
- Reference (many are organized in suhdirectory).
  - SELinux 




- References
  - Security compliance
  - Deep dive os detection
  - 

