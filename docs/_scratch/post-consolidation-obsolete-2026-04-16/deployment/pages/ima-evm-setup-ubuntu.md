Configuring IMA (Integrity Measurement Architecture) and EVM (Extended Verification Module) on Ubuntu differs from RHEL 10 primarily in how you manage kernel parameters and persistent policy loading. While RHEL uses grubby, Ubuntu relies on standard GRUB configuration files and update-grub. 
1. Enable IMA/EVM in the Kernel 
First, you must enable the subsystems via the boot command line.
Edit the GRUB configuration:
bash
sudo nano /etc/default/grub
Use code with caution.

Modify the GRUB_CMDLINE_LINUX_DEFAULT line to include your required parameters. For a standard setup similar to RHEL's "fix" mode:
ima_policy=tcb: Uses the default Trusted Computing Base policy.
ima_appraise=fix: Allows the system to boot while labeling files.
evm=fix: Enables EVM in fix mode to initialize extended attributes.
Example: GRUB_CMDLINE_LINUX_DEFAULT="quiet splash ima_policy=tcb ima_appraise=fix evm=fix"
Update GRUB and reboot:
bash
sudo update-grub
sudo reboot
Use code with caution.

 
2. Persistent Policy Loading
In RHEL, you might have used a custom systemd service. On Ubuntu, while a service is still a valid option, the standard way to persist a custom policy is to use the dedicated /etc/ima/ima-policy file. 
Intel
Intel
 +1
Create the policy directory:
bash
sudo mkdir -p /etc/ima
Use code with caution.

Deploy your policy file:
Copy your existing configuration to the persistent location:
bash
sudo cp /path/to/your/custom_policy.txt /etc/ima/ima-policy
Use code with caution.

Create a systemd service (Ubuntu Alternative):
If you prefer the RHEL-style service for specialized loading (e.g., loading keys before the policy), create /etc/systemd/system/ima-policy-load.service:
ini
[Unit]
Description=Load Custom IMA Policy
DefaultDependencies=no
After=local-fs.target
Before=sysinit.target

[Service]
Type=oneshot
ExecStart=/bin/sh -c 'cat /etc/ima/ima-policy > /sys/kernel/security/ima/policy'
RemainAfterExit=yes

[Install]
WantedBy=sysinit.target
Use code with caution.

Enable it with: sudo systemctl enable ima-policy-load.service. 
Systron Micronix
Systron Micronix
 +1
3. Monitoring a Specific File
To monitor a specific file (e.g., /etc/shadow), your configuration file should include a rule using the fowner or obj_user criteria, or more commonly, a path-based rule if your kernel supports it.
Add this rule to your /etc/ima/ima-policy:
text
# Measure and appraise a specific file
measure func=FILE_CHECK path=/etc/shadow
appraise func=FILE_CHECK path=/etc/shadow
Use code with caution.

Note: Ensure the file path is absolute. If you want to monitor all files in a directory, use path=/your/dir/. 
Alert Logic
Alert Logic
 +1
4. Verification
After rebooting, verify your configuration is active: 
Check kernel parameters: cat /proc/cmdline.
Verify policy is loaded: sudo cat /sys/kernel/security/ima/policy.
Check measurements: sudo head /sys/kernel/security/ima/ascii_runtime_measurements. 
Read the Docs
Read the Docs
 +4
Next Step: If you are using EVM signatures rather than just HMAC, you will also need to load your public keys into the .ima and .evm keyrings, typically done via the initramfs or an early-boot script. 



