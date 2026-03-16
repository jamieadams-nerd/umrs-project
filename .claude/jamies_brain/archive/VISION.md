The antora /docs structure needs to reorganized. 

## Vision of UMRS Documentation

My vision of the document from the highest level is as follows:
1. Introduction to UMRS
   - What is it?
   - Why you might interested in getting involved
   - What you might find useful.
   - Target audience Organizational owners, security auditors, and engineers/developers
2. History and Background
   - Singificant historical events related to high-assurnace and our security industry
   - Present a story of how we evolved to today.
   - High-assurance was used in many large projects
     - Now we are thinking beyond just hardening and locking down systems
     - Now we are aplying more high-assurance techniques and practices
3. UMRS Project Architecgture
   - MLS for CUI labelling was the original goal but it's expanded to so much more.
   - Base operating system platforms: RHEL10 and Ubuntu (refer to deployment guide)
   - Explaining SELinux
     - Targeted versus MLS
     - SecurityContext objects
     - MCS - provides labels 
   - Controlled Unclassified Information (CUI) STructure
     - how it is implemented in MLS SELinux
   - Exlplain Five Eyes programs as it relaed to CUI
4. Security Concepts
   - Not to be confused with the more detailed level of implemenation such as high-assurance
     patterns.
   - Guided by Security Controls and other sources
   - Privilege Seperations
   - Integirty 
   - Chain of custody
   - Providence.
   - Truth: Source Truth, Ground Trough, etc. (I have a whole document for this).
   - I have several other documents ready for review.
5. Deployment Guide
   - Installing a system to explore UMRS as well as evaluate tools
6. Existing OS Technologies for UMRS High-assurance
   - Stuff might be mentioned in OpenSCAP hardening but may not. 
   - Enabling IMA/EVM
   - Configuring Kernel LockDown
   - Enabling FIPS
   - I have numerous related from here. Move OS Post install procedures which might better here.
7. High-assurance Patterns.
   - We have several of these already written.
8. Development Guide
   - Explaining the UMRS API from a high-level:
     - What can stuff in umrs-platform give me?
     - What can stuff in umrs-selinux (and future umrs-apparmor) give me?
     - What useful things in umrs-core.
     - Key take away is to use this Rust packages to stay consistent with high-assurance 
       and security controls. We ahve proven, tested techniques to do things.
   - Give some high-level use cases and then show which parts of the API can assisst.
     - Use case: I want to examine the security aspects of a file --> Use SecureDirent struct object
     - Severl use cases to come. I will say, "Add this to  use case list"
9. Auditing and logging
   - Lots to come here so put a placeholder.
10. UMRS Tools 
   - Explain that UMRS tools are focused security persectives and posture. For example, UMRS-ls 
     gives directory listing but it gives you information in a way to help you understand the
     security posture.
   - umrs-ls, umrs-ps, umrs-state, umrs-logspace for now.
11. Ingesting Files (Vault manager) 
   - Placeholder
12. Operations
   - Day to day care of the system
   - How you can use the UMRS tools to evaulaute the system
13. Security Controls Reference
   - Should be albe to look up a security controls and it maps to a tool or source code module
     we  have implemented it. People can go and look at it to learn.
14.  Cryptography
    - According to security controls, I believe we should track every location we are using
      cryptography. 
    - similiar to 13, we should be able to look up a cryptography algorithm or cipher and see
      wherewe are using it. 
15. Glossary of Terms


## Initial reorganization
- Scan _scratch to find if anything can fit into of the new locaitons.
- Not everything is going to fit in the right place.
- Fit as best as you can. 
- Identify those items and move them to the _scratch area and try to organize/group the newly
  placed items. I will review them and give you advice.
- try to cross-reference items as much as we can with regard to coding technique, tool, 
  and even security controls.
