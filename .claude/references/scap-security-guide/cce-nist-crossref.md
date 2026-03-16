# RHEL 10 STIG — CCE → NIST Cross-Reference

**Source:** `rhel10-playbook-stig.yml` (SCAP Security Guide)
**Unique CCEs:** 451

| CCE | NIST Controls | Signal Name | Description |
|---|---|---|---|
| CCE-86181-5 | CM-6(a), CM-7(a), CM-7(b) | `package_unbound_removed` | 'Uninstall unbound Package: Ensure unbound is removed' |
| CCE-86188-0 |  | `audit_rules_file_deletion_events_renameat2` | Set architecture for audit renameat2 tasks |
| CCE-86192-2 | IA-2(11) | `sssd_certificate_verification` | Ensure that "certificate_verification" is not set in /etc/sssd/sssd.conf |
| CCE-86241-7 | AC-17(1), AC-17(a), CM-6(a) | `sshd_set_loglevel_verbose` | Set SSH Daemon LogLevel to VERBOSE - Check if the parameter LogLevel is |
| CCE-86242-5 | SC-5 | `sysctl_net_ipv4_tcp_invalid_ratelimit` | Configure Kernel to Rate Limit Sending of Duplicate TCP Acknowledgments |
| CCE-86250-8 | CM-6(a) | `package_pcsc-lite-ccid_installed` | Ensure pcsc-lite-ccid is installed |
| CCE-86254-0 | AC-17(a), AC-6(1), CM-6(a) | `file_groupowner_sshd_drop_in_config` | Set the file_groupowner_sshd_drop_in_config_newgroup variable if represented |
| CCE-86264-9 | AC-17(a), AC-6(1), CM-6(a) | `file_permissions_sshd_config` | Test for existence /etc/ssh/sshd_config |
| CCE-86268-0 | AC-17(a), AC-6(1), CM-6(a) | `file_owner_sshd_drop_in_config` | Set the file_owner_sshd_drop_in_config_newown variable if represented by |
| CCE-86440-5 | CM-5(6), CM-5(6).1 | `root_permissions_syslibrary_files` | Set the root_permissions_syslibrary_files_newgroup variable if represented |
| CCE-86441-3 | AU-9(3), AU-9(3).1 | `aide_check_audit_tools` | Ensure aide is installed |
| CCE-86442-1 | AC-17(a), AC-6(1), CM-6(a) | `file_permissions_sshd_drop_in_config` | Find /etc/ssh/sshd_config.d/ file(s) |
| CCE-86463-7 | AC-6, AC-6(10), AC-6(8), CM-6(a) | `mount_option_noexec_remote_filesystems` | Get nfs and nfs4 mount points, that don't have noexec |
| CCE-86471-0 |  | `chrony_set_nts` | Configure Time Service to use NTS - Check That /etc/ntp.conf Exist |
| CCE-86476-9 |  | `rootfiles_configured` | Ensure rootfiles tmpfile.d is Configured Correctly - Find configuration |
| CCE-86495-9 | IA-5 (1) (c) | `tftp_uses_secure_mode_systemd` | Ensure tftp systemd Service Uses Secure Mode - Find valid drop-ins |
| CCE-86528-7 | CM-6(b) | `sshd_x11_use_localhost` | Prevent remote hosts from connecting to the proxy display - Check if the |
| CCE-86535-2 |  | `audit_rules_dac_modification_fchmodat2` | Set architecture for audit fchmodat2 tasks |
| CCE-86539-4 | AC-17(a), AC-8(a), AC-8(c), CM-6(a) | `sshd_enable_warning_banner` | Enable SSH Warning Banner - Check if the parameter Banner is configured |
| CCE-86569-1 | CM-6(a), CM-7(a), CM-7(b) | `kernel_module_tipc_disabled` | Ensure kernel module 'tipc' is disabled |
| CCE-86578-2 | AU-9 | `file_audit_tools_permissions` | Test for existence /sbin/auditctl |
| CCE-86579-0 | AC-6 (1) | `file_permissions_backup_etc_group` | Test for existence /etc/group- |
| CCE-86590-7 | AC-6(9), AU-12(c), AU-2(d), CM-6(a) | `audit_rules_media_export` | Set architecture for audit mount tasks |
| CCE-86605-3 |  | `file_permissions_home_directories` | Get all local users from /etc/passwd |
| CCE-86609-5 |  | `dconf_db_up_to_date` | Make sure that the dconf databases are up-to-date with regards to respective |
| CCE-86619-4 | CM-6(a) | `package_cron_installed` | Ensure cronie is installed |
| CCE-86620-2 |  | `audit_rules_privileged_commands_unix_update` | Ensure auditd Collects Information on the Use of Privileged Commands - unix_upda |
| CCE-86628-5 | CM-6(a), CM-7(a), CM-7(b) | `dconf_gnome_disable_automount_open` | Disable GNOME3 Automounting - automount-open |
| CCE-86632-7 | AC-6(1), CM-6(a) | `file_permissions_cron_monthly` | Find /etc/cron.monthly/ file(s) |
| CCE-86640-0 | CM-6(a), IA-5(1)(a), IA-5(c) | `no_empty_passwords` | Prevent Login to Accounts With Empty Password - Check if system relies on |
| CCE-86642-6 | CM-6(a) | `install_smartcard_packages` | Ensure pkcs11-provider is installed |
| CCE-86651-7 | AC-6(1), CM-6(a) | `file_permissions_cron_d` | Find /etc/cron.d/ file(s) |
| CCE-86659-0 |  | `accounts_user_interactive_home_directory_exists` | Get all local users from /etc/passwd |
| CCE-86671-5 | CM-6(a), SI-2(5), SI-2(c) | `dnf-automatic_apply_updates` | Configure dnf-automatic to Install Available Updates Automatically |
| CCE-86672-3 | AC-7(a), CM-6(a) | `accounts_passwords_pam_faillock_interval` | Set Interval For Counting Failed Password Attempts - Check if system relies |
| CCE-86689-7 | AC-6(1), CM-6(a) | `sysctl_fs_protected_hardlinks` | Enable Kernel Parameter to Enforce DAC on Hardlinks - Set fact for sysctl |
| CCE-86705-1 |  | `file_owner_var_log` | Set the file_owner_var_log_newown variable if represented by uid |
| CCE-86714-3 | SC-7(10) | `sysctl_kernel_core_pattern` | Disable storing core dumps - Set fact for sysctl paths |
| CCE-86727-5 | AU-12(a), AU-12.1(ii), AU-12.1(iv)AU-12(c), AU-3, AU-3.1, MA-4(1)(a) | `audit_rules_privileged_commands_kmod` | Ensure auditd Collects Information on the Use of Privileged Commands - kmod |
| CCE-86729-1 | AU-12(c), AU-2(d), CM-6(a) | `audit_rules_unsuccessful_file_modification_ftruncate` | Set architecture for audit ftruncate tasks |
| CCE-86731-7 | AC-6(1), AU-9(4), CM-6(a) | `directory_ownership_var_log_audit` | System Audit Directories Must Be Owned By Root - Register Audit Configuration |
| CCE-86737-4 | AU-12(c), AU-2(d), CM-6(a) | `audit_rules_file_deletion_events_unlink` | Set architecture for audit unlink tasks |
| CCE-86738-2 | CM-6(a), SI-7, SI-7(1) | `aide_periodic_cron_checking` | Ensure AIDE is installed |
| CCE-86744-0 | AU-12(c) | `audit_privileged_commands_poweroff` | Ensure auditd Collects Information on the Use of Privileged Commands - poweroff |
| CCE-86753-1 | AC-17(a), CM-6(a), CM-7(a), CM-7(b) | `sshd_disable_empty_passwords` | Disable SSH Access via Empty Passwords - Check if the parameter PermitEmptyPassw |
| CCE-86775-4 | AC-6, AC-6(1), CM-6(a), CM-7(a), CM-7(b), MP-7 | `mount_option_dev_shm_noexec` | 'Add noexec Option to /dev/shm: Check information associated to mountpoint' |
| CCE-86783-8 | AC-6, AC-6(1), CM-6(a), CM-7(a), CM-7(b), MP-7 | `mount_option_dev_shm_nodev` | 'Add nodev Option to /dev/shm: Check information associated to mountpoint' |
| CCE-86794-5 | AC-12, AC-17(a), AC-2(5), CM-6(a), SC-10 | `sshd_set_keepalive` | Set SSH Client Alive Count Max - Check if the parameter ClientAliveCountMax |
| CCE-86811-7 | AU-8(1)(a), CM-6(a) | `chronyd_specify_remote_server` | Detect if chrony is already configured with pools or servers |
| CCE-86820-8 | CM-6(a), CM-7(a), CM-7(b), SC-7(a) | `sysctl_net_ipv4_conf_default_accept_redirects` | Disable Kernel Parameter for Accepting ICMP Redirects by Default on IPv4 |
| CCE-86822-4 | AC-7(b), CM-6(a) | `accounts_logon_fail_delay` | Set accounts logon fail delay |
| CCE-86823-2 | CM-6 b | `file_owner_cron_deny` | Set the file_owner_cron_deny_newown variable if represented by uid |
| CCE-86839-8 | AU-9 | `file_audit_tools_group_ownership` | Set the file_audit_tools_group_ownership_newgroup variable if represented |
| CCE-86852-1 |  | `package_openssh-clients_installed` | Ensure openssh-clients is installed |
| CCE-86854-7 | AC-6 (1) | `file_permissions_backup_etc_passwd` | Test for existence /etc/passwd- |
| CCE-86857-0 | AC-6(1), CM-6(a) | `file_owner_etc_shadow` | Set the file_owner_etc_shadow_newown variable if represented by uid |
| CCE-86870-3 | AC-6(1), CM-6(a) | `file_owner_etc_group` | Set the file_owner_etc_group_newown variable if represented by uid |
| CCE-86874-5 | SC-12(2), SC-12(3), SC-13 | `configure_bind_crypto_policy` | Configure BIND to use System Crypto Policy - Check BIND configuration file |
| CCE-86882-8 | CM-6(a), CM-6(b), CM-6.1(iv), CM-7(a), CM-7(b) | `sysctl_net_ipv6_conf_all_forwarding` | Disable Kernel Parameter for IPv6 Forwarding - Set fact for sysctl paths |
| CCE-86898-4 | CM-6(a) | `package_opensc_installed` | Ensure opensc is installed |
| CCE-86918-0 | CM-7(a), CM-7(b), SC-5 | `sysctl_net_ipv4_icmp_echo_ignore_broadcasts` | Enable Kernel Parameter to Ignore ICMP Broadcast Echo Requests on IPv4 Interface |
| CCE-86924-8 |  | `file_groupowner_var_log_messages` | Set the file_groupowner_var_log_messages_newgroup variable if represented |
| CCE-86942-0 | CM-6(a) | `aide_build_database` | Build and Test AIDE Database - Ensure AIDE Is Installed |
| CCE-86957-8 | AC-6 (1) | `file_owner_backup_etc_gshadow` | Set the file_owner_backup_etc_gshadow_newown variable if represented by |
| CCE-86962-8 | AC-6(9), AU-12(c), AU-2(d), CM-6(a) | `audit_rules_privileged_commands_umount` | Ensure auditd Collects Information on the Use of Privileged Commands - umount |
| CCE-86975-0 | AC-6(1), CM-6(a) | `file_permissions_etc_gshadow` | Test for existence /etc/gshadow |
| CCE-86978-4 | AC-6(1), CM-5(6), CM-5(6).1, CM-6(a) | `file_permissions_binary_dirs` | Read list of world and group writable system executables |
| CCE-86992-5 | AC-17(a), AC-6(1), CM-6(a) | `file_groupowner_sshd_config` | Set the file_groupowner_sshd_config_newgroup variable if represented by |
| CCE-87003-0 | AU-4(1) | `auditd_overflow_action` | Insert correct line to /etc/audit/auditd.conf |
| CCE-87004-8 |  | `package_policycoreutils-python-utils_installed` | Ensure policycoreutils-python-utils is installed |
| CCE-87015-4 | CM-6(a), IA-11 | `sudo_remove_nopasswd` | Find /etc/sudoers.d/ files |
| CCE-87045-1 |  | `sshd_enable_pam` | Enable PAM - Check if the parameter UsePAM is configured |
| CCE-87052-7 | AU-12(c), AU-2(d), CM-6(a) | `audit_rules_unsuccessful_file_modification_creat` | Set architecture for audit creat tasks |
| CCE-87057-6 | AC-6(1), CM-6(a), CM-7(b) | `gnome_gdm_disable_automatic_login` | Disable GDM Automatic Login |
| CCE-87066-7 | CM-7(1) | `chronyd_no_chronyc_network` | Insert correct line to /etc/chrony.conf |
| CCE-87070-9 | AC-6, AC-6(1), CM-6(a), CM-7(a), CM-7(b), MP-7 | `mount_option_var_nodev` | 'Add nodev Option to /var: Check information associated to mountpoint' |
| CCE-87095-6 | AC-6, AC-6(1), CM-6(a), CM-7(a), CM-7(b), MP-7 | `mount_option_tmp_noexec` | 'Add noexec Option to /tmp: Check information associated to mountpoint' |
| CCE-87099-8 | SC-5 | `firewalld-backend` | Insert correct line to /etc/firewalld/firewalld.conf |
| CCE-87100-4 | CM-6(a) | `package_sudo_installed` | Ensure sudo is installed |
| CCE-87111-1 | AC-2(4), AC-6(9), AU-12(c), AU-2(d), CM-6(a) | `audit_rules_usergroup_modification_group` | Record Events that Modify User/Group Information - /etc/group - Check if |
| CCE-87122-8 |  | `accounts_umask_interactive_users` | Ensure the Default Umask is Set Correctly For Interactive Users - Get interactiv |
| CCE-87137-6 | CM-6(a), IA-5(1)(d), IA-5(f) | `accounts_password_set_max_life_existing` | Collect users with not correct maximum time period between password changes |
| CCE-87152-5 | AU-2, CM-8(3), IA-3 | `configure_usbguard_auditbackend` | Insert correct line to /etc/usbguard/usbguard-daemon.conf |
| CCE-87153-3 | CM-7(a) | `grub2_vsyscall_argument` | Check if vsyscall argument is already present in /etc/default/grub |
| CCE-87170-7 | AC-11(a), CM-6(a) | `dconf_gnome_screensaver_idle_delay` | Set GNOME3 Screensaver Inactivity Timeout |
| CCE-87220-0 | AC-6, AC-6(1), CM-6(a), CM-7(a), CM-7(b), MP-7 | `mount_option_var_log_audit_nodev` | 'Add nodev Option to /var/log/audit: Check information associated to mountpoint' |
| CCE-87249-9 | AC-17(a), CM-6(a), CM-7(a), CM-7(b), IA-2, IA-2(8), IA-2(9) | `mount_option_krb_sec_remote_filesystems` | Get nfs and nfs4 mount points, that don't have sec=krb5:krb5i:krb5p |
| CCE-87289-5 | CM-6(a), IA-5(1)(a), IA-5(4), IA-5(c) | `accounts_password_pam_minclass` | Ensure PAM Enforces Password Requirements - Minimum Different Categories |
| CCE-87294-5 | AC-6(1), CM-6(a) | `file_owner_crontab` | Set the file_owner_crontab_newown variable if represented by uid |
| CCE-87313-3 | AC-17(a), CM-6(a), CM-7(a), CM-7(b) | `sshd_disable_user_known_hosts` | Disable SSH Support for User Known Hosts - Check if the parameter IgnoreUserKnow |
| CCE-87318-2 | AC-6, AC-6(1), CM-6(a), CM-7(a), CM-7(b), MP-7 | `mount_option_tmp_nosuid` | 'Add nosuid Option to /tmp: Check information associated to mountpoint' |
| CCE-87341-4 | AC-17(1) | `rsyslog_remote_access_monitoring` | 'Ensure remote access methods are monitored in Rsyslog: Set facts' |
| CCE-87344-8 |  | `mount_option_home_nodev` | 'Add nodev Option to /home: Check information associated to mountpoint' |
| CCE-87347-1 |  | `mount_option_var_tmp_noexec` | 'Add noexec Option to /var/tmp: Check information associated to mountpoint' |
| CCE-87349-7 | AU-12(c), AU-2(d), CM-6(a) | `audit_rules_unsuccessful_file_modification_open` | Set architecture for audit open tasks |
| CCE-87352-1 | AU-5(b), CM-6(a), SC-24 | `audit_rules_system_shutdown` | Collect all files from /etc/audit/rules.d with .rules extension |
| CCE-87356-2 | CM-6(a) | `dconf_gnome_screensaver_lock_locked` | Prevent user modification of GNOME Screensaver lock-enabled |
| CCE-87377-8 | CM-6(a), CM-7(a), CM-7(b), SC-5, SC-7(a) | `sysctl_net_ipv4_ip_forward` | Disable Kernel Parameter for IP Forwarding on IPv4 Interfaces - Set fact |
| CCE-87388-5 | AC-7(a), CM-6(a) | `accounts_passwords_pam_faillock_deny` | Lock Accounts After Failed Password Attempts - Check if system relies on |
| CCE-87395-0 | AC-17(a), CM-6(a), CM-7(a), CM-7(b) | `sshd_do_not_permit_user_env` | Do Not Allow SSH Environment Options - Check if the parameter PermitUserEnvironm |
| CCE-87417-2 | AC-8(a), AC-8(b), AC-8(c) | `dconf_gnome_banner_enabled` | Enable GNOME3 Login Warning Banner |
| CCE-87420-6 | CM-6(b) | `sysctl_net_ipv4_conf_all_forwarding` | Disable Kernel Parameter for IPv4 Forwarding on all IPv4 Interfaces - Set |
| CCE-87423-0 | AC-6 (1) | `file_permissions_backup_etc_shadow` | Test for existence /etc/shadow- |
| CCE-87424-8 | CM-6(a), CM-7(a), CM-7(b), SC-7(a) | `sysctl_net_ipv4_conf_default_rp_filter` | Enable Kernel Parameter to Use Reverse Path Filtering on all IPv4 Interfaces |
| CCE-87429-7 | AU-3, CM-6 | `auditd_name_format` | Set type of computer node name logging in audit logs - Define Value to Be |
| CCE-87448-7 | AC-6, CM-6(a), CM-6(b), CM-6.1(iv) | `no_shelllogin_for_systemaccounts` | Ensure that System Accounts Do Not Run a Shell Upon Login - Get All Local |
| CCE-87454-5 | AC-17(a), AC-6(1), CM-6(a) | `file_permissions_sshd_pub_key` | Find /etc/ssh/ file(s) |
| CCE-87455-2 | AC-18(3), AC-18(a), CM-6(a), CM-7(a), CM-7(b), MP-7 | `kernel_module_bluetooth_disabled` | Ensure kernel module 'bluetooth' is disabled |
| CCE-87482-6 | CM-6 | `auditd_freq` | Insert correct line to /etc/audit/auditd.conf |
| CCE-87497-4 | CM-6(a) | `package_libreswan_installed` | Ensure libreswan is installed |
| CCE-87499-0 | AC-6(1), CM-6(a) | `file_owner_cron_daily` | Set the file_owner_cron_daily_newown variable if represented by uid |
| CCE-87502-1 | AC-6 (1) | `file_owner_backup_etc_shadow` | Set the file_owner_backup_etc_shadow_newown variable if represented by uid |
| CCE-87552-6 | AC-6(5), IA-2, IA-4(b) | `accounts_no_uid_except_zero` | Get all /etc/passwd file entries |
| CCE-87579-9 | AC-6(1), CM-6(a) | `file_groupowner_etc_shadow` | Set the file_groupowner_etc_shadow_newgroup variable if represented by gid |
| CCE-87588-0 | CM-6(a), CM-7(a), CM-7(b) | `dconf_gnome_disable_autorun` | Disable GNOME3 Automounting - autorun-never |
| CCE-87596-3 |  | `package_gssproxy_removed` | 'Uninstall gssproxy Package: Ensure gssproxy is removed' |
| CCE-87601-1 |  | `audit_rules_dac_modification_umount` | Add the audit rule to {{ audit_file }} |
| CCE-87627-6 | AC-6(1), CM-6(a) | `disable_ctrlaltdel_burstaction` | Disable Ctrl-Alt-Del Burst Action |
| CCE-87651-6 | AC-6(1), CM-6(a) | `accounts_umask_etc_profile` | Ensure the Default Umask is Set Correctly in /etc/profile - Locate Profile |
| CCE-87654-0 |  | `package_tuned_removed` | 'Uninstall tuned Package: Ensure tuned is removed' |
| CCE-87657-3 | AC-7 (a) | `account_password_pam_faillock_password_auth` | Configure the Use of the pam_faillock.so Module in the /etc/pam.d/password-auth |
| CCE-87659-9 |  | `audit_rules_privileged_commands_usermod` | Ensure auditd Collects Information on the Use of Privileged Commands - usermod |
| CCE-87662-3 |  | `audit_rules_execution_setfacl` | Record Any Attempts to Run setfacl - Set architecture for audit /usr/bin/setfacl |
| CCE-87701-9 | AC-6(1), CM-6(a) | `file_owner_etc_gshadow` | Set the file_owner_etc_gshadow_newown variable if represented by uid |
| CCE-87731-6 | CM-5, CM-5(6), CM-5(6).1 | `dir_permissions_library_dirs` | Find /lib/ file(s) recursively |
| CCE-87736-5 | AC-2(4), AC-6(9), AU-12(c), AU-2(d), CM-6(a) | `audit_rules_usergroup_modification_gshadow` | Record Events that Modify User/Group Information - /etc/gshadow - Check |
| CCE-87741-5 | AC-6(9), AU-12(c), AU-2(d), CM-6(a) | `audit_rules_execution_setsebool` | Record Any Attempts to Run setsebool - Set architecture for audit /usr/sbin/sets |
| CCE-87751-4 |  | `dconf_gnome_lock_screen_on_smartcard_removal` | Detect if removal-action can be found on /etc/dconf/db/local.d/ |
| CCE-87756-3 | CM-8(3), IA-3 | `package_usbguard_installed` | Ensure usbguard is installed |
| CCE-87762-1 | AC-6(9), AU-12(c), AU-2(d), CM-6(a) | `audit_rules_execution_chcon` | Record Any Attempts to Run chcon - Set architecture for audit /usr/bin/chcon |
| CCE-87771-2 |  | `file_permission_user_init_files` | Ensure All User Initialization Files Have Mode 0740 Or Less Permissive - |
| CCE-87777-9 | AC-17(a), CM-6(a), CM-7(a), CM-7(b) | `sshd_disable_rhosts` | Disable SSH Support for .rhosts Files - Check if the parameter IgnoreRhosts |
| CCE-87792-8 |  | `postfix_prevent_unrestricted_relay` | Insert correct line to /etc/postfix/main.cf |
| CCE-87810-8 | CM-6(b) | `mount_option_home_noexec` | 'Add noexec Option to /home: Check information associated to mountpoint' |
| CCE-87813-2 | AU-12(c), AU-2(d), CM-6(a) | `audit_rules_file_deletion_events_unlinkat` | Set architecture for audit unlinkat tasks |
| CCE-87814-0 | AC-6(9), AU-12(c), AU-2(d), CM-6(a) | `audit_rules_privileged_commands_mount` | Ensure auditd Collects Information on the Use of Privileged Commands - mount |
| CCE-87827-2 | AC-6(1), CM-6(a) | `file_owner_etc_passwd` | Set the file_owner_etc_passwd_newown variable if represented by uid |
| CCE-87829-8 |  | `package_nss-tools_installed` | Ensure nss-tools is installed |
| CCE-87837-1 | AC-6(1), CM-6(a), CM-7(b) | `dconf_gnome_disable_restart_shutdown` | Disable the GNOME3 Login Restart and Shutdown Buttons |
| CCE-87841-3 | CM-7(a), CM-7(b), SC-5 | `sysctl_net_ipv4_icmp_ignore_bogus_error_responses` | Enable Kernel Parameter to Ignore Bogus ICMP Error Responses on IPv4 Interfaces |
| CCE-87852-0 | CM-6(a), IA-5(1)(a), IA-5(4), IA-5(c) | `accounts_password_pam_minlen` | Ensure PAM Enforces Password Requirements - Minimum Length - Find pwquality.conf |
| CCE-87874-4 | AU-9 | `file_audit_tools_ownership` | Set the file_audit_tools_ownership_newown variable if represented by uid |
| CCE-87876-9 | CM-6(a), SC-30, SC-30(2) | `sysctl_kernel_randomize_va_space` | Enable Randomized Layout of Virtual Address Space - Set fact for sysctl |
| CCE-87892-6 |  | `mount_option_var_tmp_nosuid` | 'Add nosuid Option to /var/tmp: Check information associated to mountpoint' |
| CCE-87918-9 | AC-23, CM-6(a) | `dconf_gnome_disable_user_list` | Disable the GNOME3 Login User List |
| CCE-87927-0 | AC-6(9), AU-12(c), AU-2(d), CM-6(a) | `audit_rules_privileged_commands_postqueue` | Ensure auditd Collects Information on the Use of Privileged Commands - postqueue |
| CCE-87937-9 | CM-6(a) | `postfix_client_configure_mail_alias` | Configure System to Forward All Mail For The Root Account - Make sure that |
| CCE-87946-0 |  | `file_groupownership_home_directories` | Get all local users from /etc/passwd |
| CCE-87953-6 | CM-6(a), IA-5(1)(d), IA-5(f) | `accounts_password_set_min_life_existing` | Collect users with not correct minimum time period between password changes |
| CCE-87961-9 | CM-6(a), IA-5(1)(d), IA-5(f) | `accounts_maximum_age_login_defs` | Set Password Maximum Age |
| CCE-87975-9 | AC-7(b), CM-6(a), IA-5(c) | `accounts_passwords_pam_faillock_deny_root` | Configure the root Account for Failed Password Attempts - Check if system |
| CCE-87988-2 | AC-6(1), CM-5(6), CM-5(6).1, CM-6(a) | `file_ownership_library_dirs` | Set the file_ownership_library_dirs_newown variable if represented by uid |
| CCE-88013-8 | CM-6(a), MP-2 | `mount_option_nodev_remote_filesystems` | Get nfs and nfs4 mount points, that don't have nodev |
| CCE-88015-3 | CM-6(a), IA-5(4), IA-5(c) | `accounts_password_pam_maxrepeat` | Set Password Maximum Consecutive Repeating Characters - Find pwquality.conf.d |
| CCE-88018-7 | AC-17(a), AC-6(1), CM-6(a) | `file_permissions_sshd_private_key` | Find root:root-owned keys |
| CCE-88037-7 | AC-17(a), AC-6, CM-6(a) | `sshd_enable_strictmodes` | Enable Use of Strict Mode Checking - Check if the parameter StrictModes |
| CCE-88052-6 | AU-12(c), AU-2(d), CM-6(a) | `audit_rules_dac_modification_lsetxattr` | Set architecture for audit lsetxattr tasks |
| CCE-88057-5 | AC-17(a), AC-3, CM-6(a), CM-7(a), CM-7(b) | `disable_host_auth` | Disable Host-Based Authentication - Check if the parameter HostbasedAuthenticati |
| CCE-88060-9 | CM-6 b | `file_groupowner_cron_deny` | Set the file_groupowner_cron_deny_newgroup variable if represented by gid |
| CCE-88064-1 | CM-6 | `auditd_local_events` | Insert correct line to /etc/audit/auditd.conf |
| CCE-88067-4 | AU-12 b | `file_permissions_audit_configuration` | Find /etc/audit/ file(s) |
| CCE-88071-6 | CM-7(a), CM-7(b), SC-5, SC-7(a) | `sysctl_net_ipv4_conf_default_accept_source_route` | Disable Kernel Parameter for Accepting Source-Routed Packets on IPv4 Interfaces |
| CCE-88078-1 | AC-6, AC-6(1), CM-6(a), CM-7(a), CM-7(b), MP-7 | `mount_option_nosuid_removable_partitions` | Ensure permission nosuid are set on var_removable_partition |
| CCE-88084-9 | CM-6(a), CM-7(a), CM-7(b), SC-5(1), SC-5(2), SC-5(3)(a) | `sysctl_net_ipv4_tcp_syncookies` | Enable Kernel Parameter to Use TCP Syncookies on Network Interfaces - Set |
| CCE-88105-2 | CM-6(a), CM-7(a), CM-7(b) | `package_telnet-server_removed` | 'Uninstall telnet-server Package: Ensure telnet-server is removed' |
| CCE-88132-6 | AU-12(c), AU-2(d), CM-6(a) | `audit_rules_unsuccessful_file_modification_renameat` | Set architecture for audit renameat tasks |
| CCE-88136-7 | IA-11 | `sudo_require_reauthentication` | Require Re-Authentication When Using the sudo Command - Find /etc/sudoers.d/* |
| CCE-88140-9 | AC-6(1), CM-6(a) | `file_groupowner_cron_hourly` | Set the file_groupowner_cron_hourly_newgroup variable if represented by |
| CCE-88163-1 | AC-12, AC-2(5), CM-6(a), SC-10 | `accounts_tmout` | Correct any occurrence of TMOUT in /etc/profile |
| CCE-88164-9 | CM-6(a) | `package_firewalld_installed` | Ensure firewalld is installed |
| CCE-88171-4 | CM-6(a), IA-5(1)(a), IA-5(4), IA-5(c) | `accounts_password_pam_dictcheck` | Ensure PAM Enforces Password Requirements - Prevent the Use of Dictionary |
| CCE-88176-3 | CM-11(a), CM-11(b), CM-5(3), CM-6(a), SA-12, SA-12(10), SC-12, SC-12(3), SI-7 | `ensure_gpgcheck_never_disabled` | Grep for dnf repo section names |
| CCE-88192-0 | CM-6(a) | `grub2_audit_backlog_limit_argument` | Check if audit_backlog_limit argument is already present in /etc/default/grub |
| CCE-88200-1 | AU-12(c), AU-2(d), CM-6(a) | `audit_rules_dac_modification_fchmod` | Set architecture for audit fchmod tasks |
| CCE-88214-2 | AU-12(c) | `audit_privileged_commands_init` | Ensure auditd Collects Information on the Use of Privileged Commands - init |
| CCE-88235-7 |  | `file_groupowner_backup_etc_shadow` | Set the file_groupowner_backup_etc_shadow_newgroup variable if represented |
| CCE-88238-1 |  | `file_groupownership_audit_configuration` | Set the file_groupownership_audit_configuration_newgroup variable if represented |
| CCE-88240-7 | AC-7(a), AU-12(2), AU-14, AU-2(a), AU-7(1), AU-7(2), CM-6(a) | `package_audit_installed` | Ensure audit is installed |
| CCE-88243-1 | AU-12(c), AU-2(d), CM-6(a) | `audit_rules_dac_modification_lchown` | Set architecture for audit lchown tasks |
| CCE-88256-3 | CM-5(3), CM-6(a), SC-12, SC-12(3), SI-7 | `ensure_redhat_gpgkey_installed` | Read permission of GPG key directory |
| CCE-88261-3 | AC-8(a), AC-8(c) | `banner_etc_issue` | Modify the System Login Banner - Ensure Correct Banner |
| CCE-88270-4 |  | `package_nfs-utils_removed` | 'Uninstall nfs-utils Package: Ensure nfs-utils is removed' |
| CCE-88286-0 | AC-2(4), AC-6(9), AU-12(c), AU-2(d), CM-6(a) | `audit_rules_usergroup_modification_passwd` | Record Events that Modify User/Group Information - /etc/passwd - Check if |
| CCE-88290-2 | CM-5(6), CM-5(6).1 | `dir_group_ownership_library_dirs` | Set the dir_group_ownership_library_dirs_newgroup variable if represented |
| CCE-88330-6 | CM-6, SC-7(10) | `disable_users_coredumps` | Disable Core Dumps for All Users - Set dirs, files and regex variables |
| CCE-88334-8 | AC-12, AC-17(a), AC-2(5), CM-6(a), SC-10 | `logind_session_timeout` | Set 'StopIdleSessionSec' to '{{ var_logind_session_timeout }}' in the [Login] |
| CCE-88349-6 | CM-6(a) | `dconf_gnome_screensaver_user_locks` | Prevent user modification of GNOME lock-delay |
| CCE-88352-0 | AU-12(c), AU-2(d), CM-6(a) | `audit_rules_dac_modification_fremovexattr` | Set architecture for audit fremovexattr tasks |
| CCE-88356-1 |  | `sshd_rekey_limit` | Force frequent session key renegotiation - Check if the parameter RekeyLimit |
| CCE-88358-7 | AC-6, AC-6(1), CM-6(a), CM-7(a), CM-7(b), MP-7 | `mount_option_dev_shm_nosuid` | 'Add nosuid Option to /dev/shm: Check information associated to mountpoint' |
| CCE-88359-5 | AU-4(1) | `rsyslog_encrypt_offload_actionsendstreamdrivermode` | Ensure Rsyslog Encrypts Off-Loaded Audit Records - Ensure /etc/rsyslog.conf |
| CCE-88360-3 | CM-6(a), CM-7(a), CM-7(b), SC-5, SC-7(a) | `sysctl_net_ipv4_conf_all_send_redirects` | Disable Kernel Parameter for Sending ICMP Redirects on all IPv4 Interfaces |
| CCE-88362-9 | AC-9, AC-9(1) | `sshd_print_last_log` | Enable SSH Print Last Log - Check if the parameter PrintLastLog is configured |
| CCE-88366-0 | AC-3, AC-3(3)(a), AU-9, SC-7(21) | `selinux_policytype` | Insert correct line to /etc/selinux/config |
| CCE-88372-8 | CM-6(a) | `package_sssd_installed` | Ensure sssd is installed |
| CCE-88376-9 | AC-17(1), AU-10, AU-14(1), CM-6(a), IR-5(1) | `grub2_audit_argument` | Check if audit argument is already present in /etc/default/grub |
| CCE-88397-5 | AC-6(1), CM-6(a) | `dir_perms_world_writable_sticky_bits` | Verify that All World-Writable Directories Have Sticky Bits Set - Define |
| CCE-88404-9 | CM-11(a), CM-11(b), CM-5(3), CM-6(a), SA-12, SA-12(10), SC-12, SC-12(3), SI-7 | `ensure_gpgcheck_globally_activated` | Ensure GPG check is globally activated |
| CCE-88417-1 | AC-11(a), CM-6(a) | `dconf_gnome_screensaver_lock_delay` | Set GNOME3 Screensaver Lock Delay After Activation Period |
| CCE-88433-8 | AC-6(1), CM-6(a) | `file_permissions_etc_shadow` | Test for existence /etc/shadow |
| CCE-88453-6 | AC-6 (1) | `file_groupowner_backup_etc_gshadow` | Set the file_groupowner_backup_etc_gshadow_newgroup variable if represented |
| CCE-88467-6 |  | `audit_rules_execution_chacl` | Record Any Attempts to Run chacl - Set architecture for audit /usr/bin/chacl |
| CCE-88476-7 | AC-11(1), AC-11(1).1, CM-6(a) | `dconf_gnome_screensaver_mode_blank` | Implement Blank Screensaver |
| CCE-88515-2 | CM-11(a), CM-11(b), CM-6(a), SI-2(6) | `clean_components_post_updating` | Ensure dnf Removes Previous Package Versions - Ensure DNF Removes Previous |
| CCE-88520-2 | AU-12(c), AU-2(d), CM-6(a) | `audit_rules_unsuccessful_file_modification_unlink` | Set architecture for audit unlink tasks |
| CCE-88521-0 | AU-4(1) | `rsyslog_encrypt_offload_actionsendstreamdriverauthmode` | Ensure Rsyslog Authenticates Off-Loaded Audit Records - Ensure /etc/rsyslog.conf |
| CCE-88542-6 |  | `package_subscription-manager_installed` | Ensure subscription-manager is installed |
| CCE-88547-5 |  | `package_audispd-plugins_installed` | Ensure audispd-plugins is installed |
| CCE-88549-1 | AU-12(1), AU-8(1)(b), CM-6(a) | `chronyd_or_ntpd_set_maxpoll` | Configure Time Service Maxpoll Interval - Check That /etc/ntp.conf Exist |
| CCE-88576-4 | AC-18(3), AC-18(a), CM-6(a), CM-7(a), CM-7(b), MP-7 | `wireless_disable_interfaces` | Ensure NetworkManager is installed |
| CCE-88580-6 | AC-6(1), CM-6(a) | `accounts_umask_etc_bashrc` | Check if umask in /etc/bashrc is already set |
| CCE-88585-5 | AU-5(1), AU-5(2), AU-5(4), AU-5(b), CM-6(a) | `auditd_data_retention_admin_space_left_percentage` | Configure auditd admin_space_left on Low Disk Space |
| CCE-88586-3 |  | `package_tftp_removed` | 'Remove tftp Daemon: Ensure tftp is removed' |
| CCE-88587-1 | CM-6(a) | `dconf_gnome_session_idle_user_locks` | Prevent user modification of GNOME Session idle-delay |
| CCE-88604-4 |  | `accounts_have_homedir_login_defs` | Insert correct line to /etc/login.defs |
| CCE-88619-2 | AU-5(1), AU-5(2), AU-5(4), AU-5(b), CM-6(a) | `auditd_data_retention_space_left_percentage` | Configure auditd space_left on Low Disk Space |
| CCE-88632-5 | CM-8(3)(a), IA-3 | `usbguard_generate_policy` | Enable service usbguard |
| CCE-88637-4 | AC-2(4), AC-6(9), AU-12(c), AU-2(d), CM-6(a) | `audit_rules_usergroup_modification_shadow` | Record Events that Modify User/Group Information - /etc/shadow - Check if |
| CCE-88638-2 | AC-6(9), AU-12(c), AU-2(d), CM-6(a) | `audit_rules_kernel_module_loading_finit` | Ensure auditd Collects Information on Kernel Module Loading and Unloading |
| CCE-88640-8 | SC-12(2), SC-12(3), SC-13 | `configure_kerberos_crypto_policy` | Configure Kerberos to use System Crypto Policy |
| CCE-88650-7 | AC-9, AC-9(1) | `display_login_attempts` | Ensure PAM Displays Last Logon/Access Notification - Check if system relies |
| CCE-88661-4 | CM-6(a), IA-5(1)(c), IA-5(c) | `set_password_hashing_algorithm_passwordauth` | Set PAM's Password Hashing Algorithm - password-auth - Check if /etc/pam.d/passw |
| CCE-88664-8 | AC-6(1), CM-6(a) | `file_permissions_cron_hourly` | Find /etc/cron.hourly/ file(s) |
| CCE-88665-5 | CM-6(a), CM-7(a), CM-7(b) | `sysctl_net_ipv6_conf_all_accept_ra` | Configure Accepting Router Advertisements on All IPv6 Interfaces - Set fact |
| CCE-88674-7 | CM-6(a), CM-7, CM-7(a), CM-7(b), CM-7.1(ii), IA-5(1)(c), IA-5(1).1(v) | `package_vsftpd_removed` | 'Uninstall vsftpd Package: Ensure vsftpd is removed' |
| CCE-88682-0 | CM-6(a) | `package_pcsc-lite_installed` | Ensure pcsc-lite is installed |
| CCE-88686-1 | CM-6(a), SC-30, SC-30(2), SC-30(5) | `sysctl_kernel_kptr_restrict` | Restrict Exposed Kernel Pointer Addresses Access - Set fact for sysctl paths |
| CCE-88687-9 | CM-6(a), MA-4(6), SC-12(2), SC-12(3), SC-13 | `configure_libreswan_crypto_policy` | Configure Libreswan to use System Crypto Policy |
| CCE-88688-7 |  | `audit_rules_sudoers` | Ensure auditd Collects System Administrator Actions - /etc/sudoers - Check |
| CCE-88689-5 | CM-6(a), CM-7(a), CM-7(b), SC-7(a) | `sysctl_net_ipv4_conf_all_rp_filter` | Enable Kernel Parameter to Use Reverse Path Filtering on all IPv4 Interfaces |
| CCE-88691-1 | AC-6(1), CM-6(a) | `file_groupowner_grub2_cfg` | Set the file_groupowner_grub2_cfg_newgroup variable if represented by gid |
| CCE-88697-8 | CM-6(a), IA-5(1)(c), IA-5(c) | `set_password_hashing_algorithm_systemauth` | Set PAM's Password Hashing Algorithm - Check if /etc/pam.d/system-auth file |
| CCE-88724-0 | CM-6 | `auditd_write_logs` | Insert correct line to /etc/audit/auditd.conf |
| CCE-88732-3 | CM-6 | `coredump_disable_storage` | Disable storing core dump - Search for a section in files |
| CCE-88741-4 | AC-6(1), CM-6(a) | `file_owner_cron_d` | Set the file_owner_cron_d_newown variable if represented by uid |
| CCE-88752-1 | AC-2(4), AC-6(9), AU-12(c), AU-2(d), CM-6(a) | `audit_rules_privileged_commands_newgrp` | Ensure auditd Collects Information on the Use of Privileged Commands - newgrp |
| CCE-88762-0 | AU-12(c), AU-2(d), CM-6(a) | `audit_rules_file_deletion_events_rmdir` | Set architecture for audit rmdir tasks |
| CCE-88771-1 | AC-6(1), CM-5(6), CM-5(6).1, CM-6(a) | `file_permissions_library_dirs` | Find /lib/ file(s) recursively |
| CCE-88785-1 | SC-7(10) | `sysctl_kernel_yama_ptrace_scope` | Restrict usage of ptrace to descendant processes - Set fact for sysctl paths |
| CCE-88796-8 | AC-6(1), CM-6(a) | `sysctl_fs_protected_symlinks` | Enable Kernel Parameter to Enforce DAC on Symlinks - Set fact for sysctl |
| CCE-88804-0 |  | `audit_rules_privileged_commands_rmmod` | Ensure auditd Collects Information on the Use of Privileged Commands - rmmod |
| CCE-88810-7 | AC-7 (a) | `account_password_pam_faillock_system_auth` | Configure the Use of the pam_faillock.so Module in the /etc/pam.d/system-auth |
| CCE-88818-0 | AC-6(9), AU-12(c), AU-2(d), CM-6(a) | `audit_rules_execution_setfiles` | Record Any Attempts to Run setfiles - Set architecture for audit /usr/sbin/setfi |
| CCE-88825-5 | CM-6 | `coredump_disable_backtraces` | Disable core dump backtraces - Search for a section in files |
| CCE-88841-2 | AC-6(1), AU-9(4), CM-6(a) | `directory_group_ownership_var_log_audit` | System Audit Directories Must Be Group Owned By Root - Register Audit Configurat |
| CCE-88843-8 | AU-12(c) | `audit_privileged_commands_reboot` | Ensure auditd Collects Information on the Use of Privileged Commands - reboot |
| CCE-88844-6 | CM-6(a), IA-5(1)(a), IA-5(4), IA-5(c) | `accounts_password_pam_maxclassrepeat` | Ensure PAM Enforces Password Requirements - Maximum Consecutive Repeating |
| CCE-88855-2 | CM-6(b), CM-6.1(iv) | `sudoers_validate_passwd` | Find out if /etc/sudoers.d/* files contain Defaults targetpw to be deduplicated |
| CCE-88868-5 | AC-6(1), CM-6(a) | `file_permissions_etc_group` | Test for existence /etc/group |
| CCE-88874-3 | AC-6(9), AU-12(c), AU-2(d), CM-6(a) | `audit_rules_privileged_commands_ssh_keysign` | Ensure auditd Collects Information on the Use of Privileged Commands - ssh-keysi |
| CCE-88877-6 |  | `file_ownership_audit_configuration` | Set the file_ownership_audit_configuration_newown variable if represented |
| CCE-88880-0 | CM-6(a), CM-7(a), CM-7(b) | `package_gdm_removed` | 'Remove the GDM Package Group: Ensure gdm is removed' |
| CCE-88881-8 | AC-6, AC-6(1), CM-6(a), CM-7(a), CM-7(b), MP-7 | `mount_option_boot_nosuid` | 'Add nosuid Option to /boot: Check information associated to mountpoint' |
| CCE-88892-5 | CM-6(a), IA-11 | `sudo_remove_no_authenticate` | Find /etc/sudoers.d/ files |
| CCE-88897-4 | AU-5(1), AU-5(2), AU-5(4), AU-5(b), CM-6(a) | `auditd_data_retention_space_left_action` | Configure auditd space_left Action on Low Disk Space |
| CCE-88901-4 | AC-8(a), AC-8(c) | `dconf_gnome_login_banner_text` | Set the GNOME3 Login Warning Banner Text |
| CCE-88914-7 | AC-6(1), CM-6(a) | `file_owner_cron_allow` | Set the file_owner_cron_allow_newown variable if represented by uid |
| CCE-88919-6 | AC-6(1), CM-6(a) | `file_permissions_cron_daily` | Find /etc/cron.daily/ file(s) |
| CCE-88921-2 | AU-3, CM-6 | `auditd_log_format` | Insert correct line to /etc/audit/auditd.conf |
| CCE-88922-0 | AU-12(c) | `audit_privileged_commands_shutdown` | Ensure auditd Collects Information on the Use of Privileged Commands - shutdown |
| CCE-88933-7 | AC-6(9), AU-12(3), AU-7(a), AU-7(b), AU-8(b), CM-5(1) | `audit_rules_suid_privilege_function` | Set suid_audit_rules fact |
| CCE-88938-6 | AC-6(9), AU-12(c), AU-2(d), CM-6(a) | `audit_rules_login_events_lastlog` | Record Attempts to Alter Logon and Logout Events - lastlog - Check if watch |
| CCE-88943-6 | AC-6(1), CM-6(a) | `file_owner_cron_weekly` | Set the file_owner_cron_weekly_newown variable if represented by uid |
| CCE-88948-5 | AC-6(9), AU-12(c), AU-2(d), CM-6(a) | `audit_rules_login_events_tallylog` | Record Attempts to Alter Logon and Logout Events - tallylog - Check if watch |
| CCE-88957-6 | AC-6, AC-6(1), CM-6(a), CM-7(a), CM-7(b), MP-7 | `mount_option_var_log_audit_noexec` | 'Add noexec Option to /var/log/audit: Check information associated to mountpoint |
| CCE-88966-7 | AC-2(3), CM-6(a), IA-4(e) | `account_disable_post_pw_expiration` | Set Account Expiration Following Inactivity |
| CCE-88971-7 | SI-16 | `grub2_pti_argument` | Check if pti argument is already present in /etc/default/grub |
| CCE-88981-6 | AC-6, AC-6(1), CM-6(a), CM-7(a), CM-7(b), MP-7 | `mount_option_nodev_nonroot_local_partitions` | 'Add nodev Option to Non-Root Local Partitions: Refresh facts' |
| CCE-88985-7 | CM-6(a), CM-7(2), CM-7(a), CM-7(b), MA-3 | `network_sniffer_disabled` | Ensure System is Not Acting as a Network Sniffer - Gather network interfaces |
| CCE-88986-5 | AC-6(1), CM-6(a) | `file_groupowner_cron_monthly` | Set the file_groupowner_cron_monthly_newgroup variable if represented by |
| CCE-88987-3 | AC-6, AC-6(1), CM-6(a), CM-7(a), CM-7(b), MP-7 | `mount_option_home_nosuid` | 'Add nosuid Option to /home: Check information associated to mountpoint' |
| CCE-88996-4 |  | `package_policycoreutils_installed` | Ensure policycoreutils is installed |
| CCE-89000-4 | SI-11(a), SI-11(b) | `sysctl_kernel_dmesg_restrict` | Restrict Access to Kernel Message Buffer - Set fact for sysctl paths |
| CCE-89002-0 | AU-12(1), AU-8(1) | `chronyd_client_only` | Insert correct line to /etc/chrony.conf |
| CCE-89017-8 | AC-6 (1) | `file_owner_backup_etc_group` | Set the file_owner_backup_etc_group_newown variable if represented by uid |
| CCE-89018-6 | AU-4(1) | `rsyslog_encrypt_offload_defaultnetstreamdriver` | Ensure Rsyslog Encrypts Off-Loaded Audit Records - Ensure /etc/rsyslog.conf |
| CCE-89020-2 |  | `audit_rules_sudoers_d` | Ensure auditd Collects System Administrator Actions - /etc/sudoers.d/ - |
| CCE-89029-3 | AC-6(9), AU-12(c), AU-2(d), CM-6(a) | `audit_rules_privileged_commands_crontab` | Ensure auditd Collects Information on the Use of Privileged Commands - crontab |
| CCE-89035-0 |  | `file_groupowner_var_log` | Set the file_groupowner_var_log_newgroup variable if represented by gid |
| CCE-89040-0 | AU-5(1), AU-5(2), AU-5(4), AU-5(b), CM-6(a) | `auditd_data_retention_admin_space_left_action` | Configure auditd admin_space_left Action on Low Disk Space |
| CCE-89056-6 | AC-6 (1) | `file_permissions_backup_etc_gshadow` | Test for existence /etc/gshadow- |
| CCE-89062-4 | AC-6(1), CM-6(a) | `file_groupowner_crontab` | Set the file_groupowner_crontab_newgroup variable if represented by gid |
| CCE-89079-8 | CM-6(a), SC-39 | `sysctl_kernel_exec_shield` | Check if noexec argument is already present in /etc/default/grub |
| CCE-89080-6 | AC-6(1), CM-6(a) | `file_groupowner_cron_weekly` | Set the file_groupowner_cron_weekly_newgroup variable if represented by |
| CCE-89081-4 | AU-5(2), AU-5(a), CM-6(a), IA-5(1) | `auditd_data_retention_action_mail_acct` | Configure auditd mail_acct Action on Low Disk Space - Configure auditd mail_acct |
| CCE-89085-5 | AC-17(2), AC-17(a), CM-6(a), MA-4(6), SC-12(2), SC-12(3), SC-13 | `configure_crypto_policy` | Configure System Cryptography Policy - Check current crypto policy (runtime) |
| CCE-89086-3 | CM-6(a) | `grub2_page_poison_argument` | Check if page_poison argument is already present in /etc/default/grub |
| CCE-89089-7 | CM-6(a), IA-5(1)(a), IA-5(4), IA-5(c) | `accounts_password_pam_dcredit` | Ensure PAM Enforces Password Requirements - Minimum Digit Characters - Find |
| CCE-89093-9 |  | `file_owner_var_log_messages` | Set the file_owner_var_log_messages_newown variable if represented by uid |
| CCE-89106-9 |  | `package_rsyslog-gnutls_installed` | Ensure rsyslog-gnutls is installed |
| CCE-89121-8 |  | `file_permissions_cron_allow` | Test for existence /etc/cron.allow |
| CCE-89129-1 | AC-6, AC-6(1), CM-6(a), CM-7(a), CM-7(b), MP-7 | `mount_option_var_log_noexec` | 'Add noexec Option to /var/log: Check information associated to mountpoint' |
| CCE-89134-1 |  | `audit_rules_privileged_commands_pkexec` | Ensure auditd Collects Information on the Use of Privileged Commands - pkexec |
| CCE-89135-8 | CM-6(a), CM-6(b), CM-6.1(iv), CM-7(a), CM-7(b) | `sysctl_net_ipv6_conf_default_accept_source_route` | Disable Kernel Parameter for Accepting Source-Routed Packets on IPv6 Interfaces |
| CCE-89145-7 | AC-17(a), CM-6(a), CM-7(a), CM-7(b) | `sshd_disable_gssapi_auth` | Disable GSSAPI Authentication - Check if the parameter GSSAPIAuthentication |
| CCE-89177-0 | CM-6(a), CM-7(a), CM-7(b), SC-5, SC-7(a) | `sysctl_net_ipv4_conf_default_send_redirects` | Disable Kernel Parameter for Sending ICMP Redirects on all IPv4 Interfaces |
| CCE-89210-9 | AC-6(1), CM-6(a) | `file_groupowner_etc_passwd` | Set the file_groupowner_etc_passwd_newgroup variable if represented by gid |
| CCE-89215-8 | AC-2(4), AC-6(9), AU-12(c), AU-2(d), CM-6(a) | `audit_rules_privileged_commands_passwd` | Ensure auditd Collects Information on the Use of Privileged Commands - passwd |
| CCE-89232-3 | CM-6 | `sysctl_kernel_kexec_load_disabled` | Disable Kernel Image Loading - Set fact for sysctl paths |
| CCE-89241-4 | CM-6(a) | `package_openssh-server_installed` | Ensure openssh-server is installed |
| CCE-89250-5 | AC-7(b), CM-6(a) | `accounts_passwords_pam_faillock_unlock_time` | Set Lockout Time for Failed Password Attempts - Check if system relies on |
| CCE-89266-1 | CM-6(a), CM-7(a), CM-7(b) | `xwindows_runlevel_target` | Switch to multi-user runlevel |
| CCE-89282-8 | AC-18 | `kernel_module_can_disabled` | Ensure kernel module 'can' is disabled |
| CCE-89287-7 | CM-6(a), CM-7(a), CM-7(b) | `package_tftp-server_removed` | 'Uninstall tftp-server Package: Ensure tftp-server is removed' |
| CCE-89291-9 | AU-12(c), AU-2(d), CM-6(a) | `audit_rules_unsuccessful_file_modification_openat` | Set architecture for audit openat tasks |
| CCE-89297-6 | CM-6(a), IA-5(1)(a), IA-5(4), IA-5(c) | `accounts_password_pam_ocredit` | Ensure PAM Enforces Password Requirements - Minimum Special Characters - |
| CCE-89298-4 | IA-11 | `disallow_bypass_password_sudo` | Check for pam_succeed_if entry |
| CCE-89301-6 | CM-6(a), CM-7(a), CM-7(b), MP-7 | `kernel_module_usb-storage_disabled` | Ensure kernel module 'usb-storage' is disabled |
| CCE-89306-5 | AU-12(b) | `file_permissions_etc_audit_auditd` | Test for existence /etc/audit/auditd.conf |
| CCE-89307-3 | CM-6(a), IA-5(1)(d), IA-5(f) | `accounts_minimum_age_login_defs` | Set Password Minimum Age |
| CCE-89313-1 | AU-12(b) | `file_permissions_etc_audit_rulesd` | Find /etc/audit/rules.d/ file(s) |
| CCE-89314-9 | AC-6(1), CM-6(a) | `accounts_umask_etc_login_defs` | Check if UMASK is already set |
| CCE-89321-4 | AC-6(1), CM-6(a) | `file_groupowner_cron_d` | Set the file_groupowner_cron_d_newgroup variable if represented by gid |
| CCE-89341-2 |  | `no_user_host_based_files` | Remove User Host-Based Authentication Files - Define Excluded (Non-Local) |
| CCE-89346-1 | CM-3(5) | `package_s-nail_installed` | Ensure s-nail is installed |
| CCE-89350-3 |  | `no_host_based_files` | Remove Host-Based Authentication Files - Define Excluded (Non-Local) File |
| CCE-89356-0 | AU-12(c), AU-2(d), CM-6(a) | `audit_rules_dac_modification_fchmodat` | Set architecture for audit fchmodat tasks |
| CCE-89362-8 |  | `accounts_password_pam_pwquality_system_auth` | Ensure PAM password complexity module is enabled in system-auth - Check |
| CCE-89370-1 | AU-12(c), AU-2(d), CM-6(a) | `audit_rules_dac_modification_fsetxattr` | Set architecture for audit fsetxattr tasks |
| CCE-89374-3 | CM-6(a), CM-7(a), CM-7(b) | `rsyslog_nolisten` | Ensure rsyslog Does Not Accept Remote Messages Unless Acting As Log Server |
| CCE-89386-7 | AC-3, AC-3(3)(a), AU-9, SC-7(21) | `selinux_state` | Ensure SELinux State is Enforcing - Check current SELinux state |
| CCE-89389-1 | AC-6, AC-6(1), CM-6(a), CM-7(a), CM-7(b), MP-7 | `mount_option_var_log_nodev` | 'Add nodev Option to /var/log: Check information associated to mountpoint' |
| CCE-89394-1 | AC-6(9), AU-12(c), AU-2(d), CM-6(a) | `audit_rules_privileged_commands_postdrop` | Ensure auditd Collects Information on the Use of Privileged Commands - postdrop |
| CCE-89397-4 |  | `file_permissions_var_log_messages` | Test for existence /var/log/messages |
| CCE-89403-0 | AC-2(4), AC-6(9), AU-12(c), AU-2(d), CM-6(a) | `audit_rules_privileged_commands_gpasswd` | Ensure auditd Collects Information on the Use of Privileged Commands - gpasswd |
| CCE-89405-5 | AC-6, SC-7(10) | `sysctl_kernel_unprivileged_bpf_disabled` | Disable Access to Network bpf() Syscall From Unprivileged Processes - Set |
| CCE-89409-7 | CM-11(a), CM-11(b), CM-5(3), CM-6(a), SA-12, SA-12(10) | `ensure_gpgcheck_local_packages` | Ensure GPG check Enabled for Local Packages (dnf) |
| CCE-89438-6 | AC-6(1), CM-6(a) | `file_owner_grub2_cfg` | Set the file_owner_grub2_cfg_newown variable if represented by uid |
| CCE-89441-0 |  | `mount_option_var_tmp_nodev` | 'Add nodev Option to /var/tmp: Check information associated to mountpoint' |
| CCE-89448-5 | AU-5(a), AU-5.1(ii) | `postfix_client_configure_mail_alias_postmaster` | Insert correct line to /etc/aliases |
| CCE-89476-6 | CM-6(b) | `sshd_disable_x11_forwarding` | Disable X11 Forwarding - Check if the parameter X11Forwarding is configured |
| CCE-89477-4 | AC-6 (1) | `file_groupowner_backup_etc_group` | Set the file_groupowner_backup_etc_group_newgroup variable if represented |
| CCE-89479-0 | AC-6(9), AU-12(c), AU-2(d), CM-6(a) | `audit_rules_login_events_faillock` | Record Attempts to Alter Logon and Logout Events - faillock - Check if watch |
| CCE-89486-5 | CM-6(a), CM-7(a), CM-7(b) | `sysctl_net_ipv6_conf_default_accept_redirects` | Disable Kernel Parameter for Accepting ICMP Redirects by Default on IPv6 |
| CCE-89505-2 |  | `accounts_password_pam_pwquality_password_auth` | Ensure PAM password complexity module is enabled in password-auth - Check |
| CCE-89508-6 | CM-6(a), IA-5(1)(c), IA-5(c) | `set_password_hashing_algorithm_logindefs` | Set Password Hashing Algorithm in /etc/login.defs |
| CCE-89510-2 |  | `ssh_client_rekey_limit` | Ensure RekeyLimit is not configured in /etc/ssh/ssh_config |
| CCE-89514-4 |  | `dir_perms_world_writable_root_owned` | Ensure All World-Writable Directories Are Owned by root User - Define Excluded |
| CCE-89521-9 | AC-6(9), AU-12(c), AU-2(d), CM-6(a) | `audit_rules_privileged_commands_pam_timestamp_check` | Ensure auditd Collects Information on the Use of Privileged Commands - pam_times |
| CCE-89529-2 | AC-2(4), AC-6(9), AU-12(a), AU-12(c), AU-12.1(ii), AU-12.1(iv), AU-2(d), AU-3, AU-3.1, CM-6(a), MA-4(1)(a) | `audit_rules_privileged_commands_unix_chkpwd` | Ensure auditd Collects Information on the Use of Privileged Commands - unix_chkp |
| CCE-89540-9 | AU-12(c), AU-2(d), CM-6(a) | `audit_rules_dac_modification_chown` | Set architecture for audit chown tasks |
| CCE-89541-7 | AC-2(4), AC-6(9), AU-12(c), AU-2(d), CM-6(a) | `audit_rules_execution_semanage` | Record Any Attempts to Run semanage - Set architecture for audit /usr/sbin/seman |
| CCE-89551-6 | AC-2(4), AC-6(9), AU-12(c), AU-2(d), CM-6(a) | `audit_rules_privileged_commands_chsh` | Ensure auditd Collects Information on the Use of Privileged Commands - chsh |
| CCE-89571-4 | AU-12(c), AU-2(d), CM-6(a) | `audit_rules_dac_modification_setxattr` | Set architecture for audit setxattr tasks |
| CCE-89585-4 |  | `file_permission_user_init_files_root` | Ensure All User Initialization Files Have Mode 0740 Or Less Permissive - |
| CCE-89587-0 | AC-6(9), AU-12(c), AU-2(d), CM-6(a) | `audit_rules_privileged_commands_su` | Ensure auditd Collects Information on the Use of Privileged Commands - su |
| CCE-89591-2 |  | `package_chrony_installed` | Ensure chrony is installed |
| CCE-89601-9 | AC-6(9), AU-12(c), AU-2(d), CM-6(a) | `audit_rules_privileged_commands_sudoedit` | Ensure auditd Collects Information on the Use of Privileged Commands - sudoedit |
| CCE-89620-9 | AC-6(1), CM-5(6), CM-5(6).1, CM-6(a) | `file_ownership_binary_dirs` | Read list of system executables without root ownership |
| CCE-89625-8 | CM-6(a), SI-7, SI-7(1) | `aide_verify_ext_attributes` | Get rules groups |
| CCE-89631-6 | CM-6, SC-7(10) | `sysctl_net_core_bpf_jit_harden` | Harden the operation of the BPF just-in-time compiler - Set fact for sysctl |
| CCE-89640-7 | CM-6(a), SI-7, SI-7(1) | `aide_verify_acls` | Get rules groups |
| CCE-89661-3 | CM-6(a), SC-2(1) | `grub2_disable_interactive_boot` | Verify that Interactive Boot is Disabled - Verify GRUB_DISABLE_RECOVERY=true |
| CCE-89668-8 |  | `package_crypto-policies_installed` | Ensure crypto-policies is installed |
| CCE-89677-9 | AU-12(c), AU-2(d), CM-6(a) | `audit_rules_dac_modification_removexattr` | Set architecture for audit removexattr tasks |
| CCE-89684-5 | CM-6(a) | `dconf_gnome_screensaver_lock_enabled` | Enable GNOME3 Screensaver Lock After Idle Period - Enable GNOME3 Screensaver |
| CCE-89698-5 | AC-6(9), AU-12(c), AU-2(d), CM-6(a) | `audit_rules_privileged_commands_sudo` | Ensure auditd Collects Information on the Use of Privileged Commands - sudo |
| CCE-89705-8 | AC-6(1), CM-6(a) | `file_owner_cron_hourly` | Set the file_owner_cron_hourly_newown variable if represented by uid |
| CCE-89713-2 | AU-12(c), AU-2(d), CM-6(a) | `audit_rules_unsuccessful_file_modification_rename` | Set architecture for audit rename tasks |
| CCE-89730-6 | AC-17(a), AC-6(2), CM-6(a), CM-7(a), CM-7(b), IA-2, IA-2(5) | `sshd_disable_root_login` | Disable SSH Root Login - Check if the parameter PermitRootLogin is configured |
| CCE-89733-0 | AC-6(1), CM-6(a) | `file_permissions_cron_weekly` | Find /etc/cron.weekly/ file(s) |
| CCE-89745-4 | CM-5(6), CM-5(6).1 | `dir_ownership_library_dirs` | Set the dir_ownership_library_dirs_newown variable if represented by uid |
| CCE-89747-0 | AC-17(a), AC-6(1), CM-6(a) | `directory_owner_sshd_config_d` | Set the directory_owner_sshd_config_d_newown variable if represented by |
| CCE-89799-1 | AC-17(a), CM-6(b), CM-7(a), CM-7(b) | `firewalld_sshd_port_enabled` | Enable SSH Server firewalld Firewall Exception - Ensure firewalld and NetworkMan |
| CCE-89800-7 | CM-5(6), CM-5(6).1 | `file_groupownership_system_commands_dirs` | Verify that system commands files are group owned by root or a system account |
| CCE-89801-5 |  | `file_permissions_var_log` | Find /var/log/ file(s) |
| CCE-89813-0 | CM-6(a), SI-4(22) | `package_fapolicyd_installed` | Ensure fapolicyd is installed |
| CCE-89816-3 | AC-6(9), CM-6(a) | `audit_rules_immutable` | Make the auditd Configuration Immutable - Collect all files from /etc/audit/rule |
| CCE-89822-1 |  | `audit_rules_dac_modification_umount2` | Set architecture for audit umount2 tasks |
| CCE-89829-6 | AC-17(a), AC-6(1), CM-6(a) | `file_owner_sshd_config` | Set the file_owner_sshd_config_newown variable if represented by uid |
| CCE-89869-2 | AU-12(c), AU-2(d), CM-6(a) | `audit_rules_unsuccessful_file_modification_truncate` | Set architecture for audit truncate tasks |
| CCE-89893-2 | AU-12(a), AU-12(c), AU-12.1(ii), AU-12.1(iv), AU-3, AU-3.1, MA-4(1)(a) | `audit_rules_privileged_commands_modprobe` | Ensure auditd Collects Information on the Use of Privileged Commands - modprobe |
| CCE-89914-6 | AC-6 (1) | `file_groupowner_backup_etc_passwd` | Set the file_groupowner_backup_etc_passwd_newgroup variable if represented |
| CCE-89933-6 |  | `accounts_user_interactive_home_directory_defined` | Get all local users from /etc/passwd |
| CCE-89959-1 | CM-6(a), IA-5(1)(a), IA-5(4), IA-5(c) | `accounts_password_pam_ucredit` | Ensure PAM Enforces Password Requirements - Minimum Uppercase Characters |
| CCE-89963-3 | AC-17(a), AC-6(1), CM-6(a) | `directory_permissions_sshd_config_d` | Find /etc/ssh/sshd_config.d/ file(s) |
| CCE-89972-4 | AU-12(c), AU-2(d), CM-6(a) | `audit_rules_unsuccessful_file_modification_unlinkat` | Set architecture for audit unlinkat tasks |
| CCE-89982-3 | AC-6(9), AU-12(c), AU-2(d), CM-6(a) | `audit_rules_kernel_module_loading_delete` | Ensure auditd Collects Information on Kernel Module Unloading - delete_module |
| CCE-89991-4 | AC-17(a), AC-6(1), CM-6(a) | `directory_groupowner_sshd_config_d` | Set the directory_groupowner_sshd_config_d_newgroup variable if represented |
| CCE-90014-2 | AC-3, CM-6(a), IA-2 | `require_singleuser_auth` | Require Authentication for Single User Mode - find files which already override |
| CCE-90035-7 | AC-6(1), CM-6(a) | `disable_ctrlaltdel_reboot` | Disable Ctrl-Alt-Del Reboot Activation |
| CCE-90043-1 | AC-6(1), CM-6(a) | `file_groupowner_etc_gshadow` | Set the file_groupowner_etc_gshadow_newgroup variable if represented by |
| CCE-90051-4 | AC-17(a), CM-6(a), CM-7(a), CM-7(b) | `sshd_disable_compression` | Disable Compression Or Set Compression to delayed - Check if the parameter |
| CCE-90065-4 | CM-6(a), IA-2(1), IA-2(11), IA-2(2), IA-2(3), IA-2(4), IA-2(6), IA-2(7) | `configure_opensc_card_drivers` | Check existence of opensc conf |
| CCE-90078-7 | AC-6(1), CM-6(a) | `file_permissions_crontab` | Test for existence /etc/crontab |
| CCE-90081-1 |  | `audit_rules_privileged_commands_ssh_agent` | Record Any Attempts to Run ssh-agent - Set architecture for audit /usr/bin/ssh-a |
| CCE-90083-7 | CM-6(a), CM-6(b), CM-6.1(iv), CM-7(a), CM-7(b) | `sysctl_net_ipv6_conf_all_accept_redirects` | Disable Accepting ICMP Redirects for All IPv6 Interfaces - Set fact for |
| CCE-90094-4 | AC-6(1), CM-6(a) | `file_groupowner_cron_allow` | Set the file_groupowner_cron_allow_newgroup variable if represented by gid |
| CCE-90100-9 | AU-12(c), AU-2(d), CM-6(a) | `audit_rules_dac_modification_lremovexattr` | Set architecture for audit lremovexattr tasks |
| CCE-90129-8 | AC-6(1), AU-9(4), CM-6(a) | `file_permissions_var_log_audit` | Get audit log files |
| CCE-90132-2 | AC-6, AC-6(1), CM-6(a), CM-7(a), CM-7(b), MP-7 | `mount_option_boot_nodev` | 'Add nodev Option to /boot: Check information associated to mountpoint' |
| CCE-90134-8 | CM-6(a), IA-5(1)(a), IA-5(4), IA-5(c) | `accounts_password_pam_enforce_root` | Ensure PAM Enforces Password Requirements - Enforce for root User |
| CCE-90140-5 | SC-3 | `grub2_init_on_free` | Check if init_on_free argument is already present in /etc/default/grub |
| CCE-90142-1 | AC-6 | `sysctl_kernel_perf_event_paranoid` | Disallow kernel profiling by unprivileged users - Set fact for sysctl paths |
| CCE-90143-9 | AC-2(4), AC-6(9), AU-12(c), AU-2(d), CM-6(a) | `audit_rules_privileged_commands_chage` | Ensure auditd Collects Information on the Use of Privileged Commands - chage |
| CCE-90154-6 | AC-6, AC-6(1), CM-6(a), CM-7(a), CM-7(b), MP-7 | `mount_option_nodev_removable_partitions` | Ensure permission nodev are set on var_removable_partition |
| CCE-90165-2 | CM-6(a), CM-7(a), CM-7(b), SC-5, SC-7(a) | `sysctl_net_ipv4_conf_all_accept_source_route` | Disable Kernel Parameter for Accepting Source-Routed Packets on all IPv4 |
| CCE-90172-8 | AC-6(9), AU-12(c), AU-2(d), CM-6(a) | `audit_rules_kernel_module_loading_init` | Ensure auditd Collects Information on Kernel Module Loading - init_module |
| CCE-90177-7 | CM-3(5), CM-6(a) | `aide_scan_notification` | Ensure AIDE is installed |
| CCE-90182-7 | AC-7(a), AC-7(b), AC-7.1(ii) | `accounts_passwords_pam_faillock_dir` | Lock Accounts Must Persist - Ensure necessary SELinux packages are installed |
| CCE-90212-2 | AC-2(g), AC-4, AC-6(9), AU-10, AU-12(c), AU-14(1), AU-2(d), AU-3, AU-4(1), CA-3(5), CM-6, CM-6(a), CM-7(a), CM-7(b), CM-8(3)(a), IA-2(1), IA-2(11), IA-2(2), IA-2(3), IA-2(4), IA-2(6), IA-2(7), IA-3, IA-5(10), MP-7, SC-24, SC-7(21), SC-8, SC-8(1), SC-8(2), SC-8(3), SC-8(4), SI-4(22), SI-4(23) | `special_service_block` | Disable debug-shell SystemD Service - Disable Socket debug-shell |
| CCE-90237-9 | AU-12(c), AU-2(d), CM-6(a) | `audit_rules_file_deletion_events_renameat` | Set architecture for audit renameat tasks |
| CCE-90251-0 | AU-12(c), AU-2(d), CM-6(a) | `audit_rules_unsuccessful_file_modification_open_by_handle_at` | Set architecture for audit open_by_handle_at tasks |
| CCE-90260-1 | CM-6(a), SI-7, SI-7(1) | `aide_use_fips_hashes` | Configure AIDE to Use FIPS 140-2 for Validating Hashes - Ensure aide is |
| CCE-90261-9 | AC-6(1), CM-6(a) | `file_groupowner_etc_group` | Set the file_groupowner_etc_group_newgroup variable if represented by gid |
| CCE-90275-9 |  | `sssd_enable_smartcards` | Test for domain group |
| CCE-90276-7 | CM-6(a), IA-5(1)(a), IA-5(4), IA-5(c) | `accounts_password_pam_lcredit` | Ensure PAM Enforces Password Requirements - Minimum Lowercase Characters |
| CCE-90325-2 | CM-6(a), IA-5(1)(c), IA-5(c) | `set_password_hashing_algorithm_libuserconf` | Set Password Hashing Algorithm in /etc/libuser.conf - Set Password Hashing |
| CCE-90342-7 | AC-6(1), CM-6(a) | `file_groupowner_cron_daily` | Set the file_groupowner_cron_daily_newgroup variable if represented by gid |
| CCE-90343-5 | CM-6 b, CM-7 (2), CM-7 (5) (b) | `fapolicy_default_deny` | Configure Fapolicy Module to Employ a Deny-all, Permit-by-exception Policy |
| CCE-90353-4 | CM-6(a) | `package_rsyslog_installed` | Ensure rsyslog is installed |
| CCE-90362-5 | AC-12, AC-17(a), AC-2(5), CM-6(a), SC-10 | `sshd_set_idle_timeout` | Set SSH Client Alive Interval - Check if the parameter ClientAliveInterval |
| CCE-90363-3 | CM-6(a), IA-5(1)(b), IA-5(4), IA-5(c) | `accounts_password_pam_difok` | Ensure PAM Enforces Password Requirements - Minimum Different Characters |
| CCE-90372-4 | AU-4(1), AU-9(2), CM-6(a) | `rsyslog_remote_loghost` | Set rsyslog remote loghost |
| CCE-90377-3 | AC-6 (1) | `file_owner_backup_etc_passwd` | Set the file_owner_backup_etc_passwd_newown variable if represented by uid |
| CCE-90378-1 | AC-6, AC-6(1), CM-6(a), CM-7(a), CM-7(b), MP-7 | `mount_option_noexec_removable_partitions` | Ensure permission noexec are set on var_removable_partition |
| CCE-90383-1 | CM-6(a) | `rsyslog_cron_logging` | Ensure cron Is Logging To Rsyslog - Ensure /etc/rsyslog.conf exists |
| CCE-90391-4 | AC-17(a), CM-6(a), CM-7(a), CM-7(b), IA-2, IA-2(8), IA-2(9) | `use_kerberos_security_all_exports` | Drop any security clause for every export |
| CCE-90403-7 |  | `package_gnutls-utils_installed` | Ensure gnutls-utils is installed |
| CCE-90409-4 | CM-6(a), CM-7(a), CM-7(b), SC-7(a) | `sysctl_net_ipv4_conf_all_accept_redirects` | Disable Accepting ICMP Redirects for All IPv4 Interfaces - Set fact for |
| CCE-90438-3 | SC-7(10) | `service_systemd-coredump_disabled` | Disable acquiring, saving, and processing core dumps - Collect systemd Socket |
| CCE-90449-0 |  | `accounts_user_dot_no_world_writable_programs` | User Initialization Files Must Not Run World-Writable Programs - Initialize |
| CCE-90450-8 | CM-6(a), CM-7(a), CM-7(b) | `sysctl_net_ipv6_conf_all_accept_source_route` | Disable Kernel Parameter for Accepting Source-Routed Packets on all IPv6 |
| CCE-90466-4 | AU-12(c), AU-2(d), CM-6(a) | `audit_rules_dac_modification_chmod` | Set architecture for audit chmod tasks |
| CCE-90477-1 | CM-6(a) | `package_aide_installed` | Ensure aide is installed |
| CCE-90489-6 | CM-6(a), CM-7(a), CM-7(b) | `kernel_module_sctp_disabled` | Ensure kernel module 'sctp' is disabled |
| CCE-90491-2 | CM-6(b), CM-6.1(iv) | `no_empty_passwords_etc_shadow` | Collect users with no password |
| CCE-90504-2 | AC-6, AC-6(1), CM6(a) | `mount_option_nosuid_remote_filesystems` | Get nfs and nfs4 mount points, that don't have nosuid |
| CCE-90508-3 |  | `set_password_hashing_min_rounds_logindefs` | Set Password Hashing Rounds in /etc/login.defs - extract contents of the |
| CCE-90522-4 | AC-6, AC-6(1), CM-6(a), CM-7(a), CM-7(b), MP-7 | `mount_option_tmp_nodev` | 'Add nodev Option to /tmp: Check information associated to mountpoint' |
| CCE-90557-0 | CM-6(a), CM-7(a), CM-7(b) | `sysctl_net_ipv6_conf_default_accept_ra` | Disable Accepting Router Advertisements on all IPv6 Interfaces by Default |
| CCE-90568-7 | AC-7 (a) | `account_password_selinux_faillock_dir` | An SELinux Context must be configured for the pam_faillock.so records directory |
| CCE-90591-9 | AC-17(a), CM-6(a), CM-7(a), CM-7(b) | `sshd_disable_kerb_auth` | Disable Kerberos Authentication - Check if the parameter KerberosAuthentication |
| CCE-90595-0 |  | `use_pam_wheel_for_su` | Restrict usage of su command only to members of wheel group |
| CCE-90597-6 | AC-6(1), CM-6(a) | `accounts_umask_etc_csh_cshrc` | Check if umask in /etc/csh.cshrc is already set |
| CCE-90625-5 |  | `sshd_enable_pubkey_auth` | Enable Public Key Authentication - Check if the parameter PubkeyAuthentication |
| CCE-90639-6 | AC-6, AC-6(1), CM-6(a), CM-7(a), CM-7(b), MP-7 | `mount_option_var_log_nosuid` | 'Add nosuid Option to /var/log: Check information associated to mountpoint' |
| CCE-90644-6 | AC-6(1), CM-6(a) | `file_permissions_etc_passwd` | Test for existence /etc/passwd |
| CCE-90651-1 | AU-12(c), AU-2(d), CM-6(a) | `audit_rules_dac_modification_fchownat` | Set architecture for audit fchownat tasks |
| CCE-90652-9 | AC-6(9), AU-12(c), AU-2(d), CM-6(a) | `audit_rules_privileged_commands_userhelper` | Ensure auditd Collects Information on the Use of Privileged Commands - userhelpe |
| CCE-90658-6 | AC-6(1), CM-6(a), CM-7(b) | `dconf_gnome_disable_ctrlaltdel_reboot` | Disable Ctrl-Alt-Del Reboot Key Sequence in GNOME3 |
| CCE-90663-6 |  | `accounts_password_pam_pwquality_retry` | Ensure PAM Enforces Password Requirements - Authentication Retry Prompts |
| CCE-90664-4 | AC-2(4), AC-6(9), AU-12(c), AU-2(d), CM-6(a) | `audit_rules_usergroup_modification_opasswd` | Record Events that Modify User/Group Information - /etc/security/opasswd |
| CCE-90685-9 | AU-12(c), AU-2(d), CM-6(a) | `audit_rules_dac_modification_fchown` | Set architecture for audit fchown tasks |
| CCE-90694-1 | AC-6, AC-6(1), CM-6(a), CM-7(a), CM-7(b), MP-7 | `mount_option_var_log_audit_nosuid` | 'Add nosuid Option to /var/log/audit: Check information associated to mountpoint |
| CCE-90712-1 | CM-6(b) | `networkmanager_dns_mode` | NetworkManager DNS Mode Must Be Must Configured - Search for a section in |
| CCE-90730-3 | AC-7 (a) | `accounts_passwords_pam_faillock_audit` | Account Lockouts Must Be Logged - Check if system relies on authselect tool |
| CCE-90733-7 | AU-12(c), AU-2(d), CM-6(a) | `audit_rules_file_deletion_events_rename` | Set architecture for audit rename tasks |
| CCE-90741-0 | CM-6(a), IA-5(13) | `sssd_offline_cred_expiration` | Test for domain group |
| CCE-90751-9 | AC-10, CM-6(a) | `accounts_max_concurrent_login_sessions` | Find /etc/security/limits.d files containing maxlogins configuration |
| CCE-90753-5 | AC-6(1), CM-6(a) | `file_owner_cron_monthly` | Set the file_owner_cron_monthly_newown variable if represented by uid |
