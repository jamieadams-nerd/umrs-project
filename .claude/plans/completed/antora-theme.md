## Status: Completed (2026-03-11)

# UMRS (Antora) Documentation Theme Guidelines

Documentation should look like infrastructure, not marketing.

## Core Principle

Apply minimal, deliberate branding — enough to signal ownership and polish, not enough to distract from technical content. The goal is **engineering seriousness**.

## What to Do

Three changes only:

1. **Subtle color palette**
2. **Simple logo in header**
3. **Typography adjustments (optional)**

### Recommended Color Scheme

| Element        | Value     |
|----------------|-----------|
| Background     | `#ffffff` |
| Sidebar        | `#f6f8fa` |
| Primary accent | `#1f3a5f` |
| Text           | `#1f2933` |
| Code block bg  | `#f6f8fa` |
| Code border    | `#e1e4e8` |

The steel-blue accent (`#1f3a5f`) communicates precision, seriousness, and security — appropriate for a high-assurance project.

### Logo Placement

```
[ UMRS logo ]   UMRS Documentation
```
- Primary logo is in docs/images/urs-logo-1024px.png
- Let me know it conversion tools are needed to be installed for size or background. 
- Logo height: 24–32px
- No splash screens, no banners
- Another good logo is wizard-green-100px.png. 
```

### Navigation Structure

```
UMRS
 ├─ Admin Guide
 ├─ Security Guide
 ├─ Architecture
 ├─ Reference
 └─ API
```

### Footer

```
© UMRS Project
```

## What to Avoid

| Practice              | Reason                  |
|-----------------------|-------------------------|
| Bright colors         | Looks promotional       |
| Gradients             | Visual noise            |
| Giant logos           | Marketing feel          |
| Background images     | Reduce readability      |
| Decorative icons      | Clutter                 |
| Heavy CSS themes      | Maintenance burden      |

## Implementation

Style only three elements; leave everything else at Antora defaults:

1. Headings
2. Links
3. Sidebar

This keeps the UI stable across Antora upgrades.

Total customization: **~30 lines of CSS** in `ui/src/css/custom.css`.

## Reference Projects

Projects that follow this model and are worth studying:

- Kubernetes
- Rust
- PostgreSQL
- Red Hat Enterprise Linux
- HashiCorp Terraform
- Docker

