

# umrs-tui quick rework
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


# Documentation build changed slightly
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

# Inbox and Blog
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

# Major release or milestone
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

# Code work
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
    - Easeof use with the API do we need to refactor? Do we need more high-level abstraction
      for medion to lower level programmers. While still making sure they don;t  hurt themsevles.








