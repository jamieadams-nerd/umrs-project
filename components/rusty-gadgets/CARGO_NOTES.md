



## Workspaces A orkspace is a collection of one or more packages, called
workspace members, that are managed together.

The key points of workspaces are:

Common commands can run across all workspace members, like cargo check
--workspace. All packages share a common Cargo.lock file which resides in the
workspace root. All packages share a common output directory, which defaults to
a directory named target in the workspace root. Sharing package metadata, like
with workspace.package. The [patch], [replace] and [profile.*] sections in
Cargo.toml are only recognized in the root manifest, and ignored in member
crates’ manifests. The root Cargo.toml of a workspace supports the following
sections:

[workspace] — Defines a workspace.
    resolver — Sets the dependency resolver to use.
    members — Packages to include in the workspace.
    exclude — Packages to exclude from the workspace.
    default-members — Packages to operate on when a specific package wasn’t selected.
    package — Keys for inheriting in packages.
    dependencies — Keys for inheriting in package dependencies.
    lints — Keys for inheriting in package lints.
    metadata — Extra settings for external tools.

[patch] — Override dependencies.
[replace] — Override dependencies (deprecated).
[profile] — Compiler settings and optimizations.


##  Manifest (Cargo.toml)

The Cargo.toml file for each package is called its manifest. It is written in
the TOML format. It contains metadata that is needed to compile the package.
Checkout the cargo locate-project section for more detail on how cargo finds
the manifest file.

Every manifest file consists of the following sections:

cargo-features — Unstable, nightly-only features.
[package] — Defines a package.
    name — The name of the package.
    version — The version of the package.
    authors — The authors of the package.
    edition — The Rust edition.
    rust-version — The minimal supported Rust version.
    description — A description of the package.
    documentation — URL of the package documentation.
    readme — Path to the package’s README file.
    homepage — URL of the package homepage.
    repository — URL of the package source repository.
    license — The package license.
    license-file — Path to the text of the license.
    keywords — Keywords for the package.
    categories — Categories of the package.
    workspace — Path to the workspace for the package.
    build — Path to the package build script.
    links — Name of the native library the package links with.
    exclude — Files to exclude when publishing.
    include — Files to include when publishing.
    publish — Can be used to prevent publishing the package.
    metadata — Extra settings for external tools.
    default-run — The default binary to run by cargo run.
    autolib — Disables library auto discovery.
    autobins — Disables binary auto discovery.
    autoexamples — Disables example auto discovery.
    autotests — Disables test auto discovery.
    autobenches — Disables bench auto discovery.
    resolver — Sets the dependency resolver to use.


Target tables: (see configuration for settings)
    [lib] — Library target settings.
    [[bin]] — Binary target settings.
    [[example]] — Example target settings.
    [[test]] — Test target settings.
    [[bench]] — Benchmark target settings.

Dependency tables:
    [dependencies] — Package library dependencies.
    [dev-dependencies] — Dependencies for examples, tests, and benchmarks.
    [build-dependencies] — Dependencies for build scripts.
    [target] — Platform-specific dependencies.
    [badges] — Badges to display on a registry.
    [features] — Conditional compilation features.
    [lints] — Configure linters for this package.
    [hints] — Provide hints for compiling this package.
    [patch] — Override dependencies.
    [replace] — Override dependencies (deprecated).
    [profile] — Compiler settings and optimizations.



