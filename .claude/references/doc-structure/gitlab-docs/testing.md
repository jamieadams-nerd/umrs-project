# Documentation Testing at GitLab

Source: https://docs.gitlab.com/development/documentation/testing/
Retrieved: 2026-03-12

## Overview

GitLab treats documentation like code, storing it in projects alongside source files and applying similar quality assurance processes. Documentation changes undergo automated testing through multiple CI/CD jobs to maintain standards.

## CI/CD Testing Jobs

When merge requests modify Markdown (`.md`) files, the following jobs execute:

### Core Linting Jobs

**`docs-lint markdown`** runs multiple test types:
- **Vale**: Checks documentation content and style
- **markdownlint**: Validates Markdown structure
- **`lint-docs.sh` script**: Runs miscellaneous tests for specific documentation requirements

**`docs-lint links`**: Validates relative link integrity across the documentation suite

**`docs-lint mermaid`**: Uses `mermaidlint` to check Mermaid diagram syntax

**`rubocop-docs`**: Checks documentation links from `.rb` files

**`eslint-docs`**: Checks documentation links from `.js` and `.vue` files

**`docs-lint redirects`**: Ensures deleted or renamed files have proper redirects

**`docs hugo_build`**: Performs partial Hugo website builds to validate shortcodes and filenames

### Generated File Verification Jobs

**`graphql-verify`**: Fails if GraphQL documentation isn't updated through proper processes

**`docs-lint deprecations-and-removals`**: Fails when deprecations documentation isn't properly updated

## Tests in `lint-doc.sh`

The `lint-doc.sh` script identifies issues that Vale and markdownlint cannot detect:

- Curl commands must use long-form options (e.g., `--header` instead of `-h`)
- Documentation pages require front matter indicating ownership
- CHANGELOG.md cannot contain duplicate version entries
- Files in `doc/` must not have executable permissions
- Filenames and directories must use `_index.md`, underscores, and lowercase characters
- Image filenames must specify their introduction version
- Mermaid charts must render without errors

## Internationalization Testing

Quality assurance extends to translated documentation.

### General Translation Linting

The `docs-i18n-lint markdown` job runs on all translated content:
- **markdownlint**: Checks structure and formatting
- **Vale**: Applies general style rules

### Language-Specific Testing

For languages with unique requirements, dedicated CI/CD jobs run language-specific Vale rules.

## Local Testing

### Running Tests Locally

**Using Rake for `lint-doc.sh` tests**:

```bash
cd gitlab
rake lint:markdown
```

Or for specific files:

```bash
MD_DOC_PATH=path/to/my_doc.md rake lint:markdown
```

## Configuration Management

Vale and markdownlint configurations are version-controlled in each project. The GitLab project serves as the source of truth, with regular synchronization to other projects.

## Tool Versions

Match linter versions to CI/CD pipeline versions for maximum compatibility:

| Tool | Version | Command |
|------|---------|---------|
| `markdownlint-cli2` (latest with yarn) | Latest | `yarn global add markdownlint-cli2` |
| Vale (specific with mise) | Specific | `mise install` |

## Pre-push Hooks

Lefthook manages Git hooks for the GitLab project. Configuration exists in `lefthook.yml` to run tests before pushing branches, helping catch issues early.
