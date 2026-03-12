# Antora Playbook Configuration

Source: https://docs.antora.org/antora/latest/playbook/ and https://docs.antora.org/antora/latest/playbook/set-up-playbook/
Retrieved: 2026-03-12

## What Is an Antora Playbook?

An Antora playbook is a configuration file that enables technical writers to manage documentation site generation. The playbook serves as a central control point, allowing users to "control what content is included in your site, what user interface (UI) is applied to it, and where the site is published."

The typical filename is `antora-playbook.yml`.

### Key Responsibilities

The playbook file specifies several critical aspects:

- **Site Configuration**: Global settings including title and URL
- **Content Sources**: Which repositories, branches, and tags to include
- **Processing Rules**: AsciiDoc attributes and Asciidoctor extensions applied site-wide
- **UI Bundle Selection**: Visual design and layout preferences
- **Publication Targets**: Output format and destination details
- **Runtime Behavior**: Cache management and repository update handling

Playbook settings can be "overridden using CLI options or environment variables," providing flexibility for different deployment environments.

## Playbook Storage

Playbooks reside in dedicated playbook projects—repositories focused entirely on configuration. These repositories are "'configuration as code' repositories—they do not contain any content."

## Supported File Formats

Antora accepts playbooks in three formats:

- **YAML** (most common and documented)
- **JSON**
- **TOML**

## Configure Site Properties

```yaml
site:
  title: My Demo Site
  url: https://docs.demo.com
  start_page: component-b::index.adoc
```

"Assigning an absolute URL to the url key activates secondary features such as the sitemap."

## Configure Content Sources

```yaml
content:
  sources:
  - url: https://gitlab.com/antora/demo/demo-component-a.git
  - url: https://gitlab.com/antora/demo/demo-component-b.git
    branches: [v2.0, v1.0]
    start_path: docs
```

"The default branches filter is applied at runtime when a url key doesn't have a branches or tags key set on it."

If your content source root isn't at the repository root, specify its location:

```yaml
    start_path: docs
```

"Don't add leading or trailing slashes to the path."

## Configure UI Bundle

```yaml
ui:
  bundle:
    url: https://gitlab.com/antora/antora-ui-default/-/jobs/artifacts/HEAD/raw/build/ui-bundle.zip?job=bundle-stable
    snapshot: true
```

The `snapshot: true` setting ensures Antora downloads the UI bundle whenever fetch is activated.

## Complete Example

```yaml
site:
  title: My Demo Site
  url: https://docs.demo.com
  start_page: component-b::index.adoc
content:
  sources:
  - url: https://gitlab.com/antora/demo/demo-component-a.git
  - url: https://gitlab.com/antora/demo/demo-component-b.git
    branches: [v2.0, v1.0]
    start_path: docs
ui:
  bundle:
    url: https://gitlab.com/antora/antora-ui-default/-/jobs/artifacts/HEAD/raw/build/ui-bundle.zip?job=bundle-stable
    snapshot: true
```

## Path Resolution Best Practices

The documentation emphasizes portable playbook design. Paths prefixed with `./` resolve relative to the playbook file location, enabling consistent behavior regardless of the working directory.
