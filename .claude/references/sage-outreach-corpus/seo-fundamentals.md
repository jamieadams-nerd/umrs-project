# SEO Fundamentals

**Source URLs:**
- https://developers.google.com/search/docs/fundamentals/seo-starter-guide
- https://developers.google.com/search/docs/appearance/core-web-vitals
- https://developers.google.com/search/docs/crawling-indexing/overview

**Retrieved:** 2026-03-20

---

## How Search Works: Crawl → Index → Rank

### Crawling
Google's automated crawlers (Googlebot) discover pages by following links and processing sitemaps. Most sites are found automatically without submission. Technical requirements for crawlability:

- Ensure robots.txt does not block CSS or JavaScript resources — Googlebot needs to render pages the same way browsers do
- Use descriptive, clean URLs with relevant keywords in the path
- Submit XML sitemaps to Google Search Console for large or complex sites
- Canonical URLs (`<link rel="canonical">`) prevent duplicate content penalties

The crawler follows links, so internal link structure is critical: pages not linked from anywhere are invisible.

### Indexing
Googlebot renders the page (executing JavaScript) and processes the content for the search index. Key points:

- Content in JavaScript-rendered pages is indexed but may lag behind HTML-rendered content
- Title tags and meta descriptions are extracted and used in search result display
- Heading hierarchy (h1–h6) signals topic structure
- Image alt text is indexed; images without alt text contribute less to topical relevance

### Ranking
Google's ranking systems evaluate hundreds of signals. The official guidance emphasizes:

**Content quality is the dominant factor.** "Creating content that people find compelling and useful will likely influence your website's presence in search results more than any other suggestion."

Other confirmed signals:
- Backlinks from authoritative sources (still significant, not the only factor)
- Page experience signals (Core Web Vitals)
- Topic authority — consistent coverage of a subject area builds domain authority
- User engagement signals — high dwell time, low pogo-sticking (bounce back to search results)

**Confirmed non-factors:** Meta keywords, keyword density, domain keywords, content length in isolation.

---

## Core Web Vitals

Core Web Vitals are measurable performance signals that Google uses as page experience ranking inputs. Three metrics:

### Largest Contentful Paint (LCP)
- **Measures:** Loading performance — how fast the main content appears
- **Good threshold:** < 2.5 seconds
- **Needs improvement:** 2.5–4.0 seconds
- **Poor:** > 4.0 seconds
- **Impact:** Directly affects perceived page speed; slow LCP causes users to leave

### Interaction to Next Paint (INP)
- **Measures:** Responsiveness — how quickly the page responds to user interaction
- **Good threshold:** < 200 milliseconds
- **Needs improvement:** 200–500ms
- **Poor:** > 500ms
- **Replaced FID (First Input Delay) in March 2024**

### Cumulative Layout Shift (CLS)
- **Measures:** Visual stability — unexpected element movement during load
- **Good threshold:** < 0.1 score
- **Needs improvement:** 0.1–0.25
- **Poor:** > 0.25
- **Cause:** Images without dimensions, dynamically injected content above existing content

**Monitoring:** Google Search Console provides a Core Web Vitals report showing page-level performance across both mobile and desktop.

For a static GitHub Pages site like UMRS, LCP and CLS are most controllable. Keep image sizes reasonable, specify image dimensions in HTML, and avoid late-loading layout-shifting elements.

---

## Internal Linking Strategy

Internal links serve two purposes simultaneously:
1. Guide users to relevant related content
2. Signal to Google which pages are related and which are most important

**Principles:**
- Use descriptive anchor text that tells users (and Googlebot) what they'll find — not "click here"
- Link from high-traffic pages toward deeper content to distribute link equity
- Every important page should be reachable within 3 clicks from the homepage
- Blog posts should link to relevant documentation pages — the blog is the discovery layer, docs are the deep reference

**For UMRS:** Every blog post should link 2–5 times to the Antora documentation. Blog posts are the entry point; the link structure pulls readers into the technical reference and signals to Google that blog and docs are part of a unified authority site.

---

## On-Page Technical Requirements

| Element | Requirement | UMRS Context |
|---|---|---|
| Title tag | 50–60 characters, include primary keyword | e.g. "SELinux Security Context Parsing in Rust — UMRS" |
| Meta description | 120–160 characters, describe page value | Shown in search snippets; affects click-through rate |
| Heading hierarchy | One h1 per page, logical h2–h6 structure | Antora generates headings from AsciiDoc; verify output |
| Image alt text | Descriptive text for every meaningful image | Diagrams should have full text descriptions |
| Canonical URL | Self-referential canonical on every page | Prevent duplicate content from URL parameters |

---

## Content Freshness and Update Signals

Search engines favor content that is kept current:
- `dateModified` in structured data signals freshness
- Updating existing posts with new information outperforms publishing new thin posts
- For technical documentation: version-specific content should include version numbers in URL or title

**For UMRS:** When RHEL 10 STIG versions change or NIST publishes revisions, update relevant blog posts and add a changelog note. This signals ongoing maintenance and keeps rankings.

---

## UMRS-Specific SEO Applications

1. **Domain authority building:** Consistent publication under a single domain (GitHub Pages) accumulates authority over time. Cross-posting to dev.to or Lobsters should canonical back to the UMRS domain.

2. **Topical depth:** Publishing multiple posts on SELinux, RHEL 10 hardening, CMMC, and Rust security signals to Google that this site is an authority on these subjects — which improves rankings for all related queries.

3. **Long-tail keyword targeting:** "SELinux security context Rust parsing" is more achievable than "SELinux" alone. Tier 2 engineering posts naturally target long-tail queries.

4. **GitHub Pages limitations:** No server-side rendering, no analytics without JavaScript (affects Core Web Vitals measurement). Static site performance is generally excellent for CWV — advantage for UMRS.

---

## Actionable Insights for Sage

- Every post needs a unique title (50–60 chars), meta description (120–160 chars), and at least one internal link to docs
- Use headings (h2/h3) to break posts into scannable sections — search engines index heading text separately
- Write posts that answer real questions security engineers search for: "how does SELinux MLS work", "rust selinux library", "RHEL 10 FIPS compliance"
- Core Web Vitals on static GitHub Pages should be excellent by default — monitor in Search Console after launch
- Structured data (Article schema with JSON-LD) should be added to every blog post — see structured-data.md
