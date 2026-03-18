# No root needed — installs to ~/.local/bin
curl -sfL https://install.Vale.sh | sh -s -- --dir ~/.local/bin
vale --version
```

So the full root-access picture is:

| Tool | Root needed? | Install method |
|---|---|---|
| Vale | No | Binary to `~/.local/bin` |
| write-good | Yes (global npm) | `sudo npm install -g write-good` |
| textstat | Yes (system pip) | `sudo pip install textstat --break-system-packages` |
| PyYAML (for gen script) | Yes (system pip) | `sudo pip install pyyaml --break-system-packages` |

I'll make sure the plan is updated to flag these explicitly before the rust-developer touches the install steps.

---

**Corrected plan execution order:**
```
1. claude-housekeeping.md           ← Clean foundation
2. researcher-enhancement-plan.md   ← Knowledge service
3. antora-theme                     ← Look and feel first
4. antora-doc-restructure           ← Structure second
5. doc-quality-plan.md              ← Quality gates last
