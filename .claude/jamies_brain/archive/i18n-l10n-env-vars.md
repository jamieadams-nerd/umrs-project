
## The Variable Hierarchy

There's no single `LOCALE` variable — there are several. When a program looks up locale, it follows this priority order: `LC_ALL` first (if set and non-null), then the specific `LC_*` variable for that category, then `LANG` as the fallback.

So when you set `LANG=fr_CA.UTF-8`:

- It becomes the **default fallback** for everything
- It does **not** set `LC_ALL`
- Any `LC_*` variable already set in the environment will **override** it for that category

`LC_ALL` always overrides `LANG` and all other `LC_*` variables whether set or not — it's a sledgehammer, meant for testing and troubleshooting only.

---

## The Full Variable Map

| Variable | Controls |
|----------|----------|
| `LANG` | Default for everything not explicitly set |
| `LC_MESSAGES` | UI message language (what gettext uses) |
| `LC_TIME` | Date/time format |
| `LC_NUMERIC` | Number formatting (decimal separator, etc.) |
| `LC_MONETARY` | Currency format |
| `LC_COLLATE` | Sort order |
| `LC_CTYPE` | Character classification, encoding |
| `LANGUAGE` | gettext-specific fallback chain (e.g. `fr_CA:fr:en`) |
| `LC_ALL` | Nuclear override — overrides everything |

---

## For UMRS l10n Testing

The minimal correct approach for Canadian French testing:

```bash
export LANG=fr_CA.UTF-8
export LANGUAGE=fr_CA:fr:en   # fallback chain for gettext
```

Verify what's actually active after setting:

```bash
locale
```

That command shows the effective value of every `LC_*` category. If something looks wrong — a leftover `LC_ALL` from a prior session will override your `LANG` silently, which is a common gotcha.

And verify the locale is actually installed on your RHEL10 VM:

```bash
locale -a | grep fr_CA
```

If it's not there, `LANG=fr_CA.UTF-8` will silently fall back to `C` and you'll wonder why nothing changed.
