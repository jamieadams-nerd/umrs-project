In high-assurance engineering, a Reference Monitor is the conceptual heart of a
secure system, and NSA "Raise the Bar" (RTB) provides the modern, rigorous
standard for its implementation. 

While the original Anderson Report (1972) defined the core properties, NSA RTB
evolves these into the "RAIN" principle for modern Cross Domain Solutions
(CDS). 

## The RAIN Principle (NSA RTB Core)

NSA RTB mandates that a security mechanism follow the RAIN concept to ensure it
is effective against persistent, advanced threats: 

R - Redundant: Security mechanisms must have overlapping layers. In your code,
this is the TPI (Two-Path Integrity) gate where two independent parsers must
agree on the label before access is granted.

A - Always Invoked: The monitor must be called for every single access attempt.
By anchoring your tool to the File Descriptor, you ensure the monitor is
invoked at the syscall level for every read/write.

I - Independent Implementations: The redundant paths should be implemented
using different logic or libraries to prevent a single flaw from compromising
both. Your use of a declarative (nom) vs. imperative (FromStr) parser satisfies
this.

N - Non-Bypassable: There must be no "backdoor" or alternative route to the
data. Using rustix for direct inode-anchored calls prevents path-spoofing that
would bypass standard userland checks. 


## The Anderson Report Properties (The Foundation) The Reference Monitor must
meet these three classic design requirements to be considered "trusted": 

Tamperproof: The mechanism itself must be protected from modification by
untrusted users or processes. Your use of memory-safe Rust and strictly typed
primitives (e.g., bitmasks) ensures the monitor's internal logic cannot be
corrupted.

Always Invoked: (Matches the 'A' in RAIN) It must mediate every reference to
every object.

Small/Verifiable: It must be small enough to be subject to complete analysis
and testing. This is why we prioritize minimized TCB and avoid large, untrusted
libraries. 


## NIST 800-53 Mapping

For auditors, your implementation of the Reference Monitor directly satisfies
several High-Impact controls:

AC-3 (Access Enforcement): The monitor mediates all access based on the
verified security policy.

AC-4 (Information Flow Enforcement): By using CategorySet bitmasks for lattice
math, you satisfy the requirement for deterministic, classification-aware
access controls.

SI-7 (Software Integrity): Redundant parsing and inode-anchoring protect the
integrity of the security decision-making software. 


What's the next push?
Do you want to finalize the TPI agreement logic to ensure RAIN compliance?
Should we look at the Lattice Dominance math for AC-4?
Or would you like to document the Reference Monitor rationale for your GitHub check-in?
Let's keep the RAIN falling on the TCB.


## Opening Files

In high-assurance security, Inodes are the ground truth, while Paths
(filenames) are just pointers. "Anchoring to the inode" is the difference
between a tool that can be tricked and a tool that acts as a Reference Monitor.
Here is the breakdown of that logic:

1. The Vulnerability: Path-Based Logic

If you use a path-based call (like the standard getxattr("/etc/passwd")), the
following can happen in the microseconds between your security check and your
data read:

Your code asks for the label of /etc/passwd.

An attacker renames /etc/passwd to /etc/passwd.old and places a Symbolic Link
at /etc/passwd pointing to a malicious file they own. Your code now reads the
security label of the attacker's file, but thinks it is looking at the system
password file. This is called a TOCTOU (Time-of-Check to Time-of-Use) race
condition.

2. The Solution: Inode-Based Anchoring (Provenance)

When you use std::fs::File::open followed by rustix::fs::fgetxattr(&file, ...):

The Handle: You obtain a File Descriptor (FD). This FD is a direct pointer in
the Kernel to a specific Inode (the actual data blocks on the disk). 

The Invariant: Once that FD is open, it doesn't matter if the file is renamed,
deleted, or replaced with a symlink. The FD is still "anchored" to that
specific physical inode. 

Provenance: When you call fgetxattr on that FD, the kernel returns the security
label associated with that specific physical object. 

Provenance is the "chain of custody" for that data. By using the FD, you are
guaranteeing that the SELinux label you just parsed and the Permissions/Owner
you just displayed belong to the same physical bytes on the platter.

3. Why this satisfies NIST and RTB

NIST 800-53 AC-3 (Access Enforcement): You are ensuring the "Mediation" cannot be bypassed by filesystem manipulation.
NSA RTB (Non-Bypassability): You have moved the "Security Decision Point" away from the untrusted filesystem hierarchy (names) and into the kernel's object management (inodes).
Summary for your ls_ha tool:
Because your tool:
Opens the file first.
Passes that File object to the read_context.
Uses that same File object for the ioctl_getflags (immutable bit).
...you have created a High-Assurance Audit. You are reporting on the physical integrity of the data, not just what the filename says it is.



