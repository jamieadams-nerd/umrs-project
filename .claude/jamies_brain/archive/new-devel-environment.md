

# Test Accpimt
- Need an isolated LInux account for testing
- Claude team and access it via sudo and su -
- No privilege-escalation footgun (e.g., passwordless sudo in the wrong direction).
	❌ Giving your main user passwordless sudo to anything
	❌ Letting the agent user escalate back to your primary user
	❌ Sharing writable home directories
	❌ Running agents as your primary log

## Create the account (umrs)
- Creat the linux account
  - No password login = cannot be used interactively
  - Exists only as a target execution identity

```
sudo useradd -m -s /bin/bash umrs-agent
sudo passwd -l umrs-agent   # lock password (no direct login)
```

- Limit the access in sudo and limit switching

````
sudo visudo
````

Add:
```
youruser ALL=(umrs-agent) NOPASSWD: ALL
```


