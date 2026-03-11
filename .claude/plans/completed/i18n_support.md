## Support multiple langauges via the i18n framework

- The following crates each have their own text domain of the same name:
  - umrs-platform
  - umrs-selinux
  - umrs-core
- An executeable binary such as umrs-ls has it's own text domain (e.g., umrs-ls).
- I would like the binary to execute and present translations from it's text domain
  while at the same time the underlying crates are showing their own translations. 
- What is the best way to initialize these text domains from the binary. 
  - Can we do it without specialized calls to the library (or crates)?
- During development, I want to be able just to set the LANG= variable just like in real
  deploymet to test translations. 

## Coordinate the agents umrs-translator with the developer
- umrs-translator is responsible for extracting, translating, and tell the developer
  which strings in the source need to be wrapped for extraction.
- umrs-translator can coordinate and build required tools it needs to do extractions
  and manage it's /resources directory structure. 
  - Also review and manage the top-level Makefile

## Our first objective: Canadian French 
- Canadian French support for:
  - umrs-selinux
  - umrs-platform
  - umrs-core
  - umrs-ls
  - Have a canadian french version of documentation:
    - man page
    - help/yelp mallard doc
    - A new umrs-tool/umrs-ls section for french. 
      - how do we support multiple langauges in the antora playbook? 
      - Should we bother with it?


