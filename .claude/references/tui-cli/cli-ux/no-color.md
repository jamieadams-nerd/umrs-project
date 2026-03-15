# NO_COLOR

<!-- Source: https://raw.githubusercontent.com/jcs/no_color/master/index.md -->
<!-- Fetched: 2026-03-15 -->
<!-- Original site: https://no-color.org/ -->

An increasing number of command-line software programs output text with
[ANSI color](https://en.wikipedia.org/wiki/ANSI_escape_code#Colors)
escape codes by default.
While some developers and users obviously prefer seeing these colors,
some users don't.
Unfortunately, every new piece of software seems to have a
different way of disabling colored text output and some software has no way at all.

Accepting the futility of trying to reverse this trend, an informal standard
was proposed in 2017:

> **Command-line software which adds ANSI color to its output by default should
check for a `NO_COLOR` environment variable that, when present and not an empty
string (regardless of its value), prevents the addition of ANSI color.**

By adopting this standard, users that prefer to have plain, non-colored text
output can export `NO_COLOR=1` to their shell's environment and automatically
disable color by default in all supported software.

If your software outputs color by default, please consider not doing so.
If you insist, please implement this standard to make it easy for your
users to disable color and then add your software to this list by
[submitting a pull request](https://github.com/jcs/no_color).

If your software does not output color by default, you do not need to bother
with this standard.

## Example Implementation

```c
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

int
main(int argc, char *argv[])
{
    char *no_color = getenv("NO_COLOR");
    bool color = true;

    if (no_color != NULL && no_color[0] != '\0')
        color = false;

    /* do getopt(3) and/or config-file parsing to possibly turn color back on */
    ...
}
```

## Frequently Asked Questions

1. **Why not just set `$TERM` to `dumb` or `xterm` without color support?
   Or change all color definitions in the terminal to print the same color?**

   The terminal is capable of color and should be able to print color when
   instructed. `NO_COLOR` is a hint to the software running in the terminal to
   suppress addition of color, not to the terminal to prevent any color from
   being shown.

   It is reasonable to configure certain software such as a text editor to use
   color even when `NO_COLOR` is set, as the user may have requested color
   support in that specific application.

2. **Why should `NO_COLOR` override application-specific settings?**

   Because the user has explicitly stated their preference. When a user sets
   `NO_COLOR`, they want all color suppressed by default. Application-specific
   flags or config options can override this if the user explicitly opts back in
   for that specific program.

3. **Why not use `TERM=dumb`?**

   `TERM=dumb` affects terminal behavior more broadly, including disabling
   features like cursor positioning that many programs rely on. `NO_COLOR` is
   a targeted signal about color only.

4. **What about the `FORCE_COLOR` variable?**

   In 2023, an informal companion standard was proposed: `FORCE_COLOR`. When
   present and not empty, `FORCE_COLOR` instructs programs to add ANSI color
   even when they would otherwise not (e.g., when stdout is not a TTY). This
   is useful for CI systems that want color in logs.
   See [force-color.org](https://force-color.org/).

## Related Standards and Further Reading

- [force-color.org](https://force-color.org/) — companion standard for forcing color
- [CLIG: Disable color when not in terminal or user requests it](https://clig.dev/#output)
- [12 Factor CLI Apps](https://medium.com/@jdxcode/12-factor-cli-apps-dd3c227a0e46)
- GitHub repository: [github.com/jcs/no_color](https://github.com/jcs/no_color)

## Note on Supporting Software List

The full, current list of software supporting `NO_COLOR` is maintained at
[no-color.org](https://no-color.org/) and in the [GitHub repository](https://github.com/jcs/no_color).
The list includes hundreds of entries across many languages and categories including:

- System monitoring tools (btop, bpytop, bashtop)
- Build tools (cargo, cmake, meson)
- Shell utilities (bat, exa/eza, fd, ripgrep, delta)
- Language toolchains (rustc, go, python/pip)
- Version control (git via diff-highlight, lazygit)
- Container tools (docker, podman)
- Text editors (helix, micro, vim plugins)
- TUI frameworks (ratatui, crossterm, tui-rs)

For the authoritative and up-to-date list, consult the source repository directly.
