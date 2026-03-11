## Document 1
Time to documentt the os detection process and code. 
- OUr goal is to read the contents of os-release to get the goodies
- We can't trust the providence or content of the file
  - We must check each and everything
  - We can't even trust basic tools from libc or command line tools.
  - We fight our way through a ladder of trust (explain these trust levels)
- Cross-reference any other high-assurance patterns or coding techniques and security controls
- Use mermaid flow if needed 
- Target audience would be security auditors or to get new readers interested in the UMRS project.
- optionally you can mention the parsing of os-release in speciaized data types which are also
  verified.

## Document 2
- Just like Document 1 but more details
- Include code snippets if needed.
- detailed state or flow diagrams
- Serve as an explanation to security engineers and developers.
- optionally you can mention the parsing of os-release in speciaized data types which are also
  verfied.

## Antora
- Both doucments should exist in antora docs/
- Devel? Near high-assurnace patterns? After?
- One day they may serve as two seperate blog posts.
- This is a collection of technqiues many peeople struggle with implementing together.



