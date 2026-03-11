The following confined SELinux users are available in Fedora 10:

User	Domain	X Window System	su and sudo	Execute in home directory and /tmp/	Networking
guest_u	guest_t	no	no	no	no
xguest_u	xguest_t	yes	no	no	only Firefox
user_u	user_t	yes	no	no	yes
staff_u	staff_t	yes	only sudo	yes	yes
Table 4.1. SELinux User Capabilities

Linux users in the guest_t, xguest_t, and user_t domains can only run set user ID (setuid) applications if SELinux policy permits it (such as passwd). They can not run the su and /usr/bin/sudo setuid applications, and therefore, can not use these applications to become the Linux root user.

Linux users in the guest_t domain have no network access, and can only log in via a terminal (including ssh; they can log in via ssh, but can not use ssh to connect to another system).

The only network access Linux users in the xguest_t domain have is Firefox connecting to web pages.

By default, Linux users in the guest_t, xguest_t, and user_t domains can not execute applications in their home directories or /tmp/, preventing them from executing applications (which inherit users' permissions) in directories that they have write access to. This prevents flawed or malicious applications from modifying files users' own.

Linux users in the xguest_t, user_t and staff_t domains can log in via the X Window System and a terminal.

By default, Linux users in the staff_t domain do not have permissions to execute applications with /usr/bin/sudo. These permissions must be configured by an administrator.

