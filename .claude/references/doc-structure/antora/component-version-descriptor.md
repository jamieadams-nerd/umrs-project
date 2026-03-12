# Antora Component Version Descriptor (antora.yml)

Source: https://docs.antora.org/antora/latest/component-version-descriptor/
Retrieved: 2026-03-12

## Overview

The `antora.yml` file serves two essential functions in Antora documentation projects:

1. **Signals Content Location**: Its presence at a content source root indicates that Antora should locate a sibling `modules` directory containing component version source files.

2. **Provides Metadata**: It supplies required and optional component version configuration that Antora applies to all collected source files.

## Purpose and Location

Antora searches for `antora.yml` at each content source root. When discovered, it looks for a neighboring `modules` directory and processes its subdirectories and files as part of a component version.

**Key benefit**: "The _antora.yml_ files provide the ability to store your content source files anywhere in a directory hierarchy." This allows documentation to coexist with application code, tests, and other repository materials.

Without `antora.yml`, Antora skips processing that location entirely, conserving resources.

## Metadata Functions

The `antora.yml` file functions as a **component version descriptor**, providing information that identifies, describes, and configures a component version. This metadata applies to all source files within that version.

This architecture allows Antora to collect files from multiple repositories, directories, branches, and tags, compiling them into a single component version based on descriptor metadata rather than file system location.

## Required Configuration Keys

| Key | Purpose |
|-----|---------|
| **name** | Component identifier used with version to identify the component version; appears in resource IDs and URL segments |
| **version** | Version identifier used with name; appears in resource IDs and resource URLs (unless the version is unversioned) |

The `version` key can be optional if inherited from content source configuration in the playbook.

## Optional Configuration Keys

| Key | Purpose |
|-----|---------|
| **asciidoc.attributes** | AsciiDoc and page attributes applied to all pages and resources in the component version |
| **display_version** | Presentation-only version identifier for UI selectors; supports spaces and special characters |
| **nav** | Lists navigation files used in the component version's page menu |
| **prerelease** | Designates a version as prerelease; can append identifiers like `-alpha.2` |
| **start_page** | Specifies the component version's home page (defaults to `index.adoc` in ROOT module) |
| **title** | Display name for sorting and presentation in UI menus and breadcrumbs; supports spaces and special characters |
