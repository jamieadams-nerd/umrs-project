Umrs

Yes, I think that’s exactly the right direction: treat UMRS as a real Python package and make all of the “project-DNA” live there, including GTK look-and-feel helpers.

A reasonable structure might be something like this (module name lowercase, but “UMRS” in comments/docs):

umrs/
init.py
paths.py          # standard locations: /etc/pki/umrs, /var/lib/umrs, etc.
logging.py        # standardized logging/audit helpers
gtk_theme.py      # CSS + helpers for wizard/green dark UI
gtk_utils.py      # generic GTK helpers (window creation, dialogs, error popups)
sqlite.py         # hardened SQLite helpers (PRAGMAs, journaling, permissions)
crypto.py         # wrappers for signing, verification, key selection
audit.py          # helpers for dealing with auditd, parsing, events

For the GTK theming in particular, you could centralize something like:
	•	A standard UMRS CSS file (dark background, subtle green highlights, wizard-book vibe).
	•	A helper that loads that CSS into a Gtk.CssProvider and attaches it to the screen or the app.
	•	Optional helper to set a “UMRS default font” or widget class names so your CSS can target them.

Conceptually:
	•	umrs.gtk_theme.apply()
	•	Create CssProvider
	•	Load embedded CSS (string in the module, or read from a file like /usr/share/umrs/themes/dark.css)
	•	Attach to the default screen so all UMRS windows get the same style

Then every GTK tool you write just does, near the top:
	•	import umrs.gtk_theme
	•	umrs.gtk_theme.apply()

That gives you:
	•	Consistent wizard/dark/green look across all UMRS GUIs
	•	A single place to tweak colors later without touching every tool
	•	A natural home for things like “UMRS error dialog styling”, “UMRS warning banner”, etc.

And beyond theming, that UMRS package is the perfect place to centralize:
	•	Standard locations for keys, state, config (so every tool uses /etc/pki/umrs and /var/lib/umrs the same way).
	•	Common error handling and logging patterns (e.g., making sure every tool logs to the UMRS audit/log framework correctly).
	•	Shared security hardening (e.g., SQLite PRAGMAs, restricted file open helpers, FIPS-friendly crypto primitives, etc.).

So yes: a Python “umrs” package with a gtk_theme module under it is a very clean way to keep GUI styling and other cross-cut concerns consistent across the entire UMRS toolset.


Sure thing. Below is the same explanation and structure as before, rewritten in plain text only.
All code lines are prefixed with the section symbol (§) exactly as requested.

⸻

Concept: UMRS window superclass

Yes — your instinct is exactly right.

You want to establish the idea that:
	•	A “UMRS window” is not just any GTK window.
	•	It has:
	•	a defined look and feel,
	•	consistent behavior,
	•	shared security-related helpers,
	•	and a common identity across all UMRS tools.

So instead of every tool subclassing Gtk.Window or Gtk.ApplicationWindow directly, everything subclasses a UMRS base class.

That base class becomes the superclass for all UMRS GUI windows.

⸻

High-level design

Create a Python package named:

umrs

Inside it, create a UI subpackage:

umrs/ui

Inside that, define:
	•	A CSS manager (loaded once per process)
	•	A base window class (UmrsWindow)

Every GTK tool then does:

class MyToolWindow(UmrsWindow):

instead of subclassing Gtk directly.

This gives you:
	•	One place to control look and feel
	•	One place to add shared behavior later
	•	Consistency across every UMRS GUI
	•	Easier auditing and maintenance

⸻

Core ideas baked into UmrsWindow

An UmrsWindow should automatically:
	•	Load UMRS GTK CSS (dark + subtle green “wizard book” style)
	•	Add a shared GTK style class (e.g., “umrs-window”)
	•	Set sane default window sizes
	•	Center itself on screen
	•	Provide common dialogs (error/info)
	•	Be the place where future assurance-related UI behavior lives
