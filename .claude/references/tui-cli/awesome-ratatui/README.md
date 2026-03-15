<!--
source: https://raw.githubusercontent.com/ratatui/awesome-ratatui/main/README.md
fetched: 2026-03-15
-->
<!--lint disable awesome-git-repo-age-->

# Awesome Ratatui [![Awesome](https://awesome.re/badge-flat2.svg)](https://awesome.re)

[<img src="https://github.com/ratatui.png" align="right" width="100">](https://ratatui.rs)

Here you will find a list of TUI crates and applications that are made for or using [`ratatui`](https://crates.io/crates/ratatui) and [`tui`](https://crates.io/crates/tui).

<!--lint disable awesome-toc-->

## Contents

- [📦 Libraries](#-libraries)
  - [🏗️ Frameworks](#%EF%B8%8F-frameworks)
  - [🧩 Widgets](#-widgets)
  - [🔧 Utilities](#-utilities)
- [💻 Apps](#-apps)
  - [⌨️ Development Tools](#%EF%B8%8F-development-tools)
  - [🕹️ Games and Entertainment](#%EF%B8%8F-games-and-entertainment)
  - [🚀 Productivity and Utilities](#-productivity-and-utilities)
  - [🎼 Music and Media](#-music-and-media)
  - [🌐 Networking and Internet](#-networking-and-internet)
  - [👨‍💻 System Administration](#-system-administration)
  - [📟 Embedded](#-embedded)
  - [🌌 Other](#-other)

Aside from those listed here, many other apps and libraries can be easily be found via the reverse dependencies on crates.io and GitHub:

- <https://crates.io/crates/ratatui/reverse_dependencies>
- <https://crates.io/crates/tui/reverse_dependencies>
- <https://github.com/ratatui/ratatui/network/dependents>
- <https://github.com/fdehau/tui-rs/network/dependents?package_id=UGFja2FnZS0zMjE3MzkzMDMx>

## 📦 Libraries

### 🏗️ Frameworks

- [bevy_ratatui_camera](https://github.com/cxreiff/bevy_ratatui_camera) - A bevy plugin for rendering your bevy app to the terminal using ratatui.
- [egui-ratatui](https://github.com/gold-silver-copper/egui_ratatui) - A ratatui backend that is also an egui widget. Deploy on web with WebAssembly or ship natively with bevy, macroquad, or eframe.
- [mousefood](https://github.com/j-g00da/mousefood) - An embedded-graphics backend for Ratatui.
- [ratatui-minecraft](https://github.com/janTatesa/ratatui-minecraft) - A ratatui backend that uses [valence-screens](https://github.com/White-145/valence-screens)
- [ratatui-uefi](https://github.com/reubeno/tui-uefi) - A ratatui backend for use in UEFI environments.
- [ratatui-wgpu](https://github.com/Jesterhearts/ratatui-wgpu) - A wgpu based rendering backend for ratatui.
- [ratzilla](https://github.com/orhun/ratzilla) - Build terminal-themed web applications with Ratatui and WebAssembly.
- [rlt](https://crates.io/crates/rlt) - A universal load testing framework for Rust, with real-time tui support.
- [soft_ratatui](https://github.com/gold-silver-copper/soft_ratatui) - A software rendering backend for ratatui. No GPU required. TUI everywhere.
- [tui-react](https://crates.io/crates/tui-react) - TUI widgets using a react-like paradigm.
- [tui-realm](https://crates.io/crates/tuirealm) - A ratatui framework inspired by Elm and React.
- [webatui](https://github.com/TylerBloom/webatui) - An integration between the Yew and Ratatui crates for making TUI-themed WebAssembly webapps.
- [widgetui](https://crates.io/crates/widgetui) - A bevy-like widget system for ratatui and crossterm.
- [rat-salsa](https://github.com/thscharler/rat-salsa) - An event-queue for ratatui with tasks, timers, application events, focus handling, dialog windows.
- [raclettui](https://github.com/ishrut/raclettui) - A wayland layer shell window implementing the ratatui backend with cpu and wgpu rendering.

### 🧩 Widgets

- [edtui](https://github.com/preiter93/edtui) - A TUI based vim-inspired editor widget for ratatui.
- [hyperrat](https://crates.io/crates/hyperrat) - An OSC 8 link widget for ratatui.
- [ratatui-explorer](https://github.com/tatounee/ratatui-explorer) - A simple library for creating file explorer for ratatui.
- [ratatui-image](https://crates.io/crates/ratatui-image) - An image widget for ratatui, supporting sixels and unicode-halfblocks.
- [ratatui-fretboard](https://crates.io/crates/ratatui-fretboard) - A widget for displaying musical note positions on a fretboard.
- [ratatui-splash-screen](https://github.com/orhun/ratatui-splash-screen) - A widget to turn any image to a splash screen.
- [ratatui-textarea](https://crates.io/crates/ratatui-textarea) - A simple yet powerful editor widget for ratatui. Fork of `tui-textarea`.
- [ratatui-toaster](https://crates.io/crates/ratatui-toaster) - An extremely lightweight toast engine for ratatui.
- [ratatui-code-editor](https://github.com/vipmax/ratatui-code-editor) - A code editor widget for ratatui, syntax highlighting powered by tree-sitter.
- [term-rustdoc](https://github.com/zjp-CN/term-rustdoc) - A TUI for Rust docs that aims to improve the UX on tree view and generic code.
- [throbber-widgets-tui](https://crates.io/crates/throbber-widgets-tui) - A widget that displays throbber.
- [tui-additions](https://crates.io/crates/tui-additions) - Additions to the rust tui crate.
- [tui-big-text](https://crates.io/crates/tui-big-text) - A simple ratatui widget for displaying big text using the `font8x8` crate.
- [tui-dialog](https://docs.rs/tui-dialog) - A widget for entering a single line of text in a dialog.
- [tui-logger](https://crates.io/crates/tui-logger) - Logger with smart widget for ratatui.
- [tui-menu](https://github.com/shuoli84/tui-menu) - A menu widget for ratatui ecosystem.
- [tui-nodes](https://crates.io/crates/tui-nodes) - Node graph visualization.
- [tui-popup](https://github.com/joshka/tui-popup) - A Popup widget for Ratatui.
- [tui-prompts](https://crates.io/crates/tui-prompts) - A library for building interactive prompts for ratatui.
- [tui-rain](https://github.com/levilutz/tui-rain) - A widget to generate various rain effects.
- [tui-scrollview](https://crates.io/crates/tui-scrollview) - A container that provides a scrolling view at a larger area.
- [tui-term](https://crates.io/crates/tui-term) - A pseudoterminal widget for ratatui.
- [tui-textarea](https://crates.io/crates/tui-textarea) - A simple yet powerful text editor widget for ratatui and tui-rs.
- [tui-tree-widget](https://crates.io/crates/tui-tree-widget) - Tree widget for ratatui.
- [tui-widget-list](https://crates.io/crates/tui-widget-list) - A versatile list implementation for ratatui.
- [tui-checkbox](https://crates.io/crates/tui-checkbox) - A customizable checkbox widget for ratatui.
- [tui-piechart](https://crates.io/crates/tui-piechart) - A configurable, colorful piechart widget that comes in standard and high resolution.
- [rat-widget](https://crates.io/crates/rat-widget) - Widgets for data-input (text-input, date- and number-input, text-area, checkbox, choice, radiobutton, slider, calendar), structural widgets (view, split, tabbed, multi-page), a table widget for large data-sets, a file-dialog, a menubar+sub-menus, a status-bar and some more. With builtin crossterm event-handling and focus-handling.
- [tui-slider](https://crates.io/crates/tui-slider) - A highly customizable slider widget for both horizontal and vertical orientations.

### 🔧 Utilities

- [ansi-to-tui](https://crates.io/crates/ansi-to-tui) - A library to convert ansi color coded text into `ratatui::text::Text`.
- [bevy_ratatui](https://github.com/joshka/bevy_ratatui) - A Rust crate to use Ratatui in a Bevy App.
- [color-to-tui](https://crates.io/crates/color-to-tui) - Parse colors and convert them to `ratatui::style::Colors`.
- [coolor](https://github.com/Canop/coolor) - Tiny color conversion library for TUI application builders.
- [ratatui-garnish](https://github.com/franklaranja/ratatui-garnish) - A powerful composition system for Ratatui widgets.
- [ratatui-macros](https://github.com/kdheepak/ratatui-macros) - Macros for simplifying boilerplate for creating UI using Ratatui.
- [ratatui-interact](https://github.com/Brainwires/ratatui-interact) - Interactive TUI components for Ratatui with focus management and mouse support.
- [tachyonfx](https://github.com/junkdog/tachyonfx) - A shader-like effects library for ratatui.
- [terminput](https://crates.io/crates/terminput) - An abstraction over various backends that provide input events.
- [termprofile](https://github.com/aschey/termprofile) - Detect and handle terminal color/styling support. Supports converting Ratatui color and style objects.
- [tui-input](https://crates.io/crates/tui-input) - A headless input library for TUI apps.
- [tui-syntax-highlight](https://github.com/aschey/tui-syntax-highlight) - Syntax highlighting for code blocks.

## 💻 Apps

### ⌨️ Development Tools

- [ATAC](https://github.com/Julien-cpsn/ATAC) - A feature-full TUI API client for your terminal.
- [BugStalker](https://github.com/godzie44/BugStalker) - Modern rust debugger for Linux x86-64.
- [blippy](https://github.com/AksharP5/blippy) - A keyboard-first TUI for GitHub issues and pull requests.
- [burn](https://github.com/burn-rs/burn) - Comprehensive Deep Learning framework in Rust.
- [cargo-selector](https://github.com/lusingander/cargo-selector) - Cargo subcommand to select and execute binary/example targets.
- [crmux](https://github.com/maedana/crmux) - A TUI viewer for monitoring and managing multiple Claude Code sessions in tmux.
- [deadbranch](https://github.com/armgabrielyan/deadbranch) - A TUI for cleaning stale Git branches safely.
- [desed](https://github.com/SoptikHa2/desed) - Debugging tool for sed scripts.
- [deputui](https://github.com/twiddler/deputui) - Review and install NPM package updates.
- [FileSSH](https://github.com/JayanAXHF/filessh) - A TUI-based file explorer for remote servers.
- [gimoji](https://github.com/zeenix/gimoji) - Makes it easy to add emojis to your Git commit messages.
- [gitu](https://github.com/altsem/gitu) - A TUI Git client inspired by Magit.
- [gitui](https://github.com/extrawurst/gitui) - Terminal UI for Git.
- [glim](https://github.com/junkdog/glim) - Monitor GitLab CI/CD pipelines and projects with style.
- [gobang](https://github.com/TaKO8Ki/gobang) - Cross-platform TUI database management tool.
- [joshuto](https://github.com/kamiyaa/joshuto) - Ranger-like terminal file manager written in Rust.
- [lazyjj](https://github.com/Cretezy/lazyjj) - TUI for the Jujutsu/jj VCS.
- [Maelstrom](https://github.com/maelstrom-software/maelstrom) - A fast test runner that runs every test in its own container locally or distributed.
- [material](https://github.com/azorng/material) - A material design color palette for the terminal.
- [nomad](https://github.com/JosephLai241/nomad) - Customizable next-gen tree command with Git integration and TUI.
- [Oatmeal](https://github.com/dustinblackman/oatmeal) - Terminal UI to chat with large language models (LLM) using different model backends, and integrations with your favourite editors!
- [openapi-tui](https://github.com/zaghaghi/openapi-tui) - Terminal UI to list, browse and run APIs defined with openapi spec.
- [rainfrog](https://github.com/achristmascarl/rainfrog) - A database management TUI for Postgres.
- [repgrep](https://github.com/acheronfail/repgrep) - An interactive replacer for ripgrep that makes it easy to find and replace across files on the command line.
- [scooter](https://github.com/thomasschafer/scooter) - Interactive find and replace in the terminal.
- [serie](https://github.com/lusingander/serie) - A rich Git commit graph in your terminal.
- [Serpl](https://github.com/yassinebridi/serpl) - A simple terminal UI for search and replace, ala VS Code.
- [slumber](https://github.com/LucasPickering/slumber) - Terminal-based HTTP/REST client.
- [TaskUI](https://github.com/thmshmm/taskui) - Simple Terminal UI for Task / taskfile.dev.
- [tenere](https://github.com/pythops/tenere) - TUI interface for LLMs written in Rust.
- [tracexec](https://github.com/kxxt/tracexec) - Tracer for execve{,at} and pre-exec behavior, launcher for debuggers.
- [Yazi](https://github.com/sxyazi/yazi) - Blazing fast terminal file manager written in Rust, based on async I/O.

### 👨‍💻 System Administration

- [bottom](https://github.com/ClementTsang/bottom) - Cross-platform graphical process/system monitor.
- [bpftop](https://github.com/Netflix/bpftop) - Dynamic real-time view of running eBPF programs.
- [ducker](https://github.com/robertpsoane/ducker) - A terminal app for managing Docker containers, inspired by K9s.
- [kdash](https://github.com/kdash-rs/kdash) - A simple and fast dashboard for Kubernetes.
- [kmon](https://github.com/orhun/kmon) - Linux Kernel Manager and Activity Monitor.
- [oxker](https://github.com/mrjackwills/oxker) - Simple TUI to view & control Docker containers.
- [systemctl-tui](https://github.com/rgwood/systemctl-tui) - A fast, simple TUI for interacting with systemd services and their logs.
- [systeroid](https://github.com/orhun/systeroid) - A more powerful alternative to sysctl(8) with a terminal user interface.
- [xplr](https://github.com/sayanarijit/xplr) - Hackable, minimal, and fast TUI file explorer.
- [zenith](https://github.com/bvaisvil/zenith) - Cross-platform monitoring tool for system stats.
- [journalview](https://github.com/codervijo/journalview) - Journalctl log viewer.

### 🌐 Networking and Internet

- [AdGuardian-Term](https://github.com/Lissy93/AdGuardian-Term) - Real-time traffic monitoring and statistics for AdGuard Home.
- [bandwhich](https://github.com/imsnif/bandwhich) - Displays network utilization by process.
- [gping](https://github.com/orf/gping/) - Ping tool with a graph.
- [mqttui](https://github.com/EdJoPaTo/mqttui) - MQTT client for subscribing or publishing to topics.
- [oryx](https://github.com/pythops/oryx) - A TUI for sniffing network traffic using eBPF.
- [termscp](https://github.com/veeso/termscp) - A feature rich terminal UI file transfer and explorer with support for SCP/SFTP/FTP/S3/SMB.
- [trippy](https://github.com/fujiapple852/trippy) - Network diagnostic tool.

### 🚀 Productivity and Utilities

- [atuin](https://github.com/atuinsh/atuin) - Magical shell history.
- [binsider](https://github.com/orhun/binsider) - A TUI for analyzing binary files.
- [csvlens](https://github.com/YS-L/csvlens) - Command line csv viewer.
- [flawz](https://github.com/orhun/flawz) - A TUI for browsing security vulnerabilities (CVEs).
- [gpg-tui](https://github.com/orhun/gpg-tui) - Manage your GnuPG keys with ease!.
- [igrep](https://github.com/konradsz/igrep) - Interactive Grep.
- [jwt-ui](https://github.com/jwt-rs/jwt-ui) - A command line UI for decoding/encoding JSON Web Tokens.
- [taskwarrior-tui](https://github.com/kdheepak/taskwarrior-tui) - TUI for the Taskwarrior command-line task manager.
- [television](https://github.com/alexpasmantier/television) - A blazingly fast general purpose fuzzy finder for your terminal.
- [ttyper](https://github.com/max-niederman/ttyper) - Terminal-based typing test.
- [tui-journal](https://github.com/AmmarAbouZor/tui-journal) - Journaling/Notes-taking terminal-based app.

### 🌌 Other

- [confetty_rs](https://github.com/Handfish/confetty_rs) - Particle system (fireworks, stars) rendered in the terminal.
- [cotp](https://github.com/replydev/cotp) - Command-line TOTP/HOTP authenticator app.
- [hncli](https://github.com/pierreyoda/hncli) - Hacker News read-only TUI.
- [hwatch](https://github.com/blacknon/hwatch) - Alternative watch command with command history and diffs.
- [lemurs](https://github.com/coastalwhite/lemurs) - A customizable TUI login manager for Linux and BSD.
