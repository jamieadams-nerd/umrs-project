# Major Items Before First release to public
- Before we can really can release use UMRS, there are some major things to complete. 
- Before we were calling it phase 1 for lack of better term

1. MUST COMPLETE CUI LABELING WORK
  - Documentaion: 
    [ ] Completely doucment CUI Structure
    [ ] How to configure MCS with translations (we will build a rust installer/configurator)
    [ ] Explain the groupings of CUI We have. 
  - CUI and Other definitions
    [ ] Fill in missing information in NARA US-CUI.json catalog (some handling is missing)
    [ ] Must figure out U.S. DoD CUI https://www.dodcui.mil/Critical-Infrastructure/ 
        - Why are there repeated categories from the NARA registry?
        - I thought the NARA was official source and mandated proper markings.
        - Does earch organization use the registry as a reference?
        - Looks like I must create another US.DOD-CUI.json (we will automate as much as we can
          through a web retrieval from the aforementioned URL.
    [ ] To support the tool, where to we store the json catalogs. Use XDG? ~/.local/conff 
        or whatever it was? Get security-engineer involved. 
        - We need to consider breaking up the json catalog into different files to match the 
          naming convention and structure described below for the setrans configuration files. 
    [ ] Build an installer/configurator to load specifics definitons into mcs. 
        - tool will be called umrs-mcs 
        - The tool will need to make sure the mcs package is installed.
        - The tool will need to run safely as root. So lock in the permissions the best we can.
          - Get security-engineer involved
        - /etc/selinux/targeted/setrans.d is where multiple configuration files will sit (e.g.,
          us-nara-lei.conf, us-nara-agr.conf)
        - tool will load/unload desired definitions by using the "include" statement in
          /etc/selinux/targeted/setrans.conf 
        - Be prepared to switch to /etc/selinux/mls support, too.
        - Once the tool has updated setrans.conf, it will restart the mcs service: systemctl mcs
          restart
        - Need to auotmate creating the five eyes definition and json files: ca-unclas.json,
          ca-setrans.conf etc, etc. This will include populating the json catalog with appropriate
          references and handling. Use the U.S. one as a reference. 
        = Need to give the tool the ability to create a test vault. 
          - ONLY the categories/translations we've loaded into setrans.conf
          - Given a specific directory (the "Vault"),  the tool will create a directory and
            subdirectory example with several junk test files.
            - These are to demonstrate labeling  
            - For example, top-level of the vault is the country/program (perphaps) like: 
              1. CUI//
              2. CUI//LEI .. CUI//AGR
              3. Under those their subcategories like CUI//LEI/INV
              4. In those filders create test files with random text inside then label 
                 chcon each folder and file according to hits label. 
            - Leave the test vault. Then they can use umrs-ls to explore it.
            - Optionaly, we set up fcontext so the restorecond autmoaticaly laels files for them.
            - Need documentation here to show them out to examine an explore this.

2. Finish up our os-detect tool to be workable and acceptable.
  - Execute the TUI improvement plan that includes renaming of items.
  - Have documentation team write umrs-stat and umrs-uname. These are two we want available in first
    release. 
  - Work with security-engineer to create the xtask required to install these in ~/.local/bin
  - I'd eally like those helper functions moved to umrs-platform before release.
  - Complete the performance benchmark and make performance improvements.
  - Have the security-auditor, system-engineer, and interns review the tools again.
    - It would be nice to ensure the interns don't rely on the memory too much because I kind of
      what them to "arrive" here each time with basic instructions and only the documentaotn
      available to them. 
  - Have the guest-coder try a couple more examples. Let's come up with a few good examples. Or even
    ask the team like security-engineer or auditor they'd like to see a tool do. Remember simple,
    focused pieces of small information in a CLI. I just want the intern's focus on usabulity of hte
    API and docs. After running it, we write the tutorial. They return with fresh memory, and try
    the tutorial and we absorb the feedback. 
  - Have the security-engineer and auditor review the code for a good scrub. 
    - Ensure we are using our typed data types where we can. Like avoid generic strings for a
      version when we could use one of our types. but don't beeak out package dependency rules flow
      though. 
 

 3. Infrastructure for Public stuff and Documenting my AI Experience
  - Be sure to include me on the task lists. I have deliverables to the tema, too. 
  - We need to figure out public website solution that has seardch and analytics. 
    - Get Sage as much power as we can so she publish and manage this. 
  - I will be working hard posts with Sage.
  - I will setting up our YOutube channel
  - Be prepared to help with the AI documentation:
    - I want to idnetify every file in .claude of what it is how it works in a loop.
    - I want cleary document our processeswith references to files which insruct and files that we
      store stuff in:
      - How wexpand our knowedge (e.g., I or a team member requests information. 2. researcher gets
        it. etc etc etc).
      - How we plan from brain to deliverable. 
      - how the team communicates with each other
      - How feedbacks are stored and procssed.
      - Guard rails you and may have in place: e.g., swim buddies or other rules that help
        effeciency.
      - How random thoughts or rougue research data gets dropped into the inbox fo the writers, and
        the incorporate it. With you , we check our existing plans to see where this stuff might fit
        or is useful.

# Some minor Disjointed Tasks

## umrs-tui quick rework
- the bin/umrs-file-stat should become it's own binary crate called umrs-stat.
- the umrs-tui main.rs can become it's own binary umrs-uname (formerly, it built as umrs-os-detect).
- I am trying to overlay classic linux tools with umrs. Ourtools just provide a security focus
  perseptive.
  - umrs-ls
  - umrs-stat
  - umrs-uname 
  - More to come
- Once the two binaries are removed, convert umrs-tui to a dedicated umrs-ui library the other two
  tools are using. 
  - This sets us up for some quick tool completions which are needed.


## Documentation build changed slightly
- make docs -- now builds a limited set of modules.
  - These are intended for internet consumpiton 
  - these are modules considered okay to be seen  by the public
  - builds in build/site/
  - I will finish github automation so it will build into github pages under umrs/
    - this addresses our public visibility
    - We are still evualting a permenant home that has analytics and search
- make docs-draft -- builds all modules for me and others to review html
  - builds in build/site-draft
- Also some changes have been madke in ROOT module. Minor

## Inbox and Blog
- Inbox concept works great, have sage and tech-writer archive items they've acted upon though
- Once blog post has been publicly made available
  - make sure we update our doc with any changs I had to make on the fly
  - archive blog or mark it is published somehow
- I'd also like to look at creating a beautiful PDF out of our blog posts. 
  - They are worth it and some engineers like to have those at the ready.
  - And if you have a collectin of PDF, can you make a "catalog" that's searchable?
- In parallel with our regular topics to post about, I want to:
  - Document my AI journey - thisis my first project using AI and agents
  - Periodic posts about what worked for me and sharing technqiues
    - simple things like feedback loops to guardrails like my swimbudy concpet to address errors/and
      issues with claude code agents erroring out or hanging.

## Major release or milestone
- Not surewhat the phases are but we need iron these out. 
- The first available stuff in my opinion would be a system with:
  - CUI Labeling on a system running targeted policy.
  - Read my notes on what this means. It's about the awarness of the label serious enforcement will
    later. 
  - Labeling is a new concpet to most
  - System will have a cuople of basic tools: umrs-uname, umrs-ls, and a cuople of others
  - Base software stack
  - Deployment guide to know how to setup in rhel 10
    - Don't bring up ima/evm or that enhanced high-assurance until we can smooth out instructions in
      next releae. 
- Next release
  - More tools 
  - More stuff in the software stack that sage said is "hot" such as environmet scrubbing
- In parfallel, we will be doing blog posts and other things. there will be technical research and
  work.
  - Finish our public publishing of the docs set (lmited).
  - Consider how we wil continue with "help/mallard" -- i;d like to see interactive help with TUI's
    -- this is a new concept.
  - Let's put our QA (vale) stuff on the back burner. too much distraction. 

## Code work
- More code comment review. 
- We need to get our rustdoc API read for public viewing view github pages.
- CODE is king. I know we need comments to explain and cross-reference but when it clutters the
  code, engineers despise that.
- I will be doing reviews of cdeo witha focus on readability which helps in two areas: 
  - 1) claritfy for auditors tosee what's going, and 
  - 2) developes to feelmore comfortable with what's goinon in the black box and they can modiyf or maintain it. 
- Developer guide will get lots of work. 
  - let's make sure our documentation theme is good for that.
  - We ave the right examples.
  - We will soon deploy our summer intern (guest-coder).
    - armed with onlyl the documentation and PI
    - They will write example programs excercising various libraries and features of our stack
    - they're feedback will be on helpfulness of documentation, clarity etc.
    - Ease of use with the API do we need to refactor? Do we need more high-level abstraction
      for medion to lower level programmers. While still making sure they don;t  hurt themsevles.
