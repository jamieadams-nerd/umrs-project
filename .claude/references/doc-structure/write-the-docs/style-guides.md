# Style Guides — Write the Docs

Source: https://www.writethedocs.org/guide/writing/style-guides/
Retrieved: 2026-03-12

## Overview

A style guide establishes standards for writing and designing content. It maintains consistency across documentation—whether created by a single writer or large teams. Style guides serve as reference materials for discussing common topics and features, applying to various formats like API references, tutorials, release notes, and technical overviews.

Consistent tone and style improves readability, reduces cognitive load for users, and builds confidence in content authority.

## Creating Your Own Style Guide

Style guides range from simple decision lists about terminology to comprehensive institutional standards. Understanding what matters in established guides helps inform your own.

## Traditional Style Guide Resources

Established references:

- The AP Stylebook
- The Chicago Manual of Style

Additional writing resources:

- *The Sense of Style* by Steven Pinker
- *Stylish Academic Writing*

## Sample Writing Guides

### Software Documentation Classics

- Apple Style Guide
- Microsoft Writing Style Guide

### Enterprise Software Style Guides

- Google developer documentation
- The Red Hat Style Guide
- Salesforce
- Rackspace
- Mailchimp

### Government and Education Guides

- 18F Content Guide
- Writing for gov.uk
- Princeton Editorial Style Guide
- Federal Plain Language Style Guide

### Open Source Project Guides

- Open SUSE Style Guide
- GNOME Style Guide
- Write the Docs Official Website Style Guide

## Developer Documentation and APIs

### Code Sample Guidelines

- API reference code comments – Google developer documentation style guide
- REST API Documentation Best Practices

### Command Line Resources

- CLI Style Guide – Heroku Dev Center
- Command Line Interface Guidelines – CLIG
- Conventions for writing Linux man pages

### API Documentation

Clear, well-formatted API documentation enables developers to quickly implement and troubleshoot. "If a feature is not documented, it does not exist. If documented incorrectly, it is broken."

The best API documentation results from concurrent API and documentation development. Documentation cannot compensate for poor API design.

#### Documentation-Driven Design

When APIs aren't yet built, documentation supports the design process: "Documentation changes are cheap. Code changes are expensive."

Machine-readable API documentation formats enable:

- Mock servers for initial design
- Pre-deployment testing ensuring API-documentation alignment
- Interactive documentation allowing demo requests

## Content Guidelines

### Release Notes

Release notes provide vital information for continued product use. Content addresses new or updated features, remaining brief with links to details.

Consider these elements:

1. What is the specific change?
2. Why was this change made?
3. What is the user goal with this feature?
4. Do users have all necessary information?
5. Are additional articles available? Link them.
6. Would images benefit user understanding?
7. What stakeholder approval is required?

### Error Messages

Software errors are inevitable. Characteristics of good error messages:

- Provide explicit indication that something failed
- Write conversationally, not mechanically
- Avoid blaming users—remain humble
- Keep messages short and meaningful
- Include precise problem descriptions
- Offer constructive fix suggestions

## Accessibility and Bias Considerations

Your style guide should address accessibility and bias to ensure all readers understand your content.

### Avoiding Animal-Violence Idioms

Figurative idioms referencing animal violence confuse non-native English speakers. Clear alternatives include:

| Instead of | Use |
|---|---|
| "kill two birds with one stone" | "accomplish two things at once" |
| "beat a dead horse" | "belabor the point" |

**Note:** Established technical terms (like Unix `kill`, "canary deployment," "monkey-patching") retain their meanings and shouldn't be replaced.
