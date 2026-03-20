# Structured Data & Schema.org for Technical Projects

**Source:** Schema.org, Google Search Central
**Date compiled:** 2026-03-20
**Phase:** 2A — SEO & web discovery fundamentals

---

## What Structured Data Does

Structured data is machine-readable markup (JSON-LD) embedded in HTML that tells search engines what a page is about. It enables rich results (knowledge panels, article carousels, breadcrumbs, FAQ accordions) and improves content classification.

Google recommends **JSON-LD** over Microdata or RDFa. JSON-LD lives in a `<script>` tag in the `<head>` — it doesn't mix with HTML content.

---

## Schemas Relevant to UMRS

### Article / BlogPosting

For blog posts on jamieadams-nerd.github.io:

```json
{
  "@context": "https://schema.org",
  "@type": "BlogPosting",
  "headline": "Why CUI Needs Kernel Enforcement",
  "author": {
    "@type": "Person",
    "name": "Jamie Adams",
    "url": "https://jamieadams-nerd.github.io/about"
  },
  "datePublished": "2026-03-19",
  "dateModified": "2026-03-19",
  "publisher": {
    "@type": "Organization",
    "name": "UMRS Project"
  },
  "description": "...",
  "mainEntityOfPage": "https://jamieadams-nerd.github.io/2026/03/19/cui-sign-lock.html"
}
```

### SoftwareSourceCode

For the UMRS project itself:

```json
{
  "@context": "https://schema.org",
  "@type": "SoftwareSourceCode",
  "name": "UMRS",
  "description": "Unclassified MLS Reference System — high-assurance CUI handling on SELinux",
  "codeRepository": "https://github.com/...",
  "programmingLanguage": "Rust",
  "license": "...",
  "author": {
    "@type": "Person",
    "name": "Jamie Adams"
  }
}
```

### TechArticle

For Antora documentation pages (more specific than Article):

```json
{
  "@context": "https://schema.org",
  "@type": "TechArticle",
  "headline": "SELinux MCS Categories in UMRS",
  "proficiencyLevel": "Expert",
  "dependencies": "SELinux, RHEL 10"
}
```

### BreadcrumbList

For documentation navigation — helps Google understand site hierarchy:

```json
{
  "@context": "https://schema.org",
  "@type": "BreadcrumbList",
  "itemListElement": [
    {"@type": "ListItem", "position": 1, "name": "Docs", "item": "https://..."},
    {"@type": "ListItem", "position": 2, "name": "Developer Guide", "item": "https://..."},
    {"@type": "ListItem", "position": 3, "name": "High-Assurance Patterns"}
  ]
}
```

---

## Implementation for UMRS

1. **Jekyll blog**: Add JSON-LD to `_layouts/post.html` template — automated for every post
2. **Antora docs**: Add JSON-LD via supplemental UI templates or Antora extensions
3. **GitHub repo**: GitHub already generates structured data for repositories
4. **Validation**: Use Google's Rich Results Test (search.google.com/test/rich-results)

---

## Actionable Insights for Sage

- Every blog post needs `BlogPosting` JSON-LD with `datePublished`, `author`, and `description`
- Add `SoftwareSourceCode` schema to the project's main page
- Use `TechArticle` with `proficiencyLevel` for documentation aimed at experts
- Breadcrumbs help Google understand doc hierarchy — implement early

## Sources

- [Schema.org](https://schema.org/)
- [Article Structured Data | Google Search Central](https://developers.google.com/search/docs/appearance/structured-data/article)
- [Breadcrumb Structured Data | Google Search Central](https://developers.google.com/search/docs/appearance/structured-data/breadcrumb)
- [Rich Results Test | Google](https://search.google.com/test/rich-results)
