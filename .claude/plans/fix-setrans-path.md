---
  name: Don't call loading functions with full path (setrans.conf and secolor.conf)
  paths:
     - components/rusty-gadgets/umrs-*
     - components/rusty-gadgets/umrs-*/{examples, tests}/

  agents: rust-developer
---


## Do not call fixed path.
- This path changes depending on the SELinux policy
- load_config() calls for secolor and setrans should let the underlying modules determine right
  files

- Fix the code
- Fix clippy fings
- Build and run all tests.


