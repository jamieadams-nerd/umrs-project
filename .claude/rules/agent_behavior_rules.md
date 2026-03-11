## Repository Interaction Rule

- Never execute git commit.
- Never execute git push.
- Never create or modify branches.
- The agent may modify working files only.
- The agent must not alter repository history.
- Never modify production configuration unless explicitly instructed.

## Protected Files Rule

The following file patterns must never be edited unless the user explicitly instructs it:

- `**/*.json` — configuration files (CUI labels, MLS state, package manifests)
- `**/setrans.conf` — SELinux MCS translation configuration
- `**/.gitignore` — repository ignore rules

These files affect deployed system behavior or repository integrity.
Changes must be intentional and user-directed.

