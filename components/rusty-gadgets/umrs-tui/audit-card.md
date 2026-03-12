---
 name: Audit Card
 path: rusty-gadgets/umrs-tui
 agent: rust-developer
---

rust-developer create a generic program using the ratatui crate that displays a generic
"audit card" for reproting on items. It is a basic text-based box-drawn report card which
can be used as a template for other programs. 

# Phase 1: Create audit card template code

## Output Structure
- Use ratatui-garnish crate features if useful
- Should be high-tech looking and flashy by using colors and such. 
  - Collaborate with the frontend skill/agent for style and output.
  - Use ANSI definitions and as much stuff from umrs-core as possible.
- By default, as wide and tall (vertical as possible).
- Basic key strokes: 'q' quits
- The output should look like:
```
  +---------------------------------------------------------+---------------+
  |                                                         |               |
  | HEADER                                                  |  text ascii   |
  |                                                         |   logo        |
  |                                                         |               |
  +---------------------------------------------------------+---------------+
  |                                                                         |
  |                                                                         |
  |                                                                         |
  |         Dynamic data area: Can be refreshed? Can scroll?                |
  |                                                                         |
  |                                                                         |
  |                                                                         |
  |                                                                         |
  |                                                                         |
  +-------------------------------------------------------------------------+
  | Single row for status: background colorized / updated with fn           | 
  +-------------------------------------------------------------------------+
```
- Header information can contain (maybe provide a template)
  - Hostname
  - Report name
  - oject we are reporting on 
  - Maybe ohter system state information
  - text ascii logo should be our small wizard. 
    - wizard should be light-green (or configurable).
    - See umrs-tui/examples/show_logo.rs 
  - Width and height of HEADER is contigent up on the text-ascii logo
  - In the future there maybe notebook tabs before dynamic data area and after header. Put
    placeholder code to accomodate
  - Dynamic data will vary.
    - Audit reports with key/pair values.
    - Possible scrolling table later with evnets and output of umrs-ls
    - Bottom single row status is desiged to updated via a function with messages and level
      indicator to change levels and such. 


## Code 
- Include our systemd-journald-logger as shown in the new skill for new crates. 
- Not stdout/stderr (what do we do with it in TUI?
- Make it easy to plug and play with new data. 
- Mkae it easy to configure new keys to actions. I am new to TUI so comments will help


## Phase 2: Example - Demonstrate umrs-platform/examples/os_detect.rs as audit card
- Once the template audit card is created, cargo build/run creates the "umrs-tui" binary.
- This is a huge milestone to demonstrate a key building block - the audit card.
- Using this new audit card template
- Use code from os_detect.rs to display dynamic data in an audit card.
- Two notebook tabs: (1) OS Information (2) Trust/Evidence information
- Define keystroes to toggle between two tabs. Something common in the industry.

## Phase 3: Future binaries
- The crate can support multiple binary builds using the template code.
- Configure the crate/cargo to support this. 





