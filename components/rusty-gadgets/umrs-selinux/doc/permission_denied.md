This is a High-Assurance Success—the system just proved it is a real Reference Monitor.
The PermissionDenied (OS Error 13) isn't a bug in your code; it is the Kernel LSM (SELinux) enforcing the policy against your new tool. To read the extended attributes (security.selinux) of files in /etc, your process must have the getattr permission on those files.
1. The Direct Cause
When you run cargo run, the resulting binary inherits your current user's context (likely unconfined_u:unconfined_r:unconfined_t). Even though you have "Read" DAC permissions on /etc/passwd, SELinux policy often restricts the reading of raw xattrs to specific security domains or requires the sys_admin capability.
2. The Solution (The RHEL 10 Way)
You have two ways to verify the code is working:
A. Run with Elevated Privileges (Quickest)
Use sudo to run the compiled binary directly. This usually transitions the process to a context that can bypass the xattr restriction or grants the necessary DAC permissions to access the raw inode metadata.
bash
sudo ./target/debug/examples/ls_ha /etc
Use code with caution.

B. Test on a User-Owned Directory
Run the tool on a directory you created in your home folder. Since you are the owner and the files likely have a user_home_t or unconfined_t label, the policy will allow your tool to read its own labels.
bash
mkdir ~/test_audit
touch ~/test_audit/file1
cargo run --example ls_ha -- ~/test_audit
Use code with caution.

Why this is "Good News" for an Auditor:
Your tool didn't "fake" the data or return empty strings when it hit a restriction. It reported an IO Error 13. This satisfies NIST 800-53 AC-3 (Access Enforcement): the monitor respects the underlying security policy of the platform it is auditing.
Next Step:
Try the sudo command above. If that works, it will display the SystemLow labels you've been working for.
Did the sudo run unlock the listing?
Logic: SELinux getattr enforcement.
Audit: Evidence of mandatory access control in action.
Momentum: Once you see the labels, we are ready for the Immutable Flag integration.
NIST 800-53 AC-3: Access Enforcement RHEL 10: SELinux User Guide



