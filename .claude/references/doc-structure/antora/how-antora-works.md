# How Antora Works — Build Pipeline

Source: https://docs.antora.org/antora/latest/how-antora-works/
Retrieved: 2026-03-12

## Overview

Antora functions as a site generator that manages the complete documentation creation workflow. "Antora's site generator handles all aspects of creating a documentation site, from fetching and aggregating to converting and arranging the content to publishing the files of the generated site."

The system employs a modular architecture permitting integration of specialized components like validators at strategic pipeline points for customized use cases.

## Default Generator Pipeline Steps

### 1. Build Playbook
The process begins when you provide Antora with a configuration file. This playbook can be authored in YAML, JSON, or TOML format and specifies content sources, processing rules, site generation settings, and publication destinations.

### 2. Load Content Repositories
Antora retrieves the git repositories or local folders identified in the playbook's content sources. For remote URLs, the system clones repositories locally using a built-in git client, fetching updates when configured. The generator then determines which branches, tags, and start paths to utilize.

### 3. Find Content Source Roots
When Antora locates an _antora.yml_ file (the component version descriptor), it recognizes the surrounding files as part of a component version. Content files can reside in a single repository, at repository roots, in subpaths, or distributed across multiple repositories.

### 4. Transform Input Files
Antora collects all materials—text, images, samples, and supporting resources—from each content source root and converts them into virtual file objects.

### 5. Assign Files to Component Buckets
The system reads _antora.yml_ descriptor files, associating files with specific component names and versions. This decoupling enables the "URL agnostic cross references" feature Antora provides.

### 6. Compute Metadata
Using playbook site properties and source metadata, Antora calculates module information, family data, paths, and crucially, the output paths and URLs for each publishable file.

### 7. Organize into Content Catalog
Files get sorted into a queryable and transmittable content catalog structure.

### 8. Convert AsciiDoc to HTML
Page family AsciiDoc files are processed into embeddable HTML using Asciidoctor.js.

### 9. Convert Navigation Files
Navigation files transform into hierarchical navigation items organized within menus and trees.

### 10. Fetch UI Bundle
Antora retrieves the UI bundle specified in the playbook, which can be locally cached or remote.

### 11. Transform UI Files
UI bundle files are extracted and converted into virtual file objects with content and path information.

### 12. Classify UI Files
Using the _ui.yml_ descriptor, static files are identified and classified. Other files receive type assignments based on their location: asset, layout, helper, or partial.

### 13. Compute UI Output Paths
Publishable UI files (static or asset types) receive calculated output paths.

### 14. Organize UI Files
UI files are collected into a transmittable catalog.

### 15. Wrap Content in Templates
Antora selects appropriate page templates for each page, populating them with embeddable HTML, site metadata, context data for version selectors, and navigation information for menus and breadcrumbs.

### 16. Produce Sitemap
A component-partitioned sitemap is generated, with a sitemap index linking to individual component sitemaps containing page URLs.

### 17. Publish Site
The generated site files are written to specified locations using default or custom destination providers supporting multiple formats, locations, and protocols.

## Custom Generators

Users can replace the default generator by creating a library or script exporting a `generateSite(args, env)` function. This custom pipeline can import core Antora components and reassemble them as needed, activated via the `--generator` CLI option.

## Asynchronous Operations

The pipeline executes certain operations concurrently to optimize performance. For instance, "the UI is fetched and loaded while the content is being aggregated to optimize use of the network," meaning steps may occur simultaneously or in varying orders.
