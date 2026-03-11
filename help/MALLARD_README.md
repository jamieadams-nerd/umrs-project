Mallard yelp

Good question — and this is a place where GNOME terminology often gets muddled, so let’s be very clear and precise.

When people say “desktop documentation” on modern RHEL / GNOME systems, they are usually referring to three related but distinct things:
	1.	Mallard
	2.	Yelp
	3.	Traditional UNIX help (man / info / HTML) as viewed through Yelp

I’ll walk through each, then explain how they fit together and when you should use them.

⸻

	1.	Mallard — the documentation language

⸻

Mallard is a semantic help markup language, designed specifically for task-oriented GUI help.

Key characteristics:
	•	XML-based (but far simpler than DocBook)
	•	Topic-oriented, not book-oriented
	•	Designed for:
	•	“How do I do X?”
	•	“Why did this fail?”
	•	“What does this button do?”
	•	Explicit relationships between topics:
	•	guides
	•	tasks
	•	concepts
	•	troubleshooting flows

Important conceptual shift from DocBook:
	•	DocBook: “Here is a book/manual”
	•	Mallard: “Here are connected help topics the user navigates based on need”

Mallard is not meant to replace:
	•	admin guides
	•	reference manuals
	•	man pages

It is meant to support in-application help.

Typical Mallard content:
	•	Short pages
	•	One task or concept per file
	•	Minimal prose
	•	Strong linking (“See also”, “Related tasks”)

⸻

	2.	Yelp — the help viewer

⸻

Yelp is not a documentation format.

Yelp is the GNOME help viewer application.

What Yelp does:
	•	Renders Mallard help
	•	Renders DocBook
	•	Renders man pages
	•	Renders info pages
	•	Renders HTML
	•	Provides search across help content
	•	Integrates with GNOME Shell (“Help” menu, F1 key)

Think of Yelp as:
	•	“The GNOME documentation browser”
	•	Comparable to:
	•	man for CLI users
	•	a PDF viewer for books
	•	a web browser for online docs

On RHEL / GNOME:
	•	When a GUI app launches help, it usually launches yelp
	•	Yelp decides how to render the content based on format

⸻

	3.	The “other thing” you were remembering

⸻

What people often half-remember as “Yelp and something else” is usually:
	•	Mallard + Yelp, or
	•	DocBook + Yelp, historically

In the GNOME 2 / early GNOME 3 era:
	•	Applications often shipped:
	•	DocBook XML
	•	Rendered via Yelp
	•	Over time, GNOME moved toward:
	•	Mallard for GUI help
	•	DocBook retained mainly for large manuals

So the pairing is:
	•	Mallard = how you write GUI help
	•	Yelp = how users read it

⸻

	4.	How this fits with your UMRS AsciiDoc pipeline

⸻

Your current setup already covers:
	•	Web documentation (Antora)
	•	Offline PDFs
	•	Man pages

Mallard/Yelp fits alongside, not instead of, that pipeline.

Recommended mental model:
	•	AsciiDoc:
	•	authoritative documentation
	•	admin guides
	•	security model
	•	accreditation-friendly prose
	•	PDFs and web output
	•	Man pages:
	•	precise CLI reference
	•	scripting and automation
	•	administrator workflows
	•	Mallard (optional, only if you have a GUI):
	•	short, focused help topics
	•	“what does this screen do?”
	•	“why is this disabled?”
	•	“how do I fix this error?”

Mallard should not try to mirror your full AsciiDoc content.

⸻

	5.	When you should use Mallard (and when not to)

⸻

Use Mallard if:
	•	You have a GTK GUI tool (which UMRS likely will)
	•	The GUI has:
	•	non-obvious behavior
	•	security-driven restrictions
	•	error states users will encounter
	•	You want:
	•	F1 / Help menu integration
	•	searchable desktop help
	•	context-aware guidance

Do NOT use Mallard for:
	•	Long admin manuals
	•	Security architecture documents
	•	Compliance mappings
	•	CLI reference

That content belongs exactly where you already put it.

⸻

	6.	How Mallard content is shipped

⸻

Typically installed to:
	•	/usr/share/help///

For example:
	•	/usr/share/help/C/umrs-gui/
	•	/usr/share/help/en_US/umrs-gui/

