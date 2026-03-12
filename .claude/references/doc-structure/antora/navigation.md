# Antora Navigation

Source: https://docs.antora.org/antora/latest/navigation/
Retrieved: 2026-03-12

## Overview

Antora enables documentation teams to create component version navigation using AsciiDoc files stored alongside source content. The navigation system assembles multiple files into a hierarchical menu structure displayed on the published site.

## Navigation Assembly Requirements

A functional navigation system requires three essential components:

1. **Navigation Files**: At least one AsciiDoc file containing unordered lists
2. **Registration**: Files must be registered in the component version descriptor
3. **UI Bundle**: A theme package that applies styling and interactive behavior

## Navigation Files (Source)

Navigation files function as specialized AsciiDoc documents containing exclusively bulleted lists at any nesting depth. Key characteristics include:

- Files may contain one or more unordered lists
- List items can reference pages, external URLs, or serve as category headers
- Multiple files can be composed together for modular organization
- Creating one navigation file per module is common but optional

## Navigation Registration

Navigation files connect to the publishing system through the component version descriptor file's `nav` key. The registration order determines how files appear in the final menu. Only registered navigation files are processed during site generation.

## Generated Navigation Elements

### Component Version Page Menu
Antora aggregates registered navigation file contents and publishes them in the specified order, creating the component version's page menu within the site.

### Component Version Selector
The system automatically generates a dropdown listing all components and versions in alphabetical order, sorted from latest to earliest.

### Page Version Selector
When pages exist across multiple component versions, Antora automatically creates a version selector dropdown for cross-version navigation.
