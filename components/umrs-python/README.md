# UMRS Toolbox Concept 

A graphical user interface to view information about the Unclassified MLS Reference System (UMRS).

Concept overview

You can have multiple GTK4 applications that:
	•	run perfectly fine on their own (standalone mode), AND
	•	can also be launched from a single “toolbox” application that uses a GTK4 GridView.

Good news:
	•	The standalone applications do NOT need any special coding to support this.
	•	The toolbox simply launches them as separate processes.
	•	Each tool keeps its own Gtk.Application instance, window lifecycle, and audit boundaries.

This is actually the preferred model for high-assurance systems:
	•	clean isolation
	•	predictable behavior
	•	simpler auditing
	•	fewer GTK interdependencies



GTK4 Important takeaways (architecture)
* Gtk.IconView is gone in GTK4.
* Gtk.GridView is the correct modern replacement.
* Standalone tools need no special hooks to be launched from a toolbox.
* Launching tools as separate processes is simpler, safer, and more auditable.
* This pattern maps perfectly to UMRS:
* toolbox = capability launcher
* tools = independent, auditable applications
* future hooks (MLS, FIPS checks, logging) fit naturally

This is a clean, modern GTK4 design and a solid foundation for your UMRS platform.

Good news: if you get the toolbox running in the right SELinux/MLS context, the child apps will naturally inherit that same context. So most of the work is in how you launch the toolbox, not in each sub-program.

Let's break it into two parts:
1. Launching from the desktop
2. Ensuring the SELinux/MLS security context is what you want

⸻

## Launching the toolbox from the desktop

Typical flow on RHEL 10 with GNOME:
* You install a small wrapper script somewhere in $PATH (e.g. /usr/local/bin/umrs-toolbox).
* You create a .desktop file in either:
* /usr/share/applications  (system-wide)
* ~/.local/share/applications (per-user)

The .desktop file just calls the wrapper script. Example:
```
[Desktop Entry]
Type=Application
Name=UMRS Toolbox
Comment=Unclassified MLS Reference System Toolbox
Exec=/usr/local/bin/umrs-toolbox
Icon=umrs-toolbox
Terminal=false
Categories=System;Utility;
```

Wrapper launcher script:

```bash
#!/bin/bash
§exec /usr/bin/python3 -m umrs.toolbox_main
```

Make it executable:

```bash
$ chmod 755 /usr/local/bin/umrs-toolbox
```

At that point:
* GNOME sees “UMRS Toolbox” in the app grid.
* When the user clicks it, GNOME runs /usr/local/bin/umrs-toolbox.
* That script starts your GTK4 toolbox app.

Your three standalone tools (say script1.py, script2.py, script3.py) can each have their own .desktop entries too, if you want them runnable independently; they don’t need any special code to be launchable from the toolbox. From Python/GTK you’d just use Gio.Subprocess (or subprocess) to exec them by name.


## Getting the “very specific security context”
Here’s the key point: SELinux/MLS context is controlled by policy and login mappings, not really by the .desktop file itself.

On an MLS SELinux system, the usual model is:
* User logs in (GDM, SSH, etc.) at some MLS level/range with a given SELinux role/type.
* Every process they start in that session (shell, GNOME, your toolbox) inherits that SELinux context, unless there is an explicit type_transition rule causing a domain change.
* Any children that your toolbox spawns (sub-tools) inherit that same SELinux context, again unless policy says otherwise.


So, if you want “UMRS Toolbox and all its tools run as umrs_tool_t at level s0:c1,c2” for example, you usually:
* Define a domain/type for the GUI in your SELinux policy, e.g. umrs_tool_t plus an executable type umrs_tool_exec_t.
* Label your launcher and/or main Python entrypoint with that executable type:
  - `semanage fcontext -a -t umrs_tool_exec_t “/usr/local/bin/umrs-toolbox”`
  - `restorecon -v /usr/local/bin/umrs-toolbox`
* Add a type_transition so that when the appropriate SELinux user/role executes umrs_tool_exec_t, the process context becomes umrs_tool_t at the desired MLS range.

Once that is in place:
* GNOME launches /usr/local/bin/umrs-toolbox (labeled umrs_tool_exec_t).
* Policy causes it to run in umrs_tool_t at your chosen MLS level/range.
* Any sub-programs launched from the toolbox (script1.py, script2.py, etc.) are just children of that process, so they inherit umrs_tool_t with the same MLS range, unless you explicitly create a transition to something else.

A couple of practical notes:
* Do not use pkexec/sudo inside the toolbox if you want all tools to keep the exact same context; those would likely move you into a different SELinux domain or UID.
* If you really want to force a process into a specific context manually, you can use runcon in a wrapper, for example:

```bash
#!/bin/bash
exec /usr/sbin/runcon -t umrs_tool_t -r system_r -l s0:c1,c2 /usr/bin/python3 -m umrs.toolbox_main
```
but this is usually less clean than doing the right SELinux policy mapping and login role, and it can conflict with MLS rules depending on your system’s constraints.


## Putting it together for UMRS

A sane UMRS way to do this:
* Decide: “UMRS Toolbox” is the primary GUI front-end at the user’s current MLS level. It should not try to hop contexts; instead, the user logs in at the right level.
* Create: /usr/local/bin/umrs-toolbox (wrapper)
* A SELinux policy module that defines umrs_tool_t and umrs_tool_exec_t, file labels, and type_transition rules so the launcher runs in that domain.
* A .desktop file pointing to umrs-toolbox.
* Write sub-tools as plain GTK4 Python apps; they:
* Can be run standalone (via their own .desktop or CLI).
* Can be launched from the toolbox via Gio.Subprocess.
* Automatically inherit whatever SELinux/MLS context the toolbox has.