The application’s Help action launches:
	•	yelp help:umrs-gui

⸻

	7.	Practical recommendation for UMRS

⸻

Given everything you’ve built so far, the cleanest approach is:
	•	Keep AsciiDoc as your single source of truth
	•	Generate:
	•	HTML (GitHub Pages)
	•	PDFs (offline workstation docs)
	•	man pages
	•	Add Mallard only when:
	•	a GUI tool exists
	•	and there are real user-facing questions the GUI itself should answer

When you’re ready, the next logical step would be:
	•	define a minimal Mallard topic set (5–10 pages)
	•	decide which GUI screens deserve help
	•	map those to task/concept/troubleshooting Mallard pages
	•	keep the wording consistent with your AsciiDoc terminology

If you want, next time we can:
	•	sketch a minimal Mallard directory for a hypothetical UMRS GTK tool, or
	•	discuss whether Cockpit-style web UI documentation would be more appropriate for some UMRS components.

Just tell me which direction you want to go.

Yes — your understanding is exactly right.
Mallard is task-oriented, not reference-oriented.

Let me anchor this directly to your example, because you picked a perfect one.

⸻

Mallard answers: “What do I do right now?”

Not: “Here is the full theory.”

Mallard is designed for moments like:
	•	“I have this document open… what sensitivity is it?”
	•	“Why is this option disabled?”
	•	“How do I correctly handle this file?”

That maps precisely to your UMRS use case.

⸻

Your example, mapped correctly

“I want to know how to change a document’s sensitivity and handling.”

In Mallard, that would be a task page.

For example:
	•	Task: Change a document’s sensitivity label
	•	Task: Check a file’s sensitivity and category
	•	Task: Determine how a document may be shared

Each task page:
	•	answers one question
	•	has a clear “do this” flow
	•	avoids long theory
	•	links out to related tasks or concepts

⸻

What a Mallard task looks like (conceptually)

A single page would typically include:
	•	Title
“Change a document’s sensitivity”
	•	When to use this
“Use this when you need to update the sensitivity label of an existing file.”
	•	Steps
	1.	Locate the file in the UMRS workspace.
	2.	Right-click the file and select “Properties”.
	3.	Open the “Security” tab.
	4.	Select the appropriate sensitivity and category.
	5.	Apply the change.
	•	Important notes
	•	“You may not lower sensitivity without authorization.”
	•	“Some files cannot be relabeled once created.”
	•	Related tasks
	•	“Check a file’s sensitivity”
	•	“Understand document handling rules”

That’s Mallard’s sweet spot.

⸻

Where not to put certain information

This is where your instincts are very good.

Do not put these in Mallard:
	•	Full explanation of MLS theory
	•	Complete category taxonomy
	•	Accreditation or policy rationale
	•	Detailed NIST mappings

That content belongs in:
	•	Your AsciiDoc Security Model
	•	Your Administrator Guide
	•	Offline PDFs and web docs

Mallard can link to those:
	•	“For background on sensitivity levels, see the Security Model.”

⸻

How Mallard content is organized

Mallard doesn’t think in chapters. It thinks in relationships.

Typical structure:
	•	Tasks (how do I do X?)
	•	Concepts (what does this mean?)
	•	Troubleshooting (why did this fail?)

Your example would be linked like this:
	•	Task: Check a file’s sensitivity
→ Concept: What sensitivity labels mean
→ Task: Change a file’s sensitivity
→ Task: Handle a file with restricted sensitivity

Users don’t read it front-to-back.
They jump directly to what they need.

⸻

Why this is especially good for UMRS

UMRS has:
	•	non-obvious restrictions
	•	intentional security friction
	•	“this is disabled on purpose” behavior

Mallard excels at explaining:
	•	why something cannot be done
	•	what the correct action is instead
	•	what to check next

That’s exactly the kind of frustration-reducing help a secure system needs.

⸻

Short, direct answer to your question

Yes.

Mallard is task-oriented.
It is designed for “How do I…” and “Why can’t I…” situations.

Your document-sensitivity example is a textbook Mallard task page.

⸻

If you want, next we can:
	•	draft 5–7 concrete Mallard task titles specifically for UMRS, or
	•	write one complete Mallard task page in plain text so you can see how it feels compared to AsciiDoc.

You’re thinking about this exactly the right way.

