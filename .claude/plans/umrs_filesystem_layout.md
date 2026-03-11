# UMRS Filesystem Layout Standard

This document defines the canonical filesystem layout for the UMRS platform.
It aligns with established Linux standards while preserving the architectural
separation required for a high-assurance system.

The design follows conventions derived from:

- Filesystem Hierarchy Standard (FHS)
- POSIX system layout expectations
- modern Linux distribution practices

The objective is to ensure:

- clear separation of configuration, code, and data
- deterministic system structure
- predictable administration and auditing
- compatibility with standard Linux tooling


—

# Design Principles

The UMRS directory layout follows several high-assurance principles.

1. Configuration, code, and runtime state must never be mixed.
2. Administrator-managed files must remain separate from application-managed data.
3. Runtime state must be ephemeral and automatically cleared on reboot.
4. Persistent data must reside in directories intended for that purpose.
5. Static resources must be read-only and distributed with the software package.

The canonical separation is therefore:

configuration → /etc
software → /usr
persistent state → /var
runtime state → /run


—

# System Configuration

Configuration files for UMRS reside in:

 §/etc/umrs/ §

This directory contains administrator-managed configuration.

Examples include:

 §/etc/umrs/umrs.conf §/etc/umrs/policy/ /etc/umrs/labels/ /etc/umrs/vaults/ §

Rules for this directory:

- Files are text-based and human editable.
- Files are never modified automatically by the software.
- Changes are expected to be managed by administrators or configuration tools.


—

# Executable Programs

User-facing executables are installed in:
```
/usr/bin/ 
```

Examples:
```
/usr/bin/umrs 
/usr/bin/umrs-vault 
/usr/bin/umrs-label 
/usr/bin/umrs-ls 
/usr/bin/umrs-audit 
/usr/bin/umrs-state 
```


These commands provide the primary interface for operators and administrators.


—

# Internal Program Components

Internal UMRS components are installed in:
```
/usr/lib/umrs/ 
```

These are not intended to be invoked directly by users.

Examples include:
```
/usr/lib/umrs/mediator/ 
/usr/lib/umrs/guards/ 
/usr/lib/umrs/plugins/ 
```

These components may include:

- helper binaries
- plugins
- service modules


—

# Static Architecture Resources

Read-only resources distributed with the software reside in:

/usr/share/umrs/

Examples:
```
/usr/share/umrs/schemas/ 
/usr/share/umrs/catalogs/ 
/usr/share/umrs/templates/ 
/usr/share/umrs/i18n/ 
```

This directory may contain:

- JSON schemas
- taxonomy catalogs
- configuration templates
- localization resources

These files are considered static resources and should never be modified at runtime.


—

# Persistent System State

Persistent runtime data managed by the system resides in:
```
/var/lib/umrs/ 
```


Examples include:
```
/var/lib/umrs/index/ 
/var/lib/umrs/registry/*.json
/var/lib/umrs/state/ 
```

Characteristics of this directory:

- managed automatically by the software
- survives reboot
- typically not edited directly by administrators


—

# Logging

Log files reside in:
```
/var/log/umrs/
```

Example log files:
```
/var/log/umrs/mediator.log 
/var/log/umrs/security.log 
```

In many environments logging may also be handled through the system journal.


—

# Runtime State

Ephemeral runtime files are stored in:
```
/run/umrs/ 
```

Examples:
```
/run/umrs/daemon.pid 
/run/umrs/control.sock 
```

Characteristics:

- cleared automatically at reboot
- contains sockets, lock files, and runtime metadata


—

# Optional Local Extensions

Local site extensions may be installed under:
```
/usr/local/share/umrs/ 
/usr/local/lib/umrs/ 
```

This allows administrators to add local modules without modifying the packaged system files.


—

# Vault Data Domains

UMRS vault storage domains reside outside the standard system hierarchy. They just need to be
isolated and open for discussion.

Example vault roots:

/vaults-lei/ 
/vaults-agr/ 
/vaults-intel/


Each vault follows the lifecycle model:
```
intake → stage → archive 
```

Example:
```
/vaults-lei/intake/ 
/vaults-lei/stage/ 
/vaults-lei/archive/
```

These directories represent controlled data domains rather than application configuration or system software.


—

# Naming Convention

UMRS components follow a consistent naming pattern:
```
umrs-<component>
```

Examples:
```
umrs-audit 
umrs-vault 
umrs-label 
umrs-registry 
```

This naming strategy prevents collisions with other system utilities.


—

# Summary

The UMRS platform uses the following canonical filesystem layout:
```
/etc/umrs 
/usr/bin/umrs 
/usr/lib/umrs 
/usr/share/umrs 
/var/lib/umrs 
/var/log/umrs 
/run/umrs 
```

With vault data domains located at:

/vaults-* 

This structure maintains clear separation between configuration, software, runtime state, and
persistent data while remaining compatible with established Linux filesystem conventions.

This structure also allows us to create proper SELinux security policy and use the file context
definitions so the restorecond can automaticallly label items. The ingest portions of the vault
allows us to automically label files properly based on the locaiton they are dropped.



