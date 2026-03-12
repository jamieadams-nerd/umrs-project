# Organizing Content Files in Antora

Source: https://docs.antora.org/antora/latest/organize-content-files/
Retrieved: 2026-03-12

## Overview

Antora uses both convention and configuration to aggregate content and generate documentation sites.

## Storing Content Source Files

Antora retrieves content from multiple git repositories by searching for files under a start path or multiple start paths in branches, tags, or local worktrees.

Key requirements for documentation files:

- **Location**: Must be placed "under a content source root"
- **Descriptor**: Requires a file named `antora.yml`
- **Structure**: Must follow "a standard set of directories"

An important best practice: **always use lowercase for filenames**. Case sensitivity varies across filesystems and web servers, so lowercase naming prevents compatibility issues.

## Classifying Content Source Files

Once collected, Antora assigns metadata to each file to create a unique identifier called a resource ID. This metadata enables file references and configuration associations.

### Virtual Filesystem Concept

Antora creates a virtual filesystem (VFS) after collecting files. The source file's original storage location becomes irrelevant after initial discovery. All subsequent file operations reference this VFS rather than the original repositories.

The only connection maintained to physical storage is the "family-relative path," kept "merely as a convenience for the author." Other identity aspects derive from metadata like component name, version, module name, and family.

### File Metadata Assignment

Files inherit component name and version from the `antora.yml` descriptor in their content source root. This file helps organize collected materials into component versions—essentially all documentation for a specific project version.

Within each content source root, files are organized into module and family folders, providing additional identity dimensions. The family-relative path ensures unique identification across multiple repositories.

### File Location vs. URL Mapping

A crucial distinction: source file location doesn't determine published file location. Once loaded into the VFS, file metadata—including output location and URL—gets computed. Each file family has different URL computation rules, so source location and published location are independent.

## Git Refname Considerations

Branch and tag names should avoid characters requiring URL encoding to prevent subtle Antora problems.

**Critical issue**: Characters like `#` create problems because browsers interpret `#` as the URL-fragment boundary, resulting in incomplete URLs and 404 errors.

**Forward slashes**: Using `/` in refnames (like `r/3.0.x`) effectively creates folder structures but affects refname matching. The wildcard `*` doesn't cross folder boundaries, so matching `r/3.0.x` requires the pattern `r/*` rather than `r*`.
