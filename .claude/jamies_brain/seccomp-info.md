3. Assign the Skill to the Security Engineer Agent

Edit:

.claude/agents/security-engineer.md

Add the new skill:

---
name: security-engineer
description: SELinux policy engineer and Linux security architecture specialist
tools: filesystem

skills:
  - selinux-policy-engineering
  - seccomp-engineering
  - dac-filesystem-security
---

Now your security-engineer agent understands three security layers:

DAC
SELinux
seccomp


⸻

4. Populate the Seccomp Reference Corpus

Create:

.claude/skills/seccomp-engineering/refs/

Good material to place there:

seccomp-overview.txt
libseccomp-reference.txt
kernel-seccomp-doc.txt
docker-default-seccomp.json

Kernel documentation sources are excellent.

For example:

Documentation/userspace-api/seccomp_filter.rst


⸻

5. Example Use Case the Agent Can Handle

After adding this skill you can ask the agent:

Design a seccomp policy for a Rust network daemon.

Expected reasoning:

process purpose
required syscalls
dangerous syscalls removed
policy example
compatibility notes


⸻

6. Why This Skill Is Very Valuable

Your current security model becomes:

DAC        → discretionary permissions
SELinux    → object access control
seccomp    → syscall attack surface

This matches modern hardened systems and containers.

Even highly hardened platforms like:
	•	Kubernetes runtime
	•	Chrome sandbox
	•	systemd service sandboxing

use seccomp.

⸻

7. Later Enhancement (Worth Doing)

Eventually you may want a second seccomp skill:

seccomp-auditing

That agent would:
	•	analyze syscall profiles
	•	detect over-permissive rules
	•	detect bypass possibilities

That mirrors how you separated SELinux engineering vs auditing.

⸻

8. Important Note for Your Environment

Because you are working heavily with:
	•	SELinux targeted policy now
	•	MLS later

Seccomp will remain policy-independent.

It constrains the kernel interface, not security labels.

So it fits nicely into both policy models.

⸻

If you’d like, the next thing I can show you (very useful for your security agents) is a Linux syscall risk classification table — roughly 40–50 syscalls that security engineers treat as high-risk.

Giving that to the seccomp skill dramatically improves its ability to design tight syscall allowlists.

Good question — those names I listed were examples of the types of reference material, not literal files you must write from scratch. In practice you should pull authoritative documentation and convert it into text files for the agent’s reference corpus.

For seccomp, the best sources are actually already present in Linux documentation and a few well-maintained projects.

Below are the real sources and exactly how to obtain them.

⸻

1. Linux Kernel Seccomp Documentation (Best Source)

The Linux kernel itself contains the authoritative documentation.

Relevant file:

Documentation/userspace-api/seccomp_filter.rst

This explains:
	•	seccomp architecture
	•	BPF filters
	•	filter semantics
	•	syscall argument filtering
	•	return actions

The kernel project is maintained by the Linux Kernel Organization.

How to obtain it

If you already have the kernel source installed:

cd /usr/src/linux

or clone it:

git clone https://github.com/torvalds/linux.git

Then copy the documentation:

cp Documentation/userspace-api/seccomp_filter.rst \
.claude/skills/seccomp-engineering/refs/kernel-seccomp-filter.rst


⸻

2. libseccomp Documentation

Most real systems implement seccomp policies using libseccomp.

It provides a user-friendly API for building syscall filters.

You can obtain documentation directly from the project.

Clone the repository

git clone https://github.com/seccomp/libseccomp.git

Then copy useful documentation:

cp README.md \
.claude/skills/seccomp-engineering/refs/libseccomp-readme.md

You may also copy:

doc/

directory contents.

⸻

3. Docker Default Seccomp Profile

A very practical real-world example comes from Docker.

Docker ships a production seccomp profile.

File:

default.json

You can download it:

curl https://raw.githubusercontent.com/moby/moby/master/profiles/seccomp/default.json \
-o .claude/skills/seccomp-engineering/refs/docker-default-seccomp.json

This shows:
	•	real syscall allowlists
	•	argument filtering
	•	blocked high-risk syscalls

Great training material for the agent.

⸻

4. Man Pages

These are extremely useful for agents.

Export them to text:

man seccomp > .claude/skills/seccomp-engineering/refs/man_seccomp.txt
man prctl > .claude/skills/seccomp-engineering/refs/man_prctl.txt
man seccomp_unotify > .claude/skills/seccomp-engineering/refs/man_seccomp_unotify.txt

These cover:
	•	enabling seccomp
	•	filter modes
	•	user notifications

⸻

5. Syscall Lists

The agent benefits from understanding Linux syscalls.

Export syscall tables:

ausyscall --dump > .claude/skills/seccomp-engineering/refs/syscall_table.txt

This gives the agent the full syscall list.

⸻

6. Optional: Kernel Security Documentation

The kernel security docs are excellent general references.

From the kernel source:

Documentation/security/

You can copy useful files into:

.claude/references/kernel-security/


⸻

7. What the Final Directory Should Look Like

After collecting material:

.claude/skills/seccomp-engineering/refs/
├── kernel-seccomp-filter.rst
├── libseccomp-readme.md
├── docker-default-seccomp.json
├── man_seccomp.txt
├── man_prctl.txt
├── man_seccomp_unotify.txt
└── syscall_table.txt

These become the reference corpus for the seccomp skill.

⸻

8. Why This Works Well for an Agent

Claude agents work best when given:
	•	authoritative sources
	•	real configuration examples
	•	API documentation

Your corpus now contains:

kernel design documentation
library documentation
production policy examples
syscall lists

That gives the agent enough context to reason about seccomp policies properly.

⸻

9. Small Tip for Your Security Engineer Agent

When the agent designs a seccomp profile, encourage this workflow:

process purpose
↓
required syscalls
↓
allowlist filter
↓
deny everything else

This aligns with modern hardened system design.

⸻

If you want, the next upgrade for your seccomp skill would be something extremely useful for security engineers:

a curated “dangerous syscall list” used by sandbox designers (roughly 35–40 syscalls).

Giving that to the agent dramatically improves its ability to generate tight syscall policies.
